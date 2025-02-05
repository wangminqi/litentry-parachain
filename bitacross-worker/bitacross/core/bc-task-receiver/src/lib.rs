#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use thiserror_sgx as thiserror;
	pub use threadpool_sgx as threadpool;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub use crate::sgx_reexport_prelude::*;
use core::ops::Deref;

use bc_task_sender::init_bit_across_task_sender_storage;
use codec::{Decode, Encode};
use frame_support::ensure;
use lc_direct_call::{DirectCall, DirectCallSigned};
use litentry_primitives::{aes_encrypt_default, AesRequest};
use log::*;
use std::{
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use threadpool::ThreadPool;

use itp_ocall_api::{EnclaveAttestationOCallApi, EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;

use bc_relayer_registry::RelayerRegistryLookup;
use ita_stf::TrustedCallSigned;
use itp_sgx_crypto::{ecdsa::Pair as EcdsaPair, schnorr::Pair as SchnorrPair};
use lc_direct_call::handler::{sign_bitcoin, sign_ethereum};
use litentry_primitives::DecryptableRequest;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

pub struct BitAcrossTaskContext<
	SKR,
	EKR,
	BKR,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	RRL: RelayerRegistryLookup,
> where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub shielding_key: Arc<SKR>,
	pub ethereum_key_repository: Arc<EKR>,
	pub bitcoin_key_repository: Arc<BKR>,
	pub enclave_signer: Arc<S>,
	pub state_handler: Arc<H>,
	pub ocall_api: Arc<O>,
	pub relayer_registry_lookup: Arc<RRL>,
}

impl<
		SKR,
		EKR,
		BKR,
		S: StfEnclaveSigning<TrustedCallSigned>,
		H: HandleState,
		O: EnclaveOnChainOCallApi,
		RRL: RelayerRegistryLookup,
	> BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL>
where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	H::StateT: SgxExternalitiesTrait,
{
	pub fn new(
		shielding_key: Arc<SKR>,
		ethereum_key_repository: Arc<EKR>,
		bitcoin_key_repository: Arc<BKR>,
		enclave_signer: Arc<S>,
		state_handler: Arc<H>,
		ocall_api: Arc<O>,
		relayer_registry_lookup: Arc<RRL>,
	) -> Self {
		Self {
			shielding_key,
			ethereum_key_repository,
			bitcoin_key_repository,
			enclave_signer,
			state_handler,
			ocall_api,
			relayer_registry_lookup,
		}
	}
}

pub fn run_bit_across_handler_runner<SKR, EKR, BKR, S, H, O, RRL>(
	context: Arc<BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL>>,
) where
	SKR: AccessKey + Send + Sync + 'static,
	EKR: AccessKey<KeyType = EcdsaPair> + Send + Sync + 'static,
	BKR: AccessKey<KeyType = SchnorrPair> + Send + Sync + 'static,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	RRL: RelayerRegistryLookup + Send + Sync + 'static,
{
	let bit_across_task_receiver = init_bit_across_task_sender_storage();
	let n_workers = 2;
	let pool = ThreadPool::new(n_workers);

	while let Ok(mut req) = bit_across_task_receiver.recv() {
		let context_pool = context.clone();
		pool.execute(move || {
			if let Err(e) = req.sender.send(handle_request(&mut req.request, context_pool)) {
				warn!("Unable to submit response back to the handler: {:?}", e);
			}
		});
	}

	pool.join();
	warn!("bit_across_task_receiver loop terminated");
}

pub fn handle_request<SKR, EKR, BKR, S, H, O, RRL>(
	request: &mut AesRequest,
	context: Arc<BitAcrossTaskContext<SKR, EKR, BKR, S, H, O, RRL>>,
) -> Result<Vec<u8>, String>
where
	SKR: AccessKey,
	EKR: AccessKey<KeyType = EcdsaPair>,
	BKR: AccessKey<KeyType = SchnorrPair>,
	<SKR as AccessKey>::KeyType: ShieldingCryptoEncrypt + ShieldingCryptoDecrypt + 'static,
	S: StfEnclaveSigning<TrustedCallSigned> + Send + Sync + 'static,
	H: HandleState + Send + Sync + 'static,
	O: EnclaveOnChainOCallApi + EnclaveMetricsOCallApi + EnclaveAttestationOCallApi + 'static,
	RRL: RelayerRegistryLookup + 'static,
{
	let enclave_shielding_key = context
		.shielding_key
		.retrieve_key()
		.map_err(|e| format!("Failed to retrieve shielding key: {:?}", e))?;
	let dc = request
		.decrypt(Box::new(enclave_shielding_key))
		.ok()
		.and_then(|v| DirectCallSigned::decode(&mut v.as_slice()).ok())
		.ok_or_else(|| "Failed to decode payload".to_string())?;

	let mrenclave = match context.ocall_api.get_mrenclave_of_self() {
		Ok(m) => m.m,
		Err(_) => return Err("Failed to get mrenclave".to_string()),
	};
	ensure!(dc.verify_signature(&mrenclave, &request.shard), "Failed to verify sig".to_string());
	match dc.call {
		DirectCall::SignBitcoin(signer, aes_key, payload) => sign_bitcoin::handle(
			signer,
			payload,
			context.relayer_registry_lookup.deref(),
			context.bitcoin_key_repository.deref(),
		)
		.map(|r| aes_encrypt_default(&aes_key, &r).encode()),
		DirectCall::SignEthereum(signer, aes_key, msg) => sign_ethereum::handle(
			signer,
			msg,
			context.relayer_registry_lookup.deref(),
			context.ethereum_key_repository.deref(),
		)
		.map(|r| aes_encrypt_default(&aes_key, &r).encode()),
	}
}

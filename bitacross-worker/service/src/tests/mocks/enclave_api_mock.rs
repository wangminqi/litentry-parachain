/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use codec::{Decode, Encode};
use core::fmt::Debug;
use itc_parentchain::primitives::{
	ParentchainId, ParentchainInitParams,
	ParentchainInitParams::{Parachain, Solochain},
};
use itp_enclave_api::{enclave_base::EnclaveBase, sidechain::Sidechain, EnclaveResult};
use itp_settings::worker::MR_ENCLAVE_SIZE;
use itp_sgx_crypto::{ecdsa, schnorr};
use itp_stf_interface::ShardCreationInfo;
use itp_storage::StorageProof;
use itp_types::{
	parentchain::{Balance, Header},
	EnclaveFingerprint, ShardIdentifier,
};
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::ed25519;

/// mock for EnclaveBase - use in tests
pub struct EnclaveMock;

impl EnclaveBase for EnclaveMock {
	fn init(&self, _mu_ra_url: &str, _untrusted_url: &str, _base_dir: &str) -> EnclaveResult<()> {
		Ok(())
	}

	fn init_direct_invocation_server(&self, _rpc_server_addr: String) -> EnclaveResult<()> {
		unreachable!()
	}

	fn init_parentchain_components<Header: Debug + Decode>(
		&self,
		params: ParentchainInitParams,
	) -> EnclaveResult<Header> {
		let genesis_header_encoded = match params {
			Solochain { params, .. } => params.genesis_header.encode(),
			Parachain { params, .. } => params.genesis_header.encode(),
		};
		let header = Header::decode(&mut genesis_header_encoded.as_slice())?;
		Ok(header)
	}

	fn init_shard(&self, _shard: Vec<u8>) -> EnclaveResult<()> {
		unimplemented!()
	}

	fn init_proxied_shard_vault(
		&self,
		_shard: &ShardIdentifier,
		_parentchain_id: &ParentchainId,
		_funding_balance: Balance,
	) -> EnclaveResult<()> {
		unimplemented!()
	}

	fn init_shard_creation_parentchain_header(
		&self,
		shard: &ShardIdentifier,
		parentchain_id: &ParentchainId,
		header: &Header,
	) -> EnclaveResult<()> {
		unimplemented!()
	}

	fn get_shard_creation_info(&self, shard: &ShardIdentifier) -> EnclaveResult<ShardCreationInfo> {
		unimplemented!()
	}

	fn set_nonce(&self, _: u32, _: ParentchainId) -> EnclaveResult<()> {
		unimplemented!()
	}

	fn set_node_metadata(&self, _metadata: Vec<u8>, _: ParentchainId) -> EnclaveResult<()> {
		todo!()
	}

	fn get_rsa_shielding_pubkey(&self) -> EnclaveResult<Rsa3072PubKey> {
		unreachable!()
	}

	fn get_ecc_signing_pubkey(&self) -> EnclaveResult<ed25519::Public> {
		unreachable!()
	}

	fn get_ecc_vault_pubkey(&self, _shard: &ShardIdentifier) -> EnclaveResult<ed25519::Public> {
		unreachable!()
	}

	fn get_bitcoin_wallet_pair(&self) -> EnclaveResult<schnorr::Pair> {
		unreachable!()
	}

	fn get_ethereum_wallet_pair(&self) -> EnclaveResult<ecdsa::Pair> {
		unreachable!()
	}

	fn get_fingerprint(&self) -> EnclaveResult<EnclaveFingerprint> {
		Ok([1u8; MR_ENCLAVE_SIZE].into())
	}

	fn migrate_shard(&self, _old_shard: Vec<u8>, _new_shard: Vec<u8>) -> EnclaveResult<()> {
		unimplemented!()
	}

	fn publish_wallets(&self) -> EnclaveResult<()> {
		unimplemented!()
	}
}

impl Sidechain for EnclaveMock {
	fn sync_parentchain<ParentchainBlock: Encode>(
		&self,
		_blocks: &[sp_runtime::generic::SignedBlock<ParentchainBlock>],
		_events: &[Vec<u8>],
		_events_proofs: &[StorageProof],
		_: &ParentchainId,
		_: bool,
	) -> EnclaveResult<()> {
		Ok(())
	}

	fn ignore_parentchain_block_import_validation_until(&self, _until: u32) -> EnclaveResult<()> {
		todo!()
	}
}

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
pub use ita_sgx_runtime::{Balance, Index};
use ita_stf::{Getter, TrustedCall, TrustedCallSigned};
use itc_parentchain_indirect_calls_executor::error::Error;
use itp_stf_primitives::{traits::IndirectExecutor, types::TrustedOperation};
use itp_types::{
	parentchain::{
		AccountId, FilterEvents, HandleParentchainEvents, ParentchainEventProcessingError,
	},
	RsaRequest, H256,
};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};

use lc_dynamic_assertion::AssertionLogicRepository;
use lc_evm_dynamic_assertions::repository::EvmAssertionRepository;
use litentry_primitives::{
	Assertion, Identity, MrEnclave, SidechainBlockNumber, ValidationData, Web3Network, WorkerType,
};
use log::*;
use sp_core::{blake2_256, H160};
use sp_std::vec::Vec;
use std::{format, string::String, sync::Arc};

pub struct ParentchainEventHandler {
	pub assertion_repository: Arc<EvmAssertionRepository>,
}

impl ParentchainEventHandler {
	fn link_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
		encrypted_validation_data: Vec<u8>,
		encrypted_web3networks: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");

		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;
		let validation_data =
			ValidationData::decode(&mut executor.decrypt(&encrypted_validation_data)?.as_slice())?;
		let web3networks: Vec<Web3Network> =
			Decode::decode(&mut executor.decrypt(&encrypted_web3networks)?.as_slice())?;

		let trusted_call = TrustedCall::link_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			validation_data,
			web3networks,
			None,
			Default::default(),
		);
		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn deactivate_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;

		let trusted_call = TrustedCall::deactivate_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);
		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;

		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn activate_identity<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		encrypted_identity: Vec<u8>,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&encrypted_identity)?.as_slice())?;

		let trusted_call = TrustedCall::activate_identity(
			enclave_account_id.into(),
			account.clone().into(),
			identity,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);
		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;

		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn request_vc<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		account: &AccountId,
		assertion: Assertion,
	) -> Result<(), Error> {
		let shard = executor.get_default_shard();
		let enclave_account_id = executor.get_enclave_account().expect("no enclave account");

		let trusted_call = TrustedCall::request_vc(
			enclave_account_id.into(),
			account.clone().into(),
			assertion,
			None,
			Default::default(),
		);

		let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &shard)?;
		let trusted_operation =
			TrustedOperation::<TrustedCallSigned, Getter>::indirect_call(signed_trusted_call);

		let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
		executor.submit_trusted_call(shard, encrypted_trusted_call);

		Ok(())
	}

	fn set_scheduled_enclave(
		worker_type: WorkerType,
		sbn: SidechainBlockNumber,
		mrenclave: MrEnclave,
	) -> Result<(), Error> {
		if worker_type != WorkerType::Identity {
			warn!("Ignore SetScheduledEnclave due to wrong worker_type");
			return Ok(())
		}
		GLOBAL_SCHEDULED_ENCLAVE.update(sbn, mrenclave)?;

		Ok(())
	}

	fn remove_scheduled_enclave(
		worker_type: WorkerType,
		sbn: SidechainBlockNumber,
	) -> Result<(), Error> {
		if worker_type != WorkerType::Identity {
			warn!("Ignore RemoveScheduledEnclave due to wrong worker_type");
			return Ok(())
		}
		GLOBAL_SCHEDULED_ENCLAVE.remove(sbn)?;

		Ok(())
	}

	fn post_opaque_task<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		executor: &Executor,
		request: &RsaRequest,
	) -> Result<(), Error> {
		debug!("post opaque task: {:?}", request);
		executor.submit_trusted_call(request.shard, request.payload.to_vec());

		Ok(())
	}

	fn store_assertion<Executor: IndirectExecutor<TrustedCallSigned, Error>>(
		&self,
		executor: &Executor,
		id: H160,
		byte_code: Vec<u8>,
		secrets: Vec<Vec<u8>>,
	) -> Result<(), Error> {
		debug!("store assertion byte_code: {:?}, secrets: {:?}", byte_code, secrets);
		let mut decrypted_secrets = Vec::with_capacity(secrets.len());

		for secret in secrets.iter() {
			let secret = String::decode(
				&mut executor
					.decrypt(secret)
					.map_err(|e| {
						Error::AssertionCreatedHandling(format!(
							"Could not decrypt secret, reason: {:?}",
							e
						))
					})?
					.as_slice(),
			)
			.map_err(|e| {
				Error::AssertionCreatedHandling(format!("Could not decode secret, reason: {:?}", e))
			})?;
			decrypted_secrets.push(secret);
		}
		self.assertion_repository
			.save(id, (byte_code, decrypted_secrets))
			.map_err(Error::AssertionCreatedHandling)?;
		Ok(())
	}
}

impl<Executor> HandleParentchainEvents<Executor, TrustedCallSigned, Error>
	for ParentchainEventHandler
where
	Executor: IndirectExecutor<TrustedCallSigned, Error>,
{
	fn handle_events(
		&self,
		executor: &Executor,
		events: impl FilterEvents,
	) -> Result<Vec<H256>, Error> {
		let mut handled_events: Vec<H256> = Vec::new();
		if let Ok(events) = events.get_link_identity_events() {
			debug!("Handling link_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found link_identity_event: {}", event);
					let result = Self::link_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
						event.encrypted_validation_data.clone(),
						event.encrypted_web3networks.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::LinkIdentityFailure)?;
		}

		if let Ok(events) = events.get_deactivate_identity_events() {
			debug!("Handling deactivate_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found deactivate_identity_event: {}", event);
					let result = Self::deactivate_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::DeactivateIdentityFailure)?;
		}

		if let Ok(events) = events.get_activate_identity_events() {
			debug!("Handling activate_identity events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found activate_identity_event: {}", event);
					let result = Self::activate_identity(
						executor,
						&event.account,
						event.encrypted_identity.clone(),
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::ActivateIdentityFailure)?;
		}

		if let Ok(events) = events.get_vc_requested_events() {
			debug!("Handling VCRequested events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found VCRequested event: {}", event);
					let result =
						Self::request_vc(executor, &event.account, event.assertion.clone());
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::VCRequestedFailure)?;
		}

		if let Ok(events) = events.get_scheduled_enclave_set_events() {
			debug!("Handling ScheduledEnclaveSet events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found ScheduledEnclaveSet event: {:?}", event);
					let result = Self::set_scheduled_enclave(
						event.worker_type,
						event.sidechain_block_number,
						event.mrenclave,
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::ScheduledEnclaveSetFailure)?;
		}

		if let Ok(events) = events.get_scheduled_enclave_removed_events() {
			debug!("Handling ScheduledEnclaveRemoved events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found ScheduledEnclaveRemoved event: {:?}", event);
					let result = Self::remove_scheduled_enclave(
						event.worker_type,
						event.sidechain_block_number,
					);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::ScheduledEnclaveRemovedFailure)?;
		}

		if let Ok(events) = events.get_opaque_task_posted_events() {
			debug!("Handling OpaqueTaskPosted events");
			events
				.iter()
				.try_for_each(|event| {
					debug!("found OpaqueTaskPosted event: {:?}", event);
					let result = Self::post_opaque_task(executor, &event.request);
					handled_events.push(hash_of(&event));

					result
				})
				.map_err(|_| ParentchainEventProcessingError::OpaqueTaskPostedFailure)?;
		}

		if let Ok(events) = events.get_assertion_created_events() {
			debug!("Handling AssertionCreated events");
			events
				.into_iter()
				.try_for_each(|event| {
					debug!("found AssertionCreated event: {:?}", event);
					let event_hash = hash_of(&event);
					let result =
						self.store_assertion(executor, event.id, event.byte_code, event.secrets);
					handled_events.push(event_hash);
					result
				})
				.map_err(|_| ParentchainEventProcessingError::AssertionCreatedFailure)?;
		}

		Ok(handled_events)
	}
}

fn hash_of<T: Encode>(ev: &T) -> H256 {
	blake2_256(&ev.encode()).into()
}

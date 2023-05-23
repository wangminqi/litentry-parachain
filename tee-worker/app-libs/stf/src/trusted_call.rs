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

#[cfg(feature = "evm")]
use sp_core::{H160, U256};

#[cfg(feature = "evm")]
use std::vec::Vec;

use crate::{
	helpers::ensure_enclave_signer_account, MetadataOf, Runtime, StfError, System, TrustedOperation,
};
use codec::{Decode, Encode};
use frame_support::{ensure, traits::UnfilteredDispatchable};
pub use ita_sgx_runtime::{Balance, ConvertAccountId, Index, SgxParentchainTypeConverter};
pub use itp_node_api::metadata::{
	pallet_imp::IMPCallIndexes, pallet_system::SystemSs58Prefix, pallet_teerex::TeerexCallIndexes,
	pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata,
};
use itp_stf_interface::ExecuteCall;
use itp_stf_primitives::types::{AccountId, KeyPair, ShardIdentifier, Signature};
pub use itp_types::{OpaqueCall, H256};
use itp_utils::stringify::account_id_to_string;
pub use litentry_primitives::{
	aes_encrypt_default, AesOutput, Assertion, ChallengeCode, ErrorDetail, IMPError, Identity,
	ParentchainAccountId, ParentchainBlockNumber, UserShieldingKeyType, VCMPError, ValidationData,
};
use log::*;
use sp_io::hashing::blake2_256;
use sp_runtime::{traits::Verify, MultiAddress};
use std::{format, prelude::v1::*, sync::Arc};

#[cfg(feature = "evm")]
use ita_sgx_runtime::{AddressMapping, HashedAddressMapping};
use itp_storage::storage_value_key;
use itp_types::{MrEnclave, SidechainBlockNumber};
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};

#[cfg(feature = "evm")]
use crate::evm_helpers::{create_code_hash, evm_create2_address, evm_create_address};

// max number of identities in an id_graph that will be returned as the extrinsic parameter
// this has no effect on the stored id_graph, but only the returned id_graph
pub const RETURNED_IDGRAPH_MAX_LEN: usize = 20;

pub type IMTCall = ita_sgx_runtime::IdentityManagementCall<Runtime>;
pub type IMT = ita_sgx_runtime::pallet_imt::Pallet<Runtime>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	balance_set_balance(AccountId, AccountId, Balance, Balance),
	balance_transfer(AccountId, AccountId, Balance),
	balance_unshield(AccountId, AccountId, Balance, ShardIdentifier), // (AccountIncognito, BeneficiaryPublicAccount, Amount, Shard)
	balance_shield(AccountId, AccountId, Balance), // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	evm_withdraw(AccountId, H160, Balance), // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_call(
		AccountId,
		H160,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create(
		AccountId,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Salt, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create2(
		AccountId,
		H160,
		Vec<u8>,
		H256,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// litentry
	set_user_shielding_key(AccountId, AccountId, UserShieldingKeyType, H256),
	create_identity(
		AccountId,
		AccountId,
		Identity,
		Option<MetadataOf<Runtime>>,
		ParentchainBlockNumber,
		H256,
	),
	remove_identity(AccountId, AccountId, Identity, H256),
	verify_identity(AccountId, AccountId, Identity, ValidationData, ParentchainBlockNumber, H256),
	request_vc(AccountId, AccountId, Assertion, H256),
	// the following trusted calls should not be requested directly from external
	// they are guarded by the signature check (either root or enclave_signer_account)
	verify_identity_callback(AccountId, AccountId, Identity, ParentchainBlockNumber, H256),
	request_vc_callback(AccountId, AccountId, Assertion, [u8; 32], [u8; 32], Vec<u8>, H256),
	handle_imp_error(AccountId, Option<AccountId>, IMPError, H256),
	handle_vcmp_error(AccountId, Option<AccountId>, VCMPError, H256),
	set_challenge_code(AccountId, AccountId, Identity, ChallengeCode, H256),
	send_erroneous_parentchain_call(AccountId),
	set_scheduled_mrenclave(AccountId, SidechainBlockNumber, MrEnclave),
}

impl TrustedCall {
	pub fn sender_account(&self) -> &AccountId {
		match self {
			TrustedCall::balance_set_balance(sender_account, ..) => sender_account,
			TrustedCall::balance_transfer(sender_account, ..) => sender_account,
			TrustedCall::balance_unshield(sender_account, ..) => sender_account,
			TrustedCall::balance_shield(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(sender_account, ..) => sender_account,
			// litentry
			TrustedCall::set_user_shielding_key(account, ..) => account,
			TrustedCall::create_identity(account, ..) => account,
			TrustedCall::remove_identity(account, ..) => account,
			TrustedCall::verify_identity(account, ..) => account,
			TrustedCall::request_vc(account, ..) => account,
			TrustedCall::verify_identity_callback(account, ..) => account,
			TrustedCall::request_vc_callback(account, ..) => account,
			TrustedCall::set_challenge_code(account, ..) => account,
			TrustedCall::handle_imp_error(account, ..) => account,
			TrustedCall::handle_vcmp_error(account, ..) => account,
			TrustedCall::send_erroneous_parentchain_call(account) => account,
			TrustedCall::set_scheduled_mrenclave(account, ..) => account,
		}
	}

	pub fn sign(
		&self,
		pair: &KeyPair,
		nonce: Index,
		mrenclave: &[u8; 32],
		shard: &ShardIdentifier,
	) -> TrustedCallSigned {
		let mut payload = self.encode();
		payload.append(&mut nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		TrustedCallSigned { call: self.clone(), nonce, signature: pair.sign(payload.as_slice()) }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedCallSigned {
	pub call: TrustedCall,
	pub nonce: Index,
	pub signature: Signature,
}

impl TrustedCallSigned {
	pub fn new(call: TrustedCall, nonce: Index, signature: Signature) -> Self {
		TrustedCallSigned { call, nonce, signature }
	}

	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());
		self.signature.verify(payload.as_slice(), self.call.sender_account())
	}

	pub fn into_trusted_operation(self, direct: bool) -> TrustedOperation {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}

// TODO: #91 signed return value
/*
pub struct TrustedReturnValue<T> {
	pub value: T,
	pub signer: AccountId
}

impl TrustedReturnValue
*/

impl<NodeMetadataRepository> ExecuteCall<NodeMetadataRepository> for TrustedCallSigned
where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
{
	type Error = StfError;

	// TODO(Kai@litentry):
	// If this function returns Err(), it will feed the executor with Ok(ExecutedOperation::failed()),
	// which will remove the failed op from its **own** top pool while preventing it from being included
	// in a sidechain block - see `execute_trusted_call_on_stf`.
	//
	// As a result, when other workers import sidechain blocks, they will treat the op as
	// "not yet executed" (before it's not recorded in the sidechain block) and try to execute it again from
	// its own top pool (if the op is added to the top pool upon e.g. parentchain block import).
	//
	// The execution will most likely fail again. However, the state could have been changed already by applying
	// the state diff from the imported sidechain block. This could cause an inconsistent/mismatching state,
	// for example, the nonce. See the nonce handling below: we increased the nonce no matter the STF is executed
	// successfully or not.
	//
	// This is probably the reason why the nonce-handling test in `demo_shielding_unshielding.sh` sometimes fails.
	//
	// for now we should always return Ok(()) for this function and propagate the exe, at least for
	// litentry STFs. I believe this is the right way to go, but it still needs more discussions.
	//
	// Update:
	// see discussion in https://github.com/integritee-network/worker/issues/1232
	// my current thoughts are:
	// - we should return Err() if the STF execution fails
	// - the failed top should be removed from the tool
	// - however, the failed top hash needs to be included in the sidechain block
	fn execute(
		self,
		shard: &ShardIdentifier,
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<(), Self::Error> {
		let sender = self.call.sender_account().clone();
		let call_hash = blake2_256(&self.call.encode());
		let system_nonce = System::account_nonce(&sender);
		ensure!(self.nonce == system_nonce, Self::Error::InvalidNonce(self.nonce, system_nonce));

		// increment the nonce, no matter if the call succeeds or fails.
		// The call must have entered the transaction pool already,
		// so it should be considered as valid
		System::inc_account_nonce(&sender);

		// TODO: maybe we can further simplify this by effacing the duplicate code
		match self.call {
			TrustedCall::balance_set_balance(root, who, free_balance, reserved_balance) => {
				ensure!(is_root::<Runtime, AccountId>(&root), Self::Error::MissingPrivileges(root));
				debug!(
					"balance_set_balance({}, {}, {})",
					account_id_to_string(&who),
					free_balance,
					reserved_balance
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
					who: MultiAddress::Id(who),
					new_free: free_balance,
					new_reserved: reserved_balance,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Set Balance error: {:?}", e.error))
				})?;
				// This explicit Error type is somehow still needed, otherwise the compiler complains
				// 	multiple `impl`s satisfying `StfError: std::convert::From<_>`
				// 		note: and another `impl` found in the `core` crate: `impl<T> std::convert::From<T> for T;`
				// the impl From<..> for StfError conflicts with the standard convert
				//
				// Alternatively, removing the customised "impl From<..> for StfError" and use map_err directly
				// would also work
				Ok::<(), Self::Error>(())
			},
			TrustedCall::balance_transfer(from, to, value) => {
				let origin = ita_sgx_runtime::RuntimeOrigin::signed(from.clone());
				debug!(
					"balance_transfer({}, {}, {})",
					account_id_to_string(&from),
					account_id_to_string(&to),
					value
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::transfer {
					dest: MultiAddress::Id(to),
					value,
				}
				.dispatch_bypass_filter(origin)
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Transfer error: {:?}", e.error))
				})?;
				Ok(())
			},
			TrustedCall::balance_unshield(account_incognito, beneficiary, value, shard) => {
				debug!(
					"balance_unshield({}, {}, {}, {})",
					account_id_to_string(&account_incognito),
					account_id_to_string(&beneficiary),
					value,
					shard
				);
				unshield_funds(account_incognito, value)?;
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.unshield_funds_call_indexes())??,
					beneficiary,
					value,
					shard,
					call_hash,
				)));
				Ok(())
			},
			TrustedCall::balance_shield(enclave_account, who, value) => {
				ensure_enclave_signer_account(&enclave_account)?;
				debug!("balance_shield({}, {})", account_id_to_string(&who), value);
				shield_funds(who, value)?;

				// Send proof of execution on chain.
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.publish_hash_call_indexes())??,
					call_hash,
					Vec::<itp_types::H256>::new(),
					b"shielded some funds!".to_vec(),
				)));
				Ok(())
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(from, address, value) => {
				debug!("evm_withdraw({}, {}, {})", account_id_to_string(&from), address, value);
				ita_sgx_runtime::EvmCall::<Runtime>::withdraw { address, value }
					.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(from))
					.map_err(|e| {
						Self::Error::Dispatch(format!("Evm Withdraw error: {:?}", e.error))
					})?;
				Ok(())
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(
				from,
				source,
				target,
				input,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_call(from: {}, source: {}, target: {})",
					account_id_to_string(&from),
					source,
					target
				);
				ita_sgx_runtime::EvmCall::<Runtime>::call {
					source,
					target,
					input,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Call error: {:?}", e.error)))?;
				Ok(())
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(
				from,
				source,
				init,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_create(from: {}, source: {}, value: {})",
					account_id_to_string(&from),
					source,
					value
				);
				let nonce_evm_account =
					System::account_nonce(&HashedAddressMapping::into_account_id(source));
				ita_sgx_runtime::EvmCall::<Runtime>::create {
					source,
					init,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create error: {:?}", e.error)))?;
				let contract_address = evm_create_address(source, nonce_evm_account);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(())
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(
				from,
				source,
				init,
				salt,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_create2(from: {}, source: {}, value: {})",
					account_id_to_string(&from),
					source,
					value
				);
				let code_hash = create_code_hash(&init);
				ita_sgx_runtime::EvmCall::<Runtime>::create2 {
					source,
					init,
					salt,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create2 error: {:?}", e.error)))?;
				let contract_address = evm_create2_address(source, salt, code_hash);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(())
			},
			// litentry trusted calls
			// the reason that most calls have an internal handling fn is that we want to capture the error and
			// handle it here to be able to send error events to the parachain
			TrustedCall::set_user_shielding_key(signer, who, key, hash) => {
				debug!("set_user_shielding_key, who: {}", account_id_to_string(&who));
				let parent_ss58_prefix =
					node_metadata_repo.get_from_metadata(|m| m.system_ss58_prefix())??;
				let account = SgxParentchainTypeConverter::convert(who.clone());
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.user_shielding_key_set_call_indexes())??;

				match Self::set_user_shielding_key_internal(
					signer,
					who.clone(),
					key,
					parent_ss58_prefix,
				) {
					Ok(key) => {
						debug!("pushing user_shielding_key_set event ...");
						let id_graph =
							IMT::get_id_graph_with_max_len(&who, RETURNED_IDGRAPH_MAX_LEN);
						calls.push(OpaqueCall::from_tuple(&(
							call_index,
							account,
							aes_encrypt_default(&key, &id_graph.encode()),
							hash,
						)));
					},
					Err(e) => {
						debug!("pushing error event ... error: {}", e);
						add_call_from_imp_error(
							calls,
							node_metadata_repo,
							Some(account),
							e.to_imp_error(),
							hash,
						);
					},
				}
				Ok(())
			},
			TrustedCall::create_identity(signer, who, identity, metadata, bn, hash) => {
				debug!("create_identity, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(who.clone());
				let parent_ss58_prefix =
					node_metadata_repo.get_from_metadata(|m| m.system_ss58_prefix())??;
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.identity_created_call_indexes())??;

				match Self::create_identity_internal(
					signer,
					who,
					identity.clone(),
					metadata,
					bn,
					parent_ss58_prefix,
				) {
					Ok((key, code)) => {
						debug!("pushing identity_created event ...");
						calls.push(OpaqueCall::from_tuple(&(
							call_index,
							account,
							aes_encrypt_default(&key, &identity.encode()),
							aes_encrypt_default(&key, &code.encode()),
							hash,
						)));
					},
					Err(e) => {
						debug!("pushing error event ... error: {}", e);
						add_call_from_imp_error(
							calls,
							node_metadata_repo,
							Some(account),
							e.to_imp_error(),
							hash,
						);
					},
				}
				Ok(())
			},
			TrustedCall::remove_identity(signer, who, identity, hash) => {
				debug!("remove_identity, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(who.clone());
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.identity_removed_call_indexes())??;

				match Self::remove_identity_internal(signer, who, identity.clone()) {
					Ok(key) => {
						debug!("pushing identity_removed event ...");
						calls.push(OpaqueCall::from_tuple(&(
							call_index,
							account,
							aes_encrypt_default(&key, &identity.encode()),
							hash,
						)));
					},
					Err(e) => {
						debug!("pushing error event ... error: {}", e);
						add_call_from_imp_error(
							calls,
							node_metadata_repo,
							Some(account),
							e.to_imp_error(),
							hash,
						);
					},
				}
				Ok(())
			},
			TrustedCall::verify_identity(signer, who, identity, validation_data, bn, hash) => {
				debug!("verify_identity, who: {}", account_id_to_string(&who));
				if let Err(e) = Self::verify_identity_internal(
					signer,
					who.clone(),
					identity,
					validation_data,
					bn,
					hash,
					shard,
				) {
					debug!("pushing error event ... error: {}", e);
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(SgxParentchainTypeConverter::convert(who)),
						e.to_imp_error(),
						hash,
					);
				}
				Ok(())
			},
			TrustedCall::verify_identity_callback(signer, who, identity, bn, hash) => {
				debug!("verify_identity_callback, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(who.clone());
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.identity_verified_call_indexes())??;

				match Self::verify_identity_callback_internal(
					signer,
					who.clone(),
					identity.clone(),
					bn,
				) {
					Ok(key) => {
						let id_graph =
							IMT::get_id_graph_with_max_len(&who, RETURNED_IDGRAPH_MAX_LEN);
						debug!("pushing identity_verified event ...");
						calls.push(OpaqueCall::from_tuple(&(
							call_index,
							account,
							aes_encrypt_default(&key, &identity.encode()),
							aes_encrypt_default(&key, &id_graph.encode()),
							hash,
						)));
					},
					Err(e) => {
						debug!("pushing error event ... error: {}", e);
						add_call_from_imp_error(
							calls,
							node_metadata_repo,
							Some(account),
							e.to_imp_error(),
							hash,
						);
					},
				}

				Ok(())
			},
			TrustedCall::request_vc(signer, who, assertion, hash) => {
				debug!(
					"request_vc, who: {}, assertion: {:?}",
					account_id_to_string(&who),
					assertion
				);
				if let Err(e) =
					Self::request_vc_internal(signer, who.clone(), assertion, hash, shard)
				{
					debug!("pushing error event ... error: {}", e);
					add_call_from_vcmp_error(
						calls,
						node_metadata_repo,
						Some(SgxParentchainTypeConverter::convert(who)),
						e.to_vcmp_error(),
						hash,
					);
				}
				Ok(())
			},
			TrustedCall::request_vc_callback(
				signer,
				who,
				assertion,
				vc_index,
				vc_hash,
				vc_payload,
				hash,
			) => {
				debug!(
					"request_vc_callback, who: {}, assertion: {:?}",
					account_id_to_string(&who),
					assertion
				);
				let account = SgxParentchainTypeConverter::convert(who.clone());
				let call_index =
					node_metadata_repo.get_from_metadata(|m| m.vc_issued_call_indexes())??;

				match Self::request_vc_callback_internal(signer, who, assertion.clone()) {
					Ok(key) => {
						calls.push(OpaqueCall::from_tuple(&(
							call_index,
							account,
							assertion,
							vc_index,
							vc_hash,
							aes_encrypt_default(&key, &vc_payload),
							hash,
						)));
					},
					Err(e) => {
						debug!("pushing error event ... error: {}", e);
						add_call_from_vcmp_error(
							calls,
							node_metadata_repo,
							Some(account),
							e.to_vcmp_error(),
							hash,
						);
					},
				}
				Ok(())
			},
			TrustedCall::set_challenge_code(enclave_account, who, did, code, hash) => {
				if let Err(e) =
					Self::set_challenge_code_internal(enclave_account, who.clone(), did, code)
				{
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(SgxParentchainTypeConverter::convert(who)),
						e.to_imp_error(),
						hash,
					);
				}
				Ok(())
			},
			TrustedCall::handle_imp_error(_enclave_account, account, e, hash) => {
				// checking of `_enclave_account` is not strictly needed, as this trusted call can
				// only be constructed internally
				add_call_from_imp_error(calls, node_metadata_repo, account, e, hash);
				Ok(())
			},
			TrustedCall::handle_vcmp_error(_enclave_account, account, e, hash) => {
				// checking of `_enclave_account` is not strictly needed, as this trusted call can
				// only be constructed internally
				add_call_from_vcmp_error(calls, node_metadata_repo, account, e, hash);
				Ok(())
			},
			TrustedCall::send_erroneous_parentchain_call(account) => {
				// intentionally send wrong parameters, only used in testing
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.imp_some_error_call_indexes())??,
					"set_user_shielding_key".as_bytes(),
					account.encode(),
				)));
				Ok(())
			},
			TrustedCall::set_scheduled_mrenclave(account_id, bn, mrenclave) => {
				ensure!(
					is_root::<Runtime, AccountId>(&account_id),
					Self::Error::MissingPrivileges(account_id)
				);
				GLOBAL_SCHEDULED_ENCLAVE.update(bn, mrenclave).map_err(|e| {
					error!("Failed to set scheduled mr_enclave {:?}", e);
					StfError::SetScheduledMrEnclaveFailed
				})?;
				Ok(())
			},
		}?;
		Ok(())
	}

	fn get_storage_hashes_to_update(&self) -> Vec<Vec<u8>> {
		let mut key_hashes = Vec::new();
		key_hashes.push(storage_value_key("IdentityManagement","MaxIDGraphLength"));
		match self.call {
			TrustedCall::balance_set_balance(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_transfer(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_unshield(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_shield(..) => debug!("No storage updates needed..."),
			// litentry
			TrustedCall::set_user_shielding_key(..) => debug!("No storage updates needed..."),
			TrustedCall::create_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::remove_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::verify_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::verify_identity_callback(..) => debug!("No storage updates needed..."),
			TrustedCall::request_vc(..) => debug!("No storage updates needed..."),
			TrustedCall::set_challenge_code(..) => debug!("No storage updates needed..."),
			TrustedCall::request_vc_callback(..) => debug!("No storage updates needed..."),
			TrustedCall::handle_imp_error(..) => debug!("No storage updates needed..."),
			TrustedCall::handle_vcmp_error(..) => debug!("No storage updates needed..."),
			TrustedCall::send_erroneous_parentchain_call(..) =>
				debug!("No storage updates needed..."),
			TrustedCall::set_scheduled_mrenclave(..) => debug!("No storage updates needed..."),
			#[cfg(feature = "evm")]
			_ => debug!("No storage updates needed..."),
		};
		key_hashes
	}
}

fn unshield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	if account_info.data.free < amount {
		return Err(StfError::MissingFunds)
	}

	ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free - amount,
		new_reserved: account_info.data.reserved,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Unshield funds error: {:?}", e.error)))?;
	Ok(())
}

fn shield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free + amount,
		new_reserved: account_info.data.reserved,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Shield funds error: {:?}", e.error)))?;

	Ok(())
}

pub(crate) fn is_root<Runtime, AccountId>(account: &AccountId) -> bool
where
	Runtime: frame_system::Config<AccountId = AccountId> + pallet_sudo::Config,
	AccountId: PartialEq,
{
	pallet_sudo::Pallet::<Runtime>::key().map_or(false, |k| account == &k)
}

// helper method to create and push an `OpaqueCall` from a IMPError, this function always succeeds
pub fn add_call_from_imp_error<NodeMetadataRepository>(
	calls: &mut Vec<OpaqueCall>,
	node_metadata_repo: Arc<NodeMetadataRepository>,
	account: Option<ParentchainAccountId>,
	e: IMPError,
	hash: H256,
) where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
{
	debug!("pushing imp_some_error event ...");
	// TODO: anyway to simplify this? `and_then` won't be applicable here
	match node_metadata_repo.get_from_metadata(|m| m.imp_some_error_call_indexes()) {
		Ok(Ok(call_index)) => calls.push(OpaqueCall::from_tuple(&(call_index, account, e, hash))),
		Ok(e) => warn!("error getting IMP call indexes: {:?}", e),
		Err(e) => warn!("error getting IMP call indexes: {:?}", e),
	}
}

// helper method to create and push an `OpaqueCall` from a VCMPError, this function always succeeds
pub fn add_call_from_vcmp_error<NodeMetadataRepository>(
	calls: &mut Vec<OpaqueCall>,
	node_metadata_repo: Arc<NodeMetadataRepository>,
	account: Option<ParentchainAccountId>,
	e: VCMPError,
	hash: H256,
) where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType:
		TeerexCallIndexes + IMPCallIndexes + VCMPCallIndexes + SystemSs58Prefix,
{
	debug!("pushing vcmp_some_error event ...");
	match node_metadata_repo.get_from_metadata(|m| m.vcmp_some_error_call_indexes()) {
		Ok(Ok(call_index)) => calls.push(OpaqueCall::from_tuple(&(call_index, account, e, hash))),
		Ok(e) => warn!("error getting VCMP call indexes: {:?}", e),
		Err(e) => warn!("error getting VCMP call indexes: {:?}", e),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::KeyPair;
	use sp_keyring::AccountKeyring;

	#[test]
	fn verify_signature_works() {
		let nonce = 21;
		let mrenclave = [0u8; 32];
		let shard = ShardIdentifier::default();

		let call = TrustedCall::balance_set_balance(
			AccountKeyring::Alice.public().into(),
			AccountKeyring::Alice.public().into(),
			42,
			42,
		);
		let signed_call = call.sign(
			&KeyPair::Sr25519(Box::new(AccountKeyring::Alice.pair())),
			nonce,
			&mrenclave,
			&shard,
		);

		assert!(signed_call.verify_signature(&mrenclave, &shard));
	}
}

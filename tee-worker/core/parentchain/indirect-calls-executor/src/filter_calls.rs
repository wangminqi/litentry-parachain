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

use crate::{
	error::Result,
	indirect_calls::{
		BatchAllArgs, CallWorkerArgs, CreateIdentityArgs, RemoveIdentityArgs,
		RemoveScheduledEnclaveArgs, RequestVCArgs, SetUserShieldingKeyArgs, ShieldFundsArgs,
		UpdateScheduledEnclaveArgs, VerifyIdentityArgs,
	},
	parentchain_extrinsic_parser::ParseExtrinsic,
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use ita_sgx_runtime::Signature;
use itp_node_api::metadata::{NodeMetadata, NodeMetadataTrait};
use sp_core::blake2_256;
use substrate_api_client::{
	CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};

/// Trait to filter an indirect call and decode into it, where the decoding
/// is based on the metadata provided.
pub trait FilterCalls<NodeMetadata> {
	/// Call enum we try to decode into.
	type Call;

	/// Knows how to parse the parentchain extrinsics.
	type ParseParentchainExtrinsic;

	/// Filters some bytes and returns `Some(Self::Call)` if the filter matches some criteria.
	fn filter_into_with_metadata(call: &[u8], metadata: &NodeMetadata) -> Option<Self::Call>;
}

/// Indirect calls filter denying all indirect calls.
pub struct DenyAll;

impl FilterCalls<NodeMetadata> for DenyAll {
	type Call = ();
	type ParseParentchainExtrinsic = ();

	fn filter_into_with_metadata(_: &[u8], _: &NodeMetadata) -> Option<Self::Call> {
		None
	}
}

/// Default filter we use for the Integritee-Parachain.
pub struct ShieldFundsAndCallWorkerFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterCalls<NodeMetadata>
	for ShieldFundsAndCallWorkerFilter<ExtrinsicParser>
where
	ExtrinsicParser: ParseExtrinsic,
{
	type Call = IndirectCall;
	type ParseParentchainExtrinsic = ExtrinsicParser;

	fn filter_into_with_metadata(call: &[u8], metadata: &NodeMetadata) -> Option<Self::Call> {
		let call_mut = &mut &call[..];

		// Todo: the filter should not need to parse, only filter. This should directly be configured
		// in the indirect executor.
		let xt = match Self::ParseParentchainExtrinsic::parse(call_mut) {
			Ok(xt) => xt,
			Err(e) => {
				log::error!("Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};

		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		let signature = xt.signature;

		// let index = xt.function.0;
		// let signature = xt.signature;

		let xt_hash = blake2_256(&xt.encode()).into();

		if index == metadata.shield_funds_call_indexes().ok()? {
			let args = decode_and_log_error::<ShieldFundsArgs>(call_args)?;
			let (_, account_encrypted, amount, shard) = &xt.function;
			let args = ShieldFundsArgs { account_encrypted, amount, shard };
			Some(IndirectCall::ShieldFunds(args))
		} else if index == metadata.call_worker_call_indexes().ok()? {
			// let args = decode_and_log_error::<CallWorkerArgs>(call_args)?;
			let (_, request) = &xt.function;
			let args = CallWorkerArgs { request };
			Some(IndirectCall::CallWorker(args))
		} else if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			// let (_, (shard, encrypted_key)) = &xt.function;
			// let args = SetUserShieldingKeyArgs{shard, encrypted_key, xt};
			Some(IndirectCall::SetUserShieldingKey(args))
		} else if index == metadata.create_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<CreateIdentityArgs>(call_args)?;
			Some(IndirectCall::CreateIdentity(args, xt.xt))
		} else if index == metadata.remove_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveIdentityArgs>(call_args)?;
			Some(IndirectCall::RemoveIdentity(args, xt.xt))
		} else if index == metadata.verify_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<VerifyIdentityArgs>(call_args)?;
			Some(IndirectCall::VerifyIdentity(args, xt.xt))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			Some(IndirectCall::RequestVC(args, xt.xt))
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else {
			None
		}
	}
}

pub struct LitentryCallWorkerFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterCalls<NodeMetadata>
	for LitentryCallWorkerFilter<ExtrinsicParser>
where
	ExtrinsicParser: ParseExtrinsic,
{
	type Call = IndirectCall;
	type ParseParentchainExtrinsic = ExtrinsicParser;

	fn filter_into_with_metadata(call: &[u8], metadata: &NodeMetadata) -> Option<Self::Call> {
		let call_mut = &mut &call[..];

		// Todo: the filter should not need to parse, only filter. This should directly be configured
		// in the indirect executor.
		let xt = match Self::ParseParentchainExtrinsic::parse(call_mut) {
			Ok(xt) => xt,
			Err(e) => {
				log::error!("Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};

		let index = xt.xt.function.0;
		let call_args = &mut &xt.call_args[..];

		if index == metadata.set_user_shielding_key_call_indexes().ok()? {
			let args = decode_and_log_error::<SetUserShieldingKeyArgs>(call_args)?;
			Some(IndirectCall::SetUserShieldingKey(args, xt.xt))
		} else if index == metadata.create_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<CreateIdentityArgs>(call_args)?;
			Some(IndirectCall::CreateIdentity(args, xt.xt))
		} else if index == metadata.remove_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<RemoveIdentityArgs>(call_args)?;
			Some(IndirectCall::RemoveIdentity(args, xt.xt))
		} else if index == metadata.verify_identity_call_indexes().ok()? {
			let args = decode_and_log_error::<VerifyIdentityArgs>(call_args)?;
			Some(IndirectCall::VerifyIdentity(args, xt.xt))
		} else if index == metadata.request_vc_call_indexes().ok()? {
			let args = decode_and_log_error::<RequestVCArgs>(call_args)?;
			Some(IndirectCall::RequestVC(args, xt.xt))
		} else if index == metadata.update_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<UpdateScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::UpdateScheduledEnclave(args))
		} else if index == metadata.remove_scheduled_enclave().ok()? {
			let args = decode_and_log_error::<RemoveScheduledEnclaveArgs>(call_args)?;
			Some(IndirectCall::RemoveScheduledEnclave(args))
		} else {
			None
		}
	}
}
/// The default indirect call of the Integritee-Parachain.
///
/// Todo: Move or provide a template in app-libs such that users
/// can implemeent their own indirect call there.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	ShieldFunds(ShieldFundsArgs),
	CallWorker(CallWorkerArgs),
	SetUserShieldingKey(SetUserShieldingKeyArgs),
	CreateIdentity(CreateIdentityArgs),
	RemoveIdentity(RemoveIdentityArgs),
	VerifyIdentity(VerifyIdentityArgs),
	RequestVC(RequestVCArgs),
	UpdateScheduledEnclave(UpdateScheduledEnclaveArgs),
	RemoveScheduledEnclave(RemoveScheduledEnclaveArgs),
	// BatchAll(BatchAllArgs, UncheckedExtrinsicV4<CallIndex, SubstrateDefaultSignedExtra<PlainTip>>),
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for IndirectCall {
	fn dispatch(&self, executor: &Executor) -> Result<()> {
		match self {
			IndirectCall::ShieldFunds(args) => args.dispatch(executor),
			IndirectCall::CallWorker(args) => args.dispatch(executor),
			IndirectCall::SetUserShieldingKey(args, xt) => args.dispatch(executor),
			IndirectCall::CreateIdentity(args, xt) => args.dispatch(executor),
			IndirectCall::RemoveIdentity(args, xt) => args.dispatch(executor),
			IndirectCall::VerifyIdentity(args, xt) => args.dispatch(executor),
			IndirectCall::RequestVC(args, xt) => args.dispatch(executor),
			IndirectCall::UpdateScheduledEnclave(args) => args.dispatch(executor),
			IndirectCall::RemoveScheduledEnclave(args) => args.dispatch(executor),
		}
	}
}

fn decode_and_log_error<V: Decode>(encoded: &mut &[u8]) -> Option<V> {
	match V::decode(encoded) {
		Ok(v) => Some(v),
		Err(e) => {
			log::warn!("Could not decode. {:?}", e);
			None
		},
	}
}

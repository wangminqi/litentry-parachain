// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
	error::{Error, ErrorDetail, IMPError, Result},
	IndirectDispatch, IndirectExecutor,
};
use codec::{Decode, Encode};
use ita_stf::{TrustedCall, TrustedOperation};
use itp_types::{AccountId, ShardIdentifier, H256};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{Address, IdGraphIdentifier, Identity, ValidationData};
use log::debug;
use parachain_core_primitives::UserShieldingKeyNonceType;
use std::vec::Vec;
use substrate_api_client::GenericAddress;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct LinkIdentityArgs {
	shard: ShardIdentifier,
	account: AccountId,
	encrypted_identity: Vec<u8>,
	encrypted_validation_data: Vec<u8>,
	nonce: UserShieldingKeyNonceType,
}

impl LinkIdentityArgs {
	fn internal_dispatch<Executor: IndirectExecutor>(
		&self,
		executor: &Executor,
		address: Option<GenericAddress>,
		hash: H256,
	) -> Result<()> {
		let identity: Identity =
			Identity::decode(&mut executor.decrypt(&self.encrypted_identity)?.as_slice())?;
		let validation_data = ValidationData::decode(
			&mut executor.decrypt(&self.encrypted_validation_data)?.as_slice(),
		)?;

		if address.is_some() {
			debug!(
				"indirect call LinkIdentity, who:{:?}, keyNonce: {:?}, identity: {:?}, validation_data: {:?}",
				account_id_to_string(&self.account),
				self.nonce,
				identity,
				validation_data
			);

			let enclave_account_id = executor.get_enclave_account()?;
			let trusted_call = TrustedCall::link_identity(
				Address::Substrate(enclave_account_id.into()),
				IdGraphIdentifier::Substrate { address: self.account.clone().into() },
				identity,
				validation_data,
				self.nonce,
				hash,
			);
			let signed_trusted_call = executor.sign_call_with_self(&trusted_call, &self.shard)?;
			let trusted_operation = TrustedOperation::indirect_call(signed_trusted_call);

			let encrypted_trusted_call = executor.encrypt(&trusted_operation.encode())?;
			executor.submit_trusted_call(self.shard, encrypted_trusted_call);
		}
		Ok(())
	}
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for LinkIdentityArgs {
	type Args = (Option<GenericAddress>, H256, u32);
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		let (address, hash, _block) = args;
		let e = Error::IMPHandlingError(IMPError::LinkIdentityFailed(ErrorDetail::ImportError));
		if self.internal_dispatch(executor, address, hash).is_err() {
			if let Err(internal_e) =
				executor.submit_trusted_call_from_error(self.shard, None, &e, hash)
			{
				log::warn!("fail to handle internal errors in verify_identity: {:?}", internal_e);
			}
			return Err(e)
		}
		Ok(())
	}
}

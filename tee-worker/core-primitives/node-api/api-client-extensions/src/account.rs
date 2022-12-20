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

use crate::ApiResult;
use itp_types::RuntimeConfigCollection;
use pallet_balances::AccountData;
use sp_core::crypto::Pair;
use sp_runtime::MultiSignature;
use substrate_api_client::{
	rpc::{Request, Subscribe},
	AccountInfo, Api, ExtrinsicParams, GetAccountInformation,
};

/// ApiClient extension that contains some convenience methods around accounts.
pub trait AccountApi<Runtime: RuntimeConfigCollection> {
	fn get_nonce_of(&self, who: &Runtime::AccountId) -> ApiResult<Runtime::Index>;
	fn get_free_balance(&self, who: &Runtime::AccountId) -> ApiResult<Runtime::Balance>;
}

impl<P: Pair, Client, Params, Runtime> AccountApi<Runtime> for Api<P, Client, Params, Runtime>
where
	MultiSignature: From<P::Signature>,
	Client: Request + Subscribe,
	Params: ExtrinsicParams<Runtime::Index, Runtime::Hash>,
	Runtime: RuntimeConfigCollection,
{
	fn get_nonce_of(&self, who: &Runtime::AccountId) -> ApiResult<Runtime::Index> {
		Ok(self.get_account_info(who)?.map_or_else(
			|| AccountInfo::<Runtime::Index, Runtime::AccountData>::default().nonce,
			|info| info.nonce,
		))
	}

	fn get_free_balance(&self, who: &Runtime::AccountId) -> ApiResult<Runtime::Balance> {
		Ok(self
			.get_account_data(who)?
			.map_or_else(|| AccountData::default().free, |data| data.free))
	}
}

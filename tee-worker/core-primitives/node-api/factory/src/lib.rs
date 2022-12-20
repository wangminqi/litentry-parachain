/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

use itp_api_client_types::{ParentchainApi, PlainTip, WsRpcClient};
use itp_types::RuntimeConfigCollection;
use sp_core::sr25519;
use substrate_api_client::FromHexString;
/// Trait to create a node API, based on a node URL and signer.
pub trait CreateNodeApi<Runtime>
where
	Runtime: RuntimeConfigCollection,
	u128: From<PlainTip<Runtime::Balance>>,
{
	fn create_api(&self) -> Result<ParentchainApi<Runtime>>;
}

/// Node API factory error.
#[derive(Debug, thiserror::Error)]
pub enum NodeApiFactoryError {
	#[error("Failed to create a node API: {0}")]
	FailedToCreateNodeApi(#[from] itp_api_client_types::ApiClientError),
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
}

pub type Result<T> = std::result::Result<T, NodeApiFactoryError>;

/// Node API factory implementation.
pub struct NodeApiFactory {
	node_url: String,
	signer: sr25519::Pair,
}

impl NodeApiFactory {
	pub fn new(url: String, signer: sr25519::Pair) -> Self {
		NodeApiFactory { node_url: url, signer }
	}
}

impl<Runtime> CreateNodeApi<Runtime> for NodeApiFactory
where
	Runtime: RuntimeConfigCollection,
	u128: From<PlainTip<Runtime::Balance>>,
	Runtime::Hash: FromHexString,
{
	fn create_api(&self) -> Result<ParentchainApi<Runtime>> {
		let client = WsRpcClient::new(self.node_url.as_str(), 200).map_err(|e| {
			NodeApiFactoryError::FailedToCreateNodeApi(
				itp_api_client_types::ApiClientError::RpcClient(e),
			)
		})?;
		ParentchainApi::new(client)
			.map_err(NodeApiFactoryError::FailedToCreateNodeApi)
			.map(|mut a| {
				a.set_signer(self.signer.clone());
				a
			})
	}
}

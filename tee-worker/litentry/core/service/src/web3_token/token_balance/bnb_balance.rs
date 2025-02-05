// Copyright 2020-2024 Trust Computing GmbH.
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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use core::result::Result;

use lc_common::web3_token::{TokenAddress, TokenDecimals};
use lc_data_providers::nodereal_jsonrpc::{
	EthBalance, FungibleApiList, GetTokenBalance20Param, Web3NetworkNoderealJsonrpcClient,
};

use crate::*;

use super::common::calculate_balance_with_decimals;

pub fn get_balance(
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<f64, Error> {
	let mut total_balance = 0_f64;

	for address in addresses.iter() {
		let network = address.0;

		match network {
			Web3Network::Bsc | Web3Network::Ethereum => {
				let decimals = Web3TokenType::Bnb.get_decimals(network);
				if let Some(mut client) =
					network.create_nodereal_jsonrpc_client(data_provider_config)
				{
					let result = if network == Web3Network::Bsc {
						client.get_balance(address.1.as_str(), false)
					} else {
						let param = GetTokenBalance20Param {
							contract_address: Web3TokenType::Bnb
								.get_token_address(network)
								.unwrap_or_default()
								.into(),
							address: address.1.clone(),
							block_number: "latest".into(),
						};
						client.get_token_balance_20(&param, false)
					};

					match result {
						Ok(balance) => {
							total_balance += calculate_balance_with_decimals(balance, decimals);
						},
						Err(err) => return Err(err.into_error_detail()),
					}
				}
			},
			_ => {},
		}
	}

	Ok(total_balance)
}

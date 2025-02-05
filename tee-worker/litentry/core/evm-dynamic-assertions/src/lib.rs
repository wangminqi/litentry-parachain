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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use http_sgx as http;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::precompiles::Precompiles;
use ethabi::{
	decode, encode,
	ethereum_types::{H160, U256},
	ParamType, Token,
};
use evm::{
	backend::{MemoryBackend, MemoryVicinity},
	executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
	Config,
};
use lc_dynamic_assertion::{
	AssertionExecutor, AssertionLogicRepository, AssertionResult, Identity, IdentityNetworkTuple,
	Web3Network,
};
use std::{
	collections::BTreeMap,
	string::{String, ToString},
	sync::Arc,
	vec,
	vec::Vec,
};

mod precompiles;
pub mod repository;
pub mod sealing;

// feature guard ?
pub mod mock;

pub use itp_settings::files::ASSERTIONS_FILE;

pub type AssertionId = H160;
pub type SmartContractByteCode = Vec<u8>;
pub type AssertionRepositoryItem = (SmartContractByteCode, Vec<String>);

pub struct EvmAssertionExecutor<A: AssertionLogicRepository> {
	pub assertion_repository: Arc<A>,
}

impl<A: AssertionLogicRepository<Id = H160, Item = AssertionRepositoryItem>>
	AssertionExecutor<AssertionId> for EvmAssertionExecutor<A>
{
	fn execute(
		&self,
		assertion_id: A::Id,
		identities: &[IdentityNetworkTuple],
	) -> Result<AssertionResult, String> {
		let (smart_contract_byte_code, secrets) = self
			.assertion_repository
			.get(&assertion_id)
			.map_err(|_| "Could not access assertion repository")?
			.ok_or("Assertion not found")?;
		let input = prepare_execute_call_input(identities, secrets)
			.map_err(|_| "Could not prepare evm execution input")?;

		// prepare EVM runtime
		let config = prepare_config();
		let vicinity = prepare_memory();
		let state = BTreeMap::new();
		let mut backend = MemoryBackend::new(&vicinity, state);
		let metadata = StackSubstateMetadata::new(u64::MAX, &config);
		let state = MemoryStackState::new(metadata, &mut backend);
		let precompiles = Precompiles {};
		let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

		// caller, just an unused account
		let caller = hash(5); //0x05

		// deploy assertion smart contract
		let address = executor.create_address(evm::CreateScheme::Legacy { caller });
		let _create_result = executor.transact_create(
			caller,
			U256::zero(),
			smart_contract_byte_code,
			u64::MAX,
			Vec::new(),
		);

		// call assertion smart contract
		let call_result =
			executor.transact_call(caller, address, U256::zero(), input, u64::MAX, Vec::new());

		let (description, assertion_type, assertions, schema_url, meet) =
			decode_result(&call_result.1)
				.map_err(|_| "Could not decode evm assertion execution result")?;

		Ok(AssertionResult { description, assertion_type, assertions, schema_url, meet })
	}
}

fn prepare_config() -> Config {
	let mut config = Config::frontier();
	config.has_bitwise_shifting = true;
	config.err_on_call_with_more_gas = false;
	config
}

fn prepare_memory() -> MemoryVicinity {
	MemoryVicinity {
		gas_price: U256::zero(),
		origin: H160::default(),
		block_hashes: Vec::new(),
		block_number: Default::default(),
		block_coinbase: Default::default(),
		block_timestamp: Default::default(),
		block_difficulty: Default::default(),
		block_gas_limit: Default::default(),
		chain_id: U256::one(),
		block_base_fee_per_gas: U256::zero(),
		block_randomness: None,
	}
}

fn prepare_execute_call_input(
	identities: &[IdentityNetworkTuple],
	secrets: Vec<String>,
) -> Result<Vec<u8>, ()> {
	let identities: Vec<Token> = identities.iter().map(identity_with_networks_to_token).collect();
	let secrets: Vec<Token> = secrets.iter().map(secret_to_token).collect();
	let input = encode(&[Token::Array(identities), Token::Array(secrets)]);
	// hash of function to be called, all assertions contracts must have a function with this hash, signature:
	// function execute(Identity[] memory identities, string[] memory secrets)
	let function_hash = "e2561846";
	prepare_function_call_input(function_hash, input)
}

pub fn identity_with_networks_to_token(identity: &IdentityNetworkTuple) -> Token {
	let (type_index, value) = match &identity.0 {
		Identity::Twitter(str) => (0, str.inner_ref().to_vec()),
		Identity::Discord(str) => (1, str.inner_ref().to_vec()),
		Identity::Github(str) => (2, str.inner_ref().to_vec()),
		Identity::Substrate(addr) => (3, addr.as_ref().to_vec()),
		Identity::Evm(addr) => (4, addr.as_ref().to_vec()),
		Identity::Bitcoin(addr) => (5, addr.as_ref().to_vec()),
		Identity::Solana(addr) => (6, addr.as_ref().to_vec()),
	};
	let networks: Vec<Token> = identity.1.iter().map(network_to_token).collect();
	Token::Tuple(vec![Token::Uint(type_index.into()), Token::Bytes(value), Token::Array(networks)])
}

pub fn secret_to_token(secret: &String) -> Token {
	Token::String(secret.to_string())
}

pub fn network_to_token(network: &Web3Network) -> Token {
	Token::Uint(
		match network {
			Web3Network::Polkadot => 0,
			Web3Network::Kusama => 1,
			Web3Network::Litentry => 2,
			Web3Network::Litmus => 3,
			Web3Network::LitentryRococo => 4,
			Web3Network::Khala => 5,
			Web3Network::SubstrateTestnet => 6,
			Web3Network::Ethereum => 7,
			Web3Network::Bsc => 8,
			Web3Network::BitcoinP2tr => 9,
			Web3Network::BitcoinP2pkh => 10,
			Web3Network::BitcoinP2sh => 11,
			Web3Network::BitcoinP2wpkh => 12,
			Web3Network::BitcoinP2wsh => 13,
			Web3Network::Polygon => 14,
			Web3Network::Arbitrum => 15,
			Web3Network::Solana => 16,
		}
		.into(),
	)
}

fn prepare_function_call_input(function_hash: &str, mut input: Vec<u8>) -> Result<Vec<u8>, ()> {
	let mut call_input = hex::decode(function_hash).map_err(|_| ())?;
	call_input.append(&mut input);
	Ok(call_input)
}

fn decode_result(data: &[u8]) -> Result<(String, String, Vec<String>, String, bool), ()> {
	let types = vec![
		ParamType::String,
		ParamType::String,
		ParamType::Array(ParamType::String.into()),
		ParamType::String,
		ParamType::Bool,
	];
	let decoded = decode(&types, data).map_err(|_| ())?;
	Ok((
		decoded[0].clone().into_string().ok_or(())?,
		decoded[1].clone().into_string().ok_or(())?,
		{
			let arr = decoded[2].clone().into_array().ok_or(())?;

			let mut assertions: Vec<String> = Vec::with_capacity(arr.len());

			for assertion in arr.into_iter() {
				assertions.push(assertion.into_string().ok_or(())?);
			}

			assertions
		},
		decoded[3].clone().into_string().ok_or(())?,
		decoded[4].clone().into_bool().ok_or(())?,
	))
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[cfg(test)]
mod tests {
	use super::*;
	use litentry_primitives::{Address32, Identity};

	#[test]
	pub fn should_tokenize_identity_with_networks() {
		// given
		let identity = Identity::Substrate(Address32::from([0u8; 32]));
		let networks = vec![Web3Network::Polkadot, Web3Network::Litentry];

		// when
		let token = identity_with_networks_to_token(&(identity, networks));

		// then
		match token {
			Token::Tuple(tokens) => {
				assert_eq!(tokens.len(), 3);
				match tokens.get(0).unwrap() {
					Token::Uint(value) => {
						assert_eq!(value, &Into::<U256>::into(3))
					},
					_ => panic!("Expected Token::Uint"),
				};
				match tokens.get(1).unwrap() {
					Token::Bytes(value) => {
						assert_eq!(value, &[0u8; 32].to_vec())
					},
					_ => panic!("Expected Token::Bytes"),
				}
				match tokens.get(2).unwrap() {
					Token::Array(network_tokens) => {
						assert_eq!(network_tokens.len(), 2);
						match network_tokens.get(0).unwrap() {
							Token::Uint(value) => {
								assert_eq!(value, &Into::<U256>::into(0))
							},
							_ => panic!("Expected Token::Uint"),
						}
						match network_tokens.get(1).unwrap() {
							Token::Uint(value) => {
								assert_eq!(value, &Into::<U256>::into(2))
							},
							_ => panic!("Expected Token::Uint"),
						}
					},
					_ => panic!("Expected Token::Array"),
				}
			},
			_ => panic!("Expected Token::Tuple"),
		}
	}
}

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

//! Autogenerated weights for `pallet_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-17, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_bridge
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge::WeightInfo for WeightInfo<T> {
	/// Storage: ChainBridge RelayerThreshold (r:0 w:1)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	fn set_threshold() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_564_000 picoseconds.
		Weight::from_parts(10_815_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Resources (r:0 w:1)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	fn set_resource() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_196_000 picoseconds.
		Weight::from_parts(5_398_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Resources (r:0 w:1)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	fn remove_resource() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_483_000 picoseconds.
		Weight::from_parts(4_720_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge ChainNonces (r:1 w:1)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	fn whitelist_chain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3607`
		// Minimum execution time: 15_881_000 picoseconds.
		Weight::from_parts(16_194_000, 0)
			.saturating_add(Weight::from_parts(0, 3607))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:1)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:1)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn add_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `142`
		//  Estimated: `3607`
		// Minimum execution time: 18_490_000 picoseconds.
		Weight::from_parts(18_980_000, 0)
			.saturating_add(Weight::from_parts(0, 3607))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ChainBridge Relayers (r:1 w:1)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:1)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `223`
		//  Estimated: `3688`
		// Minimum execution time: 20_912_000 picoseconds.
		Weight::from_parts(21_280_000, 0)
			.saturating_add(Weight::from_parts(0, 3688))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ChainBridge BridgeFee (r:0 w:1)
	/// Proof Skipped: ChainBridge BridgeFee (max_values: None, max_size: None, mode: Measured)
	fn update_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_920_000 picoseconds.
		Weight::from_parts(12_729_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:0)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge ChainNonces (r:1 w:0)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Resources (r:1 w:0)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn acknowledge_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `3805`
		// Minimum execution time: 52_306_000 picoseconds.
		Weight::from_parts(53_301_000, 0)
			.saturating_add(Weight::from_parts(0, 3805))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Relayers (r:1 w:0)
	/// Proof Skipped: ChainBridge Relayers (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge ChainNonces (r:1 w:0)
	/// Proof Skipped: ChainBridge ChainNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Resources (r:1 w:0)
	/// Proof Skipped: ChainBridge Resources (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `3805`
		// Minimum execution time: 44_342_000 picoseconds.
		Weight::from_parts(45_477_000, 0)
			.saturating_add(Weight::from_parts(0, 3805))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ChainBridge Votes (r:1 w:1)
	/// Proof Skipped: ChainBridge Votes (max_values: None, max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerThreshold (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerThreshold (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ChainBridge RelayerCount (r:1 w:0)
	/// Proof Skipped: ChainBridge RelayerCount (max_values: Some(1), max_size: None, mode: Measured)
	fn eval_vote_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `3949`
		// Minimum execution time: 19_406_000 picoseconds.
		Weight::from_parts(19_841_000, 0)
			.saturating_add(Weight::from_parts(0, 3949))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

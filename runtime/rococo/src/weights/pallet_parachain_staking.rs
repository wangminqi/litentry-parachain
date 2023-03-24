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

//! Autogenerated weights for `pallet_parachain_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `25`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_parachain_staking
// --extrinsic=*
// --heap-pages=4096
// --steps=25
// --repeat=20
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_parachain_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking Candidates (r:1 w:1)
	/// The range of component `x` is `[1, 100]`.
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		// Minimum execution time: 24_187 nanoseconds.
		Weight::from_ref_time(30_614_151)
			// Standard Error: 12_673
			.saturating_add(Weight::from_ref_time(294_915).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking Candidates (r:1 w:1)
	/// The range of component `x` is `[1, 100]`.
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		// Minimum execution time: 24_150 nanoseconds.
		Weight::from_ref_time(28_982_869)
			// Standard Error: 9_330
			.saturating_add(Weight::from_ref_time(289_211).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		// Minimum execution time: 25_311 nanoseconds.
		Weight::from_ref_time(26_147_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		// Minimum execution time: 79_907 nanoseconds.
		Weight::from_ref_time(81_481_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		// Minimum execution time: 25_416 nanoseconds.
		Weight::from_ref_time(26_311_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		// Minimum execution time: 25_063 nanoseconds.
		Weight::from_ref_time(26_148_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_total_selected() -> Weight {
		// Minimum execution time: 25_970 nanoseconds.
		Weight::from_ref_time(27_192_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		// Minimum execution time: 23_148 nanoseconds.
		Weight::from_ref_time(24_070_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		// Minimum execution time: 33_887 nanoseconds.
		Weight::from_ref_time(34_514_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking Candidates (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn join_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 65_280 nanoseconds.
		Weight::from_ref_time(75_460_139)
			// Standard Error: 2_451
			.saturating_add(Weight::from_ref_time(268_863).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 41_939 nanoseconds.
		Weight::from_ref_time(48_950_480)
			// Standard Error: 2_315
			.saturating_add(Weight::from_ref_time(203_085).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	/// The range of component `x` is `[2, 1200]`.
	fn execute_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 118_763 nanoseconds.
		Weight::from_ref_time(121_487_000)
			// Standard Error: 771_470
			.saturating_add(Weight::from_ref_time(61_740_690).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// The range of component `x` is `[3, 1000]`.
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		// Minimum execution time: 39_156 nanoseconds.
		Weight::from_ref_time(53_419_121)
			// Standard Error: 2_529
			.saturating_add(Weight::from_ref_time(199_678).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_offline() -> Weight {
		// Minimum execution time: 37_081 nanoseconds.
		Weight::from_ref_time(37_757_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_online() -> Weight {
		// Minimum execution time: 36_564 nanoseconds.
		Weight::from_ref_time(37_164_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		// Minimum execution time: 57_424 nanoseconds.
		Weight::from_ref_time(61_670_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		// Minimum execution time: 33_962 nanoseconds.
		Weight::from_ref_time(34_541_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		// Minimum execution time: 72_308 nanoseconds.
		Weight::from_ref_time(75_013_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		// Minimum execution time: 31_086 nanoseconds.
		Weight::from_ref_time(32_220_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn schedule_leave_delegators() -> Weight {
		// Minimum execution time: 39_425 nanoseconds.
		Weight::from_ref_time(39_948_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	/// The range of component `x` is `[2, 100]`.
	fn execute_leave_delegators(x: u32, ) -> Weight {
		// Minimum execution time: 103_777 nanoseconds.
		Weight::from_ref_time(106_508_000)
			// Standard Error: 228_451
			.saturating_add(Weight::from_ref_time(37_404_645).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		// Minimum execution time: 42_475 nanoseconds.
		Weight::from_ref_time(48_567_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		// Minimum execution time: 40_275 nanoseconds.
		Weight::from_ref_time(45_012_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegator_bond_more() -> Weight {
		// Minimum execution time: 80_619 nanoseconds.
		Weight::from_ref_time(88_847_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		// Minimum execution time: 39_383 nanoseconds.
		Weight::from_ref_time(47_117_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		// Minimum execution time: 104_927 nanoseconds.
		Weight::from_ref_time(109_528_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		// Minimum execution time: 102_450 nanoseconds.
		Weight::from_ref_time(122_112_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		// Minimum execution time: 39_851 nanoseconds.
		Weight::from_ref_time(40_721_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		// Minimum execution time: 53_177 nanoseconds.
		Weight::from_ref_time(60_512_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: BridgeTransfer ExternalBalances (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: System Account (r:1001 w:1001)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	/// The range of component `x` is `[8, 100]`.
	/// The range of component `y` is `[0, 5000]`.
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		// Minimum execution time: 1_298_415 nanoseconds.
		Weight::from_ref_time(3_640_323_182)
			// Standard Error: 76_424
			.saturating_add(Weight::from_ref_time(13_579).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(215))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(207))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `y` is `[0, 1000]`.
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		// Minimum execution time: 62_867 nanoseconds.
		Weight::from_ref_time(49_058_536)
			// Standard Error: 132_839
			.saturating_add(Weight::from_ref_time(17_455_481).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(y.into())))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn base_on_initialize() -> Weight {
		// Minimum execution time: 7_698 nanoseconds.
		Weight::from_ref_time(10_540_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 100]`.
	fn set_auto_compound(x: u32, _y: u32, ) -> Weight {
		// Minimum execution time: 70_168 nanoseconds.
		Weight::from_ref_time(96_971_036)
			// Standard Error: 2_241
			.saturating_add(Weight::from_ref_time(131_562).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	/// The range of component `x` is `[0, 1200]`.
	/// The range of component `y` is `[0, 1200]`.
	/// The range of component `z` is `[0, 100]`.
	fn delegate_with_auto_compound(x: u32, y: u32, _z: u32, ) -> Weight {
		// Minimum execution time: 126_209 nanoseconds.
		Weight::from_ref_time(130_338_000)
			// Standard Error: 2_898
			.saturating_add(Weight::from_ref_time(96_072).saturating_mul(x.into()))
			// Standard Error: 2_898
			.saturating_add(Weight::from_ref_time(94_717).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}

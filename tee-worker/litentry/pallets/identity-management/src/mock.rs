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

use crate as pallet_tee_identity_management;
use crate::UserShieldingKeyType;
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32},
};
use frame_system as system;
use frame_system::EnsureSignedBy;
use litentry_primitives::{
	Identity, IdentityString, SubstrateNetwork, Web2Network, USER_SHIELDING_KEY_LEN,
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		IMT: pallet_tee_identity_management,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<31>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance; // the type that is relevant to us
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const One: AccountId32 = AccountId32::new([1u8; 32]);
}

impl pallet_tee_identity_management::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ManageOrigin = EnsureSignedBy<One, AccountId32>;
	type MaxMetadataLength = ConstU32<128>;
	type MaxVerificationDelay = ConstU32<2>;
}

const ALICE_KEY: &str = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);

pub fn alice_twitter_identity(suffix: u32) -> Identity {
	let address = IdentityString::try_from(format!("alice{}", suffix).as_bytes().to_vec())
		.expect("convert to BoundedVec failed");
	Identity::Web2 { network: Web2Network::Twitter, address }
}

pub fn alice_web3_identity() -> Identity {
	let alice_key_hex: [u8; 32] =
		hex::decode(ALICE_KEY.strip_prefix("0x").unwrap()).unwrap().try_into().unwrap();
	Identity::Substrate { network: SubstrateNetwork::Polkadot, address: alice_key_hex.into() }
}

pub fn bob_web3_identity() -> Identity {
	let bob_key_hex = [2u8; 32];
	Identity::Substrate { network: SubstrateNetwork::Litentry, address: bob_key_hex.into() }
}

pub fn new_test_ext(set_shielding_key: bool) -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);

		if set_shielding_key {
			let shielding_key: UserShieldingKeyType = [0u8; USER_SHIELDING_KEY_LEN];
			let ss58_prefix = 131_u16;
			let _ = IMT::set_user_shielding_key(
				RuntimeOrigin::signed(ALICE),
				BOB,
				shielding_key.clone(),
				ss58_prefix,
			);
		}
	});
	ext
}

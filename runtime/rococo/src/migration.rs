// Copyright 2020-2021 Litentry Technologies GmbH.
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
use frame_support::{
	inherent::Vec,
	traits::{Get, OnRuntimeUpgrade},
	StorageHasher, Twox128,
};
use sp_std::marker::PhantomData;

pub struct MigrateRecoverCollatorSelectionIntoParachainStaking<T>(PhantomData<T>);
impl<T: frame_system::Config> OnRuntimeUpgrade
	for MigrateRecoverCollatorSelectionIntoParachainStaking<T>
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		use primitives::AccountId;

		log::info!("Pre check pallet CollatorSelection exists");
		// Get Invulnerables address from CollatorSelection
		// WARN: We do not care about any Candidates storage, as we forbid any general transaction
		// by sudo and so no info there in practice
		let _invulnerables =
			frame_support::storage::migration::get_storage_value::<Vec<AccountId>>(
				b"CollatorSelection",
				b"Invulnerables",
				b"",
			)
			.expect("Storage query fails: CollatorSelection Invulnerables");

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		use sp_io::KillStorageResult;

		// Remove ParachainStaking Storage
		// TODO: Very Weak safety
		let entries: u64 = 4 + 6142;
		let _res: KillStorageResult = frame_support::storage::unhashed::clear_prefix(
			&Twox128::hash(b"ParachainStaking"),
			Some(entries.try_into().unwrap()),
			None,
		)
		.into();
		<T as frame_system::Config>::DbWeight::get().writes(entries)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		use primitives::AccountId;
		use sp_io::KillStorageResult;

		log::info!("Post check ParachainStaking");
		let res: KillStorageResult = frame_support::storage::unhashed::clear_prefix(
			&Twox128::hash(b"ParachainStaking"),
			Some(0),
			None,
		)
		.into();

		match res {
			KillStorageResult::AllRemoved(0) | KillStorageResult::SomeRemaining(0) => {},
			KillStorageResult::AllRemoved(n) | KillStorageResult::SomeRemaining(n) => {
				log::error!("Remaining entries: {:?}", n);
				return Err("ParachainStaking not removed")
			},
		};

		log::info!("Post check pallet CollatorSelection exists");
		// Get Invulnerables address from CollatorSelection
		// WARN: We do not care about any Candidates storage, as we forbid any general transaction
		// by sudo and so no info there in practice
		let _invulnerables =
			frame_support::storage::migration::get_storage_value::<Vec<AccountId>>(
				b"CollatorSelection",
				b"Invulnerables",
				b"",
			)
			.expect("Storage query fails: CollatorSelection Invulnerables");

		Ok(())
	}
}

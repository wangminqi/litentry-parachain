use crate::PrehashedEthereumMessage;
use bc_relayer_registry::RelayerRegistryLookup;
use itp_sgx_crypto::{ecdsa::Pair, key_repository::AccessKey};
use parentchain_primitives::Identity;
use std::{
	format,
	string::{String, ToString},
};

pub fn handle<RRL: RelayerRegistryLookup, EKR: AccessKey<KeyType = Pair>>(
	signer: Identity,
	msg: PrehashedEthereumMessage,
	relayer_registry: &RRL,
	key_repository: &EKR,
) -> Result<[u8; 65], String> {
	if relayer_registry.contains_key(signer) {
		let key = key_repository.retrieve_key().map_err(|e| format!("{}", e))?;
		let sig = key.sign_prehash_recoverable(&msg).map_err(|e| format!("{}", e))?;
		Ok(sig)
	} else {
		Err("Unauthorized: Signer is not a valid relayer".to_string())
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::sign_ethereum::handle;
	use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	use itp_sgx_crypto::{ecdsa::Pair as EcdsaPair, mocks::KeyRepositoryMock};
	use k256::{ecdsa::SigningKey, elliptic_curve::rand_core};
	use parentchain_primitives::Identity;
	use sp_core::{sr25519, Pair};

	#[test]
	pub fn it_should_return_ok_for_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();

		let private = SigningKey::random(&mut rand_core::OsRng);
		let signing_key = EcdsaPair::new(private);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result =
			handle(relayer_account, Default::default(), &relayer_registry, &key_repository);

		//then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());

		let private = SigningKey::random(&mut rand_core::OsRng);
		let signing_key = EcdsaPair::new(private);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result =
			handle(non_relayer_account, Default::default(), &relayer_registry, &key_repository);

		//then
		assert!(result.is_err())
	}

	#[test]
	pub fn sign_ethereum_works() {
		// test vector from bc team, verified with sp_core::ecdsa::Pair::sign_prehashed
		let private_key =
			hex::decode("038a5c907573ea7f61a7dcce5ebb2e233a6e9376e5a6f077729bd732d6cab620")
				.unwrap();
		let key_pair = EcdsaPair::from_bytes(&private_key).unwrap();
		let payload =
			hex::decode("3b08e117290fdd2617ea0e457a8eeebe373c456ecd3f6dc6dc4089380f486516")
				.unwrap();
		let result = key_pair.sign_prehash_recoverable(&payload).unwrap();
		let expected_result = hex::decode("e733e8e3cd4f90d8fc10c2f8eeb7183623451b8e1d55b5ab6c4724c5428264955289fac3da7ce2095e12f19b4eb157c55be5c58a09ac8ae3358af0b7ec266a7201").unwrap();

		assert_eq!(&result, expected_result.as_slice())
	}
}

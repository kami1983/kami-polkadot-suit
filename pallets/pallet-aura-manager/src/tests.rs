use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_consensus_aura::sr25519::AuthorityId;
use sp_core::ByteArray;
// use sp_api::RuntimeApiInfo;
use sp_runtime::RuntimeAppPublic;
use hex_literal::hex;
// use sp_runtime::sp_application_crypto::AppCrypto;
// use sp_staking::offence::Offence;

// use sp_keystore::{
// 	testing::KeyStore,
// 	{KeystoreExt, SyncCryptoStore},
// };

const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

#[test]
fn it_works_for_set_validator_list() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(AuraManager::set_validator_list(RuntimeOrigin::root(), vec![
			1,2,3,4,5
		]));
		// Check storage
		assert_eq!(AuraManager::validators(), vec![
			1, 2, 3, 4, 5
		]);

		// Add 6
		assert_ok!(AuraManager::add_validator_list(RuntimeOrigin::root(), 6));
		// Check storage
		assert_eq!(AuraManager::validators(), vec![
			1, 2, 3, 4, 5, 6
		]);
		// Remove 4
		assert_ok!(AuraManager::remove_validator_list(RuntimeOrigin::root(), 4));
		// Check storage
		assert_eq!(AuraManager::validators(), vec![
			1, 2, 3, 5, 6
		]);
	});
}


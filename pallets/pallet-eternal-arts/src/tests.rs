use crate::{mock::*, Error, Event, Pallet, pallet::*};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::app_crypto::sp_core::ConstU32;
use sp_runtime::BoundedVec;
use bound_vec_helper::BoundVecHelper;
use crate::pallet;

#[test]
fn it_works_for_update_administrator_list() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        let administrators = vec![(1, ADMIN_TYPE_IS_MINTER), (2, ADMIN_TYPE_IS_CREATOR), (3, ADMIN_TYPE_IS_CREATOR)];
        handler_set_admin_list(administrators.clone());

        assert_eq!(AdministratorList::<Test>::get(), Some(administrators.clone()));
        System::assert_last_event(Event::UpdateAdministratorList { administrator_list: administrators }.into());
    });
}

#[test]
fn correct_error_for_create_art_collection() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        System::set_block_number(1);

        handler_set_admin_list(vec![(1, ADMIN_TYPE_IS_MINTER)]);

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc.json".to_vec());

        assert_noop!(
			EternalArtsModule::create_art_collection(RuntimeOrigin::signed(1), 0, art_name.clone(), art_url.clone()),
			Error::<Test>::NotAdministrator
		);
    });
}

#[test]
fn it_works_for_create_art_collection() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        handler_set_admin_list(vec![(1, ADMIN_TYPE_IS_CREATOR)]);

        let s_id: u64 = 1;

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc.json".to_vec());
        assert_ok!(handler_create_art_collection(s_id.clone(), art_name.clone(), art_url.clone()));

        assert_noop!(
			EternalArtsModule::create_art_collection(RuntimeOrigin::signed(1), s_id.clone(), art_name.clone(), art_url.clone()),
			Error::<Test>::ArtCollectionIsExists
		);

        // Check data, from ArtCollection
        assert_eq!(ArtCollection::<Test>::get(s_id.clone()), Some(StructArtCollectionData {
            name: art_name.clone(),
            uri: art_url.clone(),
        }));

        System::assert_last_event(Event::ArtCollectionCreated { s_id: s_id, name: art_name, uri: art_url }.into());
    });
}

#[test]
fn it_works_for_update_art_collection() {
    new_test_ext().execute_with(|| {

        // Go past genesis block so events get deposited
        System::set_block_number(2);

        handler_set_admin_list(vec![(1, ADMIN_TYPE_IS_CREATOR)]);

        let s_id: u64 = 1;

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc.json".to_vec());

        assert_ok!(handler_create_art_collection(s_id.clone(), art_name.clone(), art_url.clone()));
        assert_eq!(ArtCollection::<Test>::get(s_id), Some(StructArtCollectionData {
            name: art_name.clone(),
            uri: art_url.clone(),
        }));

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName-update".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc-update.json".to_vec());

        // Dispatch a signed extrinsic.
        assert_ok!(EternalArtsModule::update_art_collection(RuntimeOrigin::signed(1), 1, art_name.clone(), art_url.clone()));
        assert_eq!(ArtCollection::<Test>::get(s_id.clone()), Some(StructArtCollectionData {
            name: art_name.clone(),
            uri: art_url.clone(),
        }));

        // Assert that the correct event was deposited
        System::assert_last_event(Event::ArtCollectionUpdated { s_id: 1, name: art_name, uri: art_url }.into());
    });
}

#[test]
fn correct_error_for_mint_art_owner() {
    new_test_ext().execute_with(|| {

        let (b_ids, s_ids, count_list) = handler_mint_param();
        let error_s_ids:TypeSidList = vec![0, 0, 1, 0];

        assert_noop!(
            EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), s_ids.clone(), count_list.clone()),
            Error::<Test>::NotAdministrator
        );

        handler_set_admin_list(vec![(1, ADMIN_TYPE_IS_MINTER), (1, ADMIN_TYPE_IS_CREATOR)]);

        assert_noop!(
            EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), error_s_ids.clone(), count_list.clone()),
            Error::<Test>::LengthNotMatch
        );

        assert_noop!(
            EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), s_ids.clone(), count_list.clone()),
            Error::<Test>::ArtCollectionNotFound
        );

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName1".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc1.json".to_vec());
        assert_ok!(handler_create_art_collection(0, art_name.clone(), art_url.clone()));

        assert_noop!(
            EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), s_ids.clone(), count_list.clone()),
            Error::<Test>::ArtCollectionNotFound
        );

    });
}

// mint_art_owner
#[test]
fn it_works_for_issue_art_ownership() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        handler_set_admin_list(vec![(1u64, ADMIN_TYPE_IS_CREATOR), (1u64, ADMIN_TYPE_IS_MINTER)]);

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName1".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc1.json".to_vec());
        assert_ok!(handler_create_art_collection(0, art_name.clone(), art_url.clone()));

        let art_name = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"testName2".to_vec());
        let art_url = BoundedVec::<u8, TypeCollectionDataLength>::create_on_vec(b"https://abc2.json".to_vec());
        assert_ok!(handler_create_art_collection(1, art_name.clone(), art_url.clone()));

        let (b_ids, s_ids, count_list) = handler_mint_param();

        // Lock collection
        EternalArtsModule::set_collection_status(RuntimeOrigin::signed(1), 0, true);

        assert_noop!(
            EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), s_ids.clone(), count_list.clone()),
            Error::<Test>::ArtCollectionIsLocked
        );

        // Unlock collection
        EternalArtsModule::set_collection_status(RuntimeOrigin::signed(1), 0, false);

        assert_ok!(EternalArtsModule::issue_art_ownership(RuntimeOrigin::signed(1), b_ids.clone(), s_ids.clone(), count_list.clone()));

        assert_eq!(NftCount::<Test>::get(0), 3);
        assert_eq!(NftCount::<Test>::get(1), 5);
        assert_eq!(NftCount::<Test>::get(2), 0);

        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-A".to_vec()), 0)),1);
        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-B".to_vec()), 0)),2);
        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-C".to_vec()), 0)),0);

        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-A".to_vec()), 1)),0);
        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-B".to_vec()), 1)),0);
        assert_eq!(NftBindInfos::<Test>::get((TypeBid::create_on_vec(b"releation-id-C".to_vec()), 1)),5);

        System::assert_last_event(Event::MintArtOwner { bn: 1, b_ids: b_ids, s_ids: s_ids, count: count_list }.into());
    });
}


fn handler_set_admin_list(administrators: Vec<(u64, u8)>) {
    assert_ok!(EternalArtsModule::update_administrator_list(RuntimeOrigin::root(), administrators.clone()));
}

fn handler_create_art_collection(s_id: u64, art_name: BoundedVec::<u8, TypeCollectionDataLength>, art_url: BoundedVec::<u8, TypeCollectionDataLength>) -> Result<(), &'static str> {
    assert_ok!(EternalArtsModule::create_art_collection(RuntimeOrigin::signed(1), s_id.clone(), art_name.clone(), art_url.clone()));

    // Check data, from ArtCollection
    assert_eq!(ArtCollection::<Test>::get(s_id), Some(StructArtCollectionData {
        name: art_name.clone(),
        uri: art_url.clone(),
    }));

    return Ok(());
}

fn handler_mint_param() -> (TypeBidList, TypeSidList, TypeCountList) {
    let b_ids = vec![
        TypeBid::create_on_vec(b"releation-id-A".to_vec()),
        TypeBid::create_on_vec(b"releation-id-B".to_vec()),
        TypeBid::create_on_vec(b"releation-id-C".to_vec()),
    ];
    let s_ids: TypeSidList = vec![0, 0, 1] ;// TypeSidList::<Test>::create_on_vec(vec![0, 0, 1]);
    let count_list: TypeCountList = vec![1, 2, 5];
    (b_ids, s_ids, count_list)
}
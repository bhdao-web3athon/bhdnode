use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_join_dao() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(BhdaoModule::join_dao(RuntimeOrigin::signed(1),b"member1".to_vec()));
		assert_eq!(BhdaoModule::members_uid_count(), 1);
		// Read pallet storage and assert an expected result.
		//assert_eq!(BhdaoModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::MemberAdded { who: 1, uid: 1 }.into());
	});
}

#[test]
fn it_fails_for_join_dao() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(BhdaoModule::join_dao(RuntimeOrigin::signed(1),b"member1".to_vec()));
		assert_eq!(BhdaoModule::members_uid_count(), 1);
		
		assert_noop!(BhdaoModule::join_dao(RuntimeOrigin::signed(1),b"member1".to_vec()),Error::<Test>::MemberAlreadyExists);
	});
}

#[test]
fn it_adds_members() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Add Qualifier
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),1,1,b"Qualifier1".to_vec()));

		// Add Contributor
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),2,2,b"Contributor1".to_vec()));

		// member counts
		assert_eq!(BhdaoModule::members_uid_count(), 2);

	});
}

#[test]
fn it_uploads_a_document() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_eq!(BhdaoModule::upload_uid_count(),0u64);

		// Add Contributor
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),2,2,b"Contributor1".to_vec()));

		// Contributor uploads a document

		assert_ok!(BhdaoModule::upload_document(RuntimeOrigin::signed(2),b"Doc1".to_vec()));
		assert_eq!(BhdaoModule::upload_uid_count(),1u64);

	});
}


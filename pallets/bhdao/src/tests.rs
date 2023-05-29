use crate::{mock::*, Error, Event, VoteType, VoteStatus, Vote, Roles, Upload, UploadStatus};
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

		assert_eq!(BhdaoModule::get_vote((VoteType::Qualification,1)),Some(Vote{yes_votes: 0, no_votes: 0,start: 1, end:1001,status: VoteStatus::InProgress }));

	});
}

#[test]
fn it_votes_on_document() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_eq!(BhdaoModule::upload_uid_count(),0u64);

		// Add Contributor 1
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),1,2,b"Contributor1".to_vec()));

		// Add four qualifiers 2,3,4,5
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),2,1,b"Qualifier1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),3,1,b"Qualifier1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),4,1,b"Qualifier1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),5,1,b"Qualifier1".to_vec()));


		// Contributor uploads a document

		assert_ok!(BhdaoModule::upload_document(RuntimeOrigin::signed(1),b"Doc1".to_vec()));
		assert_eq!(BhdaoModule::upload_uid_count(),1u64);

		// Check if Vote exists

		assert_eq!(BhdaoModule::get_vote((VoteType::Qualification,1)),Some(Vote{yes_votes: 0, no_votes: 0,start: 1, end:1001,status: VoteStatus::InProgress }));

		// Member 2 casts vote at block 200

		run_to_block(200);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(2),VoteType::Qualification,1,true));

		// Member 3 casts vote at block 500

		run_to_block(500);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(3),VoteType::Qualification,1,true));

		// Member 4 casts vote at block 900

		run_to_block(900);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(4),VoteType::Qualification,1,false));

		// Member 5 tries to cast vote after the time has expired

		run_to_block(1100);
		assert_noop!(BhdaoModule::cast_vote(RuntimeOrigin::signed(5),VoteType::Qualification,1,false),Error::<Test>::VotingWindowNotValid);

		// Finalize vote

		run_to_block(1100);
		assert_ok!(BhdaoModule::finalize_vote(RuntimeOrigin::signed(1),VoteType::Qualification,1));

		// Check if Vote passed

		assert_eq!(BhdaoModule::get_vote((VoteType::Qualification,1)),Some(Vote{yes_votes: 2, no_votes: 1,start: 1, end:1001,status: VoteStatus::Passed }));

		// Check if Verification voting started 

		assert_eq!(BhdaoModule::get_vote((VoteType::Verification,1)),Some(Vote{yes_votes: 0, no_votes: 0,start: 1100, end:2100,status: VoteStatus::InProgress }));

		// Add three verifiers 6,7,8
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),6,3,b"Verifier1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),7,3,b"Verifier2".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),8,3,b"Verifier3".to_vec()));

		// Member 6 casts vote at block 1200

		run_to_block(1200);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(6),VoteType::Verification,1,true));

		// Member 7 casts vote at block 1600

		run_to_block(1600);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(7),VoteType::Verification,1,false));

		// Member 8 casts vote at block 2000

		run_to_block(2000);
		assert_ok!(BhdaoModule::cast_vote(RuntimeOrigin::signed(8),VoteType::Verification,1,true));

		// Finalize The Vote

		run_to_block(2200);
		assert_ok!(BhdaoModule::finalize_vote(RuntimeOrigin::signed(1),VoteType::Verification,1));

		// Check if Vote passed

		assert_eq!(BhdaoModule::get_vote((VoteType::Verification,1)),Some(Vote{yes_votes: 2, no_votes: 1,start: 1100, end:2100,status: VoteStatus::Passed }));

		// Check the upload status

		assert_eq!(BhdaoModule::get_upload(1),Some(Upload{creator: 1, hash: b"Doc1".to_vec(), status: UploadStatus::UnderExpertReview}));

		// Assume no expert objection
		run_to_block(3300);
		assert_ok!(BhdaoModule::finalize_expert_review(RuntimeOrigin::signed(1),1));

		// Check the upload status

		assert_eq!(BhdaoModule::get_upload(1),Some(Upload{creator: 1, hash: b"Doc1".to_vec(), status: UploadStatus::Verified}));


	});
}

#[test]
fn it_votes_for_expanded_role() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_eq!(BhdaoModule::upload_uid_count(),0u64);

		// Add Contributor 1
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),1,2,b"Contributor1".to_vec()));

		// Add four experts 2,3,4,5
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),2,4,b"Expert1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),3,4,b"Expert2".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),4,4,b"Expert3".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),5,4,b"Expert4".to_vec()));


		
		// Add three verifiers 6,7,8
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),6,3,b"Verifier1".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),7,3,b"Verifier2".to_vec()));
		assert_ok!(BhdaoModule::set_membership(RuntimeOrigin::root(),8,3,b"Verifier3".to_vec()));

		// Contributor 1 applies for verifier role

		assert_ok!(BhdaoModule::apply_for_expanded_role(RuntimeOrigin::signed(1),Roles::Verifier));

		// Check if the vote is initialized

		assert_eq!(BhdaoModule::role_application_uid_count(),1);

		assert_eq!(BhdaoModule::get_vote((VoteType::CuratorVerification,1)),Some(Vote{yes_votes: 0, no_votes: 0,start: 1, end:1001,status: VoteStatus::InProgress }));
		
		// Member 6 casts vote at block 200

		run_to_block(200);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(6),VoteType::CuratorVerification,1,true));

		// Member 7 casts vote at block 1600

		run_to_block(600);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(7),VoteType::CuratorVerification,1,true));

		// Member 8 casts vote at block 2000

		run_to_block(900);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(8),VoteType::CuratorVerification,1,false));

		// Finalize the vote

		run_to_block(1100);
		assert_ok!(BhdaoModule::finalize_vote_for_expanded_role(RuntimeOrigin::signed(1),VoteType::CuratorVerification,1));

		// Check if the Vote passed

		assert_eq!(BhdaoModule::get_vote((VoteType::CuratorVerification,1)),Some(Vote{yes_votes: 2, no_votes: 1,start: 1, end:1001,status: VoteStatus::Passed }));

		// Check if CuratorCouncilApproval voting started 

		assert_eq!(BhdaoModule::get_vote((VoteType::CuratorCouncilApproval,1)),Some(Vote{yes_votes: 0, no_votes: 0,start: 1100, end:2100,status: VoteStatus::InProgress }));

		// Member 2 casts vote at block 200

		run_to_block(1400);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(2),VoteType::CuratorCouncilApproval,1,true));

		// Member 3 casts vote at block 1700

		run_to_block(1700);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(3),VoteType::CuratorCouncilApproval,1,true));

		// Member 4 casts vote at block 2000

		run_to_block(2000);
		assert_ok!(BhdaoModule::cast_vote_for_expanded_role(RuntimeOrigin::signed(4),VoteType::CuratorCouncilApproval,1,true));

		// Finalize the vote

		run_to_block(2200);
		assert_ok!(BhdaoModule::finalize_vote_for_expanded_role(RuntimeOrigin::signed(1),VoteType::CuratorCouncilApproval,1));

		// Check if Vote passed

		assert_eq!(BhdaoModule::get_vote((VoteType::CuratorCouncilApproval,1)),Some(Vote{yes_votes: 3, no_votes: 0,start: 1100, end:2100,status: VoteStatus::Passed }));
	});
}


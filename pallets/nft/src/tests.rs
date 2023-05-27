use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_mint_token() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.

		assert_eq!(NftModule::get_tokens_count(),0);

		assert_ok!(NftModule::mint(RuntimeOrigin::signed(1),1,10,1000,b"Token10".to_vec()));
		assert_eq!(NftModule::get_tokens_count(),1);
		assert_eq!(NftModule::balance_of(10,1),1000);
	});
}

#[test]
fn it_works_for_batch_mint() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.

		assert_eq!(NftModule::get_tokens_count(),0);

		assert_ok!(NftModule::mint_batch(RuntimeOrigin::signed(1),vec![1,2],10,vec![900,100],b"Token10".to_vec()));
		assert_eq!(NftModule::get_tokens_count(),1);
		assert_eq!(NftModule::balance_of(10,1),900);
		assert_eq!(NftModule::balance_of(10,2),100);
	});
}

#[test]
fn it_works_for_mint_and_approve() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.

		assert_eq!(NftModule::get_tokens_count(),0);

		assert_ok!(NftModule::mint(RuntimeOrigin::signed(1),1,10,1000,b"Token10".to_vec()));
		assert_eq!(NftModule::get_tokens_count(),1);

		// Check Apporval for account 2 (False)

		assert_eq!(NftModule::operator_approvals(1,2),false);

		// Set Approval for account 2

		assert_ok!(NftModule::set_approval_for_all(RuntimeOrigin::signed(1),2,true));

		// Check Apporval for account 2 (true)

		assert_eq!(NftModule::operator_approvals(1,2),true);
	});
}

#[test]
fn it_works_for_mint_and_transfer() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Account 1 mints 1000 tokens

		assert_eq!(NftModule::get_tokens_count(),0);

		assert_ok!(NftModule::mint(RuntimeOrigin::signed(1),1,10,1000,b"Token10".to_vec()));
		assert_eq!(NftModule::get_tokens_count(),1);

		// Check Balances

		assert_eq!(NftModule::balance_of(10,1),1000);
		assert_eq!(NftModule::balance_of(10,2),0);

		// Transfer 
		assert_ok!(NftModule::transfer(RuntimeOrigin::signed(1),1,2,10,200));

		// Check Balances

		assert_eq!(NftModule::balance_of(10,1),800);
		assert_eq!(NftModule::balance_of(10,2),200);
	});
}
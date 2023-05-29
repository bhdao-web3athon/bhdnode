use crate as pallet_bhdao;
use frame_support::traits::{ConstU16, ConstU32, ConstU64,OnFinalize, OnInitialize};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		BhdaoModule: pallet_bhdao,
		NftModule: pallet_nft,
	}
);

impl frame_system::Config for Test {
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
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub const VOTING_WINDOW: u32 = 1000;

impl pallet_bhdao::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type VotingWindow = ConstU32<VOTING_WINDOW>;
}

pub type Balance = u128;

impl pallet_nft::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TokenId = u128;
	type Balance = Balance;
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		BhdaoModule::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		BhdaoModule::on_initialize(System::block_number());
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

use crate as pallet_kitties;
use frame_support::traits::Hooks;
use frame_support::{
    derive_impl,
    traits::{ConstU32, ConstU64,ConstU128},
    weights::Weight,
};
#[warn(unused_imports)]
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        PalletKitties: pallet_kitties,
        Random: pallet_insecure_randomness_collective_flip,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = Balance;
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
}
impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Randomness = Random;
    type Currency = Balances;
    type MaxKittiesBidPerBlock = ConstU32<10>;
    type MaxKittiesOwned = ConstU32<10>;
    type MinBidBlockSpan = ConstU64<20>;
    type StakeAmount = ConstU128<200>;
    type MinBidIncrement = ConstU128<500>;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    sp_tracing::try_init_simple();
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 10_000_000_000),
            (2, 10_000_000_000),
            (3, 10_000_000_000),
        ],
    }
        .assimilate_storage(&mut storage)
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        log::info!("current block: {:?}", System::block_number());
        PalletKitties::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        PalletKitties::on_initialize(System::block_number());
        PalletKitties::on_idle(System::block_number(), Weight::default());
    }
}

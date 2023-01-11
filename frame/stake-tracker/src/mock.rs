use crate::{self as pallet_stake_tracker, *};
use frame_election_provider_support::{ScoreProvider, VoteWeight};
use frame_support::{parameter_types, weights::constants::RocksDbWeight};
use sp_runtime::{
	testing::{Header, H256},
	traits::IdentityLookup,
	DispatchError, DispatchResult,
};
use sp_staking::{EraIndex, Stake, StakingInterface};

pub(crate) type AccountId = u64;
pub(crate) type AccountIndex = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;

type Block = frame_system::mocking::MockBlock<Runtime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		VoterBagsList: pallet_bags_list::<Instance1>::{Pallet, Call, Storage, Event<T>},
		StakeTracker: pallet_stake_tracker::{Pallet, Storage},
	}
);

parameter_types! {
	pub static ExistentialDeposit: Balance = 1;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = RocksDbWeight;
	type RuntimeOrigin = RuntimeOrigin;
	type Index = AccountIndex;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = frame_support::traits::ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = frame_support::traits::ConstU32<1024>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

impl pallet_stake_tracker::Config for Runtime {
	type Currency = Balances;
	type Staking = StakingMock;
	type VoterList = VoterBagsList;
}
const THRESHOLDS: [sp_npos_elections::VoteWeight; 9] =
	[10, 20, 30, 40, 50, 60, 1_000, 2_000, 10_000];

parameter_types! {
	pub static BagThresholds: &'static [sp_npos_elections::VoteWeight] = &THRESHOLDS;
}

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	// Staking is the source of truth for voter bags list, since they are not kept up to date.
	type ScoreProvider = StakingMock;
	type BagThresholds = BagThresholds;
	type Score = VoteWeight;
}

pub struct StakingMock {}

// We don't really care about this yet in the context of testing stake-tracker logic.
impl ScoreProvider<AccountId> for StakingMock {
	type Score = VoteWeight;

	fn score(_id: &AccountId) -> Self::Score {
		VoteWeight::default()
	}
}

impl StakingInterface for StakingMock {
	type Balance = Balance;
	type AccountId = AccountId;
	type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;

	fn minimum_nominator_bond() -> Self::Balance {
		unimplemented!("Currently not used.")
	}

	fn minimum_validator_bond() -> Self::Balance {
		unimplemented!("Currently not used.")
	}

	fn stash_by_ctrl(controller: &Self::AccountId) -> Result<Self::AccountId, DispatchError> {
		unimplemented!("Currently not used.")
	}

	fn bonding_duration() -> EraIndex {
		unimplemented!("Currently not used.")
	}

	fn current_era() -> EraIndex {
		unimplemented!("Currently not used.")
	}

	// TODO: Impl
	fn stake(
		who: &Self::AccountId,
	) -> Result<Stake<Self::AccountId, Self::Balance>, DispatchError> {
		unimplemented!("Currently not used.")
	}

	fn bond(
		who: &Self::AccountId,
		value: Self::Balance,
		payee: &Self::AccountId,
	) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn nominate(who: &Self::AccountId, validators: Vec<Self::AccountId>) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn chill(who: &Self::AccountId) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn bond_extra(who: &Self::AccountId, extra: Self::Balance) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn unbond(stash: &Self::AccountId, value: Self::Balance) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn withdraw_unbonded(
		stash: Self::AccountId,
		num_slashing_spans: u32,
	) -> Result<bool, DispatchError> {
		unimplemented!("Currently not used.")
	}

	fn desired_validator_count() -> u32 {
		unimplemented!("Currently not used.")
	}

	fn election_ongoing() -> bool {
		unimplemented!("Currently not used.")
	}

	fn force_unstake(who: Self::AccountId) -> DispatchResult {
		unimplemented!("Currently not used.")
	}

	fn is_exposed_in_era(who: &Self::AccountId, era: &EraIndex) -> bool {
		unimplemented!("Currently not used.")
	}

	// TODO: implement
	fn is_validator(who: &Self::AccountId) -> bool {
		unimplemented!("Currently not used.")
	}

	// TODO: implement
	fn nominations(who: &Self::AccountId) -> Option<Vec<Self::AccountId>> {
		unimplemented!("Currently not used.")
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add_era_stakers(
		current_era: &EraIndex,
		stash: &Self::AccountId,
		exposures: Vec<(Self::AccountId, Self::Balance)>,
	) {
		unimplemented!("Currently not used.")
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn set_current_era(era: EraIndex) {
		unimplemented!("Currently not used.")
	}
}

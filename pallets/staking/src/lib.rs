#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
        BoundedVec
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::{FixedU128, traits::{Saturating, Zero}};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        #[pallet::constant]
        type BaseLockPeriod: Get<BlockNumberFor<Self>>;
        #[pallet::constant]
        type LockDurations: Get<[u32; 4]>;
        #[pallet::constant]
        type EmissionMultipliers: Get<[FixedU128; 4]>;
        #[pallet::constant]
        type VestingPeriod: Get<BlockNumberFor<Self>>;
    }

    #[pallet::storage]
    pub type StakedBalances<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        (BalanceOf<T>, BlockNumberFor<T>),
        ValueQuery,
    >;

    #[pallet::storage]
    pub type VestingSchedules<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<VestingSchedule<T::BlockNumber, BalanceOf<T>>, ConstU32<10>>,
        ValueQuery,
    >;

    #[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct VestingSchedule<BlockNumber, Balance> {
        pub start: BlockNumber,
        pub period: BlockNumber,
        pub amount: Balance,
        pub claimed: Balance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn stake(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            lock_period_index: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let duration_months = T::LockDurations::get()[lock_period_index as usize];
            let lock_duration = T::BaseLockPeriod::get() * duration_months;

            T::Currency::transfer(&who, &Self::account_id(), amount, frame_support::traits::ExistenceRequirement::KeepAlive)?;

            let multiplier = T::EmissionMultipliers::get()[lock_period_index as usize];
            let emissions = amount.saturating_mul(multiplier.into());

            StakedBalances::<T>::insert(&who, (amount, lock_duration));

            VestingSchedules::<T>::mutate(&who, |schedules| {
                schedules.push(VestingSchedule {
                    start: frame_system::Pallet::<T>::block_number() + lock_duration,
                    period: T::VestingPeriod::get(),
                    amount: emissions,
                    claimed: Zero::zero(),
                });
            });

            Ok(())
        }
    }
}

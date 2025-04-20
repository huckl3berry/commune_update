#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_staking::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    impl<T: Config> Pallet<T> {
        pub fn voting_power(who: &T::AccountId) -> u64 {
            let (scom_amount, lock_duration) = pallet_staking::StakedBalances::<T>::get(who);
            let base_period = T::BaseLockPeriod::get();
            let months_locked = lock_duration / base_period;
            scom_amount.saturating_mul(months_locked.into())
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;
pub mod migrations;

use sp_runtime::FixedU128;

parameter_types! {
    pub const BaseLockPeriod: u32 = 43_200; // 1 month (6s/block)
    pub const LockDurations: [u32; 4] = [1, 3, 6, 12]; // Months
    pub const EmissionMultipliers: [FixedU128; 4] = [
        FixedU128::from_float(1.0),  // 1 month
        FixedU128::from_float(2.0),  // 3 months
        FixedU128::from_float(3.0),  // 6 months
        FixedU128::from_float(5.0)   // 12 months
    ];
    pub const VestingPeriod: u32 = 129_600; // 3 months
}

impl pallet_staking::Config for Runtime {
    type Currency = Balances;
    type BaseLockPeriod = BaseLockPeriod;
    type LockDurations = LockDurations;
    type EmissionMultipliers = EmissionMultipliers;
    type VestingPeriod = VestingPeriod;
}

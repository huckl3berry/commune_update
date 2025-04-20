# Commune Staking & Governance Upgrade

This runtime upgrade introduces a new staking system with 1, 3, 6, and 12 month lock periods and proportional governance power. Stakers are rewarded with higher emissions for longer locks (up to 5x), and all emissions vest linearly over 3 months after lock expiry. Voting power is now based on both stake and lock duration. This improves Commune by incentivizing long-term commitment and increasing protocol security.

### What is SCOM?

When you stake your COMAI tokens, they are converted 1:1 into SCOM (Staked COMAI).  
- **SCOM** is a non-transferable, time-locked representation of your staked COMAI.
- SCOM determines your voting power and emission rewards.
- After your lock period, SCOM can be redeemed for COMAI and your emissions are released linearly over 3 months.

## Usage

1. Copy the `pallets` and `runtime` folders into your commune repo, replacing or merging as needed.
2. Build with:
   
       cargo build --release --locked --jobs=1

3. Submit a runtime upgrade using the generated WASM.

## Key parameters

- Lock periods: 1, 3, 6, 12 months (changeable in `runtime/src/lib.rs`)
- Emission multipliers: 1x, 2x, 3x, 5x for increasing lock periods
- Vesting period: 3 months (changeable in `runtime/src/lib.rs`)

## Dependency conflicts solved

- **schnorrkel** is pinned to 0.11.4 to avoid cryptographic errors.
- **sp-core** and **sp-runtime** are pinned for compatibility.
- All dependency versions are managed in the workspace for safety.

## Troubleshooting

If you see errors about `sp-core`, `schnorrkel`, or build failures, run:

    cargo clean
    cargo update -p sp-core --precise 9.0.0
    cargo update -p schnorrkel --precise 0.11.4
    cargo build --release --locked --jobs=1


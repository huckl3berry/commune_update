# Commune Staking & Governance Upgrade

This runtime upgrade introduces a new staking system with 1, 3, 6, and 12 month lock periods and proportional governance power. Stakers are rewarded with higher emissions for longer locks (up to 5x), and all emissions vest linearly over 3 months after lock expiry. Voting power is now based on both stake and lock duration. This improves Commune by incentivizing long-term commitment and increasing protocol security.

## Usage

1. Copy the `pallets` and `runtime` folders into your commune repo, replacing or merging as needed.
2. Build with:

       cargo build --release --locked --jobs=1

3. Submit a runtime upgrade using the generated WASM.

## Key parameters

- Lock periods: 1, 3, 6, 12 months (changeable in `runtime/src/lib.rs`)
- Emission multipliers: 1x, 2x, 3x, 5x for increasing lock periods
- Vesting period: 3 months (changeable in `runtime/src/lib.rs`)

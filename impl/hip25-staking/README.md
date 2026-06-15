# HIP-25 Staking — Reference Implementation Scaffold

Companion to [HIP-25](../HIP/HIP-25_Pure_HACD_Staking.md). These files mirror `hacash/rust` conventions and are ready to integrate via PR.

## Integration checklist (`hacash/rust`)

1. **`src/mint/component/diamond.rs`**
   - Add `DIAMOND_STATUS_STAKED = 4`, `DIAMOND_STATUS_STAKING_COOLDOWN = 5`
   - Extend `DiamondSto` with optional staking fields OR add parallel `StakingRecord` state key

2. **`src/mint/state/def.rs`**
   - Register global staking state key

3. **`src/mint/action/`**
   - Copy `diamond_staking.rs`
   - `include!("diamond_staking.rs");` in `mod.rs`
   - Register `DiamondStake` / `DiamondUnstake` in `action.rs`

4. **`src/mint/operate/diamond.rs`**
   - Reject transfer/inscribe when status is staked or cooldown

5. **`src/mint/operate/diamond.rs` + `diamond_insc.rs`**
   - Redirect 40% protocol fees to staking pool

6. **`src/mint/component/block.rs` (or block close hook)**
   - Call `distribute_rewards()` and `finalize_cooldowns()`

## HVM (`hacash/HVM`)

Copy `hvm/StakeHacd.go` and `hvm/UnstakeHacd.go` into `action/` and register VMKind `0x01` / `0x02`.

## Constants (v1 locked)

| Name | Value |
|---|---|
| `STAKING_FEE_SHARE` | 40 |
| `COOLDOWN_BLOCKS` | 864 |
| `MIN_STAKE_BLOCKS` | 25714 |
| `STAKE_HACD_VMKIND` | 0x01 |
| `UNSTAKE_HACD_VMKIND` | 0x02 |

## Build & test

```bash
cd hacash/rust
cargo test staking
cargo build --release
```
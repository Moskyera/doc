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
   - Redirect 22% protocol fees to staking pool

6. **`src/mint/component/block.rs` (or block close hook)**
   - Call `distribute_rewards()` and `finalize_cooldowns()`

## HVM (`hacash/HVM`)

Integrated in branch `hip-25-staking`:

| File | Role |
|---|---|
| `action/StakeHacd.go` | VMKind `0x01` |
| `action/UnstakeHacd.go` | VMKind `0x02` |
| `action/diamond_list.go` | `DiamondNameListMax200` wire parser |
| `trait/staking.go` | `StakingHandler` + `TransactionContext` |
| `extend/staking_call_executor.go` | `ExtendCallExecutor` for opcodes `0x01`–`0x02` |

Wire format after opcode byte: `Uint1 count` + `count × 6` literal bytes.

## RPC (implemented)

| Endpoint | Description |
|---|---|
| `query/staking/status?diamond=` | Per-HACD staking status |
| `query/staking/summary?address=` | Wallet staking summary |
| `query/staking/global` | Global pool state |

## Constants (v1 locked)

| Name | Value |
|---|---|
| `STAKING_FEE_SHARE` | 22 |
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
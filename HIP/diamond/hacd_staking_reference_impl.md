# HIP-25 Reference Implementation Outline

This document sketches the minimal code changes needed in `hacash/rust` (Mint layer) and `hacash/HVM` (external actions). It is a companion to [HIP-25](../HIP/HIP-25_Pure_HACD_Staking.md).

## File map (proposed)

```text
hacash/rust/
  mint/src/staking/
    mod.rs              # module entry
    state.rs            # StakingRecord, GlobalStakingState
    tx.rs               # validate + apply stake/unstake
    rewards.rs          # index accrual, per-block distribution
    fees.rs             # redirect 13% eligible fees to pool
    unlock.rs           # cooldown completion at block close

hacash/HVM/
  action/StakeHacd.go
  action/UnstakeHacd.go
  extend/staking_executor.go   # ExtendCallExecutor for opcodes 0x01, 0x02
```

## Core types (Rust)

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StakingStatus {
    None,
    Staked,
    Cooldown,
}

pub struct StakingRecord {
    pub status: StakingStatus,
    pub staker: Address,
    pub stake_height: u64,
    pub unlock_height: u64,
    pub reward_index: u128,
}

pub struct GlobalStakingState {
    pub total_staked_shares: u64,
    pub global_reward_index: u128,
    pub reward_pool_balance: u128,
    pub paused: bool,
}
```

## Per-block hook (Mint)

```rust
pub fn on_block_close(state: &mut ChainState, height: u64) {
    // 1. Distribute reward_pool to stakers via global_reward_index
    distribute_rewards(state);

    // 2. Finalize cooldowns whose unlock_height == height
    for literal in state.staking.pending_unlocks_at(height) {
        finalize_unstake(state, &literal);
    }
}
```

## Reward distribution

```rust
pub fn distribute_rewards(state: &mut GlobalStakingState) {
    let shares = state.total_staked_shares;
    if shares == 0 || state.reward_pool_balance == 0 {
        return;
    }
    let increment = state.reward_pool_balance / shares as u128;
    state.global_reward_index = state.global_reward_index.saturating_add(increment);
    state.reward_pool_balance = 0;
    emit_reward_distributed(increment, shares);
}
```

## Accrued reward for one literal

```rust
pub fn accrued_reward(record: &StakingRecord, global_index: u128) -> u128 {
    if record.status == StakingStatus::None {
        return 0;
    }
    global_index.saturating_sub(record.reward_index)
}
```

## Stake validation

```rust
pub fn apply_stake(
    state: &mut ChainState,
    staker: Address,
    literals: &[DiamondLiteral],
    height: u64,
) -> Result<(), StakingError> {
    if state.staking.global.paused {
        return Err(StakingError::Paused);
    }
    if literals.len() > MAX_STAKE_BATCH {
        return Err(StakingError::BatchTooLarge);
    }
    for lit in literals {
        let diamond = state.diamonds.get(lit)?;
        ensure_owner(&diamond, &staker)?;
        ensure_not_staked(&diamond)?;
        diamond.staking = StakingRecord {
            status: StakingStatus::Staked,
            staker,
            stake_height: height,
            unlock_height: 0,
            reward_index: state.staking.global.global_reward_index,
        };
        state.staking.global.total_staked_shares += 1;
        emit_staked(lit, &staker, height);
    }
    Ok(())
}
```

## Unstake + cooldown

```rust
const MIN_STAKE_BLOCKS: u64 = 25714; // ~3 months

pub fn apply_unstake(
    state: &mut ChainState,
    staker: Address,
    literals: &[DiamondLiteral],
    height: u64,
) -> Result<(), StakingError> {
    for lit in literals {
        let diamond = state.diamonds.get_mut(lit)?;
        let rec = &mut diamond.staking;
        ensure_staker(rec, &staker)?;
        if rec.status != StakingStatus::Staked {
            return Err(StakingError::InvalidStatus);
        }
        if height < rec.stake_height + MIN_STAKE_BLOCKS {
            return Err(StakingError::MinStakeAge);
        }
        let reward = accrued_reward(rec, state.staking.global.global_reward_index);
        rec.status = StakingStatus::Cooldown;
        rec.unlock_height = height + COOLDOWN_BLOCKS;
        rec.reward_index = state.staking.global.global_reward_index;
        state.staking.global.total_staked_shares -= 1;
        state.staking.pending_rewards.insert(*lit, reward);
        emit_unstake_requested(lit, &staker, height, rec.unlock_height, reward);
    }
    Ok(())
}
```

## Fee redirection (inscription example)

When processing HIP-15 inscription protocol fee `fee`:

```rust
let to_pool = fee * STAKING_FEE_SHARE / 100;
let to_burn = fee - to_pool;
state.staking.global.reward_pool_balance += to_pool;
burn(to_burn);
```

## HVM external action (Go sketch)

```go
// VMKind 0x01 — STAKE_HACD
type StakeHacdAction struct {
    Diamonds string // comma-separated literals
}

func (a StakeHacdAction) VMKind() uint8 { return 0x01 }

func (a StakeHacdAction) IsBurning90PersentTxFees() bool { return true }

func (a StakeHacdAction) Evaluate(ctx trait.Context) trait.EvalResult {
    exec := ctx.GetExtendCallExecutor().(StakingExecutor)
    err := exec.Stake(ctx.Sender(), a.Diamonds)
    if err != nil {
        return eval.ResultFatalErr(err)
    }
    return eval.ResultValue(nil)
}
```

## Test plan

| Test | Expect |
|---|---|
| Stake owned HACD | status = Staked, shares += 1 |
| Transfer staked HACD | rejected |
| Unstake before 3 months | rejected (`MIN_STAKE_AGE`) |
| Unstake → cooldown → unlock | HACD transferable, HAC paid |
| Two stakers, fee deposit | rewards proportional to shares |
| Unstake during zero stake pool | only cooldown logic runs |
| Pause | stake rejected, unstake allowed |
| 201 literals batch | rejected |

## Deployment timeline (target)

| Week | Milestone |
|---|---|
| 1 | State types + unit tests |
| 2 | Mint hooks + fee redirect |
| 3 | HVM opcodes + integration tests |
| 4 | Public testnet + community review |
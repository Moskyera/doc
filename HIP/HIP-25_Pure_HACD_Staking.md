---
HIP: 25
Title: Pure HACD Staking
Status: Draft
Author: Moskyera
Created: 2026-06-15
Requires: HIP-15, HIP-21
---

# HIP-25: Pure HACD Staking

## Table of Contents

- [Abstract](#abstract)
- [Motivation](#motivation)
- [Design Principles](#design-principles)
- [Definitions](#definitions)
- [User Operations](#user-operations)
  - [Stake](#stake)
  - [Unstake](#unstake)
- [Reward Pool & Fee Redirection](#reward-pool--fee-redirection)
- [Reward Distribution](#reward-distribution)
- [Restrictions While Staked](#restrictions-while-staked)
- [On-chain State](#on-chain-state)
- [Transaction Format](#transaction-format)
- [HVM Integration (HIP-21)](#hvm-integration-hip-21)
- [Events](#events)
- [RPC / Explorer Extensions](#rpc--explorer-extensions)
- [Activation & Governance](#activation--governance)
- [Backward Compatibility](#backward-compatibility)
- [Security Considerations](#security-considerations)
- [Open Questions](#open-questions)
- [Future Work (HIP-26+)](#future-work-hip-26)
- [References](#references)

---

## Abstract

This HIP introduces a **native, opt-in staking mechanism for HACD** on the Hacash Layer-1.

Any HACD holder may lock one or more diamonds and earn **HAC rewards** funded by redirected network fees. There are no new tokens, no liquid staking derivatives, no borrowing, and no change to HACD's PoW issuance model.

Staking is intentionally minimal: two user-facing actions (`stake`, `unstake`), perpetual lock duration, per-block reward accrual, and a short cooldown before final unlock.

---

## Motivation

HACD holders want passive HAC income **without selling their diamonds**.

- **HIP-2** (HACD mortgage lending) is economically powerful but complex and has never been activated on mainnet.
- **HIP-21** (HVM) provides the execution layer needed for small, auditable native extensions.
- A **pure staking** primitive is the smallest useful step: lock HACD → earn HAC → unlock when ready.

If the community accepts this HIP, richer features (delegation, tiered rewards, inscription-aware boosts, etc.) can follow in later HIPs.

---

## Design Principles

1. **Opt-in only** — unstaked HACD behavior is unchanged.
2. **No new assets** — rewards are paid in native HAC only.
3. **Minimal state** — one staking record per HACD literal; global counters for reward math.
4. **Fee-funded, not inflation-funded** — rewards come from redirected existing fees, not new coinbase issuance. **v2:** only HIP-15 inscription protocol fees (not transfer fees); undistributed pool burns when no stakers (HIP-11 alignment).
5. **HACD stays non-fungible** — staking locks specific literals; there is no staking receipt token.
6. **Start simple** — no fixed terms, no pools, no liquid staking.

---

## Definitions

| Term | Meaning |
|---|---|
| **HACD literal** | 6-character diamond name from alphabet `WTYUIAHXVMEKBSZN` |
| **Staker** | Address that initiated `stake` and still owns the cooldown/unlock rights |
| **Staked** | HACD locked in the staking state machine; non-transferable |
| **Cooldown** | Waiting period after `unstake` before HACD and rewards are released |
| **Reward pool** | Protocol account accumulating redirected fees, distributed per block to stakers |
| **Share** | One staked HACD literal counts as **1 share** (HACD is indivisible) |

**Block time reference:** per HIP-15 cadence, `1000 blocks ≈ 3.5 days` (~5 minutes/block).

| Constant | Value | Notes |
|---|---|---|
| `COOLDOWN_BLOCKS` | `864` | ~3 days |
| `MIN_STAKE_BLOCKS` | `25714` | ~90 days (3 months); must be staked before unstake |
| `MAX_STAKE_BATCH` | `200` | aligned with max HACD transfer batch |
| `STAKING_FEE_SHARE` | `10%` | share of HIP-15 inscription protocol fees routed to reward pool |
| `POOL_SWEEP_BLOCKS` | `1008` | burn undistributed pool if zero stakers this many consecutive blocks |
| `ACTIVATION_TIMELOCK` | `30 days` | before mainnet enable after release |
| `STAKE_HACD_VMKIND` | `0x01` | HVM external action opcode |
| `UNSTAKE_HACD_VMKIND` | `0x02` | HVM external action opcode |

---

## User Operations

### Stake

```
stake(diamond_literals: string)
```

**Behavior:**

1. Caller must own all listed HACD literals.
2. Each literal must be **unstaked** and pass normal ownership checks.
3. Each literal enters `Staked` state at current block height `H`.
4. Staker address is recorded as the reward beneficiary.
5. Literal becomes **non-transferable** immediately.

**Batch rules:**

- Up to `200` literals per transaction (same limit as HACD transfers).
- Literals are comma-separated, same format as transfers: `WTYUIA` or `AHXVME,KBSTZN`.

**Gas:** standard HVM transaction fee; `90%` burned per existing Hacash rules unless overridden by future fee policy.

---

### Unstake

```
unstake(diamond_literals: string)
```

**Behavior:**

1. Caller must be the recorded **staker** for each literal.
2. Each literal must be in `Staked` state (not already in cooldown).
3. **Minimum stake age:** `H >= stake_height + MIN_STAKE_BLOCKS` (~3 months). Unstake before this is rejected.
4. Literal transitions to `Cooldown` state at block `H`.
5. Unlock block is set to `H + COOLDOWN_BLOCKS`.
6. Accrued HAC rewards are **finalized** at `H` and held until unlock.

**After cooldown:**

- HACD ownership returns to normal transferable state.
- Accumulated HAC is credited to the staker address in the same settlement step.

There is **no fixed maximum staking term**. After the minimum stake age (~3 months), the holder chooses when to unstake.

**v1 reward payout:** all accrued HAC is paid only at cooldown completion. There is **no** `claim_rewards` in v1.

---

## Reward Pool & Fee Redirection

Rewards are **not minted**. They are funded by redirecting a fixed share of existing HACD-related fees into a protocol staking pool.

### Eligible fee sources

| Source | Current behavior (baseline) | HIP-25 v2 redirect |
|---|---|---|
| HACD inscription **protocol fee** (HIP-15) | Burned | `10%` → staking pool; `90%` → burn |
| HACD transfer tx fee | `90%` burned | **unchanged** (no redirect — v2 economics) |
| HACD bidding / mining fees | Burn + miner | **unchanged** |
| Inscription gas fee | `90%` burned | **unchanged** |

### Accounting

At each block `B`:

```
pool_deposit(B) = sum of redirected fees from all eligible txs in block B
reward_pool_balance += pool_deposit(B)
```

If `reward_pool_balance == 0` or `total_staked_shares == 0`, no rewards are distributed that block (fees still accumulate in the pool). If `total_staked_shares == 0` for `POOL_SWEEP_BLOCKS` consecutive blocks, the pool is **burned** (counted in `diamond_insc_burn_zhu`) rather than left indefinitely outside circulation.

### HIP-11 / HIP-2 alignment (v2)

- **Not coinbase issuance** — no new HAC minted; only a partial redirect of fees that would otherwise burn.
- **Unlike HIP-2** — no mortgage loan, no principal+interest repayment loop; HIP-25 remains a simpler lock-and-earn primitive.
- **Supply transparency** — on-chain counters: `cumulative_deposit_zhu`, `cumulative_paid_zhu`, `cumulative_pool_burned_zhu` (see `/query_global_staking`).
- **Mainnet** requires community economic consensus per HIP-11 before `staking_activation_height` is set.

---

## Reward Distribution

### Per-block proportional distribution

Let:

- `S` = total staked shares at block `B`
- `R` = `reward_pool_balance` available for distribution at block `B`
- `s_i` = shares staked by address `i`

Each block, the entire distributable pool balance for that block is allocated:

```
reward_i(B) = R * (s_i / S)
```

Each staked literal counts as **1 share**. Staking 10 HACD = 10 shares.

### Efficient implementation: reward index

To avoid per-block writes per diamond, use a global cumulative index (similar to DeFi staking accumulators):

```
global_reward_index   // cumulative HAC per share, fixed-point
per_literal_index     // index snapshot at last stake or unstake
accrued_reward(literal) = (global_reward_index - per_literal_index) * 1 share
```

On each block with `S > 0` and `R > 0`:

```
global_reward_index += R / S
reward_pool_balance = 0
```

Rounding dust (< 1 satoshi equivalent) stays in the pool.

### APR expectation

APR is **variable** and depends on:

- inscription protocol fee activity
- total staked supply
- fee market

No guaranteed yield is promised by the protocol.

---

## Restrictions While Staked

While a HACD is `Staked` or in `Cooldown`:

| Action | Allowed? |
|---|---|
| Transfer HACD | **No** |
| Stake again | **No** |
| Inscribe / erase (HIP-15) | **No** |
| Inscription update/delete/transfer (HIP-22) | **No** |
| HDNS change (HIP-6) | **No** |
| HIP-2 mortgage | **No** |
| Receive HAC / BTC | **Yes** (address balance unaffected) |
| Initiate unstake | **Yes** (only from `Staked`, not `Cooldown`) |

Rationale: staked HACD is a **committed long-hold position**; allowing state mutations would complicate ownership and reward accounting.

---

## On-chain State

### Per-HACD record (stored in diamond state extension)

```text
StakingRecord {
    status:        enum { None, Staked, Cooldown }
    staker:        Address
    stake_height:  uint64
    unlock_height: uint64   // 0 unless Cooldown
    reward_index:  uint128   // snapshot for accrual math
}
```

### Global staking state

```text
GlobalStakingState {
    total_staked_shares:   uint64
    global_reward_index: uint128
    reward_pool_balance:   uint128
    paused:                bool
}
```

Storage target: **< 64 bytes** additional per staked HACD beyond existing diamond state.

---

## Transaction Format

New transaction type: `HACD_STAKING` (or HVM-wrapped native action).

```text
HACDStakingTx {
    version:     uint8
    action:      enum { STAKE = 1, UNSTAKE = 2 }
    diamonds:    string   // comma-separated literals, max 200
    fee_payer:   Address    // optional; may differ from staker per Hacash gas rules
    signature:   ...
}
```

**Validation errors (non-exhaustive):**

| Code | Condition |
|---|---|
| `NOT_OWNER` | caller does not own literal |
| `ALREADY_STAKED` | literal already staked |
| `NOT_STAKED` | unstake on free literal |
| `NOT_STAKER` | unstake by non-staker address |
| `IN_COOLDOWN` | duplicate unstake while cooling down |
| `MIN_STAKE_AGE` | unstake before `MIN_STAKE_BLOCKS` elapsed |
| `STAKING_PAUSED` | global pause active (stake only) |
| `BATCH_TOO_LARGE` | > 200 literals |

---

## HVM Integration (HIP-21)

HIP-25 is implemented as **two native HVM external actions** via `ExternalActionCall`:

| Opcode | VMKind | Name | Burns 90% gas? |
|---|---|---|---|
| `0x01` | `STAKE_HACD` | Lock HACD and start earning | yes |
| `0x02` | `UNSTAKE_HACD` | Begin cooldown and finalize rewards | yes |

These opcodes are reserved in the HIP-25 HVM extension registry. Future HIPs must not reuse `0x01`–`0x02` within the staking extension namespace.

### Execution flow

```text
User HVM tx
  └─ DynamicExternalActionCall
       └─ STAKE_HACD / UNSTAKE_HACD
            └─ Mint-layer diamond state update
            └─ GlobalStakingState update
            └─ Emit event
```

### Mint-layer hooks (fullnode)

- `ValidateStakingTx()`
- `ApplyStake()` / `ApplyUnstake()`
- `OnBlockClose()` → distribute rewards, process cooldown unlocks
- `RedirectEligibleFees()` → credit reward pool

Reference implementation targets **hacash/rust** `Mint` layer + **hacash/HVM** external action registry.

---

## Events

| Event | Fields |
|---|---|
| `Staked` | `literal`, `staker`, `height`, `total_staked_shares` |
| `UnstakeRequested` | `literal`, `staker`, `height`, `unlock_height`, `accrued_reward` |
| `Unstaked` | `literal`, `staker`, `height`, `reward_paid` |
| `RewardDistributed` | `height`, `amount`, `total_shares` (once per block) |

---

## RPC / Explorer Extensions

### New query endpoints (proposed)

**`GET /query_staking_status?diamond=WTYUIA`**

```json
{
  "literal": "WTYUIA",
  "status": "Staked",
  "staker": "hac1...",
  "stake_height": 1234567,
  "unlock_height": 0,
  "min_unstake_height": 1259441,
  "accrued_reward": "1.23456789",
  "ret": 0
}
```

**`GET /query_staking_summary?address=hac1...`**

```json
{
  "staked_count": 12,
  "cooldown_count": 2,
  "total_accrued_reward": "45.678",
  "ret": 0
}
```

**`GET /query_global_staking`**

```json
{
  "total_staked_shares": 15432,
  "reward_pool_pending": "12.5",
  "estimated_apr": "0.042",
  "ret": 0
}
```

Explorer SHOULD show a staking badge on staked literals and filter "staked / available" in address views.

---

## Activation & Governance

1. **Testnet first** — deploy on public testnet for ≥ 2 weeks.
2. **30-day timelock** — mainnet activation height announced 30 days in advance.
3. **Global pause** — multisig / governance can pause new stakes; unstake always allowed (funds are never frozen indefinitely).
4. **Soft fork** — nodes enforce new rules from `ACTIVATION_HEIGHT`.

---

## Backward Compatibility

- Unstaked HACD: **zero behavior change**.
- HIP-2 / HIP-3 / HIP-4: unaffected; staking is mutually exclusive with mortgage if both ever coexist.
- Wallets / exchanges: must treat staked literals as **non-withdrawable** until unstaked + cooldown complete.
- Existing APIs remain valid; new fields are additive.

---

## Security Considerations

| Risk | Mitigation |
|---|---|
| Reward pool drainage | fee-funded only; no mint; pool cannot go negative |
| Stake/unstake spam | gas burn + `MIN_STAKE_BLOCKS` (3 months) before unstake |
| Exchange accounting errors | staked literals excluded from transfer lists; API `status` field |
| Governance capture | 30-day timelock + public audit before activation |
| Rounding exploits | fixed-point index; dust retained in pool |
| Emergency bug | global pause; unstake always permitted |

---

## Resolved Parameters (v1)

| Parameter | Decision |
|---|---|
| Fee redirect | `10%` on inscription protocol fees only; **no** HACD transfer fee redirect |
| Idle pool | Burn after `1008` blocks with zero stakers |
| Early claim | **Not in v1** — rewards paid only at cooldown unlock |
| Minimum stake age | `25714` blocks (~3 months) before unstake |
| HVM opcodes | `STAKE_HACD = 0x01`, `UNSTAKE_HACD = 0x02` |

---

## Future Work (HIP-26+)

Possible extensions if HIP-25 is accepted:

- Tiered rewards by diamond serial / inscription count
- Delegated staking (keep custody, delegate rewards)
- Staking statistics dashboard standard
- Integration with HIP-22 agent skill permanence incentives

---

## References

- [HIP-1: HACD Bidding Fee Destruction](https://github.com/hacash/paper/blob/master/HIP/diamond/hacd_bidding_fee_destruction.md)
- [HIP-2: HACD Mortgage](https://hacashtalk.com/t/diamond-mortgage-loan-proposal/117)
- [HIP-15: HACD Inscription](https://github.com/hacash/paper/blob/master/HIP/diamond/hacd_inscription.md)
- [HIP-21: Hacash Virtual Machine](https://hacash.com/hvm.pdf)
- [HIP-22: Upgrade HACD Inscriptions](https://github.com/hacash/doc/blob/main/HIP/diamond/Upgrade_HACD_Inscriptions.md)
- [HACD Exchange Integration Notes](https://github.com/hacash/doc/blob/main/server/hacd_explain_for_exchange.md)
- [HVM Repository](https://github.com/hacash/HVM)

---

## Community Feedback

Comment on the GitHub PR or open a discussion thread. Core v1 parameters are locked (see [Resolved Parameters](#resolved-parameters-v1)). Remaining open topic for HIP-26: inscription-weighted reward boosts.

**Status:** Draft — implementation ready for testnet review (branch `hip-25-staking` on [Moskyera/rust](https://github.com/Moskyera/rust) and [Moskyera/HVM](https://github.com/Moskyera/HVM); 10 Mint integration tests passing).
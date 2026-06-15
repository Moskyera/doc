# HIP-25: Pure HACD Staking

**Status:** Draft  
**Author:** Moskyera  
**Created:** 2026-06-15  

## Abstract
This HIP adds a very simple native staking mechanism for HACD.  
Any HACD holder can lock (stake) their diamond and earn automatic network rewards.  
No new tokens, no liquid staking, no borrowing — just pure staking.

## Motivation
HACD holders want passive income without selling their diamonds.  
HIP-2 (HACD mortgage) is too complex and never activated.  
This is the simplest possible first step using HVM (HIP-21).

## Specification

### Staking Rules (super simple)
- `stake(hacdLiteral)` → locks the HACD and starts earning rewards.  
- The HACD cannot be transferred while staked.  
- **Staking duration:** Indefinite / perpetual. No fixed periods (no 1-month, 3-month locking). The holder decides when to unstake.  
- **Rewards:** Every block you receive HAC automatically, proportional to the number of staked HACD.  
- Reward source: **40% of all HACD inscription fees** (art/AI) + HACD transfer fees (automatically redirected).  
- `unstake()` → unlocks the original HACD + all accumulated rewards in HAC after a **3-day cooldown**.

### Technical (minimal)
- Two new opcodes: `STAKE_HACD` and `UNSTAKE_HACD`  
- Small storage (only a flag + accumulated reward per HACD)  
- Events: `Staked`, `Unstaked`, `RewardClaimed`  
- Completely opt-in and 100% backward compatible.

## Community Feedback
Anyone can comment directly on this GitHub Pull Request or file with suggestions.

## Rationale
Start extremely simple. If the community likes it, more advanced features can be added later (e.g. HIP-26).

## Backward Compatibility
No changes to existing HACD transfers, HIP-2, or any previous logic.

## Reference Implementation
- Tiny HVM change + testnet deployment in < 4 weeks  
- Full code will be provided after discussion

## Security Considerations
- 30-day timelock + pause on first deployment

## References
- HIP-2, HIP-21, HIP-22

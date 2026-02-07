---
HIP: 22
Title: Upgrade HACD Inscriptions (Transfer / Per-entry Delete / Single-entry Update)
Created: 2026-02-07
Author: HACD Labs
Require: HIP-15
---

# HIP-22: Upgrade HACD Inscriptions
**Support: Transfer, Per-entry Delete, Single-entry Update (without erasing history)**  

## Table of Contents
- [Abstract](#abstract)
- [Background & Problem](#background--problem)
- [Goals](#goals)
- [Design Principles](#design-principles)
- [Capability Definitions](#capability-definitions)
  - [Single-entry Update](#1-single-entry-update)
  - [Per-entry Delete](#2-per-entry-delete)
  - [Transfer](#3-transfer)
- [Cadence (Cooldown) Recommendation: 1000 blocks → 200 blocks](#cadence-cooldown-recommendation-1000-blocks--200-blocks)
- [Fee & Anti-spam Principles](#fee--anti-spam-principles-simplified)
- [Compatibility](#compatibility)
- [Appendix A: Why these capabilities strengthen AI Agents, and why HACD is better than typical chains](#appendix-a-why-these-capabilities-strengthen-ai-agents-and-why-hacd-is-better-than-typical-chains-non-normative)

---

## Abstract
This proposal argues that data inscribed on HACD should support three capabilities:

1) **Transfer**: a specific inscription entry can be migrated from one HACD to another (with recipient consent)  
2) **Per-entry Delete**: delete a specific entry, instead of only “wipe all”  
3) **Single-entry Update**: update a specific entry, instead of only appending more entries or wiping everything

In addition, to improve usability for AI Agent scenarios, this proposal recommends adjusting the current write cadence from:

**“one inscription per HACD per 1000 blocks”** to **200 blocks**.

All delete/update/transfer actions must preserve HACD’s core principle: **on-chain history must not be erased**. Changes only affect the **current effective state** derived by interpretation.

These capabilities materially strengthen the HACD × AI Agent integration by making agent skills, configuration, permissions, and reputation **maintainable, revocable, and portable**.

---

## Background & Problem
The current inscription model is strong because it is append-only, traceable, and supports erase/reset.  
However, in AI Agent use cases, inscribed data is often a “state commitment” that must evolve over time—e.g., skill version, policy boundaries, endpoint commitments, delegation/permission summaries, etc.

Without per-entry lifecycle controls, we face:

- **No precise revocation**: when one permission/config becomes risky, the only option is to wipe everything  
- **No smooth upgrades**: teams keep appending forever, and it becomes hard to understand the “current effective state”  
- **No assetized portability**: skill/config modules cannot be migrated or traded across agent identities

---

## Goals
- Upgrade inscriptions into **per-entry lifecycle management**
- Give AI Agents three critical capabilities:  
  **Upgradeable (Update)**, **Revocable (Delete)**, **Portable/Tradable (Transfer)**
- Preserve HACD’s fairness, anti-spam properties, cost anchoring, and miner-aligned security feedback

---

## Design Principles
1) **History must remain immutable**: once inscribed, it is always auditable  
2) **State may evolve**: delete/update/transfer do not erase history—only change the “currently effective state”  
3) **Recipient consent**: transfers must be accepted by the recipient to prevent junk injection  
4) **Cost consistency**: state changes must carry protocol cost (aligned with burn + miner incentive spirit)  
5) **Effective-state first**: clients should show the “currently effective set” by default, while still supporting full audit history

---

## Capability Definitions

### 1) Single-entry Update
Update a specific entry. Old versions remain in history but are no longer the currently effective version.  
Constraints: only the current HACD holder can initiate; updates must be auditable and traceable.

### 2) Per-entry Delete
Revoke the effectiveness of a specific entry. History remains, but the entry is no longer effective.  
Constraints: only the current HACD holder can initiate; delete is revocation, not erasure.

### 3) Transfer
Migrate a specific entry from one HACD to another so inscriptions can become portable modules (skills/policies/credentials).  
Key constraint: **recipient consent is mandatory**, and provenance should be auditable.

---

## Cadence (Cooldown) Recommendation: 1000 blocks → 200 blocks
Current rule: one inscription per HACD per 1000 blocks (historically discussed as ~3.5 days scale).  
This proposal recommends: **one inscription-state action per HACD per 200 blocks**  
(where “inscription-state actions” include inscribe / update / delete / transfer and similar state-change actions).

### Why 200 blocks (Core Reasons)
1) **AI Agent “safe revocation” needs a faster window**  
When endpoints/keys/policies become risky, per-entry delete and update must happen sooner—otherwise the on-chain identity becomes a slow-variable liability.

2) **Agents iterate frequently, but still need on-chain cadence constraints**  
200 blocks is not “instant.” It preserves the concept that on-chain changes are slow variables, while avoiding product deadlock caused by 1000-block waits.

3) **Usability improvements directly increase adoption**  
If state changes are too slow, developers will move critical state off-chain, weakening HACD’s role as an asset/state container. 200 blocks encourages “critical commitments on-chain” by default.

4) **Anti-spam should rely on cost + rules, not only slowness**  
HACD already has strong cost anchoring (burn spirit, protocol fee concepts). With real costs in place, reducing cooldown from 1000 to 200 does not equal “free spam”—it enables legitimate demand to settle on-chain.

Principle: **Use cost and rules to deter abuse; use a reasonable cadence to unlock real demand.**

---

## Fee & Anti-spam Principles
- Update / Delete / Transfer must carry protocol cost  
- The cost model should remain directionally consistent: high-frequency changes become more expensive  
- After reducing cooldown, maintain the overall burn + miner incentive spirit to preserve fairness and anti-abuse properties

---

## Compatibility
- Does not change the availability of historical inscriptions  
- New rules mainly affect how “current effective state” is interpreted and displayed  
- Legacy clients can still show raw inscription lists; upgraded clients can show “effective state + audit history”

---

## Appendix A: Why these capabilities strengthen AI Agents, and why HACD is better than typical chains

### A1. Why AI Agents need Transfer / Delete / Update
AI Agent inscriptions are typically “commitments/fingerprints,” such as:
skill pack hashes, tool list hashes, policy boundaries & budget commitments, endpoint commitments, delegation/permission summaries, and reputation event summaries.

Therefore, agents require:
- **Update**: auditable upgrades (prevent silent mutation)  
- **Delete**: precise revocation and risk containment (avoid all-or-nothing wipes)  
- **Transfer**: migratable/tradable skill/policy/credential modules (composable agent assets)

### A2. Why HACD is better than typical chains for AI Agents
1) **PoW cost anchoring fits auditable identity/capability changes**  
The core problem is not “can we write state,” but “is the state change credible and abuse-resistant.” HACD’s cost constraints and burn-aligned spirit make changes inherently costly and constrained—well-suited for high-value agent state.

2) **HACD unifies “asset container + state container”**  
Many chains fragment identity, assets, and metadata across multiple contracts/standards. HACD’s container paradigm naturally consolidates identity (letters/serial/history), readable displays, name resolution (HDNS), and state commitments (inscriptions) into a coherent agent shell.

3) **Delete/update/transfer leave audit trails, enabling verifiable reputation**  
For agents, accountability depends on whether actions are traceable and reconstructable. HACD’s design philosophy keeps history auditable—critical for reputation and responsibility boundaries.

4) **Stacking containers compounds network effects**  
As more agents, skill modules, and assets are expressed in the same PoW container system, composability rises, discovery improves, and trust becomes reusable—creating stacking-driven network effects that contract-fragmented ecosystems struggle to achieve.

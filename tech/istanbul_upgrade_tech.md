# Istanbul Upgrade Technical Whitepaper

> For protocol researchers, technical analysts, wallet and gateway developers, auditors, security reviewers, and DeFi system designers.  
> This document is based primarily on the current `fullnodedev` codebase and presents a whitepaper-style explanation of the eleven core capabilities associated with the Istanbul upgrade.

---

## Abstract

The Istanbul upgrade is not merely a collection of new actions, opcodes, or interfaces. Its deeper significance lies in expanding Hacash programmability from a single-layer model into a **layered programmable financial execution system**.

In that system:

- the **transaction layer** expresses execution boundaries, semantics, and control flow explicitly;
- the **settlement layer** unifies multi-asset, multi-party clearing;
- the **asset layer** turns third-party assets into protocol-level objects;
- the **account layer** introduces both script-controlled accounts and abstract account entrypoints;
- the **execution layer** uses HVM to carry complex financial logic;
- the **auditability layer** uses IR decompilation to improve the readability of deployed on-chain logic;
- the **resource layer** uses protocol_cost, gas, and state lease semantics to constrain long-term resource consumption.

If Bitcoin is understood as a **constrained value-transfer system**, and Ethereum as a **general-purpose smart-contract state machine platform**, then post-Istanbul Hacash is better understood as:

> **a programmable financial execution system jointly composed of transactions, accounts, assets, settlement, and contracts.**

This paper focuses on three core questions:

1. How these eleven capabilities are implemented in code;
2. Which monetary, asset-circulation, and large-scale DeFi scenarios they are best suited for;
3. Why, in combination, they materially change how a public blockchain can carry complex financial business.

---

## 1. Scope and Method

### 1.1 Scope

This paper covers the following eleven capabilities:

1. ActionGuard  
2. TxBlob  
3. AST  
4. TEX  
5. HIP20  
6. HVM  
7. P2SH  
8. IR decompilation  
9. Account abstraction  
10. Intent  
11. Contract state lease semantics

### 1.2 Method

The approach taken here is to **derive the capability model from the code**, rather than simply restating high-level descriptions. The focus is on:

- core data structures;
- execution paths;
- scope and permission boundaries;
- state transition patterns;
- settlement semantics;
- the financial scenarios each capability is designed to support.

### 1.3 Notes

- This document is an explanation of **capabilities and architecture**, not a mainnet activation notice.  
- It deliberately avoids presenting “design goals” as if they were already finalized activation details. The emphasis is on the **implementation model visible in the current repository**.  
- Some high-level terms are not represented in code as single modules, but as combinations of mechanisms. For example:
  - ActionGuard is expressed mainly through `ActScope::GUARD` plus a set of guard actions;
  - HIP20 is expressed mainly through “native asset objects + contract asset hooks”;
  - account abstraction is expressed mainly through `abstract Permit*/Payable*/Construct/Change/Append`.

---

## 2. Terminology and Object Model

To make the rest of this paper easier to follow, it is useful to standardize several key terms first.

### 2.1 Transaction Objects

- **Type3 transaction**: the most important programmable transaction type at present, supporting AST, TEX, P2SH, contract calls, and other complex execution paths.
- **Action**: the smallest action unit inside a transaction.
- **Action Scope**: where an action may appear and execute, such as `TOP`, `AST`, `GUARD`, and `CALL`.

### 2.2 Account Objects

- **Privakey address**: a standard private-key-controlled address.
- **Contract address**: a contract account address.
- **scriptmh address**: a P2SH-style script account address.

### 2.3 Contract Objects

- **HVM**: the contract execution engine.
- **IR / Bytecode**: the two code representations supported by HVM.
- **Abstract Call**: protocol-defined account or lifecycle hook entrypoints such as `PermitAsset`, `PayableHAC`, `Construct`, and similar entries.

### 2.4 Settlement Objects

- **TEX Cell**: the smallest settlement unit inside TEX.
- **TEX Ledger**: the per-transaction temporary settlement ledger used for zero-sum checks and final settlement.

### 2.5 Intent and State Objects

- **Intent**: an in-VM temporary intent scope and KV space isolated by contract ownership.
- **Active Storage**: active state; readable and writable.
- **Recoverable Storage**: recoverable state; still on chain, but not normally readable or writable; it may be renewed, recovered, or deleted.
- **Absent Storage**: fully expired state, treated as nonexistent.

### 2.6 Cost and Resource Objects

- **gas**: protocol resource metering for complex execution paths.
- **protocol_cost**: a protocol fee charged for long-term resource occupation or expansion of protocol-level objects.
- **storage rent / lease**: the model in which state continuously consumes credit as blocks advance.

---

## 3. Why a New Layered Programmability Model Is Needed

### 3.1 The Limits of Traditional Script Programmability

The Bitcoin model demonstrated that on-chain value transfer can encode not only value movement, but also **spending conditions**.  
P2SH, witness structures, and script constraints make Bitcoin particularly well suited to:

- payments;
- cold/hot wallet separation;
- multisig and custody;
- conditional unlocking;
- highly deterministic and conservative fund control.

However, if the goal is to express:

- multi-asset exchange;
- multi-step atomic workflows;
- conditional routing;
- unified settlement;
- large-scale financial state machines;

then a traditional script system is generally not a natural fit.

### 3.2 The Limits of General-Purpose Contract Programmability

The Ethereum model demonstrated that handing complex logic to a general-purpose VM can dramatically expand protocol innovation.  
It is well suited to:

- rapid experimentation with new protocols;
- arbitrary business state machines;
- DeFi lego-style composition.

But its familiar trade-offs are equally clear:

- critical semantics are often buried deep inside contracts;
- the outer transaction layer is not especially self-explanatory;
- multi-asset settlement is usually left to contracts themselves;
- asset standards are often contract conventions rather than protocol objects;
- contract auditing is expensive, and many risks hide in subtle state-machine details.

### 3.3 Hacash’s Layered Approach

Post-Istanbul Hacash does not fully follow either the Bitcoin or Ethereum single-track route. Instead, it distributes different classes of problems across different layers:

- **condition problems** go to the Guard layer;
- **semantic problems** go to the Blob layer;
- **flow problems** go to the AST layer;
- **settlement problems** go to the TEX layer;
- **asset problems** go to the protocol asset layer;
- **account entry problems** go to the P2SH / abstract account layer;
- **complex business logic** goes to HVM;
- **readability and auditability problems** go to the IR decompilation layer;
- **goal-oriented execution coordination** goes to Intent;
- **long-term resource boundaries** go to lease and fee models.

The most important implication of this structure is:

> **not all complexity has to be embedded inside a single black-box contract.**

---

## 4. Comparison with Bitcoin and Ethereum Programmability

### 4.1 Bitcoin: A Constrained Value-Spending System

Bitcoin is best understood as a system for answering:

> “Can this value be spent, and under what conditions may it be spent?”

Its strengths are:

- strong constraints;
- strong verifiability;
- clear rule boundaries;
- an excellent fit for custody, payments, and conservative financial account control.

But it is not naturally well suited to:

- complex on-chain flow orchestration;
- unified multi-asset settlement;
- general financial state machines;
- highly semantic transaction expression.

### 4.2 Ethereum: A General Smart Contract State Machine Platform

Ethereum is best understood as a system for:

> “letting developers implement all kinds of on-chain protocols on top of a general VM.”

Its strengths are:

- extremely high expressiveness;
- a broad innovation surface;
- a mature standards ecosystem;
- flexible contract composability.

But its familiar limitations include:

- weak transaction-layer semantics;
- many critical constraints are not made explicit at the outer layer;
- multi-asset settlement is often delegated to each protocol contract;
- long-term state bloat remains a structural pressure.

### 4.3 Post-Istanbul Hacash: A Layered Programmable Financial Execution System

Post-Istanbul Hacash emphasizes:

- explicit transaction semantics;
- transaction-level flow orchestration;
- protocol-level settlement;
- protocol-level asset objects;
- coexistence of script accounts and abstract accounts;
- readable and auditable contract logic;
- explicit lease boundaries for long-term state resources.

### 4.4 Comparison Table

| Dimension | Bitcoin | Ethereum | Post-Istanbul Hacash |
|---|---|---|---|
| Main locus of programmability | Spending conditions / scripts | Contract state machine | Transaction layer + account layer + settlement layer + contract layer |
| Transaction semantic expression | Weak | Weak, often buried in calldata / contracts | Strong, explicit at the transaction layer |
| Multi-step flow orchestration | Very weak | Often implemented inside contracts | Native AST support |
| Unified multi-asset settlement | Unnatural | Usually maintained inside protocol contracts | Protocol-level TEX support |
| Third-party assets | Weak at native layer | Usually contract standards | HIP20 integrated into a protocol-level asset system |
| Account programmability | Script accounts | Contract wallets / AA extensions | P2SH + protocol-level abstract account hooks |
| Contract readability | Not the main focus | Often depends on verified source code | IR decompilation + sourcemap |
| Long-term state cost boundaries | No complex contract storage issue | High state-bloat pressure | Lease, recovery, and deletion semantics are explicit |
| Best-fit scenarios | Conditional payments, custody | General dApps, rapid experimentation | Monetary finance, asset circulation, complex settlement, large-scale DeFi |

### 4.5 Structural Diagram of the Three Routes

```text
Bitcoin route
  Transaction
   ↓
  UTXO selection
   ↓
  Script / P2SH unlock validation
   ↓
  Input/output settlement

Ethereum route
  Transaction
   ↓
  Call some contract entrypoint
   ↓
  Contract-internal state machine execution
   ↓
  Logs / balances / storage changes

Post-Istanbul Hacash
  Transaction
   ├─ Guard: declare execution boundaries first
   ├─ Blob: attach business semantics
   ├─ AST: choose execution paths
   ├─ TEX: unify multi-asset settlement
   ├─ P2SH / abstract accounts: control fund ingress and egress
   └─ HVM: carry complex financial logic
                ↓
        deferred / refund / fee finalization
```

### 4.6 Explicit Programmability Layer Matrix

| Capability | Bitcoin primary locus | Ethereum primary locus | Hacash primary locus |
|---|---|---|---|
| Conditional constraints | Script layer | Mostly contract layer | Transaction Guard layer + account layer |
| Business semantics | Very weak | Mostly calldata / contract interpretation | Transaction Blob layer |
| Flow orchestration | Very weak | Mostly inside contracts | Transaction AST layer |
| Multi-asset settlement | Unnatural | Mostly inside protocol contracts | Protocol TEX layer |
| Asset model | Weak natively, often externalized | Contract standards | Protocol asset layer + contract hooks |
| Account rules | Script conditions | Contract wallets / AA | P2SH + abstract accounts |
| Complex financial logic | Very limited | General VM | Financially oriented HVM |
| Readability / auditability | Mainly script readability | Often depends on verified source | IR decompilation + sourcemap |
| Long-term state boundary | No contract storage lease model | Mostly gas + design discipline | Protocol leases + protocol_cost |

---

## 5. Overall Architecture and Layer View

### 5.1 Logical Layer Diagram

```text
┌──────────────────────────────────────────────┐
│ Resource boundary layer                      │
│ protocol_cost / gas / state lease            │
├──────────────────────────────────────────────┤
│ Readability & audit layer                    │
│ IR decompilation / sourcemap                 │
├──────────────────────────────────────────────┤
│ Business coordination layer                  │
│ Intent / defer                               │
├──────────────────────────────────────────────┤
│ Contract execution layer                     │
│ HVM / abstract entry / contract lifecycle    │
├──────────────────────────────────────────────┤
│ Account entry layer                          │
│ P2SH(scriptmh) / account abstraction         │
├──────────────────────────────────────────────┤
│ Asset & settlement layer                     │
│ HIP20 / TEX                                  │
├──────────────────────────────────────────────┤
│ Transaction flow layer                       │
│ AST                                          │
├──────────────────────────────────────────────┤
│ Transaction semantics & constraint layer     │
│ ActionGuard / TxBlob                         │
└──────────────────────────────────────────────┘
```

### 5.2 Design Principles

As reflected in the code, the Istanbul upgrade exhibits five clear design principles:

1. **Layer complexity**: constraints, flow, settlement, accounts, and contracts each take responsibility for the layer they are best suited to;
2. **Make semantics explicit where possible**: critical conditions and execution intent do not have to be hidden entirely inside black-box contracts;
3. **Atomicity and rollback first**: both AST and TEX prioritize execution isolation and end-state consistency;
4. **Protocolize accounts and assets**: account-entry behavior and asset objects are elevated into the protocol layer;
5. **Preserve long-term sustainability**: state occupation, deployment expansion, and complex execution all require bounded pricing.

---

## 6. Unified Execution Flow

This section first explains the system as a whole, rather than one capability at a time.

### 6.1 Type3 Transaction Main Flow

```text
Submit Type3 transaction
   ↓
Transaction topology / scope precheck
   ↓
Initialize gas budget
   ↓
Execute top-level actions in order
   ↓
Each action passes through a common wrapper:
  - upgrade / gate check
  - runtime scope check
  - base gas by action size
  - business logic execution
  - action hook trigger
   ↓
After all actions complete
   ↓
Run TEX unified settlement
   ↓
Run deferred phase
   ↓
On success path: gas refund / statistics commit / fee deduction
```

This means a transaction is not simply “finished as soon as the last top-level action succeeds.” After the top-level actions there are still:

- a **unified settlement phase**;
- a **deferred callback phase**;
- a **final gas and fee finalization phase**.

In other words, the protocol already gives transactions a multi-phase commit flavor.

### 6.2 AST Child Execution Flow

```text
Enter AST child item
   ↓
Create local snapshot (state / vm volatile / logs / tex)
   ↓
Attempt child action execution
   ├─ success → charge gas → commit child snapshot
   ├─ Revert  → roll back child snapshot only
   └─ Fault   → bubble up; outer layer decides whole-tx failure
```

This allows AST to support:

- fallback;
- conditional branching;
- failure isolation;
- partial-success selection.

### 6.3 Transfer Hook Flow

```text
Execute ordinary transfer action
   ↓
Enter do_action_hook
   ↓
Check whether from / to are special accounts
   ├─ from is scriptmh → call P2SH entry to validate script
   ├─ from is contract → call Permit* abstract entry
   └─ to   is contract → call Payable* abstract entry
```

So an ordinary asset transfer is not just “mutate balances.” It can naturally become:

- account-level policy validation;
- automatic receiver-side bookkeeping;
- sender-side authorization;
- script-account unlock execution.

### 6.4 TEX Unified Settlement Flow

```text
Execute each TEX cell
   ↓
Update local state + update TEX ledger
   ↓
At tx end, enter do_settlement
   ↓
Check whether HAC / SAT / HACD / Asset nets sum to zero
   ↓
Do FIFO allocation for the diamond pool
   ↓
Perform final materialized transfers
```

This shows that TEX is not merely an “action list,” but a **transaction-internal clearing ledger**.

---

## 7. Overview of the Eleven Capabilities

| # | Capability | Position in the system | Primary problem it solves |
|---|---|---|---|
| 1 | ActionGuard | Transaction constraint layer | Declare under what conditions execution is allowed |
| 2 | TxBlob | Transaction semantic layer | Let transactions carry business context |
| 3 | AST | Transaction flow layer | Give transactions conditionals and path control |
| 4 | TEX | Settlement layer | Unify multi-asset, multi-party clearing |
| 5 | HIP20 | Protocol asset layer | Bring third-party assets into the protocol object system |
| 6 | HVM | Contract execution layer | Carry complex financial logic |
| 7 | P2SH | Script account layer | Control account spending with script conditions |
| 8 | IR decompilation | Audit layer | Make deployed logic more readable and reviewable |
| 9 | Account abstraction | Abstract account layer | Turn accounts into programmable policy entrypoints |
| 10 | Intent | Business coordination layer | Provide temporary coordination space for goal-oriented multi-step execution |
| 11 | Contract state lease | Resource boundary layer | Put explicit cost boundaries around long-term state |

### 7.1 Collaboration Diagram: All Eleven Capabilities in One Complex Transaction

```text
[Guard]
  ├─ ChainAllow
  ├─ HeightScope
  └─ BalanceFloor
        ↓
[Blob]
  └─ order / RFQ / risk-control context
        ↓
[AST]
  ├─ Path A
  ├─ Path B
  └─ Path C
        ↓
[Account entry layer]
  ├─ scriptmh -> P2SH
  └─ contract -> Permit*/Payable*
        ↓
[HVM]
  └─ pool / vault / clearing logic
        ↓
[TEX]
  └─ unified multi-asset settlement
        ↓
[Deferred / Refund / Fee]
```

### 7.2 “Problem Solved” Matrix for the Eleven Capabilities

| Capability | Main technical problem solved | More finance-oriented problem solved |
|---|---|---|
| ActionGuard | Make pre-execution conditions explicit | deadlines, collateral floors, risk boundaries |
| TxBlob | Carry transaction semantics | orders, quotes, business receipts |
| AST | Path selection and failure fallback | routing aggregation, conditional execution |
| TEX | Zero-sum checks and unified settlement | multi-party trades, multi-asset clearing |
| HIP20 | Protocolize third-party assets | stablecoins, shares, notes |
| HVM | Execute complex state machines | AMMs, redemption, liquidation, strategy logic |
| P2SH | Script-account authorization | custody, approvals, conditional unlocks |
| IR decompilation | Readability of deployed logic | auditability, protocol transparency |
| Account abstraction | Programmable account in/out rules | smart vaults, smart wallets |
| Intent | Execution-time intermediate coordination | goal-oriented multi-step execution, complex routing |
| State lease | Long-term state resource boundaries | sustainability for large-scale DeFi |

---

## 8. ActionGuard

### 8.1 Technical Positioning

In code, ActionGuard is not a single class, but a category of actions under **`ActScope::GUARD`**. They may appear in:

- top-level transactions;
- AST branches;

but they cannot be triggered arbitrarily from VM `CALL` execution context.

### 8.2 Major Guard Actions in the Current Implementation

- `ChainAllow`: restricts chain ID;
- `HeightScope`: restricts the valid block-height window;
- `BalanceFloor`: restricts the minimum balance of an address at the current execution point;
- `TxMessage` / `TxBlob`: semantic actions at the guard layer.

### 8.3 Execution Mechanics

Guard actions have two key checks before and around execution:

1. **Scope check**: they may only execute in allowed positions;
2. **Transaction topology check**: a transaction cannot consist entirely of Guards without any real business action.

So the functional role of Guard is:

> **to declare execution boundaries, not to replace the business action itself.**

### 8.4 Purpose and Significance

Guards allow a transaction to express, before any actual asset transfer, contract call, or settlement occurs:

- timing conditions;
- chain-environment conditions;
- balance floors;
- contextual constraints.

This is especially important in financial systems, because the true risk of a transaction often lies not in “how it executes,” but in **whether it should be allowed to execute at all**.

### 8.5 Usage Template

```text
Guard 1: restrict chain environment
Guard 2: restrict valid block range
Guard 3: restrict minimum balance at execution time
Action : real business action (transfer / TEX / contract call / routing)
```

### 8.6 Typical Scenarios

- limit orders and expiring orders;
- base-position protection for large transfers;
- anti-cross-chain replay;
- pre-risk-control transactions for institutional accounts.

### 8.7 Financial Example

**Conditional FX order**:

- only valid on mainnet;
- only valid within a chain-time window corresponding to the next 20 minutes;
- after execution the account must still retain a minimum collateral floor;
- if those conditions are no longer satisfied, the transaction fails directly.

This is very close to a traditional finance order with explicit validity and risk boundaries.

---

## 9. TxBlob

### 9.1 Technical Positioning

`TxBlob` is a guard-layer action that does not mutate state and carries only binary business data.

### 9.2 Execution Mechanics

- it runs through the common action wrapper;
- it passes scope and upgrade checks;
- it consumes gas proportional to its serialized size;
- it performs no on-chain state transition;
- it returns an empty result.

### 9.3 Purpose and Significance

The value of TxBlob is not executable logic, but:

> **turning a transaction from a pure state-change record into a business carrier with context.**

It can carry:

- order summaries;
- quote snapshots;
- settlement descriptions;
- audit-proof hashes;
- business-system indexing fields.

### 9.4 Why It Matters

If a complex financial transaction reduces to “contract call + opaque bytes,” wallets, explorers, and audit systems struggle to explain what the transaction actually means. TxBlob provides a protocol-layer semantic slot.

### 9.5 Usage Template

```text
TxBlob(data = order params / settlement proof / routing context)
+ Guard constraints
+ real transfer / settlement / contract actions
```

### 9.6 Financial Example

**Off-chain matched, on-chain settled OTC block trade**:

Off-chain price negotiation is already complete, and on-chain execution is only responsible for delivery. TxBlob can therefore carry:

- order ID;
- quote timestamp;
- matching summary from both sides;
- settlement batch ID;
- risk-review proof hash.

This way the chain records not only that “a settlement happened,” but also **which business instruction it belonged to**.

---

## 10. AST

### 10.1 Technical Positioning

AST is the transaction-level control-flow system and one of the most structurally important capabilities of Type3 transactions.

The two key node types are:

- `AstSelect`: try a set of actions, requiring only a minimum number of successes;
- `AstIf`: evaluate a condition branch and select either the `if` or the `else` path accordingly.

### 10.2 Execution Mechanics

The key point of AST is not merely that it has if/else-like syntax, but that it provides **isolation and rollback semantics**:

- each child executes inside its own local snapshot;
- success commits the child snapshot;
- `Revert` rolls back only that child item;
- faults bubble upward and let an outer layer decide whole-transaction failure.

So AST is not merely syntactic sugar. It is:

> **a transaction-level branching and rollback control system.**

### 10.3 Design Value

AST does not solve the problem of “adding a few more steps.” It solves:

- how one transaction can try multiple paths;
- how failed paths avoid polluting successful ones;
- how business logic can gain real flow control at the transaction layer instead of only inside contracts.

### 10.4 Usage Templates

#### Template A: Routing Selection

```text
AstSelect(min=1, max=1)
  ├─ Path A
  ├─ Path B
  └─ Path C
```

Meaning: any one of the three paths succeeding is sufficient.

#### Template B: Conditional Execution

```text
AstIf(
  cond = condition check,
  br_if = execution path if condition holds,
  br_else = fallback path if condition does not hold
)
```

### 10.5 Typical Scenarios

- routing aggregation;
- conditional execution;
- automatic fallback;
- route switching on settlement failure;
- strategy path selection.

### 10.6 Financial Example

**Best-route multi-pool aggregator**:

A user submits an exchange transaction:

1. first try the deepest pool;
2. if it fails because of slippage, roll back automatically;
3. then try the second-best pool;
4. if still unsuccessful, route through an inventory vault or orderbook inventory path;
5. the transaction succeeds only if at least one path succeeds.

This is one of the core capabilities of advanced DeFi aggregators, and AST elevates it to the transaction layer.

### 10.7 AST Decision Tree Diagram

```text
                 [condition / target]
                        │
                  AstIf / AstSelect
             ┌──────────┼──────────┐
             │          │          │
           Path A     Path B     Path C
             │          │          │
          success     revert     success
             │          │          │
             └────┐     │     ┌────┘
                  │     │     │
                  └─ choose successful path ─┘
                          │
                   continue to later settlement
```

---

## 11. TEX

### 11.1 Technical Positioning

TEX is the protocol-level atomic settlement layer.  
It is not an ordinary action list, but rather:

- a settlement cell language;
- a per-transaction temporary netting ledger;
- a transaction-end settlement engine.

### 11.2 Core Objects

- `TexCellAct`: a participant-signed cell bundle;
- `TexLedger`: records net changes in HAC / SAT / HACD / Asset;
- `CellTrs*`: pay/get cells;
- `CellCond*`: condition cells;
- `do_settlement`: final settlement at transaction end.

### 11.3 Execution Model

The essence of TEX can be summarized as:

> **book inside the transaction first, then settle uniformly at the end.**

The concrete flow is:

1. each participant submits its own `TexCellAct`;
2. each cell updates local state and records net effects into `TexLedger`;
3. at the end, the protocol checks whether all asset categories net to zero;
4. if not, the whole transaction fails;
5. for diamonds, final materialization uses a shared pool with FIFO name assignment.

### 11.4 A Key Implementation Feature: Replayable Signature Fragments

`TexCellAct` signs `addr + cells`, not the transaction hash.  
This makes TEX bundles especially suitable for:

- off-chain matching;
- multi-party pre-signing;
- on-chain assembly by an aggregator.

But it also means:

- they should generally be combined with `HeightScope`, `ChainAllow`, or TEX condition cells;
- so that their valid scope is constrained and unconstrained replay is avoided.

### 11.5 Design Value

TEX lifts settlement consistency out of any single contract and into the protocol layer.  
For multi-party trading and multi-asset exchange, this is crucial, because the hard problem in advanced financial systems is often not “there are many actions,” but rather:

- who pays what;
- who receives what;
- whether failed intermediate steps leave dirty state behind;
- whether the end state actually balances;
- whether all of it can be settled in one shot.

### 11.6 Usage Template

```text
Participant A's TexCellAct:
  Pay Asset X
  Get HAC
  Cond Height <= deadline

Participant B's TexCellAct:
  Pay HAC
  Get Asset X
  Cond ChainId == target

At transaction end:
  TEX ledger checks that all nets equal zero
```

### 11.7 Typical Scenarios

- orderbook clearing;
- atomic multi-asset exchange;
- multi-party net settlement;
- vault-share redemption;
- pooled diamond delivery.

### 11.8 Financial Example

**Orderbook matching and clearing engine**:

- A pays a stable asset and receives HAC;
- B pays HAC and receives the stable asset;
- there may also be a fee receiver or an inventory provider;
- each participant signs its own TEX bundle;
- the aggregator assembles them into one transaction;
- final net checking and settlement happen at the end.

This is very close to a small on-chain clearing house.

### 11.9 TEX Settlement Ledger Diagram

```text
Participant A: Pay Asset X  ─────┐
                                 │
Participant B: Pay HAC      ─────┼──> [TEX Ledger]
                                 │        │
Participant C: Fee Receive ──────┘        │
                                          │
                           Check whether all nets equal zero
                                          │
                           zero   -> unified material settlement
                           nonzero -> whole transaction fails
```

---

## 12. HIP20 (Third-Party Asset Issuance)

### 12.1 Technical Positioning

From the implementation, HIP20 is not merely a plain contract token standard. It is closer to:

> **protocol-level native asset objects + contract-level business control hooks.**

### 12.2 Core Mechanisms

#### Asset Creation

- `AssetCreate` creates asset metadata;
- the metadata includes `issuer / serial / supply / decimal / ticket / name`;
- the protocol checks serial windows and metadata validity;
- `protocol_cost` is charged;
- the full initial supply is assigned to the `issuer` address.

#### Asset Transfer

- `AssetToTrs`
- `AssetFromTrs`
- `AssetFromToTrs`

These are protocol actions, not user-written contract balance tables.

#### Hook Integration with Contracts

When assets move from or to contract accounts, the protocol automatically triggers:

- `PermitAsset`
- `PayableAsset`

### 12.3 Key Difference from Ethereum ERC20

The essence of ERC20 is “the contract itself maintains a balance mapping.”  
In the current HIP20 implementation, the model is closer to:

- the asset object is maintained by the protocol;
- balances move inside unified protocol state;
- the contract does not necessarily own the asset ledger, but instead controls the business semantics around the asset.

This makes the asset layer easier to coordinate with:

- TEX;
- Guard;
- account abstraction;
- protocol statistics and fee models.

### 12.4 Design Value

The meaning of HIP20 is not merely “being able to issue a token,” but:

> **bringing third-party assets into the protocol-native asset system so they can be uniformly settled, guarded, and account-controlled.**

### 12.5 Usage Template

```text
1. Use AssetCreate to create the asset
2. issuer receives the initial supply
3. Move it via Asset*Trs protocol transfers
4. When it moves in or out of contract accounts, PermitAsset / PayableAsset enforce business rules
```

### 12.6 Typical Scenarios

- stablecoins;
- deposit receipts;
- vault shares;
- business points;
- notes;
- RWA fractional shares;
- in-game or application assets.

### 12.7 Financial Example

**HACD fractional asset (HACDS)**:

- the contract receives HACD;
- it maps HACD into divisible asset shares `HACDS` according to a rule;
- the shares can continue to circulate, settle, and trade in secondary markets;
- later they can be redeemed back into HACD according to that rule.

This already reflects the standard pattern of “underlying scarce asset -> financialized share asset.”

---

## 13. HVM

### 13.1 Technical Positioning

HVM is Hacash’s contract execution engine and the core runtime for complex financial logic.

### 13.2 Core Features

- supports both `IRNode` and `Bytecode` formats;
- supports deployment, upgrades, abstract entrypoints, and ordinary function entrypoints;
- supports `edit / view / pure` modes;
- supports state storage, logs, context access, and deferred callbacks;
- supports three execution entry modes: ordinary entry, P2SH entry, and abstract entry.

### 13.3 Execution Model

```text
Entry call (Main / Abst / P2sh)
   ↓
Construct execution context and frame bindings
   ↓
Validate parameters and effect mode
   ↓
Execute bytecode / or convert IR to bytecode first
   ↓
Access state, logs, storage, callbacks, and chain context via host
   ↓
Validate return value at the boundary
```

### 13.4 Why Dual IR and Bytecode Formats Matter

The dual-format design allows HVM to optimize for two goals at the same time:

- **execution efficiency and verification**: runtime can normalize to bytecode;
- **readability and auditability**: deployment and storage can preserve IR semantics.

This is not common in traditional general-purpose VMs.

### 13.5 Design Value

HVM’s importance is not simply that it “supports contracts,” but that it:

- composes naturally with the transaction layer, account layer, and settlement layer;
- integrates naturally with abstract accounts, P2SH, Intent, and state leases;
- is suitable for reserve logic, share logic, redemption logic, rates, term structures, and boundary conditions in financial systems.

### 13.6 Usage Template

```text
Write Fitsh / IR
   ↓
Compile to IR or bytecode
   ↓
Deploy via ContractDeploy
   ↓
Interact through external functions / abstract calls / transfer hooks
```

### 13.7 Typical Scenarios

- AMMs;
- vault strategies;
- lending state machines;
- share accounting;
- stable asset issuance and redemption;
- automated liquidation logic;
- deferred or callback-style fund processing.

### 13.8 Financial Example

**AMM / LP share accounting**:

The test code shows a typical pattern:

- record total shares;
- record per-user LP shares;
- check deadlines before trading;
- execute ratio and share calculations;
- update both local and global state on liquidity withdrawal.

This shows HVM is already capable of carrying typical DeFi state machines directly, rather than being limited to lightweight scripting.

### 13.9 HVM Position in the System Diagram

```text
Transaction / account event
        ↓
 Ordinary entry / abstract entry / P2SH entry
        ↓
               HVM
        ↓       ↓       ↓
     storage   calls   callbacks
        ↓       ↓       ↓
   state mut. child logic defer
        ↓
 integrate with TEX / account hooks / Intent
```

---

## 14. P2SH

### 14.1 Technical Positioning

In the current implementation, P2SH is more accurately described as `scriptmh`:

> **Pay to Script-Merkle-Hash**

It is not “one address binds one script,” but “one address binds the Merkle root of a set of script leaves.”

### 14.2 Core Mechanisms

- leaf commitments include `libs + codeconf + lockbox`;
- branch hashes are computed by fixed rules;
- the final root becomes a `scriptmh` address via `ripemd160(root_sha3)`;
- spending requires a `P2SHScriptProve` containing witness, script leaf, and Merkle path;
- after proof, the protocol stores the validated script object inside current transaction context;
- later transfers from that address automatically execute through the P2SH entrypoint.

### 14.3 Design Value

Compared to a traditional single-script-hash model, `scriptmh` offers several major advantages:

- one address can carry multiple strategies;
- script paths for different risk levels can coexist;
- the address remains stable while authorization strategies are layered;
- it is highly suitable for custody and treasury systems.

### 14.4 Usage Template

```text
1. Construct the set of script leaves
2. Build the canonical Merkle tree
3. Obtain the scriptmh address
4. Fund that address
5. When spending, submit P2SHScriptProve + the real transfer action
6. The protocol automatically validates through the P2SH entry in action hooks
```

### 14.5 Typical Scenarios

- multi-strategy treasury accounts;
- custody accounts;
- multi-condition unlocks;
- tiered approval payments;
- business accounts controlled by multiple roles.

### 14.6 Financial Example

**Institutional treasury**:

The same `scriptmh` address contains three classes of scripts:

- routine spending scripts;
- large-payment approval scripts;
- emergency migration or freeze scripts.

This keeps the funding address stable while enabling a deeply layered permission structure, which is ideal for professional treasury systems.

---

## 15. IR Decompilation

### 15.1 Technical Positioning

IR decompilation is the core of the auditability layer.  
It means on-chain logic does not have to exist only in unreadable low-level form; it can be reconstructed into relatively readable source-like structure.

### 15.2 Core Mechanisms

- `lang_to_irnode / lang_to_ircode`: source -> IR;
- `ircode_to_lang / format_ircode_to_lang`: IR -> readable source-like output;
- `SourceMap` restores:
  - library names;
  - function names;
  - parameter names;
  - local variable names;
  - constant names;
  - parameter prelude structure.

### 15.3 Design Value

On many chains, once deployed contracts lose verified source code, ordinary readers are left facing bytecode black boxes.  
IR decompilation changes that by:

- lowering the barrier to auditing;
- increasing protocol transparency;
- enabling wallets, gateways, and governance tools to produce deeper logic visualizations;
- strengthening public verifiability in the on-chain protocol ecosystem.

### 15.4 Usage Template

```text
Deployment side keeps IR / sourcemap
   ↓
External tool reads IR code
   ↓
Decompile into source-like structured text
   ↓
Use it for auditing, governance review, wallet prompts, or risk analysis
```

### 15.5 Typical Scenarios

- auditing AMMs or vault strategies;
- comparing contract logic before and after upgrades;
- wallet display of “what sort of logic this interaction goes through”;
- security access control, whitelisting, and scoring systems.

### 15.6 Financial Example

**A third-party wallet integrating a stable-asset vault**:

The wallet does not merely want to know that “the contract can be called.” It also wants to know:

- whether there is a hidden withdrawal path;
- whether there are extra privileged management paths;
- whether redemption logic is transparent;
- whether upgrade authorization is reasonable.

IR decompilation reduces dependence on the project’s own narrative.

---

## 16. Account Abstraction

### 16.1 Technical Positioning

This form of account abstraction is not an external bundler model, but a protocol-native **account behavior hook mechanism**.

### 16.2 Core Entrypoints

- `PermitHAC / PermitSAT / PermitHACD / PermitAsset`: control spending;
- `PayableHAC / PayableSAT / PayableHACD / PayableAsset`: control receiving;
- `Construct / Change / Append`: control lifecycle and upgrades.

### 16.3 Execution Mechanics

After an ordinary asset transfer completes, the protocol automatically checks:

- if `from` is a contract account, call `Permit*`;
- if `to` is a contract account, call `Payable*`;
- if this is a deploy or upgrade path, call `Construct / Change / Append`.

Therefore, the account itself can define:

- who may send assets to it;
- what happens when it receives assets;
- under what conditions it may send assets out;
- who may alter its logic and permission structure.

### 16.4 Design Value

The essence of account abstraction here is:

> **turning accounts from passive balance containers into active business entrypoints.**

This is particularly well suited to financial systems, because many constraints naturally belong at the **account boundary**, not merely at the contract-function boundary.

### 16.5 Usage Template

```text
Define abstract Payable*
  -> control receiving semantics
Define abstract Permit*
  -> control spending semantics
Define abstract Construct/Change/Append
  -> control lifecycle and upgrade permissions
```

### 16.6 Typical Scenarios

- smart vaults;
- restricted receiving accounts;
- auto-bookkeeping accounts;
- compliant asset accounts;
- on-chain enterprise treasury accounts.

### 16.7 Financial Example

**A rule-controlled stable-asset vault**:

- it only accepts certain assets from whitelisted clearing contracts;
- it only allows reserve assets to leave when redemption conditions are satisfied;
- each receipt automatically updates internal books;
- contract upgrades must be granted by governance logic.

This is an “account as policy entrypoint” model for finance.

---

## 17. Intent

### 17.1 Technical Positioning

Intent is an HVM-internal temporary coordination space, isolated by contract ownership and bindable to a scope.  
It is not persistent storage; it is an execution-time “goal context container.”

### 17.2 Core Mechanisms

- `intent_new`: create an intent;
- `intent_use`: bind the current execution scope;
- `intent_put / get / take / del`: read and write intermediate state;
- `intent_require*`: validate intermediate constraints;
- `defer`: register deferred callbacks, optionally with explicit intent binding.

### 17.3 A Key Semantic Property

Intent is not a global shared KV map. It has:

- an owner;
- a scope;
- call-chain propagation rules;
- ownership enforcement, so that other contracts cannot arbitrarily read or write another contract’s intent data even if they see a handle.

### 17.4 Design Value

Complex financial flows often require large amounts of execution-time intermediate state, including:

- target asset;
- routing stage;
- intermediate results;
- slippage thresholds;
- clearing markers;
- callback proofs.

If all of that is written into persistent storage, the result is:

- higher cost;
- pollution of long-term state;
- more complicated audit boundaries.

The value of Intent is therefore:

> **keeping temporary goals and intermediate constraints inside the execution phase, not in permanent chain state.**

### 17.5 Usage Template

```text
intent_new(kind)
  ↓
intent_use(handle)
  ↓
intent_put(key, value)
  ↓
scope propagates automatically across contract calls
  ↓
defer / require / final check
```

### 17.6 Typical Scenarios

- multi-hop routing by aggregators;
- multi-step liquidation;
- callback-style fund distribution;
- strategy-stage coordination;
- temporary binding of execution goals.

### 17.7 Financial Example

**Multi-pool router**:

The aggregator creates an intent and writes:

- target asset;
- minimum amount out;
- current path under trial;
- intermediate conversion results;
- the final condition to validate at callback time.

Then multiple pools and vaults execute around the same intent. In the end, the system confirms whether the target was actually achieved. This is very close to organizing execution around the business goal, rather than merely stacking imperative steps.

---

## 18. Contract State Lease Semantics

### 18.1 Technical Positioning

Contract state lease semantics are one of the most easily underestimated, yet most important, capabilities of the Istanbul upgrade.  
They address a fundamental issue:

> **long-term on-chain state cannot grow for free forever.**

### 18.2 Core Objects

Each stored value `ValueSto` includes:

- `charge`: the last settlement height;
- `live_credit`: active-period credit;
- `recover_credit`: recovery-period credit;
- `data`: the actual value.

### 18.3 State Lifecycle

```text
Create state
   ↓
Active (readable, writable)
   ↓ live_credit continuously burns down
Recoverable (still on chain, but not normally readable/writable)
   ↓ recover_credit continuously burns down
Absent (fully treated as nonexistent)
```

### 18.4 Supported Operations

- `storage_new`: create with an initial active lease period;
- `storage_edit`: only allowed for active state;
- `storage_rent`: extend the active period;
- `storage_recv`: extend the recovery period;
- `storage_del`: delete and refund part of the remaining credit;
- `storage_stat`: inspect remaining active and recovery block counts.

### 18.5 Design Value

The meaning of state lease semantics is not just “one more fee.” At a structural level, it means:

- long-term state has a real price;
- inactive but still relevant state can enter a recovery phase;
- permanently abandoned state eventually exits active space;
- protocol sustainability is improved;
- large-scale DeFi gets an explicit governance boundary around state growth.

### 18.6 Usage Template

```text
storage_new(key, value, initial_period)
   ↓
renew active state periodically with storage_rent
   ↓
if already in recovery state, use storage_recv
   ↓
if no longer needed, remove with storage_del
```

### 18.7 Typical Scenarios

- LP positions;
- user balances;
- receipt states;
- deposit records;
- long-lived orders;
- note and instrument lifecycle management.

### 18.8 Financial Example

**Large-scale LP account system**:

A DeFi protocol may eventually hold millions of LP accounts and fragmented positions. The lease model allows the protocol to distinguish between:

- highly active state;
- paused but still recoverable state;
- long-abandoned state that should leave active space.

This prevents indefinite historical-state accumulation from making the system unsustainable.

---

## 19. Resource Boundaries and Economic Constraints

Although not one of the eleven capabilities as a standalone item, understanding the Istanbul upgrade requires understanding two things together:

- **protocol_cost**
- **gas**

These capabilities are only sustainable when bounded by explicit resource rules.

### 19.1 The Role of protocol_cost

In the current implementation, protocol_cost appears primarily in:

- `AssetCreate`: asset issuance;
- `ContractDeploy`: contract deployment;
- `ContractUpdate`: contract expansion or editing.

This means protocol_cost is not a “feature tax,” but rather:

> **a price on long-term occupation of protocol resources.**

In particular:

- when a new asset enters protocol object space;
- when a new contract enters long-term on-chain state;
- when contract bytes and structure expand;

explicit cost must be paid.

### 19.2 The Role of gas

gas, in Type3 transactions and HVM execution, serves as:

- a resource boundary for complex execution paths;
- a consumption constraint on transaction orchestration and VM execution;
- the cost of AST snapshot attempts;
- differential charging for return-path and extended actions.

Its meaning is not merely “a tip for executors.” It is closer to:

> **a protocol cost mechanism that constrains complex execution and preserves long-term sustainability boundaries.**

### 19.3 Why Financial Systems Especially Need Them

Advanced DeFi is not achieved simply by writing more code. It must also solve:

- how long-term state is priced;
- how multi-step execution is priced;
- how multi-asset settlement is constrained;
- how contract upgrades are priced;
- how protocol-object expansion is bounded.

One of the clearest signs of maturity in the Istanbul upgrade is that it does not speak only about expressiveness; it also gives expressiveness a parallel economic boundary.

---

## 20. What These Capabilities Can Do in Combination

### 20.1 On-Chain RFQ / Orderbook Matching System

Can combine:

- ActionGuard: constrain time, chain, and collateral floor;
- TxBlob: attach order and quote semantics;
- AST: fallback and multi-path route choice;
- TEX: multi-party unified settlement;
- HIP20: protocol-level asset delivery;
- account abstraction: account-level receive/spend policy.

This can realize:

- limit orders;
- time-valid orders;
- multi-asset trading;
- unified clearing;
- wallet-readable trade semantics.

### 20.2 Stable Assets / Vault Shares / RWA Circulation Systems

Can combine:

- HIP20: issue share assets;
- HVM: implement collateralization, redemption, fees, and strategy logic;
- P2SH: control underlying reserve accounts;
- account abstraction: implement receive/spend and upgrade permissions;
- IR decompilation: make logic easier for auditors and wallets to understand;
- state lease: price large-scale account and share state.

This can realize:

- vault shares;
- collateralized stable assets;
- note-style and certificate-style assets;
- auditable controlled reserve systems.

### 20.3 Large-Scale DeFi Execution and Clearing Systems

Can combine:

- HVM: financial state machines and numerical logic;
- Intent: goal-oriented multi-step coordination;
- AST: conditional branches and fallback;
- TEX: final net settlement;
- state lease: constrain long-term state growth.

This can realize:

- multi-pool aggregation;
- composite payments;
- automated liquidation;
- multi-asset rebalancing;
- institutional-grade fund workflows;
- systems closer to real financial business flow.

### 20.4 Capability Mapping Matrix for Three Representative System Types

| System Type | Guard | Blob | AST | TEX | HIP20 | HVM | P2SH / AA | Intent | Lease |
|---|---|---|---|---|---|---|---|---|---|
| RFQ / orderbook matching | Strongly dependent | Strongly dependent | Strongly dependent | Strongly dependent | Common | Common | Common | Optional | Medium |
| Vault shares / RWA | Medium | Optional | Optional | Common | Strongly dependent | Strongly dependent | Strongly dependent | Optional | Strongly dependent |
| Large-scale DeFi routing | Strongly dependent | Common | Strongly dependent | Strongly dependent | Common | Strongly dependent | Common | Strongly dependent | Strongly dependent |

### 20.5 End-to-End Diagram: From Order to Settlement

```text
Order conditions
   ↓
Guard
   ↓
Business context
   ↓
TxBlob
   ↓
Path orchestration
   ↓
AST
   ↓
Account authorization / vault rules
   ↓
P2SH / Permit* / Payable*
   ↓
Business logic execution
   ↓
HVM
   ↓
Unified multi-asset clearing
   ↓
TEX
   ↓
Deferred callback / refund / finalization
```

---

## 21. Implications for Technical Research and Security Auditing

From a security and research perspective, the most important fact about the Istanbul upgrade is not merely that it adds more features, but that the **risk surface becomes structurally different**.

### 21.1 Risk Is No Longer Only Inside Contracts

Risk may now be distributed across:

- whether Guard conditions are sufficient;
- whether AST rollback boundaries are correct;
- whether TEX balancing is strict;
- whether account hooks admit abnormal paths;
- whether P2SH unlock scripts are too permissive;
- whether Intent scope propagation is correct;
- whether state leases create boundary-state misuse.

### 21.2 The Audit Object Expands from “Single Contract” to “Execution System”

For researchers, it is no longer enough to inspect contract bytecode alone. One must also inspect:

- transaction-layer expression;
- account-entry control;
- unified settlement semantics;
- deferred callbacks and intent binding;
- long-term resource and state lifecycle.

### 21.3 Audit Transparency Also Improves Significantly

At the same time, IR decompilation, explicit Guards, protocol-level asset objects, and a unified settlement layer help move auditing from black-box reasoning toward more readable, structured analysis.

In other words, this upgrade both increases system capability and improves the conditions for examining the system correctly.

---

## 22. Code Module Index

### 22.1 Transactions and Scopes

- `basis/src/component/action.rs`
- `protocol/src/action/level.rs`
- `protocol/src/action/macro.rs`
- `protocol/src/transaction/type3.rs`

### 22.2 Guard / Blob

- `protocol/src/action/blob.rs`
- `protocol/src/action/chain.rs`

### 22.3 AST

- `protocol/src/action/astselect.rs`
- `protocol/src/action/astif.rs`
- `protocol/src/action/asthelper.rs`
- `protocol/src/context/sub.rs`

### 22.4 TEX

- `protocol/src/tex/action.rs`
- `protocol/src/tex/transfer.rs`
- `protocol/src/tex/condition.rs`
- `protocol/src/tex/settle.rs`
- `basis/src/component/tex.rs`

### 22.5 HIP20 / Native Assets

- `mint/src/action/asset.rs`
- `protocol/src/action/asset.rs`
- `vm/src/hook/action.rs`
- `vm/tests/hacds.rs`

### 22.6 HVM / Contract Lifecycle

- `vm/src/rt/code.rs`
- `vm/src/contract/function.rs`
- `vm/src/action/contract.rs`
- `vm/src/field/contract.rs`
- `vm/tests/amm.rs`
- `vm/tests/hrc20.rs`

### 22.7 P2SH

- `vm/src/action/p2sh.rs`
- `vm/src/action/p2sh_tool.rs`
- `vm/src/machine/entry.rs`
- `field/src/core/address.rs`
- `protocol/src/context/context.rs`

### 22.8 IR Decompilation

- `vm/src/lang/mod.rs`
- `vm/src/lang/formater.rs`
- `vm/src/rt/sourcemap.rs`

### 22.9 Account Abstraction

- `vm/src/rt/abst_call.rs`
- `vm/src/fitshc/parse_top.rs`
- `vm/src/hook/action.rs`
- `vm/src/action/contract.rs`

### 22.10 Intent / defer

- `vm/src/machine/resource.rs`
- `vm/src/native/intent.rs`
- `vm/src/frame/call.rs`
- `vm/src/native/defer.rs`

### 22.11 State Lease

- `vm/src/field/storage.rs`

---

## 23. Conclusion

The most important innovation of the Istanbul upgrade is not any single isolated feature, but that it brings together several layers that on-chain finance genuinely needs:

- transaction conditions;
- transaction semantics;
- transaction flow;
- multi-asset settlement;
- protocol-level assets;
- programmable accounts;
- a financial contract runtime;
- readability of deployed logic;
- goal-oriented execution coordination;
- long-term state cost boundaries.

At the system level, this means Hacash is moving from “a chain that can do value transfer” toward:

> **an open financial execution system that can express complex financial business, unify settlement, expose logic to public audit, and place institutionalized pricing boundaries around long-term resource occupation.**

For technical researchers, the real significance is structural:

- transactions begin to look like business workflows;
- accounts begin to look like strategy entrypoints;
- assets begin to look like protocol-level financial objects;
- contracts become easier to audit;
- complex DeFi becomes easier to deploy within one coherent protocol structure.

That is the deepest technical meaning of the Istanbul upgrade.

---

## Appendix B: Case-Oriented Understanding — How These Capabilities Become Real Systems

This appendix no longer proceeds capability by capability. Instead, it explains the system through business scenarios that are easier for external readers to understand:

- what capabilities need to work together to build on-chain orderbooks and RFQ matching;
- what capabilities need to work together to build underlying-asset fractionalization and reserve-vault systems;
- what capabilities need to work together to build large-scale DeFi routing and unified settlement systems.

Its purpose is not to repeat the main text, but to answer a more direct question:

> **What kinds of actual on-chain systems become possible when these capabilities are used together?**

---

### B.1 Case One: On-Chain RFQ / Orderbook Matching System

#### B.1.1 Background

The real difficulty in traditional orderbook or RFQ systems is not merely that “someone wants to buy and someone wants to sell,” but rather:

- whether the order has an expiration;
- whether it is valid only on a specified chain environment;
- whether the account still satisfies inventory or collateral requirements before execution;
- whether a failed primary route can switch to a fallback route;
- whether multi-party asset exchange can settle atomically in one shot;
- whether wallets and audit systems can understand what the transaction is actually doing.

If only traditional scripting exists, one can usually express only local spending conditions.  
If only general-purpose contracts exist, large amounts of order constraints, matching context, and settlement logic often have to be packed into a single contract state machine.

Post-Istanbul Hacash can instead distribute those concerns across layers.

#### B.1.2 Capability Combination

This scenario typically combines:

- **ActionGuard**: restrict time windows, chain environment, and inventory or collateral floors;
- **TxBlob**: carry order parameters, quote summaries, and batch IDs;
- **AST**: try different matching routes and roll back failed paths;
- **TEX**: settle the final multi-party trade uniformly;
- **HIP20**: let stable assets, note-like assets, and share assets enter protocol-level delivery;
- **account abstraction**: let vault accounts, market-maker inventory accounts, and clearing accounts enforce in/out rules automatically.

#### B.1.3 A Typical Execution Flow

```text
User submits a Type3 transaction
  ↓
Guard declares:
  - only valid on the target chain
  - only valid before the deadline height
  - post-execution account must still satisfy minimum balance / inventory
  ↓
TxBlob carries:
  - order ID
  - quote summary
  - RFQ result summary
  - risk-review proof hash
  ↓
AST tries matching routes:
  - Route A: preferred pool
  - Route B: fallback inventory pool
  - Route C: market-maker inventory route
  ↓
The successful route enters TEX
  ↓
TEX checks whether all multi-party asset nets equal zero
  ↓
Settlement completes
```

#### B.1.4 Why This Is Stronger than a Traditional Contract Call

Because it protocolizes the most important dimensions of an order system:

- **preconditions** do not have to be buried entirely in the contract;
- **business semantics** do not have to disappear into raw calldata;
- **fallback behavior** does not have to be implemented manually inside one huge contract control-flow block;
- **unified settlement** does not have to be self-managed by a single matching contract;
- **account permissions** do not collapse into “whoever can sign can move funds.”

#### B.1.5 What This Can Evolve Into

With additional components such as:

- market-maker inventory accounts;
- controlled clearing vaults;
- multiple HIP20 assets;
- deferred risk-confirmation callbacks;

Hacash can naturally support:

- limit orders;
- RFQ execution;
- OTC block settlement;
- multi-asset orderbooks;
- on-chain clearing flows that look much closer to traditional trading venues.

#### B.1.6 Intuitive Difference from Bitcoin and Ethereum Routes

- In a Bitcoin-style system, conditional spending is natural, but multi-party, multi-asset, path-fallback matching and clearing is not;
- In an Ethereum-style system, complex matching contracts are possible, but many semantics and settlement rules remain hidden inside contracts;
- In Hacash, **order conditions, transaction semantics, flow orchestration, and final clearing can be expressed in separate layers**.

This materially improves:

- wallet display capability;
- readability for risk systems;
- protocol auditability;
- compatibility with traditional financial business rules.

---

### B.2 Case Two: Fractionalization and Reserve-Vault Circulation for HACD / RWA / Reserve Assets

#### B.2.1 Background

Many assets are not naturally suited for direct high-frequency circulation, such as:

- scarce assets like HACD;
- physical assets or notes;
- long-term reserve assets;
- RWA-style certificates;
- large treasury positions.

To bring such assets into a more active on-chain market, one usually needs:

1. custody;
2. fractionalization;
3. certificate issuance;
4. secondary circulation;
5. verifiable redemption.

#### B.2.2 Capability Combination

This scenario typically combines:

- **HIP20**: issue protocol-level share assets;
- **HVM**: implement deposit, mapping, redemption, fees, share logic, and restrictions;
- **P2SH**: place the underlying reserve asset into a script-controlled vault;
- **account abstraction**: define the conditions for receiving and releasing the underlying asset;
- **IR decompilation**: make the fractionalization logic auditable to outside observers;
- **state lease**: place long-term cost boundaries on holder state and vault-share state.

#### B.2.3 A Typical Execution Flow

```text
User deposits the underlying asset into the vault account
  ↓
The vault validates deposit legitimacy via Payable* entry
  ↓
HVM calculates how many shares should be minted
  ↓
Protocol-level asset (HIP20) is issued to the user account
  ↓
The user can transfer, settle, and trade those shares like any normal protocol asset
  ↓
Later the user submits a redemption transaction
  ↓
Permit* / vault logic validates redemption conditions
  ↓
The underlying asset is released and the share asset is burned or reclaimed
```

#### B.2.4 Why This Model Matters

Because it turns the chain of “reserve -> share -> circulation -> redemption” into a single unified protocol structure, rather than relying entirely on a custom token contract plus external scripts.

The four most important benefits are:

1. **the underlying asset can be custodied cleanly**: P2SH and account abstraction make custody rules explicit;
2. **the share asset can circulate**: HIP20 lets the share live directly inside the protocol asset system;
3. **the logic can be audited**: HVM and IR decompilation improve transparency;
4. **resources remain sustainable**: large numbers of share-holder accounts and vault states do not grow for free forever.

#### B.2.5 An Intuitive HACD-Centric View

HACD makes the significance of this model easy to understand:

- HACD itself is naturally a high-value, low-frequency, scarce asset;
- but the market may want smaller, higher-frequency, more composable circulation units;
- so a contract can map HACD into divisible share assets;
- those shares can then be used for:
  - secondary circulation;
  - collateralization;
  - composite payments;
  - liquidity pools;
  - vault receipts.

This is effectively the **financialization of a reserve-value underlying asset**.

#### B.2.6 Larger-Scale Extension: RWA and Reserve Certificates

The same structure can extend to:

- precious-metal certificates;
- warehouse receipts;
- notes;
- treasury-style certificates;
- stable-asset reserve receipts.

In this setting, Hacash is no longer merely “issuing a token,” but:

> **making reserve assets, certificate assets, and redemption rules into one programmable, auditable, settleable protocol structure.**

#### B.2.7 Difference from Traditional Public-Chain Approaches

- On many traditional chains, a share asset is often just some contract’s internal token;
- In Hacash, a share asset can enter the protocol asset layer and coordinate with TEX, Guard, abstract accounts, and unified transfer hooks;
- This makes share assets easier to include in unified settlement and unified financial workflows, rather than being trapped inside isolated contract systems.

---

### B.3 Case Three: Large-Scale DeFi Routing, Composite Execution, and Unified Clearing

#### B.3.1 Background

The real difficulty of advanced DeFi is rarely just “writing a contract.” It is:

- how to choose among multiple liquidity pools;
- how to execute multi-step operations atomically;
- how to keep failed intermediate steps from polluting final state;
- how to unify clearing at the end of multi-asset paths;
- how to keep millions of users and large numbers of positions sustainable over time.

If all these concerns are pushed into one generic contract, the system often becomes:

- too state-heavy;
- too deep in execution paths;
- too difficult to audit;
- too opaque in resource cost.

#### B.3.2 Capability Combination

This scenario typically combines:

- **HVM**: carries pool logic, share logic, quote logic, and clearing logic;
- **Intent**: carries execution-time goals and intermediate context;
- **AST**: handles conditional branching, fallback, and route choice;
- **TEX**: performs unified net settlement at the end;
- **Guard**: constrains time windows, chain environment, and minimum-result conditions;
- **state lease**: constrains long-term state growth.

#### B.3.3 A Typical Execution Flow

```text
User submits a complex DeFi transaction
  ↓
Guard declares:
  - deadline height
  - target chain environment
  - minimum received amount / minimum collateral condition
  ↓
TxBlob carries routing summary / risk-control context
  ↓
HVM Router creates an intent
  ↓
The intent records:
  - target asset
  - minimum amount out
  - current route index
  - intermediate fund state
  - final validation conditions
  ↓
AST tries different routes in order:
  - Pool A
  - Pool B
  - vault inventory
  - composite clearing route
  ↓
Asset flows and accounting from all paths enter the TEX ledger
  ↓
Unified net settlement at transaction end
  ↓
Defer / final check validates whether the target was achieved
```

#### B.3.4 Why Intent Is Especially Important Here

Complex routing often produces large amounts of intermediate information:

- which path is currently being attempted;
- how much asset the previous step output;
- how much capital remains to process;
- whether some stage-level condition is satisfied;
- what the final target condition is.

If all of that is written into persistent storage:

- cost rises;
- long-term state gets polluted;
- audit boundaries worsen;
- historical garbage accumulates.

Intent keeps such data in the **execution phase**, which is crucial for high-frequency, high-complexity DeFi.

#### B.3.5 Why TEX Is Especially Important Here

Even when a route is logically valid, one fundamental question remains:

> **Did the entire transaction actually complete unified settlement across all involved asset types?**

Here TEX is not “some swap action.” It serves as:

- the convergence point of multiple execution paths;
- a multi-asset accounting balancer;
- a transaction-level clearing engine.

This makes it easier for large DeFi systems to move from “multiple contracts are locally correct” to “the whole business flow is globally consistent.”

#### B.3.6 Why State Lease Cannot Be Ignored in Large-Scale DeFi

Suppose the future system contains:

- millions of users;
- large numbers of small positions;
- large numbers of LP shares;
- large quantities of long-unused historical state.

Without lease and recovery boundaries, the protocol would easily face unbounded state growth.

The significance of state leases in large-scale DeFi is not merely charging fees, but:

- institutionalizing who is responsible for long-term state;
- institutionalizing which states deserve to remain in active space;
- institutionalizing whether the protocol can sustainably carry huge numbers of financial objects.

#### B.3.7 What This Can Ultimately Evolve Into

Once these capabilities mature and stack together, Hacash can naturally support:

- routers;
- multi-pool aggregators;
- clearing engines;
- multi-asset payment networks;
- automated strategy execution systems;
- composite DeFi workflow engines;
- on-chain execution systems closer to real financial business chains.

In other words, this is not merely “a few more swap contracts.” It is:

> **a foundation where the transaction layer, account layer, settlement layer, and execution layer are all programmable at the same time for complex financial systems.**

---

### B.4 Unified Conclusion Across the Three Cases

Looking across all three cases, post-Istanbul Hacash is not merely a set of point improvements. It is establishing a new kind of system capability.

#### B.4.1 Transactions Are No Longer Just Transfer Containers

Transactions can now express:

- conditions;
- semantics;
- paths;
- settlement;
- context.

That means transactions increasingly look like **business units**, rather than pure state-modification requests.

#### B.4.2 Accounts Are No Longer Just Signing Subjects

Accounts can now express:

- receiving rules;
- spending rules;
- script constraints;
- upgrade permissions;
- business-entry semantics.

That means accounts increasingly look like **strategy entrypoints**, rather than mere balance containers.

#### B.4.3 Assets Are No Longer Just Contract-Internal Mappings

Assets can now:

- become protocol-level objects;
- be transferred uniformly;
- be settled uniformly;
- be guarded uniformly;
- be connected uniformly to account abstraction and script-account control.

That means assets increasingly look like **unified financial objects**, rather than local variables inside isolated token contracts.

#### B.4.4 Public Chains Become More Suitable for Real Financial Business Chains

Real financial business is usually not a single step. It is:

- check conditions first;
- attach business context;
- execute several steps;
- possibly roll back or reroute in the middle;
- settle uniformly at the end;
- account for long-term state cost.

The most important significance of the Istanbul upgrade is that it gives Hacash native expressive power for this full chain.

---

### B.5 One-Sentence Summary for Readers

If these capabilities must be summarized in the most direct and intuitive way:

- **Bitcoin** showed that on-chain value can be spent safely under conditions;
- **Ethereum** showed that on-chain protocols can grow freely inside a general-purpose VM;
- **post-Istanbul Hacash** is trying to push both one step further:

> **making transactions, accounts, assets, settlement, and contracts together form an open execution system better suited for monetary finance, asset circulation, and large-scale DeFi.**

That is why this upgrade deserves close attention from technical researchers.  
It is not merely answering “what small new feature can be added on chain,” but rather:

> **how a public chain should natively carry more complex, more auditable, and more sustainable financial business.**

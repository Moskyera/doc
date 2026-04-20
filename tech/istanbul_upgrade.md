# Istanbul Upgrade Preview

The Hacash mainnet is about to undergo the most significant technical upgrade in its history. It will be activated after block height 765,432, which is expected to be around July 19, 2026. Most of the core development work has already been completed (repository: [fullnodedev](https://github.com/hacash/fullnodedev)), and the project is now in a strict audit and testing phase.

This is not a routine version update, nor is it simply a matter of adding a few new actions or interfaces. What it really changes is this: **Hacash will move beyond being a chain mainly suited for basic transfers, and further evolve into a public blockchain with layered, semantic, and auditable transaction orchestration capabilities—one that can support advanced DeFi and more complex on-chain business flows.** In other words, the Istanbul Upgrade is about turning transactions from mere “transfers” into on-chain instructions that can directly express conditions, flow, settlement, and business intent.

---

## Eleven Key Capabilities

The most important and representative technical components of this upgrade are:

- **ActionGuard**  
  The transaction guard layer. It defines under what conditions a transaction is allowed to execute before the transaction itself runs. This gives transactions native risk control, restriction, and constraint capabilities.

- **TxBlob**  
  The transaction data layer. It allows transactions to carry richer business data, parameters, and contextual information, making on-chain operations more semantic and easier for wallets, gateways, and audit tools to understand.

- **AST**  
  The transaction orchestration layer. It introduces `if / else / select` style flow expression into transactions, so a transaction no longer has to be just a fixed list of steps—it can become a lightweight business process.

- **TEX**  
  The native settlement layer. It enables multi-asset, multi-step, multi-condition operations to complete unified settlement within a single transaction, which is a key foundation for complex matching, combined payments, and advanced DeFi.

- **HIP20 (Third-Party Asset Issuance)**  
  The asset expansion layer. It allows third-party assets to enter the protocol-level asset system, opening room for stablecoins, points systems, notes, gaming assets, and other financial instruments.

- **HVM**  
  The monetary-financial virtual machine layer. HVM is not a general VM designed primarily for ordinary “decentralized applications.” It is designed around money, finance, and DeFi use cases. It includes many native instructions well suited for interest rates, exchange rates, curves, ratios, slippage, reserves, and settlement-related calculations, so financial logic does not always have to be built in roundabout ways.

- **P2SH**  
  The programmable account layer. It extends address control from “single private-key signature” to “script-based conditional control,” laying the groundwork for custody, conditional unlocks, multi-party control, and account abstraction.

- **IR Decompilation**  
  The contract readability layer. HVM IR can be stably decompiled back into readable source-like form, which means deployed on-chain logic does not have to remain a black-box bytecode forever—it becomes easier to audit, verify, and review.

- **Account Abstraction**  
  The account capability layer. An account no longer has to mean only “a holder of a private key sending a transaction.” Through contract hooks, P2SH, and orchestrated transactions, account behavior can be upgraded into a customizable business entry point.

- **Intent**  
  The transaction intent layer. With intent mechanisms in HVM, contracts can organize state, permissions, and execution paths around “what goal should be achieved” rather than only “what exact manual steps must be performed,” which is more suitable for complex business processes and advanced DeFi.

- **Contract State Leasing**  
  The state resource layer. Contract storage is no longer implicitly treated as free and permanently occupied forever. Instead, it gains explicit semantics for lifetime, renewal, recovery, and deletion, which helps price long-term state resources and improves sustainability.

---

## What Exactly Does This Upgrade Change?

On many blockchains, “programmability” mainly exists at the smart-contract layer: the logic may be powerful, but the semantics are often buried deep inside the contract. The distinctive feature of the Istanbul Upgrade is not just stronger contracts, but turning the **transaction itself** into something clearer, stronger, and easier to audit.

The most important directions of this upgrade can be summarized in four words:

- **Layered**
- **Semantic**
- **Auditable**
- **Composable / Orchestrated**

### 1. Layered

Hacash is no longer putting every capability into one place. Instead, it is forming a clearer capability structure:

- ActionGuard handles constraints;
- TxBlob handles expression;
- AST handles flow;
- TEX handles settlement;
- HIP20 handles asset expansion;
- HVM and P2SH handle deeper programmability;
- IR decompilation turns on-chain logic back into readable text;
- account abstraction upgrades the “address” into a programmable entry point;
- Intent organizes complex business goals into clearer execution semantics;
- contract state leasing establishes boundaries and cost for long-term storage.

This layered structure matters. It means future on-chain business does not have to put everything into one large contract. Different classes of problems can be handled at the most appropriate layer.

More importantly, this structure helps balance three goals that have often conflicted with one another in the past:

- **Expressiveness**: transactions, scripts, contracts, and settlement can each handle the responsibilities they are best suited for;
- **Auditability**: critical semantics do not have to be buried entirely in black-box bytecode;
- **Sustainability**: state growth and complex execution have cost and lifetime boundaries from the beginning.

### 2. Semantic

After the upgrade, on-chain transactions will look more like “business instructions” rather than opaque low-level data.

You can express more clearly:

- under what conditions a transaction may execute;
- what business information it carries;
- what flow it should follow;
- how it should settle in the end;
- which assets and accounts it affects.

This matters greatly to wallets, explorers, gateways, audit systems, and developers. The more complex on-chain business becomes, the more important it is that people can **read it, explain it, and verify it clearly**.

### 3. Auditable

The Istanbul Upgrade does not emphasize hiding logic. It emphasizes making critical semantics as explicit as possible.

That means future DeFi protocols and business systems can more easily become:

- readable before execution;
- traceable during execution;
- reviewable after execution;
- easier to analyze for risk.

In complex financial scenarios, that matters more than simply “whether more complicated code can be written.”

IR decompilation is especially worth highlighting here.  
On many chains, once a contract is deployed, ordinary users are left with only obscure bytecode. But in the Hacash HVM route, IR can be decompiled back into readable source-like text. That means:

- it becomes easier to check whether on-chain logic matches source code;
- it becomes easier for auditors, developers, and the community to understand deployed logic;
- it becomes easier to build a genuinely transparent on-chain protocol ecosystem.

For a blockchain that values verifiability and reviewability, this is not a small optimization. It is important trust infrastructure.

### 4. Orchestrated

This is one of the most representative changes in the Istanbul Upgrade.

After the upgrade, a transaction is no longer just “a bundle of actions.” It can begin to express things like:

- execute only if conditions are satisfied;
- switch to another path if they are not;
- compose multiple actions according to business flow;
- settle multiple assets together in the end;
- hand certain steps over to script accounts or the VM for deeper control.

In other words:

> **Transactions begin to gain flow semantics, and on-chain business begins to gain native orchestration capability.**

---

## Why HVM Is a VM for Money, Finance, and DeFi

When people hear “virtual machine,” they often naturally think of a generic execution environment for general dApps.  
But what is worth emphasizing about HVM is precisely this:

> **Its design orientation is not primarily toward generic applications, but toward money, finance, settlement, and DeFi.**

HVM does not merely provide basic arithmetic. It includes many native computational capabilities closely aligned with financial scenarios, for example:

- **fixed-point exponentiation**: suitable for compounding, interest curves, and exponential-style financial calculations;
- **ceiling division / rounded division**: suitable for share allocation, settlement, and fee calculation;
- **multiply-divide composite operations**: suitable for prices, reserves, shares, exchange rates, and high-precision intermediate calculations;
- **square root / ceiling square root**: suitable for curve models, shares, and reserve calculations;
- **saturating add/subtract, absolute difference, clamp**: suitable for risk controls and boundary handling;
- **deviation scaling, basis-point bound checks, weighted average, linear interpolation**: suitable for slippage, deviation measurement, pricing, market making, AMM logic, and more advanced DeFi math.

These capabilities matter because real DeFi is not just about “moving assets around.” It depends heavily on:

- ratios and exchange rates;
- fixed-point math;
- reserves and shares;
- interest rates and curves;
- price boundaries and slippage protection;
- complex but deterministic settlement formulas.

In many generic VMs, developers have to repeatedly assemble this kind of financial math themselves.  
The HVM approach is to move these **high-frequency financial computation capabilities down into native primitives**, making it more like an “on-chain financial computer” rather than just an ordinary script execution engine.

That is also why, when HVM is combined with TEX, AST, Intent, and account abstraction, it becomes naturally more suitable for advanced DeFi:

- HVM handles financial logic and calculations;
- AST handles flow orchestration;
- TEX handles atomic settlement;
- Intent handles goal organization;
- account abstraction and P2SH handle entry control and authorization models.

This is not a pile of isolated features. It is a coordinated execution system built around on-chain finance.

---

## Why This Matters for Advanced DeFi

What makes advanced DeFi hard has never simply been “writing a contract.”  
What is truly difficult is:

- coordinating multiple assets;
- making multi-step actions execute atomically;
- pushing risk controls to the front of execution;
- auditing complex paths;
- making account control more flexible;
- ensuring final settlement consistency.

The Istanbul Upgrade is filling in precisely these foundational gaps.

After the upgrade, Hacash will more naturally support directions such as:

- conditional execution and time-bounded execution;
- combined payments and combined settlement;
- atomic multi-asset swaps;
- protocol-level issuance and circulation of third-party assets;
- custody, lockups, and conditional unlocks;
- contract strategies and automated financial logic;
- more advanced wallets, account abstraction, and on-chain business systems;
- more complex business execution organized around intent.

So the key point of this upgrade is not merely that “the code becomes more complex,” but that:

> **On-chain business gains a framework that is far more suitable for expressing, executing, and settling DeFi.**

---

## In What Ways Is It Similar to Bitcoin, Ethereum, and Move Chains?

### In what ways is it like Bitcoin?

Bitcoin first showed the world that on-chain transactions could do more than transfer value—they could also carry conditions and script constraints.  
P2SH is one of the classic examples.

The Istanbul Upgrade continues this spirit of strong constraints, strong verifiability, and strong determinism, but moves one step further:

- not just controlling whether funds can be spent;
- but also beginning to control how execution happens, what flow it follows, and how settlement is completed.

### In what ways is it like Ethereum?

Ethereum demonstrated that smart contracts are crucial for the explosion of on-chain applications.  
HVM likewise gives Hacash the ability to support more complex protocol logic.

But Hacash is not simply copying the route where “all complexity goes into contracts.” Instead, it emphasizes:

- placing part of the key semantics at the transaction layer;
- placing part of the complex logic at the VM layer;
- making transactions and contracts work together rather than replace one another.

### In what ways is it similar to Move chains?

Move-based chains have pushed the market to pay more attention again to asset safety and resource semantics.  
The Istanbul Upgrade, in addition to those concerns, places stronger emphasis on **transaction-level orchestration** and **atomic settlement**.

If Move emphasizes “defining assets and modules safely,” then the Istanbul Upgrade places more emphasis on:

> **compressing complex business into a single smarter and more auditable transaction.**

---

## What Will Change on the Chain After Mainnet Activation?

From the perspective of users and business builders, the most direct changes after the upgrade can be understood in eight categories.

### 1. Transactions gain explicit preconditions

Through ActionGuard, transactions can declare execution boundaries before execution begins.

For example:

- they may only execute within a certain block-height range;
- they may only execute on a specific chain environment;
- the account balance after execution may not fall below a given floor.

This changes transactions from “submit and attempt to execute” into “execute only when conditions are satisfied.”

### 2. Transactions gain business semantics

Through TxBlob, transactions can carry more structured information.

Future on-chain records will not be limited to balance changes, but may also include:

- order parameters;
- business proofs or vouchers;
- protocol interaction explanations;
- upper-layer business context;
- human-readable information for wallets and audit tools.

### 3. Transactions gain flow capability

Through AST, transactions are no longer limited to linear execution. They can express branching and selection.

That means future transactions can look more like:

- an on-chain order;
- an on-chain business process;
- a condition-triggered automated instruction;
- a bundled atomic operation.

### 4. Transactions gain unified settlement capability

Through TEX, multi-asset and multi-step operations can complete unified settlement within the same transaction context.

This matters greatly for DeFi, because the value of advanced DeFi is not that it has “more actions,” but that:

- the multiple steps remain consistent with one another;
- failures do not leave dirty intermediate state behind;
- settlement can complete atomically in one shot.

### 5. On-chain assets become genuinely more diverse

Through HIP20, third-party assets can enter the unified on-chain asset system.

That means Hacash will no longer be only a base-coin transfer network. It can gradually support:

- stablecoin-like applications;
- commercial points and tokens;
- digital notes;
- game assets;
- a broader set of financial assets on-chain.

### 6. Accounts and contracts become more flexible

Through P2SH and HVM, account control and protocol logic gain much stronger programmability.

For example:

- multi-condition unlocks;
- script-based custody;
- contract-driven strategies;
- more flexible wallet structures;
- more complex automated interaction and business logic.

Behind this is the rise of account abstraction.  
Accounts will no longer simply mean “whoever holds the private key sends the transaction.” They can gradually evolve into:

- accounts with custom receive/transfer logic;
- accounts that react to inbound asset events;
- accounts that combine permissions, business rules, and payment entry points;
- on-chain entities closer to “smart accounts.”

### 7. On-chain business becomes more intent-driven

The importance of Intent lies in moving on-chain business from “manually writing every action step by step” toward “organizing execution around the goal.”

Put simply, intent does not first emphasize “how exactly to do it.” It first emphasizes:

- what I want to trade;
- what I want to swap;
- what conditions I want to be satisfied;
- what resources and permissions should be organized around that goal.

This is especially important for advanced DeFi, automated strategies, complex settlement, and multi-step business processes.  
It makes on-chain execution closer to “goal-oriented orchestration” rather than “stacking low-level commands.”

### 8. On-chain storage gains a sense of lease and lifecycle

Contract state leasing is another feature in this upgrade that is easy to underestimate, but is in fact very important.

Its significance is that on-chain state is not treated as free and permanently accumulated forever.  
State gains explicit semantics for:

- lifetime;
- renewal;
- recovery;
- deletion and reclamation.

This creates two very important long-term benefits:

1. **It gives on-chain storage resources a real price, preventing unlimited bloat;**
2. **It helps the chain remain sustainable even while supporting more complex applications.**

For any public chain that truly aims to support large-scale on-chain business, this is an important institutional foundation.

---

## What Is the Most Important Innovation in the Istanbul Upgrade?

If only one point must be emphasized, it is not any single isolated feature, but the overall combination:

> **Hacash is turning the transaction from a static data package into a layered, semantic, and auditable business orchestration unit.**

This matters a great deal.

Because in the end, competition among public chains is not about who can add more opcodes. It is about:

- who can express on-chain business more naturally;
- who can support complex financial operations at lower cost;
- who can help wallets, gateways, explorers, and audit systems truly understand transactions;
- who can remain clear, verifiable, and controllable as complexity rises.

What the Istanbul Upgrade gives Hacash is not just “more features,” but a meaningful step forward in **expressiveness, orchestration capability, and ecosystem-carrying capacity**.

---

## Protocol Cost and Gas Pricing Model

As on-chain capabilities become stronger, the protocol also needs clearer boundaries for state growth and complex execution.  
The Istanbul Upgrade opens new capabilities while also making the pricing model more explicit.

### Where protocol_cost is charged

The most important protocol_cost charging points in this upgrade are three categories:

- **HIP20 third-party asset issuance**  
  A clear protocol fee is charged when third-party assets enter the protocol-level asset system.

- **HVM contract deployment**  
  protocol_cost is charged when new contracts are deployed on-chain and occupy long-term state and storage resources.

- **HVM contract upgrades**  
  protocol_cost is charged when contracts add bytes, edit bytes, or expand state.

In simple terms:

> **protocol_cost mainly appears where new long-term on-chain state is introduced or long-term protocol resource usage is expanded.**

It is not a “feature tax.” It is a resource-pricing mechanism.  
Whoever causes the protocol to maintain more long-term state should pay for that resource cost.

### How gas is charged

The gas model after the Istanbul Upgrade is clearer and more suitable for programmable execution paths:

- gas is mainly used for programmable execution such as Type3, AST, and HVM;
- the transaction first reserves a maximum gas budget;
- unused portions are refunded after execution;
- the actually used portion is settled according to the protocol pricing model;
- **used gas is burned, not paid to miners.**

This point is very important.

It means gas is better understood as:

- a protocol-level resource consumption cost;
- a system boundary on complex execution;
- a pricing mechanism that helps keep the chain sustainable.

Rather than simply being “a tip for miners.”

### Why this model matters

As the chain moves further into advanced DeFi and complex business, it has to face at least three issues:

1. **long-term state cannot grow for free without limit;**
2. **complex execution cannot exist without explicit resource boundaries;**
3. **programmable accounts, intent organization, and contract storage all need sustainable economic boundaries.**

protocol_cost and gas are part of building those long-term rules.

### Can protocol_cost be adjusted in the future?

Yes.

The current protocol_cost design is relatively conservative because the initial priority is to establish clear and stable resource boundaries during the early phase of the upgrade.  
If the HAC price rises significantly in the future, then new **HIP proposals** can lower the relevant protocol_cost parameters.

That means:

> **Today’s fee model is not fixed forever. It can continue to evolve with ecosystem maturity, token price changes, and governance decisions.**

In other words, the protocol prices resource usage while still leaving room to reduce access costs in the future.

---

## Conclusion

The significance of the Istanbul Upgrade does not lie in how many individual features it adds, but in the fact that it further organizes transactions, accounts, assets, settlement, contracts, and state management—capabilities that were previously more scattered—into a more complete programmable financial foundation.

From ActionGuard, TxBlob, AST, and TEX to HIP20, P2SH, HVM, IR decompilation, account abstraction, Intent, and state leasing, this upgrade does not focus on one isolated direction. Rather, it systematically strengthens several key layers required for programmable on-chain finance.

These capabilities do not automatically create an ecosystem by themselves, but they do materially change one important thing:  
Hacash will have stronger underlying conditions for supporting complex financial protocols, on-chain asset coordination, and multi-step business workflows.

From that perspective, the Istanbul Upgrade can be understood as a round of **foundation building**:

- it makes transaction expression clearer;
- it makes execution flows more composable;
- it makes account entry points more flexible;
- it makes contract logic easier to audit;
- it gives state growth clearer cost boundaries;
- and it makes it easier for monetary-financial and DeFi scenarios to be implemented within one coherent system.

The long-term goal of Hacash has never been to become a blockchain that is merely “feature-rich” in an abstract sense. It has been to gradually build an **open financial system centered on sound money**.  
If that goal requires monetary foundations, an asset system, programmable transactions, programmable accounts, programmable settlement, and programmable contracts to work together, then what the Istanbul Upgrade does is move those key underlying pieces substantially forward.

A more appropriate way to put it is not that “this upgrade itself equals ecosystem explosion,” but rather:

> **It provides a more complete, more stable, and more adoptable foundation for Hacash to enter a more active stage of ecosystem development.**

How far the ecosystem develops from there will still depend on sustained participation from developers, asset issuers, protocol designers, wallets, and infrastructure builders.  
But in terms of technical readiness at the foundation layer, the Istanbul Upgrade is indeed an important starting point.

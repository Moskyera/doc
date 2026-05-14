# Hacash Multi-Layer Programmability Architecture

## Executive Summary

Hacash employs a unique "layered progressive" programmability architecture. From atomic asset operations (Layer 1) to fully Turing-complete contracts (Layer 6), each layer extends capabilities on top of the previous one while maintaining backward compatibility. Notably, Layer 5's IR decompilation capability allows on-chain contract bytecode to be fully reconstructed into fitsh source code, granting contracts "native auditability" — a uniquely valuable property for high-value financial settlement and institutional DeFi scenarios. This report provides an objective technical assessment from the perspective of high-value financial settlement and DeFi public chains.

---

## 1. Architecture Overview

```
Layer 6 ─ Contract state read/write, custom assets, arbitrary DeFi
Layer 5 ─ IR decompilation & dual-format VM code (fitsh ↔ bytecode)
Layer 4 ─ Account abstraction (AbstCall), P2SH script accounts
Layer 3 ─ In-transaction script execution (MainCall + EXTACTION)
Layer 2 ─ AST conditional logic (AstSelect / AstIf)
Layer 1 ─ Atomic action composition (AND-semantics transactions)
```

Each transaction (tx) contains an ordered list of actions. All actions must succeed, or the entire transaction rolls back. This is the foundational transactional guarantee.

---

## 2. Layer-by-Layer Analysis

### 2.1 Layer 1: Atomic Action Composition

**Mechanism**: A transaction contains `Vec<Box<dyn Action>>`, executed sequentially. If any action fails, the entire tx rolls back. Each action is a typed struct (e.g., `HacToTrs`, `SatFromTrs`, `DiaFromToTrs`) corresponding to a deterministic asset operation.

**Strengths**:
- Declarative semantics: Transfer intent is fully visible in the transaction body without execution. Miners, wallets, and explorers can parse the action list to determine asset flows — critical for transparency in high-value settlement.
- Native multi-asset: HAC, SAT (one-way Bitcoin transfer), HACD (diamond NFT), and custom Assets have dedicated action types at the protocol level, rather than being implemented through contract token standards. This eliminates the attack surface common in ERC-20-style approve/transferFrom two-step patterns.
- Deterministic gas: Each action's gas cost equals its serialized size (`self.size() as u32`), calculable before submission.
- Native multi-party signing: `AddrOrList` allows transactions to carry multiple addresses with corresponding signatures. `req_sign()` collects all required signer addresses across all actions. Multi-party atomic swaps require no additional contract logic.

**Limitations**:
- Action types are protocol-hardcoded; adding new asset operation types requires a hard fork.
- AND semantics cannot express "try A, if it fails do B" — precisely what Layer 2 addresses.

**Comparison**: Bitcoin's UTXO scripts are declarative but lack native multi-asset support and multi-action composition. Ethereum transactions can only call a single entry point; multi-step operations must be orchestrated through contracts. Hacash's Layer 1 has a structural advantage in declarative auditability and multi-asset atomic operations.

**Practical scenario**: A $50 million cross-border trade settlement where the buyer pays HAC, the seller delivers HACD asset certificates, and the guarantor releases collateral — all three steps complete atomically in a single tx via Layer 1's multi-action AND composition. If any step fails, everything rolls back. The entire logic is fully visible in the transaction body; miners and auditors can verify asset flows without execution.

---

### 2.2 Layer 2: AST Conditional Logic

**Mechanism**: `AstSelect` and `AstIf` are special action types that can nest other actions, forming an abstract syntax tree.

- `AstSelect(min, max, actions[])`: Attempts each child action sequentially. Successful ones are retained (merge), failed ones are rolled back (recover). Finally checks whether the success count falls within `[min, max]`.
- `AstIf(cond, br_if, br_else)`: Executes cond; on success, merges cond state and executes br_if; on failure, recovers cond state and executes br_else.

**State isolation**: Before each attempt, `ctx_snapshot()` captures a complete snapshot (state fork, VM volatile, logs, ctx volatile). On failure, `ctx_recover()` performs a full rollback. Gas consumption is monotonic — gas consumed by failed branches is never refunded, preventing free-probing attacks.

**Depth limit**: AST nesting depth is capped at 6 levels (`AST_TREE_DEPTH_MAX`), enforced via `AstLevelGuard` RAII guard that guarantees correct level counter restoration.

**Strengths**:
- Achieves conditional logic without introducing Turing-complete scripting, preserving Layer 1's declarative auditability. The AST structure in the transaction body can be statically analyzed — all possible execution paths and involved asset operations are fully visible before submission.
- Highly valuable for high-value financial settlement: enables constructing atomic transactions like "if condition A is met, execute plan X; otherwise execute plan Y" without trusting any contract code.
- `collect_req_sign()` recursively collects signature requirements from all branches, ensuring that regardless of which path is taken, all required signatures are already provided.

**Limitations**:
- Condition expressiveness is limited — the cond branch's success/failure is the only branching criterion; finer-grained branching based on return values is not possible.
- 6-level nesting depth may be insufficient for complex multi-party negotiation scenarios. However, as an on-chain consensus layer constraint, 6 levels cover the vast majority of practical scenarios.

**Comparison**: This is one of Hacash's most distinctive designs. Bitcoin Script has no conditional action composition capability. Ethereum has no transaction-level conditional branching (must be implemented through contracts). The closest analogy is Cosmos SDK's `MsgMultiSend`, but it lacks conditional logic.

**Practical scenario**: Institutional overnight lending — using AstIf to construct an atomic transaction: "if collateral ratio meets threshold, disburse loan and lock collateral; otherwise return principal." The entire conditional logic and asset operations are statically visible in the transaction body, requiring no trust in any contract code. For large-value conditional payments, this "contract-free conditional settlement" capability eliminates smart contract vulnerability risk while maintaining full auditability.

---

### 2.3 Layer 3: In-Transaction Script Execution

**Mechanism**: `TransactionType3` introduces a `gas_max` field, allowing transactions to carry VM-executable script code. `setup_vm_run()` enters VM execution, and the VM can call back into the protocol layer's action system via the `EXTACTION` instruction.

**Execution flow**:
1. `do_tx_execute()` calls `do_vm_init()` to pre-initialize the VM before executing actions
2. MainCall action enters the VM via `setup_vm_run(ctx, Main, ...)`
3. The VM executes the script; upon encountering `EXTACTION`, it calls back through `host.ext_action_call()` → `ctx.action_call()` → `ctx_action_call()` to the protocol layer to create and execute actions
4. `ctx_action_call()` uniformly handles burn_90 gas multiplier and signature verification

**EXTACTION security restrictions**:
- Only `Main` mode, `depth == 0`, non-`callcode` context allows EXTACTION
- EXTACTION is forbidden in contract calls (Abst/Outer/Inner) and nested calls
- This ensures asset transfer operations can only be triggered by the transaction initiator's top-level script

**Gas model**:
- `GasCounter` manages the entire tx's gas budget; all VM calls share the same counter
- Each call type has a minimum gas cost (`main_call_min`, `abst_call_min`, `p2sh_call_min`)
- Re-entry depth limit (`max_reentry_depth = 4`) managed via `GasCallGuard` RAII guard
- Gas settlement (HAC burn) executes only once when the outermost call returns

**Strengths**:
- Scripts can orchestrate complex multi-step operations while reusing Layer 1's atomic asset operations through EXTACTION, avoiding reimplementation of transfer logic in the script layer.
- EXTACTION's strict restrictions prevent contract code from directly initiating asset transfers — a significant security improvement over Ethereum where contracts can arbitrarily call `transfer()`.
- Shared gas counter and re-entry depth limits provide a predictable upper bound on resource consumption.

**Limitations**:
- EXTACTION is limited to top-level Main calls; contracts cannot transfer assets to each other via EXTACTION — they must use the AbstCall hook mechanism. This increases the complexity of inter-contract interaction.
- The scripting language (fitsh) is Hacash-custom; ecosystem tooling and developer community must be built from scratch.

---

### 2.4 Layer 4: Account Abstraction & P2SH

**Mechanism**: Hacash's address system distinguishes three types:
- `privakey`: Traditional private key addresses
- `contract`: Contract addresses (calculated from deployer address + nonce)
- `scriptmh`: Script hash addresses (P2SH)

When Layer 1 asset transfer actions involve contract or script hash addresses, the `action_hook` system automatically triggers corresponding VM calls:

**AbstCall (Contract Account Abstraction)**:
- `PermitHAC/SAT/HACD/Asset`: Triggered when transferring assets out of a contract address; the contract can verify and authorize
- `PayableHAC/SAT/HACD/Asset`: Triggered when transferring assets into a contract address; the contract can execute receive logic
- `Construct`: Constructor function during contract deployment
- `Change`/`Append`: Contract upgrades

**P2SH (Script Hash Accounts)**:
- Address is generated from a script hash; spending requires providing the original script and witness data
- The script executes in the VM to verify whether the transfer is authorized

**Strengths**:
- Account abstraction is protocol-native, requiring no additional infrastructure layer like ERC-4337. Every contract address inherently has Permit/Payable hook capabilities.
- P2SH provides flexible custom signature verification, supporting multisig, timelocks, conditional payments, etc., without deploying a contract.
- The hook mechanism decouples Layer 1 asset operations from Layer 4 account logic — the action itself is unaware of whether the target address is a contract; the hook system handles it transparently.

**Limitations**:
- AbstCall types are hardcoded enums (Construct, Change, Permit*, Payable*); adding new abstract call types requires a protocol upgrade.
- P2SH scripts must provide complete code with each transfer (unlike contracts which are persistently stored), increasing transaction size.

**Comparison**: Ethereum's ERC-4337 account abstraction requires EntryPoint contracts, Bundlers, Paymasters, and other complex infrastructure. Hacash's approach is more concise — contract addresses inherently possess account abstraction capabilities without additional deployment. However, ERC-4337 offers greater flexibility (custom verification logic is not constrained by enum types).

---

### 2.5 Layer 5: IR Decompilation & Dual-Format VM Code

**Mechanism**: The Hacash VM contract code system supports two equivalent representation formats: the fitsh high-level language and raw bytecode. The compilation pipeline is fitsh → IR (intermediate representation) → bytecode, and this pipeline is fully reversible: bytecode → IR → fitsh.

**IR layer design**:
- IR defines container nodes including IRLIST (0xF1), IRBLOCK (0xF2), IRBLOCKR (0xF3), IRIF (0xF4), IRIFR (0xF5), and IRWHILE (0xF6), providing precise control over stack discipline and evaluation order
- Each IR node marks whether it produces a return value via `hasretval()`; the compiler uses this to automatically insert POP instructions for stack balance
- IR nodes are serializable/deserializable, guaranteeing roundtrip stability

**Decompilation capability**:
- `formater.rs` + `decompilation_helper.rs` can fully reconstruct readable fitsh source code from raw on-chain bytecode
- Supports SourceMap symbol name recovery, producing decompiled output approaching original source readability
- Recognizes patterns such as PACKLIST → `[...]`, PACKMAP → `map{...}`, LOGn → `log(...)` to reconstruct high-level syntax structures

**Strengths**:
- Native auditability: Any third party can independently reconstruct and verify contract logic from on-chain bytecode without relying on developers to provide source code. This is a rare property in public chain ecosystems — contrast with Ethereum, where users must trust that the source code uploaded by development teams on Etherscan actually matches the deployed bytecode.
- For high-value financial scenarios, this eliminates the trust black box of "closed-source contracts." Regulators, auditors, and counterparties can all independently verify every line of contract logic.
- The bidirectional compilation pipeline ensures fitsh is not a "decorative layer" on top of bytecode, but a strictly equivalent representation — providing a solid foundation for formal verification and automated audit tooling.

**Limitations**:
- Decompilation produces structurally equivalent source code; semantic information such as variable names depends on SourceMap availability. Without SourceMap, decompiled output readability decreases (but logical completeness is unaffected).
- The decompilation toolchain's maturity and coverage require ongoing validation to ensure all bytecode patterns are correctly reconstructed.

**Practical scenario**: After a DeFi vault contract managing $100 million in assets is deployed on-chain, any auditor can decompile the complete fitsh source directly from on-chain bytecode, verifying fund management logic, permission controls, and liquidation rules line by line. Institutional investors can independently complete contract audits before depositing large sums into a protocol, without relying on the project team's "open-source promise." For compliance review and institutional DeFi adoption, this represents a fundamental trust infrastructure upgrade.

---

### 2.6 Layer 6: Contract State & DeFi

**Mechanism**: The VM provides full state read/write capabilities:
- `storage_save(key, value)`: Persistent storage, charged via rent model
- `storage_load(key)`: Read storage
- `storage_del(key)`: Delete storage
- `storage_rent(key, amount)`: Pay storage rent
- `storage_rest(key)`: Query remaining rent period

**Contract system features**:
- Inheritance chain: Contracts can declare inheritance; `super.f()` traverses up the chain
- Library references: Other contracts can be referenced as libraries via `lib`
- Function visibility: `public` functions are externally callable; private functions are internal only
- `callcode`: Similar to Solidity's `delegatecall`, executes target code in the current context

**Storage rent model**:
- Storage is not permanently free; rent must be paid per block height
- Expired storage entries are treated as non-existent (`sload` returns empty)
- Renewal via `storage_rent`

**Resource limits**:
- `SpaceCap` defines upper bounds for stack depth, heap size, global variable count, memory capacity, etc.
- Contract loading charges gas per byte (`contract_load_bytes / 64`)
- Each new contract load incurs a fixed gas charge (`load_new_contract`)

**Strengths**:
- The storage rent model addresses "state bloat" — one of Ethereum's core unsolved challenges. For long-running DeFi protocols, this forces developers to design more efficient storage schemes.
- Contract inheritance and library systems provide code reuse, reducing redundant on-chain deployment.
- Granular resource limits (stack, heap, globals limited separately) prevent specific types of resource exhaustion attacks more effectively than Ethereum's single gas limit.

**Limitations**:
- Storage rent increases operational complexity for DeFi protocols — protocols must continuously pay rent or design user cost-sharing mechanisms.
- The fitsh language ecosystem maturity is far below Solidity/Vyper, lacking mature audit tools, formal verification frameworks, and developer community.
- Inter-contract interaction must go through AbstCall hooks or Outer calls; there is no Ethereum-style arbitrary contract-to-contract calling flexibility.

---

## 3. Assessment from a High-Value Financial Settlement Perspective

### 3.1 Strengths

**Auditability**: Layer 1-2's declarative design allows complete pre-submission audit of asset flows in high-value transactions. This is critical for institutional settlement — compliance teams can verify all possible paths of a transaction before signing. Layer 5's IR decompilation capability further extends auditability to the contract layer — any contract deployed on-chain can be independently reconstructed into fitsh source code for review.

**Atomicity guarantees**: Multi-party atomic swaps are natively supported at Layer 1, with no contract dependency. Combined with Layer 2's conditional logic, complex conditional settlement schemes can be constructed (e.g., "if Party A's assets are in place, execute the swap; otherwise refund"). For example, a large OTC trade involving HAC, BTC (SAT), and HACD assets uses Layer 1's atomic AND composition to ensure all three assets settle simultaneously or roll back entirely — there is no intermediate state where "HAC was paid but BTC was not received."

**Minimized attack surface**:
- Asset operations are hardcoded at the protocol layer, eliminating contract-level transfer vulnerabilities
- EXTACTION restrictions prevent contract code from directly initiating asset transfers
- AbstCall hooks provide contract-level asset inflow/outflow control
- Monotonic gas consumption prevents free-probing attacks

**Contract transparency**: Layer 5's bytecode → fitsh decompilation capability eliminates the trust black box of "closed-source contracts." Regulators can reconstruct contract logic directly from on-chain bytecode, independently verifying whether fund flow rules comply with regulatory requirements, without trusting third-party-provided source code.

**State controllability**: The storage rent model ensures chain state does not grow unboundedly — critical for long-running settlement systems.

### 3.2 Risks & Concerns

**Double-edged sword of protocol-level hardcoding**: Asset operation types, AbstCall types, and action types are all protocol-hardcoded. This provides security but means new feature introduction is gated by hard fork cycles. For a rapidly evolving DeFi ecosystem, this may be a bottleneck.

**Unsafe code in VM safety-critical path**: `setup_vm_run()` uses raw pointers to work around Rust borrow checker limitations. While comments thoroughly document safety assumptions (single-threaded, no concurrent replacement), this is unsafe code on the consensus-critical path requiring extra caution.

**Global mutable state**: `ACTION_HOOK_FUNC` and `VM_ASSIGN_FUNC` use `static mut` globals accessed via `unsafe`. While safe in single-threaded consensus execution, this would become a data race source if parallel execution is introduced in the future.

**fitsh language maturity**: As a custom language, fitsh lacks:
- Mature formal verification tools
- Extensive security audit experience accumulation
- Large-scale production environment validation

This is a material risk for DeFi protocols handling high-value financial assets. However, Layer 5's IR decompilation capability partially mitigates this risk — even if the fitsh toolchain is not yet mature, anyone can independently verify the actual behavior of on-chain contracts through decompilation.

### 3.3 Comparative Positioning Against Major Chains

| Dimension | Hacash | Ethereum | Bitcoin |
|-----------|--------|----------|---------|
| Transaction-level atomic composition | Native multi-action AND semantics | Single entry point, requires contract orchestration | UTXO atomicity, no multi-step composition |
| Conditional logic | AST layer (Layer 2) declarative | In-contract if/else | Limited Script conditions |
| Asset operations | Protocol-level hardcoded | Contract standards (ERC-20, etc.) | UTXO native |
| Account abstraction | Protocol-native | ERC-4337 (infrastructure layer) | None |
| Contract transparency | Native decompilation (Layer 5) | Depends on developer-uploaded source | No contracts |
| State management | Rent model | Permanent storage | Stateless |
| Contract flexibility | AbstCall + Outer call | Arbitrary inter-contract calls | No contracts |
| Auditability | Transaction body statically analyzable + contracts decompilable | Requires simulation | UTXO traceable |

---

## 4. Conclusion

Hacash's multi-layer programmability architecture is a deliberately designed system that makes explicit trade-offs between security and flexibility:

**Core design philosophy**: Solidify the most common financial operations (asset transfers) at the protocol layer, progressively opening more advanced programmability through layered escalation. Each layer has clear capability boundaries and security constraints. The six-layer architecture allows simple settlements and complex DeFi to each use the appropriate level, without forcing an either-or choice between security and flexibility.

**Best-suited scenarios**: High-value asset settlement, multi-party atomic swaps, conditional payments, financial applications requiring high auditability. Layer 1-2's declarative design has a structural advantage in these scenarios. Layer 5's IR decompilation capability provides native on-chain support for institutional compliance audits.

**Trade-off scenarios**: Highly flexible DeFi protocols (e.g., complex AMMs, lending protocols). Inter-contract interaction restrictions and AbstCall enum type constraints may increase development complexity. However, the six-layer progressive design allows simple scenarios (e.g., token swaps) to be completed at Layer 1-2, reserving only truly state-dependent complex logic for Layer 6.

**Key risks**: The fitsh language ecosystem maturity is currently the most material risk, but Layer 5's decompilation capability provides an important risk mitigation — on-chain contract logic can always be independently verified. The evolution speed of protocol-level hardcoding is a long-term concern.

From an engineering implementation quality perspective, the codebase demonstrates a high degree of attention to consensus safety — RAII guards, monotonic gas consumption, EXTACTION restrictions, state snapshot/recovery mechanisms, and IR bidirectional compilation fidelity are all carefully considered designs. Test coverage includes multi-level nesting, partial commit rollback, depth limits, and other critical scenarios.

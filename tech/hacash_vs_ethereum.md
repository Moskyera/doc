# How Hacash's Multi-Layer Programmability Addresses Ethereum's Problems

## Executive Summary

Since its 2015 launch, the Ethereum community has continuously patched architectural deficiencies through EIP proposals, yet many core problems remain unsolved. Hacash's five-layer programmability architecture structurally avoids several of these issues by design. This report provides an item-by-item comparison, distinguishing "structural resolution" (architecturally eliminated) from "significant improvement" (mitigated but not fully eliminated).

---

## 1. Major Problems

### 1.1 ERC-20 approve/transferFrom Authorization Model Flaw

**Ethereum status**: ERC-20's `approve` + `transferFrom` two-step pattern is one of DeFi's largest attack surfaces. Users must first authorize (typically unlimited allowance), then a contract transfers on their behalf. This has led to massive approval phishing attacks and unlimited allowance risks. EIP-2612 (permit) introduced off-chain signature authorization, EIP-7702 further improved it, but the fundamental problem — the "delegated authorization" model — remains.

**Hacash's solution**: Layer 1's atomic action composition eliminates this problem at its root. Asset transfers are native protocol-level actions (`HacToTrs`, `HacFromTrs`, `DiaFromToTrs`, etc.) requiring no "authorize-then-transfer" two-step pattern. Multi-party asset swaps complete atomically within a single tx through multiple actions, each directly requiring the corresponding address's signature via `req_sign()`. The concept of "delegated authorization" does not exist.

```
// Ethereum: two-step operation, authorization can be abused
token.approve(spender, MAX_UINT256);  // Step 1: unlimited approval
spender.transferFrom(user, to, amt);  // Step 2: delegated transfer

// Hacash: atomic completion within single tx, no authorization concept
tx.actions = [
    HacFromTrs { from: alice, amount: 100 },  // alice signs
    HacToTrs   { to: bob,   amount: 100 },    // no additional authorization needed
]
```

**Rating: Structural resolution.** Protocol-native asset operations + multi-action atomic composition architecturally eliminate the authorization model.

---

### 1.2 Account Abstraction Complexity

**Ethereum status**: Native EOAs (Externally Owned Accounts) only support ECDSA signatures with no custom verification logic. EIP-4337 introduced account abstraction but requires an EntryPoint contract, Bundler network, Paymaster, and other complex infrastructure. EIP-7702 allows EOAs to temporarily delegate code, but remains a patch-style solution. As of 2025, full account abstraction is still not a protocol-native capability.

**Hacash's solution**: Layer 4's AbstCall mechanism is protocol-native account abstraction. Every contract address inherently has `Permit*` (outbound authorization) and `Payable*` (inbound handling) hooks. When Layer 1 asset transfer actions involve contract addresses, the `action_hook` system automatically triggers corresponding VM calls — no additional infrastructure required.

P2SH (script hash accounts) further provides custom signature verification without deploying contracts — multisig, timelocks, conditional payments only require providing a script and witness.

**Rating: Structural resolution.** Account abstraction is a protocol-native capability, requiring no EntryPoint/Bundler/Paymaster infrastructure stack.

---

### 1.3 State Bloat

**Ethereum status**: Ethereum storage is permanent — once written, data occupies full node storage forever unless the contract explicitly calls `SSTORE(key, 0)`. This causes continuous state growth (exceeding 300GB by 2025). EIP-4444 (history expiry), EIP-4762 (Verkle tree gas adjustments), and Vitalik's repeatedly proposed "state expiry" schemes have not materialized. EIP-7702 and the EOF series also do not address this fundamental issue.

**Hacash's solution**: Layer 5's storage rent model solves state bloat through economic mechanisms.

Code evidence (`vm/src/field/storage.rs:363-424`):
- `ssave` charges rent based on `value_len + base_size` when writing storage; new keys incur additional `key_create_fee`
- Storage has an expiration time (`expire` field); expired entries return Nil on `sload`
- `srent` allows renewal, `sdel` enables active deletion
- Data past the grace period is physically reclaimed (`is_delete` branch)
- `ssave` auto-renews for one period when remaining lease is insufficient

**Rating: Structural resolution.** Storage rent + expiry reclamation solves state bloat at both economic and technical levels, while Ethereum still has no viable state expiry scheme.

---

### 1.4 Transaction Auditability & MEV

**Ethereum status**: Ethereum transactions have only one entry point (`to` address + `calldata`); actual asset flows can only be determined through simulation. This means: (1) users cannot know exactly what a transaction will do before signing; (2) MEV searchers can discover arbitrage opportunities through simulation and execute sandwich attacks. EIP-3074 (AUTH/AUTHCALL) and EIP-7702 attempted to improve UX but did not address auditability.

**Hacash's solution**: Layer 1-2's declarative design makes transaction bodies statically analyzable.

- Layer 1: All asset operations (transfers in, out, swaps) are explicitly declared in the action list; asset flows are determinable without execution
- Layer 2: All branches of `AstSelect` and `AstIf` are visible in the transaction body; `collect_req_sign()` recursively collects signature requirements from all possible paths
- Wallets and audit tools can display all possible execution paths and asset changes before signing

Regarding MEV: Since Layer 1-2 transactions don't involve contract state reads, their execution results don't depend on transaction ordering — naturally immune to sandwich attacks. Only Layer 3-5 transactions involving VM execution may be affected by MEV.

**Rating: Significant improvement (Layer 1-2 structurally resolved; Layer 3-5 similar to Ethereum).**

---

### 1.5 Reentrancy Attacks

**Ethereum status**: Reentrancy is Ethereum's most classic security issue (The DAO incident). While the Solidity community developed ReentrancyGuard patterns and checks-effects-interactions best practices, these rely on developer discipline. EIP-1153 (transient storage) provides more efficient reentrancy locks, but remains optional.

**Hacash's solution**: Multi-layered defense.

1. **Layer 1-2 naturally immune**: Atomic actions and AST conditional logic involve no external calls — no reentrancy path exists.
2. **Layer 3 EXTACTION restrictions**: `EXTACTION` is only allowed in `Main` mode, `depth == 0`, non-`callcode` context (`execute.rs:245-248`). Contract code cannot initiate asset transfers via EXTACTION, cutting off the classic reentrancy path.
3. **Layer 4-5 hard reentry depth limit**: `GasCounter.reentry_depth` is managed via `GasCallGuard` RAII guard with a hard cap of `max_reentry_depth = 4` (`SpaceCap`). Exceeding the limit triggers an immediate error — no reliance on developer-implemented locks.

```rust
// machine.rs - Protocol-enforced reentry depth limit
fn enter(&mut self) -> Rerr {
    let next_depth = self.reentry_depth.checked_add(1)...;
    if next_depth > self.max_reentry + 1 {
        return errf!("re-entry depth {} exceeded limit {}", ...)
    }
    ...
}
```

**Rating: Structural resolution (Layer 1-2) + Significant improvement (Layer 3-5 protocol-enforced limits, no developer reliance).**

---

### 1.6 Multi-Party Atomic Swap Complexity

**Ethereum status**: Multi-party atomic swaps require contract implementation (e.g., HTLC or dedicated swap contracts), involving multiple transactions, timelocks, timeout refunds, and other complex logic. EIP-7702 improved single-user batch operations, but multi-party scenarios still require contract orchestration.

**Hacash's solution**: Layer 1 natively supports multi-party atomic swaps. A single tx can contain multiple `FromTrs`/`ToTrs` actions involving different addresses and asset types (HAC, SAT, HACD, custom Assets), atomically executed after all parties sign.

Combined with Layer 2's `AstIf`, conditional atomic swaps can be constructed:
```
AstIf {
    cond: [check some on-chain condition],
    br_if: [
        HacFromTrs { from: alice, amount: 100 },
        DiaFromToTrs { from: bob, to: alice, diamond: "WTYUIA" },
    ],
    br_else: [no-op or alternative plan],
}
```

**Rating: Structural resolution.** Protocol-native support — no contracts, no HTLC, no multiple transactions needed.

---

### 1.7 User-Friendly Auditability of Signed Content

**Ethereum status**: A common attack path is not a protocol bug itself, but frontend substitution of signing payloads. In wallets, users often see raw `hex` blobs or complex typed structures (especially across `eth_sign` / `personal_sign` / permit-style flows), making it hard to tell who is being authorized, what is being authorized, and which execution paths may follow. If the frontend is compromised, users can unknowingly sign dangerous approvals that attackers later submit on-chain to drain assets.

**Hacash's solution**: Readable contracts and script contracts (P2SH) bind signatures to human-readable semantics. Layer 1-2 action/AST transaction bodies are declarative by design, so wallets can show clear pre-sign summaries: asset type, amount, source/target addresses, conditional branches, and required signer sets. Users sign auditable business intent rather than opaque `hex` payloads, significantly reducing the attack surface of frontend signature substitution.

**Rating: Structural resolution (Layer 1-2 + readable contracts/P2SH).**

---

## 2. Medium Problems

### 2.1 Gas Metering Predictability

**Ethereum status**: EVM gas metering depends on runtime state (cold/warm storage access, SSTORE refund rules, etc.). EIP-2929 (cold/warm access lists), EIP-3529 (reduced SSTORE refunds), EIP-7623 (calldata gas adjustments) continuously patch this, but gas estimation remains imprecise, frequently causing transaction failures or overpayment.

**Hacash's solution**: Layered gas model.

- Layer 1-2: Gas cost equals the action's serialized size (`self.size() as u32`) — fully deterministic, precisely calculable before submission.
- Layer 3-5: VM execution gas is based on an instruction table (`GasTable`); storage operations charge linearly by value size with no cold/warm access distinction. Each call type has minimum gas (`main_call_min`, `abst_call_min`, `p2sh_call_min`), preventing extremely low-cost spam calls.
- Monotonic gas consumption: Gas consumed by failed AST branches is never refunded (`GasCounter.remaining` comment: `never restored by AST recover`), eliminating gas refund complexity.

**Rating: Significant improvement (Layer 1-2 fully deterministic; Layer 3-5 simpler model but still requires estimation).**

---

### 2.2 Contract Upgrade Safety

**Ethereum status**: Contract upgrades rely on proxy patterns (Proxy Pattern), with risks including storage layout conflicts, implementation contracts being accidentally self-destructed (partially addressed by EIP-6780), and upgrade permission management. OpenZeppelin's UUPS/Transparent Proxy is the de facto standard but adds complexity and attack surface.

**Hacash's solution**: Layer 5's contract system has built-in upgrade mechanisms.

- `ContractUpdate` action (kind=98) is a protocol-level upgrade operation
- `ContractEdit` supports adding new functions (`Append`) and modifying existing ones (`Change`)
- Upgrades require contract owner signature
- `revision` field tracks version numbers
- Contracts can define `Change` AbstCall hooks to implement custom upgrade verification logic

No proxy pattern needed; no storage layout conflict issues (storage keys are explicit, not dependent on slot numbers).

**Rating: Significant improvement.** Protocol-native upgrade mechanism eliminates proxy pattern complexity and risks.

---

### 2.3 Transaction Batching

**Ethereum status**: A single Ethereum transaction can only call one function on one contract. Batch operations require Multicall contracts or EIP-7702's temporary code delegation. EIP-3074 (AUTH/AUTHCALL) was superseded by EIP-7702, but batching is still not a protocol-native capability.

**Hacash's solution**: Layer 1 natively supports batch operations — a single tx can contain any number of actions, executed atomically in sequence. This is a foundational protocol design requiring no additional mechanisms.

**Rating: Structural resolution.**

---

### 2.4 Token Standard Fragmentation

**Ethereum status**: ERC-20, ERC-721, ERC-1155, ERC-4626 and other token standards are independently defined; interoperability depends on developers following interface specifications. Non-compliant token implementations (e.g., `transfer` missing return values) have caused numerous compatibility issues. EIP-7575 and others attempt to unify vault interfaces, but fragmentation persists.

**Hacash's solution**: Layer 1's native multi-asset system. HAC, SAT, HACD have dedicated action types at the protocol level with completely consistent behavior — no possibility of "non-compliant implementations." Custom Assets are registered at the protocol level via `AssetSmelt` and transferred through standard actions like `AssetToTrs`/`AssetFromTrs`.

**Limitation**: Custom Assets have less flexibility than ERC-20 (cannot customize transfer logic), but this is precisely the source of security.

**Rating: Structural resolution (native assets) / Partial improvement (custom Asset flexibility limited).**

---

## 3. Minor Problems

### 3.1 Inter-Contract Call Transparency

**Ethereum status**: Inter-contract `CALL`/`DELEGATECALL`/`STATICCALL` are invisible at the transaction level, only analyzable through traces. EIP-3155 (trace standardization) improved debugging but on-chain transparency remains unchanged.

**Hacash's solution**: Layer 3's EXTACTION mechanism makes VM-to-protocol callbacks trackable — every EXTACTION passes through the unified `ctx_action_call()` entry point, enabling recording and auditing. However, inter-contract Outer/Inner calls remain VM-internal behavior with transparency similar to Ethereum.

**Rating: Partial improvement.**

---

### 3.2 Storage Slot Collisions

**Ethereum status**: EVM storage is based on 256-bit slot numbers; in proxy patterns, implementation and proxy contracts may use the same slot, causing storage collisions. EIP-1967 (standard proxy storage slots) and EIP-7201 (namespaced storage layout) are patch solutions.

**Hacash's solution**: Layer 5 storage uses an explicit key-value model (`ssave(key, value)`) where keys are Value types rather than fixed slot numbers. Contract addresses serve as key prefixes (`Self::skey(cadr, &k)`), naturally isolating different contracts' storage spaces. Combined with protocol-native upgrade mechanisms that don't require proxy patterns, storage collision problems fundamentally do not exist.

**Rating: Structural resolution.**

---

### 3.3 Signature Verification Flexibility

**Ethereum status**: EOAs only support secp256k1 ECDSA signatures. EIP-7212 (secp256r1 precompile) provides support for Passkey scenarios, but curves are still added one by one. General signature abstraction requires EIP-4337's UserOperation validation.

**Hacash's solution**: Layer 4's P2SH allows arbitrary signature verification logic — scripts execute in the VM and can implement any verification algorithm. AbstCall's `Permit*` hooks also allow contracts to customize verification. No need to add precompiles for each signature algorithm.

**Rating: Structural resolution.**

---

### 3.4 Users Pay Gas for Failed Transactions

**Ethereum status**: Failed transactions (revert) still consume gas and charge fees. Users paying for failed transactions is a persistent UX pain point.

**Hacash's solution**: Layer 1-2's deterministic gas model significantly reduces transaction failure probability — action execution results are predictable in most scenarios. Layer 2's `AstSelect(min=0)` allows "best-effort" semantics where partial action failures don't cause the entire tx to fail. However, Layer 3-5 VM execution failures still consume gas, consistent with Ethereum.

**Rating: Partial improvement (Layer 1-2 reduces failure probability; Layer 3-5 unchanged).**

---

## 4. Problems Hacash Does Not Solve or Newly Introduces

In fairness, Hacash's design comes with its own costs:

| Problem | Description |
|---------|-------------|
| Protocol evolution speed | Action types and AbstCall types are hardcoded; new features require hard forks |
| Inter-contract interaction flexibility | Cannot arbitrarily call between contracts as in Ethereum; must use AbstCall/Outer call |
| Language ecosystem | fitsh is a custom language lacking mature audit tools and developer community |
| Storage rent operational burden | DeFi protocols must continuously pay rent or design cost-sharing mechanisms |
| EXTACTION restrictions | Contracts cannot directly initiate asset transfers, increasing implementation complexity for certain DeFi patterns |

---

## 5. Summary Comparison Table

| Ethereum Problem | Related EIPs | Hacash Solution Layer | Rating |
|-----------------|-------------|----------------------|--------|
| approve/transferFrom flaw | EIP-2612, EIP-7702 | Layer 1 | Structural resolution |
| Account abstraction complexity | EIP-4337, EIP-7702 | Layer 4 | Structural resolution |
| State bloat | EIP-4444, EIP-4762 | Layer 5 | Structural resolution |
| Transaction auditability / MEV | EIP-3074 | Layer 1-2 | Significant improvement |
| User-friendly auditability of signed content (anti-frontend substitution) | EIP-712, EIP-2612, EIP-7702 | Layer 1-2 + readable contracts/P2SH | Structural resolution |
| Reentrancy attacks | EIP-1153 | Layer 1-5 | Structural + Significant |
| Multi-party atomic swaps | No direct EIP | Layer 1-2 | Structural resolution |
| Gas predictability | EIP-2929, EIP-3529 | Layer 1-2 | Significant improvement |
| Contract upgrade safety | EIP-1967, EIP-6780 | Layer 5 | Significant improvement |
| Transaction batching | EIP-3074, EIP-7702 | Layer 1 | Structural resolution |
| Token standard fragmentation | ERC-20/721/1155 | Layer 1 | Structural resolution |
| Storage slot collisions | EIP-1967, EIP-7201 | Layer 5 | Structural resolution |
| Signature verification flexibility | EIP-7212, EIP-4337 | Layer 4 | Structural resolution |
| Failed tx gas charges | No direct EIP | Layer 1-2 | Partial improvement |
| Inter-contract call transparency | EIP-3155 | Layer 3 | Partial improvement |

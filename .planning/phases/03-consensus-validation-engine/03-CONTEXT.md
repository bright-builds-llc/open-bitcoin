---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 3-2026-04-11T21-32-30
generated_at: 2026-04-11T21:33:00.000Z
---

# Phase 3: Consensus Validation Engine - Context

**Gathered:** 2026-04-11  
**Status:** Ready for planning and execution  
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 3 owns the pure-core consensus and validation surface that sits on top of
the Phase 2 primitives and codecs. It includes script execution, transaction
checks, block checks, consensus-oriented hashing, and deterministic parity
fixtures derived from the pinned Knots baseline. It does not include chainstate,
mempool policy, peer/network orchestration, wallet behavior, or adapter-owned
I/O.

</domain>

<decisions>
## Implementation Decisions

### Pure-Core Placement
- **D-01:** Phase 3 stays in a new pure-core crate under `packages/` so
  consensus logic can remain independent from runtime adapters and later
  chainstate storage.
- **D-02:** Consensus hashing and validation stay explicit in repo-owned Rust
  code rather than pulling in third-party Bitcoin libraries.

### Validation Surface
- **D-03:** Model validation outcomes as typed Rust errors with stable reject
  reasons instead of ad-hoc strings or panics.
- **D-04:** Reuse Phase 2 domain invariants aggressively. For example, output
  values are already `Amount`, so consensus checks only need to guard totals and
  relationships that can still fail at runtime.

### Script Scope
- **D-05:** Start with deterministic stack, equality, numeric, and hash opcode
  behavior that can be exercised against upstream vectors without introducing
  signature cryptography yet.
- **D-06:** Unsupported consensus surfaces such as `CHECKSIG`, P2SH, witness
  program execution, and contextual locktime or maturity checks must stay
  explicit in verification gaps instead of being silently treated as complete.

### Block Rules
- **D-07:** Implement context-free block checks first: proof-of-work, merkle
  root, duplicate-transaction malleation detection, coinbase placement, block
  size, and legacy sigop counting.
- **D-08:** Keep block-level input validation callable through explicit spent
  output views rather than smuggling chainstate into the pure core.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md` § Phase 3
- `scripts/verify.sh`
- `scripts/check-pure-core-deps.sh`
- `scripts/pure-core-crates.txt`
- `packages/bitcoin-knots/src/script/script.h`
- `packages/bitcoin-knots/src/script/interpreter.cpp`
- `packages/bitcoin-knots/src/consensus/tx_check.cpp`
- `packages/bitcoin-knots/src/consensus/validation.h`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/src/pow.cpp`
- `packages/bitcoin-knots/src/test/data/script_tests.json`
- `packages/open-bitcoin-primitives/src/transaction.rs`
- `packages/open-bitcoin-codec/src/transaction.rs`
- `packages/open-bitcoin-codec/src/block.rs`

</canonical_refs>

<code_context>
## Existing Code Insights

- Phase 2 already provides byte-faithful `ScriptBuf`, `Transaction`, `Block`,
  and codec functions, so Phase 3 can focus on validation rules rather than raw
  parsing.
- The repo already enforces pure-core dependency hygiene and 100% line coverage
  for allowlisted pure-core crates, so the new crate must ship with exhaustive
  unit coverage.
- `Transaction::txid()` in the primitives crate was only a placeholder shape,
  not a real consensus hash, so consensus hashing needs its own crate-level API.

</code_context>

<deferred>
## Deferred Ideas

- Signature verification and script paths that depend on secp256k1, P2SH, or
  witness program execution
- Contextual transaction and block rules that require height, time, or coinbase
  maturity
- Cross-process Knots-vs-Open-Bitcoin execution harnesses beyond deterministic
  fixture parity

</deferred>

---

*Phase: 03-consensus-validation-engine*  
*Context gathered: 2026-04-11*

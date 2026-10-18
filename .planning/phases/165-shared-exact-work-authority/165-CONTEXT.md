---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T16:15:00Z
---

# Phase 165: Shared Exact Work Authority - Context

**Gathered:** 2026-10-14
**Status:** Ready for planning
**Mode:** Yolo; recommended answers auto-selected in one pass under the requested strict-push workflow.

<domain>
## Phase Boundary

One internally derived exact work authority for live header selection, connected candidates and reorg decisions, plus explicit pre-mutation refusal of unprovable legacy work. Requirements: CWOR-01, CWOR-02, CWOR-03, CWOR-05. Preserve later canonical-bootstrap/context prerequisites.
</domain>

<decisions>
## Implementation Decisions

### Exact arithmetic and target admission

- **D-01:** Extend the existing first-party consensus `ProofWork`; no additional dependency or parallel arithmetic implementation. Numerical work alone is not validation provenance.
- **D-02:** Derive block proof as floor(2^256 / (target + 1)), accumulate modulo 2^256, compare all unsigned 256 bits, and use lossless byte/hex boundaries. No extra cumulative-overflow refusal; any future refusal requires an intentional parity difference.
- **D-03:** Match pinned compact normalization, sign, zero and overflow boundaries, including tiny mantissas truncated to zero and equivalent encodings. Header admission additionally requires selected-network powLimit and hash satisfaction. Share decoding validity between proof and admission without claiming contextual difficulty completeness.
- **D-04:** Independently expected vectors cover values beyond 128 bits, mixed targets, carry/wrap and negative/zero/overflow edges. Keep arithmetic/source-derived evidence separate from actual-daemon differential qualification.

### Header and connected branch ordering

- **D-05:** Accepted headers derive cumulative work from accepted parent plus own target proof. Update the best header only on strictly greater work; equal work preserves the first accepted best header, and a shorter higher-work branch wins.
- **D-06:** Connected candidate ordering derives the same work internally, followed by pinned body/ancestor eligibility arrival order. Header arrival is not body eligibility order. Consider eligible retained side branches as well as the best-header branch; an equal-work first-header branch must not suppress the earlier eligible connected candidate.
- **D-07:** Enforce work/eligibility preference at actual connect and reorg owners before mutation. Inflated caller numbers, forged position metadata, direct managed reorg entrypoints and staged previews cannot authorize a stronger active branch. Preserve explicit fixture boundaries where historical tests need synthetic roots.
- **D-08:** Define known equal-work activation cases from the pin; document truly ambiguous internal/restart ties as bounded evidence. Do not describe a deterministic hash/pointer fallback as proven Knots activation parity.

### Safe legacy compatibility and scope

- **D-09:** Extend Phase 164's filesystem-only ordinary gate with an explicit work-compatibility decision. Refuse unprovable legacy synthetic-work state before Fjall recovery, coins/schema migration, initialization, prune resume, BASIC writes or live authority changes. Report observed work as unavailable when bytes have not been proved; never infer a profile from a forged marker or file shape.
- **D-10:** Keep explicit legacy codec representation/classification only where the shared-work transition requires it for historical snapshot fixtures. Decoded u128 scores are legacy numbers, never silently zero-extended into verified authority. No automatic repair, destructive conversion or reinterpretation.
- **D-11:** Prove refusal through closed source-manifest preservation, backend-spy and migration/prune controls, malformed/forged inputs and ordinary operator/daemon routes. Preserve all four selector support/refusal classifications and default-off networking.
- **D-12:** Phase 166 owns canonical durable bootstrap, exact-work checkpoint/reopen acceptance and genesis BASIC; Phase 167 owns contextual consensus prerequisites; Phase 168 owns actual pinned-daemon differential qualification. Representation changes needed to compile Phase 165 are permitted, but do not claim those later outcomes.

### Agent Discretion

Choose small modules, restricted authority constructors, bounded diagnostics and focused behavioral tests. Reuse existing networking, chainstate and storage seams, preserve one active Fjall chainstate and pure-core/shell boundaries. Research must distinguish compatibility number carriers from earned runtime authority and identify all call sites that can select an active branch.
</decisions>

<canonical-refs>
## Canonical References

Downstream agents must read these before planning or implementation.

- `.planning/ROADMAP.md` — Phase 165 criteria and later ownership.
- `.planning/REQUIREMENTS.md` — CWOR-01/02/03/05 and milestone evidence contract.
- `.planning/PROJECT.md` — approved scope and retained limits.
- `.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-CONTEXT.md` and `164-VERIFICATION.md` in that directory — existing read-only compatibility and support gates.
- `.planning/research/ARCHITECTURE.md` and `.planning/research/PITFALLS.md` — shared authority and mutation pitfalls.
- `packages/bitcoin-knots/src/arith_uint256.h` and `packages/bitcoin-knots/src/arith_uint256.cpp` — exact wrapping arithmetic and compact normalization at pin a9aee730466ac67d35a3c03ee24676be5e045878.
- `packages/bitcoin-knots/src/chain.cpp` — GetBlockProof.
- `packages/bitcoin-knots/src/pow.cpp` — target admission and powLimit.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — cumulative work, strict best-header updates and candidate ordering.
- `packages/bitcoin-knots/src/validation.cpp` — body/ancestor eligibility sequence and branch activation.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` — local verification, style and Bun scripting contracts.
  </canonical-refs>

<code-context>
## Existing Code Insights

- `packages/open-bitcoin-consensus/src/chain_proof.rs` already provides full-width proof arithmetic, ordering and modulo accumulation; existing tests exercise above-u128 proof and wrapping.
- `packages/open-bitcoin-consensus/src/crypto.rs` owns compact target decoding and proof-of-work checks; tiny mantissas can currently decode to zero without an admission-specific zero check.
- `packages/open-bitcoin-chainstate/src/types.rs` and engine/stage/replay paths carry caller-supplied u128 work; the exact numerical carrier must not substitute for earned provenance.
- `packages/open-bitcoin-node/src/network/inventory.rs` derives count-like next work; sync block reconciliation currently follows only best-header entries. Both need actual shared-authority wiring.
- `packages/open-bitcoin-node/src/storage/chain_compatibility.rs` intentionally inspects filesystem shape without opening Fjall. Raw Fjall opening performs recovery/migration and is too late for ordinary compatibility inspection.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` has legacy u128 DTOs; decode success alone cannot prove datadir identity, headers or work.
  </code-context>

<specifics>
## Specific Ideas

Exercise shorter higher-work and distinct equal-header/equal-body arrival cases. Test direct reorg callers supplying maximal scores and unchanged state on refusal. Use exact repo-local Cargo/Bazel commands in operator guidance. All nested commits are deferred to the wrapper's clean phase-verification gate; keep normal hooks.
</specifics>

<deferred>
## Deferred Ideas

Canonical bootstrap, exact durable recovery authority, contextual history validation and actual-daemon differential acceptance retain Phases 166–168 ownership. No new networks, automatic legacy repair, public-default activation or production/funds claims. No pending todos matched this phase.
</deferred>

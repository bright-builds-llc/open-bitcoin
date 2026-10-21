---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: 2026-10-18T22:36:40.350Z
---

# Phase 166: Canonical Durable Bootstrap and Genesis BASIC - Context

**Gathered:** 2026-10-18
**Status:** Ready for planning
**Mode:** Yolo; recommended choices selected in one pass under the strict-push wrapper.

<domain>
## Phase Boundary

Initialize and reopen an empty canonical datadir with proved genesis, coins best block, exact cumulative work and explicitly enabled BASIC authority. Close CBOT-02, CBOT-03 and CWOR-04. Ordinary history remains gated on Phase 167; actual pinned-daemon differential qualification belongs to Phase 168.
</domain>

<decisions>
## Implementation Decisions

### Canonical genesis and authority

- **D-01:** Reuse the exact pinned genesis header and complete body for all four existing network selectors. A selected network, file shape or marker alone never proves stored identity. Reject mismatched network/root/body before accepting authority or mutating incompatible original storage.
- **D-02:** A fresh successful initialization has height zero, canonical genesis as best header/block and coins best block, internally derived exact genesis proof and no spendable genesis coinbase outputs. Preserve the serialized genesis transaction and its output script for BASIC input; do not treat maturity or script unspendability as the genesis UTXO exclusion mechanism.
- **D-03:** Separate genesis bootstrap/reopen capability from ordinary-history qualification. Keep all missing contextual prerequisites explicit, including Signet challenge validation; genesis support does not authorize ordinary history, candidate connects, reorgs or public-network activation.

### Durable initialization and reopen

- **D-04:** Interrupted initialization either resumes from independently proved complete authority or explicitly refuses. Do not publish partial genesis, coins, header, work or index progress as accepted. Prefer the smallest robust atomic/publication protocol supported by current storage; no automatic repair, import, destructive conversion or legacy reinterpretation.
- **D-05:** Preserve the Phase 164/165 pre-effect compatibility boundary for existing unproved, foreign-network, noncanonical or legacy synthetic-work state. Design a proved compatible reopen route rather than bypassing refusal via raw Fjall open. Refusal diagnostics remain bounded and redacted, with unchanged incompatible original data.
- **D-06:** Checkpoint and actual same-datadir reopen after releasing every storage handle must preserve every bit of exact 256-bit work. Recovered authority binds selected network, complete stored header ancestry, internally recomputed proof/accumulation, active tip and coins best block. Decoded numerical work, metadata, fixture snapshots or cloned live handles cannot confer authority. Test independently expected above-u128 encoding/validation separately from small-work genesis histories.
- **D-07:** Inconsistent header/body/network/work/coins bindings, missing records and interrupted checkpoint states recover only to a proved durable prefix or explicitly refuse; never synthesize accepted progress. Preserve pinned modulo-2^256 arithmetic and existing finite-resource policies.

### BASIC and ordinary operator integration

- **D-08:** Explicit BASIC enablement on fresh canonical storage produces the exact pinned genesis BASIC filter and commitment with zero predecessor. Reuse validated lineage, accepted-state owner and durable publication/fencing; no seeded history or fixture-authority route in the product path.
- **D-09:** Genesis indexing and reopen integrate into existing durable startup and configured operator/daemon routes. Ordinary index continuation uses the same owner once validated history is available; this phase does not bypass Phase 167 to manufacture it. Keep disabled BASIC free of index initialization and keep public networking default-off.
- **D-10:** Test actual durable bytes, all-handle release/reopen, boundary corruption and failure injection. Preserve prior receipts and distinguish source-derived genesis/filter expectations, software fault controls, arithmetic vectors and future actual-daemon differential evidence.

### Agent Discretion

Choose focused typed modules and restricted constructors within existing crates, using the existing Fjall, coins, work and BASIC infrastructure. Add no production dependency, workspace crate or external service. Use pure data transformations for authority checks and thin shell adapters for I/O. Research must resolve pre-effect reopen proof and publication ordering before plans are accepted.
</decisions>

<canonical-refs>
## Canonical References

- `.planning/ROADMAP.md` — Phase 166 four success criteria and later ownership.
- `.planning/REQUIREMENTS.md` — CBOT-02, CBOT-03, CWOR-04 and milestone evidence boundaries.
- `.planning/PROJECT.md` — approved scope and retained limits.
- `.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-CONTEXT.md` and `.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-VERIFICATION.md` — read-only compatibility and support gates.
- `.planning/phases/165-shared-exact-work-authority/165-CONTEXT.md` and `.planning/phases/165-shared-exact-work-authority/165-VERIFICATION.md` — exact-work authority, legacy refusal and deferred reopen.
- `.planning/research/ARCHITECTURE.md` and `.planning/research/PITFALLS.md` — bootstrap, recovery and evidence pitfalls.
- `packages/bitcoin-knots/src/kernel/chainparams.cpp` — four genesis identities at pin `a9aee730466ac67d35a3c03ee24676be5e045878`.
- `packages/bitcoin-knots/src/validation.cpp` — genesis special handling, coins best block and activation ordering.
- `packages/bitcoin-knots/src/chain.cpp` and `packages/bitcoin-knots/src/arith_uint256.h` — proof and cumulative arithmetic.
- `packages/bitcoin-knots/src/blockfilter.cpp` and `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — genesis BASIC content and predecessor commitment.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md` and `standards/languages/typescript-javascript.md` — repo workflow, parity, style and verification contracts.
</canonical-refs>

<code-context>
## Existing Code Insights

- `packages/open-bitcoin-consensus/src/chain_spec.rs` owns pure canonical genesis and separate unsupported-history facts.
- `packages/open-bitcoin-node/src/storage/chain_compatibility.rs` currently refuses every ordinary open before backend effects. Raw `FjallNodeStore::open` recovers and migrates, so it cannot substitute for safe compatibility inspection.
- `packages/open-bitcoin-node/src/network/canonical_admission.rs` checks root identity then requires ordinary history. Bootstrap needs its own restricted capability without broadening this gate.
- Existing numerical snapshot/header decoders and `HeaderStore::from_entries` retain unproved work; existing live-owner rebind is not independent reopen evidence.
- `packages/open-bitcoin-node/src/sync/open_runtime.rs`, coins storage and BASIC lifecycle already provide startup, checkpoint, recovery and validated-lineage seams to extend.
</code-context>

<specifics>
## Specific Ideas

Prove genesis coinbase output absence through actual coin lookup and reopen. Compare exact serialized genesis/filter/commitment bytes against pinned independent expectations. Reject mutations to every authority binding without granting accepted progress. Use copy-pasteable repo-local Cargo/Bazel operator commands. All nested commits are deferred to the wrapper's clean phase-verification and lifecycle gate; normal hooks remain mandatory.
</specifics>

<deferred>
## Deferred Ideas

Phase 167 contextual ordinary-history rules; Phase 168 actual pinned Knots differential runner; Phase 169 fork/recovery qualification; Phase 170 canonical pruning integration. No new networks, implicit download/import/repair, public-default activation or production/service/funds claim. No pending todos matched this phase.
</deferred>

---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-25T18:13:00.349Z
---

# Phase 132: Typed Package Vocabulary and Staged Admission - Context

**Gathered:** 2026-07-25
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Deliver the pure mempool package vocabulary and staged admission engine for bounded local dry-run and child-with-unconfirmed-parents submission. The phase owns exact context-free shape validation, input-ordered results, individual-first partial acceptance, effective-fee grouping, scoped replacement/TRUC/ephemeral-dust/witness policy, coherent staged commits, pressure trim, and truthful final membership. Phase 133 owns peer-originated same-peer 1P1C assembly, Phase 134 owns cross-cache lifecycle projection, and Phase 137 owns RPC and sanitized operator evidence adapters.

</domain>

<decisions>
## Implementation Decisions

### Package shape, identity, and ordered results

- **D-01:** Parse raw transaction vectors at the package boundary into an opaque well-formed package type. Its fallible constructor proves non-empty input, at most 25 transactions, at most 404,000 total weight units, unique txid and wtxid identities, topological order, and no internal input conflicts before expensive validation.
- **D-02:** Refine a well-formed package into a distinct child-with-unconfirmed-parents submission type. Submission-only shape rules must not be caller booleans or repeated checks deep inside admission.
- **D-03:** Compute canonical txid/wtxid member pairs once and preserve request order in private ordered storage. Keyed maps may exist only as lookup projections; they must never define response order.
- **D-04:** Return one input-index-aligned package report with a package-wide status, exactly one typed member result per request member, and explicit non-empty effective-fee groups whose membership is an ordered wtxid list. Avoid optional-field combinations that can represent impossible result states.
- **D-05:** Keep a package fingerprint or package hash separate from request order and per-member admission identity so Phase 133 can reuse package identity without changing the local result contract.

### Dry-run, partial acceptance, and staged commit

- **D-06:** Use distinct dry-run and submission command types over shared package primitives. Dry-run evaluates the complete pipeline and returns the same ordered vocabulary while leaving mempool entries, rolling fee, relay, persistence, and evidence state byte-for-byte unchanged.
- **D-07:** Preserve pinned individual-first behavior. Evaluate members in input order; retain successful singleton admissions in the prospective view; retry only eligible reconsiderable or missing-input members as the remaining subpackage; and allow a valid parent to remain finally accepted when its child fails.
- **D-08:** Implement a typed prospective overlay rather than repeatedly calling the live single-transaction mutator or cloning the entire mempool. Each accepted singleton or package group produces a checked coherent sub-delta; compose those facts into one package transition and perform one guarded live apply after final trimming.
- **D-09:** Bind a prepared transition to the exact base state it evaluated. Applying it to a changed mempool must fail before mutation, and any validation, replacement, limit, script, trim, or delta-composition failure must discard the overlay and rolling-fee changes.
- **D-10:** Keep attempt vocabulary in ordered package/member results and committed facts in `MempoolLifecycleDelta`. Witness aliases and failed candidates never appear as admitted or removed lifecycle members.

### Effective fee and final policy boundaries

- **D-11:** Preserve Phase 130 fee-role separation. Every ordinary member must satisfy the static relay floor independently; an eligible non-empty package aggregate may satisfy the active rolling floor. Incremental relay fee remains only a replacement/pressure input.
- **D-12:** Implement the unchanged PACK-06/PACK-07 surface rather than silently narrowing it. Phase 132 must cover the pinned limited package-RBF, TRUC inheritance/topology and explicit enforced-TRUC fee exception, ephemeral-dust spend checks, same-txid/different-witness handling, and reconsiderable-failure classification needed by the selected local package modes.
- **D-13:** Follow one explicit policy order: context-free shape checks; exact-mempool/witness-alias/new-candidate classification; individual evaluation; residual reconsiderable grouping; ordinary static-floor checks; TRUC checks; aggregate rolling-floor assessment; ancestor/descendant limits and limited replacement; ephemeral-dust checks; scripts; coherent staged commit; one Phase 131 pressure trim; then final-membership result rewriting.
- **D-14:** Model `SameTxidDifferentWitness` with the existing wtxid explicitly, keep aliases out of effective-fee groups and lifecycle deltas, and distinguish reconsiderable failures from hard rejects without adding peer-origin state in this phase.
- **D-15:** Rewrite every initially successful member result from authoritative post-trim membership. A member removed by replacement or final pressure cannot remain reported as accepted merely because earlier preparation succeeded.
- **D-16:** Package admission itself does not enqueue relay, write persistence, mutate serving/compact/orphan/retry caches, or publish operator evidence. It emits the typed results and semantic lifecycle facts later phases consume.

### the agent's Discretion

- Exact Rust names and module split, provided the opaque refinements, ordered-report invariant, mode separation, and prospective-apply guard remain explicit.
- The internal overlay representation and base-state token/version, provided it avoids whole-mempool cloning on the normal path and has a recomputation oracle in tests.
- Exact enum granularity for package-wide and per-member failure reasons, provided hard, reconsiderable, witness-alias, already-present, finally-present, and post-trim-absent states cannot be confused.
- Whether dry-run and submission share private helper functions or sealed internal stages, provided their public command types make invalid capability combinations unrepresentable.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract and research

- `.planning/ROADMAP.md` — Phase 132 goal, success criteria, and boundaries against Phases 133, 134, and 137.
- `.planning/REQUIREMENTS.md` — PACK-01 through PACK-07 and the v2.2 exclusion boundary.
- `.planning/research/ARCHITECTURE.md` — Staged package transition, authority, fee-role, and build-order recommendations.
- `.planning/research/FEATURES.md` — Local package surfaces, exact bounds, partial-acceptance behavior, and pinned policy feature inventory.
- `.planning/research/PITFALLS.md` — False atomicity, fee-floor collapse, witness identity, partial mutation, and final-membership hazards.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 package admission scope and release-claim boundary.

### Locked predecessor decisions

- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` — Static/incremental/rolling/effective fee roles, package-floor separation, attempt-versus-lifecycle vocabulary, and final-membership delta contract.
- `.planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md` — Accounted-capacity trim, rolling-floor behavior, pressure delta, and post-admission trim contract.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md` — Mempool mutation, replacement, and lifecycle authority boundaries.
- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md` — Single managed runtime authority inherited by later adapters.
- `.planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-CONTEXT.md` — Deterministic verification and narrow claim guardrails.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/doc/policy/packages.md` — Package terminology, limits, individual-first behavior, fee aggregation, and limited replacement rationale.
- `packages/bitcoin-knots/src/policy/packages.h` — Package aliases, limits, hash, and validation declarations.
- `packages/bitcoin-knots/src/policy/packages.cpp` — Empty/count/weight/duplicate/topology/input-conflict and child-with-parents checks.
- `packages/bitcoin-knots/src/validation.h` — Package-wide and per-transaction admission result vocabulary.
- `packages/bitcoin-knots/src/validation.cpp` — `AcceptMultipleTransactions`, `AcceptSubPackage`, `AcceptPackage`, and `ProcessNewPackage` ordering, staging, fee grouping, replacement, trimming, and result rewriting.
- `packages/bitcoin-knots/src/txmempool.h` — Changeset and staged mempool mutation model.
- `packages/bitcoin-knots/src/policy/truc_policy.h` and `packages/bitcoin-knots/src/policy/truc_policy.cpp` — TRUC inheritance, topology, replacement, and package policy.
- `packages/bitcoin-knots/src/policy/ephemeral_policy.h` — Ephemeral-dust package checks.
- `packages/bitcoin-knots/src/test/txpackage_tests.cpp` — Shape, partial-acceptance, package-fee, replacement, and valid-parent/invalid-child unit anchors.
- `packages/bitcoin-knots/test/functional/mempool_truc.py` — TRUC and ephemeral-dust functional boundaries.

### Open Bitcoin seams

- `packages/open-bitcoin-mempool/src/fee.rs` — Existing static-member and aggregate-rolling package-floor assessment.
- `packages/open-bitcoin-mempool/src/outcome.rs` — Existing single-transaction attempt vocabulary.
- `packages/open-bitcoin-mempool/src/types.rs` — Existing policy configuration, entry identity, graph aggregates, and result types.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` — Current prospective single-transaction admission, replacement, trim, delta build, and live assignment.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — Checked txid/wtxid identity pairs, admitted ordering, removals, and final membership.
- `packages/open-bitcoin-mempool/src/pool/topology.rs` — Graph recomputation and package ancestry/descendant invariants.
- `packages/open-bitcoin-mempool/tests/parity.rs` — Existing pinned mempool parity fixtures.
- `docs/parity/catalog/mempool-policy.md` — Current package-policy claims and deferred gaps.
- `docs/parity/source-breadcrumbs.json` — Required source breadcrumb registry for new Rust files.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `Mempool::commit_transaction_with_context` already constructs prospective entries and rolling state, validates limits, trims, builds a lifecycle delta, and assigns live state only at the end. Split its preparation and apply seams rather than building a parallel engine.
- `PackageFeeFloorAssessment` already encodes independent static-member and aggregate-rolling decisions.
- `MempoolLifecycleDeltaBuilder` already rejects conflicting txid/wtxid identity pairs, preserves admitted order, and records final membership.
- `recompute_state`, topology helpers, accounted-resource ledgers, and Phase 131 pressure trim provide deterministic oracles for overlay verification.
- Existing transaction txid/wtxid helpers provide canonical identities without adding a Bitcoin library dependency.

### Established Patterns

- Keep validation and state transitions in the pure `open-bitcoin-mempool` core; clocks, persistence, RPC, network, and evidence stay in adapters.
- Parse boundary data into strong types and make invalid capability combinations unrepresentable.
- Attempt results and committed lifecycle facts are separate contracts.
- Verification is deterministic, hermetic, and parity-anchored; public-network behavior is not a default gate.

### Integration Points

- Add package vocabulary near `open-bitcoin-mempool` crate types, using `foo.rs` plus `foo/` if the admission implementation needs child modules.
- Refactor single-transaction preparation from `pool/admission.rs` so package staging and legacy single admission share validation without repeated live mutation.
- Extend `MempoolError` and outcome projections only with stable typed categories needed by the package core.
- Phase 133 will consume the refined shared package type for peer 1P1C candidates.
- Phase 134 will project the final package lifecycle delta across managed caches.
- Phase 137 will adapt typed dry-run/submission reports to RPC and sanitized operator surfaces.

</code-context>

<specifics>
## Specific Ideas

- Prefer opaque refinements and input-index-aligned vectors over a raw `Vec<Transaction>` plus a result map.
- Treat “atomic package” as coherent mutation of each accepted subpackage and one guarded package transition, not global all-or-nothing acceptance.
- Use the current single-admission prospective path as the extraction seam, but do not preserve its per-member full-mempool clone/recompute cost in the package normal path.
- Keep a distinct package fingerprint available for Phase 133 without letting it define result ordering.

</specifics>

<deferred>
## Deferred Ideas

- Peer-originated reconsiderable caching and bounded same-peer 1P1C assembly — Phase 133.
- Applying package lifecycle facts to serving, relay, compact reconstruction, orphan/reject, retry, persistence, and evidence caches — Phase 134.
- Snapshot schema, checkpointing, and recovery — Phase 135.
- Receive-independent maintenance, fanout, and transport receipts — Phase 136.
- RPC/CLI/dashboard/status methods and sanitized operator evidence — Phase 137.
- Cross-phase adversarial pressure/restart/release proof — Phase 138.

</deferred>

***

*Phase: 132-typed-package-vocabulary-and-staged-admission*
*Context gathered: 2026-07-25*

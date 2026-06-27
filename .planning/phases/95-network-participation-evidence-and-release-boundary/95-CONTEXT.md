---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 95-2026-06-27T12-48-17
generated_at: 2026-06-27T12:49:20.758Z
---

# Phase 95: Network Participation Evidence and Release Boundary - Context

**Gathered:** 2026-06-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 95 closes v1.9 by proving parity roots, release-boundary no-claims, non-regression, repo-local UAT guidance, support-bundle redaction, and 28/28 requirement traceability for inbound peer serving and network participation. This phase is evidence, documentation, and deterministic checker work. It must not expand runtime network participation, transaction relay, compact block relay, mempool propagation, public inbound defaults, production-service claims, or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Deterministic Release-Boundary Checker
- **D-01:** Add one focused Phase 95 aggregate Bun checker and fixture test, wired immediately after Phase 94 in `scripts/verify.sh` and in the legacy `VERIFY_COMMAND_ORDER` block. Do not scatter Phase 95 closeout logic across Phase 90 through Phase 94 checkers unless a genuinely phase-local gap is discovered.
- **D-02:** The Phase 95 checker must prove BOUND-01, BOUND-03, BOUND-04, BOUND-05, and BOUND-06 through static, deterministic, public-network-free assertions over release/parity/operator docs, support evidence roots, `scripts/verify.sh` ordering, and requirement traceability.
- **D-03:** The checker must reject positive v1.9 claims for transaction relay, compact block relay, mempool propagation, broad/full address relay beyond the scoped Phase 92 boundary, public inbound defaults, public-network CI, production-service operation, and production full-node readiness. Valid no-claim, deferred, unsupported, future, opt-in UAT, and evidence-boundary wording must remain allowed.

### Parity Roots And Traceability
- **D-04:** Add a compact v1.9 release-boundary closeout surface inside the existing parity roots rather than creating a standalone competing manifest. `docs/parity/index.json` remains the machine-readable root; `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/release-readiness.md` carry the human review evidence.
- **D-05:** `docs/parity/catalog/p2p.md` should include a Phase 95/v1.9 closeout rollup that cites the required Knots anchors: `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/addrman.cpp`, `packages/bitcoin-knots/src/banman.cpp`, and `packages/bitcoin-knots/src/net_permissions.cpp`, plus any already-used protocol or functional-test anchors where they clarify evidence.
- **D-06:** Requirement traceability must keep all 28 v1.9 requirements mapped exactly once across Phase 90 through Phase 95, with BOUND-01 through BOUND-06 assigned to Phase 95. Planning, summaries, verification, and any milestone audit artifacts should reference the parity roots instead of becoming separate release evidence sources.

### UAT And Non-Regression Evidence
- **D-07:** Phase 95 should use a deterministic closeout mix: focused Phase 95 checker/test commands for local iteration, full `bash scripts/verify.sh` as the repo-native non-regression proof, and operator UAT docs with copy-pasteable repo-local Cargo and Bazel commands for loopback or synthetic inbound review.
- **D-08:** UAT guidance must use explicit repo-local command forms, including `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- ...`, `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`, `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- ...`, `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- ...`, `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, and `bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- ...` where applicable.
- **D-09:** Public-network full-sync, soak, real service-manager, and live support-bundle collection remain optional operator evidence only. They must not become default verification, release-blocking CI, or wording that implies production readiness.

### Support Bundle Redaction
- **D-10:** Close BOUND-05 primarily with aggregate Phase 95 checker assertions over existing Phase 90 through Phase 94 support evidence roots. Preserve behavior unless the aggregate check exposes a real leak.
- **D-11:** The checker should assert that support evidence preserves useful inbound diagnosis across admission, permission, address, eviction/ban, misbehavior, and resource-governance surfaces while redacting or bounding raw peer addresses, endpoints, peer IDs, permission strings, config names, payload bytes, credentials, and unbounded tables.
- **D-12:** Add a narrow resource-governance support evidence assertion if needed so Phase 94 resource counters and latest decision evidence are preserved without leaking raw payload or peer material.

### the agent's Discretion
- Exact checker helper structure, fixture layout, and failure-message wording.
- Whether Phase 95 requires a new milestone audit file during execution or records closeout only through phase verification and existing parity roots.
- How to minimize brittle prose scanning while still catching forbidden release claims.
- Whether to reuse Phase 88 no-claim helper patterns directly or extract small shared local helpers inside the Phase 95 checker.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope
- `.planning/ROADMAP.md` — Phase 95 goal, success criteria, and active milestone traceability.
- `.planning/REQUIREMENTS.md` — BOUND-01 through BOUND-06 and 28/28 v1.9 requirement mapping.
- `.planning/STATE.md` — Current v1.9 completion context and prior Phase 90 through Phase 94 decisions.
- `AGENTS.md` — Repo-local GSD, verification, Rust, TypeScript, and UAT command rules.
- `AGENTS.bright-builds.md` — Bright Builds workflow, verification, testing, architecture, and code-shape rules.

### Prior v1.9 Context
- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` — Opt-in listener, admission, redaction, UAT, and no-claim boundaries.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` — Permission labels, inactive relay effects, support redaction, and no-claim boundaries.
- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` — Local advertisement, bounded `getaddr`, learned-address evidence, and full address-relay deferral.
- `.planning/phases/93-eviction-ban-and-misbehavior-policy/93-CONTEXT.md` — Eviction/ban/misbehavior policy evidence and support redaction.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` — Resource-governance labels, deterministic verification, support evidence, and no-claim boundaries.

### Release And Parity Roots
- `docs/parity/catalog/p2p.md` — Existing Phase 90 through Phase 94 v1.9 parity surfaces, Knots anchors, and deferred network participation wording.
- `docs/parity/index.json` — Machine-readable parity surface root and release evidence index.
- `docs/parity/checklist.md` — Human parity checklist and requirement surface table.
- `docs/parity/release-readiness.md` — Release-readiness evidence and no-claim review root.
- `docs/parity/production-claim-boundary.md` — Production-readiness and deferred-surface claim guardrails.
- `docs/parity/support-matrix.md` — Supported, preview, opt-in UAT, unsupported, and deferred support classifications.
- `docs/parity/source-breadcrumbs.json` — Parity breadcrumb mapping for first-party source and tests.

### Operator And Evidence Docs
- `docs/operator/runtime-guide.md` — Operator UAT command patterns and inbound review guidance.
- `docs/architecture/status-snapshot.md` — Shared status evidence contract.
- `docs/architecture/operator-observability.md` — Status, metrics, logs, and support evidence projection.

### Checker And Verification Patterns
- `scripts/verify.sh` — Repo-native verification contract and checker ordering.
- `scripts/check-phase88-deterministic-claim-guardrails.ts` — Existing release no-claim checker pattern.
- `scripts/check-phase88-deterministic-claim-guardrails.test.ts` — Fixture pattern for release claim guardrails.
- `scripts/check-phase90-inbound-listener-admission.ts` — Phase 90 inbound boundary checker pattern.
- `scripts/check-phase91-peer-permissions.ts` — Phase 91 peer-permission checker pattern.
- `scripts/check-phase92-address-boundaries.ts` — Phase 92 address-boundary checker pattern.
- `scripts/check-phase93-peer-policy.ts` — Phase 93 peer-policy checker pattern.
- `scripts/check-phase94-dos-resource-governance.ts` — Phase 94 resource-governance checker pattern.
- `scripts/check-phase94-dos-resource-governance.test.ts` — Recent fixture and verifier-order test pattern.

### Support Evidence
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` — Existing support-bundle redaction and inbound evidence tests through Phase 94.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` — Inbound support renderer.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` — Inbound status renderer.
- `packages/open-bitcoin-node/src/status/inbound.rs` — Shared inbound status contract.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `scripts/check-phase94-dos-resource-governance.ts`: recent Bun checker structure for target-file corpus loading, parity index validation, verifier ordering, no-claim scanning, labels, metrics, docs, and support-evidence boundaries.
- `scripts/check-phase94-dos-resource-governance.test.ts`: recent fixture builder pattern with one pass fixture, mutation-based negative tests, and Arrange/Act/Assert test structure.
- `scripts/verify.sh`: existing release-boundary checker ordering and legacy `VERIFY_COMMAND_ORDER` text block that downstream checkers inspect.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs`: existing Rust coverage for redacted inbound endpoint, permission, address-boundary, peer-policy, and resource-governance support evidence.

### Established Patterns
- Phase checkers are Bun/TypeScript scripts with paired `.test.ts` files and no extra package install step.
- Release-boundary and no-claim checks use curated file sets, not repository-wide scans or `.planning` history scans.
- Default verification stays deterministic, public-network-free, service-manager-free, and short-running.
- Operator UAT docs must include repo-local Cargo and Bazel command forms rather than relying on an installed `open-bitcoin` alias.
- Support evidence exposes low-cardinality labels, bounded counts, and latest decisions while redacting raw endpoints, peer IDs, config names, raw permission strings, payloads, credentials, and unbounded tables.

### Integration Points
- Add the Phase 95 checker/test immediately after Phase 94 in both the executable `run_step` path and the `VERIFY_COMMAND_ORDER` heredoc in `scripts/verify.sh`.
- Add the v1.9 closeout surface to `docs/parity/index.json` and `docs/parity/checklist.md`.
- Update `docs/parity/catalog/p2p.md`, `docs/parity/release-readiness.md`, and `docs/operator/runtime-guide.md` with Phase 95 closeout, UAT, no-claim, and support-redaction evidence.
- If first-party Rust or test files are added under parity-breadcrumb enforced paths, update `docs/parity/source-breadcrumbs.json`; Phase 95 is expected to be mostly docs/scripts and may not need new Rust breadcrumbs unless a focused support assertion is added in Rust.

</code_context>

<specifics>
## Specific Ideas

- Treat Phase 95 as a release-boundary and evidence closure phase, not a runtime feature expansion phase.
- Prefer a single aggregate checker/test over broad rewrites to completed Phase 90 through Phase 94 checkers.
- Make `bash scripts/verify.sh` the clean non-regression proof before final push.
- Keep public-network and production-language boundaries explicit and deterministic.

</specifics>

<deferred>
## Deferred Ideas

- Transaction relay and mempool propagation remain future relay scope.
- Compact block relay remains future relay scope.
- Full address relay and broader peer discovery remain future address-relay/network-participation scope.
- Public inbound serving by default, public-network CI, production-service operation, and production full-node readiness remain future release/support scope.
- Signed packaging, hosted dashboards, GUI, migration apply mode, destructive repair, production-funds wallet operation, automatic support-bundle upload, and Windows service integration remain outside v1.9.

</deferred>

---

*Phase: 95-network-participation-evidence-and-release-boundary*
*Context gathered: 2026-06-27*

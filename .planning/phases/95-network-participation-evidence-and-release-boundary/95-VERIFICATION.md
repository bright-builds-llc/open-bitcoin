---
phase: 95-network-participation-evidence-and-release-boundary
verified: 2026-06-27T16:49:42Z
status: passed
score: 14/14 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 95-2026-06-27T12-48-17
generated_at: 2026-06-27T16:49:42Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 95: Network Participation Evidence and Release Boundary Verification Report

**Phase Goal:** Close v1.9 by proving parity roots, non-regression, UAT guidance, support redaction, and deterministic release-boundary checks that keep deferred network participation claims out of scope.
**Verified:** 2026-06-27T16:49:42Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Release and parity docs cite Knots anchors or record intentional deviations for inbound serving, permissions, address handling, eviction/ban, and resource governance. | VERIFIED | `docs/parity/index.json` lists the Phase 95 surface and upstream sources; `docs/parity/catalog/p2p.md` lines 611-621 cite `net.cpp`, `net_processing.cpp`, `addrman.cpp`, `banman.cpp`, and `net_permissions.cpp`; `scripts/check-phase95-network-participation-release-boundary.ts` lines 35-39 and 304-315 enforce those anchors. |
| 2 | Deterministic checkers reject transaction relay, compact block relay, mempool propagation, public inbound default, production-service, and production-readiness claims for v1.9. | VERIFIED | The checker scans README/parity/runtime-guide corpus for positive deferred-surface claims at lines 86-126 and 318-380; mutation tests at lines 122-202 prove positive claims fail. `bun test scripts/check-phase95-network-participation-release-boundary.test.ts` passed 9 tests / 19 assertions. |
| 3 | Existing outbound sync, full-sync, soak, support-bundle, production no-claim, and release-boundary behavior remains verified and non-regressed. | VERIFIED | `scripts/verify.sh` passed end-to-end in 4m 10.399s, including previous Phase 73-94 release-boundary checkers, Cargo tests, benchmark smoke, Bazel build/provenance, and pure-core coverage. |
| 4 | Operator UAT includes copy-pasteable repo-local Cargo and Bazel commands for loopback or synthetic inbound review. | VERIFIED | `docs/operator/runtime-guide.md` lines 609-692 contain Phase 95 closeout commands for `open-bitcoind`, `open-bitcoin-cli`, `open-bitcoin`, and matching Bazel targets; checker lines 55-65 and 382-386 enforce required command families. |
| 5 | Support bundles preserve useful inbound serving diagnosis while redacting peer addresses where needed. | VERIFIED | `support_status_for_bundle` calls `redact_inbound_resource_governance_evidence` before rendering at `packages/open-bitcoin-cli/src/operator/support/redaction.rs` lines 58-66; endpoint-shaped and sensitive markers are detected at lines 300-347; focused Rust regression passed. |
| 6 | Requirements, roadmap, summaries, verification, and audit artifacts maintain 28/28 requirement traceability. | VERIFIED | `.planning/REQUIREMENTS.md` maps BOUND-01 through BOUND-06 to Phase 95 and reports 28 total / 28 mapped / 0 unmapped; `docs/parity/index.json` parsed successfully with 28 unique v1.9 IDs exactly once; checker lines 481-560 enforce requirements, roadmap, and checklist traceability. |
| 7 | Support bundles preserve inbound resource-governance diagnosis without leaking raw peer, endpoint, payload, permission, config, or credential material. | VERIFIED | `inbound_support_redacts_raw_phase94_resource_governance_material` injects endpoint-shaped values, `peer_id=`, payload, permission, config, RPC password, credential, secret, and cookie material at lines 1153-1171, then asserts JSON and Markdown redaction at lines 1185-1223. |
| 8 | Resource-governance support redaction happens before JSON and Markdown rendering through the shared support status path. | VERIFIED | The sanitizer is in `support_status_for_bundle` at lines 58-66, which is the shared snapshot path consumed by support JSON and Markdown rendering; the regression obtains serialized JSON and rendered Markdown from the same sanitized bundle at lines 1174-1183. |
| 9 | The redaction summary advertises inbound resource-governance safeguards alongside prior inbound support safeguards. | VERIFIED | `INBOUND_RESOURCE_GOVERNANCE_REDACTION_SAFEGUARD` is defined at lines 22-23 and included in `redaction_summary().safeguards` at lines 44-54; existing safeguard test expectations include the new text. |
| 10 | The parity root has one Phase 95 closeout surface for BOUND-01 through BOUND-06. | VERIFIED | `docs/parity/index.json` has `v1-9-network-participation-release-boundary` with requirements exactly `BOUND-01` through `BOUND-06`; direct JSON check reported `v1.9 requirements exactly once: 28/28`. |
| 11 | Human release-readiness docs distinguish bounded opt-in inbound evidence from deferred relay, public-default, service, and production-readiness claims. | VERIFIED | README lines 21-27, `docs/parity/production-claim-boundary.md` lines 10-15 and 49-50, `docs/parity/support-matrix.md` lines 25-30 and 68-70, and release-readiness BOUND rows preserve bounded opt-in evidence while deferring broader claims. |
| 12 | Public entrypoint docs no longer imply all inbound serving is absent after v1.9, while still rejecting public inbound defaults and production participation claims. | VERIFIED | README lines 21-27 and 124-150 state bounded opt-in inbound evidence exists while rejecting public inbound defaults and production readiness; support matrix row `inbound serving` classifies it as `opt-in UAT` with deferred public/default claims. |
| 13 | Default verification runs a Phase 95 checker immediately after Phase 94 and before pure-core checks. | VERIFIED | `scripts/verify.sh` visible order lines 299-302 and executable `run_step` lines 380-384 place Phase 95 after Phase 94 and before `bash scripts/check-pure-core-deps.sh`; checker lines 399-455 validates both text paths. |
| 14 | Full repo verification remains deterministic, local by default, public-network-free, service-manager-free, and release-boundary aware. | VERIFIED | `scripts/verify.sh` executable Phase 95 scanner forbids public-network/service-manager/long-running strings at lines 104-117 and 450-454; full verifier passed. The runtime-guide UAT commands are documented operator guidance, not required manual closeout. |

**Score:** 14/14 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator/support/redaction.rs` | Resource-governance support redaction helper wired from `support_status_for_bundle` | VERIFIED | Exists, substantive, and wired. Defines/calls `redact_inbound_resource_governance_evidence`; detects generic endpoint-shaped values and sensitive markers. |
| `packages/open-bitcoin-cli/src/operator/support/tests.rs` | Regression coverage proving JSON and Markdown support output redacts raw resource-governance material | VERIFIED | Exists and substantive. Test injects raw Phase 94 resource-governance material and asserts `redacted_resource_governance_evidence` in JSON and Markdown. |
| `docs/parity/index.json` | Machine-readable Phase 95 surface and exactly-once BOUND mapping | VERIFIED | JSON parses; Phase 95 surface is `done`; direct check found 28/28 v1.9 requirement IDs exactly once. |
| `docs/parity/checklist.md` | Human checklist row for Phase 95 release-boundary closeout | VERIFIED | Contains `v1-9-network-participation-release-boundary` row with BOUND-01 through BOUND-06 and canonical evidence roots. |
| `docs/parity/catalog/p2p.md` | P2P closeout rollup with Knots anchors | VERIFIED | Contains Phase 95 closeout, required Knots anchors, and explicit no-claim wording. |
| `docs/parity/release-readiness.md` | Release-review matrix for v1.9 closeout | VERIFIED | Maps BOUND-01 through BOUND-06 to evidence, deterministic verification, opt-in UAT posture, residual risk, and next gates. |
| `docs/operator/runtime-guide.md` | v1.9 closeout UAT commands and no-claim guidance | VERIFIED | Contains all required Cargo/Bazel command forms and deterministic verification commands. |
| `README.md` | Public repository status aligned with v1.9 closeout | VERIFIED | Names v1.9 bounded opt-in evidence and rejects public-default, relay, service, and production-readiness claims. |
| `docs/parity/production-claim-boundary.md` | Production claim boundary wording for bounded opt-in inbound evidence vs public defaults | VERIFIED | Preserves Phase 82 vocabulary while adding bounded v1.9 inbound evidence and deferred broad claims. |
| `docs/parity/support-matrix.md` | Support classification wording aligned with bounded v1.9 inbound evidence | VERIFIED | Classifies inbound serving as opt-in UAT and public defaults/production participation as deferred. |
| `scripts/check-phase95-network-participation-release-boundary.ts` | Aggregate Phase 95 checker | VERIFIED | Exports `checkPhase95NetworkParticipationReleaseBoundary`; parses JSON roots, scans no-claim corpus, verifies UAT commands, redaction roots, verifier order, and 28/28 traceability. |
| `scripts/check-phase95-network-participation-release-boundary.test.ts` | Mutation fixture tests for Phase 95 checker | VERIFIED | 9 tests cover pass fixture and negative cases for anchors, forbidden claims, UAT commands, redaction roots, requirement drift, README claims, and heredoc-only verifier wiring. |
| `scripts/verify.sh` | Default verifier wiring immediately after Phase 94 | VERIFIED | Phase 95 test/checker present in visible order and executable `run_step` order before pure-core checks. |

**Artifacts:** 13/13 verified

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `support/redaction.rs` | `support/tests.rs` | `support_status_for_bundle` called by support bundle fixture | WIRED | Test serializes and renders the sanitized bundle after calling shared support bundle path. |
| `support/redaction.rs` | `support/render/inbound.rs` | Sanitized `OpenBitcoinStatusSnapshot` rendered to Markdown | WIRED | Sanitization occurs before JSON/Markdown rendering; regression asserts both formats. |
| `docs/parity/index.json` | `docs/parity/checklist.md` | Matching surface ID and BOUND requirements | WIRED | Same `v1-9-network-participation-release-boundary` ID and exact BOUND list present in machine and human roots. |
| `docs/parity/catalog/p2p.md` | Knots source anchors | Required anchor links | WIRED | P2P catalog cites all five required `packages/bitcoin-knots/src/*.cpp` anchors. |
| `docs/operator/runtime-guide.md` | `scripts/verify.sh` | Default verification command reference | WIRED | Runtime guide line 692 points to `bash scripts/verify.sh`; checker enforces it. |
| `README.md` | `docs/parity/release-readiness.md` | Public release-review pointer | WIRED | README lines 45-46 and 146-150 point public readers to the v1.9 release-readiness closeout. |
| `scripts/verify.sh` | `scripts/check-phase95-network-participation-release-boundary.ts` | `run_step` after Phase 94 checker | WIRED | Executable order lines 380-384 run Phase 95 immediately after Phase 94 and before pure-core checks. |
| `scripts/check-phase95-network-participation-release-boundary.ts` | `docs/parity/index.json` | `JSON.parse` parity root validation | WIRED | Checker reads the fixed corpus and parses `docs/parity/index.json` at lines 166 and 187-194. |
| `scripts/check-phase95-network-participation-release-boundary.ts` | `docs/operator/runtime-guide.md` | Repo-local Cargo/Bazel command assertions | WIRED | Checker requires all command families at lines 55-65 and verifies them at lines 382-386. |

**Wiring:** 9/9 verified

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `support/redaction.rs` | `OpenBitcoinStatusSnapshot.peers.inbound.latest_resource_governance_decision` | `support_status_for_bundle` mutates status before support JSON/Markdown rendering | Yes - preserves bounded status evidence while redacting raw fields | FLOWING |
| `support/tests.rs` | Serialized support bundle and Markdown line | `phase77_support_bundle_with_status` and `render_support_markdown` | Yes - same sanitized bundle drives both outputs | FLOWING |
| `check-phase95...ts` | Fixed target corpus text and parsed parity JSON | `readText` over checked-in docs/source plus `JSON.parse` | Yes - validates current repo files, not hardcoded pass values | FLOWING |
| `scripts/verify.sh` | Executable command order | `run_step` calls in shell script | Yes - `bash scripts/verify.sh` executed the Phase 95 checker before pure-core checks | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Phase 95 checker mutation suite proves expected failure modes | `bun test scripts/check-phase95-network-participation-release-boundary.test.ts` | 9 pass, 0 fail, 19 assertions | PASS |
| Real repo Phase 95 release-boundary corpus validates | `bun run scripts/check-phase95-network-participation-release-boundary.ts` | `validated Phase 95 network participation release boundary` | PASS |
| Resource-governance support redaction regression passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_support_redacts_raw_phase94_resource_governance_material --all-features` | 1 targeted test passed; related binaries had 0 matching tests | PASS |
| v1.9 requirement IDs are exactly once in parity root | `bun -e '...'` exact-count check over `docs/parity/index.json` | `v1.9 requirements exactly once: 28/28` | PASS |
| Full repo non-regression contract passes | `bash scripts/verify.sh` | Completed successfully in 4m 10.399s | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| BOUND-01 | 95-02, 95-03, 95-04 | Release docs, parity docs, and checkers prohibit relay, compact block, mempool, production readiness/service, and public inbound default claims. | SATISFIED | No-claim docs plus checker forbidden-claim scanning and mutation tests. |
| BOUND-02 | 95-02, 95-04 | v1.9 parity breadcrumbs/docs cite required Knots anchors or deviations. | SATISFIED | P2P catalog, parity index, release-readiness, and checker anchor arrays. |
| BOUND-03 | 95-03, 95-04 | Existing outbound sync/full-sync/soak/support/release boundary behavior remains non-regressed. | SATISFIED | Full `bash scripts/verify.sh` passed after Phase 95 and post-review fixes. |
| BOUND-04 | 95-03, 95-04 | Operator UAT guidance includes repo-local Cargo and Bazel command forms. | SATISFIED | Runtime guide Phase 95 section and checker command-family assertions. |
| BOUND-05 | 95-01, 95-04 | Support bundles redact inbound peer addresses while preserving diagnostic evidence. | SATISFIED | Resource-governance sanitizer, redaction summary safeguard, and focused Rust regression. |
| BOUND-06 | 95-02, 95-04 | Requirements, roadmap, summaries, verification, and audit artifacts map every v1.9 requirement exactly once. | SATISFIED | Requirements/roadmap traceability plus parity index exact-count check; this verification report supplies the verification artifact. |

**Coverage:** 6/6 requirements satisfied

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| `scripts/check-phase95-network-participation-release-boundary.ts` | 625 | `console.log` | Info | CLI success message only; not a log-only implementation. |
| `docs/parity/release-readiness.md` | 387 | `console.log` | Info | Documented one-line `bun -e` reviewer command. |
| `scripts/check-phase95-network-participation-release-boundary.test.ts` | 2, 308 | `writeFileSync` | Info | Test fixture writes temporary files; not circular expected-value generation. |

No blocker or warning anti-patterns were found. Disabled test scan found no skipped/todo/ignored Phase 95 tests.

### Human Verification Required

None - Phase 95 is deterministic/static release-boundary verification. Operator UAT commands are documented guidance and are not required manual closeout for this phase.

### Gaps Summary

No gaps found. Phase 95 achieves the goal through existing parity roots, public no-claim wording, shared support redaction, deterministic checker coverage, verifier wiring after Phase 94, and a passing full repo verification contract.

## Verification Metadata

**Verification approach:** Goal-backward verification from ROADMAP success criteria plus PLAN frontmatter must-haves.
**Must-haves source:** ROADMAP success criteria merged with non-duplicative PLAN must-haves.
**Lifecycle provenance:** Validated - `95-CONTEXT.md`, all four `95-*-PLAN.md` files, all four `95-*-SUMMARY.md` files, and this `95-VERIFICATION.md` share `lifecycle_mode: yolo` and `phase_lifecycle_id: 95-2026-06-27T12-48-17`.
**Previous verification:** None found before this report.
**Project instructions used:** `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.
**Project skills:** No `.claude/skills` or `.agents/skills` project skill indexes found.
**Automated checks:** 5 focused checks plus full `bash scripts/verify.sh` passed.
**Human checks required:** 0

---
_Verified: 2026-06-27T16:49:42Z_
_Verifier: the agent (gsd-verifier)_

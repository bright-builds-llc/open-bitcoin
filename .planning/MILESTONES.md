# Milestones: Open Bitcoin

## v1.2 Full Mainnet Network Syncing (Shipped: 2026-05-23)

**Delivered:** An opt-in `open-bitcoind` public-mainnet initial block download workflow with daemon-owned sync activation, outbound peer lifecycle, validated header and block progress, durable restart/recovery, operator observability and control, live-smoke evidence, and security closeout. The shipped scope does not include production-node operation, production-funds wallet use, inbound serving, transaction relay, or packaged-service hardening.

**Phases completed:** 7 phases, 13 plans

**Key accomplishments:**

- Added explicit daemon mainnet sync activation and preflight while keeping public-network behavior opt-in and outside default hermetic verification.
- Built bounded mainnet peer discovery and outbound lifecycle behavior with resolver injection, retry/backoff, stall handling, and peer telemetry.
- Integrated header-first sync, block download/connect, restart recovery, and reorg-aware durable state transitions for the v1.2 IBD claim.
- Made sync progress and health visible through status, dashboard, metrics, logs, RPC-facing output, and operator pause/resume controls.
- Added opt-in live-mainnet smoke reporting and refreshed operator/parity docs so shipped claims stay bounded and auditable.
- Completed the Phase 41 security-analysis audit with `threats_open: 0`, `needs_phase_count: 0`, and no new security implementation phase required before archive.

**Stats:**

- 26/26 v1.2 requirements complete.
- 7 phase directories archived under `.planning/milestones/v1.2-phases/`.
- 89,674 tracked first-party lines in the final LOC report at archive time.
- Full repo-native verification passed during Phase 41 closeout and again through the Phase 41 UAT commit hook.

**Archived artifacts:**

- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`
- `.planning/milestones/v1.2-phases/`

**Known gaps:**

- No dedicated `.planning/v1.2-MILESTONE-AUDIT.md` was created before archive. The archive uses Phase 40 live-smoke closeout plus Phase 41 security audit, verification, and UAT as the milestone-closeout evidence trail.

**What's next:** Start the next milestone with `/gsd-new-milestone`.

---

## v1.1 Operator Runtime and Real-Network Sync (Shipped: 2026-04-30)

**Delivered:** A shipped terminal-first operator runtime milestone with durable storage, real-network sync foundations, truthful status and service surfaces, practical wallet workflows, and audited dry-run migration guidance. The shipped scope does not include unattended public-mainnet full sync through `open-bitcoind`.

**Phases completed:** 22 phases, 69 plans, 60 recorded summary tasks

**Key accomplishments:**

- Shipped the `open-bitcoin` operator binary with rich status output, config-path discovery, and idempotent onboarding backed by hermetic binary coverage.
- Added durable Fjall-backed runtime storage, restart recovery, real-network sync foundations, bounded metrics history, and structured log retention.
- Delivered truthful launchd/systemd service lifecycle behavior plus a Ratatui dashboard that reuses the shared status, metrics, logs, and service contracts.
- Expanded the operator-facing wallet surface with preview/confirm send, managed-wallet backup export, wallet freshness reporting, and scoped RPC wallet selection.
- Made migration auditable and dry-run-first with Core/Knots detection, parity-ledger-backed notices, custom source selection, and selected-source service review truth.
- Closed the post-audit cleanup debt with runtime-backed benchmark fidelity, configless live bootstrap truth, operator-surface cleanup, and the `DetectionScan` ownership-model refactor.

**Stats:**

- 44/44 v1.1 requirements complete.
- 330 files changed across the milestone delivery range.
- 42,363 production Rust lines and 83,494 total first-party lines at ship time.
- Full repo-native verification passed on the archive closeout path, including benchmarks and Bazel smoke builds.

**Archived artifacts:**

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`
- `.planning/milestones/v1.1-phases/`

**What's next:** v1.2 Full Mainnet Network Syncing shipped on 2026-05-23; see the entry above.

---

## v1.0 Headless Parity (Shipped: 2026-04-26)

**Delivered:** A headless Rust Bitcoin node and wallet baseline with audited behavioral parity against Bitcoin Knots `29.3.knots20260210` for the scoped v1.0 surfaces.

**Phases completed:** 22 phases, 80 plans, 72 counted summary tasks

**Key accomplishments:**

- Pinned the Bitcoin Knots baseline and built first-party Rust workspace, Cargo, and Bazel/Bzlmod guardrails.
- Implemented typed primitives, serialization, consensus validation, chainstate, mempool policy, networking, wallet, RPC, CLI, and config surfaces for the scoped headless milestone.
- Added reusable parity evidence through fixtures, cross-implementation harnesses, property-style protocol coverage, benchmark smoke reports, and parity checklist documentation.
- Preserved functional-core boundaries with architecture-policy checks and kept persistence, network, process, and runtime effects in adapter-owned surfaces.
- Closed consensus parity gaps for contextual headers, lax DER handling, subsidy-plus-fees reward limits, and MoneyRange fee-boundary behavior.
- Hardened reachable panic-like production paths into typed errors and added a guard against new unclassified panic sites.
- Archived the milestone after the v1.0 audit closed GAP-01 through GAP-04 with no open blockers.

**Stats:**

- 28/28 v1 requirements complete.
- 22 phase directories retained in `.planning/phases/` for raw execution history.
- Archive artifacts written under `.planning/milestones/`.

**Archived artifacts:**

- `.planning/milestones/v1.0-ROADMAP.md`
- `.planning/milestones/v1.0-REQUIREMENTS.md`
- `.planning/milestones/v1.0-MILESTONE-AUDIT.md`

**What's next:** v1.1 and v1.2 have shipped; see the newer entries above.

---

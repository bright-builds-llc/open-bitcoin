# Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-22
**Phase:** 138-parity-adversarial-pressure-restart-and-release-guardrails
**Mode:** Yolo
**Areas discussed:** Coverage composition, Benchmark gating, Parity and UAT closeout, Claim-guardrail ownership

---

## Coverage composition

| Option | Description | Selected |
|--------|-------------|----------|
| Inventory-and-gap-fill 4×6 matrix Bun checker | Map pinned fixtures, fake-clock, graph-oracle, and failure-injection cells onto existing Phase 130–137 evidence; add only missing cells | ✓ |
| One new integrated mega-harness | Single narrative walk of package → pressure → expiry → checkpoint → restart → retry | |
| Treat per-phase checkers as sufficient | Docs/checkbox wiring only; assume 130–137 VERIFICATION.md already proves MPVFY-01 | |

**User's choice:** Inventory-and-gap-fill with a 4×6 matrix Bun checker (recommended default)
**Notes:** Yolo auto-select. Restart proof stays hermetic: rebuild derived state, reset rolling fee, preserve canonical entries and local unbroadcast, remint retry timers, plus one injected install/write failure. Phases 136 and 137 have no Bun checkers, so named runnable evidence is required.

---

## Benchmark gating

| Option | Description | Selected |
|--------|-------------|----------|
| Count-based work bounds in default verify | Enforce trim/clone/member/cycle counts only | |
| Documented opt-in benches only | Timing and work reports stay outside default verify | |
| Hybrid: default work-count assertions + opt-in timing benches | Default verify gates work units; `--full`/UAT may record latency | ✓ |

**User's choice:** Hybrid: default work-count assertions + opt-in timing benches (recommended default)
**Notes:** Yolo auto-select. Retire the Phase 131 2s `Instant` smoke gate from default verify. Keep `threshold_free` for default smoke reports. `command-timings.ts` is not a release gate.

---

## Parity and UAT closeout

| Option | Description | Selected |
|--------|-------------|----------|
| Closeout surface + 138-UAT.md; flip only MPVFY | Smallest ledger change; leaves PACK/MPDUR/IBR/MPOBS Pending | |
| Closeout surface + 138-UAT.md; flip implemented-Pending rows | Honest ledger without new 132/136 surfaces | |
| Backfill missing 136/PACK/PRESS owners, promote in_progress surfaces, add closeout + 138-UAT.md, flip all implemented-Pending rows | Phase 117 exactly-once ownership of every v2.2 ID | ✓ |
| Collapse all v2.2 evidence into one catalog entry | One review page; fights existing per-phase surface IDs | |

**User's choice:** Backfill missing surfaces, add closeout catalog plus 138-UAT.md, and flip implemented-but-Pending rows (recommended default)
**Notes:** Yolo auto-select. Do not archive the milestone. Do not create a competing closeout manifest. PROJECT.md already deferred PPKG-04/IBR to Phase 138 closeout.

---

## Claim-guardrail ownership

| Option | Description | Selected |
|--------|-------------|----------|
| New Phase 138 checker as last v2.2 no-claim gate | 117 stays the v2.1 BOUND gate; 138 owns MPVFY-04 on current docs | ✓ |
| Extend Phase 117 in place | One last-gate owner; mixes v2.1 BOUND with v2.2 MPVFY | |
| New Phase 138 checker before 117 | Smallest verify.sh change; last gate still does not own MPVFY-04 | |

**User's choice:** New Phase 138 checker as the last v2.2 no-claim gate (recommended default)
**Notes:** Yolo auto-select. Allowed wording is bounded local-package APIs, same-peer 1P1C over ordinary transaction messages, ordinary fanout, and initial-broadcast-retry. Reject general package wire, whole-mempool rebroadcast, public/default/production relay, guaranteed propagation, public-network CI, and production-readiness. Freeze or narrowly allow Phase 117's live corpus so truthful v2.2 copy does not weaken archived v2.1 "package relay remains deferred" wording.

---

## Claude's Discretion

Exact checker helper names, fixture organization, paragraph-classification implementation, smallest honest catalog-owner splits, doc section placement, and whether optional UAT items are recorded as pending or not run.

## Deferred Ideas

- `/gsd-complete-milestone v2.2` archival after this phase passes
- FUT-12 through FUT-17 broader package/relay/production claims

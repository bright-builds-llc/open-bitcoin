---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-debug
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T22:16:00Z
status: resolved
---

# Native gate correction: historical constructor routing

The first default verifier attempt stopped before Rust at the Phase 123 real-repository positive control. It expected `initialize(` and the old daemon `open_with_runtime_activation(` spelling; Phase 157 uses configured initialization and the configured constructor. The synthetic positive passed, demonstrating stale live-source anchors rather than the requested production route.

- [x] Confirm root cause and inventory any equivalent stale routes in the remaining native live guards, without changing production behavior.
- [x] Retarget the failed guard and its independent positive/removal/ordering mutations to the actual configured route and compatibility delegation; preserve all timing, activation and durable startup invariants.
- [x] Run affected live/mutation controls, scoped style/diff checks and independent review of every changed guard; then restart the complete native contract from the beginning.

Initial edit ownership is only `scripts/check-phase123-runtime-timing-evidence-integrity.ts`, its `.test.ts`, and its existing `checks.ts` child. The entrypoint is a thin re-export; the actual routing predicates reside in `scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts`. This child is the same guard task, with no runtime or architectural expansion. Other failed guard paths require a root scope amendment before edits. The debugger owns `.planning/debug/phase157-legacy-verifier-routing.md` and targeted progress here. No production, manifest/verifier bypass, Git, STATE/ROADMAP/REQUIREMENTS/config or completion mutation is delegated. Runtime source fingerprints and the 33 declared mitigations remain unchanged; any guard change receives fresh independent review before finalization.

## Native inventory amendment

The read-only post-123 inventory executed 23 live checks:20 passed,3 failed. Phase 128 still expects a `save_block(...)?` spelling, but the new explicit error branch returns before durable progress/announcement on failure. Phase 129 delegates that same checker. Phase 134 recognizes only the former error-before-dependent-effects order; Phase 157 intentionally applies dependent effects to accepted state before returning persistence failure, with actual behavioral regression proof in Plan 06.

The Phase 128 worker owns only `scripts/check-phase128-production-compact-announcement-transport.ts` and its `.test.ts`, preserving save-success-before-progress/queue and save-failure early return with independent mutations. Phase 129 needs no edit. A separate Phase 134 worker will own the specifically identified guard/reachability/mutation-fixture paths after their exact inventory is recorded. No production changes or blanket helper exclusions are authorized; retain classification, pure/effect boundaries and accepted-before-error ordering checks.

## Phase 134 exact correction boundary

The Phase 134 worker owns the existing `aggregate-roots.ts` and `reachability.ts` children under `scripts/check-phase134-apply-boundaries/`, plus `scripts/check-phase134-authoritative-lifecycle.test.ts` and its existing `fixture.ts`, `apply-helpers.ts`, `apply-helpers/aggregate-reachability.ts`, `apply-helpers/strict-reachability.ts` and `apply-helpers/token-scanner-reachability.ts` children. Use only the necessary subset; record every changed path. Correct the exact connected-root ordering and recognized `persist_result` adapter idiom. Do not whitelist arbitrary `map_err` calls, relax reachable-helper mutation safety, or introduce another parser.

Independent positives must pass before negatives count as evidence. The negative controls must reject error-before-dependent-effects, skipped accepted effects, hidden helper mutation/I/O and unclassified reachable calls while retaining all existing strict/token/reachability controls. Root independently reviews these guard-only corrections before another full native attempt.

## Phase 123 scoped progress | 2026-10-05T22:26:00Z

The Phase 123 repair follows real balanced configured function bodies and exact daemon argument forwarding, preserving the compatibility path and durable initialization order. The thin entrypoint and all production files remain unchanged by this debugger.

- [x] Phase 123 current live CLI and immutable snapshot positive pass.
- [x] Phase 123 complete suite passes: 55 tests, 103 assertions, zero failures; includes 16 new independent live removal, ordering and comment controls alongside all previous timing/write/inbound mutations.
- [x] Managed starter checks and scoped simplification/diff review pass; zero managed findings.
- [x] Read-only live inventory reports 23 commands: 20 pass and three failed paths (Phase 128 durable save spelling, its Phase 129 delegate, Phase 134 connected-root persistence error ordering). Root owns separate scope amendments and delegated repairs.
- [x] Root completes fresh independent guard review, source/security review refresh and full default native verifier rerun before strict finalization.

Evidence: `/tmp/phase157-phase123-final-tests.log`, `/tmp/phase157-legacy-live-inventory.json`, `/tmp/phase157-legacy-live-inventory-newer.json`. The active debug session retains exact causes and handoff state.

## Completion review 2026-10-05T23:10:32.574Z

Configured-route, explicit-save-error and accepted-effect ordering guards were corrected without production changes. Every changed guard received independent review; all91source files are clean, and the complete default native attempt2 passed. Residual boundaries remain the existing software/fixture and deferred product scope, with formal/Git closure separate.

---
phase: 75-multi-day-soak-runner-and-evidence-ledger
verified: 2026-06-15T05:20:05Z
status: passed
score: "12/12 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: "75-2026-06-14T22-59-23"
generated_at: 2026-06-15T05:20:05Z
lifecycle_validated: true
lifecycle_validation_notes:
  - "75-CONTEXT.md and all PLAN.md files carry lifecycle_mode=yolo and phase_lifecycle_id=75-2026-06-14T22-59-23."
  - "75-01 through 75-06 SUMMARY.md files carry matching lifecycle provenance."
  - "Lifecycle validation passed after normalizing body dividers that conflicted with the frontmatter scanner."
overrides_applied: 0
re_verification:
  previous_status: "gaps_found"
  previous_score: "11/12"
  gaps_closed:
    - "Default verification support-bundle regression now expects clean_completion for the target-height fixture instead of unexpected_termination."
  gaps_remaining: []
  regressions: []
---

# Phase 75: Multi-Day Soak Runner and Evidence Ledger Verification Report

**Phase Goal:** Operators can run bounded multi-day full-sync soaks with durable run identity, resumable reports, typed stop reasons, and deterministic synthetic soak coverage.
**Verified:** 2026-06-15T05:20:05Z
**Status:** passed
**Re-verification:** Yes - after gap closure commit `2886dfe`

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Operator can start an explicit opt-in soak with elapsed time, checkpoint interval, target height, datadir, network, peer policy, disk budget, and stop conditions. | VERIFIED | `OperatorCommand::Soak`, `SoakStartArgs`, positive value parsers, and value enums are present in `packages/open-bitcoin-cli/src/operator.rs`; binary soak start coverage remains present in `packages/open-bitcoin-cli/tests/operator_binary.rs`. |
| 2 | The soak runner is wired through the operator binary and uses shared status collection rather than a separate live-smoke script. | VERIFIED | `packages/open-bitcoin-cli/src/operator/runtime.rs` dispatches `OperatorCommand::Soak(args)` to `execute_soak_command`; `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` collects shared status snapshots. |
| 3 | Durable run identity and source-of-truth ledger exist under the selected datadir. | VERIFIED | `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` defines `<datadir>/soak/run-index.json`, per-run `events.jsonl`, `report.json`, and `report.md`, with atomic run-index writes. |
| 4 | Started, checkpoint, resume, stop, and verdict events are typed, versioned, bounded, and resilient to interrupted trailing writes. | VERIFIED | `SoakLedgerEvent` variants and JSONL envelope handling are implemented in `ledger.rs`; partial trailing bytes are ignored while malformed complete lines fail. |
| 5 | Reports are reproducible projections from ledger events and identify source ledger plus latest sequence. | VERIFIED | `packages/open-bitcoin-cli/src/operator/soak/report.rs` builds `SoakReportProjection` from ledger events with `source_ledger_path`, `latest_sequence`, and projection markers. |
| 6 | Final soak outcomes distinguish clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, and unexpected termination. | VERIFIED | `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` defines all six labels and the checker still requires both `clean_completion` and `unexpected_termination` anchors. |
| 7 | Start/resume run a bounded observe loop with production sleeps while tests remain fast. | VERIFIED | `SystemSoakClock` sleeps with `std::thread::sleep`; `SoakTestClock` advances scripted time. Runtime tests remain covered by the focused support and checker evidence below plus the clean review. |
| 8 | Resume preserves original elapsed-time budget and classifies interrupted evidence by the latest invocation segment. | VERIFIED | Resume/start/stop semantics were re-reviewed cleanly in `75-REVIEW-CLEAN-2.md`; runtime code uses latest-invocation event slicing for stop checks. |
| 9 | Operator-stop conditions and external stop races are honored without appending after a terminal stop. | VERIFIED | `75-REVIEW-CLEAN-2.md` reports clean review of start/resume/stop regressions after the resume-stop fix. |
| 10 | `soak stop` rejects terminal latest invocations but can stop active resumed invocations after historical terminal verdicts. | VERIFIED | `75-REVIEW-CLEAN-2.md` confirms the warning is resolved and the regression `soak_runtime_stop_accepts_active_resume_after_historical_terminal_verdict` covers the active-resume case. |
| 11 | Support bundles include compact redacted soak evidence derived from the selected datadir ledger, with missing evidence marked unavailable. | VERIFIED | `collect_soak_support_evidence` in `packages/open-bitcoin-cli/src/operator/support.rs` reads the selected datadir run index and ledger, projects `SoakReportProjection`, and `support/render.rs` renders compact `## Soak Evidence` fields. |
| 12 | Default verification remains green while checking Phase 75 source, docs, tests, support summary, and boundaries without public-network or multi-day execution. | VERIFIED | Gap closed by `2886dfe`: `operator_binary.rs` now expects `clean_completion` for the target-height fixture, `scripts/check-phase75-soak-runner.ts` enforces the updated anchor, and all four focused commands passed. |

**Score:** 12/12 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator.rs` | Clap `SoakArgs` and operator command contract | VERIFIED | Soak command, bounds, peer policy, stop condition, and stop reason parser contracts present. |
| `packages/open-bitcoin-cli/src/operator/runtime.rs` | Runtime dispatch to soak executor | VERIFIED | `OperatorCommand::Soak(args)` dispatches into the soak executor. |
| `packages/open-bitcoin-cli/src/operator/soak.rs` | Soak module entry and domain contracts | VERIFIED | Module entry declares ledger, outcome, report, runtime, and test modules. |
| `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` | Datadir run index and append-only JSONL ledger | VERIFIED | Run layout, atomic index write, append/read, sequence bounds, and partial-line behavior exist. |
| `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` | Outcome labels and classifier | VERIFIED | Six-label taxonomy and shared-evidence classifier present. |
| `packages/open-bitcoin-cli/src/operator/soak/report.rs` | JSON/Markdown report projections | VERIFIED | Projection and report writers derive output from ledger events. |
| `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` | Start/resume/stop/report execution and bounded loop | VERIFIED | Bounded loop, production/test clocks, resume, stop, and report wiring present. |
| `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs` | Runtime helper classification and ledger scanning | VERIFIED | Stop-condition evaluation, latest-invocation helpers, and terminal detection present. |
| `packages/open-bitcoin-cli/src/operator/soak/tests.rs` | Ledger/outcome/report synthetic tests | VERIFIED | Synthetic ledger replay anchors remain enforced by the Phase 75 checker. |
| `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` | Runtime behavior tests | VERIFIED | Clean code review confirms start/resume/stop regression coverage in `75-REVIEW-CLEAN-2.md`. |
| `packages/open-bitcoin-node/src/sync/tests/soak.rs` | Deterministic long-run sync soak tests | VERIFIED | `phase75_synthetic_soak_` anchors remain enforced by the Phase 75 checker. |
| `packages/open-bitcoin-cli/tests/operator_binary.rs` | Operator binary soak/support regression coverage | VERIFIED | Fixed support-bundle binary test passed; source now expects `clean_completion` and `Latest sequence > 0`. |
| `scripts/check-phase75-soak-runner.ts` | Deterministic Phase 75 checker | VERIFIED | Checker passed and enforces updated support projection anchor. |
| `scripts/check-phase75-soak-runner.test.ts` | Checker fixture tests | VERIFIED | Bun test suite passed 8/8. |
| `scripts/verify.sh` | Default verifier wiring | VERIFIED | Phase 75 checker test and checker run are wired after `check-v1.6-release-boundaries.ts` and before Rust verification. |
| `docs/operator/runtime-guide.md` and parity docs | Operator commands, proof boundaries, parity surface | VERIFIED | Phase 75 checker verifies docs, Cargo/Bazel command forms, and parity surface `phase75-multi-day-soak-runner-evidence-ledger`. |
| `docs/parity/source-breadcrumbs.json` | Breadcrumbs for new Rust files | VERIFIED | Breadcrumb anchors remain part of Phase 75 checker coverage. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `operator.rs` | `operator/runtime.rs` | `OperatorCommand::Soak(args)` | WIRED | Parsed soak commands enter the operator runtime dispatch. |
| `operator/runtime.rs` | `operator/soak/runtime.rs` | `execute_soak_command` | WIRED | Runtime dispatch calls the soak executor. |
| `operator/soak/runtime.rs` | `operator/soak/ledger.rs` | ledger create/resume/read/append APIs | WIRED | Start, resume, stop, and report flows read and write durable ledger artifacts. |
| `operator/soak/runtime.rs` | shared status collection | `collect_status_snapshot` | WIRED | Checkpoints use existing operator status snapshots. |
| `operator/soak/report.rs` | `operator/soak/ledger.rs` | `SoakLedgerEventEnvelope` projection | WIRED | Reports are projections over ledger events. |
| `operator/support.rs` | soak ledger/report | `collect_soak_support_evidence` | WIRED | Support evidence reads the selected datadir ledger and derives compact projection fields. |
| `operator/support/render.rs` | support soak evidence | `push_soak_evidence` | WIRED | Markdown renders state, run, final outcome, source ledger, report paths, and latest sequence. |
| `scripts/verify.sh` | `scripts/check-phase75-soak-runner.ts` | Bun checker command | WIRED | Default verifier runs the Phase 75 checker after v1.6 release-boundary checks. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| Soak start command | `SoakBounds` and run id | Clap args plus resolved datadir/network | Yes | FLOWING - binary fixture starts a target-height run and writes durable ledger/report files. |
| Support bundle soak evidence | `SoakSupportEvidence` | selected datadir `run-index.json` plus latest run `events.jsonl` | Yes | FLOWING - support collector rejects missing/mismatched evidence and projects final outcome from ledger events. |
| Support Markdown | final outcome/latest sequence labels | `SupportEvidenceBundle.soak_evidence` | Yes | FLOWING - `push_soak_evidence` renders the same fields checked in JSON. |
| Phase 75 checker | support projection anchors | source files, docs, plans, `scripts/verify.sh` | Yes | FLOWING - checker passed after anchor update from `unexpected_termination` to `clean_completion` for this fixture. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Fixed support-bundle binary projection | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_includes_phase75_soak_summary --all-features` | 1 passed, 0 failed | PASS |
| Support bundle soak unit coverage | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase75_soak_support_ --all-features` | 4 passed, 0 failed | PASS |
| Phase 75 boundary checker | `bun run scripts/check-phase75-soak-runner.ts` | `validated Phase 75 soak runner and evidence ledger boundaries` | PASS |
| Phase 75 checker fixture tests | `bun test scripts/check-phase75-soak-runner.test.ts` | 8 passed, 0 failed | PASS |
| Clean final code review | `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-REVIEW-CLEAN-2.md` | 0 findings; status clean | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| SOAK-01 | 75-01, 75-02, 75-04, 75-05, 75-06 | Explicit opt-in multi-day soak with durable run identity, checkpoints, and resumable report state | SATISFIED | Soak command, datadir ledger, resume/report support projection, docs, checker, and binary flows are implemented and verified. |
| SOAK-02 | 75-01, 75-02, 75-05, 75-06 | Bounds by elapsed time, target height, datadir, network, peer policy, disk budget, and stop condition without changing default verification | SATISFIED | Parser/runtime bounds are present; `scripts/verify.sh` runs deterministic checker coverage, not public-network or multi-day work. |
| SOAK-03 | 75-01, 75-02, 75-04, 75-05, 75-06 | Distinguish clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, unexpected termination | SATISFIED | Outcome taxonomy remains present, and the fixed support fixture now correctly reports `clean_completion` for target-height completion while retaining unexpected-termination anchors elsewhere. |
| SOAK-04 | 75-03, 75-05, 75-06 | Replay deterministic synthetic soak scenarios without public-network or multi-day tests | SATISFIED | Checker enforces synthetic Rust test anchors and default-verification boundary strings; checker tests passed. |

No additional Phase 75 SOAK IDs are orphaned in `.planning/REQUIREMENTS.md`; SOAK-01 through SOAK-04 are mapped to Phase 75 and appear in Phase 75 plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| `scripts/check-phase75-soak-runner.ts` | 365 | `console.log` | Info | Expected CLI success output for the checker, not a stub or unimplemented handler. |

No TODO/FIXME/placeholder, hardcoded-empty user-output, or console-only implementation blockers were found in the changed support fixture/checker paths.

### Human Verification Required

None for the Phase 75 deterministic default-verification contract. Actual public-network multi-day soak UAT remains explicit opt-in operator work and is outside this phase's automated pass/fail gate.

### Gaps Summary

No remaining gaps. The previous support-bundle gap is closed: the target-height fixture now asserts `clean_completion`, the checker anchors match that intended behavior, and the focused Cargo and Bun verification commands pass.

---

_Verified: 2026-06-15T05:20:05Z_
_Verifier: the agent (gsd-verifier)_

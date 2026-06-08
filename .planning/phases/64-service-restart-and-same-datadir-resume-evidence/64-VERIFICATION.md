---
phase: 64-service-restart-and-same-datadir-resume-evidence
verified: 2026-06-08T03:43:38.403Z
status: passed
score: "4/4 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:43:38.403Z
lifecycle_validated: true
---

# Phase 64: Service Restart and Same-Datadir Resume Evidence Verification

**Phase Goal:** Operators can prove service-supervised restarts reopen durable
state and resume sync safely.
**Verified:** 2026-06-08T03:43:38.403Z
**Status:** passed

## Goal Achievement

| # | Roadmap Success Criterion | Status | Evidence |
|---|---|---|---|
| 1 | Daemon restart under service supervision reopens durable sync state and reports clean versus unclean prior shutdown. | VERIFIED | `ServiceRestartResumeStatus` exposes `prior_shutdown`, `datadir`, `same_datadir`, progress, stale in-flight, recovery category, and next action. Focused node and CLI tests passed for contract serialization and clean/unclean metadata projection. |
| 2 | Same-datadir restart tests cover extended loop recovery without duplicate block requests, duplicate block connects, corrupted active chainstate, or lost progress counters. | VERIFIED | Phase 58 durable same-datadir tests remain the core duplicate/progress guard; Phase 64 status projection loads metadata from the selected datadir and surfaces durable downloaded/connected progress. |
| 3 | Restart and resume status gives explicit next-action guidance and resumes bounded sync work without stale in-flight requests. | VERIFIED | CLI projection tests cover zero in-flight as `cleared` and nonzero as `stale_requests_recorded`, and preserve recovery action guidance. |
| 4 | Restart and resume evidence is available through deterministic tests and opt-in UAT reports without making public-network checks part of default verification. | VERIFIED | `scripts/check-phase64-service-restart-resume.ts` passes and is invoked by `scripts/verify.sh`; checker guards docs/source strings and rejects live service-manager/public-network commands in default verification. |

## Required Artifacts

| Artifact | Status | Details |
|---|---|---|
| `packages/open-bitcoin-node/src/status.rs` | VERIFIED | Defines Phase 64 restart/resume status types and additive `ServiceStatus.restart_resume` serde default. |
| `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` | VERIFIED | Adds `durable_runtime_metadata()` and reuses it for durable sync state. |
| `packages/open-bitcoin-cli/src/operator/status/service_status.rs` | VERIFIED | Projects selected-datadir metadata into service restart/resume evidence. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | VERIFIED | Renders compact `restart_resume=` human status evidence. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` | VERIFIED | Adds restart/resume rows to the Service dashboard section. |
| `packages/open-bitcoin-cli/src/operator/service.rs` | VERIFIED | Successful service restart output points operators to status JSON with the same datadir. |
| `docs/operator/runtime-guide.md` | VERIFIED | Documents repo-local Cargo/Bazel review commands and field interpretation. |
| `docs/parity/catalog/p2p.md` | VERIFIED | Frames Phase 64 as scoped service-supervised restart/resume evidence. |
| `scripts/check-phase64-service-restart-resume.ts` and `scripts/verify.sh` | VERIFIED | Deterministic checker is wired into default verification. |

## Verification Commands

Focused checks:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node service_restart_resume_status_contract --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_resume --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_resume_status_render --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_service_restart_resume --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_calls_manager_and_renders_commands --all-features
bun run scripts/check-phase64-service-restart-resume.ts
bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check
```

Aggregate gate:

```bash
bash scripts/verify.sh
```

Result: passed, with `verify.sh completed in 3m 46.964s (226964ms)`.

## Gaps Summary

No deterministic Phase 64 gaps found. Real launchd/systemd restart review and
public-network `--restart-after-progress` smoke remain optional UAT evidence and
are intentionally excluded from default verification.

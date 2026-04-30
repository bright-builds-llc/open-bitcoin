---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 23-2026-04-28T17-18-36
generated_at: 2026-04-28T17:18:36.921Z
---

# Phase 23: Service Apply Completion and Status Truthfulness - Context

## Phase Boundary

**Goal:** Finish real launchd and systemd apply semantics and make service state
projections truthful for status and dashboard surfaces.

**Success criteria:**
1. `open-bitcoin service install --apply` executes the required bootstrap or
   reload-enable steps on supported platforms after writing service artifacts.
2. Service status and dashboard summaries distinguish installed, enabled,
   running, failed, stopped, and unmanaged states using real manager evidence
   instead of inference.
3. Dashboard service actions reuse the corrected apply path and remain
   confirmation-gated.
4. Service verification evidence and requirements bookkeeping are refreshed
   around the corrected runtime behavior.

**Out of scope:**
- Wallet-aware live-status fixes and build provenance work owned by Phase 24.
- Migration source-selection hardening owned by Phase 25.
- The broader orphaned requirement reconciliation owned by Phase 26.
- New operator start, restart, or packaged-service workflows beyond the
  existing `install`, `uninstall`, `enable`, `disable`, and `status` surface.

## Requirements In Scope

- `SVC-01`: macOS launchd lifecycle support with dry-run plist previews.
- `SVC-02`: Linux systemd lifecycle support with dry-run unit previews.
- `SVC-03`: Service commands surface scope, generated files, command previews,
  and recovery context before mutation.
- `SVC-04`: Service status truthfully reports installed, enabled, running,
  failed, stopped, and unmanaged states plus diagnostics.
- `SVC-05`: Service lifecycle tests stay hermetic and never mutate the
  developer machine's real service manager state.
- `DASH-03`: Dashboard service-affecting actions remain confirmation-gated and
  reuse the same corrected service runtime path.

## Canonical References

- `.planning/ROADMAP.md` — Phase 23 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — service and dashboard requirement ledger.
- `.planning/v1.1-MILESTONE-AUDIT.md` — blocker `INT-01`, service flow break,
  and enabled-state ambiguity findings.
- `AGENTS.md` — repo-native verification contract and phase-document workflow
  expectations.
- `packages/open-bitcoin-cli/src/operator/service.rs` — shared service
  contracts, renderers, and runtime dispatch.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` — launchd apply
  and status adapter.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` — systemd apply
  and status adapter.
- `packages/open-bitcoin-cli/src/operator/status.rs` — dashboard or CLI status
  projection from service snapshots.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` — confirmation
  gate and shared service-action runtime path.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Keep service adapters in the imperative shell and parsing or projection logic
  in pure helpers where practical.
- Prefer focused hermetic tests over real service-manager mutation in local
  verification.
- Refresh contributor-facing planning evidence when a gap-closure phase lands.

## Current State

- `LaunchdAdapter::install()` and `SystemdAdapter::install()` write service
  files and return preview command lists, but `--apply` stops after the write
  instead of completing registration with the manager.
- `collect_service_status()` infers `enabled` from `ServiceLifecycleState`,
  which cannot represent combinations like "failed but enabled" or "running but
  not enabled".
- Dashboard service actions already route through `execute_service_command()` and
  remain confirmation-gated, so fixing the shared service path should close the
  dashboard install gap without a second action implementation.

## Decisions

1. **Install apply completes registration immediately after writing service
   files.**
   - launchd: run `launchctl enable` then `launchctl bootstrap`
   - systemd: run `systemctl --user daemon-reload` then
     `systemctl --user enable`
2. **Dry-run previews and apply-mode behavior must stay aligned.**
   The same previewed command sequence should be what the apply path executes so
   operator guidance stays trustworthy.
3. **Manager-reported startup enablement becomes explicit snapshot data.**
   `ServiceStateSnapshot` should carry `maybe_enabled` so downstream status and
   dashboard projections stop guessing.
4. **Status truth prefers manager evidence over enum inference.**
   The shared operator status model must preserve combinations like
   failed-plus-enabled and running-plus-disabled when the manager reports them.
5. **Hermetic parser and projection tests are the verification backbone.**
   The phase should add pure parsing coverage for `launchctl print-disabled` and
   `systemctl is-enabled`, plus fake-manager status projection tests and dry-run
   command-preview assertions.
6. **Phase 23 owns its own bookkeeping refresh.**
   The phase should update roadmap progress, requirements traceability, summary
   frontmatter, and verification evidence instead of reopening Phase 18 or Phase
   19 summaries.

## Key Files and Likely Change Surfaces

- `packages/open-bitcoin-cli/src/operator/service.rs`
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs`
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs`
- `packages/open-bitcoin-cli/src/operator/service/fake.rs`
- `packages/open-bitcoin-cli/src/operator/service/tests.rs`
- `packages/open-bitcoin-cli/src/operator/status.rs`
- `packages/open-bitcoin-cli/src/operator/status/tests.rs`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`

## Risks

- `launchctl print-disabled` output varies between systems, so the parser must
  tolerate both `=>` and `=` forms and degrade cleanly when the command is
  unavailable.
- `systemctl is-active` and `is-enabled` can disagree, so status projection
  must preserve both signals instead of flattening them into one enum guess.
- Apply-mode failures can still leave a written unit or plist behind, so
  descriptions and diagnostics must make the partial state obvious to operators.

## Implementation Notes

- Reuse pure helper functions for preview-command generation and status parsing
  so dry-run tests cover the same command sequences apply mode uses.
- Keep the dashboard action module unchanged unless a shared-service-path gap is
  discovered during verification.
- Prefer small, focused test additions over broad integration fixtures for this
  phase.

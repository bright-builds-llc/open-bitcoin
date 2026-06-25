---
phase: 91-peer-permissions-and-connection-classes
plan: 10
subsystem: verification
tags: [verification, checker, peer-permissions, no-claim-boundary]

requires:
  - phase: 91-09
    provides: "Documented permission evidence, UAT commands, and parity roots"
provides:
  - "Deterministic Phase 91 checker for permission evidence and no-claim boundaries"
  - "Default verifier wiring for the Phase 91 checker immediately after Phase 90"
  - "File-length remediation for Phase 91 Rust source touched during verification"
affects:
  - scripts/verify.sh
  - docs/parity/source-breadcrumbs.json
  - 95-network-participation-evidence-and-release-boundary

tech-stack:
  added: []
  patterns:
    - "Use fixed-file Bun checkers for deterministic docs/parity/no-claim verification"
    - "Keep default verification local and free of public-network, service-manager, and long-running gates"
    - "Split Rust modules at responsibility boundaries when verification exposes file-length drift"

key-files:
  created:
    - scripts/check-phase91-peer-permissions.ts
    - scripts/check-phase91-peer-permissions.test.ts
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-network/src/inbound/permissions/error.rs
    - .planning/phases/91-peer-permissions-and-connection-classes/91-10-SUMMARY.md
  modified:
    - scripts/verify.sh
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-network/src/inbound/permissions.rs

key-decisions:
  - "Phase 91 drift checks read fixed evidence files only, not `.planning/` archives."
  - "Forbidden-claim detection allows explicit no-claim/deferred wording but rejects unscoped support claims."
  - "Support-bundle redaction and permission parse-error types were split into child modules to satisfy production Rust file-length limits."

requirements-completed: [PERM-01, PERM-02, PERM-03, PERM-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T19:27:57Z

duration: 27min
completed: 2026-06-25
---

# Phase 91 Plan 10: Deterministic Checker and Verifier Wiring Summary

**Phase 91 now has deterministic guardrails in default verification for permission evidence, UAT command forms, parity roots, breadcrumbs, inactive effect labels, support redaction boundaries, and deferred-surface no-claim language.**

## Accomplishments

- Added `scripts/check-phase91-peer-permissions.ts` to validate the Phase 91 surface id, PERM-01 through PERM-04, Knots anchors, required evidence files, source breadcrumbs, permission labels, metrics, UAT commands, and verifier order.
- Added fixture tests covering the passing evidence shape plus missing requirements, missing UAT commands, missing breadcrumbs, missing inactive labels, forbidden verifier drift, forbidden relay/public-default/production/whitelist claims, and raw support evidence leakage.
- Wired the Phase 91 checker and its test into `scripts/verify.sh` immediately after the Phase 90 checker.
- Kept `scripts/verify.sh` free of public-network, service-manager, multi-day, whitelist, and whitebind execution text.
- Split support-bundle permission redaction helpers into `support/redaction.rs` and permission parse errors into `inbound/permissions/error.rs` after the file-length guard exposed Phase 91 Rust source drift.
- Updated `docs/parity/source-breadcrumbs.json` for the new first-party Rust files.

## Task Commits

1. **Task 1: Write Phase 91 checker fixture tests** - pending commit (`test`)
2. **Task 2: Implement fixed-file Phase 91 checker** - pending commit (`test`)
3. **Task 3: Wire checker into repo verification and run final checks** - pending commit (`test/refactor`)

## Files Created/Modified

- `scripts/check-phase91-peer-permissions.ts` - Fixed-file Phase 91 evidence and boundary checker.
- `scripts/check-phase91-peer-permissions.test.ts` - Bun fixture tests for checker pass/fail behavior.
- `scripts/verify.sh` - Adds visible and executed Phase 91 checker steps after Phase 90.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Extracted support-bundle redaction helpers.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Keeps support bundle orchestration under the file-length limit.
- `packages/open-bitcoin-network/src/inbound/permissions/error.rs` - Extracted peer permission parse-error type.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - Keeps permission domain logic under the file-length limit.
- `docs/parity/source-breadcrumbs.json` - Adds breadcrumbs for the new Rust source files.

## Verification Results

- `rg -n "PERM-01|PERM-02|PERM-03|PERM-04|v1-9-peer-permissions-connection-classes|openbitcoininboundpermissionclass|inactive_relay|whitebind|public inbound by default|Arrange|Act|Assert" scripts/check-phase91-peer-permissions.test.ts` - passed
- `rg -n "checkPhase91PeerPermissions|REQUIRED_UAT_COMMANDS|REQUIRED_EVIDENCE_LABELS|FORBIDDEN|inactive_relay|inactive_blockfilters" scripts/check-phase91-peer-permissions.ts` - passed
- `bun test scripts/check-phase91-peer-permissions.test.ts` - passed, 8 tests
- `bun run scripts/check-phase91-peer-permissions.ts` - passed
- `rg -n "test Phase 91 peer permissions checker|check Phase 91 peer permissions|check-phase91-peer-permissions" scripts/verify.sh` - passed
- `! rg -n "public-network|service-manager|multi-day|whitebind|whitelist" scripts/verify.sh` - passed
- `bun run scripts/check-phase90-inbound-listener-admission.ts` - passed
- `bash scripts/check-file-lengths.sh` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-network --all-features --no-run` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib inbound_support_json_and_markdown_redact_raw_permission_config_evidence -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib class_definitions_reject_direction_only_missing_in_and_outbound_rules -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib class_addresses_accept_only_literal_ip_values -- --nocapture` - passed
- `git diff --check` - passed

## Deviations from Plan

- The planned checker and verifier wiring exposed existing Phase 91 Rust production file-length drift. Remediation stayed mechanical: split redaction and parse-error helpers into child modules and added their breadcrumbs.
- Full `bash scripts/verify.sh` remains a phase-final check because local generated Rust test binaries have previously hung during unscoped test execution.

## Next Phase Readiness

Phase 91 is ready for code review and phase-level verification. The checker provides reusable guardrails for Phase 95 release-boundary evidence.

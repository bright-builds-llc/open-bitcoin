---
phase: 21-drop-in-parity-audit-and-migration
verified: 2026-04-28T00:35:35Z
status: passed
score: 5/5 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-28T00:35:35Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 21: Drop-In Parity Audit and Migration Verification Report

**Phase Goal:** Make Open Bitcoin's replacement story evidence-based, explicit, and safe for users with existing Core or Knots installs.
**Requirements:** CLI-07, WAL-08, MIG-01, MIG-02, MIG-03, MIG-04, MIG-05
**Verified:** 2026-04-28T00:35:35Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The repository now has an auditable drop-in migration surface across CLI, RPC, config, datadir layout, service behavior, wallet behavior, sync/logging, and operator documentation. | VERIFIED | [`docs/parity/catalog/drop-in-audit-and-migration.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/catalog/drop-in-audit-and-migration.md) records the Phase 21 audit matrix; [`docs/parity/index.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json) and [`docs/parity/deviations-and-unknowns.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/deviations-and-unknowns.md) register the machine-readable and human-readable deviation trail. |
| 2 | Existing Core/Knots install evidence remains read-only and is visible through detection, onboarding, status, and migration planning on macOS and Linux. | VERIFIED | [`packages/open-bitcoin-cli/src/operator/detect.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/detect.rs) still provides read-only install, config, cookie, service, and wallet detection; [`packages/open-bitcoin-cli/src/operator/onboarding.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/onboarding.rs) and [`packages/open-bitcoin-cli/src/operator/status.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/status.rs) surface that evidence; [`packages/open-bitcoin-cli/tests/operator_binary.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_binary.rs) proves migration planning uses sandboxed source evidence without mutation. |
| 3 | `open-bitcoin migrate plan` explains benefits, tradeoffs, unsupported surfaces, rollback expectations, and backup requirements before the action list. | VERIFIED | [`packages/open-bitcoin-cli/src/operator/migration.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/migration.rs) renders the explanation-first human and JSON output; [`packages/open-bitcoin-cli/src/operator/migration/planning.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/migration/planning.rs) builds the typed explanation model; planner unit tests assert those sections appear. |
| 4 | Dry-run migration plans enumerate config, file/datadir, service, wallet, and follow-up actions without mutating source installs or leaking secrets. | VERIFIED | [`packages/open-bitcoin-cli/src/operator/migration/planning.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/migration/planning.rs) builds the grouped read-only/manual/deferred actions; [`packages/open-bitcoin-cli/src/operator/migration/tests.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/migration/tests.rs) covers redaction and ambiguity handling; [`packages/open-bitcoin-cli/tests/operator_binary.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_binary.rs) verifies the source sandbox remains unchanged after `migrate plan`. |
| 5 | Relevant intentional differences are recorded in the parity ledger and surfaced by migration output with a sync guard that prevents runtime/doc drift. | VERIFIED | [`docs/parity/index.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json) stores the migration deviation records; [`packages/open-bitcoin-cli/tests/operator_flows.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows.rs) asserts the runtime notice IDs and summaries match the parity ledger; [`packages/open-bitcoin-cli/src/operator/migration/planning.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/migration/planning.rs) filters and renders only the relevant notices. |

**Score:** 5/5 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| CLI-07 | SATISFIED | Migration planning reuses the existing read-only Core/Knots detection model and keeps source evidence explicit rather than hidden or mutated. |
| WAL-08 | SATISFIED | External wallet candidates are enumerated read-only in the migration plan, with backup requirements and no import/copy/rewrite path. |
| MIG-01 | SATISFIED | The new Phase 21 audit matrix and parity ledger entries document the drop-in migration surface across the required categories. |
| MIG-02 | SATISFIED | Existing onboarding and status detection support remains intact and is reused directly by the migration surface. |
| MIG-03 | SATISFIED | The migration planner output always explains benefits, tradeoffs, unsupported surfaces, rollback expectations, and backup requirements before the action list. |
| MIG-04 | SATISFIED | `open-bitcoin migrate plan` renders a dry-run-only action plan that lists config, file, service, wallet, and follow-up actions before any future mutation-capable work. |
| MIG-05 | SATISFIED | Migration-relevant deviations are recorded in `docs/parity/index.json`, summarized in the human rollup, and surfaced by runtime output with a dedicated sync test. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows -- --nocapture` passed.
- `node "$HOME/.claude/get-shit-done/bin/gsd-tools.cjs" verify lifecycle "21" --require-plans --require-verification --raw` returned `valid`.
- `bash scripts/verify.sh` passed end-to-end, including:
  - deterministic LOC freshness
  - parity breadcrumb validation
  - pure-core dependency/import checks
  - production Rust file-length validation
  - panic-site validation
  - Rust workspace build, test, and doctest coverage
  - benchmark smoke report generation
  - Bazel smoke build

## Human Verification Required

None. Phase 21 ships read-only operator planning and documentation surfaces with hermetic tests and repo-native verification.

## Residual Risks

- Automatic or destructive migration remains intentionally out of scope. Any future apply-mode work must keep the current dry-run-first and backup-aware safety posture unless a later phase explicitly changes that boundary.
- Phase 22 still needs the release-hardening and benchmark closeout work before the milestone can make broader v1.1 readiness claims.

---

_Verified: 2026-04-28T00:35:35Z_
_Verifier: GPT-5.4 (GSD yolo wrapper)_

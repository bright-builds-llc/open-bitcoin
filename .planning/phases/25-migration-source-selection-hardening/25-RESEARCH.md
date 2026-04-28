---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:28:00.000Z
---

# Phase 25 Research

## Audit Finding Being Closed

### `INT-W03`

- **Problem:** `open-bitcoin migrate plan --source-datadir <custom-path>` only
  works when the custom path was already inside the default detection roots.
  Otherwise the planner falls back to manual review instead of selecting the
  requested source install.
- **Evidence:** `.planning/v1.1-MILESTONE-AUDIT.md` points to
  `packages/open-bitcoin-cli/src/operator/runtime.rs` and
  `packages/open-bitcoin-cli/src/operator/migration/planning.rs`.

## Existing Code Facts

1. `execute_operator_cli_inner()` in
   `packages/open-bitcoin-cli/src/operator/runtime.rs` gathers install detection
   once from `detection_roots(&config_resolution)` and reuses that evidence for
   every command.
2. `detection_roots()` only includes the target Open Bitcoin datadir plus the
   detector's default home-based locations, so a migration source datadir in an
   arbitrary custom location is invisible unless it already lives in those
   roots.
3. `plan_migration()` in
   `packages/open-bitcoin-cli/src/operator/migration/planning.rs` already has
   the right explicit selector shape: it takes `--source-datadir` and tries to
   match it to a detected installation.
4. The existing operator-binary migration test uses `source_data_dir =
   sandbox.child(".bitcoin")`, which is equivalent to `HOME/.bitcoin` and
   therefore cannot detect the custom-path gap.

## Constraints From Guidance

- Phase 21 context decision `D-04` says to reuse existing detection evidence
  instead of inventing a new scanner.
- Phase 21 decisions `D-02`, `D-05`, and `D-07` keep migration dry-run only,
  explanation-first, and secret-safe.
- Bright Builds verification and testing guidance favor a narrow runtime helper
  plus focused unit and binary regressions before the repo-native verify gate.

## Chosen Implementation Strategy

### Runtime detection

- Introduce a command-scoped detection helper in `operator/runtime.rs`.
- For migration only, clone the normal detection roots and append the explicit
  `--source-datadir` to `roots.data_dirs` before calling the shared
  `detect_existing_installations()` helper.
- Leave all other commands on the existing detection path.

### Planner hardening

- Keep the explicit match behavior in `select_source_installation()`, but add a
  small guard that rejects explicit paths with no source config, cookie, or
  wallet evidence.
- Return clear manual-review guidance when the explicit path exists but does not
  yet look like a supported Core or Knots source datadir.

### Verification

- Planner unit test for unsupported explicit paths.
- Operator-binary regression test for an explicit custom datadir outside default
  detection roots.
- Full `open-bitcoin-cli` package test run.
- Repo-native `bash scripts/verify.sh` before finalization.

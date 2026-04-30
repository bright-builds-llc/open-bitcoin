---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 29-2026-04-29T14-01-32
generated_at: 2026-04-29T14:01:32Z
---

# Phase 29: Closeout Hygiene and Build Provenance - Context

## Phase Boundary

**Goal:** Address the remaining optional post-audit cleanup around build
provenance truthfulness and milestone-closeout hygiene before archive.

**Success criteria:**
1. Build provenance claims stay truthful for non-Cargo builds through either
   populated metadata or explicitly documented unavailable behavior.
2. Milestone closeout docs and roadmap surfaces remain internally consistent
   after the final gap-closure work.
3. Optional cleanup added here does not reopen the passing benchmark,
   migration, or service-flow gates.

**Out of scope:**
- Reopening the Phase 28 service log-path repair.
- Changing the shared status contract beyond what is required to keep build
  provenance truthful.
- Broad milestone archive work such as `/gsd-complete-milestone`.
- New release packaging or distribution workflows outside the existing Cargo and
  Bazel local build surfaces.

## Requirements In Scope

This phase is optional cleanup and does not reopen requirement IDs in
`.planning/REQUIREMENTS.md`.

It still protects the already-closed operator truthfulness contract by keeping
`build.*` fields honest across supported local build paths and by leaving the
milestone ledgers internally consistent.

## Canonical References

- `.planning/ROADMAP.md` — Phase 29 goal, dependency edge, and success
  criteria.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-27.md` — remaining tech debt note that
  Bazel-built CLI targets likely surface build provenance as `Unavailable`.
- `.planning/PROJECT.md` — operator-surface truthfulness and verification
  expectations.
- `.planning/phases/24-wallet-aware-live-status-and-build-provenance/24-CONTEXT.md`
  — original build-provenance intent and unavailable-value semantics.
- `.planning/phases/24-wallet-aware-live-status-and-build-provenance/24-02-SUMMARY.md`
  — Cargo-side build provenance implementation details and the explicit non-Cargo
  fallback note that Phase 29 closes.
- `packages/open-bitcoin-cli/src/operator/status.rs` — shared build-provenance
  collection path.
- `packages/open-bitcoin-cli/BUILD.bazel` — current Bazel Rust target wiring for
  the shared CLI library and binaries.
- `packages/open-bitcoin-cli/build.rs` — Cargo-side build metadata emission.
- `.bazelrc` — repo-local Bazel defaults.
- `scripts/verify.sh` — repo-native verification contract including the Bazel
  smoke build.
- `docs/operator/runtime-guide.md` — operator-facing status and dashboard
  guidance.
- `docs/architecture/status-snapshot.md` — shared status contract and
  unavailable semantics.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Keep Bash thin for small orchestration helpers; prefer Bun and TypeScript for
  richer validation logic.
- Keep build-provenance truth at the compile boundary instead of inventing
  runtime guesses.
- Prefer early returns and small helpers over nested control flow.
- Keep optional or unavailable values explicit rather than silently defaulting
  them.

## Current State

- Cargo builds already populate build provenance through
  `packages/open-bitcoin-cli/build.rs`.
- The shared status collector reads build metadata via `option_env!`, so a build
  system only needs to inject truthful compile-time env vars for the fields to
  appear.
- Bazel currently builds `packages/open-bitcoin-cli` without setting those env
  vars, so a Bazel-built `open-bitcoin status --format json` shows
  `build.commit`, `build.build_time`, `build.target`, and `build.profile` as
  unavailable.
- The same Bazel path also leaves `build.version` at the rules_rust default
  `0.0.0`, which is another truthfulness bug for operator-facing status output.
- Phase 28 already repaired the other mandatory post-audit blocker and the
  planning ledgers now point at this optional cleanup as the final remaining
  v1.1 phase.

## Decisions

1. **Repair Bazel provenance at compile time, not at runtime.**
   The shared status collector already models unavailable values correctly, so
   the fix belongs in Bazel target wiring rather than in new runtime detection
   logic.
2. **Stamp the shared CLI library, not just one binary.**
   `packages/open-bitcoin-cli/src/operator/status.rs` lives in the shared
   library target, so the Bazel provenance fix should attach to
   `open_bitcoin_cli_lib` and flow to every consuming binary.
3. **Use one repo-owned Bazel workspace-status script for Git commit metadata.**
   The repo currently has no Bazel status command. A thin checked-in script
   keeps commit metadata explicit and reviewable.
4. **Treat Bazel target and profile strings as build-system-specific provenance.**
   The Cargo path emits Rust `TARGET` and `PROFILE`; Bazel can truthfully expose
   `TARGET_CPU` and `COMPILATION_MODE` instead of pretending they are identical.
5. **Close the phase with automated Bazel runtime evidence plus repo-native
   verification.**
   A focused Bazel-built `status --format json` check should prove the surfaced
   fields stay populated under the non-Cargo path the audit called out.

## Risks And Watchouts

- Enabling Bazel stamping on the wrong target could broaden cache invalidation
  beyond the shared CLI library.
- Build-time metadata must remain explicit when unavailable; placeholder strings
  like `unknown` would be less truthful than a proper unavailable state.
- The verification addition should stay fast enough to remain part of the
  repo-native contract.
- Phase 29 is optional cleanup, so it should not reopen the Phase 24 or Phase
  28 requirement ledger.

## Implementation Notes

- Keep Cargo build metadata emission untouched unless the Bazel fix reveals a
  clear shared helper worth extracting.
- Prefer a focused Bun-based checker for Bazel runtime output over a large shell
  pipeline.
- Update operator-facing docs only where they clarify the build-system-specific
  provenance semantics that the status surface now exposes.

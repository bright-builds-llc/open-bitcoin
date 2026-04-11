---
phase: 01-workspace-baseline-and-guardrails
verified: 2026-04-11T12:12:37Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: interactive
phase_lifecycle_id: 01-2026-04-11T11-36-20
generated_at: 2026-04-11T12:12:37Z
lifecycle_validated: true
---

# Phase 1: Workspace, Baseline, and Guardrails Verification Report

**Phase Goal:** Establish the pinned Knots baseline, top-level workspace tooling, and the verification and architecture guardrails that all later implementation work will rely on.
**Verified:** 2026-04-11T12:12:37Z
**Status:** passed

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The repository contains a vendored, pinned Knots baseline under `packages/` and first-party workspace targets can run from the repo root. | ✓ VERIFIED | `git -C packages/bitcoin-knots describe --tags --exact-match` returns `v29.3.knots20260210`; `cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps` passes; `bazel build //:core //:node` passes |
| 2 | Verification fails when pure-core code imports forbidden I/O or runtime effects. | ✓ VERIFIED | `bash scripts/check-pure-core-deps.sh` passes and enforces both dependency and forbidden-import checks against `open-bitcoin-core` |
| 3 | Contributors have a repo-native verification entrypoint that covers format, lint, build, tests, coverage, and architecture-policy checks. | ✓ VERIFIED | `bash scripts/verify.sh` completes successfully and runs the architecture check, `cargo fmt`, `cargo clippy`, `cargo build`, `cargo test`, and `cargo llvm-cov` |
| 4 | The production path is fenced to first-party Rust Bitcoin crates rather than third-party Rust Bitcoin libraries. | ✓ VERIFIED | The first-party workspace contains only `open-bitcoin-core` and `open-bitcoin-node`; `open-bitcoin-node` depends only on `open-bitcoin-core`, and no third-party Rust Bitcoin library dependency is declared |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `.gitmodules` | Knots baseline submodule entry | ✓ EXISTS + SUBSTANTIVE | Registers `packages/bitcoin-knots` with the upstream Knots URL |
| `packages/Cargo.toml` | First-party Cargo workspace | ✓ EXISTS + SUBSTANTIVE | Defines the workspace members `open-bitcoin-core` and `open-bitcoin-node` |
| `MODULE.bazel` | Bzlmod root and `rules_rust` toolchain | ✓ EXISTS + SUBSTANTIVE | Pins `rules_rust` and the Rust 1.85.0 / edition 2024 toolchain |
| `scripts/verify.sh` | Repo-native verification command | ✓ EXISTS + SUBSTANTIVE | Runs architecture checks, Rust verification, and pure-core coverage |
| `docs/parity/index.json` | Initial parity/deviation ledger | ✓ EXISTS + SUBSTANTIVE | Seeds the baseline tag and all in-scope surfaces with `planned` status |

**Artifacts:** 5/5 verified

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `.gitmodules` | `packages/bitcoin-knots` | submodule path registration | ✓ WIRED | `git submodule status packages/bitcoin-knots` reports the pinned Knots gitlink |
| `BUILD.bazel` | `packages/open-bitcoin-core/BUILD.bazel` | `alias(name = "core")` | ✓ WIRED | `bazel query 'set(//:core //:node)'` resolves `//:core` successfully |
| `BUILD.bazel` | `packages/open-bitcoin-node/BUILD.bazel` | `alias(name = "node")` | ✓ WIRED | `bazel query 'set(//:core //:node)'` resolves `//:node` successfully |
| `scripts/verify.sh` | `scripts/check-pure-core-deps.sh` | direct shell invocation | ✓ WIRED | `scripts/verify.sh` calls `bash scripts/check-pure-core-deps.sh` before Cargo verification |
| `CONTRIBUTING.md` | `docs/parity/index.json` | contributor guidance | ✓ WIRED | `CONTRIBUTING.md` explicitly instructs contributors to update `docs/parity/index.json` for intentional deviations |

**Wiring:** 5/5 connections verified

## Requirements Coverage

| Requirement | Status | Blocking Issue |
|-------------|--------|----------------|
| `REF-01`: Contributors can build and test against a vendored Bitcoin Knots `29.3.knots20260210` baseline stored under `packages/`. | ✓ SATISFIED | - |
| `REF-02`: Contributors can inspect an explicit deviation ledger for any intentional behavior differences from the pinned baseline. | ✓ SATISFIED | - |
| `ARCH-01`: Contributors can build first-party packages from the repository root with Bazelisk and Bazel/Bzlmod. | ✓ SATISFIED | - |
| `ARCH-02`: Pure-core crates and modules reject direct filesystem, socket, wall-clock, environment, process, thread, async-runtime, and randomness dependencies. | ✓ SATISFIED | - |
| `ARCH-04`: The production implementation path uses first-party Rust Bitcoin libraries rather than third-party Rust Bitcoin libraries. | ✓ SATISFIED | - |
| `VER-01`: Contributors can run a repo-native verification flow that enforces formatting, linting, build, tests, and architecture-policy checks for changed paths. | ✓ SATISFIED | - |
| `VER-02`: CI fails when pure-core packages lose 100% unit-test coverage or leak forbidden I/O/runtime dependencies. | ✓ SATISFIED | - |

**Coverage:** 7/7 requirements satisfied

## Anti-Patterns Found

None.

## Human Verification Required

None — all verifiable items checked programmatically.

## Gaps Summary

**No gaps found.** Phase goal achieved. Ready to proceed.

## Verification Metadata

**Verification approach:** Goal-backward against the Phase 1 roadmap goal and success criteria  
**Must-haves source:** `01-01` through `01-04` PLAN frontmatter plus the Phase 1 roadmap success criteria  
**Lifecycle provenance:** validated  
**Automated checks:** 4 passed, 0 failed  
**Human checks required:** 0  
**Total verification time:** 1 min

---
*Verified: 2026-04-11T12:12:37Z*
*Verifier: the agent (inline fallback)*

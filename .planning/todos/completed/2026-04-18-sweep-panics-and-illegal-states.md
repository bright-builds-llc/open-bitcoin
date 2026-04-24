---
created: 2026-04-18T01:39:26.710Z
title: Sweep panics and illegal states
area: general
files:
  - packages/
---

## Problem

The codebase has grown quickly across the pure-core crates and adapters, which
raises the risk that some public or reusable paths still rely on panics,
`expect`, `unwrap`, unchecked assumptions, or state shapes that are only valid
by convention. Even when these are locally "safe", they can make the API harder
to compose, reduce robustness under edge cases, and weaken the project's goal
of being reference-grade, type-safe, and automation-friendly.

This is especially important for a Bitcoin node and wallet implementation,
where graceful failure, explicit invariants, and making illegal states
unrepresentable should be preferred over latent crash paths.

## Solution

Run a focused codebase-quality sweep aimed at panic prevention and stronger
domain modeling.

Approach hints:
- Search non-test code for `unwrap`, `expect`, `panic!`, `unreachable!`, array
  or slice indexing assumptions, and implicit `Option` or `Result` invariants.
- Prioritize public APIs, reusable pure-core helpers, and adapter boundaries
  where callers should receive typed errors instead of crashes.
- Refactor constructors and state models so invalid states cannot be created in
  the first place where practical.
- Replace convention-based invariants with typed state transitions, newtypes,
  enums, or validated builders when the added structure meaningfully reduces
  risk.
- Where a panic remains truly impossible or intentional, document why the state
  is unreachable and keep the justification narrow and explicit.

## Completion Review

Completed in Phase 11, `Panic and Illegal-State Hardening`.

- Replaced reachable first-party production panic-like sites in mempool,
  wallet, consensus, CLI, codec, networking, primitives, benchmark, and
  test-harness paths with typed errors or non-panicking control flow.
- Added `scripts/check-panic-sites.sh` and wired it into `scripts/verify.sh`.
- Kept `scripts/panic-sites.allowlist` empty at close; future production
  panic-like sites must be fixed or justified with a narrow local invariant.

Verification evidence:

- `bash scripts/check-panic-sites.sh`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`

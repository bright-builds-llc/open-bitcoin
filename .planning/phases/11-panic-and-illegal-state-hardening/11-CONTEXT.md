---
phase: "11-panic-and-illegal-state-hardening"
created: "2026-04-24"
source_todo: ".planning/todos/completed/2026-04-18-sweep-panics-and-illegal-states.md"
---

# Phase 11 Context: Panic and Illegal-State Hardening

## Intent

Reduce real crash risk in first-party Rust production code without mechanically removing every local invariant assertion.

## Scope

- Include `packages/open-bitcoin-*/src/**/*.rs`.
- Exclude `packages/bitcoin-knots`, `packages/target`, files named `tests.rs`, and inline code after `#[cfg(test)]`.
- Prioritize public APIs, reusable pure-core helpers, adapter boundaries, mempool state maintenance, chainstate apply/disconnect, wallet signing/building, RPC response construction, and taproot witness handling.

## Locked Decisions

- Keep externally observable Bitcoin, RPC, CLI, wallet, mempool, networking, and consensus behavior stable except replacing reachable crashes with existing-style typed failures.
- Prefer existing crate error enums and small validated helpers over generic string errors or new dependencies.
- Leave proven local invariants only when they are narrowly documented and covered by the regression guard allowlist.
- Wire the guard into `bash scripts/verify.sh` so CI/pre-commit catches new unclassified production panic-like sites.

## Starting Evidence

A production-only scan found about 55 panic-like sites after excluding test modules. The largest clusters were:

- `packages/open-bitcoin-mempool/src/pool.rs`
- `packages/open-bitcoin-consensus/src/block.rs`
- `packages/open-bitcoin-rpc/src/http.rs`
- `packages/open-bitcoin-consensus/src/script/taproot.rs`
- `packages/open-bitcoin-chainstate/src/engine.rs`

## Standards Inputs

- Local `AGENTS.md` repo guidance and `bash scripts/verify.sh` verification contract.
- `AGENTS.bright-builds.md` functional-core, illegal-state, and verification guidance.
- `standards-overrides.md` has no active local exception for this work.
- Bright Builds architecture, testing, verification, and Rust language standards.

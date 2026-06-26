---
phase: 91-peer-permissions-and-connection-classes
reviewed: 2026-06-25T21:18:14Z
depth: standard
files_reviewed: 49
files_reviewed_list:
  - docs/architecture/config-precedence.md
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator/support.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction.rs
  - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-network/src/inbound.rs
  - packages/open-bitcoin-network/src/inbound/permissions.rs
  - packages/open-bitcoin-network/src/inbound/permissions/error.rs
  - packages/open-bitcoin-network/src/inbound/tests.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer/inbound_state.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/inbound.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/status/inbound.rs
  - packages/open-bitcoin-node/src/status/inbound/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/inbound.rs
  - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
  - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
  - packages/open-bitcoin-rpc/src/config/tests.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - packages/open-bitcoin-rpc/src/http.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - scripts/check-phase91-peer-permissions.test.ts
  - scripts/check-phase91-peer-permissions.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 91: Code Review Report

**Reviewed:** 2026-06-25T21:18:14Z
**Depth:** standard
**Files Reviewed:** 49
**Status:** passed (clean)

## Summary

Reviewed the cumulative Phase 91 source scope from `origin/main..HEAD` plus the latest uncommitted local fixes in `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/inbound.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`, and `packages/open-bitcoin-rpc/src/inbound_listener.rs`.

The review focused on Phase 91 behavioral risks: typed permission parsing, literal-IP class matching, protected inbound reserved-slot use, inactive relay/mempool/filter safeguards, runtime listener admission, shared inbound status projection, support redaction, deterministic checker coverage, and the recent inbound rejection status evidence fixes.

No actionable issues remain. The prior stale `latest_permission_decision` risk is addressed by clearing `maybe_latest_permission_decision` on every recorded rejection. Runtime self-connection rejection evidence is now recorded before peer removal, and listener cleanup tolerates peers already removed by disconnect handling.

This review was materially informed by `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/core/operability.md`, `standards/core/local-guidance.md`, and `standards/languages/rust.md`.

## Findings

All reviewed files meet the Phase 91 quality, security, and behavioral boundary expectations. No Critical, Warning, or Info findings were found.

## Verification Notes

- `bun run scripts/check-phase91-peer-permissions.ts` - passed.
- `bun test scripts/check-phase91-peer-permissions.test.ts` - passed, 8 tests.
- `git diff --check -- packages/open-bitcoin-node/src/network.rs packages/open-bitcoin-node/src/network/inbound.rs packages/open-bitcoin-rpc/src/dispatch/tests.rs packages/open-bitcoin-rpc/src/inbound_listener.rs scripts/check-phase91-peer-permissions.ts scripts/check-phase91-peer-permissions.test.ts scripts/verify.sh` - passed.
- Focused Cargo lib tests for the two latest rejection-status regressions were attempted but blocked by an existing long-running `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` holding the artifact lock. The blocked retry was stopped without touching the unrelated build.

---

_Reviewed: 2026-06-25T21:18:14Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_

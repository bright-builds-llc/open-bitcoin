---
phase: 44-peer-contribution-attribution
reviewed: 2026-05-25T16:47:29Z
depth: standard
files_reviewed: 6
files_reviewed_list:
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/progress.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - scripts/run-live-mainnet-smoke.ts
  - scripts/test-run-live-mainnet-smoke.sh
  - docs/operator/runtime-guide.md
finding_counts:
  critical: 1
  warning: 3
  info: 2
  total: 6
findings:
  critical: 1
  warning: 3
  info: 2
  total: 6
status: issues_found
---

# Phase 44: Code Review Report

**Reviewed:** 2026-05-25T16:47:29Z
**Depth:** standard
**Files Reviewed:** 6
**Status:** issues_found

## Summary

Reviewed the listed Rust sync runtime files, sync tests, live-mainnet smoke runner, smoke runner shell regression, and operator runtime guide. This review was informed by `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and the pinned Bright Builds architecture, code-shape, testing, verification, Rust, and TypeScript/JavaScript standards.

The main risks are a shell-injection path in smoke-runner preflight command detection, stalled peers being counted as connected peers in sync summaries, millisecond retry settings being added to Unix-second timestamps, and smoke-runner build skipping when only one binary override is supplied.

## Critical Issues

### CR-01: Shell command interpolation allows environment override injection

**File:** `scripts/run-live-mainnet-smoke.ts:410`

**Issue:** `commandExists` builds a `sh -c` string with `command` interpolated inside double quotes. The command value can come from `OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN` or `OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN`, so shell metacharacters in those environment variables can execute during preflight instead of being treated as a binary path.

**Fix:**
```ts
function commandExists(command: string): boolean {
  try {
    execFileSync("sh", ["-c", "command -v \"$1\" >/dev/null 2>&1", "sh", command], {
      stdio: "ignore",
    });
    return true;
  } catch {
    return false;
  }
}
```

## Warnings

### WR-01: Stalled peers are counted as connected outbound peers

**File:** `packages/open-bitcoin-node/src/sync.rs:369`

**Issue:** `record_outcome` increments `summary.connected_peers` for every non-failed `PeerProgress`, which includes `PeerSyncState::Stalled`. A stalled session is then reported as a connected outbound peer and contributes to peer counts/resource pressure even though the runtime immediately disconnects it and marks backoff.

**Fix:** Count only healthy connected outcomes:
```rust
if progress.state == PeerSyncState::Connected {
    summary.connected_peers += 1;
}
```

### WR-02: Retry backoff milliseconds are added to Unix-second timestamps

**File:** `packages/open-bitcoin-node/src/sync.rs:217`

**Issue:** `sync_until_idle_with_resolver` advances `current_timestamp` by `retry_backoff_ms` even though the timestamp argument is Unix seconds. With the default `retry_backoff_ms = 1_000`, idle-loop timestamps jump 1,000 seconds instead of 1 second; the same unit conversion should be applied consistently anywhere retry backoff is converted into `next_attempt_unix_seconds`.

**Fix:** Convert milliseconds to seconds before adding to Unix-second timestamps:
```rust
let retry_backoff_seconds =
    i64::try_from(self.config.retry_backoff_ms.div_ceil(1_000)).unwrap_or(i64::MAX);
current_timestamp = current_timestamp.saturating_add(retry_backoff_seconds.max(1));
```

### WR-03: Partial binary overrides skip required builds

**File:** `scripts/run-live-mainnet-smoke.ts:821`

**Issue:** `ensureBuiltBinaries` returns when either the daemon override or status override is set. If an operator overrides only one binary, the other default binary is not built before the runner tries to execute it from `packages/target/debug`, producing stale-binary or missing-file failures.

**Fix:** Skip the build only when both binaries are overridden, or check/build each missing default independently:
```ts
if (
  process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN !== undefined &&
  process.env.OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN !== undefined
) {
  return;
}
```

## Info

### IN-01: Numeric parsing accepts trailing junk

**File:** `scripts/run-live-mainnet-smoke.ts:343`

**Issue:** `parsePort` and `parsePositiveInteger` use `Number.parseInt`, so values like `8333abc` or `10s` are accepted as valid numbers. That can make operator input look valid while silently changing its meaning.

**Fix:** Require a full decimal match before converting:
```ts
if (!/^[0-9]+$/.test(value)) {
  throw new Error(`${label} must be a positive integer`);
}
const parsed = Number(value);
```

### IN-02: Smoke runner has grown past a maintainability threshold

**File:** `scripts/run-live-mainnet-smoke.ts:1`

**Issue:** The smoke runner is 1,683 lines and now mixes CLI parsing, config generation, JSONC parsing, network probing, process supervision, status decoding, failure classification, and report rendering. This is above the Bright Builds file-size refactor trigger and makes future review of operator-facing behavior harder.

**Fix:** Split into focused modules such as `live-smoke/options.ts`, `live-smoke/preflight.ts`, `live-smoke/runtime.ts`, `live-smoke/report.ts`, and `live-smoke/classification.ts`, keeping `run-live-mainnet-smoke.ts` as a thin orchestration entrypoint.

---

_Reviewed: 2026-05-25T16:47:29Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_

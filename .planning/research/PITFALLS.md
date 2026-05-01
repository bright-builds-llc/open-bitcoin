# Pitfalls Research

> Historical note: this is pre-v1.1 pitfalls research from 2026-04-26. Keep it
> as planning history; use current parity, architecture, and operator docs for
> shipped behavior.

**Domain:** Bitcoin node operator runtime and migration
**Researched:** 2026-04-26
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Dashboard Before Truth

**What goes wrong:**
The TUI looks polished but is backed by incomplete, stale, or synthetic status data.

**How to avoid:**
Build the status snapshot, metrics history, and sync-state model before the Ratatui phase.

**Warning signs:**
Dashboard tests assert strings or layout only, with no status-model tests.

**Phase to address:**
Phase 13 and Phase 16.

### Pitfall 2: Unsafe Migration

**What goes wrong:**
The wizard detects an existing Core/Knots install and mutates data before the user understands the tradeoffs.

**How to avoid:**
Make migration dry-run-first, backup-aware, and explicit. Treat write operations as a separate approved plan.

**Warning signs:**
Detection and apply logic share the same function, or tests need real user datadirs.

**Phase to address:**
Phase 17 and Phase 21.

### Pitfall 3: Storage Choice Without Recovery Semantics

**What goes wrong:**
A database adapter lands, but restart, partial-write, corruption, reindex, and schema-upgrade behavior are undefined.

**How to avoid:**
Define storage contracts, schema versioning, fsync/flush expectations, recovery tests, and reindex behavior before live sync relies on storage.

**Warning signs:**
Storage tests only round-trip one happy-path snapshot.

**Phase to address:**
Phase 13 and Phase 14.

### Pitfall 4: Live Network Sync That Is Not Bounded In Tests

**What goes wrong:**
Tests depend on public network availability, are slow, or become nondeterministic.

**How to avoid:**
Separate deterministic simulated-network tests, controlled local process tests, and optional live-network smoke benchmarks.

**Warning signs:**
Default `bash scripts/verify.sh` waits on external peers or requires internet availability.

**Phase to address:**
Phase 15 and Phase 22.

### Pitfall 5: Service Install Without Inspectability

**What goes wrong:**
The daemon can be installed but operators cannot tell what unit/plist was written, why startup failed, or how to disable it.

**How to avoid:**
Each service command must support dry-run or plan output, and `status` must show service manager state and relevant logs.

**Warning signs:**
Install command writes files without a matching status, disable, uninstall, and test fixture path.

**Phase to address:**
Phase 18.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Manual CLI parsing for every new command | Avoids parser refactor | Inconsistent help, errors, and global flags | Only for baseline-compatible pass-through tokens that clap cannot model cleanly. |
| Store Open Bitcoin-only state in `bitcoin.conf` | One config file | Breaks Core/Knots compatibility and migration safety | Never for wizard/TUI/service-only state. |
| Use public network tests as default verification | Looks like real coverage | Flaky local and CI verification | Optional benchmark or smoke job only. |
| Treat service install as shell snippets | Fast first implementation | Hard to test, quote, dry-run, and support across OSes | Only as generated artifacts behind typed commands. |

## "Looks Done But Isn't" Checklist

- [ ] **Status:** Works when daemon is stopped, not only over live RPC.
- [ ] **Dashboard:** Uses real status and metrics snapshots, not local placeholder counters.
- [ ] **Storage:** Survives restart and has tested partial-write/recovery behavior.
- [ ] **Sync:** Handles peer disconnects, restart, and partial progress.
- [ ] **Service lifecycle:** Has dry-run, install, enable, disable, uninstall, and status.
- [ ] **Migration:** Detects, explains, dry-runs, backs up, and requires explicit apply.
- [ ] **Wallet:** Does not conflate Open Bitcoin wallet state with Core/Knots wallet files without an audit trail.

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Dashboard before truth | Phase 13, Phase 16 | Status model tests and metrics-history tests exist before TUI rendering tests. |
| Unsafe migration | Phase 17, Phase 21 | Dry-run plans are snapshot-tested and no write occurs during detection. |
| Weak storage recovery | Phase 14 | Restart, partial-write, and reindex tests cover the chosen adapter. |
| Flaky live sync tests | Phase 15, Phase 22 | Default verify remains hermetic; live sync is opt-in and benchmark-scoped. |
| Opaque service install | Phase 18 | Generated plist/unit files are testable and surfaced by `status`. |

## Sources

- v1.0 deferred surfaces in `docs/parity/release-readiness.md`.
- Existing in-memory store limitations in `packages/open-bitcoin-node/src/chainstate.rs` and `packages/open-bitcoin-node/src/wallet.rs`.
- User v1.1 milestone goals, 2026-04-26.

---
*Pitfalls research for: Open Bitcoin v1.1*
*Researched: 2026-04-26*

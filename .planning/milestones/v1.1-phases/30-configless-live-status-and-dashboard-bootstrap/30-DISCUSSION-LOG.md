# Phase 30: Configless Live Status and Dashboard Bootstrap - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in `30-CONTEXT.md` - this log preserves the
> alternatives considered during the yolo discuss pass.

**Date:** 2026-04-29T16:19:20.795Z
**Phase:** 30-configless-live-status-and-dashboard-bootstrap
**Mode:** Yolo
**Areas discussed:** Live RPC bootstrap, Shared status/dashboard path, Fallback
semantics, Verification and docs

---

## Live RPC Bootstrap

| Option | Description | Selected |
|--------|-------------|----------|
| Require an on-disk `bitcoin.conf` | Keep the current behavior and treat a missing implicit config file as "no live RPC bootstrap available". | |
| Derive startup from resolved datadir, network, cookies, and defaults | Reuse the existing startup/config loader path, but only pass `-conf` when the resolved `bitcoin.conf` actually exists. | ✓ |
| Create a new status/dashboard-only RPC parser | Build a separate bootstrap path for operator surfaces instead of reusing the existing startup contract. | |

**User's choice:** Auto-selected the shared datadir/network/default bootstrap.
**Notes:** This is the narrowest fix that closes `INT-v1.1-02` while preserving
the existing precedence model and reusing `resolve_startup_config()`.

---

## Shared status/dashboard path

| Option | Description | Selected |
|--------|-------------|----------|
| Keep one shared bootstrap through `status_runtime_parts()` | `status` and `dashboard` continue to use the same runtime wiring and shared snapshot collector. | ✓ |
| Add a dashboard-only bootstrap path | Repair the dashboard separately from `status`. | |
| Add a status-only bootstrap path | Repair `status` first and defer dashboard alignment. | |

**User's choice:** Auto-selected the shared bootstrap path.
**Notes:** The dashboard already reuses the shared status snapshot, so a forked
bootstrap would create a new truth-drift risk immediately.

---

## Fallback semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Missing implicit config means "stopped" | Continue collapsing the snapshot when `bitcoin.conf` is absent. | |
| Missing implicit config still attempts live RPC; real bootstrap failures stay explicit | Treat a missing implicit config file as normal and keep stopped/unreachable fallback only for genuine auth, cookie, or connectivity failures. | ✓ |
| Expand scope to new operator RPC override flags | Solve the problem by adding new operator-facing RPC config inputs in this phase. | |

**User's choice:** Auto-selected the narrow bootstrap repair plus existing
fallback semantics.
**Notes:** This preserves the truthful `Unavailable` model from prior phases and
avoids scope creep into new CLI surfaces.

---

## Verification and docs

| Option | Description | Selected |
|--------|-------------|----------|
| Code-only fix | Repair the bootstrap code without updating tests or operator docs. | |
| Focused regression tests plus runtime-guide alignment | Add targeted regression coverage for the missing-implicit-config workflow and keep the documented flag-only workflow truthful. | ✓ |
| Full end-to-end operator harness expansion | Build a broader operator integration harness in addition to the bug fix. | |

**User's choice:** Auto-selected focused regression tests and minimal doc
alignment.
**Notes:** This matches the phase goal and the repo's preference for focused,
hermetic verification rather than broad new harness work.

---

## Claude's Discretion

- Exact helper names and whether the shared bootstrap conversion stays inside
  `operator/runtime.rs` or moves to a nearby helper.
- The precise mix of runtime-level versus status-level tests, as long as the
  documented flag-only workflow and the stopped-node counterexample are both
  covered.

## Deferred Ideas

- Explicit operator RPC override flags remain a separate capability.
- A deeper refactor that persists fully materialized runtime RPC config inside
  `OperatorConfigResolution` is not required for this phase.

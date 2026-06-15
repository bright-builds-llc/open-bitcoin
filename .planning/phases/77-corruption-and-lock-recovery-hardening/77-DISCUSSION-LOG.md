# Phase 77: Corruption and Lock Recovery Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered.

**Date:** 2026-06-15T18:39:45.451Z
**Phase:** 77-Corruption and Lock Recovery Hardening
**Mode:** Yolo
**Areas discussed:** Lock contention and stale-lock evidence, Recovery action taxonomy, Operator evidence surfaces, Deterministic recovery fixtures

---

## Lock Contention And Stale-Lock Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Backend-open classification | Use Fjall/backend open errors and current `StorageLockContention` mapping. | |
| Read-only lock probe plus typed evidence | Probe without hidden mutation and distinguish held lock, stale lock artifact, and concurrent datadir evidence. | yes |
| Owner heartbeat/sentinel metadata | Add mutable owner metadata such as PID/start/version evidence. | |
| OS process scan | Use platform process inspection such as `lsof` as evidence. | |

**User's choice:** Auto-selected read-only lock probe plus typed evidence.
**Notes:** This satisfies the no-hidden-mutation constraint and keeps backend
open errors as a separate adapter path. Concurrent datadir use can combine lock
evidence with existing service/RPC/same-datadir status evidence. Owner
sentinels and process scans were deferred because they add mutation,
portability, cleanup, and redaction concerns.

---

## Recovery Action Taxonomy

| Option | Description | Selected |
|--------|-------------|----------|
| Stable category plus typed recovery evidence | Preserve `SyncRecoveryCategory` labels and add explicit cause/action-class evidence. | yes |
| Expand `SyncRecoveryCategory` per root cause | Add categories for every stale-lock, schema, corruption, partial-write, and open-failure case. | |
| Reuse current categories/actions with wording changes | Keep current fields and only harden prose. | |
| New primary `StorageRecoveryCase` with legacy projection | Make a richer case object primary and derive legacy fields from it. | |

**User's choice:** Auto-selected stable category plus typed recovery evidence.
**Notes:** Existing category labels are already consumed across status,
dashboard, support evidence, soak reports, live-smoke reports, and docs. The
selected approach avoids contract churn while adding the REC-07 action classes:
safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate.

---

## Operator Evidence Surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Top-level `recovery_evidence` on `OpenBitcoinStatusSnapshot` | Use one shared status field for stopped-node, lock, open-failure, support, soak, and renderer surfaces. | yes |
| `sync.recovery_evidence` beside existing sync fields | Keep richer evidence inside `sync`. | |
| Renderer/support/soak derived summaries | Derive summaries independently from existing fields. | |
| Support/soak artifact-first recovery ledger | Put richer recovery evidence mainly in post-run artifacts. | |

**User's choice:** Auto-selected top-level `recovery_evidence` on
`OpenBitcoinStatusSnapshot`.
**Notes:** Phase 77 evidence is datadir/store evidence and can exist before
durable sync state is available. A top-level status field best preserves the
repo's existing source-of-truth rule and avoids renderer-local string parsing.
Existing `sync.recovery_category` and `sync.recovery_action` stay as
compatibility summaries.

---

## Deterministic Recovery Fixtures

| Option | Description | Selected |
|--------|-------------|----------|
| Hybrid real-Fjall temp fixtures plus lock-holder subprocess | Prove real adapter behavior with small temp datadirs and a narrow lock holder for lock contention. | yes |
| Test-only storage opener/failure seam | Prove classifier behavior through deterministic injected failures. | |
| Expand existing Bun/live-smoke fixtures | Validate operator-facing report and wording contracts only. | |
| OS fault simulation | Use filesystem permissions, full disk, or service-like orchestration. | |

**User's choice:** Auto-selected hybrid real-Fjall temp fixtures plus
lock-holder subprocess.
**Notes:** Real Fjall temp fixtures are the primary acceptance evidence for
schema mismatch, corruption markers, recovery markers, partial writes, path
open failures, and lock contention. A test-only seam is allowed only if a
specific OS/backend behavior is not deterministic enough for default
verification. Bun checkers remain supplemental for docs and artifact contracts.

---

## the agent's Discretion

- The planner may choose exact type names, field names, and plan boundaries as
  long as the resulting implementation preserves stable categories, exposes
  typed recovery evidence from one shared contract, and avoids hidden mutation.
- The executor may decide whether a lock-holder subprocess is needed or whether
  a narrower in-process fixture can prove lock contention reliably.

## Deferred Ideas

- Owner heartbeat or PID sentinel metadata.
- OS process scans such as `lsof`.
- Separate forensic recovery ledger outside the shared status contract.
- Automatic destructive repair, hidden lock cleanup, hidden reindex, hidden
  datadir relocation, or source datadir mutation.

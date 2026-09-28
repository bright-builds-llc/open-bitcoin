# Phase 150: Operator Prune Surfaces and Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-28
**Phase:** 150-operator-prune-surfaces-and-evidence
**Mode:** Yolo
**Lifecycle:** 150-2026-09-28T17-08-23
**Areas discussed:** Prune status fields, Config integer, Manual prune request, Prune locks, Sanitized support evidence

This `--yolo --chain` run reaffirmed the same recommended answers already captured for Phase 150. No decision changed. The lifecycle id was refreshed so planning and execution share this attempt.

---

## Prune status fields

| Option | Description | Selected |
|--------|-------------|----------|
| Knots getblockchaininfo quartet, plus status, CLI, and dashboard | `pruned`, `pruneheight`, `automatic_pruning`, `prune_target_size`, kept distinct from the earned block label | ✓ |
| Earned have-pruned boolean only | Report `pruned: true` only after a delete | |
| Dashboard-only summary | Skip RPC shape | |

**User's choice:** Knots quartet on RPC, status, CLI, and dashboard (recommended default)
**Notes:** `pruned` means mode is on. Phase 149 `pruned_count` stays the earned label.

## Config integer

| Option | Description | Selected |
|--------|-------------|----------|
| JSONC `prune` via existing `parse_prune_arg` | `0` / `1` / `>=550`, default disabled | ✓ |
| Leave mode test-only | Status would always read disabled | |
| New byte-target field | Second constructor beside the MiB integer | |

**User's choice:** JSONC `prune` via `parse_prune_arg` (recommended default)
**Notes:** Phase 147 left this wiring for Phase 150.

## Manual prune request

| Option | Description | Selected |
|--------|-------------|----------|
| RPC `pruneblockchain` and CLI, dashboard observes | Refusal or height, no dashboard delete control | ✓ |
| Dashboard button plus RPC | Destructive control on the terminal UI | |
| CLI only | Skip the Knots RPC name | |

**User's choice:** RPC and CLI request; dashboard observes (recommended default)
**Notes:** Disabled and keep-window targets refuse without clamping or deletes.

## Prune locks

| Option | Description | Selected |
|--------|-------------|----------|
| Durable named list, set, and clear on RPC and CLI | Reuse `PruneLockInfo`; dashboard lists only | ✓ |
| In-memory locks | Lost on restart | |
| Dashboard editor | Lock edits from the terminal UI | |

**User's choice:** Durable named list, set, and clear (recommended default)
**Notes:** Clear-by-name is the unset half of set. Buffer formula stays Phase 147.

## Sanitized support evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Counts plus last prune height, no paths | Batch count, height count, last deleted height | ✓ |
| Full datadir listing | Conflicts with OPER-03 | |
| Mode flag only | Omits the required counts and height | |

**User's choice:** Counts plus last prune height, no paths (recommended default)
**Notes:** Zeros and absent height until a durable delete succeeds.

## Claude's Discretion

- Dashboard section layout
- CLI subcommand spelling
- Storage placement of last-prune counters
- JSONC file placement for `prune`

## Deferred Ideas

- Phase 151 parity docs and no-claim checkers
- Dashboard control that deletes payloads
- Archive, assumeutxo, BIP37, compact filters, public defaults, and prune-during-init

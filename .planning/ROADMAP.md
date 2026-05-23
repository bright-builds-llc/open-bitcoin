# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** — Phases 1 through 12 (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** — Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** — Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)

## Current Focus

No active milestone is currently planned. The next milestone should start with
`/gsd-new-milestone` so requirements, roadmap phases, and scope boundaries are
defined from a fresh discussion.

## Completed Milestones

<details>
<summary>✅ v1.2 Full Mainnet Network Syncing (Phases 35-41) — SHIPPED 2026-05-23</summary>

- [x] **Phase 35: Daemon Mainnet Sync Activation** — Explicit opt-in daemon mainnet sync activation and preflight.
- [x] **Phase 36: Mainnet Peer Discovery and Outbound Lifecycle** — DNS/manual peer resolution, bounded outbound peer lifecycle, rotation, and telemetry.
- [x] **Phase 37: Header-First Mainnet Sync Integration** — Durable validated header synchronization and restart recovery.
- [x] **Phase 38: Block Download, Connect, and Restart Recovery** — Bounded block download, validation, connection, reorg-aware state, and restart recovery.
- [x] **Phase 39: Operator Sync Observability and Control** — Truthful sync status, dashboard, metrics, logs, RPC surfaces, and pause/resume control.
- [x] **Phase 40: Live Mainnet Smoke, Docs, and Parity Closeout** — Opt-in live-mainnet smoke reporting and shipped-claim documentation.
- [x] **Phase 41: Security Analysis Audit and Follow-Up** — Security-analysis closeout with `threats_open: 0` and no new security phase required.

Detailed phase execution history is archived under
[milestones/v1.2-phases/](milestones/v1.2-phases/).

</details>

## Progress

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22/22 | 80/80 | Archived | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22/22 | 69/69 | Archived | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 7/7 | 13/13 | Archived | 2026-05-23 |

## Next Step

Start the next milestone:

```bash
/gsd-new-milestone
```

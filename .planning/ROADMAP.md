# Roadmap: Open Bitcoin

## Current Status: Ready For Next Milestone

v2.0 Transaction Relay and Mempool Participation Boundary shipped on 2026-07-03. The milestone archived the full roadmap, requirements, and audit evidence under `.planning/milestones/` and left no active requirements file.

Start the next milestone with:

```bash
/gsd-new-milestone
```

## Latest Completed Milestone: v2.0 Transaction Relay and Mempool Participation Boundary

**Delivered:** Bounded transaction relay and mempool participation through explicit activation, permission-aware txid/wtxid download, orphan and admission outcomes, durable mempool recovery, relay serving/fanout, sanitized operator evidence, and deterministic no-claim guardrails.

**Boundary:** v2.0 does not claim compact block relay, bloom/filter serving, broad package relay, public transaction relay by default, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet safety.

**Phases completed:** Phases 100 through 109.

**Archive:**

- [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)
- [v2.0-REQUIREMENTS.md](milestones/v2.0-REQUIREMENTS.md)
- [v2.0-MILESTONE-AUDIT.md](milestones/v2.0-MILESTONE-AUDIT.md)

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** - Phases 68 through 74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Full-Sync Soak and Recovery Hardening** - Phases 75 through 81 (shipped 2026-06-20). Archive: [v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Production Full-Node Readiness Boundary** - Phases 82 through 89 (shipped 2026-06-25). Archive: [v1.8-ROADMAP.md](milestones/v1.8-ROADMAP.md)
- ✅ **v1.9 Inbound Peer Serving and Network Participation Boundary** - Phases 90 through 99 (shipped 2026-06-29). Archive: [v1.9-ROADMAP.md](milestones/v1.9-ROADMAP.md)
- ✅ **v2.0 Transaction Relay and Mempool Participation Boundary** - Phases 100 through 109 (shipped 2026-07-03). Archive: [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)

## Milestone History

| Milestone | Phases | Plans | Status | Shipped | Archive |
| --- | ---: | ---: | --- | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 | [roadmap](milestones/v1.0-ROADMAP.md) |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 | [roadmap](milestones/v1.1-ROADMAP.md) |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 | [roadmap](milestones/v1.2-ROADMAP.md) |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 | [roadmap](milestones/v1.3-ROADMAP.md) |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 15 | Shipped | 2026-06-05 | [roadmap](milestones/v1.4-ROADMAP.md) |
| v1.5 Unattended Mainnet Node Operation Readiness | 8 | 22 | Shipped | 2026-06-10 | [roadmap](milestones/v1.5-ROADMAP.md) |
| v1.6 Mainnet Full-Sync Completion | 7 | 27 | Shipped | 2026-06-14 | [roadmap](milestones/v1.6-ROADMAP.md) |
| v1.7 Full-Sync Soak and Recovery Hardening | 7 | 37 | Shipped | 2026-06-20 | [roadmap](milestones/v1.7-ROADMAP.md) |
| v1.8 Production Full-Node Readiness Boundary | 8 | 26 | Shipped | 2026-06-25 | [roadmap](milestones/v1.8-ROADMAP.md) |
| v1.9 Inbound Peer Serving and Network Participation Boundary | 10 | 56 | Shipped | 2026-06-29 | [roadmap](milestones/v1.9-ROADMAP.md) |
| v2.0 Transaction Relay and Mempool Participation Boundary | 10 | 36 | Shipped | 2026-07-03 | [roadmap](milestones/v2.0-ROADMAP.md) |

## Traceability

- Latest requirements archive: [v2.0-REQUIREMENTS.md](milestones/v2.0-REQUIREMENTS.md)
- Latest milestone audit: [v2.0-MILESTONE-AUDIT.md](milestones/v2.0-MILESTONE-AUDIT.md)
- v2.0 research summary: [SUMMARY.md](research/SUMMARY.md)
- Active requirements: none. A new `.planning/REQUIREMENTS.md` is created by `/gsd-new-milestone`.

## Next Step

Define the next milestone with `/gsd-new-milestone`.

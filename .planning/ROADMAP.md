# Roadmap: Open Bitcoin

## Current Status

v2.3 Chainstate Durability and Historical Serving shipped and was archived on 2026-09-20. No milestone is currently active; start the next milestone with `/gsd-new-milestone`.

## Latest Completed Milestone: v2.3 Chainstate Durability and Historical Serving

**Delivered:** Disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.

**Boundary:** v2.3 is storage-first. It does not add prune or archive product modes, assumeutxo / assumevalid / IBD snapshot shortcuts, compact-filter or BIP37 serving, LevelDB or rust-bitcoin, public serving or relay defaults, public-network CI as a release gate, production full-node readiness, or production-funds wallet claims. Functional-core crates stay I/O-free. Historical `.planning/phases/` directories stay tracked.

**Phases completed:** Phases 139 through 145 (29 plans).

**Archive:**

- [v2.3-ROADMAP.md](milestones/v2.3-ROADMAP.md)
- [v2.3-REQUIREMENTS.md](milestones/v2.3-REQUIREMENTS.md)
- [v2.3-MILESTONE-AUDIT.md](milestones/v2.3-MILESTONE-AUDIT.md)

## Milestones

- ✅ **v1.0 Headless Parity** — 22 phase entries, including inserted closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** — Phases 13–34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** — Phases 35–41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** — Phases 42–53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** — Phases 54–59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** — Phases 60–67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** — Phases 68–74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Full-Sync Soak and Recovery Hardening** — Phases 75–81 (shipped 2026-06-20). Archive: [v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Production Full-Node Readiness Boundary** — Phases 82–89 (shipped 2026-06-25). Archive: [v1.8-ROADMAP.md](milestones/v1.8-ROADMAP.md)
- ✅ **v1.9 Inbound Peer Serving and Network Participation Boundary** — Phases 90–99 (shipped 2026-06-29). Archive: [v1.9-ROADMAP.md](milestones/v1.9-ROADMAP.md)
- ✅ **v2.0 Transaction Relay and Mempool Participation Boundary** — Phases 100–109 (shipped 2026-07-03). Archive: [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)
- ✅ **v2.1 Block Serving and Compact Block Relay Boundary** — Phases 110–129 (shipped 2026-07-22). Archive: [v2.1-ROADMAP.md](milestones/v2.1-ROADMAP.md)
- ✅ **v2.2 Package Relay and Long-Lived Mempool Policy** — Phases 130–138, including inserted 133.1 (shipped 2026-08-22). Archive: [v2.2-ROADMAP.md](milestones/v2.2-ROADMAP.md)
- ✅ **v2.3 Chainstate Durability and Historical Serving** — Phases 139–145 (shipped 2026-09-20). Archive: [v2.3-ROADMAP.md](milestones/v2.3-ROADMAP.md)

## Next Step

Run `/gsd-new-milestone` to define fresh requirements and a roadmap for the next version.

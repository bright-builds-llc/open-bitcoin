---
phase: 40
phase_name: "Live Mainnet Smoke, Docs, and Parity Closeout"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "40-2026-05-02T13-22-45"
generated_at: "2026-05-02T13:22:45.481Z"
---

# Discussion Log 40

## Yolo Decisions

- Live mainnet validation should stay explicitly opt-in and outside `bash scripts/verify.sh`.
- The supported evidence path should reuse repo-owned daemon and operator surfaces, not require direct store inspection or ad hoc reviewer commands.
- Live smoke or benchmark flows should fail early with clear prerequisite guidance for datadir, config, disk, network, and runtime safety issues.
- Generated evidence should remain local JSON or Markdown style artifacts with rerunnable provenance, not checked-in timing gates.
- README, operator runtime docs, and parity-ledger surfaces should be updated together so v1.2 shipped claims and known non-claims stay aligned.

## Deferred

- Any change that makes public-network validation part of the default hermetic verify flow.
- Production-node or production-funds claims beyond the bounded v1.2 operator-ready mainnet sync workflow.
- Timing-threshold benchmark enforcement or checked-in live-network artifacts.

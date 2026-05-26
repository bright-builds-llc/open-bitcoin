---
generated_by: gsd-plan-phase
phase: 45
phase_name: Runtime Resource Bounds and Store Coordination
lifecycle_mode: yolo
phase_lifecycle_id: "45-2026-05-26T16-41-34"
generated_at: "2026-05-26T16:41:34Z"
status: complete
requirements:
  - NODE-01
  - NODE-04
---

# Phase 45 Research

## Findings

### R-01: Sync already has bounded runtime knobs

`SyncRuntimeConfig` already carries the main bounded execution controls: outbound target, peer read message cap, sync rounds, peer retries, per-peer in-flight blocks, and total in-flight blocks. The runtime validates block caps and request reconciliation uses those caps before asking peers for blocks.

### R-02: Status pressure is narrower than runtime config

`SyncResourcePressure` currently reports observed `blocks_in_flight`, global max in-flight blocks, observed outbound peers, and target outbound peers. It does not report the per-peer block cap, message cap, or sync round cap that define the operator-visible resource envelope.

### R-03: Metrics and logs already have bounded retention

Metric retention defaults to a bounded sample count and age. Structured logs use bounded file count, age, and total bytes. Phase 45 should link those existing retention policies into operator docs rather than duplicating retention constants in sync code.

### R-04: Config only exposes a subset of sync limits

Open Bitcoin JSONC config currently exposes `manual_peers`, `dns_seeds`, and `target_outbound_peers` for sync. The runtime caps exist in code, but operators cannot set or inspect most of them through config/status.

### R-05: Offline mutating control can be unsafe when live RPC is unavailable

The CLI tries live RPC first, then falls back to opening the durable store directly for `status`, `pause`, and `resume`. Direct offline mutation is useful when no daemon is active, but it should not proceed when durable metadata shows an unclean, active sync lifecycle that may still be owned by a daemon.

## Recommendation

Implement Phase 45 as a focused hardening pass:

1. Add resource-bound fields to shared sync status projection.
2. Add validated JSONC config overrides for existing sync runtime caps.
3. Add an offline mutating-control guard that emits an explicit second-writer diagnostic.
4. Update operator docs and deterministic tests.

This keeps the implementation inside existing status/config/control seams and avoids changing Bitcoin protocol behavior.

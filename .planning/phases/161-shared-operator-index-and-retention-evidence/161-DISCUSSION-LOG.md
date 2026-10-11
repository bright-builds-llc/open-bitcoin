# Phase 161: Shared Operator Index and Retention Evidence - Discussion Log

Audit trail only. Downstream planning/research/execution consumes CONTEXT.md.

**Date:** 2026-10-10 CDT
**Mode:** Yolo — recommended answers auto-selected by the explicitly invoked workflow.
**Areas:** shared contract, failures/privacy, retention growth/protection, terminal consumers/evidence.
**Lifecycle:** `161-2026-10-11T04-48-29`

## Shared contract

| Option | Effect | Selected |
| --- | --- | --- |
| Extend existing authority/snapshot | All consumers share explicit configured/progress/durable facts | Yes |
| Each consumer infers readiness | More duplicated logic and disagreement across lag/reorg cases | No |
| Extend pinned getindexinfo shape | Changes the baseline external RPC contract | No |

Recommended: D-01–04, using existing owner/FieldAvailability and additive defaults. Distinguish initial-sync latch, current lag and safe durability. Prior Phases 144/157/159/160 already lock these semantics.

## Failures and privacy

| Option | Effect | Selected |
| --- | --- | --- |
| Fixed categories with independent retained availability | Actionable evidence without private backend detail | Yes |
| One healthy/failed flag | Hides missing body/undo, resource and integrity distinctions | No |
| Raw error text | Risks paths, scripts and peer/storage identifiers | No |

Recommended: D-05–08. One read-only authority capture; support agrees with captured live facts and preserves caller data. Retained availability must not promise arbitrary queries succeed.

## Retention growth and protection

| Option | Effect | Selected |
| --- | --- | --- |
| Bounded authoritative logical accounting separate from payload target | Honest growth including retained stale rows and protection bounds | Yes |
| Scan full index each status poll | Unbounded polling work and allocations | No |
| Charge filters to block/undo prune target | Changes the shipped retention contract | No |

Recommended: D-09–12. Research determines the simplest recoverable accounting seam, with retry/reorg/reopen/fault controls. Incomplete totals remain unavailable; no physical-space or restored-body/wallet claim.

## Terminal consumers and verification

| Option | Effect | Selected |
| --- | --- | --- |
| Compact existing label/value sections | Fits terminal layout and shared projection | Yes |
| New dashboard screen/controls or hosted frontend | Broadens the phase and product | No |
| Only synthetic snapshot tests | Cannot prove authority wiring or measurement freshness | No |

Recommended: D-13–16. Terminal UI contract, genuine Fjall/validated-chain consumer proof, redaction, scoped parity/native guards, independent review and full native/lifecycle gates. Phase 162 remains pending.

## Research and prior decisions

Targeted primary-source background consulted BIP157 and Bitcoin index documentation; local pinned Knots source defines exact behavior. Read-only scouts identified existing owner/status/consumer seams and reviewed relevant prior context decisions. No matched pending todos. No user questions were required in yolo mode.

## Deferred ideas

Complete integrated retained-client proof remains Phase 162. BIP37/V0, filter import, wallet acceleration, automatic repair/download, archive-scale and public/default/production/funds claims remain excluded.

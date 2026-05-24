# Phase 42: Live Smoke Entry and Network Preflight - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-24T13:40:48.450Z
**Phase:** 42-live-smoke-entry-and-network-preflight
**Mode:** Recommended Review
**Areas discussed:** Invocation Contract, Preflight Coverage, No-Progress Diagnosis, Evidence Output, Verification Posture

---

## Invocation Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing live-smoke script | Keep `scripts/run-live-mainnet-smoke.ts` as the operator entrypoint and evolve its options/reporting. | yes |
| Add separate preflight CLI | Create a new command or binary dedicated to network preflight. | no |
| Move smoke flow into `open-bitcoin` now | Convert the script into a first-party operator subcommand during Phase 42. | no |

**Recommended choice:** Extend existing live-smoke script.
**Rationale:** Phase 42 is about making the existing review flow truthful and diagnosable. A new CLI surface would expand scope and duplicate current report behavior.

---

## Preflight Coverage

| Option | Description | Selected |
|--------|-------------|----------|
| Local plus endpoint outcomes | Keep datadir/config/clock/disk/command checks and add DNS/manual-peer resolution, connection, handshake, failure, and skipped buckets. | yes |
| Local prerequisites only | Keep the existing local checks and leave endpoint diagnosis to daemon logs. | no |
| Full sync proof preflight | Treat preflight as a mini sync run that must prove header or block progress. | no |

**Recommended choice:** Local plus endpoint outcomes.
**Rationale:** The last UAT passed local preflight but still produced `0 outbound peers`. Phase 42 should close that diagnostic gap without taking over later sync-progress evidence phases.

---

## No-Progress Diagnosis

| Option | Description | Selected |
|--------|-------------|----------|
| Typed no-progress causes | Classify DNS, TCP connect, handshake, unsupported capability, validation, storage, timeout, and cancellation when evidence exists. | yes |
| Generic timeout | Keep the current broad no-progress message and rely on operator interpretation. | no |
| Treat no progress as runtime failure | Fail every no-progress run as daemon/runtime failure. | no |

**Recommended choice:** Typed no-progress causes.
**Rationale:** Phase 42 exists because `0 outbound peers` is too coarse. The report should tell the operator the next concrete action.

---

## Evidence Output

| Option | Description | Selected |
|--------|-------------|----------|
| Extend JSON and Markdown reports in place | Keep local reports under `packages/target/live-mainnet-smoke-reports` and make JSON authoritative. | yes |
| Add checked-in fixtures from live runs | Store public-network report examples in git. | no |
| Add elapsed-time release gates | Pass or fail release readiness based on timing thresholds. | no |

**Recommended choice:** Extend JSON and Markdown reports in place.
**Rationale:** Existing parity docs treat live-smoke reports as local evidence. Phase 42 should strengthen that evidence without changing release gates or tracking generated public-network artifacts.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic default tests plus opt-in UAT | Test parsing, preflight classification, injected endpoint outcomes, and report serialization locally; keep public-network runs manual/opt-in. | yes |
| Add live network to `bash scripts/verify.sh` | Make public-network smoke part of the default verification contract. | no |
| Manual-only verification | Rely only on operator UAT and skip deterministic regression tests. | no |

**Recommended choice:** Deterministic default tests plus opt-in UAT.
**Rationale:** Project guidance keeps default verification deterministic. Phase 42 should still be testable without depending on public DNS/TCP availability.

---

## the agent's Discretion

- Exact TypeScript helper boundaries and report field names may be selected by the planner, provided the schema remains typed and stable.
- The planner may decide whether endpoint outcomes come from daemon durable state, daemon output parsing, an injected preflight helper, or a combination, as long as the report states the source clearly.

## Deferred Ideas

- Peer rotation/backoff and mixed-peer survival are Phase 43 work.
- Peer contribution attribution is Phase 44 work.
- Final live-mainnet header/block/restart evidence is Phase 50 work.

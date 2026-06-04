---
phase: 57-block-download-and-connect-progress
status: passed
reviewed_at: 2026-06-04T10:21:08Z
generated_by: local-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 57-2026-06-03T13-56-54
---

# Phase 57 Code Review

## Result

No remaining blocking issues found.

## Scope Reviewed

- Bounded best-chain block request scheduling and in-flight cleanup.
- Typed block connect dispositions and peer-attributed no-credit block responses.
- Durable downloaded/connected block height and hash projection.
- Live-smoke `result.firstBlockProgress` pass/no-progress derivation and block-specific diagnoses.
- Operator and parity documentation for the bounded block-progress claim.

## Fixed Review Finding

- Unrequested block bodies could reach `receive_sync_message` and connect active chainstate before the sync shell made the requested-best-chain credit decision. Fixed in `e234cd8` by classifying unrequested block bodies before connect handling, keeping them no-credit, and adding `unrequested_extending_block_response_is_no_credit_and_does_not_mutate_chainstate`.

## Checks

- Useful `blocks_received` credit is limited to requested best-chain block bodies that validate and connect.
- Duplicate, disconnected, non-extending, invalid, malformed, and `notfound` block responses stay peer-attributed without useful progress credit.
- Downloaded-only progress remains distinct from connected-chain progress in durable status and live-smoke reports.
- Phase 57 live-smoke pass status still requires connected block height advancement.
- Public-network smoke remains opt-in and outside `bash scripts/verify.sh`.

## Residual Risk

- Phase 57 proves bounded first-block download/connect behavior and deterministic diagnosis paths; it does not claim unattended public-mainnet full sync, inbound serving, transaction relay, support bundle closeout, or restart/resume closeout beyond the covered status projections.

## Self-Check: PASSED

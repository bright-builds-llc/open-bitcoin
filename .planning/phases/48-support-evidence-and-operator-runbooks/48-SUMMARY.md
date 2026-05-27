---
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 48-2026-05-27T13-21-54
generated_at: 2026-05-27T13:59:27.099Z
phase: 48
plan: 01
status: complete
---

# Phase 48 Summary: Support Evidence and Operator Runbooks

## Completed

- Added `open-bitcoin support bundle` with optional `--output-dir` and
  `--include-live-smoke-report` flags.
- Wrote local bundle artifacts as `support-evidence.json` and
  `support-evidence.md`.
- Embedded the existing `OpenBitcoinStatusSnapshot` in the JSON evidence instead
  of creating a separate support-only status DTO.
- Added config-path evidence, metadata-only credential evidence, store-health
  availability, redaction metadata, and summary-only live-smoke ingestion.
- Added integration coverage for bundle creation, JSON/Markdown fields,
  live-smoke summary allowlisting, and absence of fixture secrets.
- Updated operator runbooks, status snapshot architecture docs, parity
  breadcrumbs, and the tracked LOC report.

## Verification

- `cargo fmt --all`
- `cargo test --all-features -p open-bitcoin-cli support_bundle --test operator_binary`
- `cargo test --all-features -p open-bitcoin-cli open_bitcoin_support_bundle_routes_to_operator_command`
- `cargo clippy -p open-bitcoin-cli --all-targets --all-features -- -D warnings`
- `bash scripts/verify.sh`

## Simplification Pass

The support bundle reuses the status collector and shared snapshot contract.
Rendering helpers were split into `operator/support/render.rs` so the command
entrypoint stays below the repo production-file limit without adding a broader
abstraction.

## Residual Risk

The bundle is intentionally local evidence. It does not claim unattended
production-node readiness, does not run public-network checks in default
verification, and only summarizes allowlisted fields from live-smoke reports.

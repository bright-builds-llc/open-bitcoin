---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
plan_id: "35-03"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T21:29:26.254Z"
status: completed
---

# Summary 35-03: Operator Docs And Phase Verification

## Completed

- Updated the operator runtime guide with JSONC and CLI activation examples plus explicit Phase 35 limitations.
- Updated config precedence docs to describe `open-bitcoind` Open Bitcoin-only sync flags and keep them out of `bitcoin.conf`.
- Updated README, AGENTS, planning docs, and parity docs to distinguish activation/preflight from unattended full public-mainnet sync.
- Updated the Phase 35 roadmap plan links and clarified that long-lived sync cancellation/shutdown belongs to later live-sync phases.

## Residual Risks

- Public-network sync remains outside default verification.
- Full operator-ready mainnet IBD docs still depend on Phases 36-40.

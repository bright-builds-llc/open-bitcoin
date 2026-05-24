---
phase: 42
slug: live-smoke-entry-and-network-preflight
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-24T14:39:51.732Z
updated: 2026-05-24T14:39:51.732Z
---

# Phase 42 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Operator CLI arguments -> generated config | Manual-peer inputs become daemon config used for public-network connection attempts. | Operator-supplied endpoint strings and generated JSONC config. |
| Local network preflight -> report evidence | DNS/TCP diagnostics may influence operator troubleshooting and reviewer conclusions. | Endpoint host, port, resolved endpoint, state, and failure metadata. |
| Runtime durable status -> typed diagnosis | Stored peer/status data is summarized into no-progress causes and next actions. | Durable sync progress, peer telemetry, final status, and failure reasons. |
| Cancellation signal -> partial report | Interrupt handling must stop the child process and still write accurate partial evidence. | SIGINT/SIGTERM state, daemon exit signal, and partial report data. |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-42-01-01 | Tampering | Generated JSONC config | mitigate | `--manual-peer` values are parsed before use, generated config is written from parsed option state, generated config path is recorded in report options, and mixed `--config` plus `--manual-peer` is rejected. Evidence: `scripts/run-live-mainnet-smoke.ts` argument parsing and `optionsWithGeneratedManualPeerConfig`; `scripts/test-run-live-mainnet-smoke.sh` validates manual peers and `dns_seeds: []`. | closed |
| T-42-01-02 | Spoofing | Endpoint outcomes | mitigate | Endpoint outcomes carry explicit `stage` and `source` fields, with preflight outcomes emitted separately from runtime durable peer telemetry. Evidence: `EndpointOutcomeStage`, `EndpointOutcomeSource`, `endpointOutcome`, and `endpointOutcomesFromFinalStatus` in `scripts/run-live-mainnet-smoke.ts`; UAT test 2 passed. | closed |
| T-42-01-03 | Repudiation | No-progress diagnosis | mitigate | Reports persist `maybeNoProgressCause`, `nextAction`, endpoint outcomes, final status, daemon stdout/stderr tails, and status snapshots. Evidence: `SmokeReport` assembly and Markdown report rendering in `scripts/run-live-mainnet-smoke.ts`; `scripts/test-run-live-mainnet-smoke.sh` validates TCP no-progress classification and cancellation evidence. | closed |
| T-42-01-04 | Denial of Service | Network preflight | mitigate | Network preflight has bounded TCP timeout and per-source endpoint attempt limits, and deterministic fixture/skip environment variables keep default verification public-network-free. Evidence: `DEFAULT_NETWORK_PREFLIGHT_TIMEOUT_MS`, `DEFAULT_ENDPOINTS_PER_SOURCE`, `networkPreflightEndpointOutcomes`, and fixture-backed tests in `scripts/test-run-live-mainnet-smoke.sh`. | closed |
| T-42-01-05 | Information Disclosure | Reports | mitigate | Reports include operator-supplied public peer endpoints and the existing fixed local smoke RPC command echo only; daemon output tails are bounded. Evidence: `commands` report fields use literal `smoke` credentials for local smoke RPC, `MAX_TAIL_BYTES` bounds captured child output, and runtime guide documents operator-visible report behavior. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

No accepted risks.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-24 | 5 | 5 | 0 | Codex secure-phase |

---

## Verification Evidence

- `bash scripts/test-run-live-mainnet-smoke.sh` passed during Phase 42 verification and UAT.
- Targeted UAT confirmed runtime `handshook` endpoint telemetry is projected when durable peer capabilities are present.
- `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `cargo test --manifest-path packages/Cargo.toml --all-features` passed before the Phase 42 UAT commit.
- `bash scripts/verify.sh` passed in the Phase 42 implementation commit hook and UAT commit hook.

---

## Standards Context

- Material local guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and `bright-builds-rules.audit.md`.
- Canonical standards consulted: Bright Builds standards index, verification, and TypeScript/JavaScript guidance at pinned commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`.
- `standards/index.md` is not present locally in this downstream repo; the pinned upstream canonical pages were used through the audit manifest source.

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-24

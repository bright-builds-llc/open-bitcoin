---
phase: 40
slug: live-mainnet-smoke-docs-and-parity-closeout
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-23T02:47:19.151Z
---

# Phase 40 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 40 added an explicit opt-in live mainnet smoke runner, offline regression
coverage for that runner, and documentation/parity closeout updates. The phase
plan did not include a `<threat_model>` block and the summary did not include a
`## Threat Flags` section, so there are no registered threats requiring auditor
closure for this phase.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Operator shell -> smoke runner | The operator invokes `bun run scripts/run-live-mainnet-smoke.ts` with an explicit datadir and optional config path. | Local filesystem paths, timeout and poll options. |
| Smoke runner -> local daemon/RPC | The runner builds local binaries, launches `open-bitcoind` with explicit `mainnet-ibd` activation, and probes `getblockchaininfo` over loopback RPC. | RPC credentials scoped to the transient smoke process, sync status JSON, daemon stdout/stderr tails. |
| Smoke runner -> generated reports | The runner writes local JSON and Markdown evidence reports under the selected output directory. | Runtime provenance, command lines, preflight results, sync snapshots, final status, and daemon output tails. |
| Documentation -> operator expectations | README, operator guide, and parity docs describe shipped behavior and non-claims. | Human-readable instructions, support boundaries, and parity metadata. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|

No registered threats were present in the Phase 40 plan or summary artifacts.
Security review verified the relevant live-smoke controls and documentation
boundaries listed below.

## Verified Controls

| Control | Evidence | Result |
|---------|----------|--------|
| Live mainnet work is explicit and opt-in | `scripts/run-live-mainnet-smoke.ts` requires `--datadir`; `docs/operator/runtime-guide.md` and `README.md` keep live smoke outside default verification. | closed |
| Datadir/config preflight fails early | `buildPreflightChecks` requires an existing datadir, checks optional config path existence, validates plausible system time, checks disk headroom, and records preflight failure reports. | closed |
| Daemon activation is bounded to the intended local surface | The runner launches the built daemon with `-main`, `-openbitcoinsync=mainnet-ibd`, `-rpcbind=127.0.0.1`, and transient smoke RPC credentials. | closed |
| Failure reports are actionable | The runner writes JSON/Markdown reports for preflight failures, runtime failures, and no-progress outcomes; zero-outbound-peer runs produce DNS/TCP or explicit-peer guidance. | closed |
| Default verification remains hermetic | `scripts/test-run-live-mainnet-smoke.sh` uses mock daemon/status commands and confirms report generation plus preflight messaging without public-network access. | closed |
| Documentation avoids over-claiming | README, operator runtime docs, parity checklist, release readiness, and deviations docs preserve non-production, non-funds, and non-packaged-service boundaries. | closed |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-23 | 0 | 0 | 0 | Codex |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-23

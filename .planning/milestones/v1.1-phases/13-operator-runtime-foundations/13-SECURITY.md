---
phase: 13
slug: operator-runtime-foundations
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-03
---

# Phase 13 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| contributors -> architecture contracts | Phase 13 docs define future operator-runtime implementation constraints that later adapters and commands will trust. | Dependency selection, retention defaults, status semantics, CLI routing, config ownership |
| local inputs -> typed contracts | Untrusted CLI argv, JSONC text, environment, `bitcoin.conf`, and cookie-file metadata must map into typed contracts without leaking secrets. | Command tokens, config values, precedence metadata, credential-source metadata |
| later runtime collectors/adapters -> shared models | Future storage, logging, metrics, status, and service collectors will project effectful runtime state into these shell-owned contracts. | On-disk state, telemetry samples, service state, node status, provenance |
| shared models -> operator surfaces | CLI, JSON, and dashboard consumers will trust stable enums, unavailable-state semantics, and bounded retention policies. | Status fields, health signals, log/metrics retention, operator-visible metadata |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-13-01-01 | T | `docs/architecture/storage-decision.md` | mitigate | ADR records `Decision: fjall`, compares `fjall`, `redb`, and `rocksdb`, and documents explicit recovery obligations and RocksDB fallback constraints. | closed |
| T-13-01-02 | D | `packages/open-bitcoin-node/src/storage.rs` | mitigate | Storage contracts model schema mismatch, corruption, interrupted writes, reindex, repair, and backup restoration guidance as typed errors/actions instead of panics. | closed |
| T-13-01-03 | E | pure core crates | mitigate | Storage contracts remain in `open-bitcoin-node`; the phase-13 storage contract file has no `std::fs`, `fjall::`, `redb::`, or `rocksdb::` imports. | closed |
| T-13-01-04 | D | storage dependency selection | mitigate | The ADR keeps RocksDB as fallback-only pending measured Rust-native failure, blocking unreviewed native dependency expansion. | closed |
| T-13-02-01 | D | `MetricRetentionPolicy` | mitigate | Observability docs and metrics contracts encode bounded defaults of 30-second sampling, 2880 samples, and 24-hour max age. | closed |
| T-13-02-02 | D | `LogRetentionPolicy` | mitigate | Observability docs and logging contracts encode daily rotation, 14 files, 14 days, and a 268435456-byte total cap, separate from rolling-file creation. | closed |
| T-13-02-03 | I | `logging.rs` | mitigate | Logging/status contract surface is limited to levels, paths, retention, and availability; docs explicitly keep RPC passwords and cookie contents out of operator-visible fields. | closed |
| T-13-02-04 | T | serialized metrics/logs | mitigate | Metrics and logging contracts use serde rename rules and typed enums for metric kinds, log levels, and availability states instead of free-form strings. | closed |
| T-13-03-01 | I | `OpenBitcoinStatusSnapshot` | mitigate | Status contracts expose availability/path metadata and omit raw RPC passwords, cookie contents, and auth headers from the shared snapshot. | closed |
| T-13-03-02 | T | `BuildProvenance` | mitigate | Missing build provenance serializes as `Unavailable { reason }`, preventing renderers from silently hiding absent provenance data. | closed |
| T-13-03-03 | R | stopped-node status | mitigate | Status docs and tests require stopped/unreachable live fields to remain explicit `Unavailable` values with reasons for supportability. | closed |
| T-13-03-04 | S | daemon state | mitigate | The shared status model uses typed `NodeRuntimeState` variants instead of free-form daemon-state strings. | closed |
| T-13-04-01 | T | `route_cli_invocation` | mitigate | `route_cli_invocation` sends `open-bitcoin-cli` invocations to `BitcoinCliCompat(Vec<OsString>)`, with route tests preserving raw `-named` compatibility tokens. | closed |
| T-13-04-02 | I | clap help/output | mitigate | The operator command contract does not model `rpcpassword`, cookie contents, or auth headers, leaving existing auth handling in the compatibility parser path. | closed |
| T-13-04-03 | D | CLI parsing | mitigate | `-stdin` and `-stdinrpcpass` remain owned by the compatibility parser, and regression coverage proves normal no-stdin CLI calls do not hang. | closed |
| T-13-04-04 | S | binary route detection | mitigate | Route tests cover binary names ending in `open-bitcoin` and `open-bitcoin-cli`, preserving distinct operator and compatibility identities. | closed |
| T-13-05-01 | T | `parse_open_bitcoin_jsonc_config` | mitigate | `jsonc-parser` feeds serde structs with `deny_unknown_fields`, and tests cover comments, trailing commas, onboarding answers, and unknown-field rejection. | closed |
| T-13-05-02 | T | `ConfigPrecedence` | mitigate | Config docs and tests encode the exact order `CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`. | closed |
| T-13-05-03 | I | config/status surfaces | mitigate | Open Bitcoin JSONC and status/config docs keep cookie files as credential-source metadata only and exclude RPC password/cookie contents from owned config contracts. | closed |
| T-13-05-04 | R | `bitcoin.conf` compatibility | mitigate | Config tests prove Open Bitcoin-only keys such as `dashboard` and `service` are rejected by the baseline-compatible `bitcoin.conf` loader. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

No accepted risks.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-03 | 20 | 20 | 0 | Codex (`gsd-secure-phase`) |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-03

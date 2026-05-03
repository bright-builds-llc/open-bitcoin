---
phase: 17
slug: cli-status-and-first-run-onboarding
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-03
updated: 2026-05-03
---

# Phase 17 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 17 delivers the operator-facing `open-bitcoin` binary surface for status,
config-path discovery, first-run onboarding, and read-only Core or Knots
detection. The phase threat models cover config precedence, read-only detection,
shared status rendering, onboarding write gates, operator or compatibility
routing, and redaction of local credential evidence.

The Phase 17 plans declared 23 threats across five plan slices. No additional
open threat flags were found in the Phase 17 summaries.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| CLI args, environment, JSONC, and `bitcoin.conf` to operator runtime | Untrusted operator input and local config files become typed startup and status evidence. | Config paths, datadir, network, RPC endpoint, source precedence, and onboarding answers. |
| Local filesystem to detection and onboarding | Existing Core or Knots evidence is surfaced for support or onboarding without mutating source installs. | Datadir, `bitcoin.conf`, `.cookie`, wallet paths, service-definition candidates, mtime, and confidence or uncertainty metadata. |
| Live RPC and durable local evidence to shared status snapshot | Runtime RPC responses and managed log or metrics state become support-oriented human and JSON output. | Network, chain tip, mempool, wallet freshness, build provenance, local paths, unavailable reasons, and warning health signals. |
| Terminal and stdin to onboarding writes | Interactive and non-interactive answers can trigger managed config writes. | Network, datadir, config path, write approval, overwrite intent, and onboarding plan messages. |
| Operator binary to compatibility CLI surface | Routing must preserve baseline-compatible `open-bitcoin-cli` behavior while exposing operator-only commands on `open-bitcoin`. | `argv[0]`, parsed subcommands, operator-only flags, and explicit deferred-surface messages. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-17-01-01 | Tampering | `operator/detect.rs` contract | mitigate | Detection stays read-only and acceptance scans reject write, remove, copy, rename, or service-manager calls in the detection surface. | closed |
| T-17-01-02 | Information Disclosure | status and config contract names | mitigate | Contract types report credential source or path presence only and do not carry raw password, authorization, cookie value, or secret payload fields. | closed |
| T-17-01-03 | Spoofing | command routing | mitigate | `open-bitcoin-cli` compatibility routing tests remain green so baseline-compatible RPC invocations are not parsed as operator commands. | closed |
| T-17-01-04 | Repudiation | new Rust files | mitigate | Phase 17 Rust sources stay covered by parity breadcrumbs and the breadcrumb verification check. | closed |
| T-17-02-01 | Spoofing | source precedence | mitigate | Config resolution preserves the documented `CLI > environment > JSONC > bitcoin.conf > cookies > defaults` order and tests lock the source labels. | closed |
| T-17-02-02 | Tampering | JSONC and `bitcoin.conf` parsing | mitigate | JSONC is parsed through the typed config parser and Open Bitcoin-only keys in `bitcoin.conf` continue to fail deterministically. | closed |
| T-17-02-03 | Information Disclosure | credential reporting | mitigate | Operator config and status report only credential source or path metadata; redaction tests reject raw password, auth-header, and cookie-content output. | closed |
| T-17-02-04 | Denial of Service | malformed JSONC | mitigate | Invalid JSONC returns typed `Error reading open-bitcoin.jsonc` failures instead of panicking or silently falling back. | closed |
| T-17-03-01 | Tampering | Core or Knots detection | mitigate | Detection tests prove candidate file contents, readonly bits, and mtimes remain unchanged across scans. | closed |
| T-17-03-02 | Information Disclosure | cookie and wallet paths | mitigate | Detection reports cookie and wallet paths only; it does not render `.cookie` contents or wallet database contents. | closed |
| T-17-03-03 | Spoofing | product identity | mitigate | Detection emits explicit confidence and uncertainty fields when Core versus Knots evidence is ambiguous. | closed |
| T-17-03-04 | Elevation of Privilege | service candidates | mitigate | Detection and status stay read-only and do not invoke `launchctl`, `systemctl`, or privileged service lifecycle operations. | closed |
| T-17-04-01 | Information Disclosure | status JSON and human output | mitigate | Status redaction tests reject raw passwords, `Authorization` or `Basic` headers, and cookie contents in both renderers. | closed |
| T-17-04-02 | Denial of Service | RPC and status collector | mitigate | Status degrades to unavailable fields and `unreachable` snapshots on RPC failures rather than failing the whole command. | closed |
| T-17-04-03 | Tampering | metrics and log evidence | mitigate | Status uses the managed Phase 16 log and metrics loaders and does not parse unmanaged raw files outside configured managed paths. | closed |
| T-17-04-04 | Spoofing | build, config, and source reporting | mitigate | Status includes source paths, build provenance, unavailable reasons, and the new live-RPC bootstrap warning so support output does not imply evidence that was never collected. | closed |
| T-17-04-05 | Spoofing | Core or Knots detection evidence | mitigate | Detection confidence and uncertainty are mapped into health signals and tested as non-definitive support evidence. | closed |
| T-17-05-01 | Tampering | onboarding JSONC writes | mitigate | Non-interactive onboarding requires `--approve-write` for creation and `--force-overwrite` for existing configs; tests prove no overwrite without force. | closed |
| T-17-05-02 | Tampering | `bitcoin.conf` compatibility | mitigate | Onboarding writes only `open-bitcoin.jsonc`, and tests assert `bitcoin.conf` is never created or modified. | closed |
| T-17-05-03 | Spoofing | binary or routing boundary | mitigate | `open-bitcoin` is a separate binary and compatibility tests keep `open-bitcoin-cli` behavior isolated. | closed |
| T-17-05-04 | Information Disclosure | onboarding and status output | mitigate | Onboarding and status reuse the redaction contract and avoid printing raw cookie contents, RPC passwords, auth headers, or wallet database contents. | closed |
| T-17-05-05 | Denial of Service | non-interactive onboarding | mitigate | Missing required onboarding values fail deterministically without prompting or writing files. | closed |
| T-17-05-06 | Spoofing | live status adapter | mitigate | Running-node tests use deterministic fake RPC responses, while real RPC failures degrade to unavailable fields instead of fabricated live data. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Read-only detection surface | `operator/detect.rs` performs path discovery through `read_dir`, `is_file`, and related metadata checks, while detection tests prove files are unchanged after scans. | verified |
| Breadcrumb accountability | `docs/parity/source-breadcrumbs.json` covers the Phase 17 Rust additions and breadcrumb verification remains part of the repo contract. | verified |
| Exact config-precedence order | `operator/config.rs` resolves typed config through the documented source order and tests lock the precedence labels. | verified |
| Typed config parse and deterministic failures | JSONC parsing goes through `parse_open_bitcoin_jsonc_config`, and invalid JSONC or Open Bitcoin-only `bitcoin.conf` keys fail with typed errors. | verified |
| Credential redaction | Status and onboarding tests reject raw `secret`, `Authorization`, `Basic`, `rpcpassword`, and cookie-content rendering. | verified |
| Ambiguous-detection honesty | Detection and status render confidence and uncertainty metadata instead of pretending Core or Knots identity is certain. | verified |
| Support-oriented status degradation | Status keeps successful command execution while returning stopped or unreachable snapshots with explicit unavailable reasons and warning health signals. | verified |
| Managed local evidence loading | Status uses the managed log and metrics loaders rooted in configured paths instead of parsing arbitrary filesystem state. | verified |
| Build and source provenance | Shared status output includes config paths, build provenance, and a live-RPC bootstrap warning when credentials cannot be rediscovered. | verified |
| Onboarding write gates | Onboarding requires explicit write approval, requires force for replacement, and never writes `bitcoin.conf`. | verified |
| Compatibility and operator routing separation | `open-bitcoin` and `open-bitcoin-cli` route through separate surfaces with compatibility tests guarding the boundary. | verified |
| Deterministic live-status wiring | Fake-running RPC tests prove running status fields, and live-RPC bootstrap without rediscoverable auth now emits an explicit warning instead of failing silently. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-03 | 23 | 23 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted review of all five Phase 17 `<threat_model>` blocks and summary artifacts. | Found 23 declared threats across Plans 17-01 through 17-05 and no additional open threat flags in summaries. |
| `rg -n "fs::write|remove_file|remove_dir|rename|copy|OpenOptions::new\\(|launchctl|systemctl" packages/open-bitcoin-cli/src/operator/detect.rs packages/open-bitcoin-cli/src/operator/detect/tests.rs` | Production detection surface stayed read-only; only test fixtures write candidate files. |
| Targeted `rg` review across operator config, status, onboarding, and binary tests for secret-bearing strings and redaction assertions. | Redaction coverage present for passwords, `Authorization`, `Basic`, and cookie contents. |
| Targeted review of operator routing, onboarding write-gate tests, and live-status bootstrap tests. | Separate binary routing, deterministic onboarding failures, explicit write gates, and live-RPC bootstrap warnings verified. |
| `bash scripts/verify.sh` | Passed during the current Phase 17 patch verification run, including targeted and repo-wide Rust, benchmark smoke, Bazel smoke, breadcrumb, and policy checks. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-05-03

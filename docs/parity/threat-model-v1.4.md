# v1.4 Threat Model and Release Boundaries

## Scope

This document is the reviewer-facing threat model for the v1.4 Mainnet IBD
Convergence and Peer Compatibility milestone. It covers the shipped opt-in
outbound IBD evidence path, public peer compatibility handling, header and block
input, runtime resource bounds, same-datadir restart/resume evidence, report
redaction, support evidence, operator-facing live evidence, deterministic local
verification, and the parity roots that bind those claims together.

Phase 59 is a documentation, support-evidence, and reviewer traceability
closeout. It adds no new authentication, network service, cryptography, wallet
production use, hosted upload, or public dashboard behavior. Public-network
checks remain explicit operator UAT and stay outside `bash scripts/verify.sh`.

The v1.4 claim is source-built, opt-in outbound IBD progress evidence for
mainnet review. It can show outbound peer compatibility, validated header
progress, downloaded or connected block progress, same-datadir restart/resume
evidence, or a typed diagnosed blocker with next operator action. It is not an
inbound serving, transaction relay, production-funds wallet use, migration apply
mode, packaging, hosted dashboard, GUI, Windows service support, or unattended
production-node operation claim.

## Assets

| Asset | Why It Matters | Evidence Surface |
| --- | --- | --- |
| Public peer compatibility handling | Reviewers need to distinguish compatible outbound handshakes and useful contribution from reachable but incompatible peers. | Compatibility harnesses, durable peer telemetry, `scripts/run-live-mainnet-smoke.ts`, `result.peerOutcomeSummary` |
| Header and block input | Headers and blocks from public peers must be validation-gated before they become progress evidence. | `result.firstHeaderProgress`, `result.firstBlockProgress`, durable status snapshots |
| Runtime resource bounds | Opt-in public-network review must stay bounded and diagnosable instead of becoming an unbounded daemon claim. | Runtime guide resource-bound settings, sync status, metrics, structured logs |
| Restart/resume evidence | Same-datadir evidence must prove durable continuity without duplicate block connects or source-datadir mutation. | `result.restartResumeEvidence`, before/after status snapshots, support evidence |
| Report redaction and support evidence | Review artifacts must summarize local diagnostics without leaking credentials, raw logs, raw endpoint tables, wallet material, or unbounded report data. | `support-evidence.json`, `support-evidence.md`, redaction summary |
| Operator-facing live evidence | Operators and reviewers need field-level pass/fail interpretation rather than timing or startup claims. | `docs/operator/runtime-guide.md`, live-smoke JSON/Markdown reports |
| Deterministic verification | Default repo verification must remain hermetic and public-network-free. | `bash scripts/verify.sh`, `scripts/test-run-live-mainnet-smoke.sh` |
| Parity and release claim roots | Human and machine roots must link the current v1.4 evidence without rewriting v1.3 history. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md` |

## Trust Boundaries

| Boundary | Inputs | Expected Control |
| --- | --- | --- |
| Public peer input -> sync runtime | Manual peers, DNS peers, wire messages, idle peers, malformed payloads, invalid headers or blocks | Outbound-only activation, compatibility diagnosis, validation-gated contribution, typed no-credit peer outcomes |
| Sync runtime -> durable status | Headers, downloaded blocks, connected blocks, peer outcomes, resource pressure, recovery state | Separated header/downloaded/connected progress, bounded resource settings, recovery guidance, latest-error projection |
| Same datadir -> restart review | Pre-restart state, process termination, fresh daemon launch, post-restart state | Same resolved datadir checks, before/after status snapshots, duplicate-connect verdicts, recovery diagnosis |
| Local machine -> reviewer artifact | Live-smoke reports, support bundles, metrics, structured logs, config paths, credential sources | Allowlisted summaries, recursive redaction, metadata-only credential reporting, generated artifacts kept out of git |
| Operator evidence -> release claim | Runtime guide commands, support evidence, parity roots, release-readiness docs | Field-level evidence acceptance, explicit non-claims, deterministic verification boundary |
| Historical docs -> current docs | v1.3 threat model, v1.3 release roots, v1.4 closeout docs | Separate v1.4 threat model and checklist surface; v1.3 remains historical evidence |

## STRIDE Threat Register

| Threat ID | STRIDE | Component | Scenario | Mitigation / Evidence | Residual Risk / Future Gate |
| --- | --- | --- | --- | --- | --- |
| V14-TM-01 | Spoofing, Tampering, Denial of Service | Public peer compatibility handling | A peer completes TCP or partial handshake but rejects the baseline-compatible flow, sends unsupported message order, disconnects, or idles without useful contribution. | Compatibility diagnostics distinguish version rejection, network mismatch, service-bit mismatch, unsupported order, timeout, disconnect, malformed payload, and local config failure. Live-smoke and support evidence preserve peer outcomes without useful-progress credit. | Inbound serving, address advertisement, peer eviction, and ban policy need a future production-node threat model. |
| V14-TM-02 | Tampering, Denial of Service | Header and block input validation | A public peer sends malformed, invalid, duplicate, disconnected, or unavailable headers or blocks that could be mistaken for progress. | Header progress is accepted only after validated header-height increase. Block progress separates downloaded block bodies from connected chainstate. Invalid, malformed, duplicate, disconnected, and `notfound` block responses are peer-attributed and uncredited. | Broader relay, compact-block, and long-running peer-governance behavior remain future gates. |
| V14-TM-03 | Denial of Service | Runtime resource bounds | Public-network work exhausts message, header, block, metric, log, or durable-write capacity during review. | Runtime guide documents bounded messages per peer, sync rounds, in-flight blocks per peer and total, one header request per peer, metrics retention, log retention, and no unbounded durable-write queue. Resource exhaustion is a typed recovery diagnosis. | Unattended production-node operation requires long-run evidence, service supervision, and resource-governance review. |
| V14-TM-04 | Repudiation, Tampering | Same-datadir restart/resume evidence | A restart report uses the wrong datadir, loses durable progress, duplicates block connection, or confuses downloaded-only progress with connected progress. | `result.restartResumeEvidence` records requested/resolved same-datadir checks, before/after header, downloaded block, and connected block status, stable hashes when heights do not move, duplicate-connect verdict, and recovery diagnosis. | Automatic daemon supervision and production recovery loops remain deferred. |
| V14-TM-05 | Information Disclosure | Report redaction and support evidence | A support bundle leaks cookie contents, `rpcpassword`, `rpcauth`, wallet material, manual peer lists, endpoint tables, raw snapshots, or daemon output tails. | Support evidence uses allowlisted live-smoke summaries, metadata-only credential reporting, recursive redaction, and explicit omitted-category summaries. Raw live-smoke input and unbounded local artifacts stay local. | Hosted support upload or public artifact validation needs a new data-protection design. |
| V14-TM-06 | Repudiation, Spoofing | Operator-facing live evidence interpretation | Reviewers mistake elapsed time, daemon startup, support-bundle existence, peer reachability, downloaded-only progress, or diagnosed blockers for a broader production-readiness claim. | Runtime guide and release-readiness docs require field-level interpretation of `result.status`, `result.progressDetected`, `result.firstHeaderProgress`, `result.firstBlockProgress`, `result.restartResumeEvidence`, `result.maybeNoProgressCause`, `result.nextAction`, and final durable status counters. | A stronger release gate would need explicit criteria and fresh milestone scope. |
| V14-TM-07 | Repudiation, Denial of Service | Default deterministic verification boundary | Public-network checks accidentally become part of default repo verification or reviewers assume `bash scripts/verify.sh` proves live public sync. | Default verification remains deterministic. The release docs require `bash scripts/verify.sh` and deterministic fixture checks, while live-smoke commands remain opt-in UAT outside the default gate. | Public-network CI or release-blocking live sync remains a future policy decision. |
| V14-TM-08 | Repudiation, Spoofing | Parity and release claim roots | Machine-readable or human-readable parity roots omit v1.4 evidence, hide deferred surfaces, or rewrite v1.3 closeout evidence as current. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md`, and this file add separate v1.4 roots while preserving `threat-model-v1.3.md` as historical evidence. | Future milestones that expand deferred surfaces need new roots and threat models rather than editing history. |

## ASVS L1 Mapping

This mapping uses OWASP ASVS v5.0.0 as a reviewer vocabulary for local evidence
handling. It is not a certification claim and does not invent product scope.

| ASVS v5.0.0 Area | v1.4 Local Mapping | Evidence |
| --- | --- | --- |
| Input validation and sanitization | Public header and block input is parsed and validation-gated before progress credit; support input is allowlisted before rendering. | V14-TM-01, V14-TM-02, V14-TM-05 |
| Logging and error privacy | Support evidence summarizes metrics, logs, store health, peer outcomes, and live-smoke status without raw daemon tails, raw endpoint tables, or credential values. | V14-TM-05, `support-evidence.json`, `support-evidence.md` |
| Configuration and secrets management | Config paths and credential sources are reported as metadata; cookie contents, `rpcpassword`, and `rpcauth` values are not support evidence. | `docs/architecture/config-precedence.md`, V14-TM-05 |
| Access-control limits for local-only artifacts | Phase 59 adds no new authentication or remote administration surface; artifacts remain local files generated by explicit operator commands. | V14-TM-06, V14-TM-07 |

## Evidence Acceptance

Accepted v1.4 evidence is field-first and local-artifact based:

1. Deterministic local verification:
   - `bash scripts/verify.sh` passes without invoking public-network live
     smoke or `--restart-after-progress`.
   - `bash scripts/test-run-live-mainnet-smoke.sh` proves schema v2 fixture
     behavior for live-smoke evidence fields.
2. Opt-in outbound IBD progress:
   - Live-smoke JSON/Markdown reports include `result.status`,
     `result.progressDetected`, `result.firstHeaderProgress`,
     `result.firstBlockProgress`, `result.maybeNoProgressCause`,
     `result.nextAction`, and final durable status counters.
   - Header progress, downloaded block progress, and connected block progress
     remain distinct.
3. Same-datadir restart/resume evidence:
   - `result.restartResumeEvidence` records same requested/resolved datadir
     checks, before/after durable status, duplicate-connect verdict, and
     `result.restartResumeEvidence.recoveryDiagnosis.category`.
4. Support evidence:
   - `support-evidence.json` and `support-evidence.md` summarize config paths,
     status, store health, metrics/log availability, allowlisted live-smoke
     fields, and redaction metadata.
   - Support evidence is review context. Its existence does not prove public
     sync success.

Required local commands and artifact names are documented in
`docs/operator/runtime-guide.md` and `docs/parity/release-readiness.md`.

## Release Boundary Matrix

| Surface | v1.4 Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements |
| --- | --- | --- | --- | --- | --- |
| v1.4 opt-in outbound IBD claim | Source-built operators can collect opt-in outbound IBD evidence for peer compatibility, header progress, block progress, restart/resume, or typed diagnosed blockers. | `bash scripts/verify.sh`, `bash scripts/test-run-live-mainnet-smoke.sh`, live-smoke JSON/Markdown, `support-evidence.json`, `support-evidence.md`, runtime guide commands. | v1.4 does not make public-network checks part of default verification and does not claim unattended public service. | Future release policy can decide whether live evidence becomes a stronger gate. | OBS-01, OBS-02, OBS-03, SEC-01, SEC-02, SEC-03 |
| Outbound peer compatibility | Compatible outbound peers and incompatible/no-credit peers are distinguishable in deterministic and opt-in live evidence. | Compatibility harnesses, durable peer telemetry, `result.peerOutcomeSummary`, support evidence. | No inbound serving or address advertisement claim. | Future PRODNODE-02. | OBS-01, SEC-01, SEC-02 |
| Header progress | First validated header-height increase can be recorded with before/after status. | `result.firstHeaderProgress`, durable status snapshots. | Header progress alone is not a connected block or full sync claim. | Future broader IBD completion evidence. | OBS-01, SEC-01 |
| Downloaded block progress | Downloaded best-chain block body progress can be recorded separately. | `result.firstBlockProgress.kind=downloaded`, final durable downloaded height. | Downloaded-only progress is not connected chainstate progress. | Future block-connect completion evidence. | OBS-01, SEC-01 |
| Connected block progress | Connected block progress remains the stronger block pass evidence when observed. | `result.firstBlockProgress.kind=connected`, final durable connected height. | v1.4 does not claim full chain sync or long-run convergence. | Future full-IBD milestone. | OBS-01, SEC-01 |
| Same-datadir restart/resume evidence | A restart review can prove durable state continuity on the same selected datadir. | `result.restartResumeEvidence`, Cargo/Bazel `sync status --format json`. | No service-manager restart policy or unattended recovery loop claim. | Future PRODNODE-01 and service-supervision work. | OBS-01, OBS-03, SEC-01 |
| Support evidence | Operators can generate local redacted v1.4 support evidence that summarizes diagnostics without raw sensitive data. | `support-evidence.json`, `support-evidence.md`. | Support bundles are not release validators, hosted uploads, or public-mainnet proof by themselves. | Future artifact validator or hosted support design. | OBS-02, OBS-03, SEC-01 |
| Threat model and parity roots | Reviewers can inspect v1.4 threat, release, checklist, and machine-readable parity roots. | `docs/parity/threat-model-v1.4.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`. | v1.3 docs remain historical and are not rewritten into current claims. | Future milestone roots when scope expands. | SEC-01, SEC-02 |
| Deterministic verification | Default verification stays local, deterministic, and public-network-free. | `bash scripts/verify.sh`, deterministic fixture scripts. | No public-network CI gate or default live smoke claim. | Future release policy decision. | SEC-03 |
| Inbound serving | No shipped v1.4 claim. | Deferred-surface docs and release matrix rows. | v1.4 does not claim inbound serving or address advertisement. | Future PRODNODE-02. | SEC-02 |
| Transaction relay | No shipped v1.4 claim. | Deferred-surface docs and release matrix rows. | v1.4 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03. | SEC-02 |
| Production-funds wallet use | No shipped v1.4 claim. | Deferred-surface docs and release matrix rows. | v1.4 does not claim production-funds wallet use. | Future WALPROD-01. | SEC-02 |
| Migration apply mode | No shipped v1.4 claim. | Dry-run migration docs and release matrix rows. | v1.4 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future MIGAPPLY-01. | SEC-02 |
| Packaging | No shipped v1.4 claim. | Source-built install docs and release matrix rows. | v1.4 does not claim packaging, signed installers, or OS-native release channels. | Future PKG-01. | SEC-02 |
| Hosted dashboard | No shipped v1.4 claim. | Local terminal/dashboard docs and release matrix rows. | v1.4 does not claim hosted dashboard or public dashboard operation. | Future hosted operations design. | SEC-02 |
| GUI | No shipped v1.4 claim. | Headless and terminal-first docs. | v1.4 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | SEC-02 |
| Windows service support | No shipped v1.4 claim. | Source-built local service docs and release matrix rows. | v1.4 does not claim Windows service support or certification. | Future PKG-02. | SEC-02 |
| Unattended production-node operation | No shipped v1.4 claim. | Deferred-surface docs, runtime-guide limitations, release matrix rows. | v1.4 does not claim unattended production-node operation. | Future PRODNODE-01. | SEC-02 |

## Requirements Traceability

| Requirement | v1.4 Trace |
| --- | --- |
| OBS-01 | Shared status, dashboard, metrics, structured logs, RPC-facing blockchain info, and live-smoke evidence preserve header, downloaded block, connected block, peer state, progress signal, and latest-error distinctions. |
| OBS-02 | Support evidence summarizes v1.4 live-smoke, peer, status, metrics/log, config, and store-health diagnostics through allowlisted redacted fields. |
| OBS-03 | Runtime-guide and release-readiness docs provide repo-local Cargo and Bazel commands plus field-level pass/fail interpretation. |
| SEC-01 | This STRIDE register covers public peer compatibility handling, header and block input, resource bounds, restart/resume evidence, report redaction, support evidence, and operator-facing live evidence. |
| SEC-02 | The release boundary matrix and parity roots distinguish the v1.4 opt-in outbound IBD claim from deferred surfaces. |
| SEC-03 | Default repo verification remains deterministic; public-network live smoke stays opt-in UAT evidence outside `bash scripts/verify.sh`. |

## Residual Risks

- Public-network conditions are operator-environment dependent. A no-progress
  report is valid only when it preserves typed diagnosis, status snapshots,
  peer outcomes, and next operator action.
- Header progress, downloaded block progress, connected block progress, and
  restart/resume evidence are intentionally separate. Reviewers should not
  collapse them into a single "synced" or production-ready flag.
- Support bundles are local, redacted troubleshooting artifacts. Hosted upload,
  raw artifact validation, or public sharing requires fresh data-protection
  review.
- Inbound serving, transaction relay, production-funds wallet use, migration
  apply mode, packaging, hosted dashboard, GUI, Windows service support, and
  unattended production-node operation each require separate future milestones
  and fresh threat models before becoming shipped claims.

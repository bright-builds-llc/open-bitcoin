# v1.5 Threat Model and Release Boundaries

## Scope

This document is the reviewer-facing threat model for the v1.5 Unattended
Mainnet Node Operation Readiness milestone. It covers the shipped explicit
opt-in unattended mainnet operator review workflow, bounded daemon sync loop,
resource bounds, recovery states, long-run truth surfaces, launchd/systemd
service supervision, same-datadir service restart/resume evidence, redacted
support evidence, compatibility wrapper reports, deterministic local
verification, and the parity roots that bind those claims together.

Phase 67 is a documentation, parity-root, and deterministic verification
closeout. It adds no new authentication, network service, transaction relay,
wallet production use, migration apply mode, hosted upload, packaging
distribution, Windows service integration, or GUI behavior. Public-network
long-run checks and real service-manager commands remain explicit operator UAT
and stay outside `bash scripts/verify.sh`.

The v1.5 claim is source-built, explicit opt-in extended unattended mainnet
operator review readiness. It can show bounded unattended loop behavior,
documented stop/retry/recovery states, consistent long-run truth surfaces,
user-level service lifecycle management, service-supervised same-datadir
restart/resume evidence, redacted support bundles, compatibility harness
reports, or typed diagnosed blockers with next operator action. It is not an
inbound serving, transaction relay, compact block relay, production-funds wallet
use, migration apply mode, packaging distribution, hosted dashboard, GUI,
Windows service support, or broad production-node support claim.

## Assets

| Asset | Why It Matters | Evidence Surface |
| --- | --- | --- |
| Unattended sync loop control | Reviewers need the daemon loop to remain explicit, bounded, pausable, resumable, and shutdown-aware. | Phase 60 summaries, durable sync status, `open-bitcoind`, runtime guide |
| Resource bounds and recovery taxonomy | Long-running review must stay bounded and surface typed recovery actions instead of free-form errors. | `sync.resource_pressure`, `sync.recovery_category`, support evidence |
| Long-run truth surfaces | Status, dashboard, RPC, metrics, logs, support, and live-smoke reports must agree on the same sync facts. | `OpenBitcoinStatusSnapshot`, metrics/log projections, live-smoke snapshots |
| Service supervision lifecycle | launchd/systemd operator workflows must stay user-scoped and reviewable without packaged-service claims. | `open-bitcoin service preview|install|start|stop|restart|status`, service docs |
| Service restart/resume evidence | Restart review must prove the same Open Bitcoin datadir and preserve durable progress. | `service.restart_resume`, same-datadir status, Phase 64 evidence |
| Support evidence and redaction | Local bundles must summarize diagnostics without leaking credentials, wallet material, raw logs, or raw peer tables. | `support-evidence.json`, `support-evidence.md`, redaction summaries |
| Compatibility wrapper reports | Operators need deterministic compatibility reports without calling Rust internals or implying public probing. | `compatibility-harness-report.json`, `compatibility-harness-report.md` |
| Deterministic verification and parity roots | Default verification and reviewer roots must fail on missing v1.5 claim boundaries. | `bash scripts/verify.sh`, `scripts/check-v1.5-release-boundaries.ts`, parity docs |

## Trust Boundaries

| Boundary | Inputs | Expected Control |
| --- | --- | --- |
| Operator config -> unattended daemon loop | `open-bitcoin.jsonc`, `-openbitcoinsync=mainnet-ibd`, manual peers, DNS seeds, bounds | Explicit mainnet-only activation, bounded cycles, durable pause/resume/shutdown state, typed stop reasons |
| Public peer input -> sync runtime | Manual peers, DNS peers, wire messages, invalid headers, malformed blocks, idle peers | Validation-gated progress, no-credit failures, bounded retries, recovery categories |
| Sync runtime -> operator truth surfaces | Durable status, metrics, structured logs, support summaries, live-smoke reports | Shared typed fields, explicit `Unavailable` reasons, bounded samples and compact summaries |
| Local service manager -> operator workflow | launchd/systemd user commands, generated unit/plist files, selected datadir/config path | Preview-first service definitions, user-scope actions, typed lifecycle labels, no real manager calls in default verification |
| Same datadir -> restart review | Pre-restart state, service restart action, post-restart durable state | Same-datadir evidence, clean/unclean shutdown classification, stale in-flight verdict, durable progress preservation |
| Local artifacts -> support/reviewer package | Live-smoke reports, compatibility reports, metrics/log paths, support bundle output | Allowlisted summaries, redaction, generated artifacts kept out of git |
| Evidence docs -> release claim | Threat model, release-readiness matrix, parity index, checklist, runtime guide | v1.5-specific roots, explicit non-claims, historical v1.3/v1.4 evidence preserved |

## STRIDE Threat Register

| Threat ID | STRIDE | Component | Scenario | Mitigation / Evidence | Residual Risk / Future Gate |
| --- | --- | --- | --- | --- | --- |
| V15-TM-01 | Spoofing, Repudiation, Denial of Service | Unattended sync loop control | An operator or reviewer mistakes daemon startup, elapsed time, or repeated cycles for production-node readiness. | Activation stays explicit through `sync.network_enabled = true` plus `sync.mode = "mainnet-ibd"` or `-openbitcoinsync=mainnet-ibd`; durable status records lifecycle, phase, stop reasons, pause/resume, and shutdown state. | Production full-node support requires future PRODNODE-01 evidence and a fresh threat model. |
| V15-TM-02 | Denial of Service | Resource bounds and recovery taxonomy | Long-running review exhausts peer, header, block, metric, log, durable write, retry, or support evidence bounds. | `SyncResourcePressure`, bounded metrics/log retention, no unbounded retry queues, and stable `sync.recovery_category` labels expose resource pressure and next actions. | Broader production resource governance and peer eviction policy remain future scope. |
| V15-TM-03 | Repudiation, Spoofing | Long-run truth surfaces | Status, dashboard, RPC, metrics, logs, live-smoke, and support summaries disagree, hiding a stopped, waiting, failed, or unavailable state. | Phase 62 field contract keeps lifecycle, phase, targets, attempts, progress, stop reason, recovery, pressure, peer health, and block evidence tied to shared typed status. | Future surfaces that expose sync truth must join the shared contract before being claimed. |
| V15-TM-04 | Tampering, Repudiation | Service supervision lifecycle | A generated launchd/systemd flow supervises the wrong binary, wrong datadir, wrong config, or implies a packaged production service. | Service preview/install renders `open-bitcoind`, selected datadir/config paths, user-scope manager commands, and lifecycle labels before side effects. | Signed packaging, machine-wide services, Windows services, and production uptime policy require later milestones. |
| V15-TM-05 | Tampering, Repudiation | Service restart/resume evidence | Service restart uses the wrong datadir, loses durable progress, duplicates block work, or treats fresh network progress as mandatory proof. | `service.restart_resume` records service manager, same-datadir verdict, prior shutdown, durable progress, stale in-flight verdict, recovery category, and next action. Fresh post-restart progress is optional stronger UAT, not required default verification. | Automatic recovery loops and production restart policies remain future gates. |
| V15-TM-06 | Information Disclosure | Support evidence and local reports | Support bundles or compatibility reports leak credentials, cookie contents, wallet material, raw daemon logs, raw wire payloads, raw endpoint tables, or unbounded local artifacts. | Support and compatibility reports use allowlisted compact summaries, redaction metadata, and explicit omitted categories. Raw reports, logs, datadirs, and generated evidence stay local and out of git. | Hosted support upload or artifact signing needs a separate data-protection design. |
| V15-TM-07 | Repudiation, Spoofing | Compatibility wrapper reports | Operators treat a deterministic transcript scenario or report existence as proof that a live public peer was contacted or that inbound/relay behavior is complete. | `open-bitcoin compatibility harness` reports supplied peer endpoint labels, scenario, diagnosis, redaction boundaries, and next action while delegating diagnosis to `open-bitcoin-network::evaluate_transcript`. | Real public-peer probing, inbound serving, relay, compact blocks, and peer-governance policy need future scoped phases. |
| V15-TM-08 | Repudiation, Denial of Service | Deterministic verification and parity roots | Default verification accidentally starts public-network/service-manager work, or parity roots omit v1.5 boundaries while docs still imply readiness. | `scripts/check-v1.5-release-boundaries.ts` validates REL-01 through REL-04 roots, required evidence paths, deferred-surface wording, and forbidden default-verification strings in `scripts/verify.sh`. | Public-network CI or release-blocking live evidence remains a future release-policy decision. |

## ASVS L1 Mapping

This mapping uses OWASP ASVS v5.0.0 as a reviewer vocabulary for local evidence
handling. It is not a certification claim and does not invent product scope.

| ASVS v5.0.0 Area | v1.5 Local Mapping | Evidence |
| --- | --- | --- |
| Input validation and sanitization | Public peer input remains validation-gated before progress credit; report inputs are allowlisted before rendering. | V15-TM-02, V15-TM-06, V15-TM-07 |
| Logging and error privacy | Support and compatibility evidence summarize diagnostics without raw logs, raw wire payloads, credentials, or wallet material. | V15-TM-06, `support-evidence.json`, `compatibility-harness-report.md` |
| Configuration and secrets management | Config paths and credential sources are metadata-only; Open Bitcoin-only sync settings live in JSONC, not `bitcoin.conf`. | V15-TM-01, V15-TM-04, `docs/architecture/config-precedence.md` |
| Access-control limits for local-only artifacts | Phase 67 adds no remote administration, auth expansion, hosted upload, or public dashboard. Artifacts remain local files generated by explicit operator commands. | V15-TM-06, V15-TM-08 |

## Evidence Acceptance

Accepted v1.5 evidence is field-first and local-artifact based:

1. Deterministic local verification:
   - `bash scripts/verify.sh` passes without invoking public-network live smoke,
     manual peer probing, `--restart-after-progress`, `systemctl --user`, or
     `launchctl`.
   - `bun run scripts/check-v1.5-release-boundaries.ts` proves the parity roots,
     release docs, threat model, and default-verification boundary.
2. Unattended review loop evidence:
   - Status and docs show explicit activation, bounded cycles, pause/resume,
     shutdown, stop reasons, retry/backoff, and no hot-loop behavior.
3. Long-run and recovery evidence:
   - Operator surfaces expose lifecycle, phase, configured targets, attempt
     counters, progress signals, stop reasons, recovery category/action,
     resource pressure, peer health, and downloaded/connected block evidence.
4. Service supervision and restart evidence:
   - Service docs and status expose user-scope launchd/systemd lifecycle,
     generated service files, selected datadir/config paths, and
     `service.restart_resume`.
5. Support and compatibility evidence:
   - `support-evidence.json`, `support-evidence.md`,
     `compatibility-harness-report.json`, and `compatibility-harness-report.md`
     summarize diagnostics without raw sensitive or unbounded material.

Required local commands and artifact names are documented in
`docs/operator/runtime-guide.md` and `docs/parity/release-readiness.md`.

## Release Boundary Matrix

| Surface | v1.5 Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements |
| --- | --- | --- | --- | --- | --- |
| v1.5 unattended operator review | Source-built operators can opt into bounded unattended mainnet review and inspect durable, service, support, compatibility, and recovery evidence. | `bash scripts/verify.sh`, v1.5 threat/release docs, runtime guide, Phase 60-66 verification artifacts. | Not broad production-node support or a production full-node service guarantee. | Future PRODNODE-01. | REL-01, REL-02, REL-03, REL-04 |
| Unattended sync loop | The daemon loop is explicit, bounded, pausable, resumable, and shutdown-aware. | Phase 60 evidence, durable sync status, runtime guide. | Not default-on public-network operation or unattended production service. | Future production-node milestone. | REL-01, REL-02 |
| Resource bounds and recovery | Operators can inspect resource bounds, pressure, recovery categories, and next actions. | Phase 61 evidence, `sync.resource_pressure`, `sync.recovery_category`, support docs. | Not a guarantee of unlimited long-run operation. | Future resource-governance policy. | REL-01 |
| Long-run truth surfaces | Status, dashboard, RPC, metrics, logs, live smoke, and support evidence share the same truth fields. | Phase 62 evidence, `OpenBitcoinStatusSnapshot`, architecture docs. | Not a hosted dashboard or remote admin claim. | Future hosted operations design. | REL-01 |
| Service supervision | Operators can preview and manage user-scope launchd/systemd supervision. | Phase 63 evidence, service docs, service status output. | Not signed packaging, machine-wide service support, Windows service support, or uptime policy. | PKG-01 and PKG-02. | REL-01, REL-02 |
| Service restart/resume | Operators can review same-datadir service restart/resume evidence. | Phase 64 evidence, `service.restart_resume`, status JSON. | Not automatic recovery-loop or production restart policy. | Future PRODNODE-01 recovery gate. | REL-01 |
| Support bundle/operator review | Operators can collect redacted local support evidence and follow repo-local review commands. | Phase 65 evidence, `support-evidence.json`, `support-evidence.md`. | Support bundles are not release validators, hosted uploads, or public-mainnet proof by themselves. | Future artifact validator or hosted support design. | REL-01, REL-02 |
| Compatibility wrapper | Operators can generate local compatibility transcript reports through the CLI wrapper. | Phase 66 evidence, `compatibility-harness-report.json`, `compatibility-harness-report.md`. | Report existence is not live public-peer contact, inbound serving, relay, or production-node proof. | Future public-peer probing and relay milestones. | REL-01, REL-02 |
| Deterministic verification | Default verification stays deterministic and public-network/service-manager free. | `scripts/check-v1.5-release-boundaries.ts`, `scripts/verify.sh`. | No public-network CI gate, default live-smoke run, real `systemctl --user`, or real `launchctl` call. | Future release-policy decision. | REL-03, REL-04 |
| Inbound serving | No shipped v1.5 claim. | Deferred-surface docs and release matrix rows. | v1.5 does not claim inbound serving or address advertisement. | Future PRODNODE-02. | REL-02 |
| Transaction relay | No shipped v1.5 claim. | Deferred-surface docs and release matrix rows. | v1.5 does not claim transaction relay or mempool propagation. | Future PRODNODE-03. | REL-02 |
| Compact block relay | No shipped v1.5 claim. | Deferred-surface docs and release matrix rows. | v1.5 does not claim compact block relay. | Future relay parity milestone. | REL-02 |
| Production-funds wallet use | No shipped v1.5 claim. | Deferred-surface docs and release matrix rows. | v1.5 does not claim production-funds wallet use. | Future WALPROD-01. | REL-02 |
| Migration apply mode | No shipped v1.5 claim. | Dry-run migration docs and release matrix rows. | v1.5 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future MIGAPPLY-01. | REL-02 |
| Packaging distribution | No shipped v1.5 claim. | Source-built install docs and release matrix rows. | v1.5 does not claim signed packages or OS-native distribution. | Future PKG-01. | REL-02 |
| Hosted dashboard | No shipped v1.5 claim. | Local dashboard docs and release matrix rows. | v1.5 does not claim hosted dashboard or public dashboard operation. | Future hosted operations design. | REL-02 |
| GUI | No shipped v1.5 claim. | Headless and terminal-first docs. | v1.5 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | REL-02 |
| Windows service support | No shipped v1.5 claim. | Source-built local service docs and release matrix rows. | v1.5 does not claim Windows service support or certification. | Future PKG-02. | REL-02 |

## Requirements Traceability

| Requirement | v1.5 Trace |
| --- | --- |
| REL-01 | This document and `docs/parity/release-readiness.md` cover unattended sync loop behavior, service supervision, long-run evidence, resource bounds, recovery states, support redaction, and compatibility wrapper output. |
| REL-02 | The release boundary matrix, parity roots, P2P catalog, and deviations register distinguish v1.5 operator-review readiness from deferred production, inbound, relay, wallet, migration, packaging, hosted-dashboard, GUI, Windows-service, and production-node claims. |
| REL-03 | `scripts/verify.sh` remains deterministic and excludes public-network long-run checks, manual peers, restart-after-progress, and real service-manager commands. |
| REL-04 | `scripts/check-v1.5-release-boundaries.ts` fails when v1.5 docs or parity roots omit the unattended-operation claim boundaries. |

## Residual Risks

- Public-network conditions remain operator-environment dependent. Opt-in
  long-run reports are evidence only when they preserve typed diagnosis, status
  snapshots, peer outcomes, and next actions.
- Service-manager behavior differs by OS and user environment. Default
  verification uses deterministic fakes and docs/checkers; real manager commands
  are opt-in UAT.
- Support and compatibility reports are local review artifacts. Hosted upload,
  public sharing, signing, or artifact validation needs fresh data-protection
  and release-engineering design.
- Inbound serving, transaction relay, compact block relay, production-funds
  wallet use, migration apply mode, packaging distribution, hosted dashboard,
  GUI, Windows service support, and production full-node operation each require
  future milestones and fresh threat models before becoming shipped claims.

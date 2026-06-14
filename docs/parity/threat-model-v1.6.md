# v1.6 Threat Model and Release Boundaries

## Scope

This document is the reviewer-facing threat model for the v1.6 Mainnet
Full-Sync Completion milestone. It covers the shipped source-built, explicit
opt-in full-sync completion evidence for `open-bitcoind`: active-chain
validation, durable restart/resume state, best-known tip and stay-current
reporting, reorg/no-progress recovery, bounded resource behavior, shared status
evidence, redacted support bundles, opt-in UAT commands, deterministic local
verification, and parity roots that bind those claims together.

Phase 74 is a documentation, parity-root, and deterministic verification
closeout. It adds no authentication surface, network listener, inbound serving,
address relay, block serving, transaction relay, compact block relay,
production-funds wallet safety, migration apply mode, signed packaging,
Windows service support, GUI parity, hosted dashboards, public-network CI,
release-blocking live sync, or broad production-node readiness.

The v1.6 claim is source-built, explicit opt-in full-sync completion evidence.
It can show validated active-chain progress to the best-known peer tip,
durable restart/resume continuity, stay-current review, reorg handling,
typed no-progress recovery, resource bounds, shared status fields, redacted
support evidence, and documented opt-in UAT commands. It is not a production
full-node or production-funds wallet claim.

## Assets

| Asset | Why It Matters | Evidence Surface |
| --- | --- | --- |
| Active-chain validation | Full-sync completion only counts after consensus validation and durable connection to the active chain. | Phase 68 verification, chainstate tests, durable status |
| Tip tracking and stay-current state | Operators need the best-known tip and freshness state to distinguish caught-up, stale, and recovering nodes. | Phase 69 verification, `sync.best_known_tip`, status fields |
| Reorg and no-progress recovery | Long mainnet review must survive competing branches and peer failures with typed next actions. | Phase 70 verification, `sync.latest_reorg`, `sync.no_progress_diagnosis` |
| Resource bounds and restart/resume | Operators need bounded runtime envelopes and safe same-datadir recovery. | Phase 71 verification, `sync.resource_pressure`, restart/resume matrix |
| Observability and support evidence | Status, dashboard, RPC, metrics, logs, live-smoke, and support bundles must agree on the same full-sync truth. | Phase 72 verification, `OpenBitcoinStatusSnapshot`, support evidence |
| Opt-in UAT and deterministic verification | Public-mainnet UAT must remain explicit while default verification stays local. | Phase 73 verification, runtime guide UAT matrix, `scripts/check-phase73-uat-verification.ts` |
| v1.6 release-boundary roots | Reviewers need one current machine-readable and human-readable closeout. | `docs/parity/index.json`, checklist, release-readiness, this threat model |

## Trust Boundaries

| Boundary | Inputs | Expected Control |
| --- | --- | --- |
| Operator activation -> daemon sync loop | `-openbitcoinsync=mainnet-ibd`, selected datadir, config, manual peers, DNS seeds | Activation remains explicit, bounded, and reviewable; default verification never starts public-network sync. |
| Public peers -> sync runtime | Headers, blocks, peer stalls, malformed data, invalid data, `notfound`, disconnects | Validation-gated progress, peer-attributed failures, retry/backoff, and typed recovery categories. |
| Durable store -> restart/resume evidence | Header store, block index, UTXO/undo data, runtime metadata, recovery markers | Same-datadir reopen, durable active-chain state, duplicate-connect prevention, storage recovery actions. |
| Sync runtime -> operator truth surfaces | CLI status, dashboard, RPC, metrics, logs, support, live-smoke reports | Shared full-sync fields and explicit unavailable or no-progress reasons. |
| Local artifacts -> reviewer evidence | Live-smoke reports, support bundles, compatibility reports, metrics/log paths | Redacted summaries, local-only artifact paths, no checked-in live-mainnet reports. |
| Release docs -> shipped claim | README, runtime guide, parity roots, release-readiness, threat model | v1.6-specific roots, explicit non-claims, prior milestone evidence preserved as historical context. |

## STRIDE Threat Register

| Threat ID | STRIDE | Component | Scenario | Mitigation / Evidence | Residual Risk / Future Gate |
| --- | --- | --- | --- | --- | --- |
| V16-TM-01 | Spoofing, Repudiation | Full-sync completion claim | A reviewer mistakes daemon startup, peer reachability, elapsed time, or report existence for sync-to-tip completion. | Docs and checker require validated active-chain progress, best-known-tip evidence, stay-current state, and explicit evidence interpretation. | Future production-node readiness needs a separate milestone and threat model. |
| V16-TM-02 | Tampering, Repudiation | Durable active-chain state | Restart/resume evidence could be accepted without durable UTXO/undo/block-index continuity. | Phase 68 and Phase 71 evidence require durable active-chain, UTXO/undo, block index, runtime metadata, and same-datadir reopen coverage. | Broader cache-flush and multi-chainstate policy remain future scope. |
| V16-TM-03 | Spoofing, Denial of Service | Best-known tip and stay-current reporting | A stale or recovering node could be presented as current. | Phase 69 exposes best-known tip source, freshness, current/stale/recovering states, and peer agreement evidence. | Public-network freshness remains operator-environment dependent. |
| V16-TM-04 | Tampering, Denial of Service | Reorg and peer recovery | Malformed peers, stale in-flight work, or reorgs could hide failed progress or unsafe branch replacement. | Phase 70 evidence covers cumulative-work branch selection, peer-attributed no-credit failures, bounded backoff, and typed next actions. | Future peer-governance and banning policy need separate scope. |
| V16-TM-05 | Denial of Service | Resource bounds and storage pressure | Long sync attempts could exhaust queues, caches, logs, metrics, support evidence, or disk without actionable diagnosis. | Phase 71 evidence records bounded peers, in-flight work, caches, retention, and storage-pressure recovery guidance. | Production resource governance remains future scope. |
| V16-TM-06 | Information Disclosure | Support and live evidence | Support bundles or live-smoke summaries could leak credentials, wallet material, raw logs, raw endpoint tables, or unchecked local artifacts. | Phase 72 support evidence uses redacted compact summaries and keeps generated live-mainnet reports, logs, metrics stores, support bundles, and datadirs out of git. | Hosted support upload or signed artifact sharing needs a separate data-protection design. |
| V16-TM-07 | Repudiation | Opt-in UAT and deterministic verification | Public-mainnet full-sync, manual-peer, or restart-after-progress commands could drift into default verification or become release-blocking live sync. | Phase 73 checker and runtime guide keep UAT explicit, repo-local, and outside `bash scripts/verify.sh`. | Public-network CI is a future release-policy decision. |
| V16-TM-08 | Repudiation, Elevation of Privilege | Release-boundary roots | Docs or status surfaces could imply inbound serving, relay, production-wallet, migration apply, packaging, GUI, hosted-dashboard, public-network CI, or broad production-node readiness. | `scripts/check-v1.6-release-boundaries.ts` validates v1.6 roots, REL-01 through REL-03, all 26 requirement ids, deferred-scope wording, and forbidden default-verification strings. | Each deferred production-adjacent surface requires a future scoped phase and fresh threat model before becoming a shipped claim. |

## ASVS L1 Mapping

This mapping uses OWASP ASVS v5.0.0 as reviewer vocabulary for local evidence
handling. It is not a certification claim and does not expand product scope.

| ASVS v5.0.0 Area | v1.6 Local Mapping | Evidence |
| --- | --- | --- |
| Input validation and output encoding | Public peer data only counts after validation-gated header, block, and active-chain handling; reports render compact allowlisted fields. | V16-TM-01, V16-TM-02, V16-TM-04 |
| Logging and error privacy | Support and live-smoke evidence summarize diagnostics without raw logs, credentials, wallet material, or raw endpoint tables. | V16-TM-06, support evidence docs |
| Configuration and secrets management | Mainnet sync activation stays explicit and Open Bitcoin-only settings remain separate from baseline `bitcoin.conf` ownership. | V16-TM-01, runtime guide, config precedence docs |
| Secure deployment limits | v1.6 adds no remote admin, hosted dashboard, signed packaging, Windows service support, public-network CI, or production-node claim. | V16-TM-07, V16-TM-08 |

## Evidence Acceptance

Accepted v1.6 evidence is field-first and local-artifact based:

1. Deterministic local verification:
   - `bash scripts/verify.sh` passes without public-network live smoke, manual
     peer probing, `--restart-after-progress`, real `systemctl --user`, real
     `launchctl`, or mainnet IBD activation.
   - `bun run scripts/check-v1.6-release-boundaries.ts` proves v1.6 parity
     roots, release docs, threat model, README/operator docs, and default
     verification boundaries.
2. Full-sync completion evidence:
   - Status and reports show validated active-chain progress, connected height,
     best-known tip, cumulative work, tip freshness, and stay-current or
     diagnosed no-progress state.
3. Restart/resume and recovery evidence:
   - Same-datadir reopen preserves durable headers, block index, active-chain
     state, UTXO/undo data, runtime metadata, restart checkpoints, reorg
     evidence, and typed recovery guidance.
4. Operator evidence:
   - Runtime guide UAT commands, support bundles, live-smoke reports, metrics,
     logs, dashboard, RPC, and CLI status share the same full-sync truth
     interpretation.
5. Release-boundary evidence:
   - Parity roots, checklist, release-readiness, README, runtime guide, and
     catalog pages preserve the deferred-scope non-claims.

Required local commands and artifact names are documented in
`docs/operator/runtime-guide.md` and `docs/parity/release-readiness.md`.

## Release Boundary Matrix

| Surface | v1.6 Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements |
| --- | --- | --- | --- | --- | --- |
| v1.6 full-sync completion | Source-built operators can explicitly opt into mainnet full-sync review and inspect validated active-chain progress to the best-known peer tip. | Phase 68 through Phase 73 evidence, release-readiness, runtime guide, checker. | Not broad production-node readiness or a packaged production service. | Future production-node milestone. | REL-01, REL-02, REL-03 |
| Active-chain validation and persistence | Sync progress counts after consensus validation and durable active-chain connection. | Phase 68 verification, chainstate parity tests, durable status fields. | Not block serving or assumeutxo/snapshot shortcutting. | Future production-node chainstate policy. | REL-01 |
| Tip tracking and stay-current operation | Operators can inspect best-known tip, freshness, current/stale/recovering states, and stay-current progress. | Phase 69 verification, status truth contract. | Not a public-network timing SLA or release-blocking live sync gate. | Future release-policy decision. | REL-01, REL-03 |
| Reorg, peer rotation, and no-progress recovery | Operators can inspect reorg evidence, peer-attributed failures, retry/backoff, no-progress causes, and next actions. | Phase 70 verification, `sync.latest_reorg`, `sync.no_progress_diagnosis`. | Not peer banning, address relay, transaction relay, or compact block relay. | Future peer-governance and relay milestones. | REL-01, REL-02 |
| Resource bounds and restart/resume | Long sync attempts have documented/tested bounds and same-datadir recovery evidence. | Phase 71 verification, resource pressure, restart/resume matrix. | Not unlimited unattended operation or production resource policy. | Future production resource governance. | REL-01 |
| Observability and support evidence | CLI, dashboard, RPC, metrics, logs, live-smoke, and support surfaces share one full-sync truth contract. | Phase 72 verification, architecture docs, support evidence. | Not hosted dashboards, remote admin, raw support upload, or production-wallet safety. | Future hosted operations and wallet milestones. | REL-01, REL-03 |
| Opt-in UAT and deterministic verification | Public-mainnet commands are copy-pasteable UAT; default verification remains deterministic. | Phase 73 verification, runtime guide UAT matrix, Phase 73 checker. | Not public-network CI or release-blocking live sync. | Future release-policy decision. | REL-02, REL-03 |
| v1.6 parity roots | Reviewers can inspect current threat model, release-readiness matrix, machine-readable roots, README, runtime guide, and catalogs. | This document, `release-readiness.md`, `index.json`, checklist, README, runtime guide, checker. | Historical v1.3-v1.5 docs are evidence, not the current v1.6 claim. | Future milestone roots when scope expands. | REL-01, REL-02, REL-03 |
| Inbound serving and address relay | No shipped v1.6 claim. | Deferred-scope docs and checker assertions. | v1.6 does not claim inbound serving or address relay. | Future PNODE-01/PNODE-02. | REL-02 |
| Block serving, transaction relay, compact block relay | No shipped v1.6 claim. | Deferred-scope docs and checker assertions. | v1.6 does not claim block serving, transaction relay, or compact block relay. | Future PNODE-02/PNODE-03. | REL-02 |
| Production-funds wallet safety | No shipped v1.6 claim. | Deferred-scope docs and checker assertions. | v1.6 does not claim production-funds wallet safety. | Future wallet production milestone. | REL-02 |
| Migration apply mode | No shipped v1.6 claim. | Dry-run migration docs and deferred-scope rows. | v1.6 does not claim migration apply mode or source-datadir mutation. | Future migration apply safety design. | REL-02 |
| Signed packaging and Windows service support | No shipped v1.6 claim. | Source-built docs and deferred-scope rows. | v1.6 does not claim signed packaging, OS distribution, or Windows service support. | Future packaging milestones. | REL-02 |
| GUI parity and hosted dashboards | No shipped v1.6 claim. | Headless docs and deferred-scope rows. | v1.6 does not claim GUI parity or hosted dashboards. | Future product-surface milestones. | REL-02 |

## Requirements Traceability

| Requirement | v1.6 Trace |
| --- | --- |
| REL-01 | This document, `docs/parity/release-readiness.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, and `docs/operator/runtime-guide.md` describe the explicit opt-in full-sync completion claim and link Phase 68 through Phase 73 evidence. |
| REL-02 | The release boundary matrix, parity roots, catalog pages, deviations register, README, runtime guide, and `scripts/check-v1.6-release-boundaries.ts` preserve non-claims for inbound serving, relay, production-wallet safety, migration apply mode, packaging, GUI, hosted dashboards, public-network CI, and broad production-node readiness. |
| REL-03 | The runtime guide and release-readiness docs explain shipped sync-to-tip evidence, opt-in UAT commands, support evidence locations, failure interpretation, and deferred scope. |

## Residual Risks

- Public-network sync-to-tip evidence depends on operator environment, reachable
  peers, and current network conditions. Default verification remains
  deterministic and does not attempt to prove live mainnet freshness.
- Generated live-mainnet reports, support bundles, daemon logs, metrics stores,
  compatibility reports, and local datadirs stay local and out of git.
- Inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-funds wallet safety, migration apply mode, signed
  packaging, Windows service support, GUI parity, hosted dashboards,
  public-network CI, release-blocking live sync, and broad production-node
  readiness each require future scoped milestones before becoming shipped
  claims.

# v1.3 Threat Model and Release Boundaries

## Scope

This document is the reviewer-facing threat model for the v1.3 Public Mainnet
Sync Proof and Node Hardening milestone. It covers the shipped opt-in daemon
sync review path, local support evidence, release claim boundaries, and the
Phase 50 evidence acceptance contract.

Phase 49 is documentation and reviewer traceability only. It does not add
runtime behavior, support schema, public-network default verification, hosted
CI network checks, or checked-in live-report fixtures. Public-network checks
remain opt-in and outside `bash scripts/verify.sh`.

The v1.3 claim is source-built, opt-in, local-evidence public-mainnet sync
proof and node-hardening review. It is not a production-node, production-funds
wallet, inbound-serving, transaction-relay, migration-apply, packaging,
hosted/public dashboard, GUI, or unattended production-node claim.

## Assets

| Asset | Why It Matters | Evidence Surface |
| --- | --- | --- |
| Validated sync progress | Reviewers need evidence that headers, blocks, and restart/resume progress are separated from mere endpoint reachability. | `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`, `OpenBitcoinStatusSnapshot` |
| Durable sync and store state | Corrupt, incompatible, partial, or second-writer state can invalidate public-mainnet evidence. | `OpenBitcoinStatusSnapshot`, store-health evidence in `support-evidence.json` |
| Public peer input | Public peers can fail, stall, disconnect, send invalid data, or provide no useful contribution. | Endpoint outcomes, peer contribution rows, Phase 42 through Phase 46 summaries |
| Operator RPC controls | Pause, resume, status, and support collection must not imply unsafe offline mutation or unauthenticated public control. | `docs/operator/runtime-guide.md`, status snapshot contract |
| Local support evidence | Shared support artifacts must preserve diagnosis without leaking secrets or broadening the release claim. | `support-evidence.json`, `support-evidence.md` |
| Release and parity docs | Reviewers need one auditable path from requirements to evidence and explicit non-claims. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md` |

## Trust Boundaries

| Boundary | Inputs | Expected Control |
| --- | --- | --- |
| Public peer input -> sync runtime | DNS seed endpoints, manual peers, wire messages, peer failures, invalid data | Bounded outbound peer targets, typed endpoint outcomes, validation-gated contribution, invalid-data attribution |
| Sync runtime -> durable store | Headers, block bodies, chainstate progress, peer metadata, lifecycle state | Separated header/downloaded/connected progress, restart recovery, store-health and recovery guidance |
| Operator CLI/RPC -> sync controls | Status, pause, resume, support bundle commands | Live-RPC-first control path, offline second-writer refusal, metadata-only credential reporting |
| Local machine -> reviewer artifact | Live-smoke reports, support bundles, status snapshots, logs, metrics | Allowlisted summaries, redaction summary, local-only reports, no checked-in live fixtures |
| Roadmap requirements -> release claims | PROOF-06, SEC-01, SEC-02, Phase 50 success criteria | Claim boundary matrix, future gates, explicit non-claims, deterministic doc checks |

## STRIDE Threat Register

| Threat ID | STRIDE | Asset | Trust Boundary | Scenario | Mitigation / Evidence | Residual Risk / Future Gate |
| --- | --- | --- | --- | --- | --- | --- |
| V13-TM-01 | Tampering, Denial of Service | Public peer input and validated sync progress | Public peer input -> sync runtime | A public peer sends invalid headers or blocks, stalls after connection, or stays active without useful progress. | Phase 44 counts useful contribution only after accepted headers or preserved blocks. Phase 46 rejects invalid peer data without saving bad blocks or advancing active chainstate. Live-smoke reports keep endpoint outcomes and runtime peer contribution rows separate. | Future inbound serving, peer eviction policy, and transaction relay require fresh threat modeling before claim expansion. |
| V13-TM-02 | Denial of Service | Runtime resource bounds | Public peer input -> sync runtime -> durable store | Repeated peer failures or large sync work exhausts in-flight request, block, metric, log, or durable-write capacity. | Phase 45 documents and projects max header requests, headers per message, per-peer and total block in-flight caps, max messages per peer, max sync rounds, target outbound peers, bounded metrics, and bounded structured logs. | Long unattended production-node operation remains a future gate because v1.3 does not claim unlimited long-run public service behavior. |
| V13-TM-03 | Tampering, Denial of Service | Durable sync and store state | Sync runtime -> durable store | Partial downloads, invalid data, incompatible stores, corrupt stores, or second writers make evidence misleading. | Phase 45 offline mutating controls refuse unclean active owners. Phase 46 separates validated header height, downloaded block height, and connected chainstate height, and records recovery guidance for storage and peer failures. | Future destructive migration apply mode and production service cutover need a backup-aware design and separate safety review. |
| V13-TM-04 | Spoofing, Elevation of Privilege, Tampering | Operator RPC controls | Operator CLI/RPC -> sync controls | An operator command appears to control sync when credentials are missing, stale, or routed to the wrong state source. | Runtime guide documents normal RPC auth sources. Status and dashboard preserve `Unavailable` reasons. Offline pause/resume refuse second-writer mutation when live RPC is unavailable and durable state is unclean. | Public remote administration, ACL policy, and unattended daemon supervision are deferred beyond v1.3. |
| V13-TM-05 | Information Disclosure | Log/report redaction and local support evidence | Local machine -> reviewer artifact | A shared support bundle or live-smoke example leaks RPC credentials, cookie values, private wallet material, seed phrases, raw logs, or raw live-smoke input. | Phase 48 support bundles write `support-evidence.json` and `support-evidence.md` with credential metadata only, omitted secret categories, allowlisted live-smoke summary fields, and redacted sensitive text. | Hosted support upload, raw log sharing, or production-funds wallet evidence would need a new data-protection design. |
| V13-TM-06 | Repudiation, Spoofing | Live evidence handling and release claims | Roadmap requirements -> release claims | Reviewers mistake DNS/TCP reachability, support-bundle existence, or a no-progress report for completed header/block/restart proof. | This document and `release-readiness.md` require Phase 50 evidence to show observed header/block/restart-resume progress or a diagnosed environment/network blocker with typed no-progress cause, endpoint outcomes, status snapshots, and next operator action. Support bundles are local redacted evidence only. | Phase 50 closed through diagnosed blocker evidence in `50-UAT.md`; future successful progress evidence still requires a new operator run with a reachable peer. |

## Evidence Acceptance Criteria

Phase 50 evidence is artifact-first. The accepted paths are:

1. Observed header, block, and restart/resume progress:
   - Live-smoke JSON/Markdown shows header or block delta, peer endpoint,
     source, timestamp, and before/after status snapshots.
   - Restart/resume evidence uses the same datadir and shows coherent durable
     `OpenBitcoinStatusSnapshot` progress across runs.
2. Diagnosed environment/network blocker:
   - The live-smoke report records a typed no-progress cause.
   - Endpoint outcomes show DNS, TCP, handshake, capability, validation,
     storage, timeout, or operator-cancellation state.
   - Status snapshots show header height, block height, lifecycle, phase,
     outbound peers, and latest error when available.
   - The report records the next operator action.

Required local commands and evidence surfaces:

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support \
  --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

Accepted artifact names:

- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
- `support-evidence.json`
- `support-evidence.md`
- `OpenBitcoinStatusSnapshot`

The support bundle helps reviewers correlate config sources, redaction
metadata, store health, status snapshots, and allowlisted live-smoke summary
fields. It does not by itself prove public-mainnet readiness.

## Release Boundary Matrix

| Surface | v1.3 Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements / Phases |
| --- | --- | --- | --- | --- | --- |
| Public-mainnet sync evidence | v1.3 provides source-built, opt-in, local evidence for public-mainnet sync progress review or diagnosed blocker review. | `bash scripts/verify.sh`, live-smoke JSON/Markdown, status snapshots, support evidence, Phase 50 UAT. | v1.3 does not claim public-network checks run by default. | Phase 50 closes with observed progress or diagnosed blocker. | PROOF-06, SEC-01, SEC-02, Phase 49, Phase 50 |
| Outbound peer resilience | v1.3 documents bounded outbound peer attempts, retry/backoff state, replacement, and useful contribution attribution. | Phase 42 through Phase 44 summaries, live-smoke endpoint outcomes, peer contribution rows. | v1.3 does not claim inbound serving or address advertisement. | Future production-node milestone. | PEER-01 through PEER-04, SEC-01 |
| Resource bounds and durable recovery | v1.3 documents bounded runtime resources, second-writer diagnostics, separated durable progress, and invalid-data recovery guidance. | Phase 45 and Phase 46 summaries, `OpenBitcoinStatusSnapshot`, runtime guide. | v1.3 does not claim unattended long-run production operation. | Future PRODNODE readiness gate. | NODE-01 through NODE-05, SEC-01 |
| Operator RPC controls | v1.3 documents local operator status, pause, resume, and support bundle workflows with live-RPC-first semantics. | Runtime guide, status snapshot contract, support evidence. | v1.3 does not claim remote hosted administration or public RPC control. | Future auth/ACL and service-supervision gate. | OBS-01, OBS-02, SEC-01 |
| Redacted support evidence | v1.3 support bundles are local redacted evidence for review and troubleshooting. | `support-evidence.json`, `support-evidence.md`, redaction summary, Phase 48 summary. | v1.3 does not claim support bundles are release validators or public-mainnet proof by themselves. | Future artifact validator or hosted support design. | OBS-03, OBS-04, SEC-01 |
| Inbound serving | No shipped v1.3 claim. | Deferred-surface docs and release boundary rows. | v1.3 does not claim inbound peer serving or address advertisement. | Future PRODNODE-02 phase. | SEC-02 |
| Transaction relay | No shipped v1.3 claim. | Deferred-surface docs and release boundary rows. | v1.3 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03 phase. | SEC-02 |
| Production-funds wallet use | No shipped v1.3 claim. | Deferred-surface docs and release boundary rows. | v1.3 does not claim production-funds wallet use. | Future WALPROD-01 threat model and parity evidence. | SEC-02 |
| Migration apply mode | No shipped v1.3 claim. | Dry-run migration docs and release boundary rows. | v1.3 does not claim migration apply mode, source-service cutover, or source-datadir mutation. | Future MIGAPPLY-01 design. | SEC-02 |
| Packaging or signed installers | No shipped v1.3 claim. | Source-built install docs and release boundary rows. | v1.3 does not claim packaged or signed installer readiness. | Future PKG-01 phase. | SEC-02 |
| Hosted/public dashboard | No shipped v1.3 claim. | Local terminal/dashboard docs and release boundary rows. | v1.3 does not claim a hosted/public dashboard. | Future hosted operations design. | SEC-02 |
| GUI | No shipped v1.3 claim. | Headless and terminal-first docs. | v1.3 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | SEC-02 |
| Unattended production-node readiness | No shipped v1.3 claim. | Deferred-surface docs and release boundary rows. | v1.3 does not claim unattended production-node readiness. | Future PRODNODE-01 phase with long-run evidence. | SEC-02 |

## Requirement Traceability

| Requirement | Phase 49 Trace | Phase 50 Acceptance Evidence |
| --- | --- | --- |
| PROOF-06 | This document and `docs/parity/release-readiness.md` define repo-local commands, artifact names, blocker evidence, and the rule that public-network checks remain outside `bash scripts/verify.sh`. | `bash scripts/verify.sh`, live-smoke JSON/Markdown, support evidence JSON/Markdown, status snapshots, and Phase 50 UAT. |
| SEC-01 | The STRIDE register covers public peer input, resource exhaustion, storage corruption, operator RPC controls, log/report redaction, and live evidence handling. | Phase 50 must show that accepted evidence uses the documented artifacts or records a diagnosed blocker with typed cause and next operator action. |
| SEC-02 | The release boundary matrix names the v1.3 proven claim, explicit non-claims, future gates, and related phases. | Phase 50 must not widen the shipped claim beyond opt-in local public-mainnet evidence or a diagnosed environment/network blocker. |

## Residual Risks And Future Gates

- Phase 50 closed live public-mainnet evidence through the diagnosed-blocker
  path in `50-UAT.md`; future successful progress evidence still needs a new
  operator run with a reachable peer.
- Public-network conditions are operator-environment dependent, so a valid
  blocker must preserve typed no-progress cause, endpoint outcomes, status
  snapshots, and next operator action.
- Production-node operation, inbound serving, transaction relay,
  production-funds wallet use, migration apply mode, packaging,
  hosted/public dashboard, GUI work, and unattended operation each need a
  future scoped phase and fresh threat model before becoming shipped claims.

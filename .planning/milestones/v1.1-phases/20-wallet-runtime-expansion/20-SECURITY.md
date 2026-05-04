---
phase: 20
slug: wallet-runtime-expansion
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-04
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-05-04T09:10:09Z
---

# Phase 20 - Security

Per-phase security contract for Phase 20 wallet runtime expansion. This audit verifies only the declared Phase 20 threat register and does not scan for new threats.

## Scope

This audit covers only the declared Phase 20 threat register from
`20-01-PLAN.md` through `20-05-PLAN.md`, their implementation surfaces, the
phase summaries, the phase verification artifact, and the Phase 20 UAT result.
It does not broaden into unrelated repository threats or a general
vulnerability scan.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| descriptor text -> wallet core | Untrusted descriptor strings cross into pure parsing and validation | descriptor text, range/cursor metadata |
| send intent -> build/sign logic | Operator-provided fee and change inputs influence transaction funding | recipients, fee intent, fee ceiling, change policy |
| rescan progress -> status semantics | Partial progress must not be misreported as fresh | scan heights, target tip, scanning state |
| pure wallet snapshot -> Fjall codec | Domain state crosses into durable serialization | wallet snapshots, descriptors, UTXOs |
| wallet name selection -> registry lookup | Untrusted selection metadata chooses mutation target | wallet names, selected-wallet metadata |
| rescan checkpoint -> restart recovery | Persisted progress must survive restart without replay ambiguity | target tip, cursor height, freshness state |
| CLI startup flags -> transport endpoint | User-controlled wallet names influence request path selection | `-rpcwallet` values |
| HTTP path -> RPC dispatch | Request URI determines whether root or wallet scope is active | `/` and `/wallet/<name>` paths |
| RPC params -> wallet mutations | Mutating wallet methods consume untrusted fee, address, and rescan inputs | JSON-RPC params |
| durable wallet state -> status surfaces | Internal freshness and scan progress become operator-visible output | wallet status fields |
| filesystem metadata -> read-only detector | External wallet paths are untrusted and must not be mutated | datadir/config/cookie/wallet metadata |
| detector output -> migration planning docs | Classification data informs later user decisions | product, chain, wallet-format hints |
| operator input -> wallet send preview/confirm | Human-facing wrapper controls whether mutation proceeds | send args and confirmation flag |
| backup destination path -> export writer | User-supplied paths may overlap external wallet locations | backup destination path |
| shipped docs -> operator expectations | Parity claims influence whether users trust this wallet surface | README and parity catalog claims |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status | Evidence |
|-----------|----------|-----------|-------------|------------|--------|----------|
| T-20-01 | Tampering | wallet/build.rs send-intent validation | mitigate | Reject invalid/unresolvable estimate requests, fee-ceiling violations, and change policy issues with typed wallet errors. | closed | `packages/open-bitcoin-wallet/src/wallet.rs:123`, `packages/open-bitcoin-wallet/src/wallet.rs:149`, `packages/open-bitcoin-wallet/src/error.rs:38`, `packages/open-bitcoin-wallet/src/error.rs:40`, `packages/open-bitcoin-wallet/src/wallet/build.rs:108` |
| T-20-02 | Information Disclosure | wallet.rs address allocation | mitigate | Persist cursor state in wallet-domain snapshot contracts. | closed | `packages/open-bitcoin-wallet/src/wallet.rs:331`, `packages/open-bitcoin-wallet/src/wallet.rs:423`, `packages/open-bitcoin-wallet/src/wallet.rs:425`, `packages/open-bitcoin-node/src/wallet.rs:280`, `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs:173` |
| T-20-03 | Integrity | wallet/scan.rs freshness math | mitigate | Encode explicit fresh/partial/scanning state in pure types. | closed | `packages/open-bitcoin-wallet/src/wallet.rs:182`, `packages/open-bitcoin-wallet/src/wallet/scan.rs:107`, `packages/open-bitcoin-wallet/src/wallet/scan.rs:115`, `packages/open-bitcoin-wallet/src/wallet/scan.rs:123`, `packages/open-bitcoin-wallet/src/wallet/scan.rs:129` |
| T-20-04 | Tampering | wallet_registry.rs | mitigate | Enforce unique wallet names and selected-wallet validation before mutation or lookup. | closed | `packages/open-bitcoin-node/src/wallet_registry.rs:144`, `packages/open-bitcoin-node/src/wallet_registry.rs:251`, `packages/open-bitcoin-node/src/wallet_registry.rs:287`, `packages/open-bitcoin-node/src/wallet_registry.rs:323`, `packages/open-bitcoin-node/src/wallet_registry.rs:341` |
| T-20-05 | Availability | fjall_store rescan checkpoints | mitigate | Persist target tip and next cursor after bounded progress and cover reopen behavior. | closed | `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:20`, `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:119`, `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:131`, `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:132`, `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:316` |
| T-20-06 | Integrity | snapshot_codec/wallet.rs | mitigate | Version and round-trip range/cursor fields. | closed | `packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs:104`, `packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs:108`, `packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs:191`, `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs:163`, `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs:173` |
| T-20-07 | Tampering | http.rs wallet path routing | mitigate | Parse wallet name from `/wallet/<name>`, keep scope outside JSON params, and enforce root-vs-wallet allowlists. | closed | `packages/open-bitcoin-rpc/src/http.rs:89`, `packages/open-bitcoin-rpc/src/http.rs:218`, `packages/open-bitcoin-rpc/src/http.rs:269`, `packages/open-bitcoin-rpc/src/http.rs:287`, `packages/open-bitcoin-cli/src/client.rs:363` |
| T-20-08 | Elevation of Privilege | dispatch.rs method ownership | mitigate | Reject wallet-scoped methods when wallet selection is missing or ambiguous. | closed | `packages/open-bitcoin-rpc/src/method.rs:129`, `packages/open-bitcoin-rpc/src/http.rs:283`, `packages/open-bitcoin-rpc/src/context/wallet_state.rs:74`, `packages/open-bitcoin-rpc/src/context/wallet_state.rs:365`, `packages/open-bitcoin-rpc/src/context/wallet_state.rs:379` |
| T-20-09 | Integrity | sendtoaddress / rescanblockchain dispatch | mitigate | Reuse typed send-intent validation and persisted rescan job state instead of adapter-local mutation. | closed | Accepted risk `R-20-01`: rescan persistence is present at `packages/open-bitcoin-rpc/src/context/rescan.rs:155` and `packages/open-bitcoin-rpc/src/context/rescan.rs:169`, but `sendtoaddress` still constructs `BuildRequest` directly at `packages/open-bitcoin-rpc/src/dispatch/wallet.rs:46` and resolves fee policy in `packages/open-bitcoin-rpc/src/dispatch/wallet.rs:361` instead of the `SendIntent` path in `packages/open-bitcoin-wallet/src/wallet.rs:102`. |
| T-20-10 | Information Disclosure | status.rs wallet freshness | mitigate | Surface freshness and scanning explicitly. | closed | `packages/open-bitcoin-node/src/status.rs:163`, `packages/open-bitcoin-node/src/status.rs:172`, `packages/open-bitcoin-node/src/status.rs:181`, `packages/open-bitcoin-node/src/status.rs:445`, `docs/architecture/status-snapshot.md:49` |
| T-20-11 | Tampering | operator/detect.rs external wallet inspection | mitigate | Keep inspection read-only and test unchanged bytes/timestamps/permissions. | closed | `packages/open-bitcoin-cli/src/operator/detect.rs:21`, `packages/open-bitcoin-cli/src/operator/detect.rs:342`, `packages/open-bitcoin-cli/src/operator/detect/tests.rs:127`, `packages/open-bitcoin-cli/src/operator/detect/tests.rs:196`, `packages/open-bitcoin-cli/src/operator/detect/tests.rs:203` |
| T-20-12 | Repudiation | wallet parity/docs | mitigate | Update parity catalog and status contract docs for inspection boundary. | closed | `docs/architecture/status-snapshot.md:17`, `docs/architecture/status-snapshot.md:51`, `docs/parity/catalog/wallet.md:35`, `docs/parity/catalog/wallet.md:63`, `docs/parity/index.json:704` |
| T-20-13 | Tampering | operator wallet send flow | mitigate | Require preview/confirmation before invoking mutating commit path. | closed | `packages/open-bitcoin-cli/src/operator/wallet.rs:46`, `packages/open-bitcoin-cli/src/operator/wallet.rs:105`, `packages/open-bitcoin-cli/src/operator/wallet.rs:138`, `packages/open-bitcoin-cli/src/operator/wallet.rs:144`, `packages/open-bitcoin-cli/src/operator/wallet/tests.rs:154` |
| T-20-14 | Tampering | backup export destination | mitigate | Reject destinations overlapping external wallet candidates and keep export format Open Bitcoin-owned. | closed | `packages/open-bitcoin-cli/src/operator/wallet.rs:44`, `packages/open-bitcoin-cli/src/operator/wallet.rs:163`, `packages/open-bitcoin-cli/src/operator/wallet.rs:204`, `packages/open-bitcoin-cli/src/operator/wallet.rs:508`, `packages/open-bitcoin-cli/src/operator/wallet/tests.rs:223` |
| T-20-15 | Repudiation | parity/docs closeout | mitigate | Update parity catalog/index and README with shipped slice and deferrals. | closed | `docs/parity/catalog/wallet.md:25`, `docs/parity/catalog/wallet.md:33`, `docs/parity/catalog/wallet.md:81`, `docs/parity/index.json:44`, `README.md:187`, `README.md:213` |

## Open Threats

No open threats.

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| R-20-01 | T-20-09 | The Phase 20 shipped RPC path already preserves persisted rescan job state and passes the declared verification/UAT coverage, but `sendtoaddress` still resolves `BuildRequest` and fee policy in the RPC adapter instead of reusing the shared typed `SendIntent` path. This is accepted for Phase 20 so the shipped behavior can stand while a later follow-up unifies the RPC adapter with the pure wallet send-intent contract. | user | 2026-05-04 |

## Unregistered Flags

None. The Phase 20 summary files do not contain `## Threat Flags` sections.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-04 | 15 | 15 | 0 | gsd-security-auditor |

## Verification Evidence

| Command / Input | Result |
|-----------------|--------|
| Phase 20 threat-model review across `20-01-PLAN.md` through `20-05-PLAN.md` | Passed; 15 declared threats extracted, no summary `## Threat Flags` sections found. |
| Cross-check against `20-01-SUMMARY.md` through `20-05-SUMMARY.md`, `20-VERIFICATION.md`, and `20-UAT.md` | Passed; behavioral claims and verification coverage aligned with 14 closed threats. |
| Targeted implementation audit across wallet, node, RPC, and CLI files named in the threat register | Passed for 14 direct mitigations; `T-20-09` was accepted as risk `R-20-01` after user approval because the RPC path still constructs adapter-local `BuildRequest` state instead of reusing the typed `SendIntent` path. |
| `git diff --check -- .planning/milestones/v1.1-phases/20-wallet-runtime-expansion/20-SECURITY.md` | Passed. |

## Standards Inputs

Materially applied local `AGENTS.md`, `AGENTS.bright-builds.md`,
`standards-overrides.md`, Bright Builds pinned `standards/index.md`,
`standards/core/verification.md`, `standards/core/testing.md`, and the
`gsd-secure-phase` workflow. ASVS Level: 1.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-04

---
phase: "133"
slug: "package-aware-download-and-orphan-bridge"
status: verified
threats_total: 22
threats_closed: 22
threats_open: 0
asvs_level: 1
created: "2026-07-26"
---

# Phase 133 — Security

> Verification of the 22 threats declared across the four Phase 133 plans. This
> is a threat-model conformance audit, not an unrestricted vulnerability scan.

## Trust Boundaries

| Boundary | Description | Data Crossing |
| --- | --- | --- |
| Remote peer → transaction download state | Untrusted transaction announcements and bodies enter bounded reject-evidence and orphan state. | Wtxids, transaction bodies, peer provenance, missing-parent identities |
| Network crate → node admission shell | The network crate emits an opaque, same-peer one-parent/one-child candidate; the node shell alone constructs and submits the Phase 132 package. | Ordered transaction pair, aligned peer origins, receive provenance |
| Package admission → lifecycle consumers | The Phase 133 bridge returns the exact report and delta without projecting package effects into serving, fanout, compact-candidate, or storage state. | Package fingerprint, member outcomes, lifecycle delta |

## Threat Register

| Threat ID | Category | Component | Disposition | Verification Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| T-133-01 | Denial of Service | Reject-evidence filters | mitigate | Fixed `120_000` capacity, `0.000_001` false-positive target, three generations, and fixed word allocation are defined in `packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs:13`; the one-million-insertion allocation oracle is at `packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs:211`. | closed |
| T-133-02 | Spoofing | Reject-evidence identity APIs | mitigate | Hard evidence accepts only `Wtxid` at `packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs:238`; reconsiderable keys are the typed transaction/package enum at `packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs:300`. No raw transaction bytes or untyped txid API is exposed. | closed |
| T-133-03 | Repudiation / Denial of Service | Download scheduler | mitigate | Reject evidence maps only to `SuppressRecentReject` in `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:380`; the peer oracle verifies suppression without punishment, and the parity contract states no punishment or disconnection at `docs/parity/catalog/mempool-policy.md:467`. | closed |
| T-133-04 | Tampering | Active-tip evidence reset | mitigate | The paired reset is centralized in `packages/open-bitcoin-network/src/peer.rs:331` and invoked only after successful local, stored, and reorg tip transitions at `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:49`, `:107`, and `:130`. Positive and negative transition tests begin at `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs:550` and `:588`. | closed |
| T-133-05 | Information Disclosure | Reject-evidence tweak and membership | accept | Accepted-risk entry AR-133-05 below records the planned ephemeral, non-persisted, non-operator-evidence boundary and suppression-only consequence. | closed |
| T-133-10 | Spoofing | Same-peer candidate proof | mitigate | Candidate fields are crate-private at `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs:26`; construction aligns both origins to the qualifying parent peer at `:140`, and wrong-peer eligibility is rejected by `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs:669`. | closed |
| T-133-11 | Denial of Service | Orphan bodies and announcers | mitigate | One canonical orphan body is retained under global, per-peer, announcer, TTL, and `40_000_000`-byte limits in `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:29` and `:33`; late announcers are guarded at `:305`. Shared-body, peer-cap, and aggregate-byte oracles are at `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs:458`, `:527`, and `:553`. | closed |
| T-133-12 | Denial of Service | Candidate traversal | mitigate | Children are indexed newest-first with `Reverse<u64>` at `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:209`; the cursor stops at the configured reconsideration cap in `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs:129`, verified at `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs:710`. | closed |
| T-133-13 | Tampering | Orphan reverse indexes | mitigate | Cleanup is centralized through `remove_orphan` at `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:565`; the reconstructed-index oracle is at `:592`, with disconnect, expiry, eviction, and rejection coverage at `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs:913`. | closed |
| T-133-14 | Repudiation | Child reject suppression | mitigate | Child hard/reconsiderable evidence is consumed as local work suppression through `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:380`; no punishment, score, or disconnect effect is produced. | closed |
| T-133-20 | Spoofing | Candidate provenance | mitigate | The opaque candidate contains ordered members and aligned origins at `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs:26`; construction sets `[parent_peer; 2]` at `:142`, and provenance assertions are exercised at `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs:617`. | closed |
| T-133-21 | Tampering | Package identity | mitigate | The node bridge creates one `WellFormedPackage`, reads its cached fingerprint, checks reconsiderable evidence, converts that same checked package to `SubmissionPackage`, and asserts the returned report identity at `packages/open-bitcoin-node/src/network/admission_bridge/package.rs:212`–`:236`. | closed |
| T-133-22 | Tampering / Elevation of Privilege | Package-policy authority | mitigate | The candidate bridge contains one authoritative `self.mempool.submit_package` call at `packages/open-bitcoin-node/src/network/admission_bridge/package.rs:225`; the node adapter delegates to the Phase 132 engine at `packages/open-bitcoin-node/src/mempool.rs:64`. The checker enforces exactly one call at `scripts/check-phase133-package-aware-download-orphan-bridge.ts:268`. | closed |
| T-133-23 | Denial of Service | Failed-package candidate loop | mitigate | Partial/failed package fingerprints are recorded as reconsiderable at `packages/open-bitcoin-node/src/network/admission_bridge.rs:313`; candidate admission checks the cached fingerprint before submission at `packages/open-bitcoin-node/src/network/admission_bridge/package.rs:214`, while bounded cursor fallback is verified at `packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs:434`. | closed |
| T-133-24 | Repudiation / Denial of Service | Probabilistic evidence | mitigate | Both evidence domains feed suppression-only scheduler and package-candidate decisions; the no-punishment oracle is enforced by `scripts/check-phase133-package-aware-download-orphan-bridge.ts:148` and the parity contract at `docs/parity/catalog/mempool-policy.md:467`. | closed |
| T-133-25 | Tampering | Lifecycle projection | mitigate | Package results are collected separately in `package_admissions` at `packages/open-bitcoin-node/src/network/action_translation.rs:200` and `:287`. The exact report/delta and unchanged serving, fanout, compact, and storage state are verified by `packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs:103` and `:195`. | closed |
| T-133-30 | Denial of Service | Adversarial resource corpus | mitigate | Fixed-allocation million-insert, shared-body churn, late-announcer cap, byte-budget, identity-only cursor, traversal-cap, and cleanup-oracle tests are present in the Phase 133 Rust suites. The post-review bounds are carried by commits `9f284662` and `4c67e41b`; all 74 focused network tests passed during this audit. | closed |
| T-133-31 | Spoofing | Provenance and identity claims | mitigate | The deterministic checker enforces candidate privacy, same-peer aligned origins, cached fingerprints, wrong-peer exclusion, and canonical child lookup in `scripts/check-phase133-package-aware-download-orphan-bridge.ts`; its 30 mutation tests passed during this audit. | closed |
| T-133-32 | Tampering | Package-policy authority guard | mitigate | The checker rejects duplicate authoritative admission at `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts:188` and enforces the single call at `scripts/check-phase133-package-aware-download-orphan-bridge.ts:268`. Typed singleton rejection categories are preserved by commit `c40edd7e`. | closed |
| T-133-33 | Repudiation | Probabilistic-result claims | mitigate | Docs state that filter membership is suppression-only and cannot punish or disconnect at `docs/parity/catalog/mempool-policy.md:467`; checker mutation coverage rejects removal of the no-punishment oracle at `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts:76`. | closed |
| T-133-34 | Information Disclosure / Scope Leakage | Public claims | mitigate | The checker rejects general-wire, later-phase, public/default, guaranteed-propagation, and production claims at `scripts/check-phase133-package-aware-download-orphan-bridge.ts:476`. README and package docs retain the deferred-scope boundary. | closed |
| T-133-35 | Tampering | Verifier wiring | mitigate | The mutation suite and live checker appear in both verifier ordering surfaces at `scripts/verify.sh:424`–`:425` and `:582`–`:583`; commit `e27a9a8d` strengthened the guard semantics, and clean re-review commit `e115943c` confirms no remaining code-review findings. | closed |

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Controls and Revisit Trigger | Accepted By | Date |
| --- | --- | --- | --- | --- | --- |
| AR-133-05 | T-133-05 | The probabilistic filter tweak and membership state are ephemeral node-shell state, are neither persisted nor exposed as operator evidence, and can only suppress redundant local work. The bounded false-positive behavior is an intentional Knots-inspired tradeoff. | Fixed memory and false-positive parameters, active-tip reseeding, typed identities, and suppression-only effects. Revisit if membership or tweak data becomes persisted, remotely queryable, operator-visible, or capable of peer punishment. | Phase 133 threat model | 2026-07-26 |

## Unregistered Flags

None. The summaries report no new attack surface or unmapped threat flags.

## Post-Review Evidence

| Commit | Verified Security-Relevant Change |
| --- | --- |
| `9f284662` | Bounds persistent candidate cursors to child identities and the aggregate retained-byte budget. |
| `4c67e41b` | Enforces the per-peer orphan cap for late announcers. |
| `c40edd7e` | Preserves exact typed singleton policy-rejection categories. |
| `e27a9a8d` | Strengthens resource-guard and verifier mutation semantics. |
| `e115943c` | Records a clean code re-review after all four fixes. |

All five commits are ancestors of the audited `HEAD`.

## Verification Run

| Check | Result |
| --- | --- |
| Phase 133 checker mutation suite | 30 passed, 0 failed |
| Phase 133 live checker | passed |
| Focused network transaction-relay tests | 74 passed, 0 failed |
| Focused node package-bridge tests | 7 passed, 0 failed |
| Focused node mempool-lifecycle tests | 17 passed, 0 failed |
| `git diff --check` before report creation | passed |

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | ASVS Level | Run By |
| --- | --- | --- | --- | --- | --- |
| 2026-07-26 | 22 | 22 | 0 | 1 | Codex GSD security auditor |

## Sign-Off

- [x] All 22 threats classified by disposition.
- [x] All 21 mitigations verified against implementation, tests, docs, or verifier guards.
- [x] Accepted risk T-133-05 documented with rationale, controls, and revisit trigger.
- [x] Summary threat flags reviewed; no unregistered flags found.
- [x] `threats_open: 0` confirmed.
- [x] `status: verified` set in frontmatter.

**Approval:** verified 2026-07-26

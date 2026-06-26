---
phase: 92-address-advertisement-and-discovery-boundaries
verified: "2026-06-26T10:43:40Z"
status: passed
score: "6/6 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: "2026-06-26T10:43:40Z"
lifecycle_validated: true
overrides_applied: 0
---

# Phase 92: Address Advertisement and Discovery Boundaries Verification Report

**Phase Goal:** Add privacy-aware listener advertisement and bounded address request/management behavior without claiming full address relay or broader public-network discovery parity.
**Verified:** 2026-06-26T10:43:40Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Local address candidate selection respects configured listener addresses, routability, reachability, and privacy-network boundaries. | VERIFIED | `packages/open-bitcoin-network/src/address/advertisement.rs` derives candidates from `InboundListenerEndpoint` and runtime-bound evidence only; `address.rs` classifies IPv4/IPv6 routability and unsupported privacy/future networks with stable labels. `maybe_version_sender_address` uses the same policy before allowing a nonzero version sender address. |
| 2 | Bounded `getaddr` response behavior is deterministic, permission-aware, and capped by count, age, cache, source, and repeated-request state. | VERIFIED | `packages/open-bitcoin-network/src/address/response.rs` uses `PHASE92_GETADDR_RESPONSE_LIMIT`, inbound permission effects, `AddressResponseCache::from_sources`, and `GetAddrRequestState.served`. `packages/open-bitcoin-network/src/peer/address_boundary.rs` sends direct `Addr` responses only for eligible inbound `getaddr` requests and records suppression reasons otherwise. |
| 3 | Learned addresses enter a typed address-management contract with routability, source, freshness, and persistence-boundary evidence. | VERIFIED | `packages/open-bitcoin-network/src/address/book.rs` records `LearnedAddressEntry` fields for network kind, source, freshness, services, routability, and `persistence_eligible`, with deterministic rejection for stale/future, duplicate, unroutable, unsupported, invalid-port, and over-cap batches. |
| 4 | Docs and release checks distinguish local listener advertisement, address request responses, peer discovery, and full address relay. | VERIFIED | `docs/parity/catalog/p2p.md`, `docs/parity/index.json`, `docs/operator/runtime-guide.md`, and `docs/architecture/status-snapshot.md` keep Phase 92 scoped to listener advertisement, bounded direct `getaddr`, and learned-address evidence. `scripts/check-phase92-address-boundaries.ts` rejects positive claims for full address relay, peer discovery, public-network defaults, or production readiness. |
| 5 | Code-review warning WR-01 is fixed: empty-payload commands reject trailing payload. | VERIFIED | `packages/open-bitcoin-network/src/message.rs:253` through `:256` route `verack`, `wtxidrelay`, `sendheaders`, and `getaddr` through `decode_empty_message`; `decode_empty_payload` calls `Cursor::finish()` at `message.rs:437`. `packages/open-bitcoin-network/src/message/tests.rs:81` verifies a non-empty payload is rejected for all four commands. |
| 6 | Code-review warning WR-02 is fixed: over-cap learned `addr` rejection count projects through managed status. | VERIFIED | `packages/open-bitcoin-network/src/peer/address_boundary.rs` carries `learned_address_rejection_count` separately from bounded rejection samples and increments it for over-cap batches. `packages/open-bitcoin-node/src/network/inbound.rs:78` projects the aggregate count into `ManagedAddressBoundaryInfo.learned_address_rejections`. Regression tests cover both the peer evidence and managed-status projection. |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-network/src/address.rs` | Shared address-boundary vocabulary | VERIFIED | Stable decision, reason, routability, source, and network-kind labels exist and are exported. |
| `packages/open-bitcoin-network/src/address/advertisement.rs` | Pure local listener advertisement policy | VERIFIED | Uses configured listener/runtime evidence and suppresses non-routable or unsupported privacy/future networks. |
| `packages/open-bitcoin-network/src/address/book.rs` | Learned-address contract | VERIFIED | Provides typed entries, deterministic learning decisions, over-cap rejection accounting, and persistence eligibility evidence. |
| `packages/open-bitcoin-network/src/address/response.rs` | Bounded `getaddr` response policy | VERIFIED | Uses capped cache selection, permission gating, age/source inputs, and served-once state. |
| `packages/open-bitcoin-network/src/message.rs` | Wire support for `getaddr` and legacy `addr` only | VERIFIED | Empty-payload commands reject trailing bytes; `addrv2`/`sendaddrv2` remain unknown. |
| `packages/open-bitcoin-network/src/peer/address_boundary.rs` | Peer-manager address-boundary handling | VERIFIED | Handles inbound `getaddr` and `addr` as evidence/direct response paths without unsolicited relay or fanout. |
| `packages/open-bitcoin-node/src/network/inbound.rs` | Managed status projection | VERIFIED | Projects local advertisement, bounded getaddr, learned entries, aggregate learned rejections, and latest decision into shared status structures. |
| `packages/open-bitcoin-rpc/src/context/address_boundary.rs` and `network.rs` | Runtime/RPC wiring | VERIFIED | Converts listener evidence into advertisement decisions and projects bounded address evidence through Open Bitcoin network status without raw address leakage. |
| `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` and `support/render/inbound.rs` | Operator status/support rendering | VERIFIED | Renders bounded counts and stable labels; support guidance explicitly states peer discovery, unsolicited relay, DNS seed discovery, UPnP/NAT-PMP, and public-network readiness are out of scope. |
| `docs/parity/*`, `docs/operator/runtime-guide.md`, `docs/architecture/*` | Documentation and parity evidence | VERIFIED | Phase 92 surface is registered with ADDR-01 through ADDR-04 and scoped no-claim language. |
| `scripts/check-phase92-address-boundaries.ts` and `.test.ts` | Deterministic release checker | VERIFIED | Fixed corpus checker covers requirements, breadcrumbs, UAT commands, no-claim boundaries, raw-evidence boundaries, and verifier order. |
| `scripts/verify.sh` | Default verifier wiring | VERIFIED | Runs the Phase 92 checker tests and checker after Phase 91 and before pure-core checks. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| Listener runtime evidence | Local advertisement policy | `LocalAdvertisementInput` and `InboundListenerEndpoint` | WIRED | RPC context converts configured/bound listener evidence into policy inputs, then forwards decisions into the managed peer network. |
| Local advertisement decisions | Version sender address | `maybe_version_sender_address` and peer version response | WIRED | Nonzero sender addresses are emitted only when the typed advertisement policy returns an advertised candidate. |
| Inbound `getaddr` message | Bounded response selection | `handle_getaddr` -> `select_getaddr_response` | WIRED | Eligible inbound peers receive one capped direct `addr`; repeated, outbound, permission-denied, or empty-cache requests are suppressed with stable evidence. |
| Inbound `addr` message | Learned address book | `handle_addr` -> `LearnedAddressBook::learn_batch` | WIRED | Accepted and rejected learned-address decisions update peer evidence without sending relay actions. |
| Peer address evidence | Managed node status | `PeerAddressBoundaryEvidence` -> `ManagedAddressBoundaryInfo` | WIRED | `address_boundary_evidence()` is converted in `packages/open-bitcoin-node/src/network.rs`, including aggregate over-cap rejection counts. |
| Managed status | RPC, CLI status, and support bundle | Shared inbound status structs | WIRED | Open Bitcoin status and support renderers consume the shared status projection rather than renderer-local address summaries. |
| Docs/parity surface | Default verifier | Phase 92 checker in `scripts/verify.sh` | WIRED | The checker validates docs, parity metadata, source breadcrumbs, UAT command forms, no-claim text, and raw-evidence redaction. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `address/advertisement.rs` | `LocalAdvertisementDecision` | Configured listener endpoints plus runtime-bound listener evidence | Yes | FLOWING |
| `peer/address_boundary.rs` | `getaddr_responses`, `getaddr_suppressions` | Parsed inbound `getaddr`, permission evidence, local candidates, learned entries, and per-peer served state | Yes | FLOWING |
| `address/book.rs` | `LearnedAddressEntry` and rejection decisions | Parsed legacy `addr` payload announcements | Yes | FLOWING |
| `network/inbound.rs` | `ManagedAddressBoundaryInfo` | `PeerAddressBoundaryEvidence` from the peer manager | Yes | FLOWING |
| CLI status/support renderers | Displayed bounded counts and latest decision | Shared Open Bitcoin inbound status snapshot | Yes | FLOWING |
| Phase 92 checker | Corpus validation failures | Real repo docs, parity JSON, source breadcrumbs, renderers, and `scripts/verify.sh` | Yes | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Phase 92 checker test suite | `bun test scripts/check-phase92-address-boundaries.test.ts` | 11 passed, 0 failed | PASS |
| Phase 92 release-boundary checker | `bun run scripts/check-phase92-address-boundaries.ts` | Printed `validated Phase 92 address advertisement and discovery boundary evidence` | PASS |
| Empty-payload commands reject trailing payload | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network empty_payload_messages_reject_non_empty_payload --no-fail-fast` | 1 passed, 0 failed | PASS |
| Over-cap `addr` batch rejection counted at peer boundary | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network over_cap_addr_batch_records_batch_rejection_without_partial_inserts --no-fail-fast` | 1 passed, 0 failed | PASS |
| Over-cap rejection count projects through managed status | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_address_boundary_info_projects_over_cap_addr_rejections --no-fail-fast` | 1 passed, 0 failed | PASS |
| Repo-native verification contract | `bash scripts/verify.sh` | Completed successfully in 2m 19.314s | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| ADDR-01 | 92-01, 92-04, 92-06, 92-07, 92-08, 92-09 | The node can derive local listen address candidates and advertise only configured, reachable, and privacy-safe addresses according to scoped Knots parity rules. | SATISFIED | Pure advertisement policy, version sender gating, runtime listener evidence wiring, status/support projection, docs, and checker coverage are present and verified. |
| ADDR-02 | 92-02, 92-03, 92-04, 92-05, 92-06, 92-07, 92-08, 92-09 | The node can answer inbound address requests within bounded cache, count, age, and permission rules without claiming full address-relay network participation. | SATISFIED | `getaddr` decode, response cache, permission gating, served-once suppression, direct `addr` response, status/support evidence, no-relay docs, and checker coverage are present. |
| ADDR-03 | 92-01, 92-03, 92-04, 92-05, 92-06, 92-07, 92-08, 92-09 | Learned peer addresses enter a typed address-management contract with routability, source, freshness, and persistence boundaries that can be verified deterministically. | SATISFIED | Learned address book entries and rejection reasons are typed; over-cap rejection counts are preserved; managed status and tests verify aggregate projection. |
| ADDR-04 | 92-08, 92-09 | Documentation and release checks distinguish local listener advertisement, inbound `getaddr` response behavior, peer discovery, and full address relay. | SATISFIED | Parity docs, operator guide, architecture docs, checker fixtures, and default verifier wiring enforce scoped Phase 92 wording and reject broader claims. |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| None | N/A | N/A | N/A | No blocker anti-patterns found. Grep hits were benign empty match arms, checker helper `return []`, CLI checker success `console.log`, or tests/fixtures. |

### Human Verification Required

None. Phase 92 behavior is deterministic, local, public-network-free, service-manager-free, and covered by code inspection plus repo-native automated verification.

### Gaps Summary

No blockers found. The implementation satisfies the Phase 92 goal and ADDR-01 through ADDR-04 without broadening into full address relay or broader peer discovery. The two code-review warnings in `92-REVIEW.md` are fixed and covered by focused regression tests. Two gsd-tools frontmatter checks produced false positives during verification because implementation names were split across actual module boundaries and support text uses capitalized renderer labels, but manual wiring and behavioral checks verified those paths.

---

_Verified: 2026-06-26T10:43:40Z_
_Verifier: the agent (gsd-verifier)_

---
phase: 160-explicit-bounded-peer-filter-serving
reviewed: "2026-10-11T04:12:20Z"
depth: standard
files_reviewed: 123
files_reviewed_list:
  - AGENTS.md
  - README.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/fixtures/peer-filter-proof.json
  - docs/parity/fixtures/peer-filter-resource-measurements.json
  - docs/parity/fixtures/peer-filter-wire.json
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-consensus/src/chain_proof.rs
  - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
  - packages/open-bitcoin-consensus/src/lib.rs
  - packages/open-bitcoin-network/src/filter_serving.rs
  - packages/open-bitcoin-network/src/filter_serving/capability.rs
  - packages/open-bitcoin-network/src/filter_serving/capability/tests.rs
  - packages/open-bitcoin-network/src/filter_serving/policy.rs
  - packages/open-bitcoin-network/src/filter_serving/policy/tests.rs
  - packages/open-bitcoin-network/src/filter_serving/resource.rs
  - packages/open-bitcoin-network/src/filter_serving/resource/tests.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/proof.rs
  - packages/open-bitcoin-network/src/header_store/proof/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests.rs
  - packages/open-bitcoin-network/src/inbound/permissions.rs
  - packages/open-bitcoin-network/src/inbound/tests/permission_policy_cases.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/message.rs
  - packages/open-bitcoin-network/src/message/filter.rs
  - packages/open-bitcoin-network/src/message/filter/tests.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs
  - packages/open-bitcoin-network/src/resource.rs
  - packages/open-bitcoin-network/src/resource/tests.rs
  - packages/open-bitcoin-network/tests/parity.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/filter_transport.rs
  - packages/open-bitcoin-node/src/network/filter_transport/encoded_size.rs
  - packages/open-bitcoin-node/src/network/filter_transport/encoded_size/tests.rs
  - packages/open-bitcoin-node/src/network/filter_transport/pending.rs
  - packages/open-bitcoin-node/src/network/filter_transport/pending/tests.rs
  - packages/open-bitcoin-node/src/network/filter_transport/registry.rs
  - packages/open-bitcoin-node/src/network/filter_transport/tests.rs
  - packages/open-bitcoin-node/src/network/limited_serve.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/owner.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests/families.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/filter_capability.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
  - packages/open-bitcoin-node/src/sync/session/filter_serving.rs
  - packages/open-bitcoin-node/src/sync/tcp.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs
  - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/support_transport.rs
  - packages/open-bitcoin-node/src/sync/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed/race.rs
  - scripts/basic-filter-oracle.py
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase134-authoritative-lifecycle.test/mutations.ts
  - scripts/check-phase134-authoritative-lifecycle.ts
  - scripts/check-phase160-peer-filter-serving.test.ts
  - scripts/check-phase160-peer-filter-serving.ts
  - scripts/generate-peer-filter-fixtures.test.ts
  - scripts/generate-peer-filter-fixtures.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings:
  critical: 0
  warning: 3
  info: 0
  total: 3
status: clean
review_state: final_certificate_claims_and_index_loc_reconciled
initial_reviewed: "2026-10-11T02:00:19Z"
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T04:12:20Z"
diff_base: 7aa45525
diff_mode: uncommitted_worktree_and_untracked_sources
scope_source: git_diff_7aa45525_and_untracked_excluding_planning_and_tool_data
scope_content_sha256: 7eb21cf54a9c2204bc502beb91ccc1d8e7800fbe7a6b43393896410b28cc4ed5
---

# Phase 160: Code Review Report

**Reviewed:** 2026-10-11T04:12:20Z
**Depth:** standard
**Files reviewed:** 123
**Status:** clean; WR-01, WR-02 and WR-03 resolved. No open critical, warning or informational findings.

## Summary

Independent review reconciled 123 exact product source/doc/fixture paths above, refreshed from `git diff --name-only 7aa45525` plus untracked files, excluding planning, task ledgers, local review data and target tool data. The generated LOC artifact is included separately as parent-requested evidence. This includes the preceding 115-file scope, seven Plan 17 historical checker/test paths, and the additional existing permission-policy test path reopened for coverage closure. The comparison is all Phase 160 uncommitted work against HEAD `7aa45525`, including newly created source files, rather than HEAD-to-HEAD. Planning, task ledgers and ignored review-input metadata supply context and are excluded from product source scope.

The initial review identified two externally observable protocol gaps. Both corrections and their ordinary ingress controls are now inspected and backed by passing executor verification. They are retained below as resolved history. Final Plan 11 tests, Plan 12 guard/docs, Plan 17 historical guard repairs, source registrations, Python import compatibility repair, manual-peer fixture handshakes and generated metrics were reconciled after freeze. No additional actionable finding was confirmed.

Material guidance: repository AGENTS.md and Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability and Rust/TypeScript standards, and both active lesson inputs (7,188 combined bytes, loaded completely). Finalized context/research, Plans 01–16 and completed summaries supplied the contracts. The reviewer did not modify implementation, launch Cargo/native tests, acquire a build slot, commit, change shared planning state or spawn agents.

## Resolved Warnings

### WR-01: CF requests bypassed the version/verack handshake gate

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/message_dispatch.rs:32-68`

**Issue:** The original three request arms checked only peer existence. Outbound sessions already had an own-service snapshot before receiving the remote VERSION; inbound sessions gained it after VERSION and before VERACK. Valid CF requests could therefore reach complete preparation and sockets prematurely, and invalid requests could disconnect instead of remaining inert. Pinned `net_processing.cpp:3653-3657` ignores non-VERSION messages before VERSION, and `:3838-3841` ignores these application messages until VERACK.

**Fix:** Resolve the peer first, preserving UnknownPeer, then return no actions unless the existing `handshake_established()` predicate is true in each CF request arm. Cover all three families with valid and invalid fields before VERSION and before VERACK, followed by ordinary post-handshake success.

**Resolution: RESOLVED.** The parent added precisely that gate to all three arms and migrated identity/preparation fixtures to complete handshakes. Static inspection confirms semantic validation follows handshake admission. One behavioral RED and four dispatch-contract GREEN tests are recorded. The final `phase160_peer_evidence_pre_version_and_pre_verack_requests_are_inert` actual listener test sends all three families with valid and invalid fields before VERSION and before VERACK, completes the handshake, and observes a physical Pong without earlier CF bytes or a leaked reservation. The final four-test RPC library evidence run passed; existing all-family success tests preserve normal post-handshake serving.

### WR-02: Inbound framing rejected responses intended to be inert

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/resource.rs:586-610`

**Issue:** The original inbound command allowlist admitted only the three new CF request commands. A valid unsolicited `cfilter`, `cfheaders` or `cfcheckpt` frame was rejected as UnsupportedCommand and disconnected before reaching the explicitly inert ordinary peer-dispatch arms. Outbound TCP could reach those arms. This contradicted the six-family dispatch contract; pinned `net_processing.cpp:4939-4940` ignores unhandled commands.

**Fix:** Admit the three supported response names through the existing envelope gate, retain byte/checksum/strict codec validation, and route successfully parsed responses to existing inert dispatch. Add envelope controls and an actual inbound socket control proving no filter response, no reservation or receipt, and a usable subsequent ping.

**Resolution: RESOLVED.** All six CF names are present in the existing envelope allowlist while its historical bounds/checksum policy remains intact. `phase160_inbound_envelope_accepts_inert_filter_response_families` checks header admission and parsing, with one recorded RED and one GREEN. The final actual ingress test sends unsolicited CFilter, CFHeaders and CFCheckpt after handshake, then physically receives its Pong sentinel without a CF response or reservation leak. This ran in the passing four-test RPC library evidence run. No filter-client product or unrelated command expansion was added.

## Reviewed reasoning and boundaries

- Capability remains default-off, separate from backend/catch-up and relay activation. Requested blockfilters provenance is distinct from expanded all, including all-only advertisement without a backend, and prune-derived NETWORK_LIMITED is preserved.
- Ordered BASIC/own-capability, permitted stop, reversed-range and inclusive-family gates precede local quota refusal. Stop ancestry and genuine ScriptsValid provenance govern stale responses; exact fixed-width work and signed strict thirty-day tests are used.
- Complete persisted target/predecessor/checkpoint validation precedes response exposure. Missing targets produce no successful prefix; corruption and backend errors retain distinct sanitized internal outcomes. Serving does not drive index work or fetch bodies/history.
- One affine request owns the complete bounded suffix. Registry cancellation actually frees queued Vecs while handed frames and peer/request slots remain charged until settlement. One-use receipts record full framed writes and preserve physical prefixes separately from live range completion.
- The existing shared registry governs generic retained frames/encoder scratch and active CF bytes atomically in both admission orders. Exact-size preflight accounts for nested serializers, checksum stages and the retained command string; physical Tcp send_encoded writes the original governed slice.
- Inbound/outbound adapters release shared authority/context locks before socket waits and terminate unwritten suffixes on failure. Publication origin and monotonic epochs reject same-generation recovery ABA; trusted append preserves immutable captures.
- Independent wire/proof goldens, genuine accepted histories and physical socket evidence have different roles. The newly inspected 42-case typed wire test compares payload, full frame and decoded identity; proof tests compare 27 independent proof expectations and all 23 defined equivalent-time expectations.
- Simplification review found coherent existing-owner boundaries and no need for a second authority/database, generic effect-ledger reuse, request-time generation or optional encoder rewrite. The deterministic equal-work height/hash tie convention and finite local resource refusals are approved policy differences, not findings. Their explicit final parity disclosure was inspected in both the catalog and CFNET inventory.

## Final Review Addendum | 2026-10-11T02:14:39Z

The initial provisional review at 2026-10-11T02:00:19Z is superseded by this clean final source review. Both findings are resolved; all final changed paths in the signed 115-file scope have been reconciled. The content fingerprint is SHA256 over sorted repo-relative path, NUL, that file's SHA256, and newline for each listed file: `6c2df530d5bf5ad1e9ebfef71b56106c427dd42926ee6198d045ae54e1ee92de`.

Final evidence reviewed includes genuine accepted spending histories and retained forks, initial catch-up prefixes/gaps, a missing positive checkpoint at height 2000 for stop 3001, paired body/undo absence, exact legal range/one-over cases, bounded mixed traffic in both orders, and eight actual barrier races with one physical winner. Header-only eligibility negatives traverse the actual outbound loop; raw-body/UnknownLegacy controls deliberately use immediate managed dispatch because later background reconciliation can legitimately validate retained bodies. Seeded old timestamp/proof headers are explicitly negative controls and supply no accepted provenance.

The final small pressure-helper change replaces equivalent remainder comparisons with `is_multiple_of` and removes redundant 24/48 cases already covered by divisibility by 24; it introduces no behavior change. The oracle's original BASIC import is preserved alongside separate peer-message imports. Dynamic genuine-chain test glue delegates Python deadline/capture/input limits to the existing bounded Bun runner. The qualified guard selectors inspect concrete helper implementations; content-equality caches are checked for mutation/restoration, and the 128/1024 admission anchors require their complete delimiters.

Executor-owned passing receipts consumed by this review:

| Final gate | Evidence |
| -- | -- |
| Node `phase160_peer_evidence` | 8 passed, 0 failed, 81.59s; includes the final positive checkpoint-gap/no-prefix assertion |
| RPC library peer evidence | 4 passed, 0 failed, 20.61s; includes actual handshake/unsolicited-response and mixed-direction controls |
| Daemon peer evidence | 4 passed, 0 failed, 15.86s; configured versions, independent bytes, catch-up, retained stale and paired body absence |
| Independent proof tests | 2 passed; all 27 proof and 23 defined equivalent-time cases |
| Existing transport regressions | 18 inbound passed in 26.79s; 18 outbound passed in 11.31s |
| Strict node/RPC Clippy | All targets/features, `-D warnings`, passed after the narrow helper simplification |
| Final guard/mutation suite | Directly read log: 740 passed, 0 failed, 2,005 assertions, 36.35s; baseline 38 production edges and 46 executable selectors |
| Independent generator and compatibility | 54 generator tests and read-only fixture check passed; historical Phase 154–159 guards passed after the import repair |
| Registration and basic checks | 1,100 Rust breadcrumb paths, managed checks with zero findings, shell syntax and whitespace checks passed as reported; reviewer `git diff --check` also passed |

The reviewer performed read-only source, pinned-source, fixture, log and diff inspection and wrote only this review artifact. No implementation edits, test/Cargo/Bazel/native command, commit, shared-state edit or subagent work was performed by the reviewer. The parent remains responsible for the final full native, security/lifecycle verification and git gates; this report makes no claim that full native has already run.

The LOC artifact is included as the currently generated tracked-input report. Its index-mode commit-hook regeneration may include newly staged source and change its fingerprint; that generated-only refresh needs a narrow scope reconciliation and does not retroactively certify different executable source. Any implementation/test/guard change after this fingerprint requires renewed review before retaining this clean status.

______________________________________________________________________

_Reviewer: gsd-code-reviewer_
_Lifecycle: 160-20261010-peer-filter-serving_
_Final source review: clean; two historical warnings resolved._
_Signed: 2026-10-11T02:14:39Z_


## Narrow Final Daemon Evidence Addendum | 2026-10-11T02:17:37Z

Inspected the only subsequent executable-source delta in `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs`: three enabled-index matrix rows now cross general serving ON with explicit blockfilters, all, and all-plus-explicit; the all-only disabled-backend test sends a post-handshake type-255 checkpoint request after proving valid-family silence. Its existing five-second socket timeout bounds a one-byte read, which accepts only EOF/reset/abort and fails on any reply or missing disconnect. This directly proves semantic rejection still precedes absent-backend handling.

The executor reran the same four daemon evidence tests after this strengthening: **4 passed, 0 failed, 18.46s**. Its source is refrozen; existing node8/proof2/library4/inbound18/outbound18/startup6 and strict-Clippy receipts remain recorded, with the Clippy receipt preceding this final test-only strengthening. The unchanged final checker/mutation baseline remains 740 passed and zero failed. No production, guard, dependency or protocol change accompanies this delta.

Status remains **clean**, with WR-01/WR-02 resolved and all 115 paths retained. This signature supersedes the preceding content fingerprint for the changed daemon test. Current sorted path/NUL/file-SHA256/newline scope fingerprint: `d1fe690469c10e7fe769d5a0506f6f19dcdaa827047e90276cd4171ca8416740`. Full native and later staged generated-LOC reconciliation remain parent-owned gates.

_Reviewer: gsd-code-reviewer_
_Lifecycle: 160-20261010-peer-filter-serving_
_Signed: 2026-10-11T02:17:37Z_

## Integration Review Finding | 2026-10-11T02:48:24Z

The parent-owned first native verifier stopped at obsolete historical transport source contracts before Rust verification. Plan 17 adds seven checker/test files to the preceding scope and narrowly adjusts README wording; three existing test fixtures now explicitly complete their manually created peers' handshake. This expanded integration review supersedes the previous clean status until the new guard finding below is repaired.

### WR-03: Test-only source items could satisfy production connection and carrier proofs

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.ts:222-226`

**Issue:** `hasInboundAnnouncementPrefix` requires only `reexport.test(code)` for the connection runtime's `pub(super) use filter_serving::{ ... execute_inbound_emissions, ... };`. The reused `ordinaryRust` scanner masks test-only functions/modules but preserves use items. Therefore adding `#[cfg(test)]` immediately before that reexport still satisfies this proof while the production import is disconnected. The shared proof is consumed by both Phase 128 and Phase 134. This is a checker correctness gap against Plan 17's required production ownership; no production Rust defect or false physical-write receipt was established.

**Fix:** Capture the reexport match and classify its contiguous outer attributes with the existing ordinary-owner adapter, as already done for module/impl ownership. Add live mutation controls for the plain test-only attribute and one with an intervening ordinary attribute. Both guards must reject the disconnected production connection for the intended prefix invariant.

**Related site:** `scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts:55-62` originally searched `ordinaryRust(inboundCarrier)` directly for the typed struct and fields. The same scanner preserves `cfg(test)` structs, so a test-only EncodedWireResponse could satisfy the ordinary production carrier requirement. Both sites are the same missing source-item ownership classification.

**State: RESOLVED.** The shared prefix proof now captures and classifies the reexport’s contiguous attributes. The Phase123 carrier fields are checked only inside `ordinaryOwner(inboundCarrier, "pub(crate) struct EncodedWireResponse")`. Plain and intervening-attribute controls reproduced both gaps: four reexport tests failed before the repair and passed afterward; two carrier tests failed before their repair and pass in the final full 90-test Phase123 suite. The reviewer inspected both predicates and directly read the RED/GREEN logs without executing tests or mutating a corpus. All other inspected Plan17 send-before-ack, full governed batch, Written-only completion, failure-prefix cleanup and decoy controls remain intact.

_Reviewer: gsd-code-reviewer_
_Lifecycle: 160-20261010-peer-filter-serving_
_Integration review open: 2026-10-11T02:48:24Z_

## Final Integration Addendum | 2026-10-11T02:52:03Z

This clean 122-path integration signature supersedes the preceding open integration review and earlier content fingerprints. WR-01/WR-02 remain resolved and WR-03 is now resolved at both source-item ownership sites. No further actionable issue was confirmed. The reviewed final source scope is SHA256 `209f6d338496e3a8a1b78f8fb70adc743d014da336d9dd86cc85854311cf15e6`, computed with the same sorted path/NUL/file-SHA256/newline procedure.

Plan17 preserves all 55 original Phase123 scenarios and migrates their affected historical mutations to the actual governed-buffer/exact-send contracts. The ordinary runtime owner, complete fallible batch collection, original buffer borrow, concrete TCP write and post-write ordinary acknowledgement are connected. Inbound ordinary completion checks the actual Written argument and the full gated capability/ordinary/durable-intent completion shape. Announcement proof connects the ordinary module, reexport, live drain and concrete socket adapter to the exact delegated executor, whose only consuming completion arm follows Written; current/suffix failure and Drop retirement remain checked. Comments/literals, test-only owners/items, disconnected helpers, omitted writes, early acknowledgement, ignored failure returns and failed-frame credit have explicit rejecting live mutations. These remain narrow source guards alongside actual transport tests and full native verification, not standalone semantic certification of arbitrary Rust.

The Phase128 exported inventory and Phase134 FILES include the real delegated executor, so inherited Phase129 fixtures obtain it without separate fixture changes. The Phase132 checker/tests have no diff against the base. README changes only the new non-claim predicate to “remain deferred”; broader public/default, V0/BIP37, production and Phase161/162/v2.5 success are still not claimed.

The three root-owned fixture corrections add only existing `complete_peer_handshake` calls in `peer_fixtures.rs`, `serving/tests/families.rs` and `serving/tests.rs`, after manual peer construction and before request dispatch. Their semantic, saturation, catch-up, progress and reservation assertions are unchanged. The parent records each old failing batch followed by passing focused controls in 1.22s, 0.96s and 7.55s respectively, plus passing remaining workspace preflight packages. No production Rust behavior changed.

Additional verification receipts inspected:

| Integration gate | Final evidence |
| -- | -- |
| Phase123 full suite | 90 passed, 0 failed, 276 assertions, 2.49s; all 55 historical scenarios remain active |
| Phase128/129/132/134/117 exact suite | 438 passed, 0 failed, 638 assertions, 18.19s |
| Reexport ownership RED/GREEN | RED 0 passed/4 failed; GREEN 4 passed/0 failed, 10 assertions |
| Typed-carrier ownership RED/GREEN | RED 0 passed/2 failed, 8 assertions; both controls included in final 90-test GREEN |
| Related Phase127/160 suites | 757 passed, 0 failed, 2,022 assertions, 60.12s |
| Live historical/related guards | Phase123/128/129/132/134/117/127/160 reported exit0 |
| Threat register ownership | Independently read 39 rows/39 unique IDs; T-160-35 occurs only in Plan17 |
| Managed/source checks | Zero managed findings and clean whitespace checks reported; reviewer `git diff --check` passed |

The first full native run stopped on obsolete historical Phase123 guards before Rust; it is not a successful native receipt. Focused repairs and Rust preflight do not replace the parent-owned second unchanged full native run, final security/lifecycle verification or commit gate. Later generated LOC/index-mode reconciliation remains parent-owned; no implementation, test, guard, dependency, Cargo/Bazel/native execution or git mutation was performed by this reviewer. Only this review artifact was updated.

_Reviewer: gsd-code-reviewer_
_Lifecycle mode: yolo_
_Lifecycle: 160-20261010-peer-filter-serving_
_Final integration status: clean_
_Signed: 2026-10-11T02:52:03Z_

## Generated LOC Confirmation | 2026-10-11T02:52:37Z

Inspected the parent's latest tracked-input LOC regeneration: 1,435 files, 411,707 total lines, and input fingerprint `b13c9e6363479bb077b9d3fad3c242703b9012d93ba0db5e95d105108c3f6fd2`. Recomputed all 122 source/doc/fixture hashes afterward; none changed since the integration signature, whose scope fingerprint remains `209f6d338496e3a8a1b78f8fb70adc743d014da336d9dd86cc85854311cf15e6`. Later staged/index-mode regeneration remains a separate generated-artifact reconciliation. Final source review remains clean; three historical warnings are resolved.

_Reviewer: gsd-code-reviewer_
_Lifecycle mode: yolo_
_Lifecycle: 160-20261010-peer-filter-serving_
_Signed: 2026-10-11T02:52:37Z_

## Coverage Closure Review Addendum | 2026-10-11T03:33:55Z

Inspected the six test-source deltas reopened under existing Plans01/02/05/06/15 and `160-COVERAGE-CLOSURE.md`: codec filter tests, request policy tests, resource tests, header-proof tests, existing inbound permission-policy tests, and the inline dispatch fixture. No new finding, production behavior change, weakened assertion, coverage exclusion or false validation-provenance claim was found. WR-01/WR-02/WR-03 remain resolved.

The twelve added controls assert meaningful independent outcomes: malformed request markers preserve each command's identity and refuse outgoing encoding; fixed-size count inputs and direct checked sizing reject oversized/overflowing responses without allocating huge vectors; unsupported raw type/stop fields remain intact and fail the ordered semantic gate; a known stale stop missing exact proof fails despite a valid best proof; unknown/corrupt reconstructed proof facts cannot alter trusted facts or lend proof to descendants; consumed budgets cannot be reopened by later limit increases, including a limit below consumption; frame-size arithmetic crosses the four-byte/nine-byte CompactSize boundary with explicit expected totals; parsed permission classes retain requested-versus-expanded prerequisites and emit the fixed filter-effect label. The nine-byte usize response-count control is correctly restricted to 64-bit targets where that count is representable; it is not a production coverage exclusion.

The dispatch fixture now constructs valid/invalid requests directly. It preserves the original invalid types `[255, 2, 1]`, starts `[u32::MAX, 77]`, unknown-session and raw-identity assertions, and all three families under both valid/invalid and pre-VERSION/pre-VERACK loops. It removes only an impossible test match arm; the production dispatch gates and serving paths are unchanged. Raw malformed proof fixtures remain explicitly negative controls, not accepted-chain evidence.

Directly inspected `packages/target/phase160-coverage-focused.log`: **12 new boundary tests passed and all 4 existing dispatch-contract tests passed**, zero failures/ignored tests. `packages/target/phase160-network-coverage.log` records **598 unit, 2 parity and 2 property tests passed**. The parent reports the network-only `cargo llvm-cov --show-missing-lines --text --fail-uncovered-lines 0` command exited0 with no uncovered lines and no exclusions. The earlier codec RED distinguished the outgoing sizing error layer from incoming CompactSize refusal; the corrected assertions retain both contracts and changed no production behavior.

The parent-owned final `cargo fmt` completed exit0 before the third full native run began. The parent reports native attempt2 passed formatting, strict Clippy, build, Rust tests, benchmark smoke, Bazel and provenance, then failed only its final pure-core coverage gate; this is not a successful complete native receipt. The third unchanged full native run is underway and still must pass before phase or git completion. The reviewer launched none of these commands.

The refreshed 123-path inventory exactly matches git diff against `7aa45525` plus untracked product files with planning/local/task/target tool data excluded, including the generated LOC artifact. Current tracked-input LOC is **411,799 lines**, input fingerprint `fd259cea4cf31f8391f218c030fe2974163723aee667539d0c8a7de3b5ceef38`. Current sorted path/NUL/file-SHA256/newline scope fingerprint: `821780ec7e24bbb973c0819db9e6c9d1ab28d241e59155e8c78c799e5c8a96ee`. Later staged/index-mode generated-LOC reconciliation remains parent-owned. Only this review artifact was changed by the reviewer.

_Reviewer: gsd-code-reviewer_
_Lifecycle mode: yolo_
_Lifecycle: 160-20261010-peer-filter-serving_
_Final coverage-closure source status: clean_
_Signed: 2026-10-11T03:33:55Z_

## Final Certificate, Claims and Staged LOC Reconciliation | 2026-10-11T04:12:20Z

Narrowly inspected the completed Phase160 certificate and final README, BASIC catalog, parity-index row, PROJECT/ROADMAP/REQUIREMENTS, STATE and task-completion claims. The existing product source-review scope stays **123 paths**, with **three resolved warnings and no open findings**; planning/task metadata was inspected only for claim consistency and does not expand that source inventory. The parent confirms executable source and tests are unchanged since the clean coverage-closure review.

`160-VERIFICATION.md` is generated by gsd-verifier in yolo lifecycle `160-20261010-peer-filter-serving`, dated2026-10-11T04:05:52Z, status passed/lifecycle_validated true with zero overrides. Its seven grouped truths cover five roadmap criteria, six CFNET requirements, twenty decisions,51 plan truths,44 artifact declarations and41 links. It records native3 exit0 in **26m5.720s (1,565,720ms)**,4,093 primary Rust passes with three explicit historical ignored measurements, and zero uncovered pure-core lines. The reviewer directly read the native3 log's successful final test output and matching `verify.sh completed` duration. Native attempts1/2 remain failed historical runs; no commit or push success is inferred from verification.

README and the BASIC catalog link to the real certificate with correct repo-relative paths. The parity row now marks only CFNET-01–06 done and includes review/security/verification evidence. Intentional resource, equal-work tie and all-only permission differences remain disclosed. Phase161 operator work, Phase162 integrated retained-client proof, overall v2.5 completion, public/default serving, V0/BIP37, archive and production/funds claims remain deferred/excluded; scoped phase completion does not promote those boundaries.

Current planning summaries were reconciled after review identified stale rollups: **18 of22 requirements Complete, four Pending; seven of nine phases verified;58 of58 created plans complete**. CFNET's six checkboxes/traceability rows agree, STATE correctly points to161, and PROJECT's current requirements/decision summaries agree. Explicitly historical Phase159 narratives retain their dated snapshot. The task completion block honestly records the verified pre-commit snapshot and derives later hook/commit/push outcomes from actual saving commits/refs rather than claiming those future effects now.

Inspected the final **staged/index-mode LOC artifact**: **1,484 included files;427,844 total lines;373,365 code/content;21,716 comment-only;32,763 blank**. Categories sum to the total. Input fingerprint is `46be279963183778176953d143671f4d42ce303416baff3d4bb24cbfc157438f`; the staged artifact equals the worktree byte-for-byte (file SHA256 `d6228ab6fc9390308101f68544616920e21756932da36c1e4bdf981adad6cbaf`). The increase from411,799 reflects newly staged first-party files previously absent from tracked-only generation; it is the anticipated generated-artifact reconciliation, not executable behavior drift.

Current 123-path sorted path/NUL/file-SHA256/newline fingerprint is `7eb21cf54a9c2204bc502beb91ccc1d8e7800fbe7a6b43393896410b28cc4ed5`. Parent-owned live claim/Phase154–160/active-milestone guards, managed checks, parity1100, traceability, lifecycle validation and cached whitespace checks passed as reported. The reviewer also read clean worktree/index whitespace checks. Earlier certificate source-review fields describe the pre-completion metadata snapshot; the parent verifier reconciliation must bind this latest narrow metadata/LOC signature before finalization.

Only this review artifact was updated by the reviewer. No product source, test, shared-state, staging, commit, push, test suite, Cargo/Bazel/native execution or subagent action was performed. The mandatory protected commit hook and final git operations remain parent-owned; this source review does not pre-certify their future outcomes.

_Reviewer: gsd-code-reviewer_
_Lifecycle mode: yolo_
_Lifecycle: 160-20261010-peer-filter-serving_
_Final reconciliation status: clean_
_Signed: 2026-10-11T04:12:20Z_

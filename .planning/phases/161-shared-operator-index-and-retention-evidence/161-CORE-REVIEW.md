---
phase: 161-shared-operator-index-and-retention-evidence
reviewed: 2026-10-11T06:45:54Z
re_reviewed: 2026-10-11T07:16:10Z
depth: deep
review_kind: supplemental_core_diff
diff_base: a7963f943de6a779b6b1dcafd0f42bdd6ded9066
files_reviewed: 42
files_reviewed_list:
  - packages/open-bitcoin-network/src/filter_serving/capability.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/basic_operator.rs
  - packages/open-bitcoin-network/src/peer/inbound_state.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/limited_serve.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot/basic_tests.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
  - packages/open-bitcoin-node/src/network/tests/filter_capability.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/status.rs
  - packages/open-bitcoin-node/src/status/basic_index.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/recovery.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings: [WR-01]
status: clean
---

# Phase 161: Supplemental Core Code Review

**Reviewed:** 2026-10-11T06:45:54Z
**Focused re-review:** 2026-10-11T07:16:10Z
**Depth:** deep, scoped to the actual uncommitted core diff and relevant producer dependencies
**Files Reviewed:** 42, including six additional changed dependencies in Plan 11
**Status:** clean within this supplementary scope; WR-01 closed

## Summary

This is a supplemental review of Plans 01, 02 corrected, 03, 10 and 04, followed by focused Plan 11 remediation review. The file list comes from the actual tracked diff against the specified base plus nonignored untracked paths. At initial review, unchanged `chainstate/filter_index.rs`, query/readiness, flush and related lifecycle code were inspected as supporting dependencies. Query dependencies subsequently changed by Plan 11 are now included in the changed-file list; unchanged supporting files are not. CLI, RPC crate files, benchmarks, documentation and historical guards are outside this review. This report is not full-phase approval or requirement closure.

The new collector preserves the important healthy-path distinctions: accepted target versus headers, earned initial synchronization versus current lag, processed versus safe durable progress, configuration versus offers, retained growth versus prune accounting, and immutable totals versus active-prefix height. Publication accounting is installed only after successful guarded effects; primary retries and compatibility duplicate candidates do not add the same immutable row twice. The original warning concerned a real discovered storage-read fault that did not reach persistent operator evidence. Plan 11 closes that warning; the original finding and its source-derived negative case are preserved below as review history.

## Resolved Warnings

### WR-01: A discovered immutable read fault can still be followed by healthy BASIC status

**Status:** Closed by focused Plan 11 source re-review and the executor's recorded genuine caller/regression evidence. Locations and issue description below refer to the original 2026-10-11T06:45:54Z review; current implementation and closure evidence are appended at the end of this report.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/operator_snapshot.rs:221-225`

**Affected collector:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs:165-190`

**Issue:** The new projection obtains its failure only from `maybe_basic_index_failure()` and trusts retained measurement while `PublicationControl::read_integrity` remains true. Actual immutable query corruption/backend errors are returned to the query caller without changing either fact. Matching STATE/OWNER metadata and the existing append identity therefore let `project_basic_owner` report `Current`, `failure: null`, and available complete retained totals after the backend has already been observed to contain corrupt/missing ancestry or to fail a real read. A missing immutable parent also makes the cached record count numerically stale. This violates D-05/D-06/D-07 and the research requirement that known integrity/backend faults remain failed or unavailable; it does not require proactive detection of previously unseen corruption.

**Evidence and concrete negative case:**

1. `sync/tests/filter_index/catch_up/rpc_faults.rs:159-203` already earns a real row through the ordinary validated owner, then calls `inject_query_record_fault_for_test` with `CorruptTarget`, `CorruptParent` or `MissingParent`. The seam deliberately bypasses raw-write invalidation to represent external backend corruption beneath an earned epoch (`filters/query.rs:156-203`). The real `ManagedNetworkHandle::basic_filter_query` returns `StorageError::Corruption`.
2. `network/runtime_authority/filter_index/query.rs:134-139` uses `Handle::read`; `query_network` propagates the store error. `filters/query.rs:210-244` supplies only `&PublicationControl`, and target parsing, parent lookup and parent parsing at lines 296-326 only propagate their errors. They neither revoke read integrity nor retain a fixed fault category. `BeforeQueryRead` at lines 267-272 provides the corresponding existing backend-failure control.
3. `readiness/owner.rs:23-49` consumes existing manager failure but creates none. Its waiter result at lines 141-150 is transient and does not retain a store/owner fault; immediate reads do not take that collection path. The new operator collector does not consume waiter outcomes.
4. Calling `operator_snapshot()` immediately after any of those errors reads unchanged STATE/OWNER, unchanged append identity and still-true `read_integrity`. The cache is accepted at `retention.rs:165-190`; the owner remains failure-free and coherent, so the new status returns healthy operating evidence. Existing fault tests stop before asserting this subsequent snapshot. This conclusion is source-derived; this reviewer did not run a new Cargo reproducer.

**Fix:** Record a fixed, constant-size integrity/backend observation on the actual immutable storage-read error path under the existing publication guard, and revoke complete retained/progress trust when that fault warrants it. Have the existing coherent capture consume that observation. Cover immediate RPC reads, completed barrier reads and peer immutable reads without adding a second authority, status scan, retry, recovery, or trust-renewal side effect. Preserve the earned historical latch and genuinely independent known configuration. Do not latch ordinary target absence, unknown client block IDs, stale/cancelled captures, lifecycle invalidation, waiter capacity, peer-budget refusal or wire-response policy rejection as backend corruption; those would let ordinary client activity manufacture a sticky index failure.

**Minimal verification:** Extend the genuine existing `row_fault`/backend controls with a baseline operator capture, the failing production read, and a subsequent operator capture. Assert fixed integrity/backend evidence and unavailable complete retention/progress as appropriate; repeat status capture to prove it neither clears the failure nor changes stored rows/epochs/scan counts. Include negative controls for ordinary missing/unknown targets and resource refusals. Actual underlying bytes, raw error text and identifiers must remain absent from serialized BASIC evidence. Reopen with corruption must continue to fail closed.

## Other Reviewed Invariants

- **Coherence and ordinary operation:** The projection compares full accepted/active target identity, generation, branch, processed checkpoint and safe checkpoint. The observation checks persisted owner/state against the already-validated current revision. Genuine connect, scheduled turn and flush fixtures exercise the same producer chain. No additional ordinary healthy lag/flush rejection was identified. Live disable/re-enable intentionally invalidates an old manager generation; its test requires configuration/protection truth without claiming that old progress became current.
- **Initial synchronization:** `Current` requires the earned latch as well as zero lag. The replacement test constructs a real domain replacement with zero lag and an unearned latch. Later accepted work and reorg preserve the historical earned latch.
- **Read bounds:** The production BASIC observation makes exactly two fixed metadata gets. It checks borrowed lengths and checked cumulative bytes before decoding; it does not traverse retained records, peers, bodies, undo or the lock map. The new serialized types contain bounded enums, booleans, heights and five counters. Exact/one-under metadata limits and the 4097-byte input control exercise the actual collector. Tests additionally assert a 4 KiB BASIC serialization ceiling.
- **Startup recovery:** The corrected wrapper carries the scan result through the held publication guard and performs only its closing epoch advance. Ordinary populated startup routes use one forest validation; reconciliation retains two pre-existing validations, explicitly asserted in `recovery.rs:94-125`. That reconciliation case is not evidence of an extra accounting-only scan. Orphan rows are refused instead of becoming an invented empty forest. Mandatory projection/forest checks continue after optional measurement exhaustion.
- **Retained accounting:** Only immutable primary key/value bytes and payload lengths contribute. Parent/projection reads do not. Primary append installs the delta after `finish_basic_filter_batch`; compatibility preparation excludes existing identical records and duplicate candidate identities. Unknown starting totals remain unknown after a partial append. Real reorg return/reuse and real reopen tests use the production publication path; synthetic codec stores are not represented as consensus validation. Raw put/remove revokes shared integrity before effects, while ordinary append/coins/metadata revisions preserve immutable totals. Publication poison prevents old memory from being exposed.
- **Protection:** Inclusive bounds use the exact authoritative first height; starts beyond the representable reserved bound return height-space exhaustion. The removed clamp has not reappeared. No physical bytes-saved estimate or protection-release effect is introduced by collection.
- **Peer/configuration scope:** Checked old/new deltas cover inbound/outbound insertion, version offers, duplicate-version/self-connection refusal, removal and reconnect. Combined explicit/all request provenance survives; offers remain per-session facts across general-setting changes. Unsupported implicit whitelist provenance is truthfully zero. Failed host transition configuration becomes unknown, including post-effect fault cases. Known general serving remains independent of backend readiness and permissioned `all` offers.
- **Privacy:** New BASIC wire fields contain no identifier-bearing type or raw backend error. Every free-text availability reason is allowlisted on both serde directions, protection bounds are validated, and guidance is a fixed enum. No direct raw-error/identifier disclosure was found in this scoped group.

## Simplification Pass

The existing publication control, one coherent capture, scalar totals and scalar peer counters are the right small seams. No alternative cache, background scanner, peer-table walk, persisted aggregate schema or renderer resolver is needed. The warning should be addressed in the existing error path and consumed by the same capture, keeping status read-only. No style-only warnings or speculative architectural rewrites are proposed.

## Verification and Limits

- Read-only source/diff analysis and fixture-producer tracing were performed; `git diff --check` passed for node/network sources. No source files, trackers, Cargo artifacts, commits or branch state were modified by this reviewer. Only this report was created.
- No Cargo, Bazel or native verification was executed by this reviewer, as required by the delegated concurrency/ownership contract. Recorded plan results were read as prior evidence, not represented as runs by this reviewer. The initially missing fault-to-status regression was subsequently executed by Plan 11, as attributed in the closure evidence below.
- This is a diff-scoped deep review, not exhaustive review of all pre-existing node/network code. Supporting unchanged query/readiness dependencies were followed to establish WR-01. Final RPC/CLI/dashboard/support correctness, authentication/redaction at those wider consumers, cross-consumer evidence, historical guard repair, native verification and full-phase security/lifecycle approval remain root/later-plan responsibilities.
- No stronger crash-window, public-serving, archive-scale, wallet, production-readiness or Phase 162 integrated-proof claim follows from this report.

Material guidance: repository `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, standards index plus architecture/code-shape/verification/testing/operability/Rust pages; Phase 161 D-01 through D-16, research, approved terminal UI contract, and Plans 01/02 corrected/03/10/04 summaries. Both active lesson inputs were loaded (7188 bytes; 2397 conservative estimated tokens); no archives or lesson mutations were used. Project skill directories were absent. The GSD code-review instructions informed artifact shape; the parent remains responsible for final full-phase review and commits.

_Reviewer: gsd-code-reviewer. Scope: supplemental Phase 161 core diff._

## WR-01 Focused Closure: Plan 11

**Re-reviewed:** 2026-10-11T07:16:10Z. **Result:** WR-01 closed; no new actionable correctness/security finding identified in the frozen fourteen-path remediation. This closure does not approve the whole phase or later consumer work.

The Plan 11 plan, all fourteen owned paths, actual query/readiness/serving/transport dependencies and the completed `161-11-SUMMARY.md` were inspected. Eight paths overlap the original 36-file review; the six newly changed dependencies are included in the 42-file frontmatter list. The original issue remains above for audit history and is not an open warning.

### Source and Authority Evidence

- `filters/query.rs:223-291` and `filters/query/peer.rs:45-118` fold one transient stack `Cell` into the already-held publication guard before returning. Actual backend gets, admitted envelope/record decoding and required-parent absence mark fixed source-local categories. Callback return classes and strings are not used as classifiers. Original errors are retained. Within a request, the observation merge preserves Integrity over a later Backend observation; across requests, `publication/epoch.rs:17-42` retains the strongest actual observation.
- First observation revokes read integrity/complete retention, advances the existing checked epoch and invalidates append/coins/metadata proof once. Invalidated immutable rereads stop before marking a new fault, so Backend does not become Integrity merely because later reads are blocked. No request scan, new lock, persisted schema or raw-error field was introduced.
- `operator_snapshot.rs:76-100` and `:275-280` consume the fixed fault before ordinary owner coherence, so both observed categories dominate RequiredBody/RequiredUndo/FactLimit. Processed, safe durable, lag and complete retained evidence become unavailable. Known configuration, historical earned latch and accepted target remain independent. The prior earned protection scalar is captured before proof revocation and is cleared on raw/recovery invalidation or host transitions; it grants no work/prune authority.
- `publication/epoch.rs:46-53` supplies explicit verified reset. Initialization resets only after empty-artifact proof and successful publication; the compatibility checkpoint route resets after its existing complete forest/projection/candidate validation and successful effects. Record-only partial publication never resets. `ownership.rs:525-566` blocks minting or completing legacy work after a sticky fault. Append proof is already revoked on first observation.
- Startup resets only after the closing epoch succeeds. Its error branch restores the preceding sticky category and revokes trust even if an inner helper temporarily completed verification. Host enable applies equivalent restoration; direct disable resets only after complete validation and all release effects succeed. AfterEnable/AfterDisable/AfterRelease and closing-epoch controls reach their intended failure points. No extra validation pass was added; optional measurement exhaustion remains unavailable even when mandatory verification successfully renews read trust.
- The public query/summary definitions in `network/runtime_authority/filter_index/query.rs`, final-barrier machinery in `readiness/owner.rs`, and transport implementation have no diff against the review base. `getindexinfo` retains its existing latch/processed-height semantics and result shape. The remediation changes how real faults persist in the added BASIC operator group without replacing those pinned shapes.

### Genuine Caller, Negative and Receipt Evidence

The new `rpc_faults.rs:107-188` matrix uses the existing `genuine()` validated/Fjall producer and executes CorruptTarget, CorruptParent, MissingParent and BeforeQueryRead through both immediate queries and a previously earned barrier's final immutable read. Its helper checks subsequent fixed failed status, unavailable progress/growth, known configuration/latch/target/protection, 32 identical polls, exactly two metadata gets, at most 4 KiB metadata/output, unchanged stored rows/owner/cumulative scans and absence of private markers. Six genuine retained-authority combinations separately prove precedence over body/undo/resource owner pauses. Historical corruption controls retain actual failed reopen evidence; the new transient-backend case performs an actual all-handle verified reopen.

`serving/tests.rs:19-173` exercises the same four faults through real managed peer preparation while an earlier response has a handed frame. The fault rejects the new preparation and makes the old response stale. The handed frame stays charged until its isolated physical receipt settles; zero bytes are invented beforehand. Settlement is `PhysicalPrefixInvalidated`, not successful range completion, and all suffix/frame/request reservations then release. Disconnect cleanup remains empty. This is state-machine/isolated acknowledgement evidence, not live socket-write evidence.

Negative controls cover ordinary missing/unknown targets, cancelled wait and readiness counter refusal, raw/stale final completion, disabled/initial-catch-up absence, invalid/unsupported peer requests, seven peer admission axes, oversize envelopes, captured-branch mismatch, Corruption-typed envelope/hex/hash callbacks and maximum-codec wire refusal. These controls do not mark sticky read faults. Same-authority failed recovery preserves the category; ordinary metadata and blocked partial work cannot heal it. Mandatory validation can reset after optional measurement-budget exhaustion, while old epoch tokens remain revoked. Source tracing confirms those controls exercise the actual wrappers rather than a blanket error classifier.

### Recorded Checks and Attribution

The executor's completed Plan 11 summary records 114 passing executions across ten nonempty selectors: storage 2, projection 1, new callers 9, historical RPC owner faults 10, readiness 21, managed completeness 6, query epoch 7, query measurements 3, original authority 18 and retention 37. After final simplification, the combined twelve read-fault tests were rerun and passed; those repeat the new focused cases. Six additional historical budgeted-read cases passed. This is 120 distinct selected cases, not 132 distinct cases.

The final summary records strict node Clippy across all targets/features with `-D warnings`, scoped Rustfmt on all fourteen files, parity breadcrumbs, managed checks and diff checks as passing. This reviewer independently inspected the final timing JSONs: `phase161-read-fault-clippy-release`, `phase161-read-fault-final` and `phase161-read-fault-peer-regression` each have `outcome: success`, exit status 0, with 7567/29688/6842 ms respectively. Exact test-discovery counts are attributed to the executor's summary and source-visible tests, not to these timing-only records. This reviewer also reran read-only diff whitespace checks, which passed, and made no Cargo/Bazel/source/tracker/commit changes.

The fix preserves the small existing authority/capture seams. No further simplification is needed for WR-01. Known limitations remain unchanged: discovered faults are recorded without proactive corruption scanning or repair; poison/exhausted authority stays unavailable; body/wallet and crash-window claims do not expand. Root still owns full-phase source/security review, remaining consumer work, native verification and formal CFOP-01/lifecycle closure.

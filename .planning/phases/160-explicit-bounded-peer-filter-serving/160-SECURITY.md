---
phase: 160-explicit-bounded-peer-filter-serving
slug: explicit-bounded-peer-filter-serving
status: verified
threats_total: 39
threats_closed: 39
threats_open: 0
asvs_level: 1
security_profile: high
security_enforcement: true
block_on: open
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T03:40:54Z"
created: "2026-10-11T02:07:42Z"
audit_scope: declared_mitigation_presence
diff_base: 7aa45525
---

# Phase 160 — Security

All 39 unique declared threats have mitigation controls present. This is independent scoped source verification, including the enforced finalization gate. It is not phase completion, a full native receipt, public-network validation or ASVS certification. ASVS 5.0 V2 validation/business logic and V8 authorization apply at level 1/high profile as declared by the plans.

## Scope and method

State B: plans and completed summaries existed with no prior SECURITY.md. Security enforcement defaults to enabled because `.planning/config.json` has no false override. No explicit block_on override was supplied; unresolved declared threats would block advancement. All 17 current threat-model blocks were extracted and de-duplicated: T-160-01 through T-160-39 occur exactly once. Each disposition is mitigate and was classified before source verification; no accept/transfer verification was needed.

Read finalized context/research, revised decision coverage and Plan12, summaries 01–11 and 13–17, auditor/workflow/template and local guidance. Plan11 and Plan17 were consumed during final source/integration reconciliation. T-160-35 is owned uniquely by Plan17. Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture/verification pages. Both canonical active lessons were read completely (1,958 repository + 5,230 global = 7,188 bytes; 2,397 conservative tokens), without archives or lesson mutation. The parent owns its existing audit baseline.

Fresh discovery combined git diff --name-only 7aa45525 with git ls-files --others --exclude-standard: 114 changed implementation/test/script/documentation paths, 52 untracked new paths at this snapshot. The initial .local inventory was not authoritative. This audit traces declared mitigation-bearing code/tests within that scope; it does not claim a blind vulnerability scan or full general review of every inventory line.

The auditor modified only this report. No Cargo, Bun test, Bazel, full native command, commit, hook, shared state or task-ledger mutation occurred. Historical executor results are attributed to their summaries. The final Plan11 summary records eight node, four RPC library, four daemon, two proof, eighteen outbound, eighteen inbound and six startup passing focused tests, plus strict affected-crate Clippy. The final daemon rerun took 18.46s, superseding the earlier 15.86s receipt. These are attributed executor/parent receipts, not tests launched by this auditor. Plan12 summary, full native and lifecycle approval remain independent parent gates. The independent source review is clean over the reconciled 123-path scope after coverage closure; no pending native or phase result is fabricated as passed.

## Trust boundaries

| Boundary | Data and control |
| --- | --- |
| Peer/config to typed policy | Bounded raw type/heights/hashes, complete handshake, own advertisement, explicit prerequisite and semantic ordering. |
| Accepted history to branch authority | Direct active membership or genuine ScriptsValid plus strict signed timestamp/exact proof age and checked stop ancestry. |
| Durable publication to immutable plan | One complete guarded read, target/parent commitments, finite counters and opaque shared-origin monotonic read epoch. |
| Shared authority to transport | Owned bytes and affine private request/session/sequence/length receipts; socket I/O outside shared locks. |
| Concurrent generic/CF allocations to admission | Same registry atomically accounts encoder scratch, retained generic owners and handed CF frames with bounded descriptors. |
| Pinned helper to golden publication | Exact pin/clean-input/path validation, bounded process and lossless schemas before staged per-file replacement. |
| Evidence to finalization | Full native, clean independent reviews and passed lifecycle-valid verification before outer git gate. |

## Complete threat register

STRIDE letters preserve the plan register: S spoofing, T tampering, R repudiation, I information disclosure, D denial of service, E elevation of privilege. CLOSED means the declared control is present, subject to the explicit evidence boundaries below.

| Threat ID | Category | Component | Disposition | Declared mitigation | Status | Source evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T-160-01 | T | Wire parser | mitigate | Bound CompactSize lengths against remaining payload and frame limits before allocation; raw type survives to policy. | CLOSED | `packages/open-bitcoin-network/src/message/filter.rs:221`; `packages/open-bitcoin-network/src/message/filter.rs:242`; `packages/open-bitcoin-network/src/message/filter/tests.rs:372` |
| T-160-02 | D | Malformed complete request | mitigate | Nonfatal CF extraction outcome is distinct from corrupt framing/EOF; no response or accidental allocation. | CLOSED | `packages/open-bitcoin-network/src/message.rs:177`; `packages/open-bitcoin-network/src/peer/message_dispatch.rs:67`; `packages/open-bitcoin-network/src/message/filter/tests.rs:345` |
| T-160-03 | T | Raw digest order | mitigate | Pinned exact bytes and non-palindromic hash tests prevent display-hex reversal. | CLOSED | `packages/open-bitcoin-network/src/message/filter/tests.rs:14`; `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs:329` |
| T-160-04 | E | Permission expansion | mitigate | Explicit requested accessor and full all/implicit matrix enforce prerequisite without denying pinned all corner. | CLOSED | `packages/open-bitcoin-network/src/filter_serving/capability.rs:35`; `packages/open-bitcoin-network/src/filter_serving/capability/tests.rs:13`; `packages/open-bitcoin-network/src/inbound/tests/permission_policy_cases.rs:478` |
| T-160-05 | S | Version capability | mitigate | Capture own services per session; ignore remote advertised CF bit for authorization. | CLOSED | `packages/open-bitcoin-network/src/peer.rs:125`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs:255` |
| T-160-06 | T | Prune service setter | mitigate | One composition function preserves NETWORK_LIMITED/WITNESS and general CF bit. | CLOSED | `packages/open-bitcoin-node/src/network/limited_serve.rs:39`; `packages/open-bitcoin-node/src/network/tests/filter_capability.rs:11` |
| T-160-07 | E | Startup prerequisite | mitigate | Resolve BASIC/general/explicit permissions before durable runtime open or prune recovery. | CLOSED | `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:302`; `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:391` |
| T-160-08 | I | Public defaults | mitigate | Default false; enablement never toggles listener/public/relay policy. | CLOSED | `packages/open-bitcoin-rpc/src/config.rs:249`; `packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs:12` |
| T-160-09 | T | Shared runtime config | mitigate | Both live adapters read the same configured handle, not duplicate backend construction. | CLOSED | `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:340`; `packages/open-bitcoin-rpc/src/context/inbound_wire.rs:46` |
| T-160-10 | T | Proof-age arithmetic | mitigate | Integer256 operations follow Knots formula and modular overflow; independent known vectors prevent float/u128 shortcuts. | CLOSED | `packages/open-bitcoin-consensus/src/chain_proof.rs:93`; `packages/open-bitcoin-consensus/src/chain_proof.rs:138` |
| T-160-11 | E | Invalid target | mitigate | Reject negative/zero/overflow targets before authorization. | CLOSED | `packages/open-bitcoin-consensus/src/chain_proof.rs:94`; `packages/open-bitcoin-network/src/header_store/proof.rs:49`; `packages/open-bitcoin-network/src/header_store/proof/tests.rs:175` |
| T-160-12 | D | Division loop | mitigate | Fixed256-bit bounded division and checked spacing provide a fixed operation ceiling. | CLOSED | `packages/open-bitcoin-consensus/src/chain_proof.rs:67`; `packages/open-bitcoin-consensus/src/chain_proof.rs:144` |
| T-160-13 | E | Stale stop eligibility | mitigate | Active direct membership or ScriptsValid plus timestamp and exact proof age, each strict2592000 seconds. | CLOSED | `packages/open-bitcoin-network/src/filter_serving/policy.rs:164`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs:289`; `packages/open-bitcoin-network/src/filter_serving/policy/tests.rs:573` |
| T-160-14 | T | Range identity | mitigate | Stop-parent walk checks hash/height/parent; never use active projection to serve stale ranges. | CLOSED | `packages/open-bitcoin-network/src/filter_serving/policy.rs:177`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs:119` |
| T-160-15 | D | Ancestry selection | mitigate | Charge each visited parent and metadata lookup; exceed explicit local work cap with no response and no protocol-range fiction. | CLOSED | `packages/open-bitcoin-network/src/filter_serving/policy.rs:204`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:220` |
| T-160-16 | D | Hidden validation work | mitigate | Charge target+parent envelope/hash/copy before parsing; no parse_record_fields shortcut. | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs:165`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs:175` |
| T-160-17 | D | Range and checkpoint budgets | mitigate | Checked8192 ancestry/4096 record/256 metadata and explicit byte counters bound legal sparse-output work. | CLOSED | `packages/open-bitcoin-network/src/filter_serving/resource.rs:21`; `packages/open-bitcoin-network/src/filter_serving/resource.rs:193`; `packages/open-bitcoin-network/src/filter_serving/resource/tests.rs:84` |
| T-160-18 | T | Missing versus corruption | mitigate | Missing target is absence; present row missing predecessor or bad commitment is StorageFailure with no successful prefix. | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs:174`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:311` |
| T-160-19 | R | Completion evidence | mitigate | Private non-Clone sequence/length/session-bound receipts consumed exactly once after achieved frame. | CLOSED | `packages/open-bitcoin-node/src/network/filter_transport.rs:519`; `packages/open-bitcoin-node/src/network/filter_transport/registry.rs:260` |
| T-160-20 | D | Suffix lifetime | mitigate | Cancellation drops actual queued Vec suffix allocations; in-flight frame bytes and peer/request slot stay charged until completion/drop. Bound late physical-only receipts and exactly-once release. | CLOSED | `packages/open-bitcoin-node/src/network/filter_transport/registry.rs:27`; `packages/open-bitcoin-node/src/network/filter_transport.rs:192` |
| T-160-21 | T | Lifecycle race | mitigate | Consume Plan13 opaque monotonic read-integrity epochs before every frame/completion; cancel/invalidated late writes record physical prefix only, never live full success. | CLOSED | `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs:98`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs:121` |
| T-160-22 | T | Partial preparation | mitigate | Validate every required row/predecessor/checkpoint and encoded size before returning first frame. | CLOSED | `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:242`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:272` |
| T-160-23 | E | Disabled backend corner | mitigate | Preserve semantic gate ordering then no-backend no-response; all never creates storage. | CLOSED | `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:174`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:194` |
| T-160-24 | D | Request-driven owner effects | mitigate | Immediate read-only bounded turn; no readiness wait,generation,index mutation or repair. | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs:46`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs:166` |
| T-160-25 | R | Inbound achieved evidence | mitigate | Only WriteWireMessageOutcome::Written consumes frame receipt; exact bytes and sequence are observed. | CLOSED | `packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs:217`; `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs:50` |
| T-160-26 | D | Inbound backpressure | mitigate | Plan14 same-registry generic/scratch plus CFactive admission enforces8MiB internally in both orders; inbound1MiB/128 remains additional local policy. Guards survive batch suffix/backpressure until actual buffers drop. | CLOSED | `packages/open-bitcoin-rpc/src/context/inbound_wire.rs:207`; `packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs:20` |
| T-160-27 | T | Inbound lifecycle race | mitigate | Before-write and after-write currentness checks distinguish physical prefix from current full success. | CLOSED | `packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs:198`; `packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs:218` |
| T-160-28 | R | Outbound plan loss | mitigate | Explicitly consume affine filter plans; no dropped/Immediate response path can claim success. | CLOSED | `packages/open-bitcoin-node/src/sync/session.rs:427`; `packages/open-bitcoin-node/src/sync/session/filter_serving.rs:36` |
| T-160-29 | D | Outbound suffix cleanup | mitigate | Same Plan14 ledger guards generic/announcement encoded suffix/scratch and CFframes; cancellation/send failure abort suffix without uncharging handed buffers before physical settlement. | CLOSED | `packages/open-bitcoin-node/src/sync/session/emission_terminal.rs:44`; `packages/open-bitcoin-node/src/sync/session/filter_serving.rs:95` |
| T-160-30 | T | Outbound receipt identity | mitigate | Target/session/sequence/length/currentness checked around actual send; replay and stale full success denied. | CLOSED | `packages/open-bitcoin-node/src/sync/session/filter_serving.rs:70`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs:195` |
| T-160-31 | T | Self-confirming expected bytes and oracle pin/output integrity | mitigate | Independent pinned Python serializers/source arithmetic,exact pin validation,finite case/stdout/golden budgets,strict lossless schemas,mutation controls,raw digest vectors and idempotent atomic generation. No first-party Rust expectations or physical-transport credit. | CLOSED | `scripts/generate-peer-filter-fixtures.ts:310`; `scripts/generate-peer-filter-fixtures.ts:327`; `scripts/basic-filter-oracle.py:67` |
| T-160-32 | S | Synthetic validation claim | mitigate | Genuine consensus acceptance+durable coins/Fjall is distinct from sparse/header-only/legacy negative controls. | CLOSED | `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs:148`; `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs:344`; `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs:497` |
| T-160-33 | R | False transport completion | mitigate | Both actual production directions assert frames,exact prefix bytes,full flag andzero suffix reservations under forced failure. | CLOSED | `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs:311`; `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs:50` |
| T-160-34 | T | Parity and scope claims | mitigate | Scoped guard checks exact source wiring/behavior assertions and mutation-sensitive excluded claims. | CLOSED | `scripts/check-phase160-peer-filter-serving.ts:175`; `scripts/check-phase160-peer-filter-serving.test.ts:45` |
| T-160-35 | R | Premature finalization | mitigate | Preserve historical behavior through actual-corpus and mutation-sensitive guards; focused receipts do not authorize git. Parent reruns unchanged full default native on final sources,obtains clean independent source/security review and lifecycle-valid verification after this plan,then alone may stage/commit/push under Plan12. | CLOSED | `.planning/phases/160-explicit-bounded-peer-filter-serving/160-17-PLAN.md:168`; `scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts:6`; `scripts/check-phase134-authoritative-lifecycle.ts:207`; `.planning/phases/160-explicit-bounded-peer-filter-serving/160-12-PLAN.md:121` |
| T-160-36 | I | Evidence documentation | mitigate | Publish sanitized counters/local commands; omit raw paths/scripts/peer identities or credentials from product evidence. | CLOSED | `packages/open-bitcoin-node/src/network/filter_transport.rs:115`; `docs/parity/catalog/basic-compact-filters.md:642` |
| T-160-37 | T/E | Publication freshness ABA and epoch exhaustion | mitigate | Private checked monotonic shared epoch captured/validated under existing publication guard; raw put/delete, both recovery invalidations and pre-write disable advance it for all clones; overflow permanently denies authority without wrap; trusted append preserves epoch. Tests cover same-generation ABA, shared clones, every enumerated edge, failed disable, overflow and trusted-append stability. | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs:17`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs:12` |
| T-160-38 | D/T | Mixed-traffic admission race and retained-buffer undercount | mitigate | Use the SAME FilterTransportOwner.registry for atomic checked generic retained/scratch plus CFactive totals<=8MiB; remove caller snapshots; bound descriptors128/peer and1024aggregate; guards reserve before encoding/retention and survive queued suffix/blocked write/cancellation until actual buffer drop. Test both orders,barrier races,exact/one-over,shared clones,poison,near-maximum valid blocks and eventual zero after all owners settle. | CLOSED | `packages/open-bitcoin-node/src/network/filter_transport/pending.rs:137`; `packages/open-bitcoin-node/src/network/filter_transport.rs:376`; `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed/race.rs:79` |
| T-160-39 | E/T | Pure action crossing relay-only or cyclic serving dependency | mitigate | Exact data-only request actions and carriers preserve identity; explicit exhaustive relay ignore arm prevents unintended service; outcome definitions live in filter_transport.rs and08imports them. Tests prove all three request mappings,malformed/unsolicited inert cases,intent order and no resolver/success effects. | CLOSED | `packages/open-bitcoin-network/src/peer/message_dispatch.rs:32`; `packages/open-bitcoin-node/src/network/action_translation.rs:222`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:521`; `packages/open-bitcoin-network/src/peer/message_dispatch.rs:454` |

## Verification findings and evidence limits

- Parser lengths/counts are checked before allocation; raw digest order has direct vectors and independent wire-golden comparisons. Short complete requests remain inert, framing/EOF/checksum failures stay distinct, and valid request trailing fields retain pinned semantics. All three request arms now require established VERSION/VERACK. The inbound allowlist admits all six supported names so unsolicited parsed responses reach inert dispatch. Independent review WR-01/WR-02 source corrections were inspected; final warning closure belongs to the source reviewer.
- Startup validation precedes store opening/recovery. Serving defaults false, requested permission is distinct from all expansion, and all-only/no-backend remains a semantic-gated no-response corner. Own per-session services govern authorization, preserving prune NETWORK_LIMITED/WITNESS and independence from catch-up/listener/relay.
- Proof uses private four-u64 work, fixed 256-step division, bounded multiplication, checked target/spacing inputs and exact signed saturation/modular arithmetic. Stale eligibility requires genuine provenance and both strict 2,592,000-second conditions. Parent/height traversal is pre-admitted and charged; stale ancestry never substitutes active heights.
- One publication guard validates the entire target/predecessor/checkpoint set and finite encoding/hash/copy work before any response is exposed. Missing target returns no complete batch; present corruption/missing predecessor maps to StorageFailure. Immediate preparation does not drive indexing, repair, fetch bodies, regenerate filters or wait for readiness.
- Cancellation frees actual queued Vec suffixes; handed bytes and the peer/request slot remain charged until completion/drop. Receipts have private fields and one-use request/session/sequence/length identity, no Clone/deserialization constructor. Memory-only currentness checks include shared publication origin/epoch before writes and during completion. Invalidated late writes preserve physical prefix and deny live full success.
- Both adapters credit only complete frames: inbound Written and outbound concrete Tcp write_all on the original governed slice. Partial-write test sources physically write 13 bytes inbound or 7 outbound; the failing frame earns no complete credit, earlier complete frames retain exact prefix and ownership settles to zero. Historical nonzero execution is in Plan09/10 summaries; this auditor did not rerun it.
- Generic retained buffers and encoder scratch plus CFactive share atomic checked 8MiB admission, 128 descriptors/peer and 1024 aggregate. Buffer Vec drops before its charge; encoding reserves actual peak before allocation and shrinks after scratch drops. The cross-direction sources use public sync_once, concrete Tcp forwarding and the production inbound executor for both exact/one-over orders and eight barrier races. Late mixed/race.rs was freshly enumerated and inspected. Final focused execution receipts are reconciled below; full native execution remains parent-owned.
- Genuine finite fixtures establish consensus acceptance and durable coins/Fjall; independent raw-script BASIC and pinned wire serializers differ explicitly from synthetic protocol recipes, sparse/header-only/legacy negative controls and contextual maximum-weight validation. Test presence closes the declared claim-control mitigation without asserting pending runs passed.
- T-160-31: exact pinned helper/source arithmetic and six-family serializers supply expectations; Bun owns finite subprocess/input/output/golden/case budgets, strict lossless schema/raw-digest/frame checks and staged atomic per-file replacement. No first-party Rust expectation or physical-transport credit is used. Pair publication is not a two-file filesystem transaction; the next check detects interrupted publication.
- T-160-34: the native guard traces actual module ownership, callable production/test bodies, numeric policies, pin and excluded claims. Mutations remove anchors, disable/no-op evidence, substitute enqueue, change policies and omit native corpus routing. Lexical checks supplement actual behavior execution.
- T-160-35: CLOSED verifies the enforced pre-finalization control, not completion of all finalization prerequisites. Plan12 Task3 and the active task prohibit git/completion before full native, clean reviews and every requirement. The strict wrapper inspect_single_phase_result refuses non-passed verification and requires verify lifecycle --require-plans --require-verification before finalize_git. The validator checks generator fields, lifecycle identity and timestamp ordering. Full native, the final source-review addendum and final lifecycle approval were pending; parent must preserve the gate and refresh affected evidence after source changes. SECURITY cannot authorize early git finalization.
- T-160-36: public unavailable/transport evidence uses sanitized categories/counts/frame bytes, retaining backend marker details only in fault fixtures. Product docs publish bounded counters, repo-local Cargo/Bazel commands, explicit equal-work tie policy and measured timing limits. Phase161/162, public defaults, V0/BIP37, archive/production and funds-safety claims remain excluded.

## Summary threat flags

Available summaries 01–11 and 13–17 contain no literal Threat Flags section. Their security/deviation sections were read: epoch ABA maps to T-160-37; builder/handed-buffer lifetime to T-160-20/21/22; mixed admission to T-160-38; pure relay isolation to T-160-39; oracle integrity to T-160-31. No unmapped attack-surface flag was identified. Plan11 final summary reports no new production trust boundary and maps provenance/transport controls to T-160-32/33. Plan17 reports no new production trust boundary and maps historical guard/finalization controls to T-160-35. No literal Threat Flags section or unregistered flag appears in either summary. Plan12 summary remains pending and must be checked when produced.

Unregistered flags: none in available summaries. An unmapped later flag is informational unregistered_flag, never silently treated as an accepted risk.

## Accepted risks log

No accepted risks. All 39 dispositions remain mitigate; no user authorization for risk acceptance was inferred. No transferred risks.

## Fresh new-path inventory

Exact untracked source/test/script/golden paths included in fresh discovery:

- `docs/parity/fixtures/peer-filter-proof.json`
- `docs/parity/fixtures/peer-filter-resource-measurements.json`
- `docs/parity/fixtures/peer-filter-wire.json`
- `packages/open-bitcoin-consensus/src/chain_proof.rs`
- `packages/open-bitcoin-consensus/src/chain_proof/tests.rs`
- `packages/open-bitcoin-network/src/filter_serving.rs`
- `packages/open-bitcoin-network/src/filter_serving/capability.rs`
- `packages/open-bitcoin-network/src/filter_serving/capability/tests.rs`
- `packages/open-bitcoin-network/src/filter_serving/policy.rs`
- `packages/open-bitcoin-network/src/filter_serving/policy/tests.rs`
- `packages/open-bitcoin-network/src/filter_serving/resource.rs`
- `packages/open-bitcoin-network/src/filter_serving/resource/tests.rs`
- `packages/open-bitcoin-network/src/header_store/proof.rs`
- `packages/open-bitcoin-network/src/header_store/proof/tests.rs`
- `packages/open-bitcoin-network/src/header_store/tests.rs`
- `packages/open-bitcoin-network/src/message/filter.rs`
- `packages/open-bitcoin-network/src/message/filter/tests.rs`
- `packages/open-bitcoin-node/src/network/filter_transport.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/encoded_size.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/encoded_size/tests.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/pending.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/pending/tests.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/registry.rs`
- `packages/open-bitcoin-node/src/network/filter_transport/tests.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests/families.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport.rs`
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs`
- `packages/open-bitcoin-node/src/network/tests/filter_capability.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs`
- `packages/open-bitcoin-node/src/sync/session/filter_serving.rs`
- `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs`
- `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs`
- `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_measurements.rs`
- `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs`
- `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/filter_serving.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed/race.rs`
- `scripts/check-phase160-peer-filter-serving.test.ts`
- `scripts/check-phase160-peer-filter-serving.ts`
- `scripts/generate-peer-filter-fixtures.test.ts`
- `scripts/generate-peer-filter-fixtures.ts`

## Security audit trail

| Audit timestamp | Total | Closed | Open | Run by | Result |
| --- | --- | --- | --- | --- | --- |
| 2026-10-11T02:07:42Z | 39 | 39 | 0 | gsd-security-auditor | Scoped declared controls present; independent outer gates mandatory. |

## Sign-off

- [x] All 39 threats classified and verified as mitigate.
- [x] Complete unique registry and source evidence recorded.
- [x] Summaries 01–11 and 13–17 reconciled; pending Plan12 summary disclosed.
- [x] No accepted/transferred risks invented; implementation untouched.
- [x] threats_open: 0 and status: verified apply to scoped mitigation presence.
- [ ] Parent full default native receipt, clean final source-review addendum and lifecycle-valid phase verification completed/reconciled.

Approval: scoped threat-control verification at 2026-10-11T02:07:42Z; phase advancement and git finalization remain conditional on independent outer gates.

## audit-final-source-reconciliation | 2026-10-11T02:19:31Z

| Audit timestamp | Total | Closed | Open | Run by | Result |
| --- | --- | --- | --- | --- | --- |
| 2026-10-11T02:19:31Z | 39 | 39 | 0 | gsd-security-auditor | Frozen-source and completed Plan11 reconciliation; no production mitigation changed. |

The fresh source/docs/scripts inventory now contains 115 changed paths and 52 new paths; generated LOC freshness is part of the outer native contract. Re-read final wire_goldens.rs, completed Plan11 summary, final requested-branch/raw-spend evidence, actual ingress inert controls, extracted mixed/race.rs and final guard/docs/native wiring. Raw-body/UnknownLegacy controls operate at immediate dispatch before legitimate background acceptance; header-only and explicitly seeded old-age controls exercise the actual outbound loop. Seeded age facts are negative evidence only. The guard's content-equality cache retains every source/body check across mutation fixtures and final excluded-claim checks preserve Phase161/162 boundaries.

Refreshed post-formatting anchors: T-160-03 independent 42-case wire comparison now points to wire_goldens.rs:329; T-160-32 competing-branch/raw-spend tests to peer_evidence.rs:344 and :497; T-160-34 guard entry to check-phase160-peer-filter-serving.ts:175. All remaining registry anchors were checked against the current lines. Registry count, dispositions and CLOSED classifications remain unchanged.

| Focused receipt | Recorded completed result | Attribution |
| --- | --- | --- |
| Node phase160_peer_evidence | 8 passed, 0 failed; 81.59s | 160-11-SUMMARY.md |
| Daemon phase160_peer_evidence | 4 passed, 0 failed; final 18.46s | 160-11-SUMMARY.md; parent final receipt |
| RPC library phase160_peer_evidence | 4 passed, 0 failed; 20.61s | 160-11-SUMMARY.md |
| Consensus independent proof/time | 2 passed; 27 proof and 23 defined signed-time cases | 160-11-SUMMARY.md |
| Outbound regression | 18 passed, 0 failed; 11.31s | 160-11-SUMMARY.md |
| Inbound peer-filter regression | 18 passed, 0 failed; 26.79s | 160-11-SUMMARY.md |
| Configured startup regression | 6 passed, 0 failed; 3.64s | 160-11-SUMMARY.md |
| Strict node/RPC all-target/all-feature Clippy | Passed | 160-11-SUMMARY.md |
| Scoped source/claim mutation suite | 740 passed | Parent Plan12 Tasks1/2 receipt |
| Independent fixture generator suite | 54 passed | Parent Plan12 Tasks1/2 receipt |
| Source guard, independent fixtures, parity and managed checks | Passed | Parent frozen Plan12 Tasks1/2 receipt |

Full default native verification is currently running in packages/target/phase160-native-final-1.log; its observed tail shows ongoing earlier-phase mutation checks, not an aggregate completion result. No full-native exit/result is claimed. Final source-review addendum, completed Plan12 summary and lifecycle-valid phase verification remain parent-owned prerequisites before git finalization. This reconciliation launches no Cargo, test or native process and changes only SECURITY.md.

## audit-plan17-integration-reconciliation | 2026-10-11T02:53:57Z

| Audit timestamp | Total | Closed | Open | Run by | Result |
| --- | --- | --- | --- | --- | --- |
| 2026-10-11T02:53:57Z | 39 | 39 | 0 | gsd-security-auditor | Plan17 guard repair, unique T-160-35 ownership and clean 122-path source review reconciled; native2 pending. |

Re-read Plan17 PLAN/SUMMARY, revised Plan12 and decision coverage, final integration source-review finding/addenda, the repaired ordinary carrier ownership proof and connected delegated prefix proof, their live mutations and existing handshake fixture calls. All 17 current PLAN threat-model blocks have exactly39 rows/39 unique IDs. T-160-35 moved uniquely from Plan12 to Plan17; its table row now preserves the exact revised mitigation and cites Plan17 plus implementing guard controls and the final Plan12 gate. Other38 declarations/dispositions/statuses are unchanged.

Plan17 changed exactly eight declared legacy checker/test/README paths, adding no production Rust, dependency, hook or verifier routing change. Source-only inspection confirms Phase123 checks complete governed-buffer collection, concrete exact send before ordinary acknowledgement and Written-only inbound completion. Its ordinaryOwner now classifies the EncodedWireResponse declaration and all carrier fields together. The shared Phase128/134 prefix proof captures the actual reexport and classifies contiguous outer attributes through the existing hasOrdinaryRuntimeOwner adapter; both plain cfg(test) and cfg(test) with an intervening attribute are denied. Actual module/drain/socket/executor connections, only-Written consuming completion and current/suffix/Drop retirement remain checked. Mutations use the current live source, assert a passing baseline and existing replacement needle, then require the intended rejection. No permissive historical/new alternative or comment-only proof was introduced.

Independent WR-03 covered the two related source-item ownership gaps. Four reexport controls reproduced RED (0pass/4fail) and then GREEN (4pass/0fail); two carrier controls reproduced RED (0pass/2fail) and pass within the final90-test suite. The latest source reviewer signed all122 source/doc paths clean at2026-10-11T02:52:03Z and confirmed generated LOC at02:52:37Z, with WR-01/02/03 resolved. Signed scope fingerprint: 209f6d338496e3a8a1b78f8fb70adc743d014da336d9dd86cc85854311cf15e6. This is independent source review evidence, not a native result. Parent's current source inventory is122 paths/52 new paths; any subsequent executable-source change requires renewed reconciliation.

The parent-only Rust preflight found three manually constructed peer fixtures missing the established WR-01 handshake. Changes use only existing complete_peer_handshake calls in peer_fixtures.rs, serving/tests/families.rs and serving/tests.rs, after construction and before requests. Existing semantic/progress/saturation/reservation assertions and product behavior remain unchanged. The source reviewer inspected the exact calls; parent reports all three focused controls passed (1.22s,0.96s,7.55s), followed by the previously unreached four workspace packages. Preflight does not replace full native.

| Integration receipt | Completed evidence | Attribution |
| --- | --- | --- |
| Initial native attempt1 | exit1 after7m55.375s; stopped before Rust at Phase123,39pass/16fail | Plan17 PLAN/SUMMARY and independent integration review |
| Final Phase123 suite | 90pass/0fail,276 assertions,2.49s | Plan17 SUMMARY; independent review |
| Phase128/129/132/134/117 five-suite run | 438pass/0fail,638 assertions,18.19s | Plan17 SUMMARY; independent review |
| Related Phase127/160 pairs | 757pass/0fail,2022 assertions,60.12s | Plan17 SUMMARY; independent review |
| Live historical/related guards | Phase123/128/129/132/134/117/127/160 exit0 | Plan17 SUMMARY |
| Managed and whitespace checks | Zero managed findings; diff check passed | Plan17 SUMMARY; source reviewer |
| Final independent integration source review | Clean122-path signature, all3 warnings resolved | 160-REVIEW.md final integration/LOC addenda |

Native attempt1 is explicitly failed; git and requirement completion remained withheld, providing observed evidence that T-160-35's gate held. The repaired focused suites do not authorize git. The unchanged full default verifier attempt2 is running in packages/target/phase160-native-final-2.log (parent resumable session89438); observed tail is ongoing earlier-phase checks, not aggregate success. No full-native pass, final phase certificate or lifecycle approval is asserted. Parent still owns final native receipt, completed Plan12 summary, lifecycle-valid verification and git gate. This auditor ran no tests/Cargo/native job and changed only SECURITY.md.

## audit-coverage-closure-reconciliation | 2026-10-11T03:40:54Z

| Audit timestamp | Total | Closed | Open | Run by | Result |
| --- | --- | --- | --- | --- | --- |
| 2026-10-11T03:40:54Z | 39 | 39 | 0 | gsd-security-auditor | Final 123-path coverage-only test closure reconciled; production mitigations unchanged; native attempt 3 pending. |

Read 160-COVERAGE-CLOSURE.md and the six reopened test-source deltas under existing Plans01/02/05/06/15: codec filter tests, request policy tests, resource tests, header-proof tests, inbound/tests/permission_policy_cases.rs and inline dispatch fixture. Twelve added tests have independent meaningful boundary/refusal outcomes: malformed receive-only identity/outgoing refusal, enormous allocation-free response sizing/overflow, preserved raw type/stop gate precedence, missing exact proof despite a valid best, unknown/malformed reconstruction facts withholding descendant proof, consumed-budget narrowing that cannot reopen quota, four-byte/nine-byte CompactSize arithmetic, requested/effective permission prerequisites and fixed effect labels. The representability-specific nine-byte usize test is restricted to 64-bit targets; no production line is excluded. The existing handshake loops still cover all three families and valid/invalid inputs before VERSION and before VERACK. Direct construction removes one impossible test-only match arm while retaining invalid types 255/2/1, starts u32::MAX/77, unknown peer and raw identity assertions.

No production behavior, resource policy, protocol gate, coverage exclusion, requirement or threat ID changed. The sole additional existing path is the permission-policy test, assigned to Plan02's eleven-path scope. Source evidence anchors for T-160-01/02/04/11/13/17/39 now also cite the new boundary controls; T-160-03's codec test moved from line13 to14 and was refreshed. All 17 current threat registers still contain 39 unique mitigate IDs; T-160-35 remains uniquely owned by Plan17. All 39 scoped control classifications remain CLOSED, threats_open: 0; no risk acceptance was inferred.

| Coverage/finalization receipt | Observed or attributed result |
| --- | --- |
| Native attempt2 | Failed final zero-uncovered-pure-core-lines gate after 26m 7.784s; preceding formatting,strict Clippy,build,Rust tests,benchmarks,Bazel/provenance passed as recorded by parent/reviewer. Direct log tail confirms uncovered network paths and aggregate failure. |
| Focused boundary closure | 12 added tests and 4 dispatch-contract tests passed,zero failures/ignored; closure document and independent reviewer directly inspected focused log. |
| Initial codec RED | 11 passed / 1 failed preserved; corrected assertion distinguishes outgoing full-payload sizing refusal from incoming CompactSize refusal, without production change. |
| Network-only LLVM coverage | Parent reports --show-missing-lines --text --fail-uncovered-lines 0 exited0, no uncovered lines/exclusions, with 598 unit + 2 parity + 2 property passes. This is a targeted receipt, not full native. |
| Final independent source review | Clean 123-path addendum signed 2026-10-11T03:33:55Z; WR-01/02/03 remain resolved. |
| Source scope fingerprint | 821780ec7e24bbb973c0819db9e6c9d1ab28d241e59155e8c78c799e5c8a96ee; sorted path/NUL/file-SHA256/newline procedure. |
| Current tracked-input LOC | 411,799 lines; fingerprint fd259cea4cf31f8391f218c030fe2974163723aee667539d0c8a7de3b5ceef38 as inspected by reviewer. |

Native attempt2 is a failed complete verification receipt, despite earlier passed stages. Requirement/git completion stayed withheld, again exercising T-160-35's final gate. Native attempt3 is running under parent session 8805 in packages/target/phase160-native-final-3.log; its observed tail shows ongoing scoped mutation checks, not completion. No full-native pass or phase certificate is claimed. Final Plan12 summary, actual successful default-native receipt and lifecycle-valid verification remain parent prerequisites before git. This auditor performed source/document/log reads only and modified SECURITY.md; no test, Cargo, native, shared-state or git operation ran here.

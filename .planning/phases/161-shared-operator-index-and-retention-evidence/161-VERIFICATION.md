---
phase: 161-shared-operator-index-and-retention-evidence
verified: "2026-10-11T11:00:28Z"
status: passed
score: 15/15 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T11:00:28Z"
lifecycle_validated: true
overrides_applied: 0
---

# Phase 161: Shared Operator Index and Retention Evidence Verification Report

**Phase Goal:** Operators can distinguish index capability, initial synchronization, current lag, safe durable progress and failures consistently across existing status, CLI, dashboard and support evidence.

**Verified:** 2026-10-11T11:00:28Z\
**Status:** passed\
**Re-verification:** No — initial formal verification. Supplemental review findings and their corrections were independently checked.

## Verification Basis and Final Gates

Verification starts with all three ROADMAP success criteria and CFOP-01, then merges the 31 truth clauses across all eleven PLANs into the fifteen nonduplicative obligations below. No plan subtracts a roadmap criterion. All D-01–D-16 decisions are mapped separately. Existing execution summaries supply command attribution; implementation conclusions also come from actual source, caller assertions, recorded outcomes and independent final reviews.

The changed-source review baseline is `a7963f943de6a779b6b1dcafd0f42bdd6ded9066`. The explicit scope is the 113 distinct paths in `/tmp/phase161-review-scope.json`, reconciled by the final [source review](161-REVIEW.md). The pinned Knots submodule is `a9aee730466ac67d35a3c03ee24676be5e045878`; local `index/base.cpp` initial-synchronization/summary behavior and `index/blockfilterindex.cpp` retained identities remain the parity anchors.

| Required gate | Actual evidence | Result |
| --- | --- | --- |
| Frozen-source default native contract | Attempt 5: default/full `bash scripts/verify.sh` exited 0. Inner verifier elapsed 1,661,348 ms (27m 41.348s); successful full-mode timing receipt `21971786-6462-4a73-9cab-7adc41627b1e` ended at `2026-10-11T10:43:52.732Z` (wrapper 1,661,866 ms). Source/mutation/panic, format/strict Clippy/all-target build, workspace/explicit CLI tests, benchmarks, six-target Bazel/provenance and pure-core coverage stages passed. [Durable native receipts](161-NATIVE-EVIDENCE.md); log `/tmp/phase161-native-full-5.log`. | PASS |
| Independent full source review | [161-REVIEW.md](161-REVIEW.md): 113 files, zero open critical/warning/info findings; WR-02/WR-03 closed, supplemental WR-01 rechecked | PASS |
| Independent STRIDE/ASVS control review | [161-SECURITY.md](161-SECURITY.md): 35 declared mitigations, 35 closed, zero open implementation threats, zero accepted/transferred risks or unregistered flags; actual approval `verified` at `2026-10-11T10:47:21Z`; closing completed-summary flag inspection at `2026-10-11T10:56:33Z` confirms no new flag, scope expansion or source change | PASS |
| Operator acceptance | [161-UAT.md](161-UAT.md): actual Cargo/Bazel stopped commands and real support files; genuine caller and actual terminal-buffer evidence | PASS |
| Simplification | Final source review plus Plan 09 handoff: existing publication/serialized owner, scalar accounting/fault/session facts and one formatter; no second resolver/store/scanner | PASS |
| Formal provenance | CONTEXT, all eleven PLANs, all eleven completed SUMMARYs and this report share `yolo` / `161-2026-10-11T04-48-29`; no direct-fallback artifact. Latest SUMMARY timestamp is `2026-10-11T10:54:58Z`; security closing inspection is `2026-10-11T10:56:33Z`. This actual closing refresh is newer than both. | PASS |

The first full native attempt failed after 651964 ms at the panic-site policy check, before Cargo verification. The cfg(test) projection child was moved byte-identically from `network/operator_snapshot/basic_tests.rs` to `network/operator_snapshot/tests.rs`, with the parent declaration and parity mapping updated, preserving assertions and production behavior. The byte identity was independently checked as SHA256 `bb80882d322ef3567841b339e23ae50d53a92e13137506956021ea843b8ba85c`. The owning executor's canonical-layout release records eighteen passing authority cases, strict lint, formatting, parity and the unchanged panic guard passing. Root then reran Cargo formatting and LOC generation before the subsequent successful full run above. [Native receipts](161-NATIVE-EVIDENCE.md) preserve both attempts; the failed attempt is not counted as a pass.

A second full attempt failed after 1440476 ms with node results 1631 passed, one failed and three ignored. `filter_index_production_reopen_refuses_each_lone_partial_artifact` still refused safely, but complete forest validation ran before the established absent-state/artifact classification and changed the diagnostic precedence for a lone projection. The typed fail-closed error, unchanged rows/payloads and original diagnostic contract were required to remain intact; The correction restores present-state-only full measurement and retains the existing bounded owner/artifact classification before a candidate empty total can be installed. The unchanged exact refusal test passes, along with 45 startup and 22 recovery cases, strict lint/format/parity. Empty successful absent-state recovery now performs zero full scans; present-state ordering remains unchanged. Refreshed historical guards require the exact state predicate and subsequent owner proof; 283 controls/564 assertions pass, including ten new negative mutations. A third full attempt then passed all 1632 node cases (three existing opt-in ignores), 348 RPC library cases and 87 daemon cases, but failed after 1634308 ms at the black-box network-status schema assertion. Its expected seven-key list omitted the intentional additive `basic_index` field. The legacy composition test retains all seven existing keys, adds the sorted BASIC key, and compares both BASIC and block-relay groups against one actual authoritative capture; no production implementation or original assertion was removed. The exact black-box target passes all three cases; remaining workspace preflight passes 2596 cases (2577 ordinary tests and nineteen doctests), with seven explicit CLI client-binary cases and strict RPC lint/format/parity also passing. These attributed preflights never substitute for the required complete rerun. A fourth full attempt was intentionally stopped with exit 143 after 283445 ms when a separate targeted Phase 127 guard preflight exposed its stale seven-key source literal (15 controls passed, two failed; live check failed). This was a deliberate abort before the known later guard, not a command timeout or an observed Native 4 guard failure. The two historical Phase 127 scripts were refreshed only for the exact eight-key literal and an isolated BASIC-omission negative. All prior controls remain; final focused validation passes eighteen tests/nineteen assertions and the live/managed/diff checks. The final independent source scope is 113 paths. The final successful attempt above supersedes three failed runs and this deliberate abort, and includes the remaining native stages.

The first actual goal report was written at `2026-10-11T10:49:17Z` after completed native/source/security/UAT proof, with lifecycle validation explicitly false while summary sequencing was incomplete. Root then created Plan 09's provisional SUMMARY; the verifier reconciled all eleven summaries at `2026-10-11T10:52:39Z`, and strict lifecycle validation returned valid with no reasons and no stale override. Root truthfully closed all three tasks and CFOP-01 in the [completed Plan 09 SUMMARY](161-09-SUMMARY.md) at `2026-10-11T10:54:58Z`. Security read that actual completed summary and confirmed no flags or scope expansion at `2026-10-11T10:56:33Z`. This closing metadata-only refresh re-read all eleven summary IDs, timestamps and requirement coverage and the matching 113-path review. The frozen source path/content digest remains `31828e43196124e3b6d5e23261006f1ef073b0387ba9e6d6ea8c942182fd260f`, independently recomputed and identical to Native 5. No new source change, test run or behavioral claim accompanies these metadata passes. The report timestamp is later than every upstream summary and the security closing evidence. Global tracker activation and mandatory indexed LOC/hooks/Git finalization remain root-owned and are not preclaimed.

The generated LOC report uses tracked Git files and initially excludes new untracked source. Its current pre-stage totals are not phase-size evidence. Root owns index-aware LOC regeneration and the mandatory full hook verification at finalization.

## Goal Achievement

### Observable Truths

| # | Merged observable obligation | Status | Source and behavioral evidence |
| --- | --- | --- | --- |
| 1 | Existing status RPC, repo-local CLI and terminal dashboard show consistent configured capability, initial catch-up, current lag and safe durable progress from one authoritative projection. | VERIFIED | `network/runtime_authority.rs:153` takes the existing serialized read; `network/operator_snapshot.rs:216` projects BASIC once. RPC `dispatch/node.rs:263` forwards that capture; CLI `status.rs:238` copies its typed group. Genuine daemon and actual binary tests compare entire BASIC groups, including earned latch + later lag and processed ahead of durable. |
| 2 | Missing history, backend/index failure and retained filter availability have distinct actionable categories; redacted support evidence agrees with live operator facts without exposing raw scripts, paths or peer identifiers. | VERIFIED | `status/basic_index.rs` fixed enums/reasons; `operator_snapshot.rs:51,77` maps owner and source-local faults. Shared formatter and support-copy sanitizer preserve independently observed safe facts. Nine fresh private failure captures, hostile-wire exports and actual support JSON/Markdown verify agreement/privacy. |
| 3 | Operators see retained filter growth separately from the soft block/undo prune target, including protection-induced retention; useful filters never imply old block bodies or wallet eligibility have been restored. | VERIFIED | `filters/retention.rs` immutable-row totals/basis; protected bounds are independently projected. Separate prune fields and formatter notes keep growth outside the target. Actual paired deletion, retained stale records, all-handle reopen, body NotFound/no DurableBlock and unchanged-wallet rescan refusal are asserted. |
| 4 | Legacy, stopped, failed and partly absent collection preserve unknown facts; available empty endpoints differ from genesis zero and observed no failure. | VERIFIED | `status.rs:610`, RPC `method/node.rs:126` and `BasicIndexOperatorStatus` use additive unavailable defaults. Stopped `cli/status.rs:332` and failed-call `:372` do not resolve absent JSONC flags or open storage. Contract, collector, legacy replay and actual stopped UAT exercise these distinctions. |
| 5 | Complete recovery counts every unique immutable row, including displaced/disabled/ahead records, and incomplete/failed/over-budget accounting never publishes partial totals. | VERIFIED | `filters.rs:225` accumulates checked lengths in the existing full immutable validation, then completes hidden-projection checks. Present-state startup carries that result under one guard; absent-state startup can install zero only after the existing bounded owner/artifact inspection proves there are no orphan rows/projections. Recovery tests independently enumerate stored rows; exactly-at/one-over row/byte limits continue mandatory validation and expose unavailable measurement. |
| 6 | Achieved publication updates complete totals once; retries/reorg reuse/metadata preserve valid totals, while raw writes and ambiguous effects revoke trusted measurement. | VERIFIED | `append.rs:328,346` derives the delta from already deduplicated prepared rows and installs only after batch success. Compatibility writers in `publication.rs` do the same with bounded within-batch deduplication/conflict rejection. Raw invalidation in `publication/epoch.rs:78`, poison, before/after-commit, clone and reopen tests prevent stale/prospective totals. |
| 7 | Live facts are coherent with full owner/target/branch/generation/publication identity; bounded read-only polling cannot earn progress, heal faults or change protection/storage. | VERIFIED | `operator_snapshot.rs:121` checks accepted versus active target, generation, branch, processed and safe checkpoints, lifecycle, suspension and integrity; equal heights alone do not suffice. `retention.rs:146` copies scalar observations and reads two fixed metadata keys. Genuine growing-history and fault polling compare all nine namespaces, owner and cumulative validation/query counters. |
| 8 | Existing authentication and pinned getindexinfo/getblockfilter result shapes and established request/error ordering remain intact. | VERIFIED | Baseline filter dispatch, method normalization, node query/readiness and transport definitions have no diff against the review base. Source-local fault persistence is the explicitly reviewed fail-closed trust change; this is no broader protocol-parity claim. Three new daemon cases and nine historical daemon cases exercise actual auth and established filter/index behavior; unauthorized status returns 401 with an empty body. The corrected `rpc/tests/black_box_parity/phase127_composition.rs:379` captures BASIC/block-relay once and retains exact additive status keys plus both authority equalities. |
| 9 | Human/dashboard/support use the same ordered semantic rows; hostile unavailable reasons are normalized at every boundary and support leaves its source capture unchanged. | VERIFIED | `cli/status/render/basic_index.rs:13` is the nineteen-row source. Dashboard adapter and support renderer call it directly. Exact serde normalization plus `support/redaction/basic_index.rs` sanitize all fourteen availability boundaries. Actual export tests and original-capture equality cover JSON/Markdown, newlines/ANSI and private markers. |
| 10 | Actual terminal cells preserve all nineteen normal rows and seven critical narrow rows, without changing four bands, eight charts, refresh/actions or the small-window blocker. | VERIFIED | `dashboard/app.rs:133,263` retains layout and safely compacts supplied strings. Twelve real `Terminal<TestBackend>` cases inspect adjacent values at 160x55, 80x30/25, 80x24 and 40x30; oversized critical values use complete-status guidance. Existing action/refresh tests remain. |
| 11 | Public accepted-chain/Fjall callers and separately fresh private producer/replay composition prove coherent consumers; capture generation cannot masquerade as verification. | VERIFIED | Actual daemon construction/authenticated handler and dispatch-backed loopback clients exercise public cases. Private node `operator_status/captures.rs:56` and `rpc_faults/captures.rs:9` first assert genuine setup/category, then compare fresh exact groups to the committed nine-case fixture. Default tests reject stale/missing/private/malformed data; exact env=1 + serial execution alone permits generation. CLI replay is explicitly named and preserves the captured input. |
| 12 | Current docs accurately explain logical basis, protection, safe progress, unavailable facts, exclusions and reproducible repo-local operator commands. | VERIFIED | Both READMEs, status/observability/runtime docs and parity catalog carry current BASIC evidence and fixed limits. Root executed four disposable Cargo/Bazel commands; `--lib` now correctly selects terminal tests. No repair/download/source-datadir mutation or production claim is prescribed. |
| 13 | Native mutation guards reject removed/default-zero/stale/leaking/budget/claim behavior, historical safety guards remain active, and required final gates precede outer finalization. | VERIFIED | New checker pins 28 production seams, six assertive callers, module ownership, freshness, read-only bounds and claims; 34 positive/mutation controls are wired in both verifier lists. Historical 127/155/156/157/159 guards follow actual stronger guarded helpers and exact present-state/proven-empty recovery predicates without dropping prior controls. Final native/source/security/UAT gates above are achieved before this report; commit/push remain root-owned. |
| 14 | Capability is an O(1) lifecycle aggregate; actual resolved indexing/general-serving intent stays distinct from grants, offers and backend health. | VERIFIED | `open-bitcoin-network/src/peer/basic_operator.rs` maintains checked old/new counters on insertion, version/disconnect and removal. Under/overflow stays unavailable. `sync/open_runtime.rs:76` captures resolved startup intent once; serving setters and clones retain separate facts. Explicit/all overlap and unmodeled implicit origins are disclosed; failed potentially achieved host transitions conservatively make indexing intent unknown. |
| 15 | Discovered real immutable backend/integrity faults become sticky failed evidence, revoke prior read/response authority and cannot be cleared by polling/partial work or manufactured by ordinary client refusals. | VERIFIED | Source-local `Cell` observations in `query.rs:223` and `query/peer.rs:45` fold under the held publication guard; `epoch.rs:17` revokes once and preserves strongest category. Projection consumes faults before owner pauses. Complete verified/proven-empty reset sites, actual immediate/final/peer reads, stale handed-frame settlement, recovery and absence/resource/policy/stale negatives are covered. Healthy-start failed disable now calls `invalidate_read_integrity` before every fallible validation. |

**Score:** 15/15 merged truths verified. The first three rows retain the exact roadmap contract. Remaining rows merge plan-specific details without reducing any of the 31 original clauses.

### All Eleven Plan Contracts

| Plan | Original truth clauses accounted for | Obligations / concrete evidence | Result |
| --- | --- | --- | --- |
| 01 | Legacy unavailable; independently available capability/progress/failure/growth/protection | 1–4, 9; typed schema and contract tests | VERIFIED |
| 02 | All-row reopen totals; incomplete/failed measurement unavailable | 5, 7; complete scan/result carry, raw-row expectations and startup/budget/refusal tests | VERIFIED |
| 03 | Retry/reorg uniqueness; raw/ambiguous revocation versus ordinary revision preservation | 5–6, 15; both writers, duplicate/conflict and before/after-commit tests | VERIFIED |
| 04 | Coherent live capture; six failure classes; separate general/explicit/all/implicit scope | 1–2, 7, 14–15; genuine authority matrix and identity controls | VERIFIED |
| 05 | RPC/CLI exact capture forwarding; older/stopped unknown; pinned auth/filter/index schema | 1, 4, 8; schema/collector plus genuine daemon/client tests | VERIFIED |
| 06 | Ordered common human/support facts; safe immutable exports; separate growth/protection/target/body-wallet copy | 2–3, 9; common formatter, fourteen-boundary hostile matrix and actual exports | VERIFIED |
| 07 | Normal/narrow visibility; actual distinguishing cells; unchanged terminal structure/actions/cadence | 10; twelve actual renderer-buffer cases plus model/operator regressions | VERIFIED |
| 08 | Genuine public/private-composed callers; mandatory fresh captures; coherent failures/reopen/stopped; no body/wallet restoration | 1–3, 8–11; public daemon/CLI cases, nine safe producer/replay cases, real paired absence and wallet refusal | VERIFIED |
| 09 | Accurate reproducible docs; mutation/historical guards; full independent/formal gates before finalization | 12–13; source handoff, root UAT, final native/review/security and this report | VERIFIED |
| 10 | O(1) lifecycle scope; actual resolved configuration independent of backend/advertisement | 14; seven network aggregate cases, six node configuration cases and actual startup/transition wiring | VERIFIED |
| 11 | Sticky real read failure; only verified trust renewal; unlatched ordinary refusals; stale authority revoked without unsafe progress/protection/privacy | 2, 7, 15; twelve read-fault controls, corrected healthy-start disable tests and historical regressions | VERIFIED |

### Required Artifacts and Key Links

The read-only GSD helpers were run against every PLAN: 33 artifact declarations and 29 link declarations, including repeated paths. Initially only this deliberately unwritten report/self-link was missing. That self-reference is satisfied by this final artifact. Pattern checks are existence aids; the following substantive wiring was independently traced.

| Artifact group | Substance and actual wiring | Status |
| --- | --- | --- |
| `status/basic_index.rs`, `status.rs` | Fixed safe types, unavailable serde defaults, validated protection bounds; shared exported snapshot field | VERIFIED |
| `filters/retention.rs`, `filters.rs`, `startup.rs`, `ownership.rs` and retention/recovery tests | Existing complete validator → guarded recovered scalar aggregate; bounded status observation; actual recovery/projection/error/empty/budget tests | VERIFIED |
| `append.rs`, `publication.rs`, `publication/epoch.rs`, publication tests | Prepared unique missing-row delta → successful guarded batch → aggregate installation; invalidation/poison/checked sticky lifetime | VERIFIED |
| `network/operator_snapshot.rs`, `network/types.rs`, `operator_status.rs` | Existing network read → owner/publication capture → typed BASIC; genuine accepted/Fjall and pure identity checks | VERIFIED |
| RPC `context/inbound_status.rs`, `dispatch/node.rs`, `method/node.rs`, `tests/black_box_parity/phase127_composition.rs`; CLI `status.rs`, collection tests | One authoritative captured accessor → additive RPC response → single fetched CLI group; legacy/stopped/failure compatibility | VERIFIED |
| Human `render/basic_index.rs`, support `render/basic_index.rs`, `redaction/basic_index.rs`, support tests | Captured safe group → common rows; owned-copy allowlist → both JSON and Markdown; no recollection | VERIFIED |
| Dashboard `model/basic_index_section.rs`, `app.rs`, `app/tests.rs` and private buffer child | Common rows → existing section/model → real renderer; tiny test module declaration discovers actual buffer tests | VERIFIED |
| Node `operator_status/captures.rs`, `rpc_faults/captures.rs`, committed JSON; CLI `basic_index/captured.rs` | Private genuine producers → mandatory exact freshness comparison → committed safe groups → named authenticated response replay → actual binary consumers | VERIFIED |
| RPC `operator_evidence.rs`, CLI `operator_flows/basic_index.rs`, caller evidence | Existing genuine fixture builders → production daemon/runtime/dispatch/HTTP → real clients/exports; independently stated prune and fault limits | VERIFIED |
| `peer/basic_operator.rs`, node `limited_serve.rs`, `sync/open_runtime.rs` | Actual add/version/remove deltas → fixed scalar getter; successful startup/setter intent → captured configuration | VERIFIED |
| `query.rs`, `query/peer.rs`, `epoch.rs`, `lifecycle.rs`, read-fault and serving tests | Actual source failures → existing guard sticky revocation → coherent projection; existing complete reset and rejected stale receipts | VERIFIED |
| Checker/test, both `verify.sh` lists, parity row/catalog, docs, this report | Actual source/claim links → isolated mutation controls → native execution; unique CFOP-01 row retains Phase 162 pending ownership | VERIFIED |

Plan 08 specified a manifest-root include for the node helper. The implementation uses a correct compile-time relative include to the same committed fixture; the CLI helper uses `CARGO_MANIFEST_DIR`. Fresh producer tests compile and compare the intended file, so this is an informational mechanism deviation, not a missing link or observable gap. Root accepted the equivalent link without a source change. Plan 04 also conservatively reports unknown indexing configuration after failed potentially achieved host effects rather than preserving a possibly stale prior boolean; this follows D-04's authoritative-known-only rule.

### Data-Flow Trace (Level 4)

| Consumer / variable | Real source and population | Output | Status |
| --- | --- | --- | --- |
| RPC `snapshot.basic_index()` | Serialized `ManagedPeerNetwork::operator_snapshot`; actual accepted owner + same publication lifetime + stored fixed metadata and scalar totals | `openbitcoinnetworkstatus.basic_index` | FLOWING |
| CLI `network_status.basic_index` | One existing authenticated network-status fetch, typed deserialize; stopped/error paths explicitly unavailable | Shared `OpenBitcoinStatusSnapshot`, human and JSON | FLOWING |
| Dashboard `snapshot.basic_index` | Shared captured status passed to `basic_index_section` and full common formatter | Text/JSON and actual interactive cells | FLOWING |
| Support `bundle.status.basic_index` | Existing capture copied into owned sanitized status; safe available values preserved, reasons allowlisted | Same captured JSON and Markdown files | FLOWING |
| Retained measurement | Complete existing immutable/projection validation or proven-empty artifact absence, plus achieved deduplicated append deltas; no partial delta can reconstruct unknown totals | Count/payload/key-value facts independently of prune target | FLOWING |
| Private failure replay | Genuine fresh private producer independently asserts category, exact committed fixture comparison; explicit response substitution only for network status | Actual authenticated client binaries and all five operator collections | FLOWING — composition explicitly bounded |

No dynamic consumer receives a permanent empty/default stand-in for live BASIC data. Unknown/default groups are intentional unavailable outcomes at compatibility/stopped boundaries, covered by tests.

### Behavioral Evidence and Attribution

No Cargo, Bazel, test job or server was started by this verifier during root's exclusive native run. Behavioral results below belong to executors/root; this verifier inspected actual source assertions, selected success/failure timing receipts and the final native log. Timing-only receipts establish exit/outcome/duration, not discovery counts by themselves.

| Behavior / executed command selection | Actual evidence | Result |
| --- | --- | --- |
| Generation-disabled genuine node capture and fault/accounting selection: node `--lib --all-features phase161_ -- --test-threads=1` | [Caller evidence](161-CALLER-EVIDENCE.md): 88 passed, fresh exact fixture comparisons; inspected receipt exit 0, 116836 ms. Generation is not counted as verification. | PASS |
| Both real binaries/all operator exports: CLI `--test operator_flows` | 11 passed, including six Phase 161 tests and all five historical harness tests; receipt exit 0, 13022 ms | PASS |
| Actual daemon/auth/progress/pruned-retained/reopen: RPC `phase161_operator_rpc` | Three passed; receipt exit 0, 10705 ms; nine historical daemon cases separately pass | PASS |
| Legacy real composition schema: RPC `--test black_box_parity` | Three passed; inspected receipt exit 0, 9790 ms; all old keys/equality retained and BASIC matched to the same captured authority | PASS |
| Actual terminal geometry: CLI library `phase161_basic_dashboard_cells` | Twelve passed; inspected final receipt exit 0, 407 ms; source checks exact adjacent buffer values, bands/actions and all failures | PASS |
| Sticky immutable failure / source negatives / stale receipts: node `phase161_read_fault` | Twelve passed; receipt exit 0, 29688 ms; both immediate and final reads plus managed preparation/state-machine settlement | PASS |
| Healthy-start failed disable: node `phase161_disable_validation` | Meaningful RED exit 101; GREEN two discovered/two passed, exit 0, 13409 ms; eight direct/configured scenarios plus genuine managed path | PASS |
| Corrected disable regressions | 7 query-epoch + 37 retention + 12 read-fault + 18 authority = 74 passing regressions; strict node Clippy exit 0 | PASS |
| Narrow native checker: `bun test ./scripts/check-phase161-operator-index-evidence.test.ts` | 34 passed, 64 assertions, plus live checker; final independent reviewer and default native run confirm it | PASS |
| Actual stopped operator UAT | Four Cargo/Bazel status/dashboard/support commands exit 0; fourteen unavailable fields agree, support writes only requested evidence files, no regtest store created | PASS |
| Final full native | Attempt 5: default/full `bash scripts/verify.sh` exited 0. Inner verifier elapsed 1,661,348 ms (27m 41.348s); successful full-mode timing receipt `21971786-6462-4a73-9cab-7adc41627b1e` ended at `2026-10-11T10:43:52.732Z` (wrapper 1,661,866 ms). Source/mutation/panic, format/strict Clippy/all-target build, workspace/explicit CLI tests, benchmarks, six-target Bazel/provenance and pure-core coverage stages passed. [Durable native receipts](161-NATIVE-EVIDENCE.md); log `/tmp/phase161-native-full-5.log`. | PASS |

The final native log independently totals 4,247 primary Rust passes, zero failures and three existing opt-in ignores across 41 result groups before coverage reruns. Its 4,181 Bun pass records are execution records, not a deduplicated whole-project test count.

The caller window covers 111 distinct selected tests (88 node + 11 CLI + 3 new daemon + 9 historical daemon). Its nine replay rows are internal cases, not nine extra discovered tests. Plan 07's twelve cell cases and Plan 11's corrected 122-case selected set overlap other windows; this report does not sum them into a misleading whole-phase distinct-test count.

## Requirements and Locked Decisions

| Requirement | Source plans | Description | Status / evidence |
| --- | --- | --- | --- |
| CFOP-01 | All eleven PLAN frontmatters | Consistent configured capability, initial catch-up, lag, safe durable progress, missing history/failures and redacted support; retained growth separate from soft block/undo target | SATISFIED by obligations 1–15, source wiring, genuine caller/privacy/terminal/accounting evidence and final gates |

CFOP-01 is the sole requirement mapped to Phase 161; no orphaned phase requirement was found. REQUIREMENTS/ROADMAP remain root-owned and their pre-finalization pending tracker state is not evidence of an implementation failure. CFPR-02 and CFGR-01/02 remain Phase 162 requirements; they are neither claimed satisfied nor absorbed into this phase.

| Decision | Verified implementation / evidence |
| --- | --- |
| D-01 | One shared additive typed BASIC section and existing authoritative accessor; common consumers; pinned filter/index shapes retained |
| D-02 | Separate configured indexing/general serving, explicit/all/offered aggregate; implicit origins are unsupported/currently zero, never inferred from ordinary peers |
| D-03 | Earned owner latch, accepted active target, lag, processed and safe durable endpoints distinct; no-genesis/null/zero and later-lag/reorg controls |
| D-04 | Explicit availability/defaults, stopped JSONC unknown, no unseen CLI override inference, no collection-side activation/recovery/storage creation |
| D-05 | RequiredBody/RequiredUndo/Backend/IntegrityOrInvalidFacts/ResourceBound/Invalidated fixed categories and bounded guidance |
| D-06 | Retained observation independent from owner pause/failure; discovered integrity/backend faults revoke trusted measurements instead of inventing absence/zero/empty success |
| D-07 | Existing authority/publication lifetime and integrity epoch; read-only polls, sticky faults, no progress/protection effects or missing-history repair |
| D-08 | Fixed scalar/enum new facts, exact reason allowlist, captured support equality/immutability and hostile export negatives |
| D-09 | Unique immutable active+displaced rows, exact payload/key-value bases; retries/reorg reuse do not double count; separate soft target |
| D-10 | Existing recovered scalar + admitted achieved deltas; no new DB/crate/dependency/cache; bounded metadata observation, zero history/peer/payload traversal and unavailable incomplete/budget outcomes |
| D-11 | Authoritative inclusive protection bounds separately displayed; no counterfactual bytes or physical target guarantee |
| D-12 | Exact body/wallet nonrestoration copy plus physically deleted body/undo, limited-serving and actual unchanged-wallet refusal controls |
| D-13 | Existing terminal model/layout/style/cadence/actions, four bands/eight charts; actual normal/narrow/blocker/fallback cells |
| D-14 | Genuine accepted/Fjall public callers, explicit private capture/replay composition, fresh nine-case matrix, stopped/legacy and retained/failure/reopen controls |
| D-15 | Current docs/READMEs/parity/breadcrumbs, narrow mutation checker and historical compatibility controls; actual UAT/full native/independent reviews/simplification/formal gates |
| D-16 | Phase 162/v2.5 pending and all exclusions retained; no expanded crash-window, public-default, production/funds claim |

## Disconfirmation, Anti-Patterns and Limits

The audit explicitly checked three failure hypotheses: complete-looking totals after incomplete publication/recovery; discovered query faults masked by healthy owner state; and synthetic/replayed UI evidence presented as genuine live authority. Both publication writers install only after success, verified resets stay within the existing exclusive boundary, sticky source-local faults take precedence, and public versus private composition is explicit. The healthy-start failed-disable regression closes the separate case where no query had discovered corruption before the lifecycle operation.

The 113-path anti-pattern scan found no completion-blocking TODO, placeholder, empty handler or static successful data source. The only TODO token in Rust is the existing `clippy::todo` prohibition. Test mutation defaults, unavailable initial fields and empty collections are exercised controls/legitimate boundary states, not user-visible stubs. All substantive artifact groups are used beyond their declarations.

| Observation | Classification | Consequence |
| --- | --- | --- |
| Node fixture uses correct compile-time relative include instead of planned manifest-root form | INFO, accepted equivalent mechanism | Fresh comparator still binds the committed fixture; future moves must preserve compilation/freshness |
| Private owner/resource classifications and backend/publication fault seams | INFO, bounded evidence strength | Controlled software outcomes over genuine retained facts; no physical-device failure or oversized production allocation claim |
| Private producer capture + authenticated replay | INFO, explicit composition | No claim private faults were injected through a live daemon socket |
| Managed stale handed-frame receipt tests | INFO, actual state-machine/isolated acknowledgement | No claim of a new live socket-write proof |
| 402 retained rows / 4801 payload bytes / 108115 immutable key-value bytes | INFO, independently enumerated local fixture | Includes displaced and body-pruned records; payload is a subset; projections/metadata/Fjall/WAL/SST/compaction/physical allocation excluded |
| Optional recovery observation limits versus incremental updates | INFO, distinct policy | Recovery caps at 2,000,000 rows/256 GiB stop optional aggregation; admitted checked incremental appends are separately bounded, not a permanent aggregate-value ceiling |
| 64 polls at accepted prefixes 16 and 128: two metadata gets / 91 admitted bytes; BASIC JSON <=4 KiB | INFO, structural/local evidence | Nine namespaces/owner/cumulative scans/immutable reads unchanged; zero BASIC peer/record/history/body/undo/lock traversal/filter copies. Whole-capture wall time is not direct lock-hold or archive-scale latency evidence |
| Synthetic genesis/easy PoW/P2SH/maturity-one fixture; manual owner-plan at tip400, prune height20 | INFO, scoped consensus-valid local fixture | These Phase 161 caller fixtures do not exercise default prune-after threshold 1000, ordinary prune RPC planning or automatic scheduling; inherited native automatic-prune regressions are separate evidence. The old body is already outside the trailing 288 serving window, so in-window positive and old-body negative are separate facts |
| Unseen corruption is not proactively scanned by status | INFO, intentional bounded observation | Discovered real read faults persist; no proactive whole-forest scan, repair or download is claimed |
| Legacy whole-file Markdown drift and tracked-file LOC inventory | INFO, documented baseline/tool behavior | New documentation blocks checked; unrelated normalization avoided; final indexed LOC/hook validation root-owned |

No unresolvable implementation gap or accepted risk remains. Later Phase 162 owns the specifically deferred complete continuous validated-chain prune/reopen/RPC/both-transports integrated client proof and CFPR-02/CFGR-01/02. These were never Phase 161 completion obligations and are not scored as failures.

No BIP37/V0, filter import, wallet acceleration, reindex/repair/download, archive-scale, assumeutxo, public defaults/network CI, production service/readiness, funds safety or stronger v2.4 crash-window guarantee follows. The soft target remains logical, not filesystem-size/disk-capacity assurance.

### Human Verification Required

None for the accepted Phase 161 contract. Actual operator commands and exports plus real terminal-buffer tests cover the specified normal/narrow/blocker/fallback behavior. No outstanding subjective appearance, real-time/public-network, external-service or performance-feel claim is required to establish this phase's goal.

## Gaps Summary

All three roadmap criteria, CFOP-01, D-01–D-16 and the eleven plan contracts are satisfied at their stated evidence strength. Supplemental WR-01 and final WR-02/WR-03 are closed. There are no actionable gaps or overrides. All eleven completed summaries have compliant provenance and this final report is fresh relative to every summary and closing security evidence. Root may run its final strict gate, reconcile only CFOP-01/global Phase 161 tracking, then proceed through mandatory indexed LOC/hooks and authorized Git finalization. Actual hook, commit and push success are not claimed here. Phase 162 and v2.5 remain pending.

_Verified: 2026-10-11T11:00:28Z_\
_Verifier: gsd-verifier_

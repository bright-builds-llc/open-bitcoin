---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "07"
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T18:49:38Z"
status: scoped-proof-verified
---

# Configured Daemon BASIC RPC Evidence

The daemon tests use the actual configuration parser, `open_runtime_store`, `open_authoritative_network_runtime`, the opened durable network handle, `ManagedRpcContext::from_runtime_config_with_network_handle`, shared HTTP state and authenticated `handle_http_request`. No detached BASIC index, production dependency or public node test API is introduced.

## Genuine History and Parameters

`DaemonFixture::new` opens an empty store directly, accepts every block through `connect_local_block`, saves ordinary bodies and flushes the real coins fence. It never calls the raw snapshot seeder. After that runtime closes, the production daemon selection and configured BASIC startup open the store. The existing Phase157 `History::seed` remains an explicitly legacy negative control and is never relabeled genuine.

The local recipe uses synthetic genesis, easy valid proof of work (`0x207fffff`), canonical BIP34 script-number heights, P2SH verification and explicit `coinbase_maturity=1`. It passes the existing genuine managed consensus stage/absorb path. It does not establish Bitcoin's hard-coded network genesis identity, standard maturity 100, real-network header history or a public-network synchronization claim. The configured daemon uses regtest and has sync/inbound activation disabled.

Historical coinbase scripts rotate with height and differ from the current block's outputs. A second same-block spend uses the first spend's actual txid. The independent oracle reconstructs BASIC inputs from the real accepted snapshot's positions and undo, checks both historical and same-block scripts, generates the pure commitments and compares exact HTTP hex. Output-only generation is explicitly asserted different. Header display order is reversed raw uint256 order; filter bytes retain encoded order.

## Configured HTTP Tests

| Registered test suffix (`phase159_daemon_rpc_…`) | Actual evidence |
| --- | --- |
| `configured_initial_and_exact_shared_results` | Forty accepted blocks; startup processes heights0–7. Available row succeeds with exact envelope/result and independent oracle; absent accepted39 yields exact indexing -1; unknown yields exact -5; completion reaches39 with exact summary/selection. |
| `later_lag_same_owner_and_independent_summary` | Accept an unflushed successor after initial sync. HTTP actually polls Pending; another summary completes; the same opened handle drives one ordinary turn and wakes the request. Processed2 leads durable coins1, proving the shared authority rather than an unrelated metrics store. |
| `actual_maintenance_worker_completes_pending_http` | The existing one-second daemon maintenance worker receives that configured handle/store, performs the real tick, and completes Pending HTTP. HTTP finishes before `shutdown_always`; no request-owned indexing or second lifecycle worker is created. |
| `retention_actual_paired_prune_stale_and_all_handle_reopen` | Actual legal paired deletion, genuine replacement, stored stale and naturally absent accepted stale row, live-lock refusal, all-handle close/reopen and exact comparisons described below. |
| `failures_known_header_only_is_never_connected` | Real validated header admission without body acceptance; exact -5 never-connected message and unchanged accepted tip. |
| `failures_legacy_raw_seed_is_honestly_unknown` | Raw-seeded missing legacy row yields fixed -32603 before/after reopen; available stored genesis still succeeds. |
| `failures_pending_lifecycle_and_real_append_fence` | Four actual Pending transitions: owner stop, trusted disable, genuine reorg, and public coins writer invalidating append authority followed by actual scheduled publication failure. Every wait settles with exact fixed redacted -32603. |
| `failures_disabled_selection_and_type_precedence` | Actual configured disabled reopen yields empty summary and disabled BASIC; disabled recognized V0 and unknown uppercase type preserve ordering; framework type error wins over malformed hash. |
| `failures_configured_auth_cookie_duplicates_and_scope` | Empty/wrong credentials on malformed JSON complete with empty401 while the shared context is held; password and generated cookie work; duplicate named selection is exact -8; wallet scope is rejected and cookie Debug is redacted. |

Test-only timeouts detect hangs; they do not grant public readiness. The direct owner-stop control is immediate terminal settlement, not a claim that daemon graceful shutdown stops maintenance before HTTP drain. The worker-success test follows drain then worker settlement. Existing daemon shutdown regression tests cover producer ordering/clean-marker refusal.

## Actual Retention Measurements

The final distinct-script fixture starts with **401 genuinely accepted continuous blocks, heights0–400** and fully indexes them. Configuration selects manual pruning through `open-bitcoin.jsonc` (`prune=1`). Height20 is outside the unchanged **288-block keep window**. The explicit existing `flush_applying_prune_plan` owner performs the small deterministic plan; this does not claim the operator `pruneblockchain` flow or automatic threshold eligibility. Default regtest automatic prune-after **1000** is unchanged and not exercised by this fixture.

| Observation | Executed assertion |
| --- | --- |
| Target | Height20, exactly one returned deleted hash |
| Body and undo | Both actual store lookups return None immediately and after reopen |
| Durable state | `load_have_pruned()` true before and after reopen |
| Logical payload bytes before | 451,352 |
| Logical payload bytes after | 450,226 |
| Achieved logical loss | **1,126 bytes**, one actual body/undo pair |
| Original stored stale row | Height400, exact filter/header response retained |
| Additional genuine unindexed acceptance | Height401, accepted after initial sync and displaced before its scheduled index turn |
| Replacement | Eleven genuinely staged/absorbed blocks, heights391–401, equal-height replacement of the current accepted suffix |
| Naturally missing accepted stale row | Original height401 has retained ScriptsValid provenance but no immutable filter; exact ready -32603 diagnostic before and after reopen |
| Active row | Replacement height401, exact filter/header response retained |
| Recovery summary | Exactly BASIC with `synced=true`, `best_block_height=401` |

The extra original successor means **402 original accepts before reorg**, plus **11 replacement accepts**. The missing accepted row is a genuine accepted-then-displaced-before-indexing state, not injected disk corruption. Its diagnostic is the pinned unexpected-missing message; the test makes no claim that corruption caused this state.

Logical payload accounting is actual stored value length accounting, not physical filesystem size, fsync/power-loss proof, allocator/RSS measurement or a capacity guarantee. Earlier exploratory runs used less discriminating scripts and different byte totals; only the final table describes the final fixture.

Before closing, a second `FjallNodeStore::open` is asserted to refuse while the configured handles live. `DaemonFixture::close(self)` consumes HTTP state, its shared context, authoritative runtime (including its retained DurableSyncRuntime), store and config. Every temporary snapshot and extra state clone is dropped before this call; the retention test starts no worker. Only path and block recipes remain. A fresh production store/runtime/context open then succeeds. This is an actual closed-store reopen, not a cloned view.

## Fault Evidence Placement

The public configured tests above exercise the actual HTTP outcomes available through production seams. The private BASIC fault controls are unavailable to the RPC dependency because node `cfg(test)` is not enabled when built as a dependency. No test feature, public raw acceptance/history factory, dependency or fake integrity epoch was added to bypass that boundary.

The parent-approved Task3 replan passed independent checking with zero blockers/warnings. It composes private node query/owner faults with the actual production RPC mapping, configured HTTP fence/lifecycle controls and source wiring. Private publication/corruption injections did not run inside daemon HTTP.

The ten new `phase159_rpc_owner_faults_…` tests live under the existing node catch-up test owner. Each begins with an empty store, genuinely accepts genesis and its child, flushes the actual coins fence, closes the runtime, earns a configured recovered epoch, then genuinely accepts the spending successor. Complete coverage and authentic ScriptsValid history are asserted. No raw snapshot seed grants this fixture acceptance.

| New node test suffix | Actual production boundary and result |
| --- | --- |
| `before_records` | Registered production Pending query → actual ordinary publication BeforeRecords failure → barrier OwnerFailed; genuine history remains ScriptsValid. |
| `before_checkpoint` | Same real owner/query path, distinct BeforeCheckpoint failure and terminal barrier. |
| `before_protection` | Same real owner/query path, distinct BeforeProtection failure and terminal barrier. |
| `after_commit` | Same real owner/query path, actual AfterCommit publication poison and terminal barrier; no failed-write durability guarantee is inferred. |
| `corrupt_target` | Earn actual row, then private backend hash corruption below the earned epoch; `ManagedNetworkHandle::basic_filter_query` returns Storage(Corruption), and all-handle configured reopen refuses. |
| `corrupt_parent` | Same actual authority route with immediate-parent commitment corruption; typed corruption and real reopen refusal. |
| `missing_parent` | Same actual authority route with predecessor deletion; typed corruption and real reopen refusal. |
| `missing_accepted_target_preserves_captured_provenance` | Capture real Pending, earn completion, externally delete target, then actual `complete_basic_filter_read` yields Missing(ScriptsValid, initially_synchronized=true). History survives. Real reopen refuses the inconsistent projection instead of receiving a fabricated clean epoch. |
| `backend_read_is_distinct_from_absence` | Existing per-store cfg(test) control selects BeforeQueryRead at the production point-reader seam. Actual `basic_filter_query` returns Storage(BackendFailure), distinct from Missing/Found; authentic history remains. This is deterministic injected read failure, not a real operating-system/disk failure. |
| `raw_clone_invalidates_final_read` | The actual raw API removes the row and invalidates every clone before effects. Actual final completion returns Readiness(Invalidated); a fresh query returns Storage(Corruption). |

The query-record fault helper holds the existing publication guard and modifies only backend record bytes/removal. It never writes `read_integrity`, installs an owner, mints history or performs recovery. It models external storage tampering below a previously earned epoch; normal raw-writer invalidation remains separately tested. BeforeQueryRead reuses the existing per-store private publication control, with no global fault flag or persistent test key. Every helper/variant/read branch is cfg(test), and the fault API remains crate-private.

Actual production wiring connects these outcomes to public errors: `prepare_filter` calls the context's handle `basic_filter_query`, then `query_failure`/`project_query`; HTTP `finish` awaits the captured barrier and calls the same handle's `complete_basic_filter_read`, then those same projection functions. The five actual dispatcher tests include `phase159_filter_rpc_dispatch_exact_missing_matrix` and `phase159_filter_rpc_dispatch_query_faults_are_allowlisted`. Missing ScriptsValid/ready maps to exact unexpected-missing -32603; Storage/Readiness/UnknownLegacy map to fixed unavailable -32603 with backend-marker redaction. These are existing production mapper functions, not a second test classifier.

Existing private named evidence includes `phase159_basic_point_query_target_parent_corruption_and_missing_parent_fail_closed`, `phase159_basic_point_query_raw_clone_ancestor_mutation_invalidates_every_reader`, `phase159_basic_readiness_owner_publication_faults_wake_terminal_failure`, `phase159_basic_readiness_owner_accepted_before_persistence_failure_settles` and `phase159_basic_readiness_owner_final_read_rejects_raw_integrity_invalidation`. The existing zero-processed summary test exercises the real summary projection against an explicitly installed test progress snapshot; it is a bounded summary unit control, not a configured-daemon zero-progress startup claim. These reused cases retain their own discovery counts and are not credited to the ten-test new selector.

## Mutation and Verification

Replacing the retention plan's `[20]` with an empty plan produced a discovered one-test RED: returned deleted hashes were empty instead of the expected target. Disabling the query read-fault branch likewise produced a discovered one-test RED: the actual query no longer returned BackendFailure. Both mutations were restored immediately. The subsequent nine-test configured suite and ten-test private suite passed. Setup-only compile/import/config mistakes are not claimed as semantic RED.

Final checks and the approved composed fault matrix will be recorded in the companion SUMMARY. Root owns full native/Bazel verification, source/security/lifecycle review, breadcrumb registration, requirement completion and all Git finalization. No hardware/manual check, public networking, wallet funds safety, CFPR-02/CFGR/CFNET completion or broader integrated milestone proof is claimed here.

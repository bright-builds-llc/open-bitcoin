---
phase: 162-real-prune-retention-and-integrated-parity-proof
reviewed: 2026-10-11T18:34:29Z
depth: standard
diff_base: a77edab276a9daa086055aa61e6143ce86bc1218
review_scope: supplemental-frozen-plan03-and-plan13-network-fix
files_reviewed: 9
files_reviewed_list:
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
  - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
  - packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 162: Supplemental Retention Code Review

**Reviewed:** 2026-10-11T18:34:29Z
**Depth:** standard
**Files Reviewed:** 9
**Status:** clean

## Summary

No actionable correctness, security or test-reliability defect was found in the frozen Plan03 retention source or the minimal Plan13 selected-network correction. This is scoped early review, not final phase approval. Source files, the foundation report, Git state and Cargo targets were left unchanged.

The review used previously loaded root AGENTS/sidecar/overrides, architecture, code-shape, testing, verification and Rust guidance, both complete active lesson files, the current Phase162 context, Plan03's configured-reopen and private-intent amendments, and Plan13's source/test summary. New proof files were read in full; the Plan13 production and test changes were reviewed against baseline `a77edab276a9daa086055aa61e6143ce86bc1218`. Relevant production connect/reorg, flush worker, authenticated prune dispatch and automatic planner boundaries were cross-referenced.

## Acceptance and Index Provenance

- `fixtures.rs:59-125` creates ordinary Regtest runtime history with1001 real `connect_local_block` receipts, persists ordinary bodies, flushes actual coins/undo/metadata, and checks all1000 non-genesis undo-derived spent-script facts. Maturity100 is asserted from actual selected parameters. `fixtures.rs:159-211` parses real configuration and opens the actual configured BASIC owner and authenticated HTTP context; the factory does not overwrite runtime parameters after parsing.
- `coins_flush/tests/phase162.rs:136-243` drives the real private maintenance loop through elapsed events and waits for its next production wait before observing completion. It asserts bounded progress increments, a fixed accepted tip, zero peers, durable coins B and a chain-bound safe endpoint; completion must reachh1000 within256 turns. No direct index-turn shortcut is used in this retention scenario.
- `lifecycle.rs:111-151` obtains the actual acceptedh1001 receipt. Processing reachesh1001 while the safe prefix remainsh1000; only the ordinary coins flush and subsequent worker tick promote the saved safe endpoint. `lifecycle.rs:153-226` persists competing bodies, invokes the production serialized reorg with ordinary maturity/flags, checks three disconnected and four connected receipts and exact replacement hashes/heights, observes rewind toh998, then separates processedh1002 from safeh998 before actual flush promotion.
- `phase162.rs:37-103` sends only raw named blocks/parentage to the previously reviewed bounded independent oracle, validates recipe ID and ordered-input digest, and compares canonical and alternate spent-script facts. `lifecycle.rs:228-242` compares all four alternate byte/hash/header triples against that raw oracle, explicitly compares displaced originalh1000 and proves its header differs from activeh1000, then preserves exact stale hash lookup.
- Caller-supplied `AnchoredBlock` work values are inputs to this existing reorg API; the returned transition validates actual blocks and coins effects. The test does not independently prove most-work branch selection or whole-chain Knots consensus acceptance. The inherited Open Bitcoin210000 versus Knots Regtest150 subsidy-halving boundary remains unchanged.

## Manual, Automatic and Durable Retention

- `retention.rs:24-47` supplies configured Basic authentication to the normal HTTP request handler, checks successful HTTP status and parses a bounded response. The protection control at lines50-79 verifiesh8..20 body/undo remain present while the saved required-input protection is unchanged; it allows the shipped planner to delete an eligible earlier prefix rather than inventing an all-or-nothing refusal.
- `retention.rs:218-244` verifies the actual keep-window refusal for target1000, unchanged measured usage on refusal, successful manual target20, paired absence throughh20, and paired presence for every later active block. It checks `have_pruned` and the settled identity rather than treating the RPC return alone as deletion evidence.
- `fixtures.rs:149-199` restricts the reopened configuration to approved manual1 or automatic550 settings and asserts the parsed `PruneMode`. `retention.rs:299-326` consumes the manual fixture, reopens the same path with parsed550, measures actual logical usage above550MiB and invokes ordinary `flush_coins(Always)`. The694 returned deletion hashes are exhaustively checked against activeh21..714 (lines327-330); matching cardinality plus membership excludes extra deletions. Activeh715..1002 are all retained, exactly the288-block trailing window.
- `retention.rs:266-295` creates exactly584 bounded approximately-MiB codec-only body rows with unique hashes outside the accepted chain. They never pass through acceptance, contribute no receipts and are not deletion candidates. Lines249-263 stream per-body canonical length/SHA256d facts; only small commitments are retained. Those facts remain exact before/after automatic deletion and after the consuming reopen.
- `SettledIdentity::capture/assert_unchanged` at `retention.rs:94-181` compares complete durable chain/coins snapshots with deliberately deleted undo excluded, exact coins B, empty transitional heads, saved BASIC checkpoint and every one of1006 active/stale immutable records. Those record comparisons prove preservation against the pre-prune snapshot; they are not described as1006 newly independent oracle calculations. The ordinaryh1001 extension is preserved by this snapshot comparison; explicit independenth1000 and alternate-branch comparisons remain as described above.
- `fixtures.rs:321-342` consumes and drops HTTP/shared context, authoritative runtime and store before returning a handle-free handoff. The maintenance worker is shut down and joined before capture/prune (`coins_flush/tests/phase162.rs:257-272`). Live second-open refusal, successful actual configured same-path reopen, exact logical usage and all body/undo/record comparisons at `retention.rs:300-365` rule out a cache-only reopen assertion. Final shutdown/failure settlement, close, store reopen, deletion and path-absence checks remain explicit at `coins_flush/tests/phase162.rs:273-351`.

## Selected-Network Correction

The sole production hunk is `config/loader/open_bitcoin_runtime.rs:153-167`: the existing Disabled constructor retains its activation/default fields while mapping the parsed chain to the corresponding runtime network and exact default DNS vector. MainnetIbd's non-Mainnet refusal is unchanged at lines169-176; enabled-only overrides remain guarded at lines112-115. Public constructors, default activation and unrelated bounds remain unchanged.

The independent four-network expected cases in `config/tests/sync_configuration.rs:8-307` cover absent/default/disabled/off entries, precedence, exact DNS vectors, enabled-only override refusal/ignoring behavior, independent activation settings and mode guards. `tests/daemon_sync.rs:59-172` checks actual durable/transient consumers, selected wire identity, no preflight or sync worker, zero peers and retained independent activation. The amended `config/tests/blockfilter.rs:327-345` derives expected network/DNS from the selected chain and still compares the complete sync configuration; it does not copy actual sync fields into its expectation. No source workaround weakens the frozen recipe's selected-network equality.

## Verification Evidence and Limits

- No Cargo was run by this reviewer; the final focused retention rerun was active under the executor's shared-target ownership. The parent reports a first complete integrated run passing in200.38s, measured logical usage579,152,814→578,426,929 bytes, configured target576,716,800 bytes and sampled allocated file bytes1,056,243,712. These are attributed execution observations, not independently rerun measurements.
- Source explicitly asserts logical usage decreases while remaining above550MiB and logs `target_attained=false` (`retention.rs:335-371`). Nonactive ineligible pressure accounts for the residual excess. `allocated_bytes` at lines376-389 sums filesystem `st_blocks * 512`; it measures a sample of allocated files, not peak space, RSS, logical accounting or immediate filesystem reclamation.
- Plan13's summary records RED/GREEN selected-network failures and unchanged strong-fixture recovery,91 config tests and14 daemon-sync tests passing, plus strict Clippy. The reviewer inspected the implementation and concrete test assertions without duplicating those Cargo runs.
- Scoped baseline `git diff --check` passed. All nine source snapshots were independently checked for trailing whitespace and recorded below.
- This daemon proof does not directly read raw pending-intent rows or namespace bytes. Successful reopen is not raw-intent-absence evidence. Plan07's concrete node-owned fault matrix and separately executed existing `initialize_intent_` controls remain mandatory.
- Authenticated post-prune client responses, wallet/body restrictions, outbound/inbound transport writes and the remaining fault products belong to later plans. Their pending status is not a defect in this frozen Plan03 source. The final whole-phase source, declared-mitigation, native and lifecycle gates remain required.

## Simplification Review

The implementation reuses one parsed fixture factory, one actual maintenance harness, shared recipe/oracle provenance and one settled-identity comparison. Streaming auxiliary commitments avoids a second large payload cache. The explicit distinction between undo payload loss and preserved chain/coins identity is appropriate. No material simplification requiring source changes was identified.

## Reviewed Source Snapshots

SHA256 values identify the exact reviewed source while later plans may extend the same scenario/factory.

| File suffix | SHA256 |
| --- | --- |
| `tests/filter_index/phase162/lifecycle.rs` | `0dc0300982e189db1d739b0730ff93d032d95981534c5a0fcd231fe51d3353af` |
| `tests/filter_index/phase162/retention.rs` | `9b562a9715db42366decde5f91a0feccd10d453e728afa3e741953d82011a4ff` |
| `tests/filter_index/phase162/fixtures.rs` | `3f0f64e0d84e3c2154f44a8ba5fded7751d21d7618209939b763370139b8712d` |
| `tests/filter_index/phase162.rs` | `1eb3bfdf072efd1feecd94c788bfbd55e0c07bb4dad201cdc305cadd5583d413` |
| `coins_flush/tests/phase162.rs` | `80a52b8f4e633c4d9ff78d53f7365e610d11062aa7c1a8b0f38ccf114e94d359` |
| `config/loader/open_bitcoin_runtime.rs` | `506a67537f37a3f6d3ce2f33b64abf063bd692b4e9e330d258a30967ce2202f0` |
| `config/tests/sync_configuration.rs` | `632b87a5559b597a8c56ec18a9afcdff9ee6cd61a0346efeaf391447bb93e079` |
| `tests/daemon_sync.rs` | `74992724f705facdd800b84f2622ebd66342873c33ae995292cb8a653924a203` |
| `config/tests/blockfilter.rs` | `62a15ac95ff0142aefbb4af01df6e8c08fe0dfc9360103c19a668a60be1f948d` |

***

_Reviewer: gsd-code-reviewer_
_Depth: standard; supplemental frozen retention/config scope_

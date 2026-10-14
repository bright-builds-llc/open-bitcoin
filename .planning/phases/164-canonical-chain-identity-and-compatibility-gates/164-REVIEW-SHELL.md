---
phase: 164-canonical-chain-identity-and-compatibility-gates
reviewed: 2026-10-14T05:17:02Z
review_updated: 2026-10-14T09:22:14Z
depth: standard
scope: partial_rpc_cli_bench_rust
diff_base: 6cf34f1b96cdb7e719752395b2c52c8f531b559c
files_reviewed: 68
files_reviewed_list:
  - packages/open-bitcoin-bench/src/cases/rpc_cli.rs
  - packages/open-bitcoin-cli/src/operator/runtime.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support.rs
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs
  - packages/open-bitcoin-cli/src/operator/wallet.rs
  - packages/open-bitcoin-cli/src/operator/wallet/tests.rs
  - packages/open-bitcoin-cli/tests/operator_binary/compatibility_wallet.rs
  - packages/open-bitcoin-cli/tests/operator_binary/status_sync_soak.rs
  - packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs
  - packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_startup.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/tests/prune_config.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/context/tests/construction.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/dispatch/filter_index.rs
  - packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/network_fixtures.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/permissions.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/storage_fixtures.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs
  - packages/open-bitcoin-rpc/src/http/filter_index.rs
  - packages/open-bitcoin-rpc/src/http/tests.rs
  - packages/open-bitcoin-rpc/src/http/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/envelope_and_resource.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/preflight_and_advertisement.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/reconnect_policy.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/open_bitcoin_rpc.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
  - packages/open-bitcoin-cli/src/operator/soak/runtime/chain_compatibility.rs
  - packages/open-bitcoin-cli/src/operator/wallet/name_resolution.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs
  - packages/open-bitcoin-rpc/src/config/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain/tests/fixtures.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
source_snapshot_sha256: e6fb1285c6716328b75a55774dc460555f08a574eb58c89437150cfbad63f434
initial_source_snapshot_sha256: edb46ede304098e442fa4b1a230d1080d6639498fdd94df2a198f594f802c4ff
source_sha256:
  packages/open-bitcoin-bench/src/cases/rpc_cli.rs: 174e4f86174f22866191060f5a31b853d7952f9ec0df9b0ef25ca7d9c3e3d60e
  packages/open-bitcoin-cli/src/operator/runtime.rs: 3ad11d0d3b9bfbfd7f739e63182119c777d26837ddfaab2515caacea98f28868
  packages/open-bitcoin-cli/src/operator/runtime/support.rs: 34222b50b913e30247ce0d24095957f0e77d18e2584a1f630439f5d6ca1be04e
  packages/open-bitcoin-cli/src/operator/soak/runtime.rs: 5cf958492b573dc9cf4bf15d005311840c23586ce0f118dee2ae963ba2c9c702
  packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs: 93bd3b57464ed6b43a43cead277f007a8de99c7393bea4f5384a7420620694f0
  packages/open-bitcoin-cli/src/operator/wallet.rs: b4d29ed57e7b54a98b97605d24f5408849442c2531e3d520ba4511310dbcae49
  packages/open-bitcoin-cli/src/operator/wallet/tests.rs: 9e15eb95c7a7582454d1c043c1a4a8b661bcb71674bc15472aa2501d3cb51239
  packages/open-bitcoin-cli/tests/operator_binary/compatibility_wallet.rs: 59340236ee3091de69ca9093aeb16a0095c148f4c73cb8640dd47594d39e3e1c
  packages/open-bitcoin-cli/tests/operator_binary/status_sync_soak.rs: 01e02c3ebf19aa0d2d2bcaac17c95a0060b3953499dab4b13e14be72462bda50
  packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs: 5ce0e414f57476d83bc27a09d5a42cead3c6ac3b0912bc6fe661e7ebac97ac05
  packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs: 1b95457de881b2b68b880b4b8ca4e19e38ca05a3f87b231e1437f93622223f63
  packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs: a79856c1007567d6431a9c14f076d130d66330825aeec4532c20673575b7aca2
  packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs: 88e5ac86f36dd3fc059ef46146d3d889ee7bbb2807fb44624f0d04c6ff84877e
  packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs: adaa5772a3fccce0d92b2a23a455e7fc567f1a8416fb2f30c942200111743152
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs: c927b7c71b99190cbf3196a0c97ceff51dd86ae45514e53bd674051a928d3165
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs: 2a96ea915292bce360e3a917ac22dc6359a05ee19dff7b1e75eadb1ad11fd905
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs: a02c299d8ce54bec707ab9ad651a54bcf94d3d142ae3938f1ed0cf7d71f6dd59
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_startup.rs: 2021fa7f0c8851c67335c77254d1aadea3edf98d7e63930b4c2b6a6ab1b1d0f3
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs: 2ed1e3b1804c24fbee16af54542a6ba40cd2a5237229d8372bfab8c515e67c69
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs: 304b9ae10f6829ba0c88afb542341cf8902446555ea6fc07af55072e17cb2d12
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs: a6b6ccc7a3520f54da3495b959f9b7963d6121de0207b53f8a49a6812ec809fc
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs: cb99a5d7ff54e05bdb8affccf6364c01c2183aa7dacfe77ad86105a01b475e27
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs: c5f89846a95989bf32a235f40e27a2edfcc511b1ff11a166b7e87bb6e107aa2b
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs: 2a0896c2a034469758154de22e7dc53a942aa22cef4cb21c591e23c64cdc12e5
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs: 933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs: 22558f5497cc4c7919d0b4b8864519589fc28e547357a31c8718c791f4fce683
  packages/open-bitcoin-rpc/src/config.rs: 18fca8b57111efc693b74073bf9af7e5bf97c79b7c071cf45fdc9ce0ae0e8cec
  packages/open-bitcoin-rpc/src/config/tests/prune_config.rs: e327b2f56e96b71cd7d177a5f2eb4203d92358c544e04e0cb08de3572afd2871
  packages/open-bitcoin-rpc/src/context.rs: b5bb4dad46ad7d5d2d92c795a3cf8c68f783695ba01e276c128183e645756f91
  packages/open-bitcoin-rpc/src/context/mempool_recovery.rs: c583939a2ef9639cd0f494ef4bae05eb489a2d437e95af0d29596d3552b604e8
  packages/open-bitcoin-rpc/src/context/network.rs: df9fde08f21ecaea6d403424a64a35170eaff0a9974d99f1357a8475f27462d0
  packages/open-bitcoin-rpc/src/context/tests.rs: 2c7915f8e95d39f992c706afba9e120fe79ae5233d9a0c4011c94ca8419553bd
  packages/open-bitcoin-rpc/src/context/tests/construction.rs: a828fbb79ae1eea2385108b4ad9045a777aedd15a9955cf3edeb71460ffc3ad6
  packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs: 2fd9c844ef75121d5d64ab7485c2a5b3934fdce1bcd3d9605e44652ff982ddb2
  packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs: 4eae28c6444da6fa4a943064f2157c9e508b525331c5b0731cdf30daff89a948
  packages/open-bitcoin-rpc/src/context/wallet_state.rs: 8b641e69c5a1bbb70a58590f65b8f96210d778919122de0932c41153934788ba
  packages/open-bitcoin-rpc/src/dispatch/filter_index.rs: 08287da62a225b8b0dbccb2a7ab23c883ac6c4afc13241733de76935f2064b3c
  packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs: 6788329f9606e72f5d18e6c9012b0128cc8c98fe1b4f5223d0a7c3081acf66ed
  packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs: f048c4089311fd08dd8976fedb5c7a70c14dca34724ee67b12082667da6a7ea5
  packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs: ddb88d3f8b42c1d4ff1ded1ba2b64e400afbb05851d5ecbdcb4cc58ca67b2982
  packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs: be0d8f26818adc787a35625824c63682f59c883837dee826700438162b381e48
  packages/open-bitcoin-rpc/src/dispatch/tests/network_fixtures.rs: 7e413392913f01bb94f533af52a889414c0cfd5c4ed8fd17dceff0c4fd029e30
  packages/open-bitcoin-rpc/src/dispatch/tests/permissions.rs: 531e449e628ed6d6e253aa799496bb5fb1167046aa1fc3924cade96bdc8e00e9
  packages/open-bitcoin-rpc/src/dispatch/tests/storage_fixtures.rs: 1e6a23fec63a9483c918e3643e97a9a7da707aee692bf6f428dc33500986509a
  packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs: ae712454ea90a7e34cf35fc943018f512859430ef52bb4da80e4d8cf3407fecf
  packages/open-bitcoin-rpc/src/http/filter_index.rs: c5e12fb042f15981b887dd0b5b5040d4853697558425bc3b90851e37acde3b27
  packages/open-bitcoin-rpc/src/http/tests.rs: 328ea4bf642998133b40bbd74c6bddc6df1c8f5dbaab4e785899e5454649dccb
  packages/open-bitcoin-rpc/src/http/tests/filter_index.rs: ac652090e20f6342c254cdc634e04fb10673268b00feac58472f0b4cfa996489
  packages/open-bitcoin-rpc/src/inbound_listener/tests.rs: 85efa76208b1a98941fc7bf104784b9aaf6132e8a7e24cffc5555fed83360912
  packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs: 5812930dbe8324c5a057ca941d1a314d6b0453ce00225b1c15034e458314e0da
  packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs: 756a409773da87e5479275b550fe9b9ea559774dcefab44dd1983f2dfb581221
  packages/open-bitcoin-rpc/src/inbound_listener/tests/envelope_and_resource.rs: c5febdc1cceb813b7688516206f4ca2ae9e89546880e8babaef0e51f9a52611e
  packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs: 761ce8c01424dc77a5e9708db70440250a7f778bc94fc2b11acecb2acc090add
  packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs: abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6
  packages/open-bitcoin-rpc/src/inbound_listener/tests/preflight_and_advertisement.rs: 458a2ce54b0afc9fa854f471ddb1d6ca6bf2c7a50f2f71c612e810ea24551499
  packages/open-bitcoin-rpc/src/inbound_listener/tests/reconnect_policy.rs: b4a69adeb5943f287fb0240f82917a538152128caad5db5f4e6dc4ee937509e8
  packages/open-bitcoin-rpc/tests/black_box_parity/open_bitcoin_rpc.rs: 85e511be49aebd7aa410fb1fab841e40a8930f8dc4d8b06e7f49436b74ba88f5
  packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs: 4fc0a4dfef6b352a527b2e4dec2d65722241da30d2b70a16c86d91b83a58cbf8
  packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility.rs: 5c3fd9c11dd2e4dd8f3c063c2f424a2ba0aabc26d456d2e7c8e3c65eb30f8004
  packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs: ea4b8eb383d26c747f53597d1b46834a771529afbaa09c4ce544da24e6bae31f
  packages/open-bitcoin-cli/src/operator/soak/runtime/chain_compatibility.rs: 6a03585ccf8e64a71b93f0a7dd23fc86e6a9be3f0a17c15706200942d328c4ad
  packages/open-bitcoin-cli/src/operator/wallet/name_resolution.rs: 6fbad107498412e86e595bda2d53fc90dd108ad45e3b663124e9eccf3e7b8500
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs: e8154aa11974399fdb80ed08c77bf20bf9d1b3e4b7740a77dbbbe14b55eb4ac6
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs: 610bf1d251458e361115ac3ea96b1cef3dcb8a3d270fafc6d8b656372943129e
  packages/open-bitcoin-rpc/src/config/canonical_chain.rs: 4e3f77b42c594156a87918190cfaa5ab441c89d08747692b421496a46b5a95aa
  packages/open-bitcoin-rpc/src/context/canonical_chain.rs: 6e5aeca818133c28f90e739919edf02fc42f1bc4bad094cede9c12ebb643fa5f
  packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs: 4a29bc4bcec59e8dd26e55992d47d2f08b5bc7ab87ab2ac6dd6818aeb4b078bb
  packages/open-bitcoin-rpc/src/context/canonical_chain/tests/fixtures.rs: 427e52e21816a6345a7cb8fd53938e0b790ba83adef9e03fb675ea1bd8c4243f
---

# Phase 164: RPC, CLI and Bench Code Review Report

**Reviewed:** 2026-10-14T05:17:02Z
**Depth:** standard
**Files Reviewed:** 68
**Status:** clean
**Scope:** Partial phase review of the exact RPC/CLI/bench Rust paths assigned in `/tmp/open-bitcoin-phase164-review-shell.txt`, including new untracked source. Core/storage/admission, TypeScript checks and documentation are separate review scopes.

## Summary

No actionable bugs, security vulnerabilities or reliability findings were identified in this scope. All reviewed changes meet the applicable quality requirements.

Reviewed product and new-test functions with their callers and shared helpers. Inspected every historical fixture diff for assertion, argument, corpus and receipt preservation. Thirty-one mechanically changed paths additionally passed normalized source equivalence against the base, allowing only fixture-factory names, whitespace and trailing call commas. Other fixture paths contain reviewed test-only startup imports/registration or the disclosed sync-status store-sharing fix.

## Evidence and Boundary Checks

- The shared selector resolver covers all sixteen address/sync pairs; Testnet resolves to Testnet3 and contradictory selectors fail.
- The actual generic ordinary supplied-handle constructor resolves selectors, queries immutable owner identity, rejects unproved/mismatched ownership and requires ordinary support before prune policy, wallet or mempool effects. No test-build exemption was added.
- Ordinary daemon authority has no transient product fallback. Runtime store selection and metrics fallback use ordinary admission before backend open; supplied raw metrics handles cannot start workers while ordinary prerequisites are absent.
- Wallet and mempool ordinary fallbacks preserve typed refusal. Context Display uses fixed categories for backend/authority failures. Explicit public fixture factories and low-level `new` retain documented synthetic/source-derived scope; daemon and operator configuration cannot select them.
- All four CLI product opener sites use `open_ordinary`. Offline controls, wallet lookup and backup refuse before metadata/registry/output effects. Failed lookup cannot become a guessed wallet. Live sends delegate optional wallet selection to authenticated RPC without local registry opening; live authentication/operation failures remain visible.
- Soak enrichment marks resource bounds unavailable without backend reads and preserves prior sync/peer observations. New ordinary startup refusal precedes index, ledger and report creation. Existing synthetic projection fixtures remain explicitly classified.
- Historical fixture arguments and assertions are preserved. The sync-status helper intentionally shares its already open store; its configuration and existing assertions remain intact.

## Verification and Limits

Independently inspected existing successful logs: canonical 11, context 49, config 91, dispatch 86, inbound 94 plus one pre-existing ignored case, HTTP 19, daemon 112 plus one pre-existing ignored case, black-box 3, operator flows 11, CLI library 418 and CLI binary 30. Benchmark all-target compilation completed successfully. `git diff --check` passed. No Cargo job was started by this reviewer.

These are regression and refusal checks, not ordinary canonical bootstrap/work/history acceptance or external Knots differential qualification. All four ordinary modes intentionally remain unsupported in this phase; later prerequisite work is explicitly deferred. Full native verification and whole-phase aggregation remain parent-owned.

## Guidance Applied

Loaded local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, standards index and architecture/code-shape/testing/verification/Rust pages; Phase 164 context, research, Plans 05/06 and Summaries 04–06/09–13 informed the review. Both active lesson files were read completely (7,188 combined bytes; 2,397 conservative estimated tokens). No project skills directory or ignore rule excluded any assigned source path.

The SHA-256 frontmatter pins each reviewed source file. No implementation edits, commits or shared tracker updates were made.

## Native 5 Targeted Assertion Recheck — 2026-10-14T09:22:14Z

**Status:** clean; original 68-file scope and initial review history retained. The parent assigned this narrow recheck to the automation reviewer; the original full SHELL review remains attributed above.

Native 5, session **8391**, passed its Bun suites, panic policy and formatting, then exited **101** after **24m47.226s (1,487,226ms)**. Its actual `clippy::err_expect` failures at original test lines **71, 188 and 236** are preserved in `/tmp/open-bitcoin-phase164-native-fifth.log:8586`. That full-native attempt remains failed.

Exactly three assertions in `context/canonical_chain/tests.rs` now use `.map(|_| ()).expect_err(...)` in place of `.err().expect(...)`: `assert_owner_refusal`, `canonical_chain_ordinary_factory_aliases_never_construct_memory_authority` and `canonical_chain_durable_supplied_owners_preserve_records_and_wallet_selection`. Mapping only a successful context to unit supplies the Debug-capable success type needed by `expect_err`, without adding production Debug or changing APIs. Err values pass through unchanged; unexpected Ok still fails. All failure messages, typed error matches, bounded diagnostics, owner/prune snapshots and durable record/physical-manifest assertions remain identical. The fourth nonlinted wallet `.err()` remains unchanged; no suppression was added.

This reviewer reversed only the three substitutions and required line wrapping to reproduce complete original test SHA `a0efec593741ffbe8147b00bc841a4bed56f85b27d20fe508ccfc0aae9d368a7`. The final SHA is `4a29bc4bcec59e8dd26e55992d47d2f08b5bc7ab87ab2ac6dd6818aeb4b078bb`. Direct comparison with `/tmp/phase164-err-expect-before.rs.txt` and independent hashing of all three old/new function regions match `/tmp/phase164-err-expect-hashes.json`. `/tmp/phase164-err-expect-repair.diff` is the exact scoped change. Every other file in the existing SHELL 68-file scope retains its reviewed SHA.

The parent explicitly retains the current checker corpus at **93 Rust inputs and 246 edges**. This test file was outside that corpus before the repair and remains independently bound by SHELL plus executed Rust tests; no new broad input/fixture allowance or unrelated contract renewal was made. CORE 22-file and AUTOMATION 56-file source hashes remain unchanged.

Parent/executor direct serial timing-wrapped preflight, session **29354**, exited **0** for format, full workspace Clippy with all targets/features and warnings denied, full workspace build, workspace tests and doc-tests, and explicit CLI RPC-client tests. Source logs record **4,449 passing, zero failing, six existing ignored** across forty workspace result summaries; ten canonical RPC tests pass within **386 RPC library passing / one existing ignored**, and the explicit RPC client passes **seven** tests. The whole 355-test guard suite also passes, zero failures, **421 assertions**, **67.88s**. These results are attributed to `/tmp/phase164-rust-preflight-{fmt,clippy,build,tests,rpc-client}.log` and `/tmp/phase164-err-expect-guards.log`; this reviewer did not run those jobs.

Original SHELL aggregate `edb46ede304098e442fa4b1a230d1080d6639498fdd94df2a198f594f802c4ff` and final aggregate `e6fb1285c6716328b75a55774dc460555f08a574eb58c89437150cfbad63f434` hash ordered frontmatter `file:sha256` rows joined with LF without trailing LF. All five failed full-native attempts and the earlier focused path-resolution compile failure remain attributed to their actual attempts. Direct Rust green does not replace parent full native verification, normal hooks or lifecycle closure. This reviewer ran no tests, Git, Cargo, full native verifier or source changes and edited only review artifacts.


***

_Reviewer: gsd-code-reviewer_
_Depth: standard_

---
phase: 165-shared-exact-work-authority
plan: "31"
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
reviewed: 2026-10-17T19:53:18Z
depth: standard
review_scope: final-five-rust-sources-before-guard-data-pins
files_reviewed: 5
files_reviewed_list:
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
conditional: true
five_rust_source_review: clean
readonly_integration_root: authenticated
scoped_native_evidence: authenticated
full_plan_verification: pending
plan_qualified: false
phase_qualified: false
native_qualification: false
---

# Plan 31: Five-Source Rust Integration Review

**CLEAN for the exact five Rust source identities reviewed below, before guard/data/pin work.** No actionable correctness, security or test-reliability defect was identified. Task 1's three unchanged sources retain their scoped 7/0 evidence; the final Phase127/helper sources and unchanged integration root are now independently reviewed and authenticated. Plan, phase, guard/data/pin and final verification acceptance remain pending.

## Authenticated context and scope

The final checked plan matches SHA-256 `806d863b14d91d323cfa27370851b5438e5aef5bfd7606e3b725796fcab09376`. The consumer-cap/ancestry ledger matches `0f4a602319fda27a1b0a19d2e78f40d73b77235be4056f9093c5285c4e8ab736`; its independent PASS matches `8312ebcec99cf9a2c1d845f756af0fdc792086532deceebdc504ebaaefa4be4d`. The previous conditional plan was `d35adfd022c892c9300a6b728d42f97a5d30e78e0494029c6983860ffb3f940f`. The cap ledger matches `da0acf82e7beb08b4737d7b7c5ac9062467033869a3801aac3b64b354f87e683`; its check matches `2763556dc9f7a273d22b338c49980ff0a00840c2a05a8aba0c3f7f58090cc1b1`. Task 1 retains the original sparse-negative / genuine-Dense-positive contract and original full pruning command.

Accepted Summary 33 matches `d385bc67ad696e4cbec0fba84e94e479d583b711c864f6bea108b457573cbe67`. The actual root handoff is `165-33-GUARD-HANDOFF-ACCEPTANCE.md`, SHA-256 `e96d6163b3d4bc18a1bd96015dba2461d1605662e982aabd669852864e35f5d0`. Its historical full/native/closure results authorize the declared dependency handoff only; none is borrowed as verification of changed Plan 31 bytes.

Applicable local guidance was retained from the preceding reviews and current guidance identities were rechecked: AGENTS, Bright Builds sidecar/index, placeholder-only overrides, architecture, code shape, verification, testing and Rust standards. The testing standard was reread for this review. Both active lesson files were canonically resolved, measured and fully reread: 5,230 + 1,958 = 7,188 bytes, summed conservative estimate 2,397 tokens. No archive lessons were loaded.

Task 1's original before and completed three-source packets remain authenticated at:

- `.local/open-bitcoin-dev/phase165-p31-rust-before.json`: 72,617 bytes, SHA-256 `1f31af4491b5febb9e910855d9b7a6dd6755f0eb55dc218d294a13d744f48b58`.
- `.local/open-bitcoin-dev/phase165-p31-task1-review.json`: 115,737 bytes, SHA-256 `7766598fcb173dac11c94e4c8886350b269e233513867d522c71f8632c18b20b`.

The latter packet records source digest `1b5d8f42fe55dcd59152d10277091778595599219f4138c07160dcbecd81be8b`. Review authenticates that field through the full packet hash and independently recomputes each complete before/current source SHA below. Every embedded `before_source` equals its corresponding original before-packet body; every embedded current source equals the actual fixed file. Complete after sources and contextual before/after diffs were reviewed without Git.

| Task 1 source | Before SHA-256 | Current SHA-256 | Current bytes / lines / direct tests |
| --- | --- | --- | --- |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs` | `965d7ab14dd929a99961b56c047d995d523511242351db01e491e44e5fa6801e` | `c26e7b2448a7f18b8e4111271a2f7976722b6366e63cddd5f852c2bc0f000e70` | 19,249 / 546 / 1 |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs` | `4f0e301c0ebb1932fde8dd3a08cc11c50e9ff50012fd3b615c74f77587514ebd` | `9c475cdd66a9bc2004e30bf4839547a77f77d6f387948fbbdd8124d0a7001910` | 18,624 / 524 / 1 |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs` | `304b9ae10f6829ba0c88afb542341cf8902446555ea6fc07af55072e17cb2d12` | `7d11b72eada22f2fe86018b16acf90b6381fa7185a48bc2e6a344c883743ced1` | 18,883 / 546 / 5 |

All are readable and below 628 physical lines. The five parent tests include the existing Tokio test, rather than counting only literal bare `#[test]` attributes.

## Source review

### Sparse data remains explicitly unearned

`fixtures::seed` now mines every parent-linked header under the selected Regtest powLimit before insertion. It retains all 1,002 header ancestors, sparse active heights `[1,500,510,713,714,1001]`, synthetic coins/wallet/physical metadata and empty-transaction retained bodies. Mined header arithmetic does not make those bodies accepted scripts/coins history.

The new `assert_raw_cache_refuses` checks the actual owned raw route. Before connecting it captures the complete core snapshot, physical hydrated positions/coins/counts/undo, a coherent physical coins/B/heads snapshot and the attempted body's physical body/undo, and asserts cache absence. It requires exactly `Operation(Chainstate(UntrustedWorkLineage))` and rechecks all captured facts and cache absence afterward. The refusal correctly precedes empty-body validation; the test does not claim an executed consensus-body rejection.

All three original call sites are migrated: initial sparse population at parent line 151, reopened owner at line 292, and metadata-fault owner at line 303. The initial loop covers every sparse position, not one sampled call. Protected indices 2/4/5 still require actual body and undo presence and now explicitly require absent unearned cache. Deleted indices 0/1/3 retain physical body/undo removal, memory-undo cleanup and absent cache checks.

The parent diff preserves the original under-target, future deadline, pruning-forces-coins, ordered deletion, exact support counters, same-second/repeated operation, physical reopen/leftover-poison, fault-after-unlink/retry, wallet preservation/job and durable-serving/operator assertions. The former positive raw-cache premise is corrected narrowly; it is not silently replaced by an unconditional success. Positive cache evidence remains in Dense.

### Dense uses the same genuine accepted source

`DenseHistory::block` uses the full height's sign-safe minimal script-number byte encoding, actual merkle root, selected header PoW and `block_subsidy(height, params)`. It does not truncate height to u8 or retain an overpaying constant reward across Regtest halvings. The seed binds actual full `SyncNetwork::Regtest.consensus_params()`, explicitly asserts maturity 100, uses P2SH consistently and accepts every height 0 through 1001 through the managed owner.

Only the genuine empty owner is consumed into the same store's empty Fjall core. Positions, UTXOs, counts and undo then come from actual acceptance; no raw snapshot is promoted into authority. The exact same accepted core is extracted, genuinely flushed, rewrapped and consumed by the public fixture constructor with BASIC Disabled. No cache/count writer, raw future header writer, private source flag or production API change is introduced.

Configured assembly uses `assemble_durable_runtime`. The existing explicit enable still asserts Active generation 0, Empty checkpoint and protection from height 0 before maintenance. The disable/re-enable generation 2, final Disabled generation 3, stalled Periodic, protected Always, future-deadline restoration and same-second throttle invalidation assertions remain.

The ordinary duplicate-connect cache path still populates heights 0/1/713/714. Actual pruning must evict 0/1/713 and their memory undo, retain 714's cache and undo, delete the exact ordered 714 physical pairs and preserve exact support counts and reopen/refusal results. No fake cache insertion or physical-only replacement of the positive cache checks was found.

### Root-inclusive expected accounting is independent of usage measurement

The genuine acceptance snapshot is captured **before** physical payload measurement. It requires 1,002 positions and 1,002 actual derived undo entries, including root. Every expected pair size uses the actual recipe block's encoded length plus the corresponding captured derived undo's independently schema-encoded length. All those entries, including root, are passed to the real flush.

`assert_payloads` compares every physical body and undo with those expected actual identities before deletion and absence/retention after deletion. The old root-undo-absent premise and 1001-times-constant formula are corrected to all 1002 genuine entries. Initial total and per-height measured usage must match the independently built sums; deletion bytes are independently summed over pairs 0 through 713, and remaining usage must equal original measured total minus that sum.

The shared bulk filler no longer infers expected undo bytes by subtracting body bytes from its own usage metric. `encoded_undo_size` constructs the versioned schema from public undo facts; its shape was checked against the actual storage DTO/versioned pretty-JSON encoder. Calibration additionally checks the measured physical pair against independent body plus undo encoding. These are source-schema byte-count expectations, not an independent blockchain oracle or new runtime authority.

The test-only files retain their existing shell responsibilities for real persistence/worker/retention effects while using existing domain validators and public typed facts. There is no new production implementation or alternate work/validation engine.

## Actual executed evidence and attribution

This reviewer ran no test/build/import. The following are independently authenticated **executor** logs and argv.

| Stage | Actual observed result | Scope |
| --- | --- | --- |
| Original full pruning baseline | 4 passed / 3 failed / 0 ignored / 109 filtered; exit 101; 13.84s libtest | Two actual high-hash header failures and one actual UntrustedWorkLineage cache failure. |
| First attempted “task1-green” command | Compile exit 101; no libtest summary | Seven compiler diagnostics concerning API imports/block hash and unavailable Fjall `snapshot`; it is not a runtime failure/pass count. |
| Corrected compile checks | Format check exit 0; all-target/all-feature RPC check exit 0, 3.16s Cargo / 3.400s wrapper for check | These are compile/format checks, not final strict Clippy/build gates. |
| Corrected original full pruning module | **7 passed / 0 failed / 0 ignored / 109 filtered**, exit 0; **159.31s libtest / 164.589s wrapper** | All seven pruning/offline/ancestry/Dense tests execute; no new ignore or filtered-zero success. |

Authenticated logs:

- Baseline pruning: `phase165-p31-baseline-1.log`, SHA-256 `b355acaa3f513896a97fc1a862349a44a771d715c22bc1440d117d7ab08cc2ba`.
- Compile-only diagnostic: `phase165-p31-task1-green-1.log`, SHA-256 `bd09efc8cada73cedbb6503f8dd99165eea1d58a58e856f3a1fc50fd83392ef7`.
- Corrected compile check: `phase165-p31-task1-compile-check.log`, SHA-256 `2dd1dfa555fc32e9a9feabe9ac644c47909286d8c6fca81f533f23f2d968a03e`.
- Corrected full pruning: `phase165-p31-task1-corrected-1.log`, 1,419 bytes, SHA-256 `a31c3de2dbf63b50f4a2477f0df4055421e0db95be10ac7e192731bba695e5ea`.

All log paths above are under `.local/open-bitcoin-dev/`. The corrected log visibly includes both 60-second and 120-second liveness breadcrumbs and all seven named passing tests.

The exact corrected argv uses the pinned Bun 1.3.9 executable, `run scripts/command-timings.ts run --key phase165-plan24-tests-automatic-prune -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind tests::automatic_prune`. This preserves the transferred original timing key and full module filter. The 109 filtered tests are explicitly outside this module result.

## Final five-source checkpoint and preserved preliminary review

Before this update, the complete authentic preliminary report was preserved byte-for-byte at `.local/open-bitcoin-dev/phase165-independent-p31-rust-preliminary.md`, SHA-256 `8931f77cc16fae7eed669f24275317e046bce042520fceb3b98487cdf438718d`. Task 1's three complete current and before bodies in the new packet equal the original three-source packet exactly.

The final packet `.local/open-bitcoin-dev/phase165-p31-five-rust-review.json` is 203,168 bytes and matches SHA-256 `232d5d8d80121fca14d1ff81bcaefdc159086d4a753f4c1d867ba81cd8fc7e1d`. It records the ordered source digest `ee277701299d4a2ef7388f90030124feec127bf89d57934b3dc1e49cb2798c4f`. As with Task 1, this field is authenticated by the complete packet identity, with every actual complete source/before body, canonical path, byte count, physical line count and individual SHA independently checked.

| Additional reviewed Rust source | Before SHA-256 | Current SHA-256 | Current bytes / lines / direct tests |
| --- | --- | --- | --- |
| `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs` | `4fc0a4dfef6b352a527b2e4dec2d65722241da30d2b70a16c86d91b83a58cbf8` | `a787810f831b3f68e78d4a85890e9b9fa05aeafade8317aa0e4e1465037fca99` | 22,810 / 626 / 3 |
| `packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs` | `74af2f5d5b1fab50e7a7e685dd78ae71f59045a9705a63e21b837a7fed0c3ee3` | `299555281521d51b7e19cb59cd2467f720357c2fc866b4f8446e0be2bd7c2304` | 16,990 / 490 / 0 |

The shared helper's complete original 16,286-byte / 476-line body is an exact prefix of current source. It has no newly fabricated test. Phase127 has its original parameterized multi-thread Tokio test plus two actual direct negative tests.

The unchanged read-only integration root `packages/open-bitcoin-rpc/tests/black_box_parity.rs` matches 2,768 bytes / 73 lines / raw SHA-256 `e508e3206f10d10956b8611b9d23cce9d635a8b2cc9fcb39752c70490755e07d`. Its literal-bearing source identity independently reproduces `be3edb7097cb757729548961c2357523041546d69be1160a85456834599d6597`. Its canonical path, presence, bytes and raw hash match the accepted 553-entry Plan33 freeze `7df3ad98f538ad505dbfa151dc443d8c68582f3fbe179c773bb4215fba80376b`. The root has zero functions/tests and one exact Phase127 module route. It was not edited and must receive no fake TEST row. Its planned whole-SOURCE row and route-disable control remain Task 3 obligations.

## Phase127 source and assertion review

### Genuine initial model and true close

The initial fixture consumes a genuine empty owner into the same empty Fjall view and opens the public explicit fixture runtime with full Regtest parameters. It admits a named mined model root outside P2P, actually accepts/stores/flushes its body, then sends a legitimate descendant through the original scripted sync path. No raw-reopen owner is silently upgraded.

The original pre-existing shared-context sync observation and deliberately stale startup-height/status override remain. The context observes the actual connected descendant, current block/header fields, false IBD and removal of the stale warning. The constant verification-progress expectation is now 1.0 for the actual accepted height-one/header-one state; it is not replaced with a self-read expected value or a generic success.

Before reopen, the pre-sync RPC context, runtime and store are explicitly dropped. The retained root/body/summary/transport data contain no authority handle. The physical manifest returns only owned snapshots/facts; precise Rust2024 `use<>` prevents an accidental input-borrow lifetime from obstructing actual handle release. No manifest, store or original authority clone is kept to manufacture a true-close claim.

### Complete prefix replay and strict physical handoff

After actual Fjall reopen, `phase127_requalify`:

1. Reads the actual physical active-prefix snapshot and requires saved counts to remain None.
2. Validates **every** retained active body, in saved active order, through a fresh genuine Memory owner with full Regtest maturity 100, P2SH and the actual body's time.
3. Compares every derived complete ChainPosition with its physical position, binding height/hash/header/parent/work/MTP, and compares the complete derived undo map with physical undo.
4. Builds the target from the same store's physical coins view and **only genuinely derived** positions, undo and logical counts; raw metadata/count numbers do not supply authority.
5. Requires the fresh raw target to be unearned, consumes the genuine owner through strict `rebind_into`, then uses the public fixture constructor/checked source graph before any new block mutation.
6. Asserts UNKNOWN sequence for every historical nonroot body.

The positive caller compares the complete physical observation tuple before and after requalification: saved positions/counts/undo/UTXOs, coherent physical coins/B/heads, headers, pruning/index protection, runtime metadata and active-prefix physical bodies/undo. Physical None counts stay None; there is no count writer, source flag, source normalization, missing-history repair or canonical acceptance grant.

The first post-requalification extension is an ordinary actual connection without separate header readmission. The original full maturity suffix is retained at heights derived from the achieved accepted endpoint: offsets 1 through the asserted maturity 100, followed by the original actual mature-coinbase transaction admission. No maturity reduction, endpoint restart at one or skipped suffix was introduced.

### Both typed negatives reach their intended boundary

Both new tests create isolated genuine persisted fixtures, drop all original handles and exclusively reopen before fault injection. They call the same `phase127_requalify` used by the positive.

- The missing-input test removes the actually required retained root pair and requires exactly `PrefixRefusal::MissingBody(actual_hash)`.
- The physical-identity test writes a precise foreign B while leaving actual body data available and requires exactly `PrefixRefusal::Core(ChainstateError::WorkRebindMismatch)`.

Both compare complete post-fault physical observation tuples before/after refusal, return no configured replacement runtime and restore no body/undo/count state. These are exact typed failures, not generic `is_err` assertions or borrowed-handle observations. The observation tuple is an owned logical/physical-fact snapshot, not a claim to byte-for-byte closure over every filesystem/backend file.

### Protocol, RPC and cleanup assertions remain

The actual original request/response order and assertion sites remain: five individual Block checks plus an explicit two-receive all-check give seven serving writes; five Tx checks remain; the three NotFound checks still require exactly one inventory item with exact 127/126/124 object hashes. Mixed Tx/Block/NotFound response indices and Ping/Pong sequencing remain.

The real loopback listener/handshake/compact offer, authoritative seven-write counter, authenticated HTTP RPC, exact result-key sets, shared operator block-relay/BASIC equality, actual mature transaction and context identity are retained. Cleanup now additionally aborts **and joins** the RPC task, shuts down the listener, drops shared context and restarted handles/runtime, then removes the owned datadir. It does not retain a hidden source handle or ignore cleanup errors.

## Exact pure-predicate extraction and actual preview differences

The accepted inert preview proof is independently authenticated at SHA-256 `2c14ccb3bfa399b130edd6457fc94bf227a34e02b4c83b7581fc522a8ef05841`, with authentic-before packet `58c6f22944d587eadfa3a95af5a7b91de701d4f55ce33010737316cfd10f21f4`.

The three complete `matches_block`, `matches_transaction` and `matches_missing` functions move to the existing helper with only `pub(super)` visibility and ordinary separating blank lines. They retain their original signatures and full hash/length expressions, use the already available parent imports and perform no I/O or state/authority changes. All old helper imports and code remain exact. No requalification, physical-data, maturity, network or assertion helper was moved.

Independent reconstruction removes the appended helper definitions and returns the private definitions to the caller, accounting only for the two ordinary separators between definitions. It exactly restores the complete **638-line candidate plus 476-line helper**. This is not a false inverse to the original 455-line semantic baseline.

The actual 626-line caller is **not** byte-identical to the inert 626-line preview `9a397e4215a5b51b2185fd3ab9582b1f2fb0f162b6d7f575bd752e72b1ff01d5`. Its complete diff has exactly three reviewed changes:

- Checked `u64` summary-height conversion to the existing block factory's `u32` height.
- Precise `use<>` on the owned manifest return.
- Constant progress 0.0 → 1.0 for the actual new height-one/header-one model.

The first two resolve the actual compile diagnostics without changing ownership or assertions; the third corrects the actual reached stale numeric premise while retaining the strong progress assertion. No packing, assertion deletion or cap exception was found. The shared helper exactly matches its accepted 490-line preview `299555281521d51b7e19cb59cd2467f720357c2fc866b4f8446e0be2bd7c2304`.

## Additional actual native evidence

Every listed executor log's raw SHA, recorded argv and libtest summaries were independently checked. This reviewer executed none of these commands.

| Task 2 stage | Actual result | Wrapper / libtest seconds | Log SHA-256 |
| --- | --- | --- | --- |
| Initial focused command | Compile exit 101; five compiler diagnostics; no tests executed | 3.816 / none | `006f8ebe7f911b8339b7a0801603f313fc3744121119db637d8f332c6c3239e9` |
| Corrected focused typed negatives | 2 passed / 0 failed / 0 ignored / 3 filtered; exit 0 | 5.993 / 1.03 | `6a4fe5007f613b6d05aa91c20f30753ff033f817cf3af99b81d4949044d93968` |
| First actual unfiltered target | 4 passed / 1 failed / 0 ignored / 0 filtered; exit 101. Exact failure: expected progress 0.0, actual 1.0 | 2.434 / 2.10 | `47291ec4b87352a7051d88187af6bbf602768a0e18fac1fc96af25409b46f63d` |
| Final unfiltered black-box target | **5 passed / 0 failed / 0 ignored / 0 filtered**, exit 0 | **6.529 / 3.27** | `5ecfcb0c25ecd57c49d64607d99d7f19ae631aae51a85a1ae1b802c85a01113a` |
| Format check | Exit 0 | 1.877 / n/a | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| Strict all-target/all-feature RPC Clippy | Exit 0, `-D warnings` | 3.421 / n/a | `40f02c4056541754af4f66451dcd5308dcf6be0c13cec75a72392964fdf63529` |
| All-target/all-feature RPC build | Exit 0 | 0.393 / n/a | `f0788aa36d1c86b6ccdfb7f4042a5574102d82061c046422a2d42e008a1e4710` |

All logs are under `.local/open-bitcoin-dev/`, with exact paths and argv in the authenticated final packet. The focused filter is `phase127_fixture_prefix_requalification`; the final target is unfiltered `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`. Original transferred command-timings keys are preserved. Focused negatives overlap the final full target and are not summed as independent cases. The existing “can target configured Knots RPC” test name does not establish that a pinned Knots daemon actually ran.

Task 1's 7/0 log is reused only because all three complete source identities remain exact. No prior Plan33 source/native receipt is substituted for the new Phase127/helper execution.

## Source-before-pins conclusion and remaining gates

All five exact Rust sources are clean within this independent review, and the read-only parent route is authenticated. This permits the root to assess the **separate** Task 3 source/data/edge/input/pin proposals; it is not their approval or an automatic instruction to activate them.

The seven guard/data mutation owners, complete literal relocation/import/inverse proofs, current source/function/test ownership changes, parent-route and helper weakening controls, input membership/order/fresh pin, actual CLI/test/factory/static/dynamic read union and both fresh full guard matrices remain mandatory. The root must independently review all twelve mutation owners plus the unchanged root dependency and close the final raw/log freeze.

Final 14 default native/fresh coverage/Bazel, complete source/security/formal/lifecycle acceptance and normal-hook Git finalization remain pending. Four canonical ordinary selectors remain refused and networking defaults remain off. Canonical bootstrap/exact reopen/genesis BASIC belong to Phase166, contextual completeness to Phase167 and actual pinned-daemon qualification to Phase168.

This reviewer performed only read-only Python/filesystem/source/log inspection, wrote the authorized exact ignored preliminary snapshot and updated this report. No Git command, Cargo/Bazel/Bun/project import, source/plan/pin/shared metadata edit, staging, commit or push was performed. Any later source drift requires renewed affected review rather than borrowing this five-source result.

***

_Independent gsd-code-reviewer. Exact five-source Rust checkpoint clean; guard/data/pin, matrix and whole-phase acceptance remain pending._

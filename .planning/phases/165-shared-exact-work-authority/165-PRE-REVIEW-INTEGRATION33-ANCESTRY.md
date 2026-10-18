---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T17:28:30Z
depth: deep
files_reviewed: 9
files_reviewed_list:
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase164-canonical-identity/fixture-source.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase165-shared-exact-work/test-support.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: affected-plan33-ancestry-table-relocation-and-exact-data-proposal-before-pins
---

# Plan33 ancestry and table relocation review

## Verdict and exact scope

CLEAN for actual pre-pin packet `.local/open-bitcoin-dev/phase165-plan33-prepin-ancestry-review.json`, SHA256 `bfe2f2a2d3633ca77f5d688f56157719b93b18c936cce39787787b085aec8b5b`, captured `2026-10-17T17:24:40.450Z`, and its exact complete proposal `5c0459d084b3ad0b85413004b68ac7e40c665dfa7cc5f2f13e9de1165f311973`. No remaining blocker was found in the affected source, closure data or relocation proof. These exact source/proposal bytes are suitable for the parent's independent reversal and activation decision. Pins remain old and unactivated at this review; runtime/copied CLI/full-guard/native acceptance is not asserted.

This report resolves the bounded external-ancestry limitation found after the earlier source review. File-local supportedOwnership and enabled-test counting cannot detect a disabled external parent declaration. Fixed-input presence and unrelated function hashes also cannot establish that declaration. The five proposed whole-source ancestor ownership rows now close the relevant seven-Rust branch through the actual selected binary root. No all-repository or manifest-wide ancestry claim is made.

Earlier report `165-PRE-REVIEW-INTEGRATION33.md` remains byte-exact SHA256 `30c83ea744c538f42f49e7a744dea11bf843d52d541692949e6e624371bed954` and applies to historical packet785 only. Its unaffected core/inverse review is reused solely after proving those complete source bytes unchanged here. It does not qualify new source, ancestry, CLI execution or native receipts by itself.

Active Plan33 is `7a53f1405221220074570947f12273afd9f21b2530b65b7cbeeb72adedc2bf3d`; checked ancestry ledger is `1730f48023a0fd7c8a2e7a975ccd7b1f5866dda42e50246b326d1a38d1edbaa1`; independent checker is `d0d6f1e041e84c0ca7e049e327face14b96e575ad104e977604c5e2836c20c78`. Repo instructions/standards and previously loaded complete active lessons remain applicable; no source or shared-state editing is performed by this reviewer.

## Complete current source inventory

All nine complete raw bodies/hash/byte lengths/canonical paths and physical counts match the packet and live files. Six are byte-exact to785; exactly three differ as described below. Authentic original before bodies are also unchanged from785 and its authenticated pre32 baseline.

| File under `scripts/`                                 | Physical lines before pins | Complete current raw SHA256                                        |
| ----------------------------------------------------- | -------------------------: | ------------------------------------------------------------------ |
| `check-phase164-canonical-identity/contracts.ts`      |                        577 | `69fe4bca90cd085e8c1ddfc09eddc66382941b77a3e23ae5e7f8b18684f0fb2e` |
| `check-phase164-canonical-identity/source-files.ts`   |                        137 | `5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832` |
| `check-phase164-canonical-identity.test.ts`           |                        625 | `aaa1d40efc31284db15906d5c25a3643bf8ddbc6061410c06178ba8e23c10cd6` |
| `check-phase165-shared-exact-work.ts`                 |                        269 | `6907035edc08a6d849024c58de7fa52d7898a28313727f1e09b027311b173d6e` |
| `check-phase165-shared-exact-work/contracts.ts`       |                        599 | `b921bee439ac97e4ce7f30192243e5f162689c42a5ea5838ecb4e7357afefc2d` |
| `check-phase165-shared-exact-work.test.ts`            |                        627 | `25a3a4f6a271136b93885bae58b6be9295e12cbbb8dd974410b4210cf508c1d0` |
| `check-phase164-canonical-identity/fixture-source.ts` |                        187 | `cec4709ae363dc7fd7d675288d5d45b696f6ac5106a6f81aa68b14d10af6a7ae` |
| `check-phase164-canonical-identity.ts`                |                        264 | `61a17138fc78fde48bf3f2eb3e29e1300acf0225e403bafed13f1aa2ffe15555` |
| `check-phase165-shared-exact-work/test-support.ts`    |                        183 | `0f10303178fc3cad81296092694d551d6fa2ddfe832f2f58f434a12fdf0b9c2e` |

## Exact three-source forward and inverse proof

Independently reconstructed the entire three-file change from authentic785 bodies and reversed it back to those exact complete originals:

1. Remove the complete COPIED_ADAPTER_MUTATIONS declaration from the end of current165 contracts and append the nine literal ancestry rows to DAEMON_SOURCE_MUTATIONS.
1. Append that complete byte-exact table, with its original declaration and ordering, after the entire existing167-line support source. No existing support byte changes.
1. Add the moved table binding to the old164 test's existing support import and remove only its now-unused contracts import line. Every test body, direct descriptor case and copied CLI loop remains exact.

All three whole-file inverses are exact, including whitespace/newlines. Other six owner bodies are unchanged. The original whole167-line helper/runner prefix, copiedInputs body, CLI14 and DOWNSTREAM64/PEER9/PUBLIC22 tables remain exact. All16 consumer rows,58 fixture rows,26 attribution rows and prior43 daemon rows remain unchanged. Current main stays627; old test625; support183. Proposed fixed row appends would make current contracts618 and old164 contracts621, both within628 without packing or exception.

Independent restricted literal parsing accepts the moved13 rows as arrays of exactly five strings, with identical values and order to785. No mutation function is hidden in data. Authenticated BOTH actual saved exported-value files, `phase165-plan33-copied-table-before-values.json` and `phase165-plan33-copied-table-after-values.json`: each85,132 bytes and SHA256 `1b289de1ba41fd59e4e3a3f1cf2df5e9c34edff6ec0e0e66fe5a3213c3e19c14`. Both parsed values equal the independent literal parse; serialization is compact JSON plus one trailing LF.

The BEFORE capture builder `phase165-plan33-ancestry-preview.ts`, SHA256 `6ea89aa3a01025951c42591be679977a8dc5963181caff58707cbdbbda803595`, imports the actual before contracts and captures their exported table before constructing proposed strings. Proposed execution is not the expected-value oracle. The actual AFTER capture imports the relocated support; saved output log is `c4566372946177118f68eb4b5b2fe3c09951c1617eb126e047f7631471198f1f`, reporting13 cases. Actual comparison receipt `phase165-plan33-ancestry-actual-proof.json` is `a926a59abf8eaf730537b107dc75a13dc60200af72e21f4024c4919d959d416f`. Its readable Python reproduction, SHA256 `f1c9b72fef7ae1710e628f815696636cd481b6c9a24ac58806b259efde49cb80`, is explicitly saved AFTER execution and is not misrepresented as the original executed file. This reviewer inspected it and independently recalculated its important full-byte/table comparisons without running it.

The copied runner still creates a fresh root per case, copies the exact existing14 CLI modules, runs BOTH copied164 and165 entries, separately tests stale PASSED-map bytes, requires actual success/exact exit status and intended diagnostic, rejects syntax/module failures, and cleans up in an ordinary finally. All13 scenario values remain exact. Relocation does not add support to the executable CLI module graph: CLI entries do not import the test support. Actual copied CLI positive/negative execution remains pending pins.

## Five unchanged ancestors and complete relevant branch

Each new ancestor whole-source identity was independently recomputed. All five complete sources match canonical paths, byte counts and raw hashes in BOTH authentic24 freeze `aab55ba52305e212bb17103890557eaea3e07301f0ba18e3b2c085c4520c1c55` and authentic32 Rust-input freeze `1c7801b1c859230b5b9cf28a19681a98500ca534187e8c11f721414b7d69df22`, as well as the current packet/live bytes. There is no new Rust source change.

| Ancestor relative to `packages/open-bitcoin-rpc/src/bin/` | Bytes / physical lines | Raw SHA256                                                         | Proposed literal-bearing SOURCE identity                           | Direct enabled tests |
| --------------------------------------------------------- | ---------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ | -------------------: |
| `open_bitcoind/tests/filter_index/rpc.rs`                 | 11068 / 326            | `d40290e19f903e32bc3695ff181aa9a9fced11ea2a7e7205e3f754a5a04e32eb` | `af2a2c1d2e5e3534d0268c3cce59f7e8815af81fd95e80b57d77b1e807f9a906` |                    3 |
| `open_bitcoind/tests/filter_index.rs`                     | 12812 / 372            | `cb99a5d7ff54e05bdb8affccf6364c01c2183aa7dacfe77ad86105a01b475e27` | `83556274de15e99b223025b067748c33068da9a4af8c2acf727785333ff6307a` |                    9 |
| `open_bitcoind/tests/filter_index/phase162.rs`            | 5181 / 159             | `f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b` | `a733a348362681419691f4695a8fed4f7899f5071247fe51e450abc193974ce4` |                    1 |
| `open_bitcoind/tests/filter_index/peer_serving.rs`        | 12121 / 338            | `2a0896c2a034469758154de22e7dc53a942aa22cef4cb21c591e23c64cdc12e5` | `bf3609fa1c6296aead45a58415de383393d8af392bc30163bd5dceb94ebf03f1` |                    6 |
| `open-bitcoind.rs`                                        | 21307 / 602            | `adaa5772a3fccce0d92b2a23a455e7fc567f1a8416fb2f30c942200111743152` | `adaa5772a3fccce0d92b2a23a455e7fc567f1a8416fb2f30c942200111743152` |                    0 |

Counts3/9/1/6 correspond to actual direct test declarations without ignore or excluding cfg. The binary has0 direct tests and correctly gets NO TEST row. Its initial crate attribute is code at offset0, so raw/source identity coincide.

Exactly nine new route controls have unique actual anchors and distinct cfg(any()) replacements: rpc parent failures/retention; filter_index parent rpc/operator_evidence/peer_serving/phase162; phase162 parent outbound; peer_serving parent evidence; binary root's complete cfg(test)/path/mod tests declaration. Each route's file now has a proposed whole-source `ownership` row. Removing, commenting, cfg-disabling, duplicating or redirecting any relevant declaration/path changes that whole-source identity. No new parsing algorithm or old164 edge expansion is needed. The existing whole-source tests.rs row `133c58d2d2258c7b894a4ecc772e5e2de5c355aea2637ce9770596399df877ba` binds its filter_index declaration at lines165–166 and bridges these parents to the newly pinned binary root. Whole-parent binding also protects the helper module declarations present in those sources.

The new9 rows are exactly appended to original43 DAEMON rows, yielding52. The unchanged current165 strict loop requires each unique actual anchor, passing actual baseline and matching SOURCE_CONTRACTS typed-owner refusal. The nine new controls target actual module-disabling behavior. No dummy calls, fixture authority, count promotion, lineage shortcut or body grant is introduced.

## Exact final proposal and read boundary

Actual live arrays remain original141 SOURCE /74 TEST /135 old164 SOURCE_FILES /478 EDGES /267 INPUT, and current inventory literal remains `0947d5a462b1e01493ab20f1288f51af81a61cf5f306ae66bec262b9dc7400a0`. Parsed complete values/order equal authentic pre-pin originals. There is no pin activation in this review.

The complete final proposal equals `phase165-plan33-ancestry-five-data-proposal.json`, SHA256 `5c0459d084b3ad0b85413004b68ac7e40c665dfa7cc5f2f13e9de1165f311973`:

- SOURCE151: retain every prior proposed146 row exactly, including only original renewals109/137, then append five ancestry `ownership` rows. Total appends versus original141 are10.
- TEST83: retain every prior proposed79 row exactly, append only four ancestor rows3/9/1/6. No binary-root or zero-test helper row. Original74 rows retain exact values/order.
- Old164 SOURCE_FILES140 / EDGES522 remain exactly the previous independently reviewed proposal, including the same six renewals and44 actual function additions. No extra old164 scope is inferred.
- INPUT273: the sole new path beyond the previous272 is `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc.rs` at index259. The other four ancestor files were already read. Filtering this one path recovers the entire previous272 sequence; filtering all six new paths versus original267 recovers that entire original sequence. No old path/order is lost.

Independently recalculated exact JSON-array inventory SHA256 `c1189173084e431bd849a4d1fa75b2797eb6224f4b262cff5d9fd9d98f1ad282`. The read addition is one unchanged RPC parent; it is not a tenth script mutation owner, new module or production source edit. New input discovery remains fixed and pin-first; the stale pre-read refusal and actual applied-row/pin reversal must still be executed and closed by the parent after approval.

## Unchanged inverse and dependency boundary

Old164 contracts, complete working adapter and BOTH entrypoints are byte-exact to the earlier reviewed core. Therefore the canonical pure normalizer, explicit indexed tuple/sparse refusal, original ordered/nonoverlapping unique span resolution, right-to-left replacement, ten Rust/three whole-adapter inverses, unconditional PASSED raw paired-generation binding, independent hardcoded2d/b92/FAA targets, actual currentHash and other seven legacy tuples/bundle remain unchanged. Earlier independently proven actual1c→2d→933/FAA and complete adaptercec→b92 exact reversals apply to these exact same bytes. No source or expected historical identity was renewed by the ancestry/table change.

All17 dependency bodies/canonical paths/bytes/hashes match packet and live files. The original12 identities remain exactly those listed in the earlier report: seven accepted32 Rust files plus live-source support, recipe, process, unowned inbound and lifecycle. Their full raw hashes are `f0f22ea39a5fffe3d607446e4660006f28ea20c8928ae2a8765498feda8609c0`, `81f5b7cc8bb92938a2ff46c5f9ea7a0ac7289b4f598725e98b6c0c5401825393`, `e562c9d771b7248966393a5fac1c9df2085e3d7237ee6e9599deb6f8528dbd7a`, `9d5f5bc20eb75b0d28a33b8c30c65b36b17def0fb25a3dfaf86511fc65f1b2cb`, `1c1e5c2fcba87ea6ffda69c3ecfbe569a309d74bc39c011a0648bdc3afa36513`, `8ca1a8eda7c82c0817bf1fc9c4e62d8b6023af6c0aa86e188e2cf416ee1656a0`, `2450a0e496e5b6a5877d2b890cb9906fdea13390fb45f91677a1f68ba3a7f2c1`, `51d11bc50395c7b1b074f936c77752e1b7f997652dfc42cca03b990da8368248`, `82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497`, `628204b796e112a94f1ca0d24fb17c3fedfce85a655567df17c074634bbc0c26`, `d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca`, `d68dcc1f9a794b91441d3fe9686a55f1a23f94d4d31927059fc6965b1f4a91c2`. Five added immutable ancestor hashes are in the table above. Current raw158 inspectors are unchanged; historical normalization is not applied to current structural ownership.

## Actual receipts and remaining work

Authenticated post-relocation direct inverse log `phase165-plan33-direct-inverse-relocated.log`, SHA256 `00d8be18b8c2ec12f089db6d9eb97e6bbc1ea6ff021c74390e6990c46a3772be`: pinned Bun1.3.9,2 actual passes/0 failures,882 filtered,22 expectations,121ms. This covers the two direct malformed-descriptor cases only. It does not prove copied CLI, new route controls, live guards, full matrices or native modules passed.

Only read-only static/hash/literal/source/inverse calculations and owned Markdown formatting were run by this reviewer. No Bun/project import, Cargo, build, test, source/pin/shared metadata/Git mutation or full matrix was run. Only this new report is written; prior report bytes are retained.

After root's activation/reversal decision, actual stale-inventory-before-new-read refusal, all13 fresh copied cases across both entries,52 current daemon controls with real passing baselines, complete static/fixed/helper/factory/driver/observed dynamic raw union including support/RPC parent and exact CLI14, fresh complete164/165 matrices, BOTH original full fresh daemon/coins Cargo gates, strict RPC checks and full unchanged raw/log closure remain mandatory. No accepted32 native receipt is reused to qualify33. Actual Summary33/root acceptance precede31. Future31/30/14 work, final default native/coverage/Bazel/reviews/security/formal/normal-hook commit/push, CWOR-04 and166–168 qualification remain unclaimed.

```yaml
issues: []
```

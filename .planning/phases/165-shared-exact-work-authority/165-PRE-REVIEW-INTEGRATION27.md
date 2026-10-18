---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T08:38:07Z"
client_date: "2026-10-17"
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
depth: deep
status: clean
scope: "Plan27 eight-tool source and exact inverse review before affected pin renewal"
files_reviewed: 8
files_reviewed_list:
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings: 2
---

# Plan27 Preliminary Independent Source Review

Two bounded source-guard reliability findings block pin renewal for the first frozen packet. The parent confirmed the first and requested review of the executor's independently identified second. No production Rust, source/pin/shared-state/Git mutation or test/build/heavy Bun execution was performed by this reviewer. Only this report is owned. Final whole-phase review remains separate.

## Reviewed Context and Frozen Scope

Approved27, its compatibility plan check and the prior26 diagnosis govern this review. The authentic pre27 eight-source capture independently hashes to `8dc55a3e8565fc0b8da29a948e03e24bfc20457cec441b19478a2b29cd5dcc3f`. Existing repo instructions, Bright Builds sidecar, overrides and applicable architecture/code-shape/testing/verification/Rust/TypeScript standards apply. Both active lesson files were fully loaded earlier in this continuous review:7,188 bytes/2,397 conservative tokens, no archives; no project skills were present.

Every working source matched the first packet at inspection. Ordered SHA-256 over each packet path, tab, full file SHA and newline independently computes to `25fb7562ffbca9f0b9b1926d90feb12927d01255db5fa7c4959507a5721843dc`.

| File                                                            | Lines | First frozen complete-file SHA-256                               |
| --------------------------------------------------------------- | ----- | ---------------------------------------------------------------- |
| scripts/check-phase158-validated-reorg.ts                       | 476   | 862c1852f9d448b7a6bb970f6903df5b91648c59998956c79f1b9c4d8a4269fa |
| scripts/check-phase158-validated-reorg.test.ts                  | 626   | 5c94dfcfe0f93724ea7b1da66632cab93ce05c729172fe62fa892eb1c3690284 |
| scripts/check-phase162-integrated-filter-proof.ts               | 434   | 5ffd8e4ea3d54197cd88923cbae2375f977fefe755e2258bd24c3450ec4f7c44 |
| scripts/check-phase162-integrated-filter-proof.test.ts          | 623   | 2932b0ee2de0ac1d600d454590fcbe2e9281f748bcb13c50048c7480da63dfcf |
| scripts/check-phase164-canonical-identity/historical-tooling.ts | 213   | 5f05a96d7fb74a9bfe31d52b9528f624fa54983bb6f031829375657921c0e5b6 |
| scripts/check-phase164-canonical-identity.ts                    | 245   | 9371929a6f0c6c3d1ae5c0489c9d49ac90606619af9a666c3a87e99334a32195 |
| scripts/check-phase165-shared-exact-work/contracts.ts           | 502   | 4f08c8a43b2c8e9b8ede4fedad5edc6565c7de1b3df9a12f696e70ab39ef1c8d |
| scripts/check-phase165-shared-exact-work.test.ts                | 625   | 7c75418b334bb30cabec224d85e32ea2aca22299e914c8a7030df585029c24c5 |

## Warnings

### WR-27-01: Selected impl configuration can hide the consuming capability

**File:** `scripts/check-phase158-validated-reorg.ts:270` (`maybeBoundary`, selected impl start/range) and `:333` (method ownership check).

**Issue:** The generic reorg `absorb` selector validates the method's ownership but not the matched impl's attributes. `ordinaryRust` masks test-only functions/modules, not impls. Frozen159 `supportedOwnership` inspects declaration/file/inline-module configuration, not an enclosing impl. Consequently adding `#[cfg(any())]` or `#[cfg(test)]` immediately before `impl<V: CoinsView> ValidatedChainstateReorg<'_, V>` still selects its unchanged `absorb(self)` and satisfies current lexical checks, even though that consuming production path is disabled. This is a static call-chain finding; no reviewer-run mutation result is claimed.

**Fix:** Immediately after identifying `const start = headers[0].index`, require `supportedOwnership(source, start, Boolean(contract.maybeTest))` or return undefined, preserving the existing unique generic selector and balanced range. The existing helper's third argument is testConfiguration: false rejects both configurations. All three current impl-selected contracts explicitly use maybeTest=false. Retain the method check. Add both selected-impl configuration negatives with passing raw baseline, changed-source assertion and the exact stage.rs:absorb boundary finding. No unowned helper change is needed.

### WR-27-02: Split WorkAuthority anchors omit Boolean operators

**File:** `scripts/check-phase158-validated-reorg.ts:168` (`WorkAuthority::check` contract).

**Issue:** Ordered separate substrings require owner/preview/generation/tip operands and return statements but do not bind the complete Boolean conditions. Preview `&&` can become `||`, or stale generation/tip `||` can become `&&`, while every separated anchor remains present. Those changes weaken actual source authority and must be rejected by the current semantic guard. The executor identified this gap independently; reviewer source inspection confirms it.

**Fix:** Require the three complete executable if clauses, including braces and outcomes, in their actual order: owner inequality returns StageOwnerMismatch; `allow_preview && matching_preview_seal` returns Ok; `generation_mismatch || tip_mismatch` returns StaleStage. Existing whitespace compaction supports rustfmt without loosening operators. Add changed-source selected-owner controls for owner negation, preview &&→||, generation negation, stale ||→&& and tip !=→==, with passing baseline and exact work_authority.rs:check finding.

## Other Reviewed Boundaries

The current raw semantic core is called by the ordinary158 checker; claims and historical tooling remain separate. Source rows trace checked public absorption→validation→owner/generation/tip check→consuming reorg capability→private effect callee. Actual effects install overlay/active/undo/counts, accept the binding, then create the accepted receipt. Authorized Node preparation checks before staging and before history/BASIC; unqualified prepare is required to be the exact PreferenceAuthorizationRequired refusal. Commit checks authorization, acceptance availability, stage and BASIC before validated absorption, installs candidate binding, then publishes history/BASIC and persists with error propagation. The two findings above concern source-guard coverage, not a discovered production Rust change.

The stage definitions have exactly eight and three engine-parent `pub(super)` fields; the validated capability's owner/staged fields remain private. The checker retains other private proof policies and only the existing exact append identity exception. Current field controls select actual spans and use edit()'s before/after inequality; legitimate engine-parent scope is a positive rather than a no-op negative. Missing/extra/duplicate/moved fields, broader visibility, public effect helper and nonconsuming absorb controls remain meaningful. Parent module registrations and child declaration/file configuration are checked; cfg/path/duplicate/commented registration controls remain.

The four behavior dispositions select the actual stronger unflushed return, weaker-shorter refusal, explicit ancestor disconnect and explicit full disconnect cases. The latter wrappers retain their assertive helper; required behavior controls remove/disable/rename declarations, disable files and remove assertions across helper sources. Other original checkpoint/IfNeeded/no-snapshot/fault/physical/retention/test rows remain. New current dependencies remain fixed raw inputs with existing breadcrumb checks; production Rust is never replaced by a historical projection or accepted by known source digest.

## Independent Exact Attribution and Data Proof

Lightweight byte calculations independently applied each unique complete new contextual fragment in packet order. Five checker158 blocks reconstruct authentic pre27 `385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd`; seven test158 blocks reconstruct `bebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3`. Two checker162 blocks reconstruct `875460713003992a2701dc1f5ea300430eb2991b30e3635fefedb8bf387ce877`; one test162 block reconstructs `3ab730fcb1998459f410dfb3a60bb580d0fe67e8e47c8c76835080ad2062acac`. Each intermediate is byte-identical to its authentic pre27 complete source.

The original26 LEGACY_TOOL_WIRING array literal is byte-identical to pre27. Its eight/fourteen blocks then reconstruct complete CAF9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350 and784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5. New outer replacements use callbacks so previous literal dollar syntax cannot become replacement substitution. Unique fragments plus complete intermediate/final SHA equality refuse unknown, partial, duplicated, extra, mixed or reordered candidates; failed recovery returns original raw input. Historical manifest/snapshots/receipts and daemon FAA claim view remain separate. Only the two158 frozen fingerprint operands use exact legacyToolingBase; provenance stays raw, and the selected-file tests require their actual frozen diagnostic rather than unrelated version failures.

Independently deriving the original parity iteration from authentic pre27 CONTRACTS/TESTS produces the exact fixed35 entries, including repetitions, and compact JSON SHA `22c43450a9a707ae80da237a5cb1922d0da84c0159366a51d261ad75f63e1a7f`. Each original path has a removal control requiring its own parity-evidence diagnostic. New current source inputs do not acquire an old CFIX qualification claim.

The three moved document-overclaim pairs independently equal the original ordered literal, compact JSON SHA `460619b13661448038e8337c309b1b85e85227797aa9a7446e42138b3b84d8ee`; the existing loop still creates six executable document/catalog tests. This is data-only relocation within already-owned contracts. All eight files fit628; inventory remains241 with pin `c65211b1e0cafdeaf20e6ee841c6e0ff9d84e98c462132619d6f2bd01385e7d0`, with no new CLI module. Current164 entry remains pre27 byte-identical at this packet.

Original26 mixed8 data still hashes `1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c`, and its pure fixture-source adapter remains `b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3`. Plan27's162 delta is exactly the import and two-only fingerprint routing; the existing raw CF/dead-code/native/31-byte diagnostics and function view wiring are preserved.

## Execution Attribution and Remaining Closure

The parent/executor reports four fresh Rust cases each1passed/0failed/0ignored, and the final first-packet semantic suite87passed/0failed/224expectations in68.70s. The reviewer read that actual87-case log; the earlier82-case run is not closure for later additions. Both generator setup diagnostics and the subsequent approved literal-callback regeneration remain retained execution history; neither diagnostic is runtime/source-semantic proof.

The first packet is not approved for pin renewal because WR-27-01/02 remain open. Bounded checker/test fixes require new actual selected negatives/positive evidence, exact inverse regeneration, cap and complete-hash re-freeze, followed by independent closure. After that, only the two current historical-tooling SHA literals in164 entry and165 contracts may change to the independently reviewed final adapter digest. Full158/162/163-native/164/165 matrices, complete input/module freeze, resumed26 closure and final native/coverage/source/security/formal/phase gates remain pending. No historical qualification, canonical reopen/bootstrap or actual pinned-Knots daemon claim follows from this source review.

## Corrected Freeze: Findings Resolved | 2026-10-17T08:38:07Z

**Final scoped verdict: CLEAN before the two affected current pin renewals.** The preceding findings and first-packet disposition remain retained history; both are now resolved by the bounded fixes below. No additional actionable issue was found in the corrected frozen delta. This closes source review, not the pending full matrices or phase acceptance.

WR-27-01 is fixed by checking `supportedOwnership(source, start, Boolean(contract.maybeTest))` at the original selected impl offset before scanning its balanced range. For these explicitly production impl contracts, false means the existing helper rejects both cfg(any()) and cfg(test). The unique generic impl/method selector and separate method ownership/consuming checks remain. Both new impl mutations target the real selected reorg impl and require the exact stage.rs:absorb boundary.

WR-27-02 is fixed without deleting any old anchor: optional maybeClauses supplements the original eight individual WorkAuthority::check anchors with the three complete ordered owner/preview/stale if clauses, including operators, braces and returns. The normal semantic core requires both lists. The existing generic contract mutation loop now includes every clause as well as every original anchor; five dedicated negation/operator controls target actual check source. Their semantic-only fixture path first requires a passing baseline, invokes the unchanged edit() before/after inequality, then requires the exact selected file/symbol diagnostic. An unrelated historical/fingerprint failure cannot satisfy these tests.

The executor's retained review-delta log reports actual5pass/4fail RED before the fixes, followed by9pass/0fail/27expectations GREEN in2.27s. The reviewer read those logs but did not execute them. The prior87-case receipt remains valid only for its earlier scope; it is not relabeled as a full rerun after these additions. All later full source suites remain required.

The copied-root CLI initially failed0/1 because the new ordinary158 import could not load the existing159 helper. The approved fix appends exactly `scripts/check-phase159-filter-rpcs/evidence.ts` and `scripts/check-phase159-filter-rpcs/contracts.ts` after the original first two GUARDS. Both are already in the independently inventoried14-module current165 CLI import closure; neither is in its241 fixed-input list. Adding them as fixed158 loader dependencies therefore does not enlarge or repin241. Existing first-two guard evidence paths, the complete35-entry historical iteration and its removal controls remain unchanged. The executor reports the corrected isolated CLI diagnostic test1/1,4expectations,227ms; this confirms that narrow copied-layout check rather than a full positive matrix.

### Closing Source Identities and Exact Recovery

Fresh complete-file hashes match every row of the corrected packet. Exactly three source identities changed from the first packet; the other five listed above remain byte-identical.

| Corrected file                                                  | Lines | Closing complete-file SHA-256                                    |
| --------------------------------------------------------------- | ----- | ---------------------------------------------------------------- |
| scripts/check-phase158-validated-reorg.ts                       | 491   | 1b87c3bddf0d65db9eea63039780d5b6b03bd3a6db9a99d56fd61637d0391c6c |
| scripts/check-phase158-validated-reorg.test.ts                  | 628   | 418479c36f497919cf2ca289d93dcefaa8b009920dae467181cca528365c7f81 |
| scripts/check-phase164-canonical-identity/historical-tooling.ts | 213   | 47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65 |

The corrected eight-file ordered scope digest independently computes to `32dd3b731be4ad0326d353dcab889dca5fe5cb057b117e2984d2cae4ef472127`. All fit628; no cap exception, new module or helper edit was introduced.

Independent lightweight literal application again proves complete recovery: six checker158 blocks→385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd; seven test158 blocks→bebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3; two checker162 blocks→875460713003992a2701dc1f5ea300430eb2991b30e3635fefedb8bf387ce877; one test162 block→3ab730fcb1998459f410dfb3a60bb580d0fe67e8e47c8c76835080ad2062acac. Each unique ordered outer replacement reconstructs the authentic pre27 entire source byte-for-byte. Unchanged original26 eight/fourteen blocks then recover the same CAF914/784e complete destinations recorded above. The original26 array literal is still byte-identical. New outer literal callbacks and whole-byte refusal logic remain intact; no Rust projection or hash-only current semantic permission was added.

Only the two current historical-tooling digest literals in164 entry and165 contracts may now renew from db3761d7d50c25612c65dbf88abcb509294afb2a49c482800652d69e750ae810 to the reviewed47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65. That future delta still needs exact root verification. No other source/data/input/history pin change is approved by this review. Full158/162/163-native/164/165 runs and complete pre/post input/module closure, resumed26 and final phase/native/security/formal/Git gates remain separate and pending. Only this report was changed by the reviewer.

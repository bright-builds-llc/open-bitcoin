---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
status: current-byte-review-accepted
independent_acceptance: final-source-review-accepted
accepted_source_digest: 34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8
current_candidate_source_digest: 34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8
requirements-completed: []
---

# Phase 163 Scoped Checker Rebind

The earlier replacement checker bytes earned [scoped checker review](163-CHECKER-REVIEW.md) at nine-file manifest digest `e6bc2761a2d829a053dbaf379229bee8efb5c4f3290eb522ca6434e591cd98ab`, with seven initial warnings resolved and zero findings. The subsequent source-binding extension below requires fresh independent acceptance; the earlier review is not relabeled as covering current bytes. Neither scoped acceptance establishes whole-phase review, CFGR-01 completion, archive readiness or shipment. All Git finalization remains deferred to the root wrapper's normal clean gate; no hook is bypassed.

## Original and previously accepted replacement bytes

| Path                                             | Original SHA256                                                    | Replacement candidate SHA256                                       |
| ------------------------------------------------ | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| `scripts/check-phase158-validated-reorg.ts`      | `b2339c02beebd971432bd53dfc81df5f563a41968b5181ff66112045748169e5` | `db64d076530cf2c6c4df0be0863bda87935179e8243ed32fef7d55ec9f6fce32` |
| `scripts/check-phase158-validated-reorg.test.ts` | `68e444d11ae5a3203c6a51c069a00a496db6dd99bbf191c724c2ca496f1855ec` | `b33afce9a639aa86e04a4b04d6388341c5a7541b943e7a13fff14b918a36fa31` |
| `scripts/check-phase163-provenance.ts`           | New pure transitive policy                                         | `71655a6484ecb57c95bd5913a4f09a30e016ba1ee6d040f4bfe0d71fec963d54` |

The new policy is a required source through `PHASE162_FILES` and is bound directly by `checkFrozenGuards`. Freezing only the importing wrapper would leave its decisions mutable. The helper-only mutation test changes no wrapper and requires a targeted frozen-helper refusal.

## Exact scoped change for review

Review `git diff -- scripts/check-phase158-validated-reorg.ts scripts/check-phase158-validated-reorg.test.ts scripts/check-phase159-filter-rpcs.ts scripts/check-phase160-peer-filter-serving.ts scripts/check-phase161-operator-index-evidence.ts scripts/check-phase162-integrated-filter-proof.ts scripts/check-phase162-integrated-filter-proof.test.ts` against the original parent `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`, plus the new provenance policy and its tests. The unchanged original blobs remain recoverable from that parent; no original certificate was rewritten.

- Phase 158 replaces the uniform current checkbox/Phase162-owner decision with fixed-path milestone selection. Its six-source hash checks, actual function/assertion contracts, formal-report dates/generators/lifecycle/digest gates, original exact historical sentence and excluded-subject scanning remain in place.
- The optional filesystem adapter loads only the finite provenance universe. Historical formal reports are loaded when the published proof claims historical completion, independently of CFGR-01's current Pending checkbox. Missing evidence remains missing.
- Phase 162 gains a compatible optional filesystem root and the direct helper byte binding. Source/function/ownership/baseline/oracle/native-order checks remain in place.
- Phase 158/162 fixture builders explicitly construct original Phase162 ownership and status, accepting existing GFM alignment. Existing mutation assertions and per-case timeouts are retained; new cases prove honest Pending, absent formal history, map hermeticity, unauthorized exact claims and helper-only tampering.
- Phase 159/160/161 changes only pass the existing text adapter as the new optional shared claim-view/scan argument. Their own contracts and decisions remain unchanged. Root explicitly approved these additional paths for compatibility.
- The pure policy parses the first frontmatter with Bun YAML plus duplicate-key refusal, compares parsed fixed active/archive controls, validates all 22 requirement IDs and the pinned baseline, and separates historical delivery, new-owner closure and milestone publication. Supplied maps never invoke filesystem lifecycle adapters.
- Exact gap, closure, readiness and archived-control sentences are exempted only in their authorized states. Historical reports retain independent excluded-claim scanning. No broad audit/archive paragraph exemption is added.
- Independent initial review probes prompted explicit refusal controls for quoted duplicate mapping keys, ambiguous milestone identities, invalid/pre-lifecycle timestamps, altered live gap-audit links and duplicate roadmap ownership. Plain generated keys are the supported YAML boundary; quoted scalar list values in immutable audit snapshots remain valid.
- Fixtures deliberately construct original, Pending, functional closure and final publication states. A lifecycle-neutral actual-live check is separate, so later legitimate activation cannot invalidate a hard-coded assumption that future reports never exist. Readiness can be earned after the renewed passing audit while the dated functional report still records its earlier pending-audit boundary; final phase completion remains a separate refresh. Archived current wording does not exempt sentences that assert controls remain unarchived.

## Preserved historical proof

The ordered six-source `path:sha256` digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`:

1. `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs`
1. `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs`
1. `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs`
1. `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs`
1. `packages/testdata/phase162/recipe.rs`
1. `scripts/phase162-filter-oracle.ts`

Original Phase162 CONTEXT, implementation proof, SUMMARYs and VERIFICATION/NATIVE-EVIDENCE/REVIEW/SECURITY certificates remain untouched. The byte-preserved original milestone and integration audits have SHA256 `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8` and `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709` respectively; their original links and frontmatter are preserved exactly.

## Preserved checkpoint controls

Phase158 retains 47 original contracts, 33 named behavior-test routes and five checkpoint callback contracts. Phase162 retains all 208 fixed function contracts, plus the five callback contracts. `CHECKPOINT_COUNTS` remains exactly:

```text
[[1,1,1,1],
 [1,1],
 [1,1,1,1,5,1,1,1,1,2,1],
 [1,1,1,1,1,1,1,1,1,1],
 [1,1,1,1,1,1,1,1,1,1,1]]
```

These bind ordinary `persist`/IfNeeded, `flush_with_mode`, validated lineage completion, the header-checkpoint-before-prune ordering and trusted-header snapshot persistence with deletion accounting. No callback anchor/count, runtime Rust source, no-snapshot rule, frozen-source mutation assertion or ownership boundary is removed.

## Evidence and acceptance handoff

Plan01 records actual RED/GREEN and final focused suite receipts in `163-01-SUMMARY.md`. Both actual native checker entrypoints pass legitimate Phase163/Pending with no future163 verification. Helper-only tampering and absent supplied-map keys refuse. These source checks supplement executable runtime evidence; they do not create a new Rust execution receipt.

The exact nine-file replacement manifest has earned scoped independent source acceptance. Plan04 must still independently review the complete then-current phase manifest, verify security findings and complete the default native contract. Any subsequent change to bound checker/helper bytes requires a fresh explicit binding and review record; the original six-source proof digest and original certificates must remain unchanged.

## Source-binding extension | 2026-10-12T22:26:07Z

Security preflight identified ten fixed implementation inputs absent from the closure digest. The single exported `PROVENANCE_DIGEST_FILES` now contains 20 paths: its original six code/test paths, the three Phase159–161 callers and their three tests, the native transition test and fixture module, Phase162 contracts, `scripts/verify.sh`, current163 scope/context and both immutable original audits. No secondary digest manifest was introduced. Mutable current publication documents, audits and reports remain excluded; closure reports continue to require every declared implementation input to be present and nonempty.

This extension changes only the pure helper's exported list, 30 parameterized missing/empty/tampered-input refusal controls, and the exact frozen helper hash literal in162. Wave2's caller/test/native/contracts/verifier bytes were not edited by this follow-up. At this extension's boundary the158 checker/test replacements remained `d079ef8ff44ca4e7728841e53a4ff3ea04d3b7266b240df21c5acd623eb3d2e8` / `759870b52a6e2dbe282592a60ab1dd7cd7f9efe35b9c54597481fb9c023b37b4`; all six runtime proof roots/original certificates remained unchanged.

| Input                                | Previous reviewed SHA256                                           | Extension SHA256                                                   |
| ------------------------------------ | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| Pure helper                          | `7a768155c4a73687677e3065ce7054408c4442db297a3226e1a0b83928b6211b` | `a9da6af99ea418bc7822333493e61b739dfccd857e6e57dfd90cf88957772930` |
| Pure policy tests                    | Prior54-case source retained in the earlier review                 | `6251e8692d2a21bf7da697ca022d13f3bd59cc8668831fed582962592de630fb` |
| 162 checker including helper literal | Earlier review snapshot retained                                   | `7583661c4c2ade312f6483fee5f34a2c03edf0ceb8208719235d7bf217168ba0` |

The resulting fixed20-input source digest is `c017d970b9f788eabecde36828534825939551105c1a214ba630e32aced1bd86`. Existing pre-extension transition receipts retain their original execution boundary; they are not relabeled with this new digest. Fresh extension tests and the native transition rerun are recorded in the Plan01 summary addendum. Independent rebind refresh is a root action pending this extension's exact final snapshot.

Fresh extension evidence:30 intended source-binding RED failures/74ms;84 pure GREEN cases/140ms;33 affected historical/map/frozen cases/3.31s; the fresh201-case pure/native route passed with1,285 assertions in133.07s, including all117 native transition roots removed. Both actual live checker entrypoints exit0 on honest Pending. No complete historical-matrix rerun or default full verifier is claimed for this narrow extension; the root's full/default verification remains required for the final phase tree.

## Optional current page claim coverage | 2026-10-12T22:37:46Z

Security identified a supplied-map/current-page coverage omission: the registered `docs/parity/v2-5-archive-safe-provenance.md` could contain unearned readiness, shipment or production-service claims without reaching either native checker's shared scan. The fixed exported `PHASE163_DOC` now enters `PROVENANCE_FILES` only; it is neither a closure-digest input nor a mandatory historical `CHECK_FILES` input. Present page text passes through the existing state-aware `checkPhase162ExcludedClaims` inside `checkPhase162ClaimState.checked()`. Absence before creation remains valid. The scanner creates no historical-quotation or code-fence exemption.

The excluded production subject additionally accepts repeated whitespace/hyphen spellings before service/readiness/use/safety. The earlier single-space production-service spelling already matched the old scanner; its security failure was due to the unscanned page. This small deliberate strengthening is not presented as the cause of the page-coverage fix. Every prior excluded subject, formal historical gate, callback anchor and count remains retained.

Original-state fixtures omit the current page, including physical filesystem overlays that previously could inherit it. Pending and completed fixtures deliberately choose their stage's current wording. Wave2's owner separately added18 actual CLI/filesystem/map page controls; this executor did not edit those owned files.

Core evidence: eight actual readiness/shipment/production/fenced-readiness cases were RED before coverage (1.85s); ten158 compatibility/page cases passed afterward (1.464s);85 pure cases passed (133ms);51 affected page/metadata/map/frozen/original cases passed (3.12s). Actual live158/162 exit0 at the optional-page-before-creation boundary, wall0.18s/0.83s. Combined final suites and whole/default verification remain root actions after both owners' final freeze.

Current candidate hashes are reflected in the table above. Additional bytes: pure tests `bf55e9172082916d32ea2c7a82484d3d0998e9a3268471c33ec46d3a8af7cd26`;162 tests `1ab9ef1fd9721a2981c636a016c5a2c1c66be502dd47940a5ac4bece1945ae5c`;162 checker/literals `4e5518056c87dceb2126b3cfbe8e320e530db85a8673cafb607ef60d4c16b401`. The coordinated20-input source digest at22:37:46Z was `1cd982e277ec31990469cd824d0b04b23a9bfaf479b69a02eb812aa3b7872040`. Root's later CONTEXT refresh at22:38:52Z changed that digest to `63ae29d96f3016f21cd0dc586fe912fd709bf0ca6cbe03e536ca7b626d2fd09a` with all six owned code hashes unchanged. Earlier source-binding and transition receipts preserve their own snapshots; final independent acceptance must compute the coordinated digest after root's context/control freeze.

## Selector and original-fixture repair | 2026-10-12T22:57:14Z

Final review exposed exactly two causes. Active v2.5 controls using parser-valid spacing/bare headings were silently ignored alongside a valid archive, allowing their conflicts to escape comparison. The active relevance patterns now use parser-coherent whitespace and complete milestone-token boundaries; accepted roadmap archive titles remain recognized. Two-line production repair preserves unrelatedv2.6 fallback and equivalent parsed overlap.

Separately, two original162 fixture positives inherited the live index's newly dated PHASE162_COMPLETE sentence inside rationale, which then reached the shared scan and the copied proof stage. Synthetic Pending/implementation/formal builders now normalize only the exact intended-state claim sentences across their controlled index/proof/current-document strings. No assertion, real index/proof/history, excluded scan or certificate was weakened or edited.

The preceding historical run remains a failure:3,211 pass/2 fail/10,744 assertions/337.48s in `/tmp/phase163-page-combined-historical.log`. The independent native owner captured both selector controls RED (0 pass/2 fail/5.42s) before the production fix. This executor reproduced the two selector refusals plus two original fixture failures (2 equivalent positives pass/4 fail/979ms), then refreshed the helper literal: seven focused equivalent/conflict/bootstrap/implementation/formal cases pass in1.106s. The complete pure suite passes89/0/185 assertions/149ms; actual158/162 entrypoints exit0, wall0.20s/0.87s. A transient post-fix run refused only stale helper bytes before the explicit literal refresh; it is not credited as GREEN.

The158 checker/test hashes remain exactly `db64d076530cf2c6c4df0be0863bda87935179e8243ed32fef7d55ec9f6fce32` / `b33afce9a639aa86e04a4b04d6388341c5a7541b943e7a13fff14b918a36fa31`. Current helper/pure-test/162-test/162-checker hashes are `71655a6484ecb57c95bd5913a4f09a30e016ba1ee6d040f4bfe0d71fec963d54`, `921bd1345ff5e534d673b866304f5460e39b7617c75bd953b77c372c661dd7ff`, `3339fe5353164ad741315a1f29e2cee48a7bcec2e46ca09a2ea32813e9de64a3`, and `74dbf700ac1027d505ed97c62a7a8ecdd50473f3d1a88936b2e3b72ae3909a55`. The cooperating owner's frozen native test hash is `c2eb4bf83eb6fee43696b6a9507b2a8242e7c48ad930766de073340ce3b2ff11`. Coordinated20-input digest: `f3e9a2475b512202d89fa2973b9fb690cfc68e1511d9452d95097b30bca5575a`.

Both owners' source is frozen. Affected native/historical reruns and final independent review/default verification remain root actions on this exact coordinated snapshot. Earlier acceptance and failed receipts are retained at their original boundaries.

## Final current-byte acceptance | 2026-10-12T23:02:44Z

The fresh [final source review](163-REVIEW.md) accepts the current rebind at the digest of 20 inputs, `f3e9a2475b512202d89fa2973b9fb690cfc68e1511d9452d95097b30bca5575a`, with 42 reviewed files and zero open findings. Independent 89 pure cases, two actual native conflict controls, three original Phase 162 positives and both live CLIs passed. Original contract/proof/certificate/audit preservation and current frozen literals were independently recomputed. The earlier scoped review and failed/pre-repair receipts above retain their original attribution.

This acceptance records source review only. The final current native matrix also passed 226/0 with 1,470 assertions in 168.90s and 137 owned roots removed; full historical/default native, formal closure, renewed audit and normal-hook Git finalization remain separate gates. This dated acceptance note is publication metadata, outside the implementation digest; it does not relabel the review's earlier document inventory or invent another review run.

## Active identity and source presence | 2026-10-13T00:04:38Z

One fixed-format classifier now serves selection and target parsing: relatedv2.5 validates; only explicit unique canonical later numeric versions route to a fixed archive; unknown, blank, malformed, duplicate, contradictory, earlier or unsafe identities refuse with bounded active-path diagnostics. Values are line-local. The byte-exact whole-document roadmap archive index remains supported only through its existing fixed archive. Equivalent overlap, actualv2.6 and absent-active positives remain intact.

Native-owner RED:15 failures/33.52s, distinguishing13 accepted invalid inputs and2 existing refusals lacking path/category. Pure RED:6 pass/26 fail/225ms. Intermediate273-case GREEN/188.57s belongs to `f8713880…`, before the discovered zero-byte adapter gap.

Four owned zero-byte map cases were RED/2.15s; the native owner's two real zero-byte filesystem/CLI/map controls were independently RED/5.49s. Documented `sourceText(map)` now carries typed optional `maybeHasSource` alongside the existing callable API. All five158–162 adapters use it, preserving `.has` through initial state and later claim scans without sentinels, content rewriting or disk fallback. The legacy plain-callback absence/empty limitation is documented.

Final focused GREEN:13 cases/24 assertions/2.83s;121 pure cases/249 assertions/199ms; all five actual158–162 CLIs exit0. Current frozen hashes are158 `1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de`,158 tests `28c5ed07dfb670d439c4859dd8f93ff7f7b2e1624d9b58ba2b9106319778965a`, helper `cc8d36584e7d0456eb2a6969fa0ce3ee633a79dc1e6a0d45ddfd1c4897e05ad1`, pure tests `3ba274c9124e071e1c1d384f3527d04285f84b4090cb831ce55051dee56103b5`,162 `912df3e04b070833778cfbf03aaaf7dfefe20f107c3497e81d2ed4f5521867a3`,162 tests `97e69067ea0a291917465d6379c6c4adaba4da4919c00611c27aac82a7fe119c`. Native-owner source is `b85d81ee18eba16cc0a5e058c1cf34c7682d67b677a2d0e58e8d84f8062f28f8`. Final20-input digest is `34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`.

Earlier acceptance/full receipts retain their exact old snapshots. Fresh combined275-case proof, independent source/security acceptance and root's next full verifier must cover this final presence-aware candidate.

## Renewed independent acceptance | 2026-10-13T00:09:51Z

The fresh source reviewer accepted the repaired 20-input digest `34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8` in [163-REVIEW.md](163-REVIEW.md). Independent checks covered strict identity classification, exact archive routing, presence-preserving adapters, both empty-file reproductions, valid absence/overlap/next-milestone cases and unchanged original contracts/proof roots. Review records 121 pure tests, 21 native cases, 11 compatibility controls, 24 additional path-refusal/positive checks and all five live CLI exits. It also inspected the owner's final 275-test/154-root receipt. Earlier acceptance remains dated history and does not certify this repair. Full default execution, formal closure and renewed audit remain separate gates. This receipt changes only unbound review metadata.

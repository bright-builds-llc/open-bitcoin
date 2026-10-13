---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T05:13:45Z"
status: passed
exit_code: 0
source_digest: e0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d
source_inventory_count: 21
run_id: 38c0607f-9a5a-4180-9aa8-10dd17163f26
started_at: "2026-10-13T04:24:45.370Z"
ended_at: "2026-10-13T05:08:05.389Z"
duration_ms: 2600019
inner_duration_ms: 2599356
verify_mode: full
requirements_state_at_execution: pending
---

# Phase 163 corrected-source native verification

Full6 completed the entire default contract on the frozen21-input corrected source with exit0. The literal invocation was:

```bash
bash scripts/verify.sh
```

The existing cooperative wrapper executed `bun run scripts/command-timings.ts run --key verify-full --source verify --verify-mode full -- bash scripts/verify.sh`. Pinned Bun1.3.9 (fresh executable path) and Rust1.94.1 ran on Darwin/arm64. Both pinned Bun copies have SHA256cec3f87f0e17c12a294c75216bc7a08088e13b0be46623f65d5802ac56276bb1. This is actual dirty-HEADd47f6e09b4621808974cc083fcc27ee9d2c1b4c7 native evidence, not a clean commit or saving-hook result.

## Complete receipt

- Actual run: `38c0607f-9a5a-4180-9aa8-10dd17163f26`,04:24:45.370Z–05:08:05.389Z on2026-10-13,2,600,019ms wrapper; inner43m19.356s/2,599,356ms;success/exit0/no signal/full/default.
- Timing receipt: `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-13T04-24-45.370Z-38c0607f-9a5a-4180-9aa8-10dd17163f26.json`.
- Whole log: `.local/open-bitcoin-dev/phase163/verify-full-6.log`,1,617,945bytes/16,345lines,SHA25652d1c1553adf16cc61281407950f7aefc71e38d6f34517e96d7f836858dd9770.
- All176ordered stage receipts are unique, full-mode, success/exit0, and within this actual run. Index: `.local/open-bitcoin-dev/phase163/full6-receipt-index.json`. No previous or partial run fills a missing stage.
- All21raw source hashes match the pre-run `.local/open-bitcoin-dev/phase163/archive-fixed-candidate-manifest.json` after completion. All20original paths remain, plus the explicitly bound pure test-fixture helper. The original f14/f60audit snapshots and historical six-root proof remain unchanged.

| Stage                                     | Actual UTC start–end      | Duration ms | Actual result                                                     |
| ----------------------------------------- | ------------------------- | ----------: | ----------------------------------------------------------------- |
| test-phase-163-milestone-provenance       | 04:39:42.890–04:39:43.106 |         216 | 124pass/0fail/260assertions,209ms inner                           |
| test-phase-163-native-archive-transitions | 04:39:43.110–04:43:34.458 |      231348 | 160pass/0fail/1534assertions,231.34s inner                        |
| cargo-fmt                                 | 04:46:26.084–04:46:27.649 |        1565 | exit0;all-workspace format check                                  |
| cargo-clippy                              | 04:46:27.684–04:46:28.214 |         530 | exit0;all-targets/all-features,-Dwarnings                         |
| cargo-build                               | 04:46:28.218–04:46:28.521 |         303 | exit0;all-targets/all-features                                    |
| cargo-test                                | 04:46:28.524–05:07:00.115 |     1231591 | 4354pass/0fail/6pre-existingignored;40workspace/doctest summaries |
| cargo-test-cli-rpc-client-binary          | 05:07:00.124–05:07:00.485 |         361 | 7pass/0fail                                                       |
| benchmark-list                            | 05:07:00.489–05:07:00.628 |         139 | exit0                                                             |
| benchmark-smoke                           | 05:07:00.632–05:07:06.933 |        6301 | exit0                                                             |
| check-benchmark-report                    | 05:07:06.938–05:07:06.978 |          40 | exit0                                                             |
| bazel-build                               | 05:07:06.982–05:07:23.783 |       16801 | exit0;six first-party targets                                     |
| check-bazel-build-provenance              | 05:07:23.789–05:07:37.322 |       13533 | exit0                                                             |
| cargo-llvm-cov-clean                      | 05:07:37.327–05:07:38.253 |         926 | exit0                                                             |
| cargo-llvm-cov-pure-core                  | 05:07:38.257–05:08:04.872 |       26615 | exit0;uncovered-lines refusal gate passed                         |

The native suite removed all160owned roots. Its copied corpus was147files/3,585,615bytes,digest51f8c503d2cfd6d6afc386651d9aa0eaa05a76162b8afd59d5bcf826234e04c9; live control/proof presence and byte witnesses remained unchanged. Archive/final/audit fixtures are explicitly synthetic data. Actual CLI/filesystem/map paths and nested archived/next-active constructors execute within this full run. The284combined policy/native tests made1,794assertions.

Bazel built `//:core //:node //:rpc //:cli //:test_harness //:bench`. Pure-core coverage rejected uncovered-line output; its temporary text report is removed by the verifier, so no retained percentage is asserted. All historical Bun routes, traceability, current-doc guards, dependency/length/panic checks, native stages and default Rust gates belong to this one execution.

## Execution state and historical boundaries

CFGR-01’s actual checkbox/traceability row remained Pending under163 throughout this run; current publications used exact GAP wording, with no current-source closure/readiness. Nine staged control/publication byte hashes still matched the working tree at completion; their post-run check is `full6-pending-controls.json`. Source21 stayed unchanged. Fresh review was finalized early in the run, before the Phase163fixture copy/witness stage; mutable reports are outside the source binding. The actual finite corpus above reflects the reports present when native fixtures executed.

Independent preflights identified two stale administrative residues (requirements coverage prose and individual04/05roadmap checkboxes); they did not override the actual Pending row or canonical in-progress statuses. Root reconciles those after this full receipt, before fresh functional activation. No whole-run source/control mutation is relabeled as frozen proof.

[Focused archive repair preflight](163-ARCHIVE-FIXTURE-PREFLIGHT.md) records the eight original archived RED cases, all15 corrected outputs, all three active/archive/next-active layouts and source delta. [Preserved344native report](../../reports/phase163-before-fixture-revalidation/163-NATIVE-EVIDENCE.md) retains the original full4 receipt and earlier attempts. Normal hook1cab830e1failed Phase157README claims; hook2 907b45a9 was interrupted for the wrong Bun PATH; hook3 48fd406d failed three historical fixtures (604/3). Full5 8db6a513 was interrupted after38,277ms when newly reproduced archive prerequisites changed source scope;7edwas not full-certified. Their actual local logs/receipts remain failures or interruptions. Independent review also preserves its old-path bounded timeout attempt and fresh-path5/0rerun; neither is substituted for full6.

This report earns native verification only. Fresh functional closure, renewed actual audit/readiness, final phase/counter reconciliation and normal-hook commit/push are later gates. Actual archival/tag/shipment belong to the separately authorized archival task after the clean source saving checkpoint. None is asserted by this native receipt.

## Frozen21-input manifest

| Input                                                                                                         | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase163-provenance.ts`                                                                        | `cf22edda320107ce0dfda9a8692e8c29aaa436ad6ca1f82c48f66b1d778c67d7` |
| `scripts/check-phase163-provenance.test.ts`                                                                   | `cdbffaf4922bdcd70a40d49f32552085519812a3c06b9ebda113054f3580cb5f` |
| `scripts/check-phase158-validated-reorg.ts`                                                                   | `1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de` |
| `scripts/check-phase158-validated-reorg.test.ts`                                                              | `6526c6d4c1853b4bbf21e462b8dc292eeb95a6c10e493e9d0cbbf449f43c01cb` |
| `scripts/check-phase162-integrated-filter-proof.ts`                                                           | `7ff294a455913af4db6c45431d12b67c029e48246fa55ad7bf1f9840fbafbe2d` |
| `scripts/check-phase162-integrated-filter-proof.test.ts`                                                      | `01e84e45886a44d020ffabfebf32d8c898affc3afe1e35613478d38bd870cbc5` |
| `scripts/check-phase159-filter-rpcs.ts`                                                                       | `563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32` |
| `scripts/check-phase159-filter-rpcs.test.ts`                                                                  | `8c3591fcd8f4d1e523d77fbd85e4c0d9d50a27e14925ed8876879d1c380451b3` |
| `scripts/check-phase160-peer-filter-serving.ts`                                                               | `41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05` |
| `scripts/check-phase160-peer-filter-serving.test.ts`                                                          | `38b3fd04eb9bbafb1ca2d3b75eb5880ec1f75f147f71836ccb1840680edfbee8` |
| `scripts/check-phase161-operator-index-evidence.ts`                                                           | `c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23` |
| `scripts/check-phase161-operator-index-evidence.test.ts`                                                      | `05a59352ca6c1641690c94ece985644fd5ae1f72d8cb32cfdf455bdaf85e61dc` |
| `scripts/check-phase163-provenance/native.test.ts`                                                            | `f3750b40ca00a3e2faa38df1e0af24536ba152bcd7141e686d67d1c1a0a5bae9` |
| `scripts/check-phase163-provenance/fixtures.ts`                                                               | `40a76924ac64135f08f3482c50f8aafd1b5d98fa6d4dfd7052b87e0b6a492615` |
| `scripts/check-phase163-provenance/fixture-controls.ts`                                                       | `0395ada68a3f154dacfeaea41dbd444bdf1864ee908931be5f4c3588ef761968` |
| `scripts/check-phase162-integrated-filter-proof/contracts.ts`                                                 | `b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00` |
| `scripts/verify.sh`                                                                                           | `cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf` |
| `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md`                    | `a766eda3b17389bdfd390775c04085ee756c402d00a918be3fd6505c287d7956` |
| `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md`                  | `26b0ac69e42cf3eeeef175ae49f698c3a373b933b933249e5752ad942d801274` |
| `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md`   | `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8` |
| `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md` | `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709` |

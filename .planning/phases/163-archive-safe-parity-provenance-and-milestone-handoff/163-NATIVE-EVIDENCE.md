---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T18:12:18Z"
updated_at: "2026-10-13T18:49:30Z"
status: passed
exit_code: 0
source_digest: 14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977
source_inventory_count: 21
run_id: dfbd420c-9a6f-405e-8d24-65c9d9385a08
started_at: "2026-10-13T17:26:26.639Z"
ended_at: "2026-10-13T18:10:46.426Z"
duration_ms: 2659787
inner_duration_ms: 2659044
verify_mode: full
requirements_state_at_execution: pending
historical_suite_layout_matrix: passed
---

# Phase 163 logical-control correction: actual native evidence

The entire default verifier passed against the frozen 21-input logical-control correction. Its literal invocation was:

```bash
bash scripts/verify.sh
```

The cooperative wrapper was `bun run scripts/command-timings.ts run --key verify-full --source verify --verify-mode full -- bash scripts/verify.sh`. The actual run used pinned Bun 1.3.9, Rust 1.94.1 and Darwin arm64 at dirty HEAD `2a7920757ca93a1debee9ba1bdcee51fd808210c`. At this report's original 18:12:18Z publication, full native verification had passed and the separate four-layout historical-suite matrix was pending. The later completed matrix is recorded below without changing this full-run receipt.

## Full/default receipt and source binding

- Run `dfbd420c-9a6f-405e-8d24-65c9d9385a08`: 2026-10-13 17:26:26.639–18:10:46.426Z; 2,659,787ms wrapper, 2,659,044ms inner; success, exit 0, no signal, full mode, default target.
- Parent JSON: `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-13T17-26-26.639Z-dfbd420c-9a6f-405e-8d24-65c9d9385a08.json`.
- Raw log: `.local/open-bitcoin-dev/phase163/logical-control-full-verify.log`, 1,618,995 bytes / 16,353 lines, SHA256 `e26d1f3216c03480a234cdf03fcf69c903eccdbadfbf1545be510bbf1e47b216`.
- Index: `.local/open-bitcoin-dev/phase163/logical-control-full-receipt-index.json`. All 176 ordered stage JSON records were reread and matched the index: unique keys, success/exit 0/no signal/full mode, with every interval inside this parent run. The raw log hash and size also match. No prior or partial run supplies a missing stage.
- All 21 hashes in `.local/open-bitcoin-dev/phase163/logical-control-candidate-manifest.json` still matched after execution. The original six source hashes in `docs/parity/v2-5-integrated-filter-proof.md` were independently recomputed and remain equal; the original f14/f60 audit snapshots retain their bound hashes.

| Stage                                     | Actual UTC start–end on 2026-10-13 | Duration ms | Actual result                                                                      |
| ----------------------------------------- | ---------------------------------- | ----------: | ---------------------------------------------------------------------------------- |
| test-phase-163-milestone-provenance       | 17:41:46.034–17:41:46.283          |         249 | 124 pass / 0 fail / 260 assertions; 242ms inner                                    |
| test-phase-163-native-archive-transitions | 17:41:46.287–17:45:40.326          |      234039 | 160 pass / 0 fail / 1534 assertions; 234.03s inner                                 |
| cargo-fmt                                 | 17:48:36.760–17:48:38.430          |        1670 | exit 0; workspace format check                                                     |
| cargo-clippy                              | 17:48:38.471–17:48:46.902          |        8431 | exit 0; all targets/features, warnings denied                                      |
| cargo-build                               | 17:48:46.908–17:48:58.378          |       11470 | exit 0; all targets/features                                                       |
| cargo-test                                | 17:48:58.383–18:09:45.943          |     1247560 | 4354 pass / 0 fail / 6 pre-existing ignored; 40 workspace/doctest summaries        |
| cargo-test-cli-rpc-client-binary          | 18:09:45.951–18:09:46.386          |         435 | 7 pass / 0 fail                                                                    |
| benchmark-list                            | 18:09:46.390–18:09:46.526          |         136 | exit 0                                                                             |
| benchmark-smoke                           | 18:09:46.530–18:09:51.623          |        5093 | exit 0                                                                             |
| check-benchmark-report                    | 18:09:51.631–18:09:51.659          |          28 | exit 0                                                                             |
| bazel-build                               | 18:09:51.664–18:10:05.784          |       14120 | exit 0; six first-party targets                                                    |
| check-bazel-build-provenance              | 18:10:05.789–18:10:19.094          |       13305 | exit 0                                                                             |
| cargo-llvm-cov-clean                      | 18:10:19.099–18:10:20.079          |         980 | exit 0                                                                             |
| cargo-llvm-cov-pure-core                  | 18:10:20.084–18:10:45.947          |       25863 | exit 0; 1674 pass / 0 fail across 16 summaries; uncovered-line refusal gate passed |

The native suite removed all 160 owned roots. Its actual copied corpus contained **146 finite files / 3,485,048 bytes**, digest `b08539b5ebd94298fc77b615e50df23dbcf82e3625fb9b488f79dc58ee64d6b1`; live control/proof hashes remained unchanged. These are this run's measurements. The pure/native policy suites made 1,794 assertions across 284 passing tests. Archive/audit/final fixtures are explicitly synthetic data; actual CLI/filesystem/map paths and nested archived/next-active constructors execute within the native suite.

Bazel's six targets are `//:core //:node //:rpc //:cli //:test_harness //:bench`. Pure-core coverage passed the existing uncovered-line gate; the verifier removes its temporary text report, so no retained coverage percentage is asserted. Existing runtime capability, BASIC-only scope, local resource limits, public-network defaults and production/funds-safety boundaries remain unchanged.

## Historical-suite matrix and preserved attempts

At the original 18:12:18Z observation, the separate four-world matrix was **pending**: it still needed all seven complete historical test files in equivalent-overlap, exact safety-index, archive-only and next-active layouts. Successful selected cases or individual files did not establish whole-matrix acceptance. The attempts below retain their original outcomes.

- Original real safety hook `672cd52d-2d0b-4a88-a9f7-1a60bda170bc` failed at the legacy active-requirements deletion assertion: 606 pass / 1 fail, exit 1, 303,760ms wrapper / 303,243ms inner. Its raw `.local/open-bitcoin-dev/phase163/archive25-package/safety-commit-hook.log` retains SHA256 `baa218a834d4fccaa3f9f229550de6d056e7dce0f5662c527112be987ff064a4`; no safety commit occurred.
- `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-full-four-layouts.json` preserves the first sequential attempt's interruption (exit 143): complete Phase158 610/0 and Phase159 637/0, then partial Phase160. It is not a passing matrix.
- `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts.json` preserves complete Phase158/159 successes in all four worlds and actual Phase160 default-five-second timeouts during filesystem contention, followed by diagnostic interruption. Process samples showed filesystem activity and CPU progress; source, guards and timeouts were unchanged.
- `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-recovery.json` preserves the subsequent focused serial probe's failure: 13,761ms wall time, 0 pass / 1 fail under the unchanged default five-second limit. Its probe is a latency diagnostic, not whole-suite acceptance. Serial quiet recovery after the full verifier was still pending at the original report observation; its later receipt follows.

Earlier e0 full/native/review/security/audit/final evidence remains dated history in [the preserved native report](../../reports/phase163-e0c-before-logical-control-revalidation/163-NATIVE-EVIDENCE.md) and companion reports. Their passing statuses do not certify the corrected source. This run's CFGR-01 checkbox and traceability row were Pending under Phase163; stale coverage prose does not override those controls. Functional/final closure, renewed milestone audit/readiness, a new normal-hook saving commit/push and actual archival/tag/shipment remain later gates.

## Completed historical-suite matrix | 2026-10-13T18:49:30Z

The completed composite `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-complete-four-layouts.json`, generated at **18:48:18.099Z**, has SHA256 `beb01a4bc4c08cc9571e5dd3be68a12862995f58c38b3719203d17deb45029da`. It records **28 unfiltered whole-file executions / 20,128 passing tests / zero failures**, plus **24 actual CLI/filesystem/map boundaries**. Every world passed all seven complete files: Phase158 610, Phase159 637, Phase160 742, Phase161 149, Phase162 2610, Phase163 pure 124 and Phase163 native 160; **5,032 tests per world**.

| Synthetic world    | Whole files | Pass / fail | Matched per-layout corpus manifest SHA256                          |
| ------------------ | ----------: | ----------- | ------------------------------------------------------------------ |
| equivalent-overlap |           7 | 5032 / 0    | `da2c43e2809e004da16da885171be30872c29aef3e07e446de4fbc60c0f9de1a` |
| safety-index       |           7 | 5032 / 0    | `c19d9c72edd324fade0c39318ba62f9705540aaf48abe4ee6e3d994e00deb099` |
| archive-only       |           7 | 5032 / 0    | `0ca9399827cdcd638dd68ca63129d8211bcd453242b22c2d7c2f5e36d6701de4` |
| next-active        |           7 | 5032 / 0    | `2227972acdcfd4fcf253c40beb16f8b757d70bd66f6bbb5e20e32d9510f719b8` |

This is a composite of actual successful whole-file rows: Phase158/159 from the parallel attempt and Phase160–163 from serial quiet recovery, whose recorded interval is **18:11:19.243–18:48:11.255Z**. It is not a single new execution. Each world's two copied-corpus manifests are byte-identical across those attempts; the four worlds intentionally have different layouts. All 28 raw log hashes, eight paired manifest hashes, 24 boundary results and 27 before/after witnesses (21 source inputs plus six finite control candidates) were checked. Witnesses stayed unchanged throughout the matrix; control release and later live publication work occur after that evidence boundary.

The same focused safety-index probe that failed in 13,761ms during other activity later passed **1/0 in 357ms** after the full verifier ended, with the default five-second test limit, source and guards unchanged. That diagnostic contrast is separate from the 28 whole-file passing rows and does not erase earlier timeouts or establish a universal latency guarantee.

All recorded driver-owned outer roots and recorded probe roots were removed. Interrupted older child tests may leave unrecorded inner fixtures; the receipt makes no global temporary-directory cleanup claim. These synthetic layout receipts establish historical-suite compatibility, not live functional/final closure, actual milestone audit/readiness, a new normal-hook Git save, or actual archival/tag/shipment acceptance.

## Frozen 21-input manifest

| Input                                                                                                         | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase163-provenance.ts`                                                                        | `cf22edda320107ce0dfda9a8692e8c29aaa436ad6ca1f82c48f66b1d778c67d7` |
| `scripts/check-phase163-provenance.test.ts`                                                                   | `cdbffaf4922bdcd70a40d49f32552085519812a3c06b9ebda113054f3580cb5f` |
| `scripts/check-phase158-validated-reorg.ts`                                                                   | `1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de` |
| `scripts/check-phase158-validated-reorg.test.ts`                                                              | `efa71956ef2e4916a94790f9f9855a418e8f6d4b84c953db126654e89fbd85dc` |
| `scripts/check-phase162-integrated-filter-proof.ts`                                                           | `61c1919cc35f388b8d462ed28d8042cf33d5e5752afcc5d43fea79817d46ad97` |
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

---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T18:48:18.103Z"
status: passed
verification_scope: whole-archive-layout-fixture-matrix
source_digest: 14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977
source_inventory_count: 21
whole_test_files: 28
actual_boundaries: 24
passed_tests: 20128
failed_tests: 0
actual_archival: not_asserted
full_native_status: independently_recorded
---

# Whole archive-layout fixture preflight

All seven unfiltered Bun test files pass in each of four explicit synthetic checkout layouts: equivalent overlap, the exact safety-index topology, archive-only, and next-active. The accepted composite contains **28 whole-file executions, 20128 passed tests, zero failures, 66956 assertions, and 24 passed raw map/filesystem/CLI boundaries**. Each world has 5,032 passed tests. The frozen source has 21 inputs and digest `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977`.

The results are assembled from actual successful whole-file executions in two attempts whose per-layout corpus manifests are byte-identical. The interrupted parallel attempt is still interrupted, the failed latency probe is still failed, and this composite is not represented as one newly executed parent command. Synthetic reports and controls remain test data. This report does not authorize requirement closure or readiness publication, establish actual archival, or replace the independent full/default native evidence, audits, final certificate, and saving hook.

## Failure and minimal repair

The real archive safety hook failed on source e0 during run `672cd52d-2d0b-4a88-a9f7-1a60bda170bc`, 2026-10-13T17:05:34.405Z–17:10:38.165Z: 606 passed, one failed, 1,080 assertions; 303,760 ms wrapper / 303,243 ms inner, exit 1. The failed test unlinked only active requirements and expected a physical-file read error even though valid versioned requirements remained. Production correctly resolved the logical requirements candidate group. The preserved hook log is `.local/open-bitcoin-dev/phase163/archive25-package/safety-commit-hook.log`, SHA256 `baa218a834d4fccaa3f9f229550de6d056e7dce0f5662c527112be987ff064a4` (210,180 bytes). No archive commit, retirement, tag, or push resulted from that failed attempt.

The repair separates mandatory physical source files from logical requirements availability, and tests equivalent archive fallback, absence of both candidates, and invalid present active controls. The Phase162 guard retains its exact source-hash pin with only the new expected Phase158 test hash refreshed. Production resolver, claim, and refusal behavior remains unchanged. Logical candidate availability and publication authorization remain separate: fully archived selection with unarchived claim prose may correctly refuse the claim without producing a missing-control error.

The previous e0 focused preflight is preserved byte-for-byte at [its historical snapshot](../../reports/phase163-e0c-before-logical-control-revalidation/163-ARCHIVE-FIXTURE-PREFLIGHT.md), SHA256 `5bbfed7105e947b34c36604ec0d0c2b16147206338f457b58fb587da78111e25`. Its selected positive cases are historical evidence and are not relabeled as the broader matrix. The failed safety candidate snapshot is `.local/open-bitcoin-dev/phase163/archive25-package/failed-safety-candidate/manifest.json`, SHA256 `05116e6c9da926814a13b5b1d65d7fd63f53b7423438dbb9ef05a1d180954429`.

## Layouts, finite corpora, and raw boundaries

The safety-index world retains active requirements, has active ROADMAP exactly `# Roadmap\n\nv2.5 controls: .planning/milestones/v2.5-ROADMAP.md\n`, removes the active audit, and has all three fixed archived controls. Equivalent overlap retains equivalent active requirement/roadmap copies. Archive-only removes active requirements and uses archived publication wording; next-active adds unrelated v2.6 requirements and roadmap controls. These are synthetic topology proofs, not byte copies of live acceptance certificates.

| Layout             | Files / bytes | Selected requirements / roadmap / audit                                                                                               | Publication  | Identical corpus manifest SHA256                                   |
| ------------------ | ------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------ | ------------------------------------------------------------------ |
| equivalent-overlap | 296 / 7206104 | `.planning/REQUIREMENTS.md` / `.planning/ROADMAP.md` / `.planning/milestones/v2.5-MILESTONE-AUDIT.md`                                 | audit_passed | `da2c43e2809e004da16da885171be30872c29aef3e07e446de4fbc60c0f9de1a` |
| safety-index       | 296 / 7175060 | `.planning/REQUIREMENTS.md` / `.planning/milestones/v2.5-ROADMAP.md` / `.planning/milestones/v2.5-MILESTONE-AUDIT.md`                 | audit_passed | `c19d9c72edd324fade0c39318ba62f9705540aaf48abe4ee6e3d994e00deb099` |
| archive-only       | 295 / 7162554 | `.planning/milestones/v2.5-REQUIREMENTS.md` / `.planning/milestones/v2.5-ROADMAP.md` / `.planning/milestones/v2.5-MILESTONE-AUDIT.md` | archived     | `0ca9399827cdcd638dd68ca63129d8211bcd453242b22c2d7c2f5e36d6701de4` |
| next-active        | 296 / 7162672 | `.planning/milestones/v2.5-REQUIREMENTS.md` / `.planning/milestones/v2.5-ROADMAP.md` / `.planning/milestones/v2.5-MILESTONE-AUDIT.md` | archived     | `2227972acdcfd4fcf253c40beb16f8b757d70bd66f6bbb5e20e32d9510f719b8` |

Each bounded corpus includes the seven real test files, the 21 actual relative script imports, fixed first-party evidence inventories, and finite literal evidence paths. It stays below 600 files. Actual import declarations are anchored so embedded program strings cannot manufacture imports. The six raw boundaries per world are Phase158 and Phase162 supplied-map, filesystem, and CLI entrypoints. All 24 original rows passed, and serial recovery repeated those same 24 boundaries successfully.

## Actual accepted whole-file execution intervals

Every row below ran `bun test ./<file>` with the fresh pinned Bun1.3.9 executable. Accepted whole files have no name filter, skipped cases, timeout override, or outer command timeout. Existing test limits remain unchanged. The first two files in each world come from the actual parallel parent; the remaining five come from the actual quiet serial parent.

### equivalent-overlap

| File                                                     | Actual start UTC         | Actual finish UTC        | Milliseconds | Pass / fail / assertions |
| -------------------------------------------------------- | ------------------------ | ------------------------ | ------------ | ------------------------ |
| `scripts/check-phase158-validated-reorg.test.ts`         | 2026-10-13T17:39:03.435Z | 2026-10-13T17:42:42.592Z | 219157       | 610 / 0 / 1083           |
| `scripts/check-phase159-filter-rpcs.test.ts`             | 2026-10-13T17:42:42.592Z | 2026-10-13T17:49:28.472Z | 405880       | 637 / 0 / 1999           |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | 2026-10-13T18:11:22.914Z | 2026-10-13T18:12:39.548Z | 76634        | 742 / 0 / 2008           |
| `scripts/check-phase161-operator-index-evidence.test.ts` | 2026-10-13T18:12:39.548Z | 2026-10-13T18:12:54.896Z | 15348        | 149 / 0 / 180            |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | 2026-10-13T18:12:54.897Z | 2026-10-13T18:16:43.663Z | 228766       | 2610 / 0 / 9675          |
| `scripts/check-phase163-provenance.test.ts`              | 2026-10-13T18:16:43.664Z | 2026-10-13T18:16:43.895Z | 231          | 124 / 0 / 260            |
| `scripts/check-phase163-provenance/native.test.ts`       | 2026-10-13T18:16:43.895Z | 2026-10-13T18:20:29.421Z | 225526       | 160 / 0 / 1534           |

| Raw log                                                                                                       | Bytes  | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------ |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-equivalent-overlap-1.log` | 66877  | `dd8dc14f296cce74e62ff2412e0a18a5da312a1eae871db29760760201b33c87` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-equivalent-overlap-2.log` | 71662  | `4464ec36e7565820dcc797289c0e12ebaf79d4a1249c061102487b32aaa18a7c` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-equivalent-overlap-3.log` | 73416  | `08a24fb8575bd4983bdffef9d9f10776ae37f2daa802cb83cc34139d143535a0` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-equivalent-overlap-4.log` | 15575  | `8d848a0c7a1df77b3c8fe690116adfe810256b453322a16de449e2cd62358869` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-equivalent-overlap-5.log` | 238106 | `6bfbbc3bacbad903fc9496e7374e1f593217d760bf72bc57a6892982504cd346` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-equivalent-overlap-6.log` | 10674  | `e3168f686c74354f5c5427124dfaab29bc9e2d02a3caac2f60b42c5e3917b0ce` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-equivalent-overlap-7.log` | 16787  | `085711b7b4e21339024d773f9639665a687252b5715b9ccfd6eb36ac2e797b66` |

### safety-index

| File                                                     | Actual start UTC         | Actual finish UTC        | Milliseconds | Pass / fail / assertions |
| -------------------------------------------------------- | ------------------------ | ------------------------ | ------------ | ------------------------ |
| `scripts/check-phase158-validated-reorg.test.ts`         | 2026-10-13T17:39:05.775Z | 2026-10-13T17:42:38.908Z | 213133       | 610 / 0 / 1083           |
| `scripts/check-phase159-filter-rpcs.test.ts`             | 2026-10-13T17:42:38.908Z | 2026-10-13T17:49:20.454Z | 401546       | 637 / 0 / 1999           |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | 2026-10-13T18:20:31.692Z | 2026-10-13T18:21:41.589Z | 69897        | 742 / 0 / 2008           |
| `scripts/check-phase161-operator-index-evidence.test.ts` | 2026-10-13T18:21:41.590Z | 2026-10-13T18:21:56.390Z | 14800        | 149 / 0 / 180            |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | 2026-10-13T18:21:56.390Z | 2026-10-13T18:25:43.669Z | 227279       | 2610 / 0 / 9675          |
| `scripts/check-phase163-provenance.test.ts`              | 2026-10-13T18:25:43.670Z | 2026-10-13T18:25:43.932Z | 262          | 124 / 0 / 260            |
| `scripts/check-phase163-provenance/native.test.ts`       | 2026-10-13T18:25:43.932Z | 2026-10-13T18:29:39.798Z | 235866       | 160 / 0 / 1534           |

| Raw log                                                                                                 | Bytes  | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------ |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-safety-index-1.log` | 66873  | `f8b2dbb479c06e9b14c05776a44838c7e51331d8c89759de94e56e7d7bfeb89f` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-safety-index-2.log` | 71661  | `5eeacf5f0bcbe1b30a2c68f249ba5eabc7ba608a1d2344b006de3bea493e90b7` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-safety-index-3.log` | 73151  | `edb4780ca86ce5bf5611f0787d612d63dcc2a96c0445757dc0699bfb6c5ffa74` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-safety-index-4.log` | 15573  | `e6e4fc7cb614a07cb21a9930690c20c57874b653eb2668f767d4edadd98c26f6` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-safety-index-5.log` | 238089 | `b86452a132f927ddb09340dbd664a05b55c58cb26da529fe6b6f096d400beb0d` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-safety-index-6.log` | 10693  | `df2ab894e79983568e265d8b0be531dde4de62cfdc258508e519c66ab542e7b1` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-safety-index-7.log` | 16787  | `b37173f1abee15bec6d9b7322745a5ebf270e4a1c75d323f1e74306035b85ccd` |

### archive-only

| File                                                     | Actual start UTC         | Actual finish UTC        | Milliseconds | Pass / fail / assertions |
| -------------------------------------------------------- | ------------------------ | ------------------------ | ------------ | ------------------------ |
| `scripts/check-phase158-validated-reorg.test.ts`         | 2026-10-13T17:39:08.014Z | 2026-10-13T17:42:37.536Z | 209522       | 610 / 0 / 1083           |
| `scripts/check-phase159-filter-rpcs.test.ts`             | 2026-10-13T17:42:37.537Z | 2026-10-13T17:49:17.070Z | 399533       | 637 / 0 / 1999           |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | 2026-10-13T18:29:42.126Z | 2026-10-13T18:30:54.777Z | 72651        | 742 / 0 / 2008           |
| `scripts/check-phase161-operator-index-evidence.test.ts` | 2026-10-13T18:30:54.778Z | 2026-10-13T18:31:09.821Z | 15043        | 149 / 0 / 180            |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | 2026-10-13T18:31:09.822Z | 2026-10-13T18:35:14.347Z | 244525       | 2610 / 0 / 9675          |
| `scripts/check-phase163-provenance.test.ts`              | 2026-10-13T18:35:14.348Z | 2026-10-13T18:35:14.708Z | 360          | 124 / 0 / 260            |
| `scripts/check-phase163-provenance/native.test.ts`       | 2026-10-13T18:35:14.708Z | 2026-10-13T18:39:04.359Z | 229651       | 160 / 0 / 1534           |

| Raw log                                                                                                 | Bytes  | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------ |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-archive-only-1.log` | 66873  | `bc97f12ba7bcccdf0c8ed3fc75a7a16625ea523841ebc48eaac260f021805a7e` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-archive-only-2.log` | 71651  | `516d79ceedb43a3b6c22a302288666576bf55eeac92789ab92977be8b9c86eea` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-archive-only-3.log` | 73248  | `c1915ccff307bbec40c4b8adb4952780e94452908ae05cbc1482c3844f567a25` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-archive-only-4.log` | 15574  | `fb96137b940d843722ddc8756248d521ee0eaa033bd2ee58caced005f568271b` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-archive-only-5.log` | 238261 | `7a1e69ce31f67c8d5c33caeb2212c565e3d7cba4c8c3c4da987d92269f130cdc` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-archive-only-6.log` | 10692  | `64c7c0d4ed8be3c7d932b52255e94097ba2fec66b0551ff47d08ea4c680a727f` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-archive-only-7.log` | 16787  | `17a5c84be832cd7f53c4592c818318e490bf5e1a3bde99fe85afd26df8227498` |

### next-active

| File                                                     | Actual start UTC         | Actual finish UTC        | Milliseconds | Pass / fail / assertions |
| -------------------------------------------------------- | ------------------------ | ------------------------ | ------------ | ------------------------ |
| `scripts/check-phase158-validated-reorg.test.ts`         | 2026-10-13T17:39:10.354Z | 2026-10-13T17:42:39.899Z | 209545       | 610 / 0 / 1083           |
| `scripts/check-phase159-filter-rpcs.test.ts`             | 2026-10-13T17:42:39.900Z | 2026-10-13T17:49:24.269Z | 404369       | 637 / 0 / 1999           |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | 2026-10-13T18:39:06.542Z | 2026-10-13T18:40:16.377Z | 69835        | 742 / 0 / 2008           |
| `scripts/check-phase161-operator-index-evidence.test.ts` | 2026-10-13T18:40:16.377Z | 2026-10-13T18:40:31.151Z | 14774        | 149 / 0 / 180            |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | 2026-10-13T18:40:31.151Z | 2026-10-13T18:44:18.194Z | 227043       | 2610 / 0 / 9675          |
| `scripts/check-phase163-provenance.test.ts`              | 2026-10-13T18:44:18.196Z | 2026-10-13T18:44:18.452Z | 256          | 124 / 0 / 260            |
| `scripts/check-phase163-provenance/native.test.ts`       | 2026-10-13T18:44:18.453Z | 2026-10-13T18:48:11.220Z | 232767       | 160 / 0 / 1534           |

| Raw log                                                                                                | Bytes  | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------ | ------ | ------------------------------------------------------------------ |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-next-active-1.log` | 66875  | `35751d0ce29bb0350767ca2b4e4bee0d250e805c069542b01e3fdc4b839654db` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-next-active-2.log` | 71660  | `cbfdbbe3df57be93138eab56be41cb6310aeb2a78133bc1d304b2ec69c6d4f34` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-next-active-3.log` | 73161  | `bb349d3bc9fd113944071bbd134ead8a2e5dbc52a8b9078fc33dfa5662c0ff4d` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-next-active-4.log` | 15573  | `358c7521c80d81665d22e1b7fbd3315d0b7a7f15c385a3de7eec8065bd1fc71a` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-next-active-5.log` | 238072 | `7a4c10b9a286ce282ac013b5a55392775fe6404900c5258c32d2eda4940d28ad` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-next-active-6.log` | 10683  | `2193a7332fa0b146930dc3571fb913393e3462c6310100f90e2677104106acc9` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery-next-active-7.log` | 16787  | `02418f129d9f021024c2dfc75076b573d07f75ede834263732193ccebfd93883` |

## Actual attempts and resource-boundary diagnostics

| Attempt receipt                                                                           | Actual status | Recorded start UTC       | Recorded completion UTC          | Bytes / SHA256                                                             |
| ----------------------------------------------------------------------------------------- | ------------- | ------------------------ | -------------------------------- | -------------------------------------------------------------------------- |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-full-four-layouts.json`     | interrupted   | 2026-10-13T17:26:19.885Z | 2026-10-13T17:38:15.959040+00:00 | 15810 / `2b9d43b7dd5a699eeecb56e02cec93642dc5fae7141e350a8ce17507791cb5b8` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts.json` | interrupted   | 2026-10-13T17:38:59.288Z | 2026-10-13T17:53:48.813Z         | 29513 / `415f6a80729c5a4b2abb34bbab9446d1113d23a221d27c29dd52748f478c427b` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-recovery.json`       | failed        | 2026-10-13T17:55:37.126Z | 2026-10-13T17:56:17.446Z         | 12689 / `bd6b10283167b4b24b4328ef2d70e4bb9aed8338cafd2baf8adfa4504d6db129` |
| `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-serial-quiet-recovery.json` | passed        | 2026-10-13T18:11:19.243Z | 2026-10-13T18:48:11.255Z         | 40957 / `a2f65a88c4957b4516a3e5aa65cbedb643d32d697fa89bb2cee1038d478a737f` |

The initial sequential diagnostic was explicitly interrupted for a scheduling replan after successful whole158/159 executions. The four-world parallel attempt subsequently exposed default five-second Bun timeouts in whole160 mutation tests. Each raw failure and SIGTERM interruption remains in its original receipt and logs. The host had spare CPU and RAM; an actual one-second sample of the matrix-owned160 process showed 765/766 main-thread samples in filesystem open/close/write/truncate/mkdir calls. Sample SHA256 is `444fa93000dce422557629c44a8be8e7912d75945bb431531738f31987301819`, at `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-parallel-four-layouts-160-sample.txt`.

A controlled single safety-index160 case still failed under unchanged limits during the independent root Rust verifier: 0 passed / 1 failed / 3 assertions, 13,761 ms, 2026-10-13T17:55:59.401Z–17:56:13.162Z. Its log SHA256 is `f4f0cb811aab7c58c5da8dabe70f3eb401f610d58bafc48d7d87605d7cc87d7a`. After that root verifier completed, the exact controlled case passed 1/0/3 in 357 ms, 2026-10-13T18:11:19.375Z–18:11:19.732Z, SHA256 `7e0dfb6c04092bb542fe596c5149dfde740d535318a37deda1cbfb51dd9b4307`. All remaining whole files then ran serially. These observations support a host filesystem load condition; they do not turn failed attempts into passes or weaken limits.

## Composite validation, witness, cleanup, and limits

The final composite is `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-complete-four-layouts.json`, SHA256 `beb01a4bc4c08cc9571e5dd3be68a12862995f58c38b3719203d17deb45029da`. It independently validates every parent receipt and raw log hash, actual unfiltered command, pass/fail totals, each byte-identical per-layout manifest, all source/control witnesses, and all recorded outer-root cleanup results. It includes actual parent statuses, interrupted rows and timeout markers, the busy and quiet focused probes, and per-file receipt provenance. Generated at 2026-10-13T18:48:18.099Z; this is a composite generation time, not a new execution interval.

All 21 bound source hashes and presence/absence plus raw bytes for all six active/archive control candidates were identical before and after every attempt and at composite publication. The quiet attempt scopes child TMPDIR inside driver-owned roots. Every recorded driver-owned outer root and probe root was removed. Interrupted older child tests may leave unrecorded inner fixtures; no global temporary-root cleanup claim is made, and no unproven other-task directory was removed. The live repository controls were not archived by these fixtures.

The independent [NATIVE-EVIDENCE report](163-NATIVE-EVIDENCE.md), [logical-control debug record](../../debug/phase163-logical-control-mutations.md), fresh review/security results, requirement/audit activation, final phase verification, and normal-hook commit/push remain distinct evidence boundaries. This report proves the complete legacy suites survive the specified archive layouts under the frozen source; it establishes no new runtime capability or production/funds safety.

## Frozen raw source manifest

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

---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T01:15:44Z"
status: passed
exit_code: 0
source_digest: 34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8
source_inventory_count: 20
run_id: a78124d3-c707-4813-ac47-74817889fa75
started_at: "2026-10-13T00:30:00.586Z"
ended_at: "2026-10-13T01:12:17.915Z"
duration_ms: 2537329
verify_mode: full
requirements_state: pending
---

# Phase 163 Native Verification Evidence

Attempt 4 completed the entire default native contract on the repaired source snapshot, with exit 0. The literal verifier invocation was:

```bash
bash scripts/verify.sh
```

Its existing cooperative timing wrapper ran `bun run scripts/command-timings.ts run --key verify-full --source verify --verify-mode full -- bash scripts/verify.sh`. Pinned Bun 1.3.9 and Rust 1.94.1 were used on darwin/arm64. The receipt records dirty HEAD `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`; this is verification of the working source snapshot, not a clean commit or a normal commit-hook receipt.

## Actual execution receipt

- Timing receipt: `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-13T00-30-00.586Z-a78124d3-c707-4813-ac47-74817889fa75.json`.
- Actual start/end: 2026-10-13T00:30:00.586Z–2026-10-13T01:12:17.915Z; wrapper duration 2,537,329ms; `outcome: success`, `exitStatus: 0`, `signal: null`.
- Complete log: `.local/open-bitcoin-dev/phase163/verify-full-4.log`, 1,617,351 bytes / 16,337 lines, SHA256 `239f7d2ded4fca466c5787dc18d867f67da524b5d8bd47ff29ce8a80f5888b57`.
- Log bounds: line 1 confirms configured Git hooks; line 2 confirms current tracked LOC; line 16,337 records `verify.sh completed in 42m 16.626s (2536626ms)`. The verifier's inner duration differs from wrapper duration by startup/final receipt overhead.
- All 176 ordered `run_step` routes have exactly one successful full-mode timing receipt within this run's bounds. Step receipts live under `.local/open-bitcoin-dev/command-timings/verify-step-<label>/`, identified by their actual start timestamp and run UUID. No partial tail or previous run supplies a missing stage.

| Stage                                            | Actual UTC start–end on 2026-10-13 | Duration ms | Result                                                    |
| ------------------------------------------------ | ---------------------------------- | ----------: | --------------------------------------------------------- |
| Phase 163 pure provenance                        | 00:44:55.426–00:44:55.631          |         205 | 121 pass / 0 fail / 249 assertions                        |
| Phase 163 actual native transitions              | 00:44:55.635–00:48:04.385          |      188750 | 154 pass / 0 fail / 1512 assertions                       |
| Cargo fmt --all --check                          | 00:50:20.810–00:50:22.213          |        1403 | exit 0                                                    |
| Shared Phase 162 rustfmt                         | 00:50:22.217–00:50:22.243          |          26 | exit 0                                                    |
| Strict workspace/all-targets/all-features Clippy | 00:50:22.246–00:50:22.760          |         514 | exit 0; -D warnings                                       |
| Workspace/all-targets/all-features build         | 00:50:22.764–00:50:23.041          |         277 | exit 0                                                    |
| Workspace/all-features tests and doctests        | 00:50:23.045–01:11:15.782          |     1252737 | 4354 pass / 0 fail / 6 ignored across 40 result summaries |
| Explicit CLI RPC client binary tests             | 01:11:15.787–01:11:16.180          |         393 | 7 pass / 0 fail                                           |
| Benchmark list                                   | 01:11:16.184–01:11:16.321          |         137 | exit 0                                                    |
| Benchmark smoke                                  | 01:11:16.325–01:11:21.473          |        5148 | exit 0; JSON/Markdown reports written                     |
| Benchmark report validation                      | 01:11:21.479–01:11:21.507          |          28 | exit 0                                                    |
| Six-target Bazel build                           | 01:11:21.512–01:11:36.257          |       14745 | exit 0; 85 actions                                        |
| Bazel build provenance                           | 01:11:36.264–01:11:49.998          |       13734 | exit 0                                                    |
| llvm-cov clean                                   | 01:11:50.003–01:11:50.787          |         784 | exit 0                                                    |
| Pure-core llvm-cov                               | 01:11:50.791–01:12:17.464          |       26673 | exit 0; uncovered-lines refusal gate passed               |

The Bazel targets were `//:core //:node //:rpc //:cli //:test_harness //:bench`. Benchmark report validation is logged at line 12,815 and Bazel provenance success at line 14,555. Coverage uses the declared pure-core package inventory and rejects `Uncovered Lines:`; its temporary text report is removed by the verifier, so no retained percentage report is asserted.

The actual native suite removed all 154 owned roots. Its finite copied corpus was 143 files / 3,520,380 bytes, digest `371f15b4adc54f6f3245df8b3739873744be2888f3eeba0373e0ddeef486a05f`, with unchanged live control/proof witnesses (log line 6,958). Synthetic completed/archive fixtures remain test data. All historical Bun routes, the new routes, active traceability, source/claim guards, dependency/length/panic checks, and default Rust gates executed in this single run.

The prior timed format prerequisite already passed: `phase163-format/2026-10-12T23-57-25.532Z-14b285b6-50c7-409c-a2f9-f03aefa0f9f0.json`, 23:57:25.532Z–23:57:27.161Z, 1,629ms, exit 0. Subsequent repairs changed TypeScript only; this receipt-monitoring task changed no Rust source and launched no duplicate format/build/test run.

## Exact source binding and preserved history

The root supplied the launch manifest at `.local/open-bitcoin-dev/phase163/final-candidate-manifest.json`. This resume independently rehashed every input at 01:08:25Z while the run was live and again at 01:13:06Z after completion: all 20 raw-byte hashes match the launch manifest and the same source digest. Inputs total 679,426 bytes. The digest is SHA256 of the following ordered UTF-8 `path:sha256` rows joined by LF, without a trailing LF, in exported `PROVENANCE_DIGEST_FILES` order:

```text
scripts/check-phase163-provenance.ts:cc8d36584e7d0456eb2a6969fa0ce3ee633a79dc1e6a0d45ddfd1c4897e05ad1
scripts/check-phase163-provenance.test.ts:3ba274c9124e071e1c1d384f3527d04285f84b4090cb831ce55051dee56103b5
scripts/check-phase158-validated-reorg.ts:1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de
scripts/check-phase158-validated-reorg.test.ts:28c5ed07dfb670d439c4859dd8f93ff7f7b2e1624d9b58ba2b9106319778965a
scripts/check-phase162-integrated-filter-proof.ts:912df3e04b070833778cfbf03aaaf7dfefe20f107c3497e81d2ed4f5521867a3
scripts/check-phase162-integrated-filter-proof.test.ts:97e69067ea0a291917465d6379c6c4adaba4da4919c00611c27aac82a7fe119c
scripts/check-phase159-filter-rpcs.ts:563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32
scripts/check-phase159-filter-rpcs.test.ts:9bfdc8c7a7b927296d57cb03f394412630d042b6a06fd03e943e4b21ce9958f9
scripts/check-phase160-peer-filter-serving.ts:41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05
scripts/check-phase160-peer-filter-serving.test.ts:d44b875591976e6e7e959c48c6348ea5c5f291ac2b249d34bf295a60e0d97a65
scripts/check-phase161-operator-index-evidence.ts:c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23
scripts/check-phase161-operator-index-evidence.test.ts:33449ab72da6b5630349e3b5f47402322feb6576c866c9569069b3ed1a9d3177
scripts/check-phase163-provenance/native.test.ts:b85d81ee18eba16cc0a5e058c1cf34c7682d67b677a2d0e58e8d84f8062f28f8
scripts/check-phase163-provenance/fixtures.ts:588c41ff2719aec69a53b0799c7596945b697bff7f45162532eca2562c7dea5a
scripts/check-phase162-integrated-filter-proof/contracts.ts:b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00
scripts/verify.sh:cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md:a766eda3b17389bdfd390775c04085ee756c402d00a918be3fd6505c287d7956
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md:26b0ac69e42cf3eeeef175ae49f698c3a373b933b933249e5752ad942d801274
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md:f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md:f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709
```

The original ordered six-root Phase 162 digest was independently recomputed as unchanged `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`. Both immutable original audit hashes above match. Original certificates remain historical evidence; their byte-preservation assessment is recorded by the independent [review](163-REVIEW.md) and [security audit](163-SECURITY.md).

## Prior attempts and scope limits

| Attempt | Actual run ID                        | Start–end UTC                          | Duration ms | Disposition                                                                  |
| ------- | ------------------------------------ | -------------------------------------- | ----------: | ---------------------------------------------------------------------------- |
| 1       | 8e8a77fd-6120-47a9-a1f2-2073547230b0 | 2026-10-12T23:05:06.114Z–23:05:06.702Z |         588 | exit 1; stale tracked LOC                                                    |
| 2       | 9d4f0355-7d48-4ab4-826f-4a7fe233c817 | 2026-10-12T23:05:52.342Z–23:48:14.854Z |     2542512 | exit 0 on superseded f3e9a247…5575a source; does not certify current repairs |
| 3       | 04a8e9d2-4ab1-42d8-bcfc-f0a03a0e6ff7 | 2026-10-13T00:12:59.270Z–00:27:33.799Z |      874529 | exit 1; selector test exceeded its existing 5000ms bound                     |

Their actual logs remain `verify-full-1.log`, `verify-full-2.log` and `verify-full-3.log` in the same local phase directory; timing receipts retain each start/run ID. Attempt 3 recorded 2609 pass / 1 fail / 9675 assertions in the Phase 162 checker suite. Its failure was `phase162_inbound_write_expired_deadline` selector discovery (6370.86ms); it is not relabeled as success. Attempt 4 is a new entire execution on current bytes.

CFGR-01 remained unchecked/Pending through this native execution, with no canonical Phase 163 VERIFICATION required or present. After completion, REQUIREMENTS and the six current publication inputs still match the pre-run Pending snapshot. The root's concurrent planning-only five-plan revision changed PLAN/STATE/ROADMAP/config metadata outside the 20-input binding; CONTEXT and GAP-SCOPE stayed unchanged. Native execution time, source snapshot, fixture corpus and later planning/publication revisions are distinct records.

This report earns the native gate only. Formal functional activation, the dependent Plan 05 renewed audit/readiness/final reconciliation, normal-hook Git finalization and actual archive/tag/shipment are separate. No such future result is asserted here. The monitor modified only this receipt; scoped formatting and manifest/receipt consistency checks complete its document verification.

## Saving-hook compatibility follow-up | 2026-10-13T03:24:14Z

The first normal saving hook ran with index LOC452,452 and current-source full mode, but failed before a commit: `cab830e1-c041-4d59-b231-df008aa78c0b`, 2026-10-13T03:15:36.817Z–03:17:18.781Z,101,964ms wrapper/101,418ms inner,exit1. Actual log: `.local/open-bitcoin-dev/phase163/final-commit-hook.log`; timing receipt is in the verify-full directory under that start/run ID. Phase157's existing broad v2.5 capability-claim scanner rejected the root README's two exact readiness sentences, causing three positive-fixture failures (654pass/3fail), and the live157 command reproduced the same refusal. This failed whole-hook execution remains a failure.

The localized publication correction replaces only those two root README sentences with the already authorized `CLOSURE_CLAIM`. Adjacent prose still states the actual renewed audit and earned readiness publication; exact `READINESS_CLAIM` remains in the five other current publications. The original seven-sentence readiness publication is retained as dated history, not relabeled. No guard, test, implementation input, original proof or acceptance timestamp changed. The ordered20-input digest remains34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8.

After the correction, live157 and162 plus current-document reconciliation exit0. The three exact previously failing Phase157 tests pass (3/0,8assertions,561ms) in `.local/open-bitcoin-dev/phase163/commit-readme-compatibility.log`. Existing tests remain the regression guard; no broad claim exemption or timeout change was added. Independent doc-delta review and a new entire normal saving hook are required before Git success; no retry success or push is claimed here.

Independent README-delta acceptance at2026-10-13T03:26:00Z is appended in [review](163-REVIEW.md): exactly two authorized sentence replacements, all other README bytes/historical anchors preserved, five precise readiness carriers retained, unchanged34443772 source, and independent live157/162/document checks pass. Root freezes the candidate at 2026-10-13T03:28:03Z for the complete second normal hook; its outcome is derived from actual Git/local receipts, not this preparation note.

## Subsequent user-requested audit | 2026-10-13T03:35:42Z

A separate explicit audit invocation produced the current final22/22,10/10,31/31,18/18 audit at2026-10-13T03:23:55Z after phase completion. It retains the same34443772 source binding and does not relabel the earlier renewal that authorized readiness. Before replacing the earlier staged audit reports, the audit task preserved their exact raw bytes and this wrapper independently matched them against the saving index in [renewed milestone snapshot](../../reports/v2.5-MILESTONE-AUDIT-2026-10-13T03-18-48Z-before-final-reaudit.md), SHA2565456dad59f48379ee13f979cb857faa96aa32319f3a86c5b58cf116f39dd1689, and [renewed integration snapshot](../../reports/v2.5-INTEGRATION-AUDIT-2026-10-13T03-18-48Z-before-final-reaudit.md), SHA256ff909e9297b2d046dbef6a6ee0842a367a119a08ad5c922653a9bb3d6b54bff8. The milestone snapshot preserves its actual02:11:09Z renewal boundary plus the already dated final-certificate reconciliation. Both original gap snapshots remain unchanged. These extra preservation records are outside the20-input implementation binding and are not an actual milestone archival operation.

## Saving-hook runtime correction | 2026-10-13T03:45:00Z

The second normal-hook attempt is interrupted, not passed: run907b45a9-4dc4-4339-af7a-832177f74950,03:44:21.550Z–03:45:00.131Z,38,581ms wrapper,exit143/SIGTERM. The wrapper accidentally placed the Bun executable rather than its containing directory in PATH; the hook therefore resolved installed Bun1.4.2, confirmed by the actual test header. Root stopped the verifier subtree for this observed runtime mismatch, preserving final-commit-hook-2.log and its timing receipt. No commit was created. Source20 remains unchanged34443772. Recovery is to confirm PATH resolves pinned Bun1.3.9, retain frozen staged inputs, and execute a fresh entire normal hook in final-commit-hook-3.log before any push. No guard or verification mode changes.

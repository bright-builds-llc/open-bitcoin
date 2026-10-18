---
phase: 165-shared-exact-work-authority
plan: "40"
reviewed: "2026-10-18T05:33:16Z"
depth: deep
checkpoint: renewed-copier-SOURCE-before-inverse-derivation
files_reviewed: 3
files_reviewed_list:
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase163-provenance/fixtures.ts
  - scripts/check-phase164-canonical-identity/historical.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
---

# Plan 40 renewed copier source review

The current three-owner copier source checkpoint is CLEAN with zero findings. The targeted implementation/formal loader regressions, original strict missing-root case, five evidence-mode cases and all 33 existing closure controls passed on these exact source bytes. This report approves this bounded source checkpoint for Root review before renewed copier inverse derivation. It does not approve H, the final nine-owner code checkpoint, ordinary identities, full suites or native verification.

Repo-local AGENTS guidance, AGENTS.bright-builds.md, the placeholder-only standards overrides, and the architecture, code-shape, testing, verification and TypeScript standards informed this review. Both active lesson inputs were loaded under the bounded startup policy. Source-before-identity chronology, contained filesystem effects, strict original assertions and evidence of actual outcomes were material criteria.

## Exact current source scope

| Owner                                                        | Bytes | Physical lines | SHA256                                                             |
| ------------------------------------------------------------ | ----: | -------------: | ------------------------------------------------------------------ |
| scripts/check-phase158-validated-reorg.test.ts               | 39640 |            623 | `7d6ef29bdcdf43ecc4f43aaf2cd36e56d52efd68eb4851ba5284c65a6e074709` |
| scripts/check-phase163-provenance/fixtures.ts                | 22352 |            374 | `40fd6c50c3bf5d59d3f6e11291857ed99cff9ac23c7c6c2898bd4adc545fc172` |
| scripts/check-phase164-canonical-identity/historical.test.ts | 32588 |            627 | `a57424fc6f3cf8f628549060a03b83592b5654927ce20900d36e8e197221af94` |

All three current bodies match the inert proposal's complete after bodies. The proposal at `.local/open-bitcoin-dev/phase165-tools40-evidence-mode-source3-proposal.json` is 455656 bytes, SHA256 `acecdf9207805e2ba34def91d596a9921d56c23ed9860f26b86c1c80e9d29498`. It was authenticated from its raw buffer before parsing. Independent reconstruction using its exact replacement witnesses and appended test source reproduces all three after bodies without additional changes. The three before bodies matched the actual source before application; all 340 source rows in that inert generation were independently matched by canonical path, presence, bytes and SHA256.

The separate Root source delta is `113259bb38cb1cc173ccc7c0a260b422da5e466827a7eea00297bae83b9d5314`; the metadata delta is `faa210f362f9803de92114630a5bbb81b7a5f15ebcf046ba9ebb5f2c42953957`. Their authenticated application is `0bda478b599c84df3abdbd0739b96e7dbd1889a23677bdbf763b030ee473152d`. The prior closed generation remains `b6f2837d8320c7ab2028c0aca14accb983ab9d7b67e769d6118766032694f029` with its carried absent future outputs. These are source and evidence authorizations, not inverse or ordinary identity candidates.

## Behavior and preservation

The demonstrated regression was unintended runtime membership in explicitly supplied historical evidence maps. The optional loader correctly observes provenance files present on disk. Injecting H and fixture-source into those maps changed the original implementation/formal exact-key results.

The 158 test fixture now supplies its mode solely from `maybeSources === undefined`. Omitted or explicitly undefined input uses the executable default. Every explicitly supplied Map, including an empty Map, uses evidence-only. This is the only changed argument in the complete 158 body; all original 900 test definitions and loops, assertions, deletion mutations, cleanup and the strict missing-root child invocation remain unchanged.

In the existing 163 helper, the finite mode union also has an explicit runtime rejection for unknown values before graph discovery or root creation. Executable remains the default for existing callers, including direct empty-Map calls. It retains fixed-entry validation, actual bounded runtime discovery, caller-supplied module mutations, the cache boundary and separately supplied fixed JSON evidence requirements.

Evidence-only calls the common checked clone directly. It never calls runtimeSources, discovers modules or accesses the runtime graph cache. It preserves the supplied Map's keys, order, empty strings and mutated module strings, and never mutates the caller Map. Both branches apply the same 400-entry and canonical relative-path validation before mkdtemp. Both then use the unchanged caller registration before writes and the same guarded writer. Component and leaf symlink rejection deliberately occur during guarded writes after registration, so caller-created symlink and failed-setup roots remain discoverable by the caller's cleanup lifecycle. No root ownership or cleanup crosses between the 158 caller and 163-owned roots.

The historical host retains all 198 prior case identities in order. Existing unsafe-path, entry-bound and caller-created symlink assertions were expanded to exercise both modes with the same rejection requirements. Five new expanded cases use the real helper API to check exact disk/loader membership and caller values, empty default and evidence-only Maps, registered failed setup and rejection of unknown modes. The original membership assertions were not weakened or replaced. No new module, dependency, Rust change, manifest renewal or cap exception is introduced. All nine owned files passed the physical-line and byte caps; the largest is the 627-line historical host, or 628 entries when split on newline including its terminal empty entry.

## Authenticated actual outcomes

Root's `.local/open-bitcoin-dev/phase165-plan40-evidence-mode-root-source3-green-proof.json`, SHA256 `14b5674b3403ddc8e53451f905815fe095661294d318080eda02bf804b46cebe`, binds the four actual commands below. The reviewer authenticated the results, logs, validations, output receipts and checkpoints against their recorded hashes before using them. Every child started and exited zero, with null error/signal and no journal errors. Each validation and checkpoint has no failures.

| Actual command                                                            | Outcome          | Filtered | Assertions | Whole input rows |
| ------------------------------------------------------------------------- | ---------------- | -------: | ---------: | ---------------: |
| Original implementation/formal loaders plus strict missing-root case      | 3 pass / 0 fail  |      897 |         12 |             6922 |
| Five evidence-mode cases                                                  | 5 pass / 0 fail  |      198 |         15 |             6934 |
| Existing copied CLI closure controls, including expanded both-mode guards | 33 pass / 0 fail |      170 |        100 |             6946 |
| Nine-owner source caps                                                    | 9 files passed   |      N/A |        N/A |             6958 |

For each command, all expected rows equal both recorded before and after observations. Authenticated before snapshots and full raw logs are retained separately from produced results and receipts. The reviewer additionally rehashed every latest-generation input row against the current filesystem: all 6958 canonical presence/byte/SHA states matched with zero differences before writing this newly authorized report. Actual pass names match the intended original regressions and mode/closure cases; parser or loader failures were not substituted for the strict missing-evidence category.

The actual log hashes are:

- Loader: `76138cf2499dbe954fc5e95bfd439a4cc5a72f7de899d616b47551c128ab11ce`.
- Evidence mode: `014ce5508f67ced42aa70c6ca038a9ff66d4c7340915bf89bf25fa65099ea88a`.
- Closure: `730769fe0c4cb95eedc0eb4b07801d91dc5e683e5a36d1353a674515548c66e6`.
- Caps: `089044649c9bccc200d047f3c7f2d7b0d13b0dd8cc38d6278912a0be8a09771b`.

## Retained failure and remaining gates

The earlier full 158 run remains FAILED: 898 pass / 2 fail across all 900 names, 2113 assertions, 821.09 seconds of test runtime. Its two failures were the unchanged original implementation/formal optional-root membership cases; the strict missing-root case already passed. Root's immutable failed-batch closure is `56e806e35538af74035d48d6c355a2fb243ff77edd3007961caf3fdf5f3ad2e8`. The full raw log remains `3f4216b61e30e1da4785d906c64323382fd6bf2d5b9606de9ef4819b88e75c7f`. The 100 subsequent commands listed as unexecuted in that failed batch did not run. This targeted GREEN evidence establishes the repaired paths, not a fresh passing full 900 suite.

The renewed copier source gate is now review-clean only for the three hashes above. Root acceptance must precede the two renewed COPIED descriptors and current passed-copier binding changes. The exact whole original copier targets remain held at `418479c36f497919cf2ca289d93dcefaa8b009920dae467181cca528365c7f81` and `24bf75c09f40d0008bf528a7bd77fa9edf409e15ae967005cae02f6083bb25ae`; no replacement descriptor values were derived by this review. Renewed H source CLEAN plus Root acceptance must precede its external inverse to the complete original H target. The final nine-owner code gate must precede renewed ordinary SOURCE rows and input-pin values. The existing 192-row ordinary ownership table still contains the prior source values and is knowingly pending integration.

Fresh full 900, full 162, native 163, full history, selected 164/165, all 85 source CLIs and the complete default native verification contract remain required after those gates and data activation. Source CLI checks do not constitute native qualification. The earlier Task1, H and final-code review files remain immutable historical checkpoints. No canonical phase review, source, shared metadata, candidate value or Git state was modified by this reviewer; only this authorized report was created. No application import, helper execution or test was run by the reviewer.

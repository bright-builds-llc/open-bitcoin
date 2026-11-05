---
phase: 167-candidate-history-validation-rules
plan: "41"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, fjall, physical-inventory, logical-stamps]
requires:
  - phase: 167-38
    provides: Accepted opaque logical mutation stamps and tracked storage carriers
provides:
  - Bounded live physical observation under one original stamp and held root identity
  - Shared typed complete walker with unchanged strict offline errors and full equality
  - Native, filesystem, logical-authority and nonrefundable-work regression evidence
affects: [167-40, 167-39, 167-30, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private Changed/Refused routing, held read-only root, external cumulative work ledger]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/refusals.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
key-decisions:
  - Safe directory nlink differences are Changed while every Inventory field and full equality remain exact.
  - Bind one original pre-I/O stamp and a read-only held root across at most two actual attempts.
  - Keep requested content, visits and EOF allowances independent, cumulative and nonrefundable.
  - Preserve strict offline original errors and split cohesive refusal controls into the checked fifth owner.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-23T05:53:45Z"
commits: false
duration: "2h 14m 12.912s from initial source capture through final command completion; independent review and document QA are separate"
completed: 2026-10-23
---

# Phase 167 Plan 41: Bounded Live Physical Observation Summary

**Live physical observation now permits one safe changed attempt followed by two fully equal hashed walks, retaining the original logical stamp, held root identity and old work bounds.**

## Final Source Files

Exactly five declared owners changed across three tasks. Their final source totals82,587 bytes/2,286 normal lines; the largest owner is595 lines and the new refusal child is491, below its620-line target. All57 protected inputs remain exact, including pending40 opening/pressure and pending39/original305 sources.

| Source                                                                                 | Lines | SHA256                                                             |
| -------------------------------------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs`                     |   399 | `4102a31631cdb36d8d7a6d6436f4e079101a2eae8c396aba22135186f5a5450c` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs`                |   595 | `e0bb0f121a8494a152ddd5e7228caf6a1566a7917dfd9c9a3b4486a262215880` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live.rs`                |   276 | `7f5ec19447f5e65bca5c4c439b243d115b841df7f5e9204b7c3d6eb8244705bc` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests.rs`          |   525 | `757c08ede8a8e811b5e2c88e503dea526729a6e70633eab609a927d29a2b3526` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/refusals.rs` |   491 | `10315f637b0c352b001bc81f780cc386e116f6829659066565908c20f283fad6` |

## Bounded Implementation

The existing crate-private live gate alone delegates to the new private helper. One shared walker retains every identity field, complete64KiB hash chain, EOF read, opened-file/path check and full Inventory equality. Private Changed/Refused values carry the original StorageError; strict capture/copy/recheck, locking, worker recovery and safe producers return that original error immediately. Their signatures, messages and precedence remain. No public error variant, engine owner escape, production getter, dependency or writer algorithm was added.

Safe DIRECTORY nlink differences are Changed in both individual identity and complete common-entry comparison. They never become equality or success. Mode/type, regular-file hardlink, unsafe entry, permission, permanent I/O, resource and arithmetic conditions remain immediate Refused. Every Inventory root/descendant field, including directory nlink/length/mtime, stays in full equality.

The helper captures ONE original opaque stamp before filesystem work and checks it throughout both possible attempts and final returns. The Unix read-only root File pins the original inode; canonical path, directory type, device, inode and mode remain bound against held-fd and fresh-path metadata. Only this separate anchor omits dynamic DIRECTORY nlink/length/mtime. Missing/replaced/unsafe roots refuse. Non-Unix retains conservative strict single-attempt observation and original-stamp validation. No generation mutex spans I/O, stamp refresh, physical-counter authority or native pause exists.

The panic-free driver permits one second call only after actual Changed. There is no third attempt, sleep, backoff or fallback success. ONE external ledger preserves all work across attempts:8,192 visits before metadata,1,073,741,824 requested content bytes before read_exact, and8,192 SEPARATE one-byte EOF requests before reading. Partial failures keep the full requested charge. The per-capture536,870,912-byte/4,096-entry/depth16/path4,096 caps remain. Near-cap trees can conservatively exhaust work; finite work does not bound individual syscall latency or guarantee stable scheduling.

The simplification pass retained one typed walker/hash core, one external ledger, one original stamp, one held root and the explicit bounded driver. Test observation is cfg(test)-only and derives from actual attempt entry, completed hashing/EOF, completed captures and real full equality. No aggregate-only counter supplies a second-walk proof. New files have immediate infrastructure-none breadcrumbs; central manifest reconciliation remains Plan19-owned. Source scans and independent review establish no unresolved stub or unmodeled production trust surface.

## Actual Controls and Final Verification

The final live selection has23 meaningful named cases. Native controls rotate genuine originally populated namespaces, preserve all nine maps/catalog and the original stamp, and observe actual driver Changed results. Two distinct populated namespaces cause two Changed attempts and refusal. Same-value/ABA writes, actual blocked native conversion, foreign stamp, root rename/replacement/missing/symlink, readable chmod and security/resource controls refuse. The in-flight writer is released and joined before result assertions.

Both mandatory positive controls independently prove the second attempt's TWO full walks. L0 retains all first-attempt work; each later ledger delta matches its own actual entries/content/regular-file footprint. Completed hash bytes and EOF reads originate at the actual shared operations, and full-Eq observation follows the real comparison.

| Actual case                         | Second capture entries/content/EOF | Second full recheck entries/content/EOF | Result                                         |
| ----------------------------------- | ---------------------------------- | --------------------------------------- | ---------------------------------------------- |
| Native change followed by stability | 75 /67,169,275 /53                 | 75 /67,169,275 /53                      | Actual first Changed, second Ok                |
| Retained owned root directory       | 73 /67,167,281 /50                 | 73 /67,167,281 /50                      | Actual root nlink6→7, first Changed, second Ok |

The root child remains present through the complete second pair. Descendant directory nlink changes and simultaneous readable mode negatives exercise both shared classification paths. Actual OS controls report PermissionDenied and opened-directory IsADirectory/OS21. Actual truncation consumes19 bytes before UnexpectedEof while retaining the entire4,096-byte request charge. Restored-metadata content replacement, known descendant deletion, independent entry/content/EOF exhaustion, path/per-source boundaries and strict first-native-change original-error/no-retry controls pass. The short socket alias is uniquely/atomically created, removed immediately after bind before result handling, and the original canonical owned path is verified as a real socket. It uses canonical /tmp and leaves the production source traversal/TMPDIR unchanged.

All seven exact final commands ran fresh on ONE immutable all62-present freeze. Empty auxiliary harnesses do not count as discovery. These are108 overlapping test executions, not108 unique tests.

| Exact final timing key                |      Named passes | Full command wall seconds | Exit |
| ------------------------------------- | ----------------: | ------------------------: | ---: |
| `phase167-41-final-live`              |                23 |                   375.576 |    0 |
| `phase167-41-final-strict`            |                39 |                   216.300 |    0 |
| `phase167-41-final-canonical-genesis` |                29 |                   575.165 |    0 |
| `phase167-41-final-generation`        |                12 |                    10.406 |    0 |
| `phase167-41-final-seals`             |                 5 |                   171.907 |    0 |
| `phase167-41-final-format`            | check-only format |                     2.178 |    0 |
| `phase167-41-final-clippy`            |       strict lint |                     0.504 |    0 |

Every selected test command has zero failed/ignored cases. The original39 strict,29 canonical-genesis,12 generation and five seal cases remain discovered. Format retains workspace --check; Node Clippy retains --all-targets --all-features -- -D warnings. Pinned Bun1.3.9/Rust1.94.1, original target/cache/physical TMPDIR and command-timings/cooperative lock remain. Job intervals do not overlap and no first-party source write occurred during them.

Initial source capture2026-10-23T03:28:11.376Z through final command completion05:42:24.288Z is8,052.912seconds, including checked holds and development history. Independent review/document QA occur separately. No task or metadata commit was made under commits:false.

## Immutable Evidence and Source Inverse

Final packet `.local/open-bitcoin-dev/phase167/plan41/resume-directory-links/execution-v2/final-evidence-index.json` is1,593,395bytes, SHA256c9294784352156f0d5e767d29c040dd76a5c5cbe2cdb88607a7a8eb5fc518ccf. It authenticates4,524 references, all24 actual jobs, full argv/environment/raw/discovery/timing records,2,962 actual per-job source copies and TWO explicit absence observations. Six historical61-state jobs retain732 actual copies;18 later62-state jobs remain separately attributed. The two absences are Task2's new refusal-child before/after states:122 actual copies plus two absences, never124 copies. All final states are present and the seven final jobs contribute868 actual copies.

The final freeze is `execution-v2/freeze/result.json` under the same directory:128,650bytes/SHA256fbca4f1f4c220ced8ac0d65be2378bb7f94121d9e5af2f2705eb8e95ecf28d32. Every final job's62 before/after states match it. The201 native/lock files have separately authenticated actual copies; no per-job engine capture is claimed. Root's separate final-evidence authentication is SHA2566d465f0ba70f83dcf84a69ff5d6cf8a1b3b875c05c58618053c10313df9d2088.

Executor strict sandbox inverse/forward passes: `execution-v2/inverse-proof/result.json`,54,712bytes/SHA25664802b3171ad3a07054775a096f22995d95c4e18cf42e4b4ea5c631a9066b11d. BSD patch --batch -E -F0 -p1 restores ORIGINAL4ee360e5 inspection/copy bodies and its two true live-file absences, plus ONLY the a5d4cd73 refusal-child absence. It preserves57 protected states, giving62present→59present/3absent→62present. Original partial working bodies were never substituted for inverse authority. Patch/logs and actual sandbox endpoints are retained; live source was not patched.

The independent NEW source inverse is SHA25676fa5f0593cf1f1881f212aa62afef16d822d82e4f3aa3efaaabefa6fa9144f0. Old502 executor/root/reviewer proofs retain their historical source hashes and supply no new proof credit. Original40 automatic-pressure and the original305 fixture remain whole and receive no forced maintenance.

## Checked Refinements and Preserved Outcomes

The following history remains attributable to its actual source and command; no old pass or preflight substitutes for the final matrix.

| Stage                                | Retained actual outcome                                                                                                                                                                                                                                                        |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Historical Task1                     | Work contracts2PASS; genuine native RED1FAIL on the still-strict live route. All nine maps/catalog and original stamp remain valid while physical inventory changes.                                                                                                           |
| Initial Task2                        | Nine cases7PASS/2FAIL; exact original failed entry was unlogged.                                                                                                                                                                                                               |
| Diagnostic setup and observations    | E0425 is zero-test setup. Diagnostic2 fails inside an intermediate strict after-native capture. Diagnostic3 proves common-entry keyspaces/9 directory nlink5→7/same device/inode/type/mode and actual driver Refused at attempt1. No namespace name is inferred from numeric9. |
| Checked directory refinement         | Both classifiers distinguish safe directory links; independent anchor excludes dynamic directory nlink; full Inventory equality stays exact. Explicit fifth owner solves measured700–800-line projected test fit. Revised Task2 has11PASS on62 states.                         |
| Socket fixture development           | Twelve cases10PASS/2SUN_LEN setup failures; later byte/depth/entry loop cases were not reached then. Corrected twelve-case selection passes with real sockets through owned short aliases. Canonical /tmp portability is separately root-authorized.                           |
| Old502 final matrix                  | Six commands pass108 overlapping executions plus format; strict Clippy exits101 for decimal permissions zero. This entire matrix remains failed.                                                                                                                               |
| Exact octal correction and preflight | Only from_mode(0)→from_mode(0o0), two bytes, changes the child. Other61 sources stay exact. Strict preflight passes; all seven final commands then rerun fresh on fbca.                                                                                                        |

The original expected RED unwound handles before explicit fixture removal, so its explicit cleanup was not reached and is not claimed. Later owned fixtures use catch/cleanup and release/join controls. The initial diagnostic placement mistake and index-preparation mismatch are preserved as setup/document-tool history, with corrected records separately versioned.

The old Clippy command completed naturally after315seconds; the new preflight completed after9m25s. Read-only diagnosis sampled a SearchPath/ReadDir wait while simultaneous lsof showed FD6 pointing to packages/target/debug/deps. These observations strongly correlate; the syscall's fd argument was not captured and the underlying cause remains unknown. The new live command's compiler completed naturally in4m23s. Process/log observations are retained; no command was killed or source/cache/target changed to resolve these waits. Quiet/advisory output was not treated as timeout.

These are explicit checked/refined development corrections and one blocking fixture-literal lint fix. No unapproved architectural deviation, suppressed check, skipped case, budget/deadline/attempt increase or source owner beyond the checked five was introduced.

## Deviations from Plan

The directory-link classifier, separate anchor policy and fifth test owner follow the explicitly checked plan refinement. [Rule 3 - Blocking] The real socket fixture's SUN_LEN setup limitation and the octal-literal Clippy requirement were corrected under separate bounded root releases. Their original failed receipts remain intact; neither correction changed production behavior, skipped an oracle or increased a limit. No commits were made under commits:false.

## Independent Core Review and Document QA

The complete immutable report was read: `.local/open-bitcoin-dev/phase167/plan41/independent-review/revision2/167-41-CORE-REVIEW.md`,13,876bytes/SHA25679ca29e9ca217d344c9a08fd38ea650722699a26eecbbaf7e9f65a68bd54b16d. Its source/API/security/evidence/source-inverse verdict is CLEAN with zero critical, warning or information findings. It independently authenticates all4,524 references, all24 jobs and the final per-walk trace arithmetic, and reproduces a distinct strict inverse. This core report has no SUMMARY dependency and grants no root acceptance.

Scoped document QA uses the installed GFM/frontmatter mdformat plugins, exact lifecycle/baseline/requirements checks, all five source hashes and GSD verify-summary --check-count5. Prior receipts and initial/final document copies are retained under `execution-v2/document-QA/` and `execution-v2/document-QA-revision2/`; the corrected summary's receipts and copies are under `execution-v2/document-QA-revision3/`. A separate independent SUMMARY-QA packet must bind this actual summary hash and the immutable core report hash; root acceptance follows separately. No circular report/index dependency is introduced.

Repo AGENTS, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed execution. Both active lesson files were read completely:9,838bytes/3,280 conservative tokens. No lesson/archive or shared STATE/ROADMAP/requirements/guard/manifest/Git write occurred.

## Remaining Gates and Scope

The requirements-completed array copies all five plan IDs verbatim for this bounded prerequisite contribution. CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain globally OPEN. Root alone accepts41 and releases40ALL7fresh62→39ALL9fresh62→30ALL17fresh62. All prior40/39/30 failures, frames and original inverse authorities remain historical. This contribution proves neither automatic journal-pressure success nor original305-prefix support, native quiescence, actual Knots differential qualification or whole-phase completion.

Later35FIRST/13/product/corpus/formal C/D/E, canonical source review before17 guard renewal including123 ownership,18security/native/mutants,19parity/README/manifest,25writing-for-agents and20full default native verifier/fresh pure-core coverage/Bazel/source/security/goal/lifecycle/normal-hook Git gates remain. Phase168 stays unstarted. No external user setup is required for this contribution.

## Self-Check: PASSED

All five final source files and required evidence exist and match the immutable final freeze. All seven exact final commands passed on62 unchanged states; strict inverse/forward reproduces its exact original and final targets. Scoped GFM/frontmatter formatting, lifecycle/baseline/requirement equality, five source checks and GSD summary validation pass. No commits are expected under commits:false. Independent SUMMARY binding and root acceptance remain separate.

---
phase: 165-shared-exact-work-authority
plan: "35"
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T22:30:49Z"
status: proposed-move-only
---

# Plan165-35: Exact source-table capacity move

This ledger authorizes a proposed two-file data relocation after Plan34. It plans no repair to the remaining source-guard failures. At planning time there are34 plans,33 summaries and final14 pending. No source mutation or candidate activation has occurred under this plan. Root owns metadata and later repair sequencing.

## Authentic evidence and residual failures

| Evidence                                                                 | Full raw SHA256                                                  | Meaning                                                                                                                               |
| ------------------------------------------------------------------------ | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| .local/open-bitcoin-dev/phase165-source-guard-repairs-before.json        | a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd | Complete338 script bodies plus6 metadata/Rust entries captured before remaining repairs;344 entries.                                  |
| .local/open-bitcoin-dev/phase165-final14-native-r2-failure-proof.json    | 964e133b77644b5f2ec68268a3ed33ef3097061ce337a8b666e38e8ccbd74aca | Second ENTIRE default native attempt failed at73's stale coverage anchor.                                                             |
| .local/open-bitcoin-dev/phase165-final14-native-r2.log                   | 6b892fd4d81d2c1cea6e81391a14e495854e0396424d73f96a6f04d3065754bc | Actual R2 failed log; later matrices/Rust/coverage/Bazel did not run.                                                                 |
| .local/open-bitcoin-dev/phase165-final14-native-r2-source-closure.json   | e850631d377febb3ada3166f0fa05ce17288f52c8087ad46007f2388035b4330 | Accepted4829-entry raw generation to carry forward and expand.                                                                        |
| .local/open-bitcoin-dev/phase165-final-native-source-cli-diagnostic.json | 8be56c1ff47c6e99d9acca120c00f0fe609131ead92a00ee803098bcff9572bd | Serial81 source CLIs:69 pass/12 fail; excludes two generated benchmark/Bazel prerequisites from83. Diagnostic, not native acceptance. |
| .local/open-bitcoin-dev/phase165-filter-guards-diagnosis.json            | 482ea24309e54c5295a4f0ab9185cd694fde5df2bcef2fef12ff2cc53770729e | Actual159 unit628pass/9fail/637tests;161 unit147pass/2fail/149tests.                                                                  |

R1's actual72 failure remains failed evidence; accepted34 repairs only that mismatch with source/Rust distinctions intact. R2 remains failed. Remaining families73/75,123/127/128/129/134,155/156/157/159/161 need later separately checked plans. The159 failures comprise eight baseline failures and one absent literal mutation anchor; no no-op mutant counts as semantic evidence. Plan35 leaves every such guard/test/body/hash unchanged. Full native and all subsequent phase gates must run fresh after all repairs.

## Exclusive mutation owners and byte grammar

| Owner                                                     | Authentic lines/bytes | Authentic full raw SHA256                                        | Permitted change                                                                              |
| --------------------------------------------------------- | --------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| scripts/check-phase165-shared-exact-work/contracts.ts     | 620/86079             | 7e53ef25bd578218d680f40ce0809ca88f068796953ec78c5b87f018bbf4a6eb | Remove exactly lines5–169 with final LF; extend existing named re-export by SOURCE_CONTRACTS. |
| scripts/check-phase164-canonical-identity/test-modules.ts | 388/20037             | c021c8a3f6e12a410d986d63050e166c32a2345a4045d2422ed2d4f4e881b15f | Preserve complete original bytes as prefix; append one LF then exact moved block.             |

The moved block is three complete doc-comment lines plus the full typed160-row SOURCE_CONTRACTS declaration, with no aliases/imports/calls or N/H/E/P references. Its declaration alone, original lines8–169 including final LF, is162 lines/24806B/SHA48a754ce025478083f3aba14f015af8f66db0d35bcd88e59192772e2a5aee679. The complete comment+declaration is165 lines/25065B/SHA6e4ab77101d725c067bde96beda6447d040b24a64fb24aeb57fdd7d2a829e188.

Replace exactly `export { TEST_OWNERS }` with `export { SOURCE_CONTRACTS, TEST_OWNERS }`, preserving its existing module specifier. Private N/H/E/P constants and all other contracts bytes remain exact. Append-only host preserves all old TEST_MODULES/SOURCE_FILES/TEST_OWNERS/types/helpers. No formatting or row packing. The original same-edge leaf remains dependency-free, so no new cycle is required.

Planning-derived complete candidates, never treated as executed evidence:

| Owner                  | Candidate lines/bytes | Predicted full raw SHA256                                        |
| ---------------------- | --------------------- | ---------------------------------------------------------------- |
| current contracts      | 455/61032             | c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4 |
| existing old data host | 554/45103             | 10a0a32f77e9ae44baac673085584fe08d331f58b433b7a6c83be22af3e3ac95 |

Current620/628 capacity cannot accommodate more than eight later one-line rows. This move creates readable room without adding future rows or changing policy. Full-file inverse removes precisely the appended separator/block from the host and restores the exact block at its original contracts offset while reversing only the named re-export. Both reconstructed bytes and full hashes must equal authentic before; array/count checks alone are insufficient.

## Existing contracts measured during planning

Fresh Bun1.3.9 imports produced these actual ordered JSON values. Executor must capture its own BEFORE and actual AFTER and compare complete values; these observations are not post-move results.

| Export                   | Count | SHA256(JSON.stringify(value))                                    |
| ------------------------ | ----- | ---------------------------------------------------------------- |
| current SOURCE_CONTRACTS | 160   | eace9eddbdc2c7ded044a13ace3fb4d4c827c65c38e133991c66fbd3ce993997 |
| current/old TEST_OWNERS  | 88    | ef938436baf9f61f16f6eb0a47d078490462d675f7b007f8f231150fa937a23a |
| old SOURCE_FILES         | 143   | 6047f1b3e4a179b5503222bc96a2843d9cd5d442b6cdcb88572c013710c7be8e |
| old EDGES                | 606   | 0ba68857323ccaa516548503e00d5265b6a82e78dbacc4759ba3a5a5e3b5338c |
| old TEST_MODULES         | 26    | d47862d042e6376cce6f42e7d737ba5929228ff0a466800fe6ccff29e8d75614 |
| current INPUT_FILES/pin  | 279   | 80dca53e69a2f0c917ae78297108ee16ad1187f2afb6d46e255790585770f624 |

Compare every exported current data value too, including all mutation tables, bridge/history/corpus/claim/inverse constants. Preserve all original initializer/factory/entrypoint closures. Source identities are literal-bearing and remain identical; neither moved data owner is a SOURCE row or manifest/PROVENANCE_DIGEST binding. No current inventory pin renewal, self-referential table hash, historical normalization change or source-row addition follows from this move.

The existing API remains current contracts→existing old data host. Read-only consumer/needle analysis found no physical SOURCE declaration mutator requiring retargeting; tests consume imported SOURCE values. Current627-line tests and183-line test-support remain exact. CLI_MODULES already copies both complete existing modules; copiedInputs requires no binding edit. Derive actual graph from actual files; planning expectation is CLI14/static39 and unchanged inputs. Import-time module loading and bounded fixed input loading remain separately attributed.

## External root proof and evidence contract

Root must prepare/review `.local/open-bitcoin-dev/phase165-plan35-proof.ts` before executor commands; it is an external proof artifact, not a third source mutation owner. Its `preflight` mode authenticates344-entry before provenance, both complete source bodies, all before runtime values, unique move/re-export grammar, complete inert candidates, two complete inverses and physical caps; candidate imports use a complete isolated copy of the actual import closure. It emits canonical presence/bytes/SHA observations plus complete exported JSON equality and candidate hashes without replacing live owners. Root independently accepts this before live move.

Its `after` mode reads actual live bytes and fresh actual module exports, verifies exact approved candidates, every full-file inverse and old host prefix, all preserved exports/order/types/INPUT pin and original initializer/factory/entrypoint identities. It must fail nonzero on every mismatch and record actual values. Count-only checks, source-derived expected outputs and merely assuming an existing copied leaf is sufficient do not satisfy the proof. No mirror of a production algorithm is needed: existing baseline, input, copied CLI, exact inverse and ancestry tests verify transparency.

Root prepares a separate reviewed external collector/driver before verification. Preserve accepted4829 R2 raw entries, attributing only the two approved owner deltas, and union new35 artifacts/reviews/proof/collector/driver, fresh R2 proof/log/closure, diagnostics and their logs, complete344-entry before receipt and every actual source/module/factory/helper/static/dynamic/optional-absence read. Include actual source-corpus and historical adapter/ancestry reads. No prior entry is dropped. Record canonical path, presence, bytes/full SHA, exact membership and mismatch reasons; check full closure before each command and afterward, then close every actual log with full SHA and command/case count/exit/duration. Root must review changed old entries explicitly. No metadata projection stands in for a raw source union.

The plan's focused selector includes baseline, fixed input/inventory, bounded copied CLI, all existing exact tooling inverses and legacy ancestry families. Derive actual nonempty selected names from the627-line test/factory and reject ignored/skipped discovery. Add any further affected family to the root driver before its freeze. Direct live164/165, fast158/162, breadcrumbs, managed cap checks and whitespace pass on that same accepted source generation. Full20-minute matrices are final14 requirements, not duplicated35 checks.

## Decisions and dependencies

| Decision | Plan/task | Coverage | Preservation proof                                                                        |
| -------- | --------- | -------- | ----------------------------------------------------------------------------------------- |
| D-01     | 35/1–2    | Full     | No domain/dependency change; exact existing data only.                                    |
| D-02     | 35/1–2    | Full     | Arithmetic and numerical identities unchanged.                                            |
| D-03     | 35/1–2    | Full     | Compact/admission sources and rows remain exact.                                          |
| D-04     | 35/2      | Full     | Source receipts remain bounded; no daemon claim.                                          |
| D-05     | 35/1–2    | Full     | Header owners/identities preserved.                                                       |
| D-06     | 35/1–2    | Full     | Candidate eligibility/arrival evidence unchanged.                                         |
| D-07     | 35/1–2    | Full     | Authority/mutation guards and all owner data remain exact.                                |
| D-08     | 35/2      | Full     | Tie limitations/claim constants preserved.                                                |
| D-09     | 35/1–2    | Full     | Read-only refusal gate and four selectors preserved.                                      |
| D-10     | 35/1–2    | Full     | Historical codec/bridge attribution unchanged.                                            |
| D-11     | 35/1–2    | Full     | Exact arrays/input pin/copy/inverse/ancestry guards and default-off networking preserved. |
| D-12     | 35/2      | Full     | No166–168 work or new qualification.                                                      |

This is preservation coverage for a bounded continuation, not a claim35 independently implements all phase decisions. CWOR-05 contributes only; no CWOR-04 or completion flip. Existing plans retain all other phase requirement ownership.

| Task | Needs                                                                                  | Creates                                                                  | Checkpoint                                                 |
| ---- | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------- |
| 1    | Accepted34, authentic before bytes, root-reviewed inert runner/candidates/read closure | Exact two-file move, actual full-file inverse and runtime equality proof | No human checkpoint; internal root review before mutation. |
| 2    | Accepted actual two-file move and reviewed collector/driver/freeze                     | Actual affected guard receipts, complete closed raw union, Summary35     | No human checkpoint; internal independent root acceptance. |

Plan35 wave34 depends165-34. Root alone may amend pending14 to wave35/depends165-35 with bounded context; retain all original14 tasks/XML/commands and every other33 existing plan. Future source-repair plans must later interpose before14; this ledger does not plan or execute them. After all repairs, final14 must regenerate real LOC and run its ENTIRE fresh default native, both full matrices, Rust/bench/fresh coverage/Bazel, whole-phase code/security/formal/lifecycle and normal-hook strict-main-push gates. No partial35 receipt replaces those outcomes.

## Planning checks and handoff

Read AGENTS.md, Bright Builds sidecar, overrides, architecture/code-shape/verification/testing/TypeScript standards, complete active7188B lessons, GSD plan skill/schema and phase decisions. Level0 discovery: existing data/copy/re-export patterns. Simplification selects the existing leaf and existing edge; there is no new algorithm, module or third mutation owner.

Before handoff, run configured GFM/frontmatter Markdown check then scoped format only if needed; validate plan frontmatter/structure/lifecycle metadata and parse every automated command with shlex and Bash syntax checks. Only the two new planning artifacts are owned by the planner. Parent owns checklist/roadmap/state/14 changes, independent checker, source acceptance and execution routing. Return hashes and proposed14 delta, then hold; no execution auto-chain.

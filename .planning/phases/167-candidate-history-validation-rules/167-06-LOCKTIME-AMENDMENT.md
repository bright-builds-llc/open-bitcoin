# Plan167-06 locktime interpreter amendment

## amendment-locktime-dispatch | 2026-10-21T19:20:54Z

This append-only amendment belongs to the active gsd-plan-phase revision workflow and lifecycle167-2026-10-21T16-38-23/yolo. Original Plan06 generated_at2026-10-21T16:55:59Z, wave4, dependencies167-04/05, both tasks, requirementsCVLD-01/02/03/04 and both must-have truths remain. Task1 is byte-identical to the pre-edit snapshot. Root independently checks this amendment before the executor edits any newly added source owner. This planning artifact grants no implementation or phase acceptance by itself.

### Snapshot and checkpoint evidence

The complete pre-edit Plan06 is preserved at `.local/open-bitcoin-dev/phase167/plan06-locktime-amendment/before-PLAN.md`, SHA256 `e1dda77d7a31215314d0590a016be4ab0964508e5767365d2cfb28debe0bc778`. Its bytes matched current06 PLAN before this revision. The earlier167-06-AMENDMENT.md and its original snapshot remain unchanged historical evidence.

Executor checkpoint notes at `.local/open-bitcoin-dev/phase167/plan06/checkpoint-notes.md` report1/2 tasks complete. Task1 focused10/10 passed. Task2's initial9-case behavioral attempt contains seven actual omitted-rule/policy/budget RED failures; CLTV/CSV instead fail in their omission controls with UnsupportedOpcode(0xb1/0xb2). After current coins/context/budget wiring,7/9 pass and those two remain blocked. The prior400unit+3integration+23doctest regression and21focused results explicitly excluded those two cases. Those partial results are not whole-plan acceptance or valid CLTV/CSV canonical-bypass RED. Preserve all original failure/setup logs and distinguish the corrected MTP inverse from its earlier mistaken expectation as the checkpoint does.

Planner independently inspected current sources: legacy.rs has no0xb1/0xb2 dispatch arms and reaches UnsupportedOpcode; opcodes.rs declares neither constant; stack.rs::decode_script_num hard-limits four bytes; script.rs owns module registration and the ScriptError enum/Display. Existing LegacyExecutionContext already carries transaction,input_index,verify_flags; public verify_input_script already routes real base/witness/tapscript execution through the existing interpreter. No context type, signature-checker implementation, new crate/dependency, witness/taproot owner or extra test registrar is required.

### Exact ownership delta

The current ten Rust owners remain with the executor. Exactly five consensus owners are added to Task2:

| Added owner | Bounded responsibility |
| --- | --- |
| packages/open-bitcoin-consensus/src/script.rs | Register private locktime module; NegativeLockTime/UnsatisfiedLockTime errors and Display arms. |
| packages/open-bitcoin-consensus/src/script/legacy.rs | Dispatch executed CLTV/CSV instructions to the pure helper with current transaction/input/flags. |
| packages/open-bitcoin-consensus/src/script/opcodes.rs | Declare0xb1/0xb2 constants, retaining their NOP2/NOP3 identity. |
| packages/open-bitcoin-consensus/src/script/stack.rs | Reuse signed numeric decoding with an explicit five-byte request, preserving every existing four-byte caller. |
| packages/open-bitcoin-consensus/src/script/locktime.rs | New pure operand/transaction lock checks; no independent acceptance or context authority. |

Tests stay in the already owned packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs with its existing engine/tests.rs registrar. The ten-owner union becomes fifteen; Task1's seven owners are untouched by this planning revision and its content remains byte-identical. No other source/error/export/registrar/test-owner permission is implied. New locktime.rs needs pinned-source breadcrumbs; Plan19 retains manifest ownership.

### Pinned semantics and source basis

Local pinned interpreter.cpp SHA256 is `6e7367740f8d8ecc8dca4ec129d66782d09fa5733acc96469f367267fb42cc60`; script.h SHA256 is `ebd3aa6de49ac597957923328bca3abc8697e63f2b0c3b06d260f6ebdd89016c`. They match accepted05 source evidence for the locked Knots pin `a9aee730466ac67d35a3c03ee24676be5e045878`. No daemon was run and no external library was chosen.

- interpreter.cpp::EvalScript521–590: absent CLTV or CSV flag immediately breaks as NOP2/NOP3, before stack, numeric or checker access. DISCOURAGE_UPGRADABLE_NOPS applies only to the separate NOP1/4–10 case592–600; do not attach it to inactive CLTV/CSV or expand unrelated NOP policy. Unexecuted conditional arms retain existing skip behavior.
- Active opcodes peek stacktop(-1), require one item, and decode CScriptNum with explicit max5 and MINIMALDATA-dependent minimal encoding. Negative values fail before CSV's operand-disable shortcut. Signed magnitude includes negative zero; no u32 narrowing may discard a legal fifth byte. Six bytes fail numeric size. Existing ordinary numeric callers keep max4. This amendment does not retrofit unrelated arithmetic or push policy.
- interpreter.cpp::CheckLockTime1747–1781: compare operand and tx.lock_time on the same side of fixed500000000; require operand<=transaction locktime and this actual input's sequence!=0xffffffff. Equality succeeds. Neither candidate height/MTP nor mutable fixture threshold replaces these fields.
- interpreter.cpp::CheckSequence1784–1830: after the opcode's nonnegative operand-disable NOP, require tx.version>=2 and actual input disable bit clear; mask both operands with type bit1<<22 and low0xffff, require matching height/time type and masked operand<=masked input. Ignore other bits. BIP68 block-age conversion is a separate existing transaction rule, not part of this opcode comparison. Reuse TransactionInput constants.
- script.h::CScriptNum245–268 supplies signed/minimal/size semantics and script.h197–200 supplies aliases. BaseSignatureChecker in interpreter.h defaults checks to false; absent transaction context must not invent successful active checks. A CSV operand-disable NOP still precedes transaction checker access as in the pin.

The new concrete key link is legacy.rs→locktime.rs through a real locktime:: call. It is pending source implementation; metadata validation cannot establish that it currently executes.

### Execution and evidence contract

Continue RED→GREEN with funded-output execution, without relabeling missing opcodes as valid canonical flag-bypass RED. First capture successful active-opcode cases failing for missing implementation; then implement the bounded helper/decoder/dispatch and require the canonical flag-bypass controls to pass as paired fixture omission success versus selected canonical refusal. Never turn flags off on the canonical path or replace UnsupportedOpcode with unconditional success.

Use actual accepted funding coins and ordinary canonical stages for reachable positive/equality/refusal controls, with non-consuming OP_DROP, final transaction context and satisfied separate BIP68 age prerequisites. Explicit mechanical-fixture omitted-flag controls retain their taint. Some isolated opcode boundaries cannot reach script execution through a full canonical block because independent BIP113/BIP68 checks reject first; exercise those with public verify_input_script using the actual funded output and transaction/input/derived input context, then identify them as interpreter evidence rather than canonical acceptance. This is actual script execution, not a detached numeric calculator.

Tests must cover both opcodes' satisfied/equality/negative/empty/size/refusal cases, signed five-byte values including u32::MAX and above without truncation, six-byte overflow, negative zero and nonminimal encoding with explicit MINIMALDATA on/off, CLTV threshold types/final input, CSV version/input-disable/operand-disable/type/masks/ignored-bit ordering, inactive NOP2/NOP3 even with DISCOURAGE_UPGRADABLE_NOPS, skipped branches and retained four-byte arithmetic rejection. Assert typed lock errors and non-consumption; arbitrary failures or UnsupportedOpcode are insufficient. Invalid staged bodies retain accepted tip/exact work,coins best block/outpoints/undo without absorption. Direct interpreter probes leave chain state untouched and claim no stage or product authority.

Original Task2 rewards+actual fees,149/150/299/300 monetary boundaries,100maturity/99-depth refusal, LOW_S/MINIMALDATA/CLEANSTACK separation, consumed-only creating-MTP, same-block/branch metadata, every other script flag and aggregate80000/80001 obligations remain. The checkpoint does not establish those outstanding obligations complete. Preserve accepted01/03/04/05 contracts, exact selected rules and original-clock/generation protections from Task1.

After focused phase167_scripts discovery succeeds without exclusions, run the final full affected consensus+chainstate --all-features regression serially through the existing timing/lock wrapper, with no locktime skips. This catches old numeric/interpreter consumers as well as actual coins paths. Retain exact nonzero discovery counts and failed attempts. Follow existing scoped formatting/Clippy and final owned diff review requirements. Full native/fresh coverage/Bazel and independent final security/goal/source acceptance remain Plan20; root alone controls saving. No production authority, support scope, evidence class or deadline changes.

### Decision coverage

| Decision | Plan/task | Coverage | Retained or completed obligation |
| --- | --- | --- | --- |
| D-02 | 06.2 | Full | Real selected reward/fees/maturity controls remain; no genesis outputs or fixture promotion. |
| D-03 | 06.1–2 | Full | Task1 unchanged; actual transaction locks and consumed creating-MTP remain branch-derived. |
| D-04 | 06.2 | Full | Implement missing CLTV/CSV and actual selected interpreter wiring without policy flag leakage. |
| D-05 | 06.1–2 | Full | Original shared header/clock authority retained; opcode fields do not substitute candidate time or params. |
| D-07 | 06.1–2 | Full | Existing earned coins/stage owner remains; helper and direct interpreter tests confer no product authority. |
| D-08 | 06.2 | Full | Source-correct refusals preserve accepted state, with real positive and single-flag omission controls. |

Other decision mappings and requirements remain unchanged; no deferred idea is introduced. Full denotes planning coverage, not executed acceptance.

### Planner validation boundary

This bounded revision edits only167-06-PLAN.md, this new append-only amendment, and targeted06 notes in167-DECISION-COVERAGE.md, plus the explicitly required ignored snapshot. AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, architecture/code-shape/testing/verification/Rust/local-guidance standards, both complete active lesson inputs(8856bytes/2953 conservative tokens), current context/research and accepted01/03/04/05 summaries informed the revision. No project skills were present. Existing local source evidence satisfies discovery; no new dependency/API research is needed.

No source edit, test/build/Cargo/Bazel run, Git operation, subagent, shared STATE/ROADMAP/requirements/config/todo change or Plan02 producer/acquisition/corpus inspection/mutation was performed. Root's independent checker must accept the precise delta before implementation. All implementation evidence, new active-opcode positive/negative tests, completed outstanding Task2 obligations and final non-skipped regression remain pending.

### Planner artifact checks completed

gsd-tools frontmatter validate returned valid:true with no missing fields. verify plan-structure returned valid:true,2tasks,0errors/0warnings and all required task elements present. Direct pre-edit snapshot comparison confirmed SHA256 above, byte-identical Task1, unchanged phase/plan/type/wave/dependencies/requirements/generated_by/lifecycle/generated_at, unchanged two truths and task count, exactly the five listed added owners and no removed owner. Plan06 retains exactly its opening/closing standalone frontmatter delimiters. These are planning metadata checks only; no source key-link or future implementation pass is asserted.

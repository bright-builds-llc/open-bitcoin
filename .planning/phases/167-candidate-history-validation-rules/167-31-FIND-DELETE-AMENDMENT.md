---
phase: 167-candidate-history-validation-rules
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T10:03:00Z"
requirements: [CVLD-04, CVLD-06]
commits: false
status: candidate-awaiting-independent-check
source_execution_released: false
---

# Plan31 FindAndDelete refinement

This is a formal context-only revision during active Plan31, authorized by `.local/open-bitcoin-dev/phase167/plan31/find-delete/root-planning-release.json` SHA256 `ad1b737392b11e465e58c4ef7fc1ec47cf3bbbdb9fdf5bb01264337da1ff5c7b`. Source correction remains held until independent check and root release after the executor's current command completes. The executor may continue its other owned work, retains sole build ownership and holds final SUMMARY. No planner source edit/build/source formatting/Git/guard/shared metadata/canonical report edit or approval occurs.

## Exact correction and narrow supersession

The existing encoding helper's byte-by-byte search damages push payloads, nonminimal PUSHDATA length/payload sequences and malformed suffixes. Its empty-signature early return also conflicts with the pin's encoded search pattern. The approved preflight is `.local/open-bitcoin-dev/phase167/find-delete-preflight/report.md`, SHA256 `5a7eb6f62477738131f925e6e0ad9bb8bd3a21cef77d200590240b54ad2d4a26`. These are demonstrated pre-existing pin bugs; their changed outputs are intentional corrections, not preservation. This amendment expressly supersedes only Task2's promise of unchanged existing small-script behavior for opcode-boundary matching, repeated boundary deletion, complete malformed-tail preservation and empty-signature OP_0 handling. Every other small-script behavior, prior test and original Plan31 obligation remains.

One existing production/test owner is permitted: `packages/open-bitcoin-consensus/src/script/encoding.rs`. No new source owner, public API, helper signature, module, dependency, primitive Amount change or guard expansion. The stable encoding observation is SHA256 `de22fd4f0fa9d2f079eb714d52d1fba1431868b53ae1ff4f76c0b40b03c4fab1`,183 physical lines. Its helper stays `remove_signature_from_script(&ScriptBuf, &[u8]) -> ScriptBuf`; unchanged encode_push_data supplies the pattern and unchanged from_serialized_bytes rebuilds the result.

The exact pin is Bitcoin Knots29.3.knots20260210, retained commit `a9aee730466ac67d35a3c03ee24676be5e045878`. Free `FindAndDelete` is in `script/interpreter.cpp:228–255`; `CScript::GetOp` delegates to `script.cpp:383–433`. The planner captures only stable encoding and these pin references, not active31's other ten source files, SUMMARY/execution receipts or root live metadata. The preflight's older observations of callers identify reading history, not an immutable capture or new authority to edit them.

## Execution contract for the existing owner

1. Build the exact search pattern through unchanged encode_push_data, including empty signature -> byte00. The pin's empty-pattern guard does not justify treating empty signature as an empty encoded pattern.
1. Compare only at the initial cursor and successful GetOp instruction boundaries. Remove adjacent exact pattern matches repeatedly at that same boundary before advancing.
1. Save the next nonmatching instruction start and advance once using existing read_instruction_syntax. Copy the original complete instruction bytes into the output; do not decode/re-encode or minimalize a retained push.
1. On syntax failure append the complete original suffix from the saved failed-instruction start and stop. Do not use the partially advanced error cursor, discard malformed bytes, return an execution error or continue searching inside the malformed tail. Explicit handling of this stop condition is intentional observational semantics.
1. Rebuild through from_serialized_bytes, preserving wide raw bytes. Never restore the bounded-constructor fallback. Do not use read_instruction's520-byte element limit for GetOp scanning.

Existing BASE CHECKSIG/CHECKSIGVERIFY and sequential multisig calls remain unchanged; WitnessV0 bypass and Tapscript dispatch remain unchanged. This correction does not implement CONST_SCRIPTCODE/removal-count handling, CODESEPARATOR, sighash construction, signature checker/validation, NULLFAIL, flags, signature order or crypto semantics. No fake checker-success test or executed pinned-process/differential claim is permitted.

## Required behavioral evidence

Tests stay inline in encoding.rs and carry the existing `phase167_serialized_script_paths` filter. They must expose genuine behavioral RED before the fix, then GREEN, retain all prior tests and keep formatted source\<=628 with normal readable code. Test one concern per case with explicit Arrange/Act/Assert.

| Signature hex | Script hex           | Required result hex | Concern                                                      |
| ------------- | -------------------- | ------------------- | ------------------------------------------------------------ |
| 30            | 03 01 30 99 51       | 03 01 30 99 51      | Embedded target inside complete direct-push payload retained |
| 30            | 4c 01 30 51          | 4c 01 30 51         | Nonminimal push of the same item retained byte-exact         |
| 30            | 4c 02 01 30 51       | 4c 02 01 30 51      | Target inside PUSHDATA1 payload retained                     |
| 30            | 01 30 01 30 51       | 51                  | Adjacent exact boundary pushes removed repeatedly            |
| 30            | 51 01 30 01 30 52    | 51 52               | Retained instructions around repeated boundary removal       |
| 30            | 01 30 01 30          | empty               | Entire boundary-matching script removed                      |
| 30            | 4c 03 01 30          | 4c 03 01 30         | Malformed tail with no prior removal retained                |
| 30            | 01 30 4c 03 01 30    | 4c 03 01 30         | Complete malformed tail retained after removal               |
| 30            | 51 01 30 4c 03 01 30 | 51 4c 03 01 30      | Prefix/removal/malformed suffix correspondence               |
| 30            | 01 30 4c             | 4c                  | Truncated length byte retained                               |
| empty         | 00 51                | 51                  | Empty signature targets actual boundary OP_0                 |
| empty         | 01 00 00 51          | 01 00 51            | Empty target does not delete pushed zero payload             |

Also cover the report's DER-plus-sighash-shaped embedded target vector, nonminimal PUSHDATA1/2/4 encodings, malformed length/payload tails for all three, complete pushes larger than520 containing the target followed by an actual boundary target, existing wide reconstruction, and sequential removals for different signature patterns as in unchanged multisig composition. Literal source expectations are not cryptographic spend acceptance or executed pin evidence.

The unchanged Task2 path and script regression commands must discover the new tests. The executor must rerun every original31 automated command against final source, retaining complete RED/GREEN, setup failures, discovery counts, line counts/source hashes and all failures before SUMMARY. Existing BASE/Witness/Tapscript suites remain verification gates; helper-only tests do not prove broad signature parity. Strict Clippy and all final17/18/19/20/25 source/security/goal/native/fresh-coverage/Bazel/lifecycle/compact/Git gates remain.

## Scope and preservation

All original31 task XML, frontmatter, ownership and automated commands remain byte-exact. Only an explicit `<find-delete-refinement>` is added inside context; this new amendment is separately tracked. Eleven owners/three tasks and the33-plan/61-task/28-wave schedule remain. Other32 plans remain byte-exact. No new task, requirement closure, phase acceptance or source execution follows.

Before receipts under `.local/open-bitcoin-dev/phase167/plan31/find-delete/plan/` retain all33 plans, prior amendments and current canonical report, plus the release/preflight/stable-source identity and prior submission-planning history. They expressly exclude active31's other ten sources/SUMMARY/execution receipts and root metadata. The complete inverse removes only the inserted context tag to restore original Plan31 SHA256 `01966390fd0de2bc2091a9b4587054f3b15b577f933312ec2361046047f93b97`; this new amendment inverts to absence. No prior failed/planning/checker evidence is overwritten.

Canonical prefix is277853bytes SHA256 `976fb471b7284316c836689740b18ac4d9d64a6ea608edd922a39ceb35eae1bc`; the269048-byte prior prefix remains `863fa39a5068d5fe3024ef56e78a7823ec9f2749e345d8000fe0604c4d15bcb0`. Independent checker alone appends. The a889 candidate index/history and earlier d115/scanner blockers remain immutable; this context refinement is a new candidate for targeted independent review.

## Threat and acceptance checks

| Boundary/threat                               | Required mitigation                                                                                          |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Untrusted raw script → signature deletion     | Match only real GetOp boundaries and preserve original retained instruction bytes.                           |
| Malformed syntax → rebuilt script             | Preserve full suffix from saved instruction start; never silently drop partially read data.                  |
| Empty signature → search target               | Use encoded00 and retain zeros inside push payloads/length fields.                                           |
| Helper change → unrelated signature authority | Exact helper/caller/version scope, no flags/checker/sighash/crypto/API expansion.                            |
| Evidence → parity claims                      | Genuine RED/GREEN and source-derived vectors; no actual pinned-process/differential or phase approval claim. |

D-04, D-08, D-09 and D-10 are preserved by the exact transformation tests, refusal/evidence boundaries and final verification. Independent/root source release is required before the single-owner code correction; the executor's other authorized work may continue without a planner-imposed source-tree freeze.

---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T20:09:06Z"
commits: false
---

# Plan23 high-S verification prerequisite amendment

This bounded prerequisite completes the existing actual-coins LOW_S-absence obligation from original06 Task2, retained in23. It changes no locked decision or accepted structural revision history. Root independently checks the amendment before23 source release. The suspected gap is source-discovered only: no actual signed high-S behavioral RED has been captured by this planner, and no runtime failure or implemented fix is claimed.

## Immutable pre-edit scope and identities

Pre-edit23 PLAN SHA256 is `d41c43faab0fb179531b73a23ba2e43a46729efa7458f2933e994fb564cf3797`. Exact plan/coverage and signature source snapshots, absent new-file markers, pinned source and local secp documentation are preserved under `.local/open-bitcoin-dev/phase167/plan23-high-s-amendment-20261021T200638Z/before/`; `snapshot-manifest.json` records all bytes/digests/existence. The accepted167-EXECUTION-SCOPE-AMENDMENT.md, schedule history and every other plan remain untouched.

Original23 metadata remains: execute, wave6, dependencies167-06/167-21, original generated_at2026-10-21T19:39:52Z, CVLD-01/02/03/04, yolo lifecycle167-2026-10-21T16-38-23, commits:false and the original truth. high_s_amended_at records this bounded addition separately. Original single task/action, behavior/acceptance/verify/done and original five owners remain; bounded instructions append rather than delete any original obligation. The earlier five-owner stop rule is superseded only for the four explicit added paths below. One task/nine owners; phase total remains25plans/44tasks/22waves.

## Read-only source finding and evidence class

- Actual owner is `packages/open-bitcoin-consensus/src/signature.rs`, not a script/signature path. TransactionSignatureChecker::verify_ecdsa143–180 parses with the flag-gated verification parser, computes the actual Base/WitnessV0 digest and passes parsed_signature.signature directly to secp.verify_ecdsa. Its verification parser normalizes only to detect/reject high-S when LOW_S is present; absent LOW_S retains the original high-S value.
- The strict public parse_ecdsa_signature remains independently strict and rejects NonLowS; it must not be weakened. Existing signature/tests.rs's high-S cases at151ff prove parsing acceptance/rejection only, not validity for an actual key/digest.
- Pinned `packages/bitcoin-knots/src/pubkey.cpp`277–292 normalizes ECDSA before libsecp verification; `script/interpreter.cpp`200–214 applies encoding/LOW_S gates before the checker path at1675–1680. Public-key/source semantics require policy checks on original bytes before crypto normalization.
- Existing locked secp256k1 is0.31.1. Local ecdsa/mod.rs194–199 documents that verification requires a normalized signature. This is existing dependency documentation, not a new library/API choice.

Pinned pubkey.cpp SHA256: `401015e565c63d8d25ebb553cc6962c6052878e1c7d996a6526c493e276d2273`. Pinned interpreter.cpp SHA256: `6e7367740f8d8ecc8dca4ec129d66782d09fa5733acc96469f367267fb42cc60`. Current signature.rs SHA256: `025c53b212203639e9df68eb3751ae007a0885c57febd10b60b818f438df7063`. Current signature/tests.rs SHA256: `33ff5f365efbbe27b2bc67f0feb6b0593cc39ec13156d1bc734ad6d3709e5e5f`. The existing tests file has560lines; new cohesive controls belong in its child so the registrar stays below629.

These facts support a suspected shared crypto verification gap; they do not replace actual signed RED. Preserve actual outcomes if a focused runtime test already passes. No source mutation is authorized solely to satisfy the source suspicion.

## Exact added ownership

| Added owner | Narrow responsibility |
| --- | --- |
| packages/open-bitcoin-consensus/src/signature.rs | Only if genuine actual signed RED confirms need: normalize a local verification signature after existing encoding/LOW_S checks, immediately before existing crypto verification. Preserve parser/error/export/bytes/digest semantics. |
| packages/open-bitcoin-consensus/src/signature/tests.rs | Register cohesive child high_s.rs; preserve all existing tests/helpers/assertions, stay below629lines. |
| packages/open-bitcoin-consensus/src/signature/tests/high_s.rs | Actual same-key/digest low/high-S verification, explicit LOW_S/bad-digest/signature/public-key controls, and auditable deterministic signed-vector generation/verification using existing consensus test dependency. |
| packages/open-bitcoin-chainstate/src/engine/tests/phase167_policy.rs | Actual canonical funded high-S success versus explicit policy refusal, digest-bound fixed signed vectors and unchanged-state evidence; optionally move existing policy cases intact. |

Original apply.rs,overlay_apply.rs,candidate_rules.rs,phase167_scripts.rs and engine/tests.rs remain owned. The already-owned engine/tests.rs registers the new policy sibling. No additional production registrar, error, helper/export, manifest, dependency or workspace crate owner is needed. Both new test files need pinned-source breadcrumbs;19 alone owns manifest registration. Source guards stay untouched until the existing17 independent review-before-renewal gate.

The prior eight-owner option would have placed all consensus controls in the560-line tests.rs. Root explicitly authorized the cohesive child to avoid cramming; the chosen union is exactly nine, with no threshold waiver or artificial ownership shuffle.

## Executable actual-signed contract

1. Add nonzero-discovery actual TransactionSignatureChecker controls using existing consensus secp0.31.1 signing/test seams. Deterministic low-S and mathematically equivalent high-S signatures for the same key and actual digest both verify when LOW_S is absent, including ordinary DERSIG selection. Explicit LOW_S rejects original high-S as SignatureError::NonLowS before normalization; low-S succeeds. Wrong digest/transaction, invalid signature or wrong key still fails. Preserve strict public parser, original raw bytes and all DER/lax-DER/hash-type/public-key/Base/WitnessV0 behavior.
2. Retain genuine signed RED if present; setup/parser-only failures are not the claimed gap. Only after confirmed cryptographic refusal, normalize a local verification-signature copy immediately before secp.verify_ecdsa, after policy/encoding gates. No mutation of input DER/script/signature bytes, flag weakening, unconditional acceptance, policy duplication or extra authority.
3. Generate and independently verify fixed auditable signature vectors with the existing consensus test signing dependency, bound to the deterministic actual accepted funding outpoint and exact spend/digest. Freeze key/DER+sighash/transaction/amount/script/outpoint/digest facts in the new policy owner, with retained generation/source evidence; recompute and assert actual first-party sighash identity before interpreter/stage execution.
4. Chainstate imports only existing consensus/primitives. No transitive secp import, new dev-dependency or production signing API is allowed. Canonical high-S success uses genuinely accepted matured funding coins, derived flags and the real stage; explicit LOW_S on the same actual funded input/signature fails as existing ScriptError::SigHighS. Include valid low-S/bad-digest/signature controls and shared CHECKSIG/CHECKMULTISIG execution, not a parser-only or unrelated refusal proxy.
5. Preserve accepted tip/hash/full work,best-block/relevant complete coins/undo and no absorption on refusal. Optional MINIMALDATA/CLEANSTACK test relocation preserves exact assertions/inputs and test-name/evidence mapping. Keep all original23 monetary,maturity,creating-MTP,branch,opcode,aggregate and final non-skipped focused/Clippy/full affected regression obligations.

## Decision and threat coverage

| Decision | Plan/task | Coverage | Required evidence |
| --- | --- | --- | --- |
| D-04 |23.1| Full | Consensus absence of LOW_S reaches real cryptographic script execution; explicit policy still rejects original high-S. |
| D-07 |23.1| Full | Real matured funding/canonical stage remains the owner; vectors, parser success and local normalization cannot mint authority. |
| D-08 |23.1| Full | Invalid digest/signature/policy refusals preserve accepted state; normalization never accepts bad bytes. |
| D-09 |23.1| Full | Source suspicion distinguished from genuine signed RED; exact vector/digest/source/log provenance retained. |

Original CVLD-01/02/03/04 and every other23 decision/action remain mandatory. New threat T-167-23-3 covers policy-before-normalization and invalid-crypto acceptance/evidence confusion. Existing threat dispositions,17 review ordering and20 native/fresh coverage/Bazel/canonical source/security/goal/lifecycle acceptance remain. This amendment is not actual pinned-daemon qualification or new Phase168 work.

## Guidance and planning boundary

Existing AGENTS/sidecar/overrides, architecture/code-shape/testing/verification/Rust guidance, both already-loaded active lesson inputs and current CONTEXT/RESEARCH apply. The new test child follows foo.rs plus foo/ and keeps pure checker code separate from test fixture evidence. No repeated discuss/research or lesson append is needed: this is a source-discovered prerequisite, not a human correction.

Planning ownership is only23PLAN, this new amendment, targeted23coverage and ignored snapshots/check results. No source/test/build/Cargo/Bazel/Git, shared STATE/ROADMAP/requirements/config/todo/PLAN-CHECK, accepted structural-history, other plan, active02producer or active21source/test mutation occurred. No actual signed RED/GREEN is claimed. Root must independently accept the precise amendment before23 implementation.

## Actual artifact checks

gsd-tools frontmatter validate returns valid:true with no missing required fields. verify plan-structure returns valid:true,one complete task,zero errors/zero warnings. Direct snapshot comparison confirms unchanged original phase/plan/type,wave6,06+21dependencies,autonomy,requirements,generated_by,lifecycle/generated_at,commits:false and truth; original task name/done are exact, and original files/read-first/behavior/action/acceptance plus all original automated commands remain exact prefixes/content. No original owner is removed. Exactly the four listed paths are added, for nine total, matching task/header ownership; no other current167 plan owns an added path, so no new DAG/wave conflict is introduced.

Both existing signature source files, pinned inspected source and absent new test owners still match the pre-edit snapshot/existence markers. Plain unquoted regex values compile; three changed Markdown artifacts have only the opening/closing frontmatter delimiters and no trailing whitespace. The initial script/signature read alias did not exist; rg resolved the actual consensus/src/signature paths before planning. No source test or behavioral RED was run.

Detailed actual results are in `.local/open-bitcoin-dev/phase167/plan23-high-s-amendment-20261021T200638Z/planner-checks.json`. These are artifact/preservation checks only. The source suspicion and all actual high-S/policy execution evidence remain pending independent planning acceptance and23 execution.


---
phase: 167-candidate-history-validation-rules
plan: "23"
subsystem: consensus-chainstate
tags: [rust, candidate-history, ecdsa, coins, maturity]
requires:
  - phase: 167-06
    provides: Proof-initialized canonical admission and carried actual-coins integration
  - phase: 167-21
    provides: Accepted CLTV/CSV execution and registered funded interpreter boundaries
  - phase: 167-05
    provides: Shared selected transaction context and aggregate sigop budget
provides:
  - Actual canonical flags, rewards, maturity, creating-MTP and branch-lock controls
  - Signed high-S verification after original policy checks without weakening strict parsing
  - Complete non-skipped consensus and chainstate regression evidence
affects: [167-07, 167-15, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Local verification-signature copy, Digest-bound fixed test vectors, Actual staged coins controls]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/signature/tests/high_s.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_policy.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine/apply.rs
    - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
    - packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs
    - packages/open-bitcoin-chainstate/src/engine/tests.rs
    - packages/open-bitcoin-consensus/src/signature.rs
    - packages/open-bitcoin-consensus/src/signature/tests.rs
key-decisions:
  - "Require actual signed RED before correcting high-S verification; normalize only a local copy after original encoding/LOW_S checks."
  - "Keep missing-history kernel evidence distinct from canonical stage acceptance; actual canonical window-bound controls remain separate."
  - "Reuse existing coins/work/checker owners and test signing dependency; no chainstate signing import, dependency or production export."
patterns-established:
  - "Exact fixed funding outpoint and recomputed actual sighash bind canonical signature evidence."
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T20:30:07Z"
commits: false
duration: 15min2s
completed: 2026-10-21
---

# Phase 167 Plan 23: Actual coins, script policy and creating-history Summary

**Canonical staged coins enforce selected script flags, monetary and branch-lock rules; actual signed high-S succeeds through the shared checker while explicit LOW_S rejects original bytes before local normalization.**

## Files Created/Modified

- Carried integration: `packages/open-bitcoin-chainstate/src/engine/apply.rs` — selected transaction context, actual consumed coins, interpreter, pre-spend aggregate sigops, actual fees and reward limit;299 lines.
- Carried integration: `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` — shared selected body/context path;152 lines.
- Carried integration: `packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs` — consumed-only creating-ancestor extension,4096-row refusal and saved-MTP consistency;354 lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs` — preserved canonical controls plus CSV, maturity, actual creating-history, same-block and replacement-branch cases;621 lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests.rs` — existing registrar adds the policy sibling, preserving21's locktime registration;315 lines.
- Modified: `packages/open-bitcoin-consensus/src/signature.rs` — normalize a local verification-signature copy immediately before libsecp verification, after existing parsing/policy checks;328 lines.
- Modified: `packages/open-bitcoin-consensus/src/signature/tests.rs` — only registers the cohesive high_s child;562 lines.
- Created: `packages/open-bitcoin-consensus/src/signature/tests/high_s.rs` — same-key/digest Base/WitnessV0 verification, strict/policy/invalid controls and auditable deterministic vector generation;257 lines.
- Created: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_policy.rs` — fixed digest-bound actual CHECKSIG/CHECKMULTISIG canonical acceptance, explicit policy refusal and preserved original policy case;226 lines.

Exactly nine declared Rust owners: this continuation created two and edited four existing files; the three integration owners were carried unchanged during this continuation. The earlier creation/modification of those carried owners remains attributable to original06, not new work here. All nine are below629 lines. New source breadcrumbs are present;19 alone registers their manifest entries. Final owner hashes are in `.local/open-bitcoin-dev/phase167/plan23/owned-source.sha256` and all nine rechecked successfully after complete regression.

Source checkout reference only, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`. Its existing commit object was checked. No task commit was created.

## Completed task and original obligation mapping

Task1 is complete within checked23 and the bounded high-S amendment. Original06 Task2 is jointly covered by accepted21 and this plan; original06 Task1 remains separately accepted. Frontmatter requirement IDs record this bounded contribution and do not declare phase-wide requirement completion.

| Retained obligation | Actual final evidence | Evidence boundary |
| --- | --- | --- |
| Selected P2SH/DERSIG/CLTV/CSV/NULLDUMMY/WITNESS/TAPROOT reach execution | All seven real matured-coin omission/refusal pairs remain. Single-rule omission succeeds only through the explicit mechanical fixture; canonical caller-NONE execution refuses. CLTV/CSV additionally require the exact mandatory-script rejection and unsatisfied-locktime diagnostic. | Actual canonical stage versus explicitly tainted omission fixture; serialized transaction identities and body commitments remain in use. |
| Canonical active equality, ignored bits and operand-disable/version1 | Both original OP_DROP-dependent equality positives remain; new canonical CSV cases accept mismatched ignored bits and a signed five-byte operand-disable with version1/final input. | Ordinary selected stages, with independent finality/BIP68 prerequisites satisfied. |
| All original interpreter boundaries | Six accepted21 locktime tests are registered and rerun without exclusions. Five-byte signed peek, four-byte existing callers, negative/minimal/overflow, fixed types, disable/version/mask/checker order, inactive NOPs and skipped branches remain unchanged. | Actual funded interpreter probes; no canonical acceptance inferred from a probe. |
| Actual80000/80001 aggregate budget |79996 legacy plus4 real witness sigops accepts; plus5 refuses specifically as bad-blk-sigops, with complete snapshot and coins best-block unchanged. | Actual staged coin inputs, budget checked before spending. |
|149/150/299/300 subsidy plus actual fees | Original reward case remains unchanged and reruns: four independent matured outputs each pay10000sat; selected subsidy+fee accepts and+1sat refuses as bad-cb-amount. | Actual canonical coins stage; historical rewards.log remains a separate1/1 outcome. |
| Maturity100 versus99 and zero genesis outputs | Proved genesis snapshot has no UTXOs; height1 coinbase spend in candidate100 refuses as bad-txns-premature-spend-of-coinbase; candidate101 accepts after the intervening block. | Actual canonical stage, complete tip/work/coins/undo and best-block preservation on refusal. |
| LOW_S absence and explicit policy rejection | Both deterministic CHECKSIG and1-of-1 CHECKMULTISIG actual matured funding inputs accept low/high-S canonically. Exact funding txid and recomputed first-party SIGHASH_ALL digest match frozen vectors before execution. The same high-S input under explicit LOW_S returns SigHighS; low-S succeeds. Changed output/digest refuses canonical staging and leaves accepted state intact. | Real canonical stage and public actual-input policy verifier; no chainstate signer/import or new dependency. |
| MINIMALDATA/CLEANSTACK separation | Original phase167_scripts_policy_minimaldata_cleanstack_remain_absent_on_canonical_path moved verbatim to the policy sibling, preserving name, inputs and every assertion. Added explicit nonminimal CLTV numeric operand refusal and extra-stack refusal use existing VerifyFailed/WitnessCleanStack errors. | Canonical policy absence plus actual funded interpreter checks. Numeric minimality is not a new push-policy implementation. |
| Consumed-only distant creating-MTP | Actual height1 coin spend beyond155-header suffix succeeds using branch-derived creating MTP. Missing distant height0 context specifically refuses the real coins kernel as MissingAncestor; version1, input-disable and height-type controls succeed without demanding that history. | Missing-history case is a kernel probe over actual accepted coins and private overlays; it is not canonical acceptance. Complete accepted snapshot remains unchanged. |
| Inconsistent metadata and finite4096 history | Saved creating MTP+1 refuses an actual canonical body as creating-ancestor-mtp-mismatch. Parent4095 needs4096 linked rows and succeeds; parent4096 needs4097 and refuses as creating-ancestor-window-bound. Disabled-input control still accepts beyond the bound. | Separate actual canonical stage controls. The finite bound is an implementation refusal, not a claimed network consensus rule. |
| Same-block and replacement branch | Same-block time-lock0 accepts and stores exact candidate-parent MTP/height; time-lock1 refuses as non-BIP68-final. Two six-block branches disagree at7*512seconds: replacing long-time active history with short-time branch refuses, while the inverse replacement accepts. | Actual canonical staged connects/reorgs; replacement-parent facts win over active-tip facts. Full accepted snapshot/best-block remain unchanged on refusal. |
| Original context/stage/clock protection | Ten context and four selected-genesis tests rerun within final40-case discovery. No admission/work owner changed. | Accepted06 provenance, original clock and full-width work protections retained. |

All snapshot comparisons include the entire active chain with header/hash/full ProofWork, complete relevant coins and undo map. Refused body paths return no usable stage to absorb; canonical refusals additionally assert unchanged coins best-block. Private policy/kernel probes never acquire absorption authority.

## Actual signed RED and correction

Pinned pubkey.cpp277–292 normalizes ECDSA before cryptographic verification; interpreter.cpp200–214 applies encoding/LOW_S gates first. Local secp256k1-0.31.1 documentation states verification requires normalization. Those source facts initially established suspicion only.

high-s-red.log then discovered two tests: the generator passed and actual verification failed1/1 at valid high-S Base with left Ok(false), right Ok(true), after low-S and explicit NonLowS controls passed. The generator proves high-S normalizes back to the same signed low-S value and independently verifies it with libsecp for the actual digest. This is genuine signed cryptographic RED, not parser-only acceptance or a fixture setup failure.

Only after that result, signature.rs gained a local copy, normalize_s, and verification of that copy. Public strict parse_ecdsa_signature still rejects NonLowS; private flag-gated policy still examines original bytes before normalization. Original DER, hash-type, public-key, Base/WitnessV0 sighash and signature/raw-script removal behavior is unchanged. CHECKSIG and CHECKMULTISIG share this existing checker. Final tests require high-S acceptance under NONE/DERSIG, low-S acceptance under LOW_S, explicit NonLowS, strict public parsing, wrong key, changed transaction/digest and a signature from the wrong digest to refuse. Original encoded bytes remain equal after verification.

The fixed test key is explicitly [9;32], with height1 canonical coinbase script [OP_1,OP_1],100000sat output0, version2/final-sequence spend,90000sat OP_1 output, lock_time0 and SIGHASH_ALL. Exact public key, funding txid, digest and DER+sighash byte are frozen in phase167_policy.rs. Full generation bytes remain in high-s-red.log/high-s-final.log. No runtime signing API or new crate/dependency was introduced.

## Final command outcomes

All Cargo work used the pinned Bun1.3.9 PATH and scripts/command-timings.ts serially with existing targets/cooperative lock. No Cargo/Bazel job overlapped, target was copied, process was killed or hook setting changed. Complete logs are under `.local/open-bitcoin-dev/phase167/plan23/`.

| Actual command/check | Outcome | Receipt |
| --- | --- | --- |
| timing key phase167-23-focus; cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_ | Final40/40 passed:10 context,6 locktime,16 scripts,4 policy,4 selected-genesis;0 ignored/0 skipped.379 unrelated unit cases filtered; separate parity binary0 matching/3 filtered is not counted. Runtime87.21s. | focus-final.log |
| timing key phase167-23-high-s; cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_signature_high_s -- --nocapture |2/2 passed; actual named discovery,0 ignored/0 skipped. Zero-match filtered integration binaries are not counted. | high-s-final.log |
| timing key phase167-23-clippy; cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings | Exit0, fresh strict affected Clippy;9.27s. | clippy.log |
| timing key phase167-23-regression; cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-features | Exit0:419 chainstate+336 consensus unit,16 integration,27 doctests =798 passed. Every result has0 failed/ignored/filtered; no skips/exclusions. Chainstate unit runtime90.31s. | regression.log |
| Scoped rustfmt --edition2024 --config skip_children=true --check on all nine owners | Exit0; no recursive foreign-child formatting. | rustfmt.log |
| Scoped git diff --check and owned diff/new-file review | Exit0; small shared checker correction, preserved21 source and carried integration reviewed. | diff-check.log; high-s-correction.diff; owned-tracked-integration.diff |
| gsd-tools verify key-links for23 PLAN | all_verified:true;4/4 executable links through actual transaction/context/checker/public verifier calls. | key-links.json |
| Final owner hash check and stub scan | All9 frozen owner hashes match after complete regression. Stub scan exits1 for no matches; no unresolved production/UI stubs. | owned-source.sha256; stub-scan.log |

Source digest/source excerpts are retained in source-digests.log, source-pubkey-verification.log, source-policy-order.log and source-script-checker.log. No actual Knots daemon was run; these are local source-derived and ordinary first-party execution results, not differential qualification.

## All failed attempts and retained history

| Current receipt | Actual interpretation |
| --- | --- |
| high-s-red.log |2 discovered: generator passed, actual valid-high-S verification failed as Ok(false). Genuine signed RED authorizing only the bounded checker correction. |
| high-s-green.log |2/2 passed after the local-copy correction, before later descriptive test-name alignment to the prescribed phase167_signature_high_s filter. |
| policy-first.log | Compile/setup error: missing transaction_txid import in the new policy module. Not a behavioral RED. |
| policy-second.log | Corrected import;2/2 canonical signature cases passed. A transient artifact-directory lock resolved normally;29.93s total build setup, no process killed. |
| focus-first.log |38 discovered,37 passed/1 failed. Truncating internal active_chain invalidated earned work lineage first, producing UntrustedWorkLineage. This does not establish MTP-specific RED. Corrected the new missing-history probe to exercise the actual coins kernel with a complete header suffix and missing consumed ancestor; its evidence class remains kernel-only. |
| focus-second.log | Compile/setup error: nested owned error moved in a Rust pattern guard. Corrected to borrow source.as_ref(). No production change. |
| focus-third.log |40 discovered,38 passed/2 failed. The proposed equality fixture requested4097 rows at parent4096; production correctly refused its4096 bound. Corrected equality to parent4095 and refusal to4096. The new explicit MINIMALDATA push-only expectation returned Ok: that overreached the existing numeric-minimality scope. Revised it to the already-supported nonminimal CLTV numeric operand; no unrelated push-policy implementation or original test change. |
| focus.log |40/40 passed after those setup/expectation refinements. Final exact sigop and missing-ancestor assertions were then strengthened and all40 rerun in focus-final.log. |

Two read-only exploratory aliases, context/candidate.rs and chainstate/src/snapshot.rs, did not exist; actual definitions were located in context.rs and types.rs. No file changed from those reads. No authentication gate occurred. All failures above retain their original complete logs; none is relabelled as a pass or an attribution-only RED.

Original06 checkpoint/summary and accepted21 summary remain authoritative for prior history: task1-red import/setup errors; genuine missing-admission RED; incorrect initial MTP vector and corrected12-versus11 inverse; canonical small-height coinbase setup corrections; task2-red import error; seven genuine canonical flag/policy/budget gaps; initial CLTV/CSV omission controls failing UnsupportedOpcode before valid bypass evidence; original Clippy failure/fix;400unit+3integration+23doctest partial regression and21focused with exactly two explicit skips; separate genuine missing-opcode positive RED2/2;11/11 initial canonical/script GREEN;15-case boundary run with one inherited-scriptSig setup mistake followed by15/15; and separate rewards.log1/1. The15/15 and1/1 were never a historical combined16-case run. No old log was overwritten or replayed solely for attribution. Current full798-case regression is fresh and has no skipped cases.

## Guidance, simplification and threat coverage

AGENTS.md Repo-Local Guidance, Bright Builds sidecar/overrides, standards index, architecture/code-shape/testing/verification/local-guidance and Rust/TypeScript rules informed execution. Both distinct active lesson files were measured/read completely:6898+1958=8856bytes,2300+653=2953 conservative tokens; the recent audit baseline remains applicable. GSD execute-plan and summary-template contracts apply with the explicit commits:false/root-owned shared-state exception.

Simplification retained one coins/work authority, existing CandidateHistory/transaction interpreter, shared sigop budget and shared ECDSA checker. Cohesive child tests avoid growing the existing562-line signature test registrar. The original policy case moved intact to keep canonical scripts at621 lines, without another owner or threshold waiver. No foreign source, guard, breadcrumb manifest, source corpus or Plan02 evidence changed.

T-167-23-1 is covered by actual selected-stage, consumed-history and unchanged-state controls;T-167-23-2 by full failure attribution, final nonzero discovery and complete non-skipped checks;T-167-23-3 by genuine signed RED, original-byte policy rejection and wrong-key/digest/signature controls. No new network endpoint, auth path, filesystem access boundary, durable schema or independent authority surface was introduced. No unresolved stub prevents this plan's goal.

## Deviations and remaining phase work

No ownership, architecture or dependency expansion occurred. The proved high-S correction is the checked amendment's conditional planned work. New test setup/expectation refinements are recorded above. Missing-history kernel evidence is explicitly narrower than canonical acceptance and is paired with the separate required canonical window-bound and mismatch controls; no lineage refusal substitutes for MTP evidence.

19 retains new breadcrumb registration and relevant README/parity updates;17 retains source review before guard renewal;20 retains full default bash scripts/verify.sh, fresh pure-core coverage/Bazel and independent source/security/goal/lifecycle acceptance. This result does not accept durable runtime/physical reopen/historical exception/actual-daemon obligations owned by later plans. The finite creating-history refusal remains a deliberate implementation limit. No Phase168 work was started.

commits:false. No staging, commit, push, worktree, hook bypass, shared STATE/ROADMAP/config/requirements/todo/AGENTS edit or guard/breadcrumb ownership change occurred. Root owns shared progress and normal-hook saving after final phase acceptance. Stop before07; root alone releases it after accepting23.

## Performance attribution

One completed task; nine declared Rust owners, six edited/created during this continuation and three carried unchanged. Recorded continuation start2026-10-21T20:15:05Z precedes source edits; complete implementation/check observation2026-10-21T20:30:07Z gives902seconds(15min2s), excluding earlier context loading and later summary/receipt preparation. generated_at records that final-check observation. This is separate from original06's approximately29 combined minutes beginning19:05:48Z and from accepted21's measured continuation; no historical interval is charged again.

Summary validation returned passed:true/errors:[] with9/9 declared paths found and self_check:passed; frontmatter returned valid:true/missing:[]. Receipts are verify-summary.json and frontmatter.json in plan23/. The validator's commits_exist:true resolves the explicitly labelled pre-existing checkout object, not a newly created task commit.

## Self-Check: PASSED

All nine declared Rust owners and this summary exist. Final40/40 focused,2/2 signed high-S,798/798 unfiltered affected regression, strict affected Clippy, scoped formatting/whitespace, owner hash preservation and all4/4 executable links pass. The labelled existing checkout commit object exists; no task commit is asserted. Root-owned metadata and other owners remain outside this executor's edits.

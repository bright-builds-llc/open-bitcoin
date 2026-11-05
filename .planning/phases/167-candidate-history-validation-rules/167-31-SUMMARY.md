---
phase: 167-candidate-history-validation-rules
plan: "31"
subsystem: consensus-chainstate
tags: [rust, serialized-scripts, segwit, tapscript, coins, basic-filters]
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
requires:
  - phase: 167-12
    provides: Accepted canonical history and consuming chainstate owners
provides:
  - Explicit lossless script byte storage alongside the unchanged bounded constructor
  - Version-specific execution size/opcode limits and raw witness/redeem reconstruction
  - GetOp-boundary signature deletion with exact malformed suffix and empty-signature handling
  - Pinned Coins exclusion with independent raw BASIC and sigop evidence
affects: [167-32, 167-33, 167-30, 167-13, 167-17, 167-19, 167-20, 167-25]
tech-stack:
  added: []
  patterns: [Byte storage without execution authority, shared version guards, distinct Coins/filter/counter rules]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/script/serialized_tests.rs
    - packages/open-bitcoin-chainstate/src/engine/apply/serialized_tests.rs
  modified:
    - packages/open-bitcoin-primitives/src/script.rs
    - packages/open-bitcoin-consensus/src/script.rs
    - packages/open-bitcoin-consensus/src/script/legacy.rs
    - packages/open-bitcoin-consensus/src/script/witness.rs
    - packages/open-bitcoin-consensus/src/script/taproot.rs
    - packages/open-bitcoin-consensus/src/script/encoding.rs
    - packages/open-bitcoin-consensus/src/classify.rs
    - packages/open-bitcoin-consensus/src/script/sigops.rs
    - packages/open-bitcoin-chainstate/src/engine/apply.rs
key-decisions:
  - Preserve bounded ScriptBuf construction and Amount; serialized bytes confer no execution proof.
  - Apply BASE/default and WitnessV0 limits at evaluation; retain distinct Tapscript OP_SUCCESS semantics.
  - Keep Coins exclusion separate from BASIC element selection and observational sigop counting.
  - Apply the independently checked same-owner FindAndDelete refinement without expanding signature semantics.
patterns-established:
  - Exact original instruction bytes and complete malformed suffix survive observational transformation.
requirements-completed: [CVLD-01, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T10:27:04Z"
commits: false
duration: "39m 56.096s across measured execution intervals; review hold excluded"
completed: 2026-10-22
---

# Phase 167 Plan 31: Serialized scripts, execution limits and Coins semantics Summary

**Lossless script storage now feeds version-correct execution, GetOp-boundary signature deletion and the pinned Coins rule while retaining exact BASIC and legacy-sigop facts.**

All three tasks are complete across eleven source owners. The requirements field records this plan's bounded contribution; global requirements and phase acceptance remain with root. No staging, commit, push, shared state/roadmap/requirements/metrics edit or guard renewal occurred. The baseline commit above is existing history, not a task commit.

## Files Created/Modified

These are the exact final source owners. The other ten sources match the independently reviewed interim freeze; encoding includes only the checked refinement and coordinated test-typing repair. All sources are normally formatted and below629 physical lines; maximum551.

| Source                                                                  | Role                                                                   | Lines | SHA256                                                           |
| ----------------------------------------------------------------------- | ---------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `packages/open-bitcoin-primitives/src/script.rs`                        | Explicit serialized constructor; bounded body and old tests retained   |   136 | 1f4ebc6af8ed0be38e9b97a5bfd68a53f7ff04d5a4f1b51e868393f26938de58 |
| `packages/open-bitcoin-consensus/src/script.rs`                         | ScriptSize/display and conventional test child                         |   163 | 857782f6b81362b0353f9c66521424f40dd5375d243cd961829f2fa6ace942fc |
| `packages/open-bitcoin-consensus/src/script/legacy.rs`                  | Shared BASE/default/v0 size and opcode guards                          |   545 | 759304a67fa4b0107029c0199cc2e31f196bfda81d0ffa3e42272cdf6cdd3cc0 |
| `packages/open-bitcoin-consensus/src/script/serialized_tests.rs`        | Actual version boundary controls and test fixtures                     |   176 | 571dd4dba177d9582a1c6ba90b38bf0eebf6f066b82b4e4f4c9f12712615ae75 |
| `packages/open-bitcoin-consensus/src/script/witness.rs`                 | Raw P2WSH reconstruction and real v0 controls                          |   383 | 2a019eb41fd79db73f89431da3ff17c64dd7f3cf3bbd531e4e750289bdd4abcf |
| `packages/open-bitcoin-consensus/src/script/taproot.rs`                 | Raw committed script and syntax-only OP_SUCCESS scan                   |   523 | b1a529274989881b381f32f3ddb170f1935fbf3a094654aabe588bbea3c50503 |
| `packages/open-bitcoin-consensus/src/script/encoding.rs`                | Raw rebuild and exact FindAndDelete transformation                     |   375 | b9746d6d1354713b56fc8e1f1e550bafbe0e1ee2aefc73c9616956fb95afcf55 |
| `packages/open-bitcoin-consensus/src/classify.rs`                       | Raw redeem extraction                                                  |   551 | 07a88b1417e53f4f12c5bb35052e21406968a9cce30c7a8577e3b4df6167974b |
| `packages/open-bitcoin-consensus/src/script/sigops.rs`                  | Raw nested classification and syntax-only counter controls             |   165 | d75bc0876104aa325bef718a00335cdf2315d4e029183693e1d0ba4e010be38c |
| `packages/open-bitcoin-chainstate/src/engine/apply.rs`                  | Shared OP_RETURN-or-wide Coins predicate                               |   314 | 9c69d4219ad6b8ded20dea3ebedc03cc292c3fc492f50b6546515dc280d00090 |
| `packages/open-bitcoin-chainstate/src/engine/apply/serialized_tests.rs` | Canonical staging/consumption/disconnect and seeded overwrite evidence |   543 | 5f6268efcd4b8fab261bdce742a0b8e0b977e208d1d5cccf5db1f5e6928e5a34 |

## Accomplishments

Task1 adds documented ScriptBuf::from_serialized_bytes(Vec<u8>)->Self without changing the bounded from_bytes body, its error or existing primitive tests. Actual evaluation rejects10001 bytes before opcode errors for BASE/default and WitnessV0. Literal10000/10001 and201/202 controls distinguish execution versions. Tapscript has no10000-byte or201-op cap, while ordinary element/stack limits remain. ScriptSize displays the pin's exact new-error spelling, “Script is too big.”

Task2 carries raw bytes through P2WSH, committed Tapscript, redeem extraction, nested witness classification and signature rebuild. A real oversized P2WSH reaches ScriptSize instead of a manufactured witness mismatch. The syntax-only OP_SUCCESS scan handles large pushes, malformed-before-success, success-before-malformed, pushed success bytes and policy refusal. Committed wide scripts execute; commitment, normal initial stack/element, signature and49/50 validation-weight controls remain active. Fixed generated P2WPKH construction remains bounded.

The checked FindAndDelete refinement changes only encoding.rs. Existing encode_push_data supplies the exact target, including empty signature->OP_0. Removal occurs at initial/successful GetOp boundaries, repeats at the same boundary, copies retained original instructions and stops with the full original malformed suffix. Existing read_instruction_syntax supplies parsing without520-byte execution limits. BASE CHECKSIG/multisig callers, WitnessV0 bypass and Tapscript dispatch remain unchanged. Six named tests contain the full13-case literal preflight matrix, three nonminimal push encodings, one complete521-byte push control, twelve malformed-tail arrangements, sequential removal and empty-signature length-field controls. These subcases are not additional named tests or cryptographic spend evidence.

Task3 changes only the shared Coins predicate to OP_RETURN first byte OR length>10000. Actual selected canonical staging, consuming absorption and disconnect compare complete prior/next Coins, tip/work, undo and occurrence counts. The matrix retains10000-byte, empty and ordinary outputs, excludes10001-byte and OP_RETURN outputs, and proves the excluded outpoint cannot be spent. A genuine100-block canonical prefix and mature spend prove complete spend undo and restoration without creating a wide Coin. Both independently seeded real BIP30 repeat contexts preserve complete prior coins/undo/counts through actual stage/consume/disconnect; their evidence class and intentionally stronger historical restoration remain unchanged.

The same staged wide body retains its exact script/txid and BASIC/sigop facts. Independent literal wire serialization gives raw txid aa4dab1642c41c5d4d0ab969ca9529308ceb4b2cfce445772a6ab2aea8ca41c2. Bun orchestration of the existing pinned Python SipHash/source-derived BASIC oracle gives fixed filter bytes0373e6208ae2a45488, count3. Expected per-output legacy sigops are1,1,0,1,0, including the OP_RETURN script's syntactic CHECKSIG. BASIC, Coins and counters therefore retain their distinct pin rules.

## Authorization and Scope

Original root release cf71cfd12e0d1b54ef4fc9d703374970ed69eaee7729692e09bd7fd2e6102143 authorized the eleven owners. Original Plan31 SHA01966390fd0de2bc2091a9b4587054f3b15b577f933312ec2361046047f93b97 remains preserved. The effective plan is417ec1cdd6d2f4918bad1e011bb408ef46509519aa844f92c909aa88bef7840d; the same-owner amendment is60d9eee03ec1dd15338453b2ab4e62317fda1c409230a4197b267d7568a60d95 and encoding continuation release is e4762d6e04eb9d720f994a2a1c4b5b58532f54872b863a4a799b7b91297066ab. Original task XML/frontmatter/commands remain unchanged.

The ten-source review freeze d3da844e57288589b9ca39753f15a1abb6b277be145aaa7dd8247838b5f93402 remains exact. The final eleven-source v2 freeze is0a02ab08feee9652b15e4fe5db66f958ee4d7687559f40fd21d07f671416866c. The superseded373-line encoding freeze57d46fe0 and its Clippy failure remain attributable. The alias-only repair's full inverse restores that complete file; the entire production prefix remains byte-exact, SHA c3bdfdfda80d70276dcfd92068ff9848cefdaa43521f2e84eafee5895829c66a.

AGENTS.md, its sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed the pure boundaries, AAA tests, maybe names, source caps and serial verification. Active lessons were fully loaded:8856 bytes/2953 conservative estimated tokens, no archives. Existing audit baseline required no new audit. GSD init and the execute-plan workflow were loaded; root's commits:false/shared-metadata ownership overrides generic executor saving/state updates.

## Commands and Results

All Cargo jobs used pinned Bun1.3.9/Rust1.94.1, physical process-local TMPDIR, the existing target and command-timings cooperative ownership. No Cargo/Bazel overlap, target copy, new dependency, deadline increase or quiet-command termination occurred. Polling/liveness observations distinguish tool yields from timeouts. Available disk was checked before jobs, approximately18–20GiB.

Final closure is `.local/open-bitcoin-dev/phase167/plan31/execution/final-closure-2026-10-22T10-23-19.495Z.json`, SHA5f0cf7283ff6836c0df097a1e57a6b893f6701697f2f8520499445b57b550ed9, passed:true/errors:[]. It binds every final command's before/after eleven hashes to the v2 freeze. All final cases pass with zero ignored cases. Auxiliary zero-match integration harnesses are not claimed discovery.

| Final selection/key                                            |              Harness passes | Exit | Wrapper seconds | Raw log SHA256                                                   |
| -------------------------------------------------------------- | --------------------------: | ---: | --------------: | ---------------------------------------------------------------- |
| phase167-31-version / phase167_serialized_script_version       |                           6 |    0 |           8.582 | f18d3b84a967f1321a1fcf32c1d8498578f07f06b7a6099877f7cc71f68b82ac |
| phase167-31-primitives / all features                          |                 23unit+1doc |    0 |          56.492 | 69319eeafa75dc78891265c1fc0247da04f37c93aa267312a5c14ff343c300f3 |
| phase167-31-paths / phase167_serialized_script_paths           |                          22 |    0 |           0.411 | 3b09b63765acf8266797d054b72db15af9241fae4c3ee35bc85fd9995bebe1b0 |
| phase167-31-script-regressions / script                        |         98unit+2integration |    0 |           0.205 | f418ba5f5af454615707d31effb010ac8e8da1996342f7b35dafa8405393ac54 |
| phase167-31-coins / phase167_serialized_script_coins           |                           5 |    0 |           7.488 | 459f759280c538424c3965097a3819cb507355322542f6841f4ba6c27a1a61cd |
| phase167-31-clippy / strict three-crate all-target/all-feature |                         n/a |    0 |          11.109 | d174830caaa81f91a651676ea528bfcf21cee908cd462a03df67d01133dccbdb |
| phase167-31-chainstate-regressions / all features              | 439unit+3integration+23docs |    0 |         175.672 | da03d84f0f6b7fda1c0a853e3217fe31607b08b2291099b244357a50c31839cb |
| scoped rustfmt check / final-format-v2                         |                         n/a |    0 |           0.046 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
| scoped git diff --check / final-diff-check                     |                         n/a |    0 |           0.016 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |

Exact original commands were rerun on stable final source:

```bash
bun run scripts/command-timings.ts run --key phase167-31-version -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_serialized_script_version -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-31-primitives -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-primitives --all-features -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-31-paths -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_serialized_script_paths -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-31-script-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus script -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-31-coins -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_serialized_script_coins -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-31-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-primitives -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings
```

Additional full regression used bun run scripts/command-timings.ts run --key phase167-31-chainstate-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --all-features -- --test-threads=1.

Complete runner history is `.local/open-bitcoin-dev/phase167/plan31/execution/evidence-index-2026-10-22T10-23-19.581Z.json`, SHA2c685020447ab96994b8dcabd53632869db7ee943f8db2de9223258b5202865e:31 receipts with full argv/environment basis, source-before/after, raw-log SHA and UTC elapsed time. All28 Cargo jobs and three static runner checks remain recorded. The unspawned initial setup attempt is separately indexed. Harness totals are authoritative; old named-line extraction misses rustdoc spaces/timing-interleaved lines. In particular final full-chainstate has463 parsed named lines but465 actual harness passes. Overlapping selections are not distinct coverage.

### Preserved command chronology

The table includes every completed runner command, with UTC start time; P/F counts are actual harness totals. Compiler/static commands have no test discovery.

| UTC start    | Label                                           | Exit | P/F                     | Seconds |
| ------------ | ----------------------------------------------- | ---: | ----------------------- | ------: |
| 09:40:52.392 | task1-red-retry                                 |  101 | 3/3                     |  40.515 |
| 09:41:47.235 | task1-green                                     |    0 | 6/0                     |  50.301 |
| 09:44:11.633 | task2-red                                       |  101 | 10/6                    |   8.010 |
| 09:45:08.536 | task2-green                                     |    0 | 16/0                    |  10.142 |
| 09:48:27.336 | task3-red                                       |  101 | compiler setup          |  11.185 |
| 09:49:04.000 | task3-red-import-corrected                      |  101 | 2/2                     |  12.942 |
| 09:50:23.244 | task3-red-complete-controls                     |  101 | 2/3                     |   4.519 |
| 09:51:11.183 | task3-green                                     |    0 | 5/0                     |   8.417 |
| 09:52:01.276 | first-clippy                                    |    0 | n/a                     |  17.629 |
| 09:52:27.499 | final-primitives (interim source)               |    0 | 24/0                    |  21.862 |
| 09:54:23.731 | interim-script-regressions                      |    0 | 94/0                    |   0.507 |
| 09:54:36.691 | task3-strengthened-green                        |    0 | 5/0                     |   4.670 |
| 09:54:58.744 | interim-chainstate-regressions                  |    0 | 465/0                   | 175.307 |
| 10:13:04.257 | find-delete-red                                 |  101 | 0/6                     |   8.337 |
| 10:13:40.685 | find-delete-green                               |    0 | 6/0                     |  41.060 |
| 10:14:49.171 | final-version (373-line encoding source)        |    0 | 6/0                     |   0.434 |
| 10:14:49.623 | final-primitives-complete (same interim source) |    0 | 24/0                    |  11.641 |
| 10:15:01.307 | final-paths (same interim source)               |    0 | 22/0                    |   0.430 |
| 10:15:01.754 | final-script-regressions (same interim source)  |    0 | 100/0                   |   0.205 |
| 10:15:01.976 | final-coins (same interim source)               |    0 | 5/0                     |   8.231 |
| 10:15:10.251 | final-clippy (373-line encoding source)         |  101 | type-complexity warning |  50.969 |
| 10:15:32.792 | final-format (373-line encoding source)         |    0 | n/a                     |   0.065 |
| 10:18:43.133 | final-version                                   |    0 | 6/0                     |   8.582 |
| 10:18:51.764 | final-primitives-complete                       |    0 | 24/0                    |  56.492 |
| 10:19:48.298 | final-paths                                     |    0 | 22/0                    |   0.411 |
| 10:19:48.726 | final-script-regressions                        |    0 | 100/0                   |   0.205 |
| 10:19:48.949 | final-coins                                     |    0 | 5/0                     |   7.488 |
| 10:19:56.476 | final-clippy                                    |    0 | n/a                     |  11.109 |
| 10:20:11.425 | final-diff-check                                |    0 | n/a                     |   0.016 |
| 10:20:11.425 | final-format-v2                                 |    0 | n/a                     |   0.046 |
| 10:20:18.004 | final-chainstate-regressions                    |    0 | 465/0                   | 175.672 |

At09:40:30.985 the initial ignored runner setup exited1 before spawning Cargo: Bun1.3.9 rejected stderr:"stdout". Its started argv/environment/source receipt and explicit setup note remain. It is not behavioral RED. The corrected runner uses one file descriptor for both output streams.

Task1 RED exposes missing size/error precedence and Tapscript's shared op cap. Task2 RED exposes raw redeem/rebuild/nested/witness mismatches and execution-limited success scanning. Task3's first compiler failure was an incorrect test import of BlockOverwriteUndo from the crate root; corrected to the actual types module before genuine RED. Its expanded RED exposes wide Coins creation, missing-input behavior and mature-spend next-state differences. FindAndDelete RED executes all13 literal observations; nine differ, and all six named controls fail. No setup/compiler error is counted as behavioral RED.

The first final-source sequence's strict Clippy101 identified only the new test table's complex tuple annotation. Root coordinated a local test-only LiteralCase alias; no suppression, test value/assertion/count or production byte changed. Full forward/inverse patches and typing-proof.json retain that exact transition. All original commands were repeated after the v2 freeze. The earlier full-chainstate result remains interim because a maybe-name local was renamed during its source fence; a later test-only type import simplification preceded the ten-source freeze. The final full suite has stable v2 fences.

Document QA first ran the installed mdformat with explicit gfm/frontmatter extensions in check mode and exited1 for formatting. The complete pre-format SUMMARY is retained under execution/summary-before-format.md, original SHA f783d611438b58478ea57586b37b6998ee5e391045c3e5ce2544deff0c5852c4. A scoped formatter write and repeat check exited0; summary-format-first.json retains the first command/result. This changed no source.

## Performance and Evidence

- Tasks:3; files:11 (nine modified, two created).
- Measured intervals:09:38:02Z–10:01:17.096Z and10:11:38Z–2026-10-22T10:28:19Z; total39m 56.096s through successful document QA.
- Review/source hold:10:01:17.096Z–10:11:38Z,620.904s, excluded above.
- Initial startup before the first explicit UTC sample is unmeasured and excluded.
- Recorded command intervals sum747.399s; final accepted-source command intervals sum260.021s. These are execution time totals, not distinct test coverage.
- Detailed interval basis is in execution-intervals.json under the ignored execution directory.
- Final BASIC oracle result SHA f2c2c6e1073d958e93b846d81110333d0ba2b01e5f9de415188e8ceada53a974; typing proof SHA7f48a0b2a867006e89808cebd7f89ade03c38566cc5f8256ce329707b29658db.
- Final source audit source-audit-2026-10-22T10-23-19.549Z.json SHA02535ff85dc0f73a1ef6ba4036ef731c9c3fbc581206bca1676f18bc7afbd112 passes/errors:[].

## Decisions Made

Storage and execution validity remain separate. One version predicate gates legacy execution limits; the existing syntax reader handles both observational counting and Tapscript success scanning. Coins retains one shared predicate consumed by connection, disconnection and overwrite checks. No replacement parser, dependency, new authority switch or extra source owner was introduced.

The explicit simplification pass kept these small production changes and moved substantive coverage into the two declared children or owner-local test modules. The FindAndDelete change reuses syntax decoding and original-byte slices; the Clippy repair names only the repeated literal tuple shape.

## Deviations from Plan

The pre-existing FindAndDelete mismatch was held for root's independent planning check, then resolved through the formal same-owner amendment. It expressly supersedes only the enumerated small-input transformation cases; original task/command contracts and other behavior remain. The routine test-import and Clippy typing repairs stayed inside declared owners. No architectural, ownership, size-budget, verification-budget or evidence-class expansion occurred.

## Protected Inputs and Residual Limits

Amount remains byte-exact SHA b174241ff27077d71a71b0eecab728839b84feb756edaef0dcd5c1e163836f35. The original bounded constructor body and primitive tests are exact; its error behavior remains. Root before receipt7dbc5c69dc988e93fb9a71a961f033a917c5d6a78242e48e13a278b08d41b6da, all nine original source copies and two absent-child facts remain intact. HEAD is the original baseline and the index is empty. The pinned Knots commit and actual breadcrumb targets were checked.

New/touched sources carry pinned breadcrumbs; this scoped check verifies comments/target existence and source hashes. Plan19 alone reconciles the central breadcrumb manifest. Coins/undo bounded DTOs, BASIC implementation, Amount, BIP30 corpus/evidence classes and all unowned source owners received no edits from this executor. Root retains its broader independent checkout protection check.

The stub scan found no goal-blocking placeholder or unwired production path. New raw representation/interpreter/Coins transformation surfaces are covered by the plan and checked amendment threat model; no additional network/auth/session/file-I/O surface was introduced.

This is local pinned-source and actual Rust execution evidence. No actual Knots process/differential qualification, P2P-wide-script ingress, whole-chain acceptance, production/funds readiness or phase approval is claimed. CONST_SCRIPTCODE/removal-count, CODESEPARATOR, sighash/checker, NULLFAIL, flags and crypto remain outside the refinement. Existing full native, fresh coverage, Bazel, source/security/goal/lifecycle/compact and normal-hook saving obligations remain with later phase gates/root. No authentication gate or external setup was needed.

## Task Commits

None; commits:false is explicit. Root owns shared metadata, metrics and later normal-hook saving after all phase gates.

## Self-Check: PASSED

All eleven final source paths exist and match the v2 freeze. All final command fences and raw-log hashes agree with the closure. The original baseline commit exists, protected inputs agree, scoped formatting/diff checks pass, and all required final selections complete with zero ignored cases. Three tasks and eleven owners are accounted for.

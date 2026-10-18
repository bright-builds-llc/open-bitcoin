---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T03:47:31Z"
depth: standard
preliminary: true
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T03:47:31Z"
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_reviewed: 6
files_reviewed_list:
  - packages/open-bitcoin-node/src/sync/live_fixture.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
scope_sha256: c35e18a4c7478252a1c77f3dd394ee2e740529ef64ee49e382eaffa628ff0deb
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 165: Preliminary Fixture API Source Review

**Status:** clean within this preliminary source scope. No actionable bug or security finding was identified. This is not final phase acceptance.

## Scope and method

Reviewed the six listed files, including the actual tracked diff from `d0104a99` and the new untracked implementation/unit/external files. Cross-referenced only the existing origin/binding, recovered BASIC owner, fixture header admission, and checked/cached candidate interfaces needed to evaluate the new constructor. This does not repeat the separate core/network owner reviews.

Material inputs were repo `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, and the local architecture, code-shape, verification, testing and Rust standards. Both active lesson inputs were read completely: 5,230 global + 1,958 repository bytes = 7,188 bytes / 2,397 conservative estimated tokens. No lesson archives or ignored source files were reviewed; no project skill directories were found. The approved Plan 15, integration ledger/current PASS report, and relevant Plans 07–09 authority/effect/unknown-order contracts informed the review.

## Authority and physical identity checks

- `live_fixture.rs:55–73` requires genuine non-preview origin, full explicit parameter binding, no pending publication, and all-four canonical-root refusal before the first constructor physical stage. The additional retained-header root check precedes normal initialization.
- `live_fixture.rs:112–165` verifies the concrete Fjall parent under publication serialization, loads every actual active body, and replays the full prefix through the existing independent Memory engine with P2SH and the same selected parameters. Replay supplies expected positions, undo, coins/B/heads and counts. It never replaces or upgrades the supplied source.
- `live_fixture.rs:195–245` compares source and physical content with the captured independent expectation, rejects conflicting physical Some counts, retains physical None counts, requires complete ScriptsValid identity coverage, and compares actual retained bodies and validation records again. Both clean-cache aliased poison and cache-masked poison controls exercise the public constructor boundary.
- `live_fixture.rs:88–108` preserves the SAME consumed core through normal initialize/from_recovered, prepares an unwrapped network, constructs the graph, and then performs the captured final comparison. The publication guard is released before initialization, header/body graph publication and normal BASIC proof/owner methods. No nested publication lock or production import of cfg-test count/attachment/adoption helpers was introduced.

## Graph and opening checks

- `live_fixture.rs:247–269` admits captured actual headers once, checks the unique root body, and uses the cached validator for every historical nonroot. It does not assign historical sequence numbers or mint accepted-tip preference.
- The populated graph controls exercise actual strict extension and unchanged equal-work refusal. The genesis-only control uses genuine new A/B body events to earn order and perform the earlier-eligible equal-work replacement.
- `open_runtime.rs` extracts concrete private prepare/finish helpers. Ordinary startup still uses its raw recovered core, network-selected parameters and existing ordinary refusal/default activation behavior. The new public fixture path passes its explicit full parameters into runtime assembly and exposes only the specified seven-argument consuming constructor.
- All four actual canonical constructor cases first create genuine managed sources using coherent per-root test bits/BIP34-off parameters, then require the canonical-specific error. Unit counters assert no constructor physical-read/init stage for those cases; these controls are not earlier origin/parameter failures.

## Test delta and evidence limits

The parent reported a late test-only addition: `historical_order_controls` in `sync/tests.rs:96–159` and its registered test at `sync/tests/phase165_live_fixture.rs:608–611`. It was included in this review. It checks that checked duplicate delivery and a cloned/reseeded header store retain unknown historical order and cannot mint equal-work preference. This is a reviewed test-only addition, not production drift. Static inspection finds 34 enabled unit test functions and 14 enabled external test functions.

The parent reports 34/34 unit and 14/14 external GREEN, with refreshed strict compilation gates in progress. Those are parent/executor reports, not tests independently executed by this reviewer. No tests, builds, substantial Bun suites, implementation/checker changes, source mutations, Git state changes or commits were performed here. The scoped tracked `git diff --check` passed.

The executed stage-interleave control is correctly limited to capture → real initialization/recovered graph → ordinary writer → captured final check. Separate current-source guard work owns proof of public ordering. This review does not claim a whole-public-constructor concurrency experiment, rollback, or ABA/history detection when final captured facts are identical. Normal BASIC proof guards remain in force and are separate from stable ScriptsValid records and legitimate fresh BASIC generations.

Whole-phase Plan 14 native verification and the parent's final independent phase review remain required after all repair work. Current TypeScript guard renewal is outside this six-file source review. Phase 166 canonical bootstrap/exact all-handle reopen/genesis BASIC, Phase 167 contextual completeness, and Phase 168 pinned-Knots qualification remain deferred.

## Reviewed identities and closure

The following six hashes were captured after reading the final test addition and rechecked unchanged at closure on 2026-10-17T03:47:24Z. No production or test drift occurred between that reviewed hash baseline and closure. `scope_sha256` is SHA-256 of the following ordered manifest, each line encoded as `hash`, two spaces, repo-relative path, and LF.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/sync/live_fixture.rs` | `4da004e8b12a1b6bd0da8d460dde94b70a7c1119893fbe03ba2d048d707b9b9c` |
| `packages/open-bitcoin-node/src/sync/open_runtime.rs` | `e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5` |
| `packages/open-bitcoin-node/src/sync.rs` | `88cf2a12bca8a236a4a8d821afe96e17cdd13a63c5ff8f159da5d24e3dfbe035` |
| `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs` | `2768f24bf56b51acd8186a2e92eeb7feb21f7482e85c678f561ce03d75aba8f8` |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs` | `392ca0c03bf4dc095c6175ff05552beab3ad794cf78399c81d9d88d719720e4f` |
| `packages/open-bitcoin-node/src/sync/tests.rs` | `a9b4b57990b015cee3909013834862c98e234ec121b7bee7467a460fdee21732` |

| Planning input | SHA-256 |
| --- | --- |
| `165-15-PLAN.md` | `de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3` |
| `165-INTEGRATION-REPLAN.md` | `978fbfdf92d53c2619cc999872f3ba4662483f3d84b317f52032775a16a4881d` |
| `165-INTEGRATION-REPLAN-CHECK.md` | `79a7d1dadb9a02d4c422ed9d03d4410512eb8a85f294402c026af775d023d03c` |

Only this preliminary report was created. Re-review affected source if these identities change.

---
phase: 167-candidate-history-validation-rules
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T11:39:00Z"
requirements: [CVLD-01, CVLD-03, CVLD-06]
commits: false
status: candidate-awaiting-independent-check
source_execution_released: false
---

# Plan30 command safe-reopen envelope

Root release `.local/open-bitcoin-dev/phase167/reopen-bound-plan/root-planning-release.json` SHA256 `b24c57dc8ac3fafeffcbf57ffeef43ef11bcd965c33a181a3b7e0e26b6a5796f` authorizes only this formal planning revision. Plan30 remains unexecuted/unreleased. The source order remains accepted33 revalidation, root-coordinated34 production/build, then30 only after independent review/root release. Active33 retains sole build ownership during this task. No source/build/source formatting/metadata/Git/guard/report edit or source acceptance is performed by the planner.

## Existing boundary and evidence

Read-only preparation `.local/open-bitcoin-dev/phase167/plan30/preparation/REOPEN-BOUND.md` is `254c525cf84dac8980e79467916068ed23b714f09f2afefbf50989391a1a5ef0`; READINESS.md is `461ab05ab19124524a81f07e278fa693fc211e189293d22d2961465266bddeb3`. They are source-grounded preparation, not execution evidence. A genuine legacy fixture currently admits/persists307 retained identities, then the complete safe parser refuses UnsupportedHistory. The accepted factory proves its starting checkpoint fits the existing envelope, but does not constrain later header admission or KnownOnly publication.

The authoritative MAX_HISTORY_BLOCKS=306 belongs to storage/canonical_history/records.rs and is used by complete record parsing and retained graph proof. This is total retained identity capacity including genesis, KnownOnly, stale accepted forks and other retained headers, not active height. Body-registry50,000, batch128, row/decode65536/4096/512MiB and30-second worker limits are separate bounds and do not replace306. No bound is increased or mirrored.

## Exact new owner and callable limit

Add ONLY `packages/open-bitcoin-node/src/storage/canonical_history/records.rs` to the seven existing Plan30 source owners. Define a small documented `pub(crate)` INHERENT `FjallNodeStore::canonical_history_reopen_limit() -> usize` in that private records module, returning the existing MAX_HISTORY_BLOCKS. The already-exported FjallNodeStore is accessible in the same crate, so this requires no storage parent import/reexport, public carrier, new module or ninth owner. Existing constant visibility/value, all parser/proof checks and legacy adapters remain unchanged.

```rust
// Inside the already-owned storage/canonical_history/records.rs.
impl crate::FjallNodeStore {
    /// Existing total retained graph envelope of safe canonical-history reopen.
    pub(crate) const fn canonical_history_reopen_limit() -> usize {
        MAX_HISTORY_BLOCKS
    }
}
```

This is a planned same-crate interface, not compilation evidence. The provider actually calls it; no copied production306 constant, public caller capacity/boolean or dead-code suppression. Boundary literals in independent test expectations do not become the production policy owner.

The added file is still active33 during planning. Its observed snapshot is recorded as observation only, not an unchanged/frozen input; there is no stable source copy or preservation assertion for it. Root's active33 source freeze39940309 identifies the executor's own evidence, not permission for this planner to lock that source. Accepted final33 source/SUMMARY/API must be reloaded and getter placement/reachability revalidated before30 source release. All active33 six sources, SUMMARY/execution receipts, root live metadata and34 test preparation are excluded from immutable captures. Existing seven unreleased30 source/existence states can be captured statically; this does not qualify their future APIs or release their source.

## Exact order and operational policy

Retain every original authority/typed-validation/durability obligation. The new check fits this order:

1. Complete lossless raw decode, actual owner/store/selected consistency, shared selected raw intrinsic checks and fallible Amount conversion.
1. Genuine pending/persisted accepted-identity duplicate handling. It remains before capacity and descendant context, including genesis and accepted stale/witness-only cases. KnownOnly or header cache is insufficient for duplicate.
1. Under the actual operation mutex AND authority lock, count actual held HeaderStore entries and test the actual candidate hash's existing entry. The getter supplies the sole storage limit. Do not read a caller snapshot, active tip height, body availability or a clone as a preview authority.
1. Refuse a held graph already beyond the limit, or a novel identity when the total equals the limit, through the typed operational/support error path. At305, one genuinely new identity can reach306. At306, existingKnownOnly body acceptance remains possible because no identity is added; it still must pass all real body/Coins checks.
1. Only after a permitted count/membership result may detailed contextual header admission, insertion/KnownOnly publication, body retention, Coins/mempool absorption and durable publication occur. Continue ordered Always flush/BASIC and actual target/store/incarnation/freshness checks, including supported retained identity state after authority is released for BASIC turns.

No malformed/intrinsic-invalid body becomes duplicate/capacity: intrinsic checks remain earlier. At full unsupported resource capacity, operational -25 may precede a contextual missing-parent/version/time reason. This is an explicit bounded-support policy exception, not a consensus rejection, cached failed state, inconclusive result or Knots differential parity claim. No BlockValidationError or parsing-error string is fabricated. Existing typed error can express this operational category inside its owned provider child; no new error/export owner.

This constrains command-added identities, not independent legacy/public handle/raw-store adapters. External changes remain possible outside the command mutex and must be detected by final live/store/freshness checks rather than hidden behind a global capacity claim. Preserve the genuine307 source fixture and its UnsupportedHistory refusal; it remains a valid test producer and source compatibility negative. No preview HeaderStore or new authority path, no second Fjall engine and no parser/proof/row/time/resource-budget change.

## Third task and required real evidence

Both existing Task1/Task2 XML and every original automated command remain byte-exact. Add one coherent third TDD task with exactly four owners:

- packages/open-bitcoin-node/src/storage/canonical_history/records.rs
- packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
- packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs
- packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs

The union is eight owners, three tasks and34 plans/66 tasks/28 waves. No dependency/wave changes. Task2's original seven-owner contribution remains unchanged; plan-wide numerical scope/success text is updated explicitly, with the new context and this amendment authoritative. Other33 plans are wholly exact. No new lifecycle, requirement closure or phase acceptance is implied.

Tests must use actual safe configured/genesis-seal producers and genuine selected history/held HeaderStore/store, not mocked count fields or invented proof labels. Getter/test compilation is setup; genuine RED is the real unguarded command exceeding the envelope versus the expected pre-effect refusal, with actual original state/reopen evidence. GREEN must demonstrate:

- 305->306 genuine new identity acceptance, ordered full persistence/BASIC, drop ALL handles, safe configured reopen and complete proof/state comparison.
- At306, novel main and side identities refuse before any header insertion/KnownOnly/body/Coins/durable mutation, with full accepted/known/header/body/Coins/counts/undo/provenance/BASIC equality and byte-level original-manifest equality.
- Actual genesis, active descendant, stale accepted branch and valid witness-only duplicates remain possible at306 after full intrinsic/accepted identity; intrinsic-invalid same-header bodies fail before both duplicate and capacity.
- An already-admitted genuineKnownOnly candidate can earn ScriptsValid body acceptance at306 without growing identity count, then persists/drops/reopens safely. Earlier contextual/script/Coins checks are never waived by membership.
- Genesis, KnownOnly and stale/fork identities count toward capacity; active-height/body-only/accepted-only implementations must fail the tests. Unsupported direct external advancement is detected as operational support/freshness failure rather than false durable success.
- The unchanged existing `phase167_history_records_finite_complete_307_checkpoint_refuses` still produces complete307 source and rejects; no global live-adapter cap is added to make this fixture impossible.

All new cases use the named `phase167_local_submission_reopen_bound` selection, and all original Task1/Task2 selections rerun on final source with nonzero named discovery. Preserve all setup/RED/GREEN/failures, exact source fences and source/body/manifest/proof identities. Local storage/source fixtures are not actual daemon/differential or broad networking evidence. Plan13 retains authenticated freshly built process evidence; global qualification gates remain closed.

## Readable source/test feasibility

The added getter is a short same-file impl with no parent changes; the provider check is a small pure count/membership guard composed at the real lock boundary. Substantive real-history/manifest helpers stay in the existing main test owner and typed boundary tests in the nested owner; coherent helpers/cases can be redistributed between those two declared files, not compressed or hidden in a new child. Ignored normally formatted projections show the limit getter/guard and complete required test semantics, not compiled implementation. Existing source files for both test owners are genuinely absent at this planning point; do not fabricate empty historical source files.

Projected budgets remain\<=628 for all eight: consensus witness251->\<=400, consensus block479->\<=565, node provider13->\<=620 including the guard/error, network eligibility537->\<=600, canonical_history239->\<=420, records observed320 plus getter (revalidate accepted33), main test owner\<=600 and typed child\<=620 with shared helpers and all inherited matrices intact. These are planning bounds, not proof that future code compiles/fits. Stop for checked repartition if actual complete normal-format tests or interfaces need a ninth owner or629lines; no minification, suppression, file-limit/cap increase or dropped case.

## Preservation, threats and handoff

Local AGENTS, sidecar, overrides and architecture/code-shape/testing/verification/Rust standards informed the single functional-core policy owner, thin actual shell guard and behavioral test ordering. Active lessons were fully reloaded (global6898B+repo1958B,8856B/2953estimated tokens), with no archives or lesson edits. Formal gsd-plan-phase revision/template/lifecycle remains yolo167-2026-10-21T16-38-23 and commits:false; root alone owns metadata/Git and independent checker alone owns canonical verdict append.

| Threat                                                | Required mitigation                                                                                                               |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Runtime grows beyond its proved source envelope       | Actual held total entries/membership under operation+authority locks before every new identity effect.                            |
| Copied/mismatched policy limit                        | Sole inherent storage getter returns existingMAX_HISTORY_BLOCKS; no provider numeric mirror or parameter.                         |
| Capacity confused with consensus invalidity           | Explicit operational/support -25 ordering, no failed flag/inconclusive/consensus error.                                           |
| Height/accepted-only count or blanket at-limit denial | Genuine KnownOnly/stale/fork/genesis and305/306 positive/negative tests.                                                          |
| False durability/reopen result or global bound claim  | Actual Always/BASIC/drop-all/safe configured reopen, whole manifest comparisons, final freshness and unchanged legacy307 control. |
| Concurrent active33 ownership                         | Observation-only added owner, explicit active exclusions and accepted33 revalidation before30 release.                            |

Security enforcement remains enabled; high findings block acceptance. D-01/D-07/D-08/D-09/D-10 and all17/18/19/20/25 source-before-guards/native/fresh-coverage/Bazel/security/goal/lifecycle/compact/Git gates remain. Source breadcrumbs stay mandatory;19 alone owns manifest reconciliation. Existing public/network default-off and support refusals remain.

Before receipts under `.local/open-bitcoin-dev/phase167/reopen-bound-plan/plan/` retain all34 planning files/prior amendments/canonical report, the seven existing source/existence states and prior planning history, with active33 and34-preparation exclusions explicit. Complete inverse restores original Plan30 SHA256 `bf139ed5e275d5d5e0bac86b6a44471923e328431c58ed3bfd324b2658588137`; the new amendment inverts to absence. No earlier failed/readiness/checker receipt is overwritten. Current canonical prefix is296152bytes SHA256 `7ebf67880b34626139061ed2fdf1d5f6f6981a70dff854806812a1897f91e87b`; all known older prefixes remain exact. Final index awaits independent check; no source execution release or self-approval follows.

---
phase: 165-shared-exact-work-authority
plan: "22"
subsystem: genuine-runtime-event-fixtures
tags: [announcements, compact-timeout, metrics, structured-logging, source-guards]
requires:
  - phase: 165-21
    provides: Completed protocol fixtures and reviewed 124-owner current guard baseline
provides:
  - Genuine empty sources and actual root/body setup for announcement and compact timing fixtures
  - Actual accepted-child logging with full-width target-derived work
  - Preserved transport, clock, cancellation, projection and privacy assertions
  - Reviewed128-owner guard inventory and stable complete raw matrix closure
affects: [165-23, 165-24, 165-14]
tech-stack:
  added: []
  patterns: [Existing strict empty transfer, Shared prefix initialization, Data-driven mutations]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/structured_runtime_logging.rs
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
key-decisions:
  - Transfer a genuine empty source through the existing strict fixture seam before actual body mutation.
  - Select complete normal or maturity 1 parameters before genesis and header binding.
  - Preserve existing body-derived child-header admission instead of adding redundant ingress.
  - Reuse existing mutation loops and retain the timing children as unchanged dependencies.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T11:35:08Z"
client_date: "2026-10-17"
duration: 32min
completed: 2026-10-17
tasks_completed: 3
files_modified: 6
---

# Phase 165 Plan 22: Genuine Runtime Event Fixtures Summary

**Announcement, timing, projection and logging fixtures now use genuine source/root prerequisites while preserving their actual transport and event assertions; all 35 scoped Rust cases and both complete current guard matrices pass.**

## Scope and Actual Setup

All three tasks completed against checked plan `c5ab47ea56879e6ca374a62a9282032041587010b69cd7c15d222e773aa71d8f`. The actual execution evidence window is 2026-10-17T11:02:54.280Z–11:34:29.837Z,31 min 35.557 s. UTC and America/Chicago client dates are 2026-10-17. Six implementation files changed within the ten declared owners. No new Rust file, public API, dependency, helper module or breadcrumb row was introduced.

The announcement and authoritative projection factories transfer `Chainstate::new()` through existing strict `adopt_live_fixture_owner` onto their actual fresh empty Fjall targets. Their actual named model root is admitted before the original first local body connection. Existing body eligibility validates and derives subsequent child headers itself; no redundant child-header ingress was added. Announcement ordering remains actual connect → persist body → queue durable tip → dispatch.

The shared timing factory likewise starts with a genuine empty source. Its private `compact_fixture_prefix` requires empty headers, selects the complete parameters before any genesis/header binding, initializes actual genesis and then actually connects the spendable parent. Normal compact cases retain maturity 100; only the existing connectable-spend fixture selects maturity 1. The two-argument factory API remains unchanged. No already-bound source parameters or unowned child source changed.

The two logging ingress cases accept actual model genesis outside ordinary P2P, then send a real child through the existing Version/Verack/Headers/Block script. They still require four processed messages, one received header, one received block and exact peer contributions/activity. Endpoint hashes/heights and header/downloaded/connected log fields now identify height 1. Full-width expected work is the wrapping sum of actual root/child target proofs, replacing the unrelated literal 1. Original log levels, sources and progress_signal assertions remain.

Original seven-byte socket prefixes, cancellation credit, retained encoded suffixes, released transport ledger, peer capacity, enqueue races, exact error classes and sensitive-data checks remain. Timing children preserve clock counts 17/19/5, timeout+5 cadence, no early fallback, matching fallback hash/body persistence, empty inflight state, target-mismatch error and Idle/Closed/partial-frame distinctions. Projection preserves nine actual write acknowledgements, `eligible_peer_count == 2`, one compact announcement and common authoritative metrics/log snapshots. Already-GREEN unobserved/inbound projection and logging/privacy/stall/retry/storage cases remain meaningful.

## Executed Evidence

Fresh baseline commands reproduce19 RED and retain16 GREEN across the four full selectors. All four baseline children exited 101 with zero ignored cases. Their exact receipts are:

- `.local/open-bitcoin-dev/phase165-plan22-baseline-sync::tests::production_announcement_transport_cases.log`
- `.local/open-bitcoin-dev/phase165-plan22-baseline-sync::tests::runtime_timing_cases.log`
- `.local/open-bitcoin-dev/phase165-plan22-baseline-sync::tests::runtime_projection_cases.log`
- `.local/open-bitcoin-dev/phase165-plan22-baseline-sync::tests::structured_runtime_logging.log`

Announcement8, timing7 and projection2 RED cases stopped at untrusted live-source prerequisites; the two logging RED cases reached incorrect genesis-through-P2P counter expectations. Baseline libtest durations were 3.69/4.12/1.81/3.15 seconds respectively. No RED was manufactured for the existing 16 passing cases, and the immutable original Plan14 diagnostic remains separate.

All four complete GREEN selectors use cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features --lib followed by the literal filter below. All Cargo work ran serially through command-timings with pinned Rust 1.94.1 and command-local Bun 1.3.9 PATH.

| Complete selector                                    | Baseline pass/RED | GREEN pass/RED/ignored | Libtest seconds | Actual GREEN receipt                                                                                           |
| ---------------------------------------------------- | ----------------- | ---------------------- | --------------- | -------------------------------------------------------------------------------------------------------------- |
| sync::tests::production_announcement_transport_cases | 1/8               | 9/0/0                  | 5.32            | `.local/open-bitcoin-dev/phase165-plan22-task1-green-sync::tests::production_announcement_transport_cases.log` |
| sync::tests::runtime_timing_cases                    | 7/7               | 14/0/0                 | 4.40            | `.local/open-bitcoin-dev/phase165-plan22-task1-green-sync::tests::runtime_timing_cases.log`                    |
| sync::tests::runtime_projection_cases                | 2/2               | 4/0/0                  | 2.05            | `.local/open-bitcoin-dev/phase165-plan22-task2-green-sync::tests::runtime_projection_cases.log`                |
| sync::tests::structured_runtime_logging              | 6/2               | 8/0/0                  | 3.20            | `.local/open-bitcoin-dev/phase165-plan22-task2-green-sync::tests::structured_runtime_logging.log`              |

These selectors cover 35 cases; repeated baseline/GREEN execution is not additional independent test coverage. The timing parent has zero direct tests. Its14 executed cases live in the four unchanged children, not in an invented parent test row.

Required format check, strict Node all-target/all-feature Clippy with `-D warnings`, and Node all-target/all-feature build each exited 0. Exact receipts are `.local/open-bitcoin-dev/phase165-plan22-fmt.log`, `.local/open-bitcoin-dev/phase165-plan22-clippy.log` and `.local/open-bitcoin-dev/phase165-plan22-build.log`. Wrapper durations were 1.507/6.430/0.295 seconds; compiler times were 6.32 s for Clippy and 0.07 s for build. Breadcrumb audit and managed checks also passed before pin renewal.

## Independent Review and Exact Guard Renewal

The authentic ten-owner before capture is `.local/open-bitcoin-dev/phase165-p22-before.json`, SHA `f9091dc4809586f6e6e0898c7069f9274485f26d34e414153b9e0be270b18bcb`. Frozen full before/current sources, diffs, seven dependency sources and proposed data appear in `.local/open-bitcoin-dev/phase165-plan22-prepin-source-review.json`, SHA `d55aff6c39acf90b5f544b81861d748397b8e60c3cef0e0b45f877c9a1361dc2`. Root and the independent reviewer authenticated the exact source/control scope before authorizing pins.

Fifteen selected-function source mutations reuse the existing executable FIXTURE_SOURCE_MUTATIONS loop. Every actual anchor occurs exactly once inside its selected function and every replacement changes source. Controls remove genuine transfer, root initialization, actual connect or durable body save; substitute default parameters or a duplicate genesis connection; reduce nine writes; restore genesis P2P headers; or discard the child target proof. Existing test disabling and pre-read inventory controls remain enabled. These source-policy mutations complement actual Rust execution; they do not represent executed mutated Rust binaries.

The approved proposal is `.local/open-bitcoin-dev/phase165-plan22-pin-proposal.json`, SHA `a804b98bf733cd1f1a8b1417d3e69bdb2b19079a89788e6b8793a2036e7290c1`. Exactly four source rows 124–127 and three real test-owner rows 61–63 were appended, with declaration counts 9/4/8. The timing parent receives its changed source identity and no zero-count test row. All 124 old source rows and 61 old test rows remain exact. Counts become 128 source owners and 64 test-owner files.

Four required source inputs occupy zero-based 240–243, growing 253→257 while preserving every old input's relative order. With rows applied under the old pin, `.local/open-bitcoin-dev/phase165-plan22-pre-read-red.log` records exit 1 before any source read, SHA `23c1e64fed4753c2397625462c6823bedb6e76f5cd34fec20537cd874304a2da`. Only the inventory literal then changes to `68896e7925de650b2cfa0371a517248b46dbb408de3cf533cfdd7870407204db`.

Root and reviewer independently reversed only the seven appended rows and one literal, reconstructing both full pre-pin sources. The other eight owner sources and all seven dependencies remain exact. Root proof `.local/open-bitcoin-dev/phase165-p22-root-pin-proof.json` has SHA `0c28c4c7c34a2920b5ff9fec3324326967465e61c5a867bd6f8c6458d66dcb37`; complete post-pin sources are `.local/open-bitcoin-dev/phase165-plan22-postpin-source-review.json`, SHA `98552812edc476a114f77cef3f122ffe37347c3ea054a508bc66133a1b5bf46d`. Actual ordered arrays are `.local/open-bitcoin-dev/phase165-plan22-postpin-data.json`, SHA `e351b057d6ff5a20661913115c3340adbfc95cad89247802a43964bdbbbd937d`.

Targeted current/owner/inventory controls passed38/0 with 219 expectations in19.78 s, `.local/open-bitcoin-dev/phase165-plan22-targeted-controls.log`, SHA `4162977921d1e64381d6a092f4922fb14a65612aef534c0fb416b14eec655bdc`. Fast current 158 and 162 checkers exited 0 in `.local/open-bitcoin-dev/phase165-plan22-current158.log` and `.local/open-bitcoin-dev/phase165-plan22-current162.log`;162 reports 208 fixed contracts and exact originalFAA attribution.

## Final Sources and Preserved Dependencies

| Changed implementation owner                                                         | Lines | Complete raw SHA-256                                             |
| ------------------------------------------------------------------------------------ | ----- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs | 609   | 669c1a6baf4ad3ee87bde3f00a97fd4dfe57c079c486c2f6d9c312ea88522e2e |
| packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs                    | 353   | d257b0180645de9b41262ea8fbe79970dfb16ce1994a8d054f953f6c275afd33 |
| packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs                | 319   | 525816c156cdf7e451e1e3877b11101cb71d534948fa9524c38ce80736840dce |
| packages/open-bitcoin-node/src/sync/tests/structured_runtime_logging.rs              | 342   | 6bc90c323d7061afa408f050a9b9453999aaf61d378c65c132800e462b738935 |
| scripts/check-phase165-shared-exact-work.ts                                          | 266   | 1e14496e2012cf915336af31e12a41f56e8632ce386d841d5e8fb1267596dff0 |
| scripts/check-phase165-shared-exact-work/contracts.ts                                | 568   | afae7e3f27ac258cf6d07140d4f5a9bdaeabd1401ca369faf48476f1219005ad |

All three declared old164 owners remain unchanged at 135 fixed Rust inputs/477 edges; these four new owners were outside that inventory. The executable current 165 test file also remains unchanged at 627 lines. The simplification pass used existing strict source/root seams and one cohesive prefix initializer, avoiding duplicate normal/maturity 1 setup, new paths or guard-code growth. Existing Rust breadcrumbs remain valid.

Unchanged critical dependencies are recorded with full source bytes in review packets and explicitly included in the final freeze:

| Dependency                                        | Raw SHA-256                                                      |
| ------------------------------------------------- | ---------------------------------------------------------------- |
| runtime_timing_cases/compact_timeout.rs           | 9a05bd081de39352346b7878f3161dfde098270ea8169d08e101ba237e2fb604 |
| runtime_timing_cases/idle_sessions.rs             | b670db7b33d0735588dbe0ca445be5c258819f7e3499f94a6713cd7c5b4efbeb |
| runtime_timing_cases/session_boundaries.rs        | f2a1af5fed5a440654fdcbdc53d6f1ece5985f5b166473f2fbedfc6e1e89f3e4 |
| runtime_timing_cases/tcp_read_semantics.rs        | 8de1396bce9f0a90b728b56b2471af0b82d08d1596d72a1767034ee5b31ff187 |
| sync/tests/support_runtime.rs                     | 5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05 |
| sync/tests/support_blocks.rs                      | e2c553577d65bfe108975d138f99cb72a9ef6a2a81c86c95941027f05636a2f7 |
| storage/chain_compatibility/tests/preservation.rs | e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b |

Timing-child paths are under packages/open-bitcoin-node/src/sync/tests; the final three paths are under packages/open-bitcoin-node/src. The current 20,688-byte preservation source and original 20,556-byte attribution remain separate. Support helpers, authority owners, working158/162 tools, exact historical inverse composition, original 162/163 manifests/recipes/receipts/snapshots and the mixed C bundle were not edited. All six mandatory verifier registrations remain intact.

## Complete Matrices and Frozen Raw Boundary

The new ignored collector/driver preserve Plan21 originals. Changes are only unique 22 output labels/current 257 identity and five explicitly reviewed dependency additions: four timing children plus support_blocks.rs. Existing live and phase70 helper dependencies remain. All original fixed checker read sets, actual factory initializer/corpus, optional/canonical checks and TypeScript module/entry closures remain. This is the complete raw checker/factory/module union plus declared critical Rust dependencies, not a claim to freeze the entire Cargo compiler dependency closure.

The pre-run review packet is `.local/open-bitcoin-dev/phase165-plan22-matrix-review.json`, SHA `0eef76351d4baa23ca3a2f10ffffa21539c3c6674615876b1bc73e6e0d6754c9`. Root freeze proof `.local/open-bitcoin-dev/phase165-p22-root-freeze-proof.json`, SHA `424650de163753704cf870aff8ae7d36570d36cecefcbbeef6f68d906078a49e`, authenticated the exact collector diff, driver and every 484 raw entry before execution. The freeze covers482 present files/two unchanged permitted absences,36 modules,14 CLI modules and all 168 raw corpus entries, separately from current 165's257 normal fixed inputs.

| Complete current matrix | Pass/RED | Expectations | Wall seconds | Actual receipt                                                | Log SHA-256                                                      |
| ----------------------- | -------- | ------------ | ------------ | ------------------------------------------------------------- | ---------------------------------------------------------------- |
| 164                     | 868/0    | 1565         | 339.416118   | `.local/open-bitcoin-dev/phase165-plan22-matrix-164-full.log` | 2e40357632dfbeade56abe98282bc21419339df2427289e7117e78bc9ada9b6a |
| 165                     | 1641/0   | 5913         | 728.898529   | `.local/open-bitcoin-dev/phase165-plan22-matrix-165-full.log` | c3a2d15d8f1b42ed73f3feed4a9f18431b1bec838827f7dac26595cb77fa6cfe |

All seven serial driver commands exited 0. Exact164 reports 135/477; exact165 reports 128/64. Breadcrumbs pass. Managed checks scan1,521 files with zero exceptions/findings. Whitespace passes. No new ignored/skipped case was introduced. Sessions were polled within 60 seconds, and healthy quiet jobs continued normally. Exact commands, all seven log hashes and durations are in `.local/open-bitcoin-dev/phase165-plan22-matrix-matrix-results.json`, SHA `0cfc659d21e40c61eba343dd731cf917ec7f35a6f3ed42c08179fe7a4d5a80ff`.

Pre-run freeze `.local/open-bitcoin-dev/phase165-plan22-full-freeze.json` retains SHA `0ad70987300449ad8a96369d65fdea1402920a63b5740797a57158ef1771287e`. At 2026-10-17T11:34:29.837Z, `.local/open-bitcoin-dev/phase165-plan22-full-freeze-closed.json` confirms all 484 raw hashes, canonical paths, lengths and presence states unchanged, SHA `b85a04e4dcb3e476588b5c95fa4180711b7faca27086fd8e8ef23e9c88dc3dd4`. Root independently rehashed the entire boundary and authenticated all seven successful logs.

The current derived 168-entry corpus is 4,406,522 bytes/SHA `57cefc42026a3bc892467ec63b67609bab7b7fd84e0dd304dbf1f732a5d28080`; its only difference from 21 is completed-plan ROADMAP counters before this new freeze. Immutable original evidence remains exact. No shared document changed during this execution, and no additional matrix was run after success.

## Issues and Retained Coordinator Gates

All 19 actual baseline RED cases are closed, and the 16 original GREEN cases remain. No additional runtime correction, retry loop, owner expansion, new helper path, cap exception, authentication gate or guard relaxation was needed. The pre-execution plan's nine→ten owner-count correction did not change its declared paths. Stub scanning found no unfinished implementation; existing unavailable wording and omitted-operation strings are deliberate negative claim/mutation data. No new production endpoint, schema or unplanned trust surface was introduced.

Plan23, the public requalification proof28 currently under planning, Plan24 and final 14 remain root-owned; this Summary does not preclaim approval or execution of that proposed prerequisite. Root retains default native/workspace/coverage/Bazel/benchmark checks, complete source/security/formal/lifecycle acceptance, shared-state updates and normal-hook commit/push. Requirements remain contributed rather than complete. All ordinary selectors still refuse, networking stays default-off, and 166/167/168 retain canonical bootstrap/exact reopen/genesis BASIC, contextual and pinned-daemon qualification respectively.

## Self-Check: PASSED

All three tasks are complete. The Summary and six changed implementation owners exist within the ten declared paths. All four scoped Rust corpora, three strict gates, targeted controls, two full matrices, exact checkers and required policies passed. Every 484 frozen entry and all seven critical dependencies retain authenticated bytes and presence states. Timing-parent test count remains zero; all 14 child cases are accounted for. Source/data review and precise pin reversal are recorded. Git and whole-phase acceptance remain coordinator-owned.

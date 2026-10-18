---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T04:52:48Z
client_date: "2026-10-16"
depth: standard
review_kind: preliminary_source_only
review_state: completed_preliminary_source_review
status: clean
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_in_scope: 9
files_reviewed: 9
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
files_reviewed_list:
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
---

# Phase 165: Preliminary Plan 17 Rust Review

The final source-only review is clean within these nine Rust files: no actionable correctness, security or test-reliability findings. The final closure below supersedes the earlier preparation state while preserving its initial hashes and observations. All nine complete final files and their fixed-base changes were read after the parent's source-stability signal. No tests, build, heavy Bun execution, source modification or Git mutation was performed by this reviewer. Whole-phase review/security/native/formal gates remain required.

## Context Loaded

Current Plan 17 and the latest inventory-plan-check amendments preserve sequential predecessor GREEN, the extracted literal source inventory and strict historical identities. Plans 15 and 16 summaries distinguish genuine same-core fixture assembly from raw recovery, and independently checked historical body order from newly earned order. Earlier rejected designs remain historical evidence.

The root instructions, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust standards and both active lesson files were loaded in this ongoing review session. Active lessons total 7,188 bytes and a conservative 2,397-token estimate; no archive was loaded. There are no project skills or `.claudeignore`. Existing raw negative tests are not presumed live merely because positive fixture setup changes.

## Original Intent Inventory

The fixed-base assertions and fault locations were inventoried before assessing the in-progress changes. Earlier Phase 165 mechanical work and the pre-existing managed fixture wrapper are distinguished from the new Plan 17 adapters; a cumulative diff is not assumed to be entirely new Plan 17 code.

| Owner                   | Original behavior and assertions to preserve                                                                                                                                                                                                                                                                                                                    |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Branch tests            | Own-session active identity; actual disconnected ScriptsValid after paired body/undo pruning; header/body/legacy rows never imply scripts validation; invalid-gate precedence and metadata budget.                                                                                                                                                              |
| Transport tests         | Shared generic buffers and physical charges; exact peer/authority binding; complete-frame accounting; configuration/disconnect/poison/recovery invalidation during writes; immutable range across mempool and accepted extension; cancellation and cleanup without authority-lock acquisition.                                                                  |
| Query tests             | Disabled authority and disabled-before-unknown precedence; typed corruption and resolution ordering; found/missing/legacy behavior during catch-up; accepted unflushed Pending frontier, processed latch and conservative cold-open observations; pending identity before history commit; prompt initial missing and bounded continuous-height/singleton reads. |
| Accepted tests          | Direct and staged historical/same-block scripts and exact filter record; out-of-order fact suppression; accepted target retained through metadata/body/undo/coins faults; visible ordinary caller failures with old safe cursor and prune protection; mempool removal/evidence after core acceptance even when persistence errors.                              |
| Readiness tests         | Capacity/cancellation/checked IDs; completion before poll; fixed original target rather than chasing later acceptance; wrong-hash token rejection; request provenance; unflushed completion/final read; exactly one disable wake.                                                                                                                               |
| Readiness interleavings | Poison and reentrant wake only after both guards release; waker replacement/cancel/drop outside registry; latest waker exactly once; a never-connected request retains original snapshot classification after later genuine acceptance.                                                                                                                         |
| Reorg fixture           | Existing actual acceptance and staged capability path; raw reopen remains observation-only; recovered projection/rewind/parent controls and preserved physical identities; one explicit strict full-prefix transfer for an intentional later mutation.                                                                                                          |
| Append fencing          | Budget and malformed-map precedence; interruption and foreign preparation refusal; generation/header/projection fencing; actual deferred own-flush releases zero-record work with equal bounded measurements at 17 and 129 blocks, then exact durable checkpoint after closure.                                                                                 |
| Storage reorg           | Strong reserved and ordinary locks, authenticated displacement, replacement-only durable endpoint, lagging/counter/borrowed/foreign/invalidation controls, immutable retry identity and conflicting-byte refusal. Only the mutation-intended lock case changes to the strict fixture reopen.                                                                    |

## Initial Adapter Observations

The in-progress shared `ValidatedHistory` adapter accepts the actual old two-block prefix with explicit maturity-one parameters into a genuine empty Fjall owner, saves only old-prefix bodies and performs a real same-core flush before normal BASIC assembly. It avoids the old `history.seed(1)` recipe that also persists future undo. Requested-body failure setup explicitly checks the future body is absent and reads the constructor's already-captured two-header prefix; its later scripted Headers message contains only the new nonroot child.

The branch adapter now transfers genuine empty ownership before actual managed acceptance. Transport setup builds and accepts four actual blocks and uses normal constructor/indexing rather than a raw imported header store. The explicit `reopen_live_fixture` checks physical undo against the live source before missing-count preparation, then performs complete actual-prefix validation and strict ownership transfer. These are preparation observations only; they are not final behavior or source certification.

At the initial capture, `accepted.rs` had 632 lines and `fencing.rs` had 629. The parent states both are being simplified in-place. These transitional counts are recorded as open verification items, not final findings or accepted cap exceptions. Later readiness failure cases owned by Plan 20 remain outside Plan 17's closure; owned filtered results must not imply the entire readiness suite is GREEN.

The parent relayed the executor's reconciliation: eight RED filters executed 92 cases, with 63 passing and 29 failing. Historical group A37 consists of those 29 plus eight callers in readiness `failures.rs` sharing the repaired prerequisites. That module's ninth weaker-replacement case belongs to Plan 20. No unrun readiness-failures GREEN is claimed. Initial branch 4/4 and transport 16/16 are reported executor results only; query execution is still active. A compilation error distinguishing Fjall `snapshot` from `overlay_snapshot` was reportedly corrected without assertion changes. Final receipts and stable source remain pending.

## Preparation File Identities

These are initial in-progress whole-file SHA-256 values, not closing identities. Implementation may legitimately drift after this capture.

| File                                                                                                     | Initial SHA-256                                                    |
| -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs`                  | `9915768162cb82758109fa6096e0fdff7b35a0adfeab19c268211c10d318c441` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs`               | `055e5bdcaf3d290ee59ea580d9ff4c15cea261a7baffab435ee114b8dbbdd2fb` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs`                   | `9140f8c10abc5b86f2ec1d84eb528d1a8f320872e4e4c21f82d18d74f4720d16` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs`                                     | `a375b6cca7f3fe1af341976542c9d5560dcb3dab64a725566e12efe8ac87d48b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs`               | `1027bd9cb2898a509ddd01ead3331f57c1c63f42cc0c0aa20825e676696e94f0` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs` | `6c87a0c8c3b10cc15ad8c1156158b884d424cf0277dc9351e1dcd7924cbf1ea2` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs`                                 | `69a4e4eb3eb24b64f6eed3b588fbf10418bb1432556c9941c4d8db465223e075` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs`                     | `aea8400ea2ad45d3b78e76f830c88006e25e7981fbbe0bccf79740322ef17f14` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs`                              | `a13a478cdaae20f3234c58a9bb5b6152a3c923a3024a8c521e6da427fb7fb944` |

Initial ordered scope digest: `e6ea24b4f6a7d6b8e6759e48bdf3dc30b70e58f52464ab54d957b23004853d65`.

Digest construction is SHA-256 over the nine frontmatter paths in order, each encoded as UTF-8 path, tab, lowercase complete-file SHA-256, newline. The immutable supporting helper `packages/open-bitcoin-node/src/sync/tests/support_runtime.rs` still has SHA-256 `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05` at preparation time.

## Preparation Checklist (Historical)

- Wait for the executor's Rust-source stability signal.
- Read all nine final complete files and inspect their final delta against the original assertion/fault inventory.
- Recheck typed first-effect errors, actual Pending/wake paths, full selected parameters, physical body/undo/script/pruning controls and unchanged raw negatives.
- Capture exact closing hashes and attribute every changed preparation identity; verify immutable support helper and file caps.
- Attribute the executor's actual nonzero Rust receipts without claiming independent execution or broad later-plan closure.
- Finish only this preliminary report; final whole-phase review/security/native/formal gates remain required.

## Final Source Review Closure | 2026-10-17T04:52:48Z

**Verdict: clean; zero critical, warning or info findings.** This is an independent source review, with executor-reported behavioral evidence separately attributed. All preparation checklist actions are complete for this preliminary scope. The final whole-phase review remains required.

### Authority, Fault and Original Assertion Checks

- `branch/tests.rs:14` transfers genuine empty ownership into the concrete Fjall view before managed creation. Positive ScriptsValid/pruned branch setup actually connects the relevant blocks. Header admission uses selected parameters rather than importing numerical authority. Raw active/legacy/header/body negatives and invalid-gate/resource assertions retain their original intent.
- `transport/tests.rs:157` accepts a real four-block history, retains actual bodies, flushes the same source, then uses normal public BASIC assembly. The original physical-frame, charge, peer/session, poison, immutable-range, cancellation and recovery-invalidation assertions remain in place. No populated root is rebound after construction.
- `accepted.rs:38` constructs only the actual old two-block prefix under explicit maturity-one parameters, flushes the same genuine Fjall core and initializes normal BASIC before any injected successor fault. The old `history.seed(1)` future-undo recipe is not used to certify that prefix. The memory adapter independently connects the actual same prefix instead of importing a snapshot as authority.
- `accepted.rs:454` deliberately retains the original `FlushLifecycle::ready_for_test(0, 0, now, false)` zero-cache pressure policy after genuine initialization and consuming same-core extraction. This makes the direct connect reach its intended injected persistence effect. Original accepted-tip, target, retained historical/same-block script facts, paused/failure and old safe-cursor assertions remain. Ordinary local and requested-body faults are armed after valid setup; requested intake still proves actual GetData, visible peer error, accepted height two, zero credited bodies and absent failed body persistence. The historical broad `is_err` assertions are not relabeled exact typed-error assertions by this report.
- Query mutation cases use the genuine old prefix, while disabled/corruption/legacy/read-only cold-open cases retain their raw setup and original classifications. Unflushed Pending target, processed latch, conservative durable height, retained row read bounds and unknown-before-corruption ordering remain asserted.
- `readiness/tests.rs:218` separately models the raw legacy request premise. After genuine constructor setup, normal Version/Verack and contiguous new nonroot header ingress, and actual height-two acceptance, the test invokes the already-existing coverage invalidator. It observes `UnknownLegacy` for height three before capturing the Pending request, then actually accepts that block and retains the original `UnknownLegacy`/initially-synchronized result for the captured request. Source inspection confirms coverage invalidation changes Complete to UnknownLegacy/revision and preserves positive ScriptsValid facts; it does not rewrite coins, work ownership or headers. This approved setup explicitly creates the old negative premise rather than weakening the assertion.
- The shared readiness `accept` helper saves the actually accepted body's bytes after real connection. It does not restore a missing or pruned input. Capacity/cancel/token/fixed-target, no-lost-wake, exact one-wake and reentrant/poison guard-release assertions remain. `readiness/tests/interleavings.rs:310` retains a separate genuine NeverConnected-before/ScriptsValid-after case; same full parameters apply through genesis, constructor, known-only nonroot headers and later body acceptance.
- `fixture.rs:21` compares all actual durable undo with the genuine source before the existing fixture-only missing-count preparation, then closes/reopens, validates every available active body and performs strict complete physical transfer. Ordinary `reopen()` stays raw. The only newly changed Plan 17 storage caller is `storage/.../reorg.rs:38`, whose stronger reserved/operator lock test intentionally mutates after reopening. Existing earlier-phase callers are not presented as new Plan 17 ownership. No ordinary recovery path or production capability is added.
- `append/fencing.rs:505` retains actual validated expected history and genesis overlay equality, starts a genuine same-Fjall source, performs real genesis acceptance/flush and normal BASIC initialization, then executes the original suffix/flush/release experiment at 17 and 129 blocks. It preserves exact endpoint/protection, zero block-release work, bounded reads/record operations, equal measured work and all-handle durable checkpoint assertions. The extra store is consumed into the source; the later `store` is a borrow of the runtime, so dropping the runtime closes its owning handles before reopening. The analogous extra store clone in the never-connected interleaving is explicitly dropped before runtime assembly and final cleanup.
- Storage reorg counter, borrowed/foreign position, invalidation, idempotent/conflicting immutable retry, raw metadata and physical projection/fence controls retain their original assertions. No score inflation, future-undo copying, missing-input restoration, direct lower reorg bypass, count-equality weakening or ordinary raw reopen promotion was introduced by these adapters.

### Exact Stable Identities and Preparation Drift

The following full-file identities were captured after the stable-source signal, then recaptured after the complete source read; all nine matched in both final captures. Five are unchanged from preparation. Four legitimately changed during implementation and were fully reread: accepted, readiness main, readiness interleavings and append fencing. The two transitional cap overruns are resolved; the maximum is now 623 lines.

| File                                                                                                     | Closing lines | Closing SHA-256                                                    | Change since preparation                                                           |
| -------------------------------------------------------------------------------------------------------- | ------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs`                  | 390           | `9915768162cb82758109fa6096e0fdff7b35a0adfeab19c268211c10d318c441` | None                                                                               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs`               | 607           | `055e5bdcaf3d290ee59ea580d9ff4c15cea261a7baffab435ee114b8dbbdd2fb` | None                                                                               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs`                   | 439           | `9140f8c10abc5b86f2ec1d84eb528d1a8f320872e4e4c21f82d18d74f4720d16` | None                                                                               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs`                                     | 623           | `5a261b2a919c7b0022166bd727296947ef9faf04111b3c0436306e2c1afb9d0e` | Imports/helper simplification and restored original zero-cache direct-fault policy |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs`               | 381           | `bae3782a960294bbff64950dfeeb397cdd646877247189adf9ddd85bf277fb02` | Actual accepted-body retention plus explicit legacy request setup and observation  |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs` | 436           | `80df8a74c64e197c459a7588e75f6af2ef56339138b753c7403cc8763c71626a` | Drop the extra store clone before same-core assembly/final cleanup                 |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs`                                 | 602           | `69a4e4eb3eb24b64f6eed3b588fbf10418bb1432556c9941c4d8db465223e075` | None                                                                               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs`                     | 623           | `fc58454dfcd883fd0b0980bda09cf31b8da8f719a045ba9f69b384b2bedee119` | Local simplification and consuming the extra store into the source                 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs`                              | 594           | `a13a478cdaae20f3234c58a9bb5b6152a3c923a3024a8c521e6da427fb7fb944` | None                                                                               |

Closing ordered scope digest: `839e41774e424340ca0876b8457101d2ec41580f47bb9289283ab80d0ff035be`, using the same path-tab-hash-newline construction documented above. Immutable `support_runtime.rs` remains `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05` in the final capture. Scoped Rust whitespace inspection produced no findings. A subsequent source change requires delta review; this verdict applies to these exact closing identities.

### Executor Evidence and Pending Broader Gates

The parent relayed final execution of all eight owned filtered corpora: 92 passed out of 92, with no new ignored cases; final format/check exit zero, strict Node all-target/all-feature Clippy exit zero in 6.43 seconds, and build exit zero in 0.08 seconds cached, aggregate session 94926 exit zero. Earlier branch 4/4, transport 16/16, query 10/10 and accepted 13/14 diagnostics remain historical; the latter exposed the direct zero-cache effect prerequisite, now restored. These are executor receipts, not commands run by this reviewer.

The parent also reports current165 registration of the nine actual Rust owners and 83 static declared tests, distinct from 92 filtered executions, eleven actual loaded-source mutations, and ordered input growth from 219 to 228 preserving old relative order. None of these nine owners existed in old164's inventory, so its source/edge/test rows remain unchanged. Literal mutation data stays in the already-owned current165 contracts module. Those TypeScript changes and the active full164/full165 matrices are outside this nine-file Rust review and are not certified or prematurely marked passed here.

The eight readiness `failures.rs` callers sharing these prerequisites were not run by the owned filters, and their ninth weaker-replacement case remains Plan 20 work. This report makes no whole-readiness, whole-workspace, default-native, coverage, canonical selector, bootstrap, exact all-handle authority, contextual completeness or actual pinned-Knots differential claim.

Only this preliminary report was written by the reviewer. Final whole-phase source/security/formal review and the unchanged default native verifier and normal-hook Git finalization remain parent-owned.

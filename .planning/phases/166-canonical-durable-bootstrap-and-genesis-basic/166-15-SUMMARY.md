---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "15"
subsystem: storage
tags: [fjall, process-isolation, bounded-protocol, source-preservation, checked-cleanup]
requires:
  - phase: 166-04
    provides: Bounded private copying, retained source locks and exact owned-root cleanup
provides:
  - Same-executable child-only scratch recovery with fixed panic refusal
  - Complete bounded detached record protocol with checked job and result identities
  - Parent source recheck, recovered-tree validation and cleanup before proof callbacks
  - Compiled Cargo and Bazel product entrypoints and four registered test roots
affects: [166-05, 166-06, 166-09, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [same-executable reserved dispatcher, monitored stdin delivery, reaped-child evidence, checked post-cleanup callback]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-cli/src/lib.rs
    - packages/open-bitcoin-cli/src/bin/open-bitcoin.rs
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/BUILD.bazel
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Use only current_exe and the exact reserved argv; retain source locks and cleanup in the live parent.
  - Complete records require successful child exit after engine teardown plus every parent gate before validation.
  - Keep genuine raw abort characterization distinct from contained malformed refusal and no-job routing evidence.
patterns-established:
  - Child completion uncertainty retains its exact owned scratch root and forbids proof callbacks.
  - Source identity is checked before recovered-tree permission changes and exact-root cleanup.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T03:45:59Z"
commits: false
duration: 21 min recorded implementation and verification window
completed: 2026-10-20
---

# Phase 166 Plan 15: Process-Isolated Scratch Inspection Summary

**Pinned malformed engine recovery now refuses inside a same-executable child while the live parent preserves the source, reaps the child and checks its exact private scratch cleanup before handing detached records to a validator.**

## Completion Boundary

Tasks: **3/3 complete**. The recorded implementation/checking window began at 2026-10-20T03:25:05Z and completed at 2026-10-20T03:45:59Z; initial instruction loading is excluded. Fifteen implementation/evidence files are owned here, including the expressly accepted metadata-only extension to the copy module, plus this summary. All Rust files remain below 629 lines.

The combined 166-04/15 inspection goal is eligible for independent root acceptance before 166-05. This summary grants no canonical original-store admission, CBOT/CWOR closure, whole-phase acceptance or permission to skip later verification. Frontmatter requirement IDs record this contribution. Root owns shared state, roadmap, requirements, task progress and final normal-hook Git saving. No staging, commits, push, branch/worktree operation, hook bypass or historical source-contract renewal occurred.

## Accomplishments and Interfaces

The retained private contract with_inspection(&OperationLease, FnOnce(&InspectedRecords) -> Result<T, StorageError>) now runs real isolated recovery. Private inspect_owned_then retains the original application and actual existing engine locks through child completion, source identity/content/mode/mtime recheck, recovered-tree checks, exact-root cleanup and the proof callback. The callback runs after all those gates; no original backend opens here. The existing pure-copy controls still use their test-only adapter.

Private OwnedScratchJob carries only the parent-created scratch root/device/inode, separate result-file identity and fresh nonce. It contains no source path, original engine lock or operation lease. Root placement must match the canonical fixed /tmp parent, random owned directory shape and 0700 identity. The result is a parent-created 0600 regular single-link file whose selected and opened identities are checked. The child validates the exact database/result layout, private copied entries and a separate scratch operation lease before recovery.

The private versioned length-prefixed application-record codec caps stdin jobs at **16 KiB**, namespace-plus-row units at **4096**, aggregate logical payload at **512 MiB**, and result framing at **MAX_BYTES + MAX_ENTRIES * 24 + 64**. It checks lengths/counts/arithmetic before allocation, rejects duplicate namespaces/keys, unknown version/status framing, truncation, trailing bytes and job/result mismatches. Complete namespace maps, binary keys/values, empty namespaces, empty values, absent namespaces and missing rows remain distinct. The DTO has no Debug/Clone/raw-owner escape. Existing-record decoding remains reusable for immediate original revalidation in 166-06.

The controller launches only std::env::current_exe() with exactly `--exact __open_bitcoin_storage_inspection_worker --nocapture --test-threads=1`. There is no sibling, PATH, environment executable override, mutable registration or parent recovery fallback. Child stdout/stderr are null. A child-only non-payload panic hook exits **78** immediately before engine work; the inspecting parent installs no hook. All keyspaces/database owners and the separate scratch lease drop before success serialization, including Fjall worker stop/join in DatabaseInner teardown. The parent requires a complete validated result **and exit 0**.

The centralized **30-second** deadline starts before process launch and covers stdin delivery and waiting. A separate capped writer is monitored with child status and result-file size/identity. Timeout, overflow and transport failure kill/reap a live child; lifecycle errors remain visible. Uncertain child completion retains the root without chmod/removal or proof callbacks. Reaped outcomes still run source recheck, recovered database privacy/cap checks and checked cleanup, including earlier refusals. Root identity is checked before recovered-tree traversal so a replacement cannot redirect chmod into the source. Cleanup errors block accepted records.

Only doc-hidden FjallNodeStore::maybe_run_inspection_worker() -> Option<i32> and register_inspection_worker! are exposed for dispatch/registration. Node library, CLI library, CLI binary and daemon binary roots register the exact helper. The CLI dispatches before Clap. The daemon has a plain synchronous main that dispatches before calling its Tokio-decorated run_application helper, preserving ordinary application startup. Both Bazel products declare direct existing node-library dependencies.

## Files Created and Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs`: bounded job/result codec and six codec behavior cases.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs`: child-only raw adapter, exact dispatcher and monitored parent lifecycle.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker/tests.rs`: eleven real recovery/lifecycle/routing cases and exact-route test-only faults.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs`: restricted internal codec access; removed consumed existing-reader staging expectation and former raw recovery owner.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs`: post-cleanup validation under both source locks, explicit reaped/uncertain outcomes and root checks before permission traversal.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs`: retained/adapted foundation and raw characterization cases; original malformed behavior now requires actual valid recovery and exact redacted refusal, source preservation, cleanup and engine lock reacquisition.
- `packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs`: removed only the consumed secure_recovered_tree dead-code expectation; physical limits and behavior are unchanged.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`: doc-hidden dispatcher surface.
- `packages/open-bitcoin-node/src/lib.rs`: exported root registration macro and explicit node root registration.
- `packages/open-bitcoin-cli/src/lib.rs`: explicit CLI library root registration.
- `packages/open-bitcoin-cli/src/bin/open-bitcoin.rs`: dispatch before parser and explicit binary root registration.
- `packages/open-bitcoin-cli/BUILD.bazel`: direct existing node dependency on the product binary.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`: synchronous dispatch then existing Tokio application helper; explicit binary root registration.
- `packages/open-bitcoin-rpc/BUILD.bazel`: complete existing daemon module sources, explicit crate root and direct existing node/network/mempool/getrandom dependencies.
- `docs/parity/source-breadcrumbs.json`: all three new Rust paths registered with the existing infrastructure breadcrumb.

## Decisions, Deviations and Limits

The process-isolation architecture follows the checked amended plans and resolved research. The implementation stays within the existing adapter, dependency set and workspace. No external endpoint, new production dependency, lockfile or crate is introduced. Planned file/process trust surfaces are covered by T-166-15-01 through T-166-15-06; no additional unmodeled surface was found.

**[Rule 2 - Missing critical integration] Remove the consumed recovered-tree staging expectation.** The coordinator explicitly accepted the narrow extra ownership of `packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs`. Production now consumes the already-existing check, so retaining its temporary expectation would fail lint. No copy behavior or limit changed.

**[Rule 3 - Blocking packaging] Complete the existing daemon Bazel declaration.** The first fresh daemon build exposed missing child module sources. The next two retained attempts isolated crate-root inference and missing existing getrandom linkage. Auditing actual production module imports resolved the declaration with its existing module glob, explicit root and existing node/network/mempool/getrandom dependencies. The third fix attempt passes; no further fix loop or dependency change occurred. The failures remain separately recorded below.

The initial controller compilation rejected a private re-export; its visibility was corrected within the private store module. The first controller suite passed eight cases but took 151.18 seconds because repeated fresh fixtures hashed preallocated journals. A captured live sample identified active Inventory::copy_to / SHA256 work. Later failure-control fixtures perform a real all-handle reopen before snapshots; the unchanged original fresh/reopened canonical+BASIC sizing case still measures its actual physical payload. No build/test was killed for running quietly.

Fresh packaging used one new owned empty root, with root device **16,777,231**, inode **607,768,872**, uid **501**, mode **0700**. Cargo CLI/daemon/help and concrete-status checks completed first. The exact recorded Cargo subtree inode **607,768,909** (2.2 GiB) was then removed after identity checks before Bazel compilation. The Bazel subtree inode was **607,780,664**. Only the server started with that exact output_user_root was shut down. An initial cleanup guard matched its own command and refused without deletion; a later removal stopped visibly on read-only Bazel runfiles directories. Final cleanup revalidated the same identities, made only 1058 owner directories under that stopped owned tree writable without following symlinks, and removed the exact build root. No old cache, editor, unrelated root or source directory was cleaned. Runtime receipts remain ignored and preserved.

The actual nonnumeric/overflow backend cases pass on this macOS host. The previous APFS OS92 non-UTF8 fixture-creation limit remains setup evidence only; the Linux-only supporting-filesystem case stays present, with no Linux/non-UTF8 execution claim. The cleanup guarantee requires a live parent and cooperating supported application openers plus the actual engine lock. Parent/system crash, forced termination, OOM, OS crash-dump handling and arbitrary direct-engine/filesystem writers remain outside that guarantee. Test-only uncertainty seams use a supervisor that actually reaps its own process before test fixture removal; the production orchestration still observes uncertainty and retains the root.

The simplification pass retained one private pipeline for source copying, child completion, source checking, recovered-tree validation and cleanup; validation is a final continuation under the same locks. Pure codecs, engine reading, process effects and test faults remain separate modules. No placeholder or blanket refusal remains. Contributor/operator documentation and product canonical-reopen qualification stay with 166-09/13; this internal worker adds no ordinary operator workflow.

Guidance applied: repository AGENTS/native verifier/tool pins/parity ownership, Bright Builds sidecar, standards overrides (no concrete exception), standards index and architecture/code-shape/verification/testing/local-guidance/Rust pages, formal GSD execute-plan workflow and summary template. Both canonical active lesson files were fully loaded: **7977 bytes / 2660 conservative estimated tokens**. No project skills exist. Shared-main synchronization and final saving stay with root.

## Current Source Anchor Handoff

166-11/12 must review current source before renewing only their assigned guards. Existing guarded generic backend construction remains in `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs`. Raw scratch construction is now confined to `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs` raw_recover, called by the exact worker child and the separately controlled test characterization. The parent proof continuation is inspect_owned_then in `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs`.

The CLI main/dispatch anchor is `packages/open-bitcoin-cli/src/bin/open-bitcoin.rs`; daemon main is synchronous and run_application is Tokio-decorated in `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`. Four roots each add a reserved helper and a routing case. Current totals observed by filtered harnesses are **1965 node library, 423 CLI library, 2 CLI binary and 118 daemon binary tests**; renew only after the assigned complete-source review. Other zero-match integration roots are not proof. No historical receipt or source contract was changed here.

## Task Commits

None. `commits: false` follows the strict wrapper. Tasks 1, 2 and 3 are implemented and checked; root owns all final staging/commits/push after whole-phase verification through normal hooks. No auth gate or external setup was needed.

Root handoff check: `verify-summary` reports `passed: true` and a passed self-check. Its generic hexadecimal scanner also treats the decimal filesystem device/inode receipts above as commit identifiers, producing a nonblocking missing-commit diagnostic. Those numbers are filesystem evidence, not commit references; they remain preserved. Commit-existence verification is inapplicable to the intentionally uncommitted plan work.

## Automated Evidence

All Cargo/Bazel invocations were serial through `scripts/command-timings.ts` with pinned Bun 1.3.9. Normal Cargo used the coordinator-approved isolated phase target; fixtures canonicalized existing process-local TMPDIR. Fresh packaging used its independently checked new build root. Full native verification remains 166-14/root.

Historical/problem observations are preserved as their actual outcomes:

| Observation | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Codec scaffold RED | Exit 101; zero passed, one failed; a scaffold assertion, not malformed-backend proof | `.local/open-bitcoin-dev/phase166/plan15/protocol-red.log` |
| Initial codec GREEN | Exit 0; four passed; two staged unused-code warnings before controller integration | `.local/open-bitcoin-dev/phase166/plan15/protocol-green.log` |
| Initial controller compile | Exit 101; private re-export visibility error; no tests executed | `.local/open-bitcoin-dev/phase166/plan15/worker-green-first.log` |
| First actual controller GREEN | Exit 0; eight passed, 151.18 seconds | `.local/open-bitcoin-dev/phase166/plan15/worker-green-second.log` |
| Controller liveness sample | Active bounded-copy hashing, not a blocked backend | `.local/open-bitcoin-dev/phase166/plan15/worker-first-liveness.txt` |
| First expanded harness GREEN | Exit 0; 37 node cases and three other root routing cases | `.local/open-bitcoin-dev/phase166/plan15/harness-first.log` |
| Genuine original malformed RED | Earlier exit 101; zero passed, one failed with two raw child SIGABRT6/source preservation/scratch retention | `.local/open-bitcoin-dev/phase166/plan04/malformed-engine-child-red-final.log` |
| Required valid-control RED | Earlier exit 101; zero passed, one failed on valid recovery | `.local/open-bitcoin-dev/phase166/plan04/isolation-still-pending-red.log` |
| Fresh Bazel daemon initial build | Exit 1; missing existing child module source | `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-daemon.log` |
| Fresh Bazel daemon first retest | Exit 1; explicit crate-root inference required | `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-daemon-retest.log` |
| Fresh Bazel daemon second retest | Exit 1; missing direct existing getrandom linkage | `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-daemon-final.log` |

Final acceptance evidence:

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Required combined phase166_inspection harness command | Exit 0; node **39/39** (49.13 seconds), CLI library **1/1**, CLI binary **1/1**, daemon binary **1/1** | `.local/open-bitcoin-dev/phase166/plan15/harness-final.log` |
| Final worker filter | Exit 0; **11/11**, 31.21 seconds, including real 30-second deadline | `.local/open-bitcoin-dev/phase166/plan15/worker-final.log` |
| Final protocol filter | Exit 0; **6/6**, exact combined logical-unit boundary and malformed job/result controls | `.local/open-bitcoin-dev/phase166/plan15/protocol-final.log` |
| Original malformed refusal, nocapture | Exit 0; **1/1**, 41.19 seconds; integer/overflow child success, no signal, scratch absent and source preserved | `.local/open-bitcoin-dev/phase166/plan15/malformed-green.log` |
| Reserved helper discovery only | Exit 0; **four no-op helpers**, zero storage-proof claim | `.local/open-bitcoin-dev/phase166/plan15/helper-discovery.log` |
| Fresh Cargo CLI build/run | Build succeeds; launcher/concrete worker **78**, stdout/stderr empty | `.local/open-bitcoin-dev/phase166/plan15/packaging-cargo-cli.log` |
| Fresh Cargo daemon build/run | Build succeeds; launcher/concrete worker **78**, stdout/stderr empty | `.local/open-bitcoin-dev/phase166/plan15/packaging-cargo-daemon.log` |
| Ordinary Cargo CLI help | Exit **0**, ordinary usage rendered | `.local/open-bitcoin-dev/phase166/plan15/packaging-cargo-help.log` |
| Fresh Bazel CLI build/run | Build succeeds with 751 actions; launcher/concrete worker **78**, stdout/stderr empty | `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-cli.log` |
| Fresh Bazel daemon final build/run | Build succeeds; launcher/concrete worker **78**, stdout/stderr empty | `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-daemon-green.log` |
| Final node/CLI/RPC Clippy, all targets/features, warnings denied | Exit **0**, 5.53 seconds | `.local/open-bitcoin-dev/phase166/plan15/clippy-final.log` |
| Final scoped rustfmt | Exit **0** | `.local/open-bitcoin-dev/phase166/plan15/format-final.log` |
| Final breadcrumbs | Exit **0**, 1221 Rust files | `.local/open-bitcoin-dev/phase166/plan15/breadcrumbs-final.log` |
| Final managed standards | Exit **0**, zero findings | `.local/open-bitcoin-dev/phase166/plan15/managed-final.log` |
| Final diff whitespace | Exit **0** | `.local/open-bitcoin-dev/phase166/plan15/diff-check.log` |

The combined node count includes two ordinary controlled-child helper no-ops; these are separate from its 37 substantive/routing cases. The three other root cases exercise exact/closed-stdin/near-match routing, not storage recovery. Unrelated zero-match executables are excluded from positive counts. Real valid detached bytes, callback locks/order, source/root changes, private/symlink/hardlink results, nonzero/signal/panic, complete-but-live/nonzero output, capped result overflow, stalled input, cleanup failure, uncertainty and concurrent distinct roots have explicit assertions. Raw abort characterization stays separate from the now-green safe-refusal behavior.

Packaging status and concrete executable/output receipts are `.local/open-bitcoin-dev/phase166/plan15/packaging-statuses.json`, `.local/open-bitcoin-dev/phase166/plan15/packaging-cargo-concrete.json` and `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-concrete.json`. Executables came from the actual Cargo/Bazel invocation output; no production lookup fallback was used. No-job 78 establishes dispatch/build availability only. Actual canonical Cargo/Bazel reopen remains 166-09.

Ownership/cleanup receipts are `.local/open-bitcoin-dev/phase166/plan15/build-root-receipt.json`, `.local/open-bitcoin-dev/phase166/plan15/packaging-cargo-cleanup.json`, `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-shutdown.log` and `.local/open-bitcoin-dev/phase166/plan15/packaging-bazel-cleanup.json`. They confirm checked removal of the recorded task-owned build root after its server shutdown. Focused source/runtime artifacts stay ignored; earlier failure receipts remain intact.

## Self-Check: PASSED

All fifteen owned implementation/evidence files and this summary exist. The final six protocol cases, eleven controller cases, combined harnesses, original malformed behavior, four registered helper roots, concrete Cargo/Bazel dispatch checks, CLI help, scoped formatting, Clippy, breadcrumb registration, managed checks and diff review pass. The recorded temporary build root is removed. Source files remain below the managed size limit and every new Rust file is registered. This self-check covers plan 166-15; root retains combined checkpoint acceptance and later lifecycle verification/saving.

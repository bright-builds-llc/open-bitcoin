---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "09"
subsystem: adapters
tags: [canonical-genesis, daemon, operator, capability-consumption, product-reopen]
requires:
  - phase: 166-16
    provides: Sealed-consuming canonical RPC context/runtime pair and unavailable wallet
  - phase: 166-08
    provides: Canonical genesis runtime with earned saved BASIC ownership
provides:
  - Configured local durable daemon initialization and independent product reopen
  - Existing-only offline operator genesis/work/BASIC observation
  - Pre-storage activation refusal with preserved selected-network diagnostics
  - Correct displayed bestblockhash on both existing blockchain-info projectors
  - Actual Cargo and Bazel daemon/CLI packaging evidence
affects: [166-10, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [consumed storage capability, retained prebuilt context, fixed observation errors, scoped process scenarios]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs
    - packages/open-bitcoin-rpc/tests/phase166_canonical_startup.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase166_genesis.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/context/canonical_genesis/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
key-decisions:
  - Consume OpenedCanonicalGenesis through the accepted specialized RPC factory and retain its own context/runtime pair.
  - Refuse sync, inbound, transaction relay, block serving and compact relay before any storage opener; keep no-trigger startup storage-free and unsupported.
  - Keep ordinary maintenance workers absent from the genesis route without altering the fixture/ordinary context branch.
  - Reopen existing canonical storage for offline status only; retain ordinary guards for sync mutation and wallet adapters.
  - Fix only the two existing bestblockhash display projections and test the real BASIC hash normalizer roundtrip.
requirements-completed: [CBOT-02, CBOT-03, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T19:00:09Z"
commits: false
duration: Recorded execution window 17:40–19:00 UTC, including the mandatory 166-16 dependency pause
completed: 2026-10-20
---

# Phase 166 Plan 09: Configured Genesis Daemon and Operator Adapters Summary

**Configured local daemon startup now consumes proved genesis into its own RPC context/runtime, while offline operator status reopens existing storage and reports exact work and genuine BASIC facts.**

## Completion Boundary

Tasks: **2/2 complete**. Thirteen Rust implementation/test paths changed under the amended fourteen-path plan; the shared breadcrumb manifest was already updated by 166-16 for the four new 09 files. This summary is separate. No staging, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/task edit or LOC refresh occurred. Requirement frontmatter records this plan's contribution rather than whole-phase closure.

The original adapter attempt was paused when actual product startup exposed the ordinary RPC context gate. Root inserted and independently accepted formal prerequisite 166-16 before releasing 09 again. Its sealed-consuming factory and unavailable-wallet implementation remain owned by that plan; 09 wires the accepted pair and narrowly extends its test root for the RPC hash correction.

Repo AGENTS/native verifier/tool pins/parity/LOC contracts, the Bright Builds sidecar, overrides with no concrete exception, architecture/code-shape/verification/testing/Rust guidance and the formal execute-plan context informed execution. Both active lesson files were completely loaded: 8,856 bytes / 2,953 conservative estimated tokens, without archive reads. No project skills were found. Root refreshed remote state without integration during this shared execution; no executor Git mutation was performed.

## Configured Daemon Boundary

`packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs` validates peer-filter configuration and both network selectors first. Sync, inbound, transaction relay, block serving and compact relay requests require ordinary history before any opener. Their failures retain the existing `CompatibilityRefusal` selected-network, unavailable observed-work, missing-prerequisite, shape and work-decision facts. Backend or canonical-runtime details cannot enter these bounded diagnostics.

Local durable triggers retain their existing meanings: prune mode or an explicit BASIC setting opens durable storage; an explicitly configured absent directory can initialize at the adapter boundary. The normal daemon config parser still requires an explicitly supplied datadir to exist, so actual process startup uses an empty directory. Absence of a configured datadir remains distinct and refused. No-trigger startup returns no store and retains explicit unsupported-history refusal; no transient fixture factory or new core constructor is used in the product route.

The store opener returns `Option<OpenedCanonicalGenesis>`. The runtime opener consumes that owner through `ManagedRpcContext::from_canonical_genesis_runtime_config`, preserving the factory's Basic/Disabled/Unspecified mapping to Enabled/Disabled/PreserveSaved. Main retains the returned context/runtime and obtains raw store clones only after consumption. It does not send the proved runtime back through the ordinary context constructor. The existing pre-parser, pre-Tokio worker dispatch and test-root worker registration from 166-15 remain intact.

The new prebuilt genesis context keeps ordinary mempool checkpoint, coins-flush and broadcast-retry workers absent. The existing no-prebuilt-context fixture/ordinary branch retains its maintenance behavior; the fixture struct literal adds only `None`. BASIC is already published by the accepted canonical runtime. This maintenance exclusion is an authority-scope decision, not a diagnosis inferred from the earlier unsupported SIGTERM result.

The owned unit modules preserve incompatible/unsafe/raw-fixture/metrics/selector/redaction assertions. The old absent/empty matrix continues testing activation refusal, while a separate all-selector success matrix covers prune/BASIC-on/BASIC-off initialization and independent reopen for absent and empty sources. A compile-checked function signature requires the sealed owner; live raw-store fixtures remain unchanged and cannot be passed as authority. Scoped scenario mutexes serialize the owned canonical scenarios, following the accepted inspection-child pattern without changing production locks, sleeps or deadlines. Other unrelated test modules are outside these mutexes.

## Operator Observation and RPC Display

`packages/open-bitcoin-cli/src/operator/runtime/support.rs` sends only offline `SyncCommand::Status` to its focused canonical child module. It uses `reopen_canonical_genesis_existing`, selected-network conversion and `PreserveSaved`; missing/empty/unproved/unsafe sources receive the existing fixed compatibility facts. Later runtime/read failures become the fixed `CanonicalObservationUnavailable` error. It projects actual recovered sync state and BASIC owner evidence into output without saving synthesized status metadata. Compatible storage startup still uses the existing guarded recovery lifecycle; this is not a claim that compatible backend reopen is physically read-only.

Pause/Resume continue through `open_ordinary`. Wallet adapters are unchanged. Tests exercise actual Clap/config resolution, all four frozen work/hash vectors, BASIC configured/safe-height/retained-record facts, missing/empty physical preservation, sync-mutation refusal and the real parsed wallet-backup route with an uncreated destination. The accepted RPC context supplies unavailable-wallet refusal rather than registry or fixture wallet authority.

Actual product testing exposed an existing RPC display error: both `get_blockchain_info` and `durable_blockchain_info` encoded `bestblockhash` in raw byte order, whereas the existing `getblockfilter` normalizer accepts display order. Root amended ownership for `packages/open-bitcoin-rpc/src/dispatch/node.rs` and `packages/open-bitcoin-rpc/src/context/canonical_genesis/tests.rs`. Only those two existing projections reverse bytes; there is no global hash/parser change or new chainwork field. A real canonical test captures both no-metadata and saved-`DurableSyncState` branches, compares frozen display hashes and passes each result through the actual normalizer and BASIC dispatch on all selectors. The prior direct context expectation is corrected from raw to display order.

## Automated Evidence

Every ad-hoc Cargo/Bazel command ran serially through `scripts/command-timings.ts` with pinned Bun 1.3.9 PATH, the approved isolated Cargo target and canonical TMPDIR. Resumable commands were polled within 60 seconds. Quiet active commands were not interrupted; liveness samples are retained. Counts below are test functions, not inflated selector/scenario loop counts. Subset reruns and routing tests are not added as distinct behavior coverage.

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Initial daemon scaffold | Exit 101; 0/1 at parser rejection of absent explicit datadir; setup assumption, not bootstrap RED | `.local/open-bitcoin-dev/phase166/plan09/daemon-red.log` |
| Valid empty-config daemon RED | Exit 101; 0/1 at ordinary startup refusal | `.local/open-bitcoin-dev/phase166/plan09/daemon-red-valid-config.log` |
| Initial adapter attempt | Exit 101; 0/1 at ordinary RPC-context gate; led to formal 166-16 prerequisite | `.local/open-bitcoin-dev/phase166/plan09/daemon-green.log` |
| CLI scaffold compile | Exit 101; typed outcome/hash helper assumptions; no behavior executed | `.local/open-bitcoin-dev/phase166/plan09/operator-red.log` |
| Actual CLI observation RED | Exit 101; 3 passed / 1 status-refusal failure; passing count includes one routing test | `.local/open-bitcoin-dev/phase166/plan09/operator-red-behavior.log` |
| First CLI implementation compile | Exit 101; u64 timestamp required checked i64 conversion | `.local/open-bitcoin-dev/phase166/plan09/operator-green.log` |
| Actual frozen RPC display RED | Exit 101; 0/1, actual raw hash versus frozen display | `.local/open-bitcoin-dev/phase166/plan09/daemon-context-green.log` |
| Superseded process expectation | Exit 101; 0/1 at nonexistent chainwork response field; temporary raw-hash expectation is not parity evidence | `.local/open-bitcoin-dev/phase166/plan09/daemon-vectors.log` |
| CLI corrected compile, wrong hash expectation | Exit 101; 3 passed / 1 failed; shared sync-status hashes are raw-order strings | `.local/open-bitcoin-dev/phase166/plan09/operator-green-corrected.log` |
| CLI wrong Signet work expectation | Exit 101; 3 passed / 1 failed; test incorrectly reused mainnet proof | `.local/open-bitcoin-dev/phase166/plan09/operator-focused.log` |
| Expanded process attempt | Exit 101; 0/3, unsupported SIGTERM expectation followed by two poisoned-scenario failures | `.local/open-bitcoin-dev/phase166/plan09/daemon-expanded.log` |
| First display test compile | Exit 101; RequestParameters namespace mistake; no behavior executed | `.local/open-bitcoin-dev/phase166/plan09/rpc-display-red.log` |
| Both-projector display RED | Exit 101; 0/1, both actual branches returned raw hashes instead of frozen display | `.local/open-bitcoin-dev/phase166/plan09/rpc-display-behavior-red.log` |
| Display GREEN | Exit 0; 1/1, all four selectors and both metadata branches roundtrip to BASIC | `.local/open-bitcoin-dev/phase166/plan09/rpc-display-green.log` |
| Initial expanded qualified attempt | Exit 101; 0/4 at unsupported SIGTERM expectation; three poisoned-scenario follow-ons | `.local/open-bitcoin-dev/phase166/plan09/daemon-qualified.log` |
| Explicit unsupported-signal observation | Exit 101; 0/1, actual signal 15/SIGTERM with disabled-inbound stderr; not a maintenance failure | `.local/open-bitcoin-dev/phase166/plan09/unsupported-sigterm.log` |
| First SIGINT process observer | Exit 101; 0/4 after successful graceful exit, then incorrect expectation of a legacy chainstate snapshot; three poison follow-ons | `.local/open-bitcoin-dev/phase166/plan09/daemon-final.log` |
| First bin scaffold compile | Exit 101; missing CoinsView trait; no behavior executed | `.local/open-bitcoin-dev/phase166/plan09/daemon-bin-regression.log` |
| First corrected observer compile | Exit 101; nonexistent ProofWork.number method; no behavior executed | `.local/open-bitcoin-dev/phase166/plan09/daemon-final-corrected-observer.log` |
| Activation subset | Exit 0; 1/1, later included in final process suite | `.local/open-bitcoin-dev/phase166/plan09/daemon-preflight.log` |
| Parsed CLI independent-vector suite | Exit 0; three behavior functions plus registered routing tests; zero-discovery harnesses excluded | `.local/open-bitcoin-dev/phase166/plan09/operator-independent-vectors.log` |
| Final daemon process suite | Exit 0; 4/4, 182.22s | `.local/open-bitcoin-dev/phase166/plan09/daemon-final-exact.log` |
| Existing canonical daemon regressions | Exit 0; 8/8, 300.35s; unchanged assertions, before guard-only scenario serialization | `.local/open-bitcoin-dev/phase166/plan09/daemon-bin-final.log` |
| Final-source old work controls and local-success matrix | Exit 0; 4/4, 276.92s; compiles/exercises shared final scenario guard | `.local/open-bitcoin-dev/phase166/plan09/daemon-work-local.log` |
| Final-source operator compatibility suite | Exit 0; 10/10, 244.31s; includes the three new behavior functions | `.local/open-bitcoin-dev/phase166/plan09/operator-compatibility.log` |
| Existing RPC node-info regressions | Exit 0; 2/2 | `.local/open-bitcoin-dev/phase166/plan09/rpc-node-regression.log` |
| Existing blockchain-info/prune projections | Exit 0; 3/3 | `.local/open-bitcoin-dev/phase166/plan09/rpc-prune-projection.log` |
| RPC/CLI Clippy, all targets/features, warnings denied | Exit 0; 8.87s | `.local/open-bitcoin-dev/phase166/plan09/clippy.log` |
| Workspace formatting | Exit 0 | `.local/open-bitcoin-dev/phase166/plan09/format.log` |
| Breadcrumb registration | Exit 0; 1241 Rust files | `.local/open-bitcoin-dev/phase166/plan09/breadcrumbs-final.log` |
| Managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan09/managed-final.log` |
| Whitespace review | Exit 0 | `.local/open-bitcoin-dev/phase166/plan09/diff-check.log` |

The final process suite demonstrates fresh and all-handle-released reopen for every selector, exact encoded BASIC/header and displayed RPC hash roundtrip, SIGINT status zero, and exact stored-header work through the public numerical reader. Numerical observation is not used as admission authority. It also covers four source shapes across all selectors and five activation requests, twelve foreign-selector mismatches, raw local refusal and storage-free no-trigger startup. Non-mainnet mainnet-IBD requests retain their earlier parser-specific refusal. Physical snapshots compare bytes, file kinds, Unix modes, mtimes and link targets; atime is excluded. Poison follow-ons above are consequences of their first failed scenario, not separate product defects.

## Actual Cargo and Bazel Product Evidence

One additional owned Regtest datadir per packaging surface supplies proportional concrete-executable evidence; the Rust process matrix covers all four selectors. The ignored Bun helper spawns only the selected concrete product, checks real authenticated loopback RPC against frozen genesis/BASIC vectors, requires wallet and sync-mutation refusal, sends the existing supported SIGINT and waits for status zero, then repeats using the same path after every handle has exited. It retains evidence and only manages its own processes. No public Bitcoin peer or external service is contacted.

| Packaging check | Actual outcome | Full repository-relative receipts |
| --- | --- | --- |
| Cargo-built daemon fresh/reopen | Two concrete daemon exits 0 with no termination signal; RPC vector/roundtrip and mutation refusals pass | `.local/open-bitcoin-dev/phase166/plan09/cargo-product-smoke.log`, `.local/open-bitcoin-dev/phase166/plan09/cargo-product-evidence.json`, `.local/open-bitcoin-dev/phase166/plan09/cargo-daemon-0.log`, `.local/open-bitcoin-dev/phase166/plan09/cargo-daemon-1.log` |
| Actual repo-local Cargo CLI status | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ... sync status` exit 0; frozen genesis/work/BASIC assertions pass | `.local/open-bitcoin-dev/phase166/plan09/cargo-cli-status.log`, `.local/open-bitcoin-dev/phase166/plan09/cargo-cli-status.json`, `.local/open-bitcoin-dev/phase166/plan09/cargo-cli-assert.log` |
| Warm default Bazel build | Both daemon and operator targets pass, 16.230s / 89 actions; existing C dependency emits 13 warnings | `.local/open-bitcoin-dev/phase166/plan09/bazel-build.log` |
| Actual Bazel daemon launcher generation | `bazel run --script_path=... //packages/open-bitcoin-rpc:open_bitcoind` exit 0; generated script inspected and executes the concrete built target | `.local/open-bitcoin-dev/phase166/plan09/bazel-daemon-launcher.log`, `.local/open-bitcoin-dev/phase166/plan09/bazel-daemon-launcher.sh` |
| Concrete Bazel daemon fresh/reopen | Generated launcher executes twice; both product exits 0 with no termination signal; RPC vectors/roundtrip and mutation refusals pass | `.local/open-bitcoin-dev/phase166/plan09/bazel-product-smoke.log`, `.local/open-bitcoin-dev/phase166/plan09/bazel-product-evidence.json`, `.local/open-bitcoin-dev/phase166/plan09/bazel-daemon-0.log`, `.local/open-bitcoin-dev/phase166/plan09/bazel-daemon-1.log` |
| Actual repo-local Bazel CLI status | `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ... sync status` exit 0; frozen genesis/work/BASIC assertions pass | `.local/open-bitcoin-dev/phase166/plan09/bazel-cli-status.log`, `.local/open-bitcoin-dev/phase166/plan09/bazel-cli-status.json`, `.local/open-bitcoin-dev/phase166/plan09/bazel-cli-assert.log` |

The two owned stores and `.local/open-bitcoin-dev/phase166/plan09/product-smoke.ts` remain ignored evidence. Launcher generation success is distinguished from the subsequent actual process statuses. No duplicate Bazel output root was created. Liveness receipts are `.local/open-bitcoin-dev/phase166/plan09/bin-liveness.log`, `.local/open-bitcoin-dev/phase166/plan09/local-liveness.log` and `.local/open-bitcoin-dev/phase166/plan09/operator-liveness.log`.

## Deviations, Review and Remaining Boundaries

- **[Rule 3 — required prerequisite]** Actual adapter startup exposed the ordinary RPC context gate. Root inserted formal 166-16, accepted its sealed factory/unavailable-wallet tests independently and amended 09 to consume that pair. No fixture wallet/context was promoted.
- **[Rule 3 — module size]** Root declared the daemon helper and the focused CLI observation child before creation. Formatting-only maintenance exposed a 659-line CLI support file; extraction preserved the exact helper body before subsequent authorized implementation fixes. The parent is now 592 lines, child 79, daemon main 543, daemon helper 137, process tests 425 and extended context test root 609. No length exception or test deletion was introduced.
- **[Rule 1 — RPC display]** The actual frozen-vector failure justified the narrow two-projector hash correction and real normalizer roundtrip. Root amended the exact two source/test paths and retained historical receipts.
- **Test signal correction:** Root corrected its earlier SIGTERM instruction to the daemon's existing Ctrl-C/SIGINT contract. The actual SIGTERM result is retained, no signal handler/source was added, and successful SIGINT/reopen evidence closes the planned workflow. Ordinary-worker exclusion is separately justified by absent history authority.
- **Scaffold corrections:** The absent config path, typed output access, status hash order, Signet-specific proof, nonexistent RPC chainwork field, legacy snapshot assumption, enum namespace, trait import, timestamp conversion and ProofWork accessor errors remain explicitly documented above. Compile/setup failures are not behavioral RED. Actual public APIs and frozen per-selector vectors were inspected before the final corrected runs.

The simplification pass leaves focused adapter helpers, one sealed factory call and the existing BASIC lifecycle. Owned diffs and new files were reviewed; no goal-blocking stub, new dependency, endpoint, schema, auth path or unmodeled trust surface was introduced. Planned T-166-09-01/02 cover the configured adapter and diagnostic boundaries. Four new Rust breadcrumb registrations were performed by 166-16 while 09 was paused; this executor does not claim that manifest edit as its own.

This is macOS/Unix, source-derived vector and actual Open Bitcoin packaging evidence. It does not qualify ordinary descendants, contextual or Signet challenge validation, actual pinned Knots-daemon differential parity, above-u128 genesis histories, hardware power-loss recovery, arbitrary direct-engine/filesystem writers or production/funds readiness. Supported application leases and engine locks retain their prior scope. No authentication gate or external setup was required.

README and CLI README checks found earlier bootstrap-unavailable/future-166 wording needing the assigned 166-13 contributor/operator update. Current semantic guards/source-anchor renewal remain assigned to 166-11/12; full default native verification, independent reviews, LOC and normal-hook Git saving remain 166-14/root. No unrelated failure was fixed or historical receipt renewed. All Cargo/Bazel jobs and owned smoke processes have exited; the shared build resource is released.

## Task Commits

None. `commits: false` follows the strict wrapper; commit-existence verification is inapplicable to this intentionally uncommitted execution.

## Self-Check: PASSED

All fourteen declared paths, the four new Rust files, this summary and every cited receipt exist. Final behavioral suites, exact work and BASIC observations, actual Cargo/Bazel product startup/reopen and CLI invocation, all-target/all-feature Clippy, workspace formatting, breadcrumbs, managed checks and whitespace review pass. Owned Rust files remain below 629 lines, no goal-blocking stub remains, and no executor commit or shared state/task edit was made.

---
phase: 161-shared-operator-index-and-retention-evidence
plan: "05"
subsystem: api
tags: [basic-filters, rpc, cli, availability, operator-evidence]
requires:
  - phase: 161-04
    provides: Coherent bounded BASIC projection on the existing network operator snapshot
provides:
  - Additive serde-defaulted BASIC evidence on openbitcoinnetworkstatus
  - Single-response CLI collection preserving authority facts and unavailable failures
  - RPC schema/authentication and CLI legacy/stopped/fault regression evidence
affects: [161-06, 161-07, 161-08, 161-09]
tech-stack:
  added: []
  patterns: [Captured typed projection forwarding, additive unavailable defaults, single-response collection]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/tests/basic_index.rs
  modified:
    - packages/open-bitcoin-rpc/src/context/inbound_status.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs
    - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/status_inputs.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs
    - packages/open-bitcoin-bench/src/cases/operator_runtime.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Forward BASIC only from the existing captured snapshot; never query a second producer.
  - Preserve older-response omission as NotReported and collection failures as fixed LiveUnavailable.
  - Stopped JSONC input supplies no proven BASIC flags, so configuration remains unavailable.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T06:46:00Z"
duration: 6min
completed: 2026-10-11
---

# Phase 161 Plan 05: RPC and CLI BASIC Collection Summary

**The existing authenticated network-status RPC and live CLI now carry the same captured BASIC facts, while older responses, stopped collectors and failed collection retain explicit unavailable evidence.**

## Accomplishments and API

All three tasks are complete across thirteen source/manifest paths. `AuthoritativeOperatorSnapshot::basic_index() -> &BasicIndexOperatorStatus` forwards `ManagedNetworkOperatorSnapshot::basic_index()` without collecting another snapshot. `OpenBitcoinNetworkStatusResponse.basic_index` has `#[serde(default)]`; dispatch clones it once from the already captured authority. Existing exact-key assertions include only the additive `basic_index` key. No request parameters, method name, route, authentication mechanism or baseline filter/index result shape changed.

CLI live collection moves `network_status.basic_index` into `OpenBitcoinStatusSnapshot` from the one existing response. Legacy omission becomes the typed NotReported default. Method-unavailable, authentication/network failure or malformed typed response cannot become disabled, healthy, successful empty results or zero totals. Network-status collection failure uses the fixed LiveUnavailable reason without copying error text into BASIC. Stopped collection retains the earlier LiveUnavailable default: current JSONC/config collector inputs do not prove indexing or peer-serving flags, so no false booleans or alternate resolver were introduced.

All eleven fixture/bench response literal sites remain compilable, including the bench adapter and four fixture files. The planned bench reference named a managed snapshot, but the actual existing bench constructor is `OpenBitcoinNetworkStatusResponse`; that constructor received an explicit unavailable default. The new CLI test module is registered under the existing operator infrastructure breadcrumb group, preserving every other manifest mapping.

## Verification Evidence

All Cargo commands ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`. `/tmp/open-bitcoin-bun-1.3.9-fresh/bun` was re-probed as 1.3.9, matching `.bun-version`, and its directory was prepended to PATH for children. No full native verifier, Bazel job, commit, staging or push was performed by this executor.

| Key / scoped check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-rpc-schema-red`: RPC `phase161_rpc_schema` | 2 discovered; both failed expected missing-field assertions | 15,629 ms |
| `phase161-rpc-schema-green`: same selector | 2 discovered, 2 passed | 5,426 ms |
| `phase161-cli-collection-red`: CLI `phase161_cli_collection` | 9 discovered; 5 passed, 4 failed because the collector discarded BASIC | 14,883 ms |
| `phase161-cli-collection-green`: same selector | 9 discovered, 9 passed | 4,425 ms |
| `phase161-cli-collector`: CLI and bench `cargo check --all-targets` | Passed, including migrated fixtures and bench | 4,801 ms |
| `phase161-rpc-schema-final`: RPC `network_status_schema` | 12 discovered, 12 passed, including 3 Phase161 cases | 3,707 ms |
| `phase161-rpc-cli-clippy`: RPC, CLI and bench `--all-targets --all-features -- -D warnings` | Passed | 12,480 ms |
| `phase161-rpc-filter-regression`: RPC `--all-features phase159_filter_rpc` | 32 library and 2 daemon cases passed | 11,838 ms |
| `phase161-cli-status-regression`: CLI `--all-features operator::status::` | 83 discovered, 83 passed, including all 9 BASIC collector cases | 5,931 ms |
| `phase161-cli-support-fixture`: migrated sensitive authoritative support fixture | 1 discovered, 1 passed | 324 ms |
| Scoped Rustfmt, edition 2024, `skip_children=true`, twelve owned Rust paths | Final check passed | Passed |
| `bun scripts/check-parity-breadcrumbs.ts --check` | 1,109 Rust paths verified | Passed |
| `bun scripts/bright-builds-check.ts all` | 1,439 source inputs, zero findings; three repository lessons, zero findings | Passed |
| `git diff --check`, owned diff and simplification review | Passed | Passed |

The RPC tests use actual dispatch and the production HTTP handler with the current authority fixture. The HTTP case verifies unauthorized refusal and authenticated BASIC equality, with serialized BASIC at most 4 KiB. The legacy case removes the actual field and deserializes the older response. Existing filter RPC regressions retain exact `getblockfilter` bytes/header orientation, `getindexinfo` keys, selection/error precedence, fault redaction and authentication behavior.

CLI tests invoke the production collector through its existing client seam. They preserve earned initial sync with later lag, processed 21 versus safe durable 18, accepted 23, lag two, separate peer-scope counts, inclusive protection and exact logical retention measurement/basis. A failed owner with RequiredUndo retains independently observed records and totals. JSON round-trip, omitted old field, method unavailable, disabled live collection, early authentication refusal, network-status failure and malformed available boolean all pass. Every live complete collection records the same exact six existing calls, including precisely one `openbitcoinnetworkstatus`; stopped collection records none, and early authentication refusal records only `getnetworkinfo`. There is no added filter query, store open, activation, repair or other mutation in the production diff.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-05-1: private or unbounded RPC/collector evidence | Existing typed safe BASIC projection forwarded unchanged; fixed LiveUnavailable error reason; private backend/path/peer error fixture remains the exact safe unavailable object; authenticated BASIC serialized within 4 KiB. |
| T-161-05-2: missing or failed data becomes healthy | Meaningful failing-before/passing-after tests; serde-default omission; all BASIC facts unavailable for legacy/stopped/method/network/malformed cases; retained facts remain independent of owner failure. |
| T-161-05-3: authentication bypass or baseline RPC drift | Existing route and HTTP auth remain in use; production handler denies missing credentials; 34 passing Phase159 filter RPC/daemon regression cases preserve pinned behavior. |

No new endpoint, authentication path, storage schema or file-access boundary was introduced. All observations reuse Plan04's bounded capture; this consumer plan does not independently re-prove producer read counters or genuine Fjall history coherence.

## Deviations and Simplification

None requiring scope expansion. The bench wording was aligned with its actual existing response constructor. Task 1's additive wire field and mechanical fixture migration were applied together before CLI compilation, preserving the plan's compilable-boundary requirement. The RED states were observed locally and never committed.

The explicit simplification pass retained one forwarding accessor, one serde-defaulted field, one dispatch clone and one collector move. No new producer, resolver, abstraction, dependency, retry, cache or API endpoint was added. The adapter remains a thin forwarding boundary. Material guidance included repository AGENTS/sidecar, placeholder-only overrides, relevant architecture/code-shape/testing/verification/operability/Rust standards, locked context/research/UI and Plan04's actual API. Both active lesson inputs were loaded completely at 7,188 bytes / 2,397 conservative estimated tokens; archives and trackers were not edited.

## Known Stubs and Residual Limitations

No stubs prevent this plan's goal. Explicit unavailable fixture defaults describe uncollected evidence. The CLI client seam tests are collection/compatibility evidence; Plan08 still owns genuine Fjall-to-RPC-to-CLI/support/dashboard caller proof. Plan06 owns human/support rendering, and later plans own terminal coverage, documentation and deterministic guards. The final frozen-source native verifier, independent source/security review, lifecycle gate and all tracker/commit/push actions remain parent/Plan09 responsibilities. CFOP-01 is intentionally not marked complete.

D-16 exclusions, Phase162 integrated-proof ownership, body/wallet eligibility and soft block/undo targets remain intact. No archive-scale, public-default, production-service, production-readiness, funds-safety or stronger v2.4 crash-window claim is made.

## Task Commits

None by the plan's explicit outer-wrapper contract. No staging, commit, push or shared tracker edit was made.

## Self-Check: PASSED

All thirteen planned source/manifest files and this summary exist. GSD summary extraction accepts the lifecycle metadata, key files and empty requirement-completion list. Focused tests discover nonzero cases and pass; the wider schema, filter RPC, CLI status and migrated support fixture regressions pass. Scoped formatting, strict lint, parity/managed standards and whitespace checks pass. Commit checks do not apply because the outer wrapper explicitly owns finalization.

## Late Black-Box Schema Correction — 2026-10-11T10:02:39Z

Parent-authorized re-entry corrected one additional existing test file, `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs`, bringing this plan's source/manifest inventory to fourteen paths. Native run 3 supplied the exact RED: at line 425 the real `openbitcoinnetworkstatus` response contained eight sorted keys, while the old black-box assertion still expected seven. `/tmp/phase161-native-full-3.log` records two passed and one failed black-box cases and verifier exit 101 after 1,634,308 ms. Its earlier node 1,632 passed / three existing ignored, RPC library 348 passed and daemon 87 passed results were not the failure source.

**[Rule 1 - Bug]** Preserve all seven original response keys and add the additive `basic_index` key in sorted position. The existing authoritative capture now binds one snapshot and serializes both block-relay and BASIC from that same capture. The old block-relay equality assertion remains, and an additional assertion compares BASIC JSON with the captured authority. This adds no collector, production API, mock, network/storage behavior or ignored case. The original narrow checks missed this independent exact-key consumer; this appendix supersedes any inference that the initial Plan05 checks established complete black-box/workspace compatibility.

All late Cargo work ran serially through the timing runner with pinned Bun 1.3.9. The remaining workspace run was explicitly authorized after Native3 exposed this late failure; node tests were excluded because the unchanged node suite had already passed in Native3.

| Key / exact late check | Actual result | Wrapper duration |
| --- | --- | --- |
| `phase161-rpc-black-box-additive-schema`: RPC `--all-features --test black_box_parity -- --nocapture` | 3 discovered, 3 passed, 0 failed, 0 ignored | 9,790 ms |
| `phase161-remaining-workspace-preflight`: `cargo test --manifest-path packages/Cargo.toml --workspace --all-features --exclude open-bitcoin-node` | 2,596 passed, 0 failed, 0 ignored across 38 test/doctest result groups | 271,253 ms |
| `phase161-cli-alias-bin-preflight`: CLI `--bin open-bitcoin-cli --all-features` | 7 discovered, 7 passed, 0 failed, 0 ignored | 350 ms |
| `phase161-rpc-black-box-clippy`: RPC `--all-targets --all-features -- -D warnings` | Passed | 10,202 ms |
| Scoped Rustfmt and `git diff --check` | Passed | Passed |
| Parity breadcrumbs | 1,122 Rust paths verified | Passed |
| Live Phase161 guard | 28 function seams and six caller selectors checked | Passed |

The 2,596 total comprises 2,577 ordinary tests and nineteen doctests. Relevant groups include CLI library 410, binary integration 30, operator flows eleven, RPC library 348, daemon 87 and corrected black-box three. The explicit seven-test alias binary check covers the target omitted by the repository's normal verifier/default workspace selection. The existing long automatic-prune protection test completed normally; wrapper heartbeats, test progress and 99% CPU liveness were observed without terminating or timing out the command.

Late scope stayed restricted to the existing black-box test and this summary's inventory/appendix. No additional threat surface or production behavior was introduced. The full native verifier remains a mandatory parent gate; this preflight does not claim Native4 success, independent review/security sign-off or lifecycle closure. No staging, commit, push or shared tracker edit occurred.

### Late Self-Check: PASSED

The added existing test path is present and listed in frontmatter. Its exact black-box target and the broader remaining workspace preflight pass with nonzero discovery; the alias binary, strict RPC lint, scoped formatting, parity and live guard pass. The previous seven schema keys and block-relay authority assertion remain intact, and BASIC uses the same single authoritative capture.

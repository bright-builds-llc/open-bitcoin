---
phase: 110-block-serving-activation-and-eligibility-boundary
plan: 01
subsystem: network-rpc-config
tags: [block-serving, compact-relay, config, cli, eligibility, parity]
requires:
  - phase: 91-peer-permissions-and-connection-classes
    provides: peer connection classes and permission-effect labels
  - phase: 100-relay-activation-boundary-and-permission-semantics
    provides: pure relay activation policy pattern and config precedence model
provides:
  - default-off block-serving and compact-relay activation policy
  - deterministic peer block-serving eligibility matrix
  - Open Bitcoin JSONC and CLI activation settings wired into RuntimeConfig
affects: [phase-111, phase-112, phase-113, phase-114, phase-115, phase-116, phase-117, block-serving, compact-relay]
tech-stack:
  added: []
  patterns: [pure network policy, default-off operator activation, JSONC-to-RuntimeConfig mapping]
key-files:
  created:
    - .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-01-SUMMARY.md
    - packages/open-bitcoin-network/src/block_serving.rs
    - packages/open-bitcoin-network/src/block_serving/tests.rs
    - packages/open-bitcoin-rpc/src/config/loader/block_serving.rs
  modified:
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
key-decisions:
  - "Block serving and compact relay stay disabled by default across pure policy, JSONC, CLI resolution, and RuntimeConfig."
  - "Download-serving permission effects are eligibility inputs only; they do not activate serving or change public service advertisement."
  - "Transaction relay activation remains separate from block-serving activation."
patterns-established:
  - "Pure policy classifiers return stable reason labels and explicit no-public-service advertisement for deferred serving phases."
  - "Open Bitcoin-owned CLI flags map through small parser modules into runtime activation policy resolvers."
requirements-completed: [BSRV-01, BSRV-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 110-2026-07-04T02-39-48
generated_at: 2026-07-04T05:25:54Z
duration: 86m
completed: 2026-07-04
---

# Phase 110 Plan 01: Block-Serving Activation and Eligibility Summary

**Default-off block-serving and compact-relay activation now flow from Open Bitcoin config into a pure peer eligibility policy without changing service bits or transaction relay behavior.**

## Performance

- **Duration:** 86m
- **Started:** 2026-07-04T03:59:36Z
- **Completed:** 2026-07-04T05:25:54Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added `BlockServingActivationConfig`, `CompactRelayActivationConfig`, `BlockRelayActivationPolicy`, `BlockServingEligibilityInput`, `BlockServingEligibilityDecision`, `BlockServingEligibilityReason`, and `classify_block_serving_eligibility` in a pure network module.
- Covered outbound, manual, ordinary inbound, protected inbound, and permissioned inbound eligibility, including status availability, inactive permission labels, and stable reason labels.
- Verified permission expansions such as `download`, `noban`, `forceinbound`, and `all` remain policy inputs only and do not activate block serving, compact relay, or public service advertisement.
- Added Open Bitcoin JSONC fields `block_serving.enabled` and `block_serving.compact_relay_enabled`, plus CLI overrides `-openbitcoinblockserving` and `-openbitcoincompactrelay`.
- Wired block-serving activation into `RuntimeConfig.block_serving` without coupling it to existing transaction relay activation.
- Registered all new Rust files in parity breadcrumbs with Bitcoin Knots anchors.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add pure block-serving activation and eligibility policy** - `a4300cea`
2. **Task 2: Wire default-off activation into JSONC, CLI, and RuntimeConfig** - `dd12bc27`

## Validation Evidence

- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib block_serving -- --nocapture passed.
- cargo fmt --manifest-path packages/Cargo.toml --all passed.
- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings passed.
- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib block_serving -- --nocapture passed.
- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc -- --nocapture passed.
- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings passed.
- bun run scripts/check-parity-breadcrumbs.ts --check passed.
- bash scripts/check-file-lengths.sh passed after the CLI parser split.
- Plan acceptance `rg` probes passed for exported policy symbols, stable labels, permission expansion coverage, JSONC contract wiring, runtime config wiring, CLI resolver wiring, parity breadcrumbs, and absence of block-serving whitelist aliases.
- Repo-native commit hook verification passed through bash scripts/verify.sh for both task commits.

## Files Created/Modified

- `packages/open-bitcoin-network/src/block_serving.rs` - Pure activation and peer eligibility policy.
- `packages/open-bitcoin-network/src/block_serving/tests.rs` - Eligibility, default-off, permission-effect, service-bit, and stable-label coverage.
- `packages/open-bitcoin-network/src/lib.rs` - Public exports for block-serving policy types and classifier.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC `block_serving` section and conversion into activation policy.
- `packages/open-bitcoin-rpc/src/config.rs` - `RuntimeConfig.block_serving` field and config re-export.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - Loader integration for block-serving CLI parsing and runtime assignment.
- `packages/open-bitcoin-rpc/src/config/loader/block_serving.rs` - Open Bitcoin-owned block-serving CLI flag parser.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Config/CLI precedence resolver for `BlockRelayActivationPolicy`.
- `packages/open-bitcoin-rpc/src/config/tests.rs` - JSONC defaults, unknown-field rejection, CLI override, negation, and relay-separation tests.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb registrations for new Rust files.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metrics.

## Decisions Made

- The block-serving policy mirrors the existing relay policy shape but uses block-specific types and labels so later phases cannot accidentally reuse transaction relay activation.
- `PermissionEffectLabel::DownloadServingPolicyInput` is the only scoped inbound serving permission input for this plan.
- `advertises_public_service` is explicit and false in Phase 110 because public service-bit publication remains out of scope.
- CLI parsing was split into a child module to keep `loader.rs` inside the repository file-length guard.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split block-serving CLI parsing into a child module**

- **Found during:** Task 2 (Wire default-off activation into JSONC, CLI, and RuntimeConfig)
- **Issue:** Adding block-serving CLI parsing directly to `packages/open-bitcoin-rpc/src/config/loader.rs` exceeded the repo file-length guard.
- **Fix:** Created `packages/open-bitcoin-rpc/src/config/loader/block_serving.rs`, routed the loader through `mod block_serving`, and registered the new Rust file in parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-rpc/src/config/loader.rs`, `packages/open-bitcoin-rpc/src/config/loader/block_serving.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bash scripts/check-file-lengths.sh` and `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- **Committed in:** `dd12bc27`

**Total deviations:** 1 auto-fixed blocking issue.
**Impact on plan:** No behavior scope changed; the split kept the CLI contract intact while satisfying repo maintainability checks.

## Issues Encountered

- The delegated executor for this plan stopped making progress after staging Task 1 work, so the agent was shut down and the plan was completed locally from the staged implementation and plan contract.
- The first Task 2 commit attempt failed the file-length guard; the child parser module fixed it.
- A retry failed until the new child parser file had an explicit parity breadcrumb registration; the breadcrumb check passed after registration.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 111 can consume `RuntimeConfig.block_serving` and `classify_block_serving_eligibility` before any block storage read or peer response. Future phases must keep the current separation between block serving, compact relay, transaction relay, public service advertisement, and permission-effect inputs.

## Self-Check: PASSED

- [x] Block serving and compact relay default to disabled.
- [x] Peer eligibility is a pure deterministic policy.
- [x] Download-serving permission effects do not activate serving.
- [x] Transaction relay activation remains independent.
- [x] Service bits and public-service advertisement remain unchanged.

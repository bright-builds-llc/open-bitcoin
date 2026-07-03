---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 04
subsystem: docs-parity-uat
tags:
  - relay
  - parity
  - uat
  - runtime-activation
  - download-eligibility

requires:
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-01 pure relay download eligibility and typed scheduler suppressions
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-02 runtime relay activation propagation into managed network construction
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-03 sanitized relay activation and download eligibility status evidence
provides:
  - Machine-readable Phase 107 parity surface
  - Human-readable Phase 107 parity checklist row
  - Runtime activation/download eligibility architecture docs
  - Operator UAT commands for default-off, explicit relay-enabled, status, support, and verifier review
affects:
  - docs/parity/index.json
  - docs/parity/checklist.md
  - docs/operator/runtime-guide.md
  - Phase 107 checker wiring

tech-stack:
  added: []
  patterns:
    - Bounded release-facing docs for runtime integration repairs
    - Aggregate sanitized public relay evidence wording
    - Repo-local Cargo and Bazel UAT command pairs

key-files:
  created:
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md
  modified:
    - README.md
    - docs/architecture/config-precedence.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/catalog/rpc-cli-config.md
    - docs/parity/checklist.md
    - docs/parity/index.json

key-decisions:
  - "Describe Phase 107 as runtime activation/download eligibility integration, not public relay expansion."
  - "Keep granular scheduler labels internal while documenting aggregate sanitized counters as public/operator evidence."
  - "Use resolved config.inbound.enabled as the deterministic inbound-serving input and keep live listener/public-network proof outside default verification."
  - "Do not refresh docs/metrics/lines-of-code.md because the worktree LOC check reported it current."

patterns-established:
  - "Phase 107 parity roots include future checker and verification artifacts so Plan 107-05 can wire deterministic guardrails without changing the surface identity."
  - "Runtime UAT sections pair Cargo and Bazel forms for operator status and support workflows."

requirements-completed:
  - ACT-01
  - ACT-02
  - INV-02
  - INV-03
  - DL-01
  - DL-02
  - REL-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T04:53:55Z

duration: 22m
completed: 2026-07-03
---

# Phase 107 Plan 04: Runtime Relay Activation and Download Eligibility Integration Summary

**Phase 107 parity docs and UAT now describe runtime activation/download eligibility integration without expanding public relay claims.**

## Performance

- **Duration:** 22m
- **Started:** 2026-07-03T04:31:00Z
- **Completed:** 2026-07-03T04:53:55Z
- **Tasks:** 2
- **Files modified/created:** 11

## Accomplishments

- Registered `v2-0-runtime-relay-activation-download-eligibility` in `docs/parity/index.json` and `docs/parity/checklist.md` with `ACT-01`, `ACT-02`, `INV-02`, `INV-03`, `DL-01`, `DL-02`, and `REL-03`.
- Added Phase 107 bounded-claim wording to README, architecture docs, P2P, mempool-policy, and RPC/CLI/config catalogs.
- Added operator UAT commands for default-off status, explicit `-openbitcoinrelay=1`, `openbitcoinnetworkstatus`, support bundle review, and `bash scripts/verify.sh`.
- Preserved the no-claim boundary for public relay by default, compact block relay, package relay, bloom/filter serving, public-network relay CI, production service operation, production full-node readiness, production-funds wallet safety/use, and durable mempool recovery.

## Task Commits

No commits were created. The execution request explicitly instructed this executor not to commit or push.

1. **Task 1: Register Phase 107 parity roots and bounded release docs** - complete, not committed here.
2. **Task 2: Add operator UAT guidance for activation and eligibility evidence** - complete, not committed here.

## Files Created/Modified

- `README.md` - Adds Phase 107 runtime integration wording to the top status and operator preview boundary.
- `docs/architecture/config-precedence.md` - Documents resolved `RuntimeConfig.relay` and `config.inbound.enabled` as deterministic managed construction inputs.
- `docs/architecture/status-snapshot.md` - Adds Phase 107 shared status contract language for activation and aggregate eligibility counters.
- `docs/architecture/operator-observability.md` - Adds aggregate sanitized evidence rules for activation/download eligibility.
- `docs/operator/runtime-guide.md` - Adds copy-pasteable Cargo and Bazel UAT commands for default-off, enabled relay, network status, support bundle, and verifier review.
- `docs/parity/catalog/p2p.md` - Adds the Phase 107 parity surface, Knots anchors, evidence roots, and no-claim language.
- `docs/parity/catalog/mempool-policy.md` - Clarifies that Phase 107 does not change mempool admission or persistence behavior.
- `docs/parity/catalog/rpc-cli-config.md` - Adds Phase 107 RPC/CLI UAT and `sendrawtransaction` no-propagation wording.
- `docs/parity/checklist.md` - Adds the human-readable Phase 107 parity row.
- `docs/parity/index.json` - Adds the machine-readable Phase 107 surface and evidence roots.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md` - Records this execution.

## Decisions Made

- No source breadcrumb edit was needed in this plan; `docs/parity/source-breadcrumbs.json` already parsed and retained the Plan 107-01 registration for `peer/relay_download.rs`.
- `docs/metrics/lines-of-code.md` was not regenerated because `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` reported it current.
- The Phase 107 checker and `107-VERIFICATION.md` were included as planned evidence roots for Plan 107-05/107-06 without creating those artifacts in Plan 107-04.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Known Stubs

None. A targeted scan of Plan 107-04 modified docs found no `TODO`, `FIXME`, `placeholder`, or `coming soon` markers.

## Threat Flags

None. This plan changes documentation and parity roots only; it adds no network endpoint, auth path, file-access trust boundary, schema change, service-bit change, compact block behavior, package relay, bloom/filter serving, public relay default, or durable mempool recovery behavior.

## Verification

- `node -e "JSON.parse(require('fs').readFileSync('docs/parity/index.json','utf8')); JSON.parse(require('fs').readFileSync('docs/parity/source-breadcrumbs.json','utf8'));"` - passed.
- `rg -n "v2-0-runtime-relay-activation-download-eligibility|ACT-01|ACT-02|INV-02|INV-03|DL-01|DL-02|REL-03" docs/parity/index.json docs/parity/checklist.md docs/parity/catalog/p2p.md docs/operator/runtime-guide.md` - passed.
- `rg -n "sendrawtransaction.*does not guarantee public propagation|public relay by default|compact block relay|package relay|bloom/filter|production full-node readiness|production-funds" README.md docs/architecture/config-precedence.md docs/architecture/status-snapshot.md docs/architecture/operator-observability.md docs/parity/catalog/p2p.md docs/parity/catalog/mempool-policy.md docs/parity/catalog/rpc-cli-config.md` - passed.
- `rg -n "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin|bazel run //packages/open-bitcoin-cli:open_bitcoin|openbitcoinnetworkstatus|bash scripts/verify.sh" docs/operator/runtime-guide.md` - passed.
- `rg -n "aggregate|sanitized|fixed labels|public-network|default verification|config\\.inbound\\.enabled" docs/operator/runtime-guide.md docs/architecture/operator-observability.md docs/architecture/status-snapshot.md` - passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` - passed; report was current.
- `git diff --check` - passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 107-05 can wire the deterministic Phase 107 checker against the registered surface, docs, UAT commands, and bounded no-claim vocabulary created here.

## Self-Check: PASSED

- Created summary file: `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md`
- Verified `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` parse.
- Verified the Phase 107 surface and all seven requirement IDs appear in machine, human, catalog, and UAT roots.
- Verified the Plan 107-04 diff has no whitespace errors.
- No commits were created, matching the execution request.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*

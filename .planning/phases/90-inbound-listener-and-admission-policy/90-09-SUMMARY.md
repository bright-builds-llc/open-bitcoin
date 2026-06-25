---
phase: 90-inbound-listener-and-admission-policy
plan: 09
subsystem: docs-parity
tags: [docs, parity, inbound, operator-uat, evidence]

requires:
  - phase: 90-04
    provides: Runtime listener adapter and loopback admission evidence
  - phase: 90-07
    provides: Operator status rendering for inbound evidence
  - phase: 90-08
    provides: Bounded support-bundle rendering for inbound evidence
provides:
  - Phase 90 loopback inbound listener operator UAT commands
  - Architecture docs for inbound config, status, metrics, logs, RPC, and support evidence
  - P2P parity surface root for INB-01 through INB-05
  - Human and machine parity checklist registration for v1.9 inbound listener/admission evidence
affects:
  - phase-90-final-verification
  - phase-91-peer-permissions
  - phase-95-network-participation-boundary

tech-stack:
  added: []
  patterns:
    - Documentation keeps repo-local Cargo and Bazel command forms side by side.
    - Parity roots separate opt-in listener/admission evidence from later relay, permission, address, eviction, ban, DoS, public-default, and production-readiness claims.

key-files:
  created:
    - .planning/phases/90-inbound-listener-and-admission-policy/90-09-SUMMARY.md
  modified:
    - docs/architecture/config-precedence.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - docs/parity/index.json
    - docs/parity/checklist.md

key-decisions:
  - "Used loopback-first regtest UAT commands with both Cargo and Bazel forms for daemon, RPC, status, and support workflows."
  - "Registered the Phase 90 parity surface without changing Rust source files; `docs/parity/source-breadcrumbs.json` already contained the required Phase 90 file mappings and was verified unchanged."
  - "Skipped `.planning/STATE.md` and `.planning/ROADMAP.md` updates because the orchestrator owns shared state for this run."

patterns-established:
  - "Inbound evidence docs name stable status/log labels and keep detailed listener/admission evidence on Open Bitcoin-owned surfaces."
  - "Parity checklist rows for v1.9 surfaces carry explicit no-claim boundaries for later P2P governance phases."

requirements-completed: [INB-01, INB-02, INB-03, INB-04, INB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T08:44:51Z

duration: 9 min
completed: 2026-06-25
---

# Phase 90 Plan 09: Operator, Parity, and Breadcrumb Evidence Summary

**Loopback-first inbound listener operator docs and parity roots for INB-01 through INB-05 without expanding relay, public-default, or production-readiness claims**

## Performance

- **Duration:** 9 min
- **Started:** 2026-06-25T08:36:08Z
- **Completed:** 2026-06-25T08:44:51Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added Phase 90 loopback inbound listener review guidance with JSONC config, daemon CLI flags, Cargo/Bazel daemon commands, `getnetworkinfo`, `openbitcoinnetworkstatus`, `status --format json`, and support-bundle commands.
- Documented inbound config ownership, status ownership, RPC extension evidence, low-cardinality metrics, structured log labels, and bounded/redacted support-bundle evidence.
- Registered `v1-9-inbound-listener-admission-policy` in the P2P catalog, machine parity index, and human checklist with INB-01 through INB-05.
- Verified existing source breadcrumb mappings for the Phase 90 Rust files without modifying source code or the breadcrumb registry.

## Task Commits

1. **Task 1: Add operator docs and repo-local UAT commands** - `72af0c4` (docs)
2. **Task 2: Register parity roots and source breadcrumbs** - `d21bdc9` (docs)

## Files Created/Modified

- `docs/operator/runtime-guide.md` - Adds Phase 90 loopback inbound listener review commands and boundaries.
- `docs/architecture/config-precedence.md` - Records `inbound.*` JSONC ownership and Open Bitcoin-prefixed daemon overrides.
- `docs/architecture/status-snapshot.md` - Defines `OpenBitcoinStatusSnapshot.peers.inbound` as shared inbound evidence.
- `docs/architecture/operator-observability.md` - Adds inbound metric/log/support evidence labels and low-cardinality constraints.
- `docs/parity/catalog/p2p.md` - Adds the Phase 90 P2P surface, Knots anchors, implementation links, and known gaps.
- `docs/parity/index.json` - Registers the Phase 90 surface in machine-readable parity roots.
- `docs/parity/checklist.md` - Adds the human-readable Phase 90 checklist row.
- `docs/parity/source-breadcrumbs.json` - Verified unchanged; required Phase 90 mappings were already present.

## Decisions Made

- Kept public or wildcard listener review explicitly opt-in and outside `bash scripts/verify.sh`.
- Kept detailed inbound listener/admission evidence on Open Bitcoin-owned status/RPC/support surfaces instead of changing baseline-shaped `getnetworkinfo` fields.
- Preserved the user-requested no-source-code boundary; no Rust source files were modified to adjust breadcrumb comments.
- Did not update shared planning state files because the orchestrator owns `.planning/STATE.md` and `.planning/ROADMAP.md`.

## Verification

- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 276 Rust files.
- Required Task 1 `rg` scan passed for inbound config keys, daemon flags, RPC/status/support commands, status labels, and Cargo/Bazel command forms.
- Required Task 2 `rg` scan passed for `v1-9-inbound-listener-admission-policy`, INB-01 through INB-05, `net.cpp`, `net_processing.cpp`, and `p2p_handshake.py`.
- `node -e ... docs/parity/index.json ...` confirmed the new surface exists in `surfaces`, `checklist.surfaces`, and the audit lookup map.
- No-claim scan passed for the plan's forbidden overclaim phrases across the modified docs.
- `git diff --check` passed for the modified docs.
- Stub scan found no stub or placeholder patterns in the modified docs.

## Deviations from Plan

None - plan executed within the documented owner boundaries. The source breadcrumb registry already contained the required Phase 90 Rust file mappings, so no committed registry change was needed.

## Issues Encountered

- A trial breadcrumb-registry edit would have required matching Rust source breadcrumb comment changes. Because the plan explicitly forbids source-code modifications, the uncommitted JSON edit was reverted and the existing valid mappings were preserved. The final breadcrumb checker passed.

## Known Stubs

None - stub and placeholder scans found no matches in the modified docs.

## Threat Flags

None. This plan changed documentation and parity registration only; it did not add new runtime endpoints, auth paths, file access patterns, or schema trust boundaries.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## State Updates

Skipped intentionally. The user instructed that `.planning/STATE.md` and `.planning/ROADMAP.md` are orchestrator-owned for this run.

## Next Phase Readiness

Ready for Phase 90 final verification and Phase 91 planning. Operator and parity docs now expose Phase 90 inbound listener/admission evidence while keeping permission classes, address advertisement, eviction/ban policy, resource governance, public listener defaults, relay behavior, and production readiness for later scoped phases.

## Self-Check: PASSED

- Found all seven modified documentation/parity files.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-09-SUMMARY.md`.
- Found task commit `72af0c4`.
- Found task commit `d21bdc9`.

---

*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*

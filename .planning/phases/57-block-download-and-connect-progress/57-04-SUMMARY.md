---
phase: 57-block-download-and-connect-progress
plan: 04
subsystem: sync
tags: [typescript, live-smoke, block-download, operator-docs, parity]

requires:
  - phase: 57-block-download-and-connect-progress
    plan: 03
    provides: durable downloaded and connected block height/hash status fields
provides:
  - firstBlockProgress live-smoke report schema and Markdown rendering
  - connected-block-only Phase 57 pass condition
  - block-specific no-progress diagnoses for no-credit peer responses
  - operator and parity docs for bounded daemon block download/connect evidence
affects: [live-smoke, operator-status, p2p-parity, BLK-03, BLK-04]

tech-stack:
  added: []
  patterns:
    - live-smoke reports keep header/download-only evidence without treating it as block-connect pass evidence
    - durable peer failure strings map to operator-facing no-progress causes

key-files:
  created:
    - .planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh

key-decisions:
  - "Treat connected block height increase as the only Phase 57 live-smoke pass condition."
  - "Preserve downloaded-only and header-only evidence, but report it as `awaiting_blocks` until active chainstate advances."
  - "Map durable `disconnected_block`, `duplicate_block`, and `non_extending_block` peer reasons to the shared `duplicate_or_disconnected_block` diagnosis."
  - "Keep public-network smoke outside `bash scripts/verify.sh` and document it as opt-in operator evidence."

patterns-established:
  - "Live-smoke status snapshots now carry header, downloaded block, connected block, and optional hash evidence together."
  - "No-credit block response paths stay visible in live-smoke reports without granting useful progress."

requirements-completed: [BLK-03, BLK-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 57-2026-06-03T13-56-54
generated_at: 2026-06-04T10:08:27Z

duration: ~55m
completed: 2026-06-04
---

# Phase 57 Plan 04: Live-smoke Block Evidence Summary

**Live-smoke reports now prove first connected block progress or return a typed block-progress diagnosis.**

## Performance

- **Duration:** ~55m
- **Tasks:** 2
- **Files created/modified:** 5

## Accomplishments

- Added `result.firstBlockProgress` with downloaded/connected kind, height, block hash, peer/source/endpoint labels, timestamp, and before/after status snapshots.
- Changed live-smoke pass criteria so Phase 57 passes only when connected block height increases; downloaded-only and header-only progress are retained as evidence with `awaiting_blocks`.
- Added block-specific no-progress causes for `peer_notfound`, `malformed_block`, `invalid_block`, `duplicate_or_disconnected_block`, and `resource_limit`.
- Expanded deterministic live-smoke fixtures for connected pass, downloaded-only no-progress, header-only no-progress, and all listed block peer failure mappings.
- Updated operator and P2P parity docs with bounded block download/connect claims and opt-in public-network scope boundaries.

## Task Commits

1. **Task 1: Add firstBlockProgress and block-specific diagnoses to live smoke** - `8a20aeb`
2. **Task 2: Document bounded Phase 57 evidence without broadening scope** - `095cd2a`

## Verification

Passed:

```bash
bash scripts/test-run-live-mainnet-smoke.sh
bun run scripts/run-live-mainnet-smoke.ts --help
rg -n "type FirstBlockProgressEvidence|firstBlockProgressEvidence|downloadedBlockHeight|connectedBlockHeight|awaiting_blocks|peer_notfound|disconnected_block|duplicate_or_disconnected_block|resource_limit" scripts/run-live-mainnet-smoke.ts
rg -n "\"firstBlockProgress\"|\"kind\": \"connected\"|\"maybeNoProgressCause\": \"awaiting_blocks\"|\"disconnected_block\"|\"duplicate_or_disconnected_block\"|First block progress" scripts/test-run-live-mainnet-smoke.sh
rg -n "result\\.firstBlockProgress|downloaded-only evidence|awaiting_blocks|peer_notfound|malformed_block|invalid_block|duplicate_or_disconnected_block|disconnected_block|resource_limit" docs/operator/runtime-guide.md
rg -n "bounded daemon block download|first validated block connect|getdata.*notfound|disconnected no-credit|public-network checks remain opt-in" docs/parity/catalog/p2p.md
```

Both task commits also completed the repo pre-commit hook successfully, including `bash scripts/verify.sh`.

## Deviations from Plan

- None. Plan 04 was executed locally after the prior executor reached its usage limit; scope stayed within live-smoke reporting and docs.

## Issues Encountered

- No functional blockers remain for Plan 04.

## Next Phase Readiness

Phase 57 can now be verified as a whole. Phase 58 can build restart/resume evidence on top of the durable connected/downloaded status fields and the live-smoke report structure without changing Phase 57 pass semantics.

## Self-Check: PASSED

- `FOUND:8a20aeb`
- `FOUND:095cd2a`
- `FOUND:.planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md`
- `PASS:bash scripts/test-run-live-mainnet-smoke.sh`
- `PASS:bun run scripts/run-live-mainnet-smoke.ts --help`
- `PASS:docs/operator/runtime-guide.md markers`
- `PASS:docs/parity/catalog/p2p.md markers`

---
*Phase: 57-block-download-and-connect-progress*
*Completed: 2026-06-04*

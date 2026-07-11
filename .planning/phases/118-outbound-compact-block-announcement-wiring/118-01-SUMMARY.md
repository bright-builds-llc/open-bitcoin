---
phase: 118-outbound-compact-block-announcement-wiring
plan: 01
subsystem: consensus
tags: [bip152, compact-block, short-id, wtxid, knots-parity]

requires:
  - phase: 112-bip152-wire-codec-and-message-semantics
    provides: CompactBlockPayload encode/decode and structural validation
  - phase: consensus-crypto
    provides: compact_short_id_for_wtxid and transaction_wtxid helpers
provides:
  - Pure build_compact_block_payload(block, nonce) with Knots coinbase-only announce shape
  - Unit coverage for coinbase-only, multi-tx short IDs, empty-tx Err, encode/decode round-trip
  - Parity breadcrumb registry entry consensus-compact-block-build
affects:
  - 118-02 PeerManager announce_block_with_action emission
  - 118-03 ManagedPeerNetwork evidence-after-emit wiring

tech-stack:
  added: []
  patterns:
    - Pure functional-core Block→CompactBlockPayload builder in open-bitcoin-consensus
    - Knots announce shape: coinbase-only prefill + wtxid short IDs

key-files:
  created:
    - packages/open-bitcoin-consensus/src/compact_block_build.rs
    - packages/open-bitcoin-consensus/src/compact_block_build/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/lib.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Builder lives in open-bitcoin-consensus beside short-ID helpers (D-03 / RESEARCH)"
  - "Empty transactions return CodecError::CompactBlockEmpty; no unwrap/panic"
  - "Parity breadcrumbs registered at first file add because pre-commit verify requires them"

patterns-established:
  - "Production compact announce construction is pure and nonce-injected by the caller"
  - "Never prefill-all transactions on the announce path"

requirements-completed: []  # CMP-05 spans Plans 02–03; left Pending
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T17:40:21.000Z

duration: 40min
completed: 2026-07-11
---

# Phase 118 Plan 01: Outbound Compact Block Builder Summary

**Pure Knots-shaped `build_compact_block_payload` in consensus: coinbase-only prefill, wtxid short IDs, typed empty-tx Err**

## Performance

- **Duration:** 40 min
- **Started:** 2026-07-11T16:59:23Z
- **Completed:** 2026-07-11T17:40:21Z
- **Tasks:** 2/2
- **Files modified:** 5

## Accomplishments

- Shipped production `build_compact_block_payload(block, nonce)` matching Knots `CBlockHeaderAndShortTxIDs` announce construction.
- Unit tests prove coinbase-only prefill, multi-tx short-ID SipHash equality, empty-transaction typed Err, and encode/decode round-trip.
- Registered `consensus-compact-block-build` parity breadcrumbs against `blockencodings.cpp` and `net_processing.cpp`.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: RED verified locally, GREEN committed** - `e4a60f03` (feat)
   - RED suite observed at 3 failed / 1 passed against the stub (`/tmp/118-01-red.log`)
   - Separate RED commit blocked by pre-commit `verify.sh` (requires green tests + rustfmt)
   - GREEN implementation + tests + breadcrumbs landed together

**Plan metadata:** pending final docs commit

_Note: TDD RED was executed and verified; atomic RED commit could not land under repo hooks._

## Files Created/Modified

- `packages/open-bitcoin-consensus/src/compact_block_build.rs` - Pure builder API
- `packages/open-bitcoin-consensus/src/compact_block_build/tests.rs` - Knots-shaped unit coverage
- `packages/open-bitcoin-consensus/src/lib.rs` - Module + public re-export
- `docs/parity/source-breadcrumbs.json` - `consensus-compact-block-build` registry entry
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC freshness

## Decisions Made

- Builder placed in `open-bitcoin-consensus` (not codec/network) so short-ID/wtxid helpers stay co-located.
- Empty blocks fail with `CodecError::CompactBlockEmpty` before any payload assembly.
- `validate_compact_block_structure` runs before every `Ok` return.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Parity breadcrumbs required before RED commit**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit `verify.sh` rejected new Rust files without `docs/parity/source-breadcrumbs.json` mappings.
- **Fix:** Added `consensus-compact-block-build` registry entry during Task 1 (plan had breadcrumbs only in Task 2).
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** Parity breadcrumb checker passed in pre-commit
- **Committed in:** `e4a60f03`

**2. [Rule 3 - Blocking] Separate RED commit blocked by full verify**
- **Found during:** Task 1 commit
- **Issue:** Repo pre-commit runs full `verify.sh`, which requires passing tests and rustfmt; a stub-only RED commit cannot land.
- **Fix:** Verified RED failures locally, then shipped tests + implementation in one `feat(118-01)` commit after GREEN passed.
- **Files modified:** same builder/test/lib files
- **Verification:** `cargo test -p open-bitcoin-consensus -- compact_block_build` → 4 passed; pre-commit verify completed
- **Committed in:** `e4a60f03`

**3. [Rule 2 - Correctness] Keep CMP-05 Pending until Plans 02–03**
- **Found during:** State/requirements update after Task 2
- **Issue:** Plan frontmatter lists `requirements: [CMP-05]`, but CMP-05 is the full announce-wire seam across Plans 01–03.
- **Fix:** Left CMP-05 unchecked/Pending in REQUIREMENTS.md after Plan 01 (builder only).
- **Files modified:** `.planning/REQUIREMENTS.md`
- **Verification:** Checklist and traceability show Pending for Phase 118
- **Committed in:** docs metadata commit

**Total deviations:** 3 auto-fixed (Rule 3 × 2, Rule 2 × 1)
**Impact on plan:** No scope creep; CMP-05 construction deliverable unchanged. TDD was followed locally; commit granularity adapted to hooks. Requirement completion deferred to phase closeout.

## Issues Encountered

- First commit attempt failed rustfmt import order (`pub use compact_block_build` must precede `context`); fixed before GREEN commit.
- Pre-commit verify takes ~13–24 minutes; waited and retried without `--no-verify`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 118-02 can call `build_compact_block_payload` from `PeerManager::announce_block_with_action`.
- Nonce selection and evidence-after-emit remain Plans 02/03.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-consensus/src/compact_block_build.rs`
- FOUND: `packages/open-bitcoin-consensus/src/compact_block_build/tests.rs`
- FOUND: `docs/parity/source-breadcrumbs.json` entry `consensus-compact-block-build`
- FOUND: commit `e4a60f03`

---
*Phase: 118-outbound-compact-block-announcement-wiring*
*Completed: 2026-07-11*

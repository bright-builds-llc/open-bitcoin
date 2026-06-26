---
phase: 92-address-advertisement-and-discovery-boundaries
plan: 01
subsystem: networking
tags: [rust, open-bitcoin-network, address-advertisement, parity-breadcrumbs]

requires:
  - phase: 90-inbound-listener-admission
    provides: Typed inbound listener endpoint evidence
provides:
  - Typed Phase 92 address boundary vocabulary
  - Pure local listener advertisement selection
  - Version sender address gating from advertisement candidates
  - Address boundary parity breadcrumb registration
affects: [phase-92-address-book, phase-92-getaddr, network-status, operator-cli]

tech-stack:
  added: []
  patterns:
    - Pure data-in/data-out network address policy
    - Stable `as_str()` label enums for later status and support surfaces

key-files:
  created:
    - packages/open-bitcoin-network/src/address.rs
    - packages/open-bitcoin-network/src/address/advertisement.rs
    - packages/open-bitcoin-network/src/address/tests.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Local advertisement candidates use only listener endpoint and runtime-bound listener evidence."
  - "Version sender address selection reuses the same accepted advertisement decision."
  - "Private, local, documentation, multicast, unspecified, and unsupported privacy-network inputs stay suppressed with stable reasons."

patterns-established:
  - "Address policy exposes typed evidence, reason, and label fields rather than stringly status fragments."
  - "Discovery-adjacent surfaces remain outside the pure local advertisement boundary."

requirements-completed: [ADDR-01, ADDR-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: 2026-06-26T05:49:26Z

duration: 23m 09s
completed: 2026-06-26
---

# Phase 92 Plan 01: Address Advertisement Boundaries Summary

**Pure listener-based address advertisement policy with stable Phase 92 labels and conservative version sender gating**

## Performance

- **Duration:** 23m 09s
- **Started:** 2026-06-26T05:26:17Z
- **Completed:** 2026-06-26T05:49:26Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added the shared address boundary vocabulary for network kind, routability, source, decision label, and decision reason.
- Implemented local listener advertisement selection from `InboundListenerEndpoint` and optional runtime-bound `SocketAddr` evidence.
- Kept version sender address selection empty unless the same advertisement policy returns an accepted candidate.
- Registered Phase 92 address policy files in parity breadcrumbs and refreshed the tracked LOC artifact through hooks.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create typed address boundary contracts** - `6cb8b91` (`feat`)
2. **Task 2: Implement local listener advertisement selection** - `3065950` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/address.rs` - Address boundary types, classifier, exports, and child policy module.
- `packages/open-bitcoin-network/src/address/advertisement.rs` - Pure local listener advertisement and version sender selection.
- `packages/open-bitcoin-network/src/address/tests.rs` - Deterministic routability, suppression, advertisement, and sender-gating tests.
- `packages/open-bitcoin-network/src/lib.rs` - Crate-level re-exports for the new address boundary API.
- `docs/parity/source-breadcrumbs.json` - `network-address-boundaries` mapping for Phase 92 Rust files.
- `docs/metrics/lines-of-code.md` - Hook-managed generated LOC freshness update.

## Decisions Made

- Local advertisement is deliberately bounded to configured listener endpoints and runtime-bound listener evidence.
- Public listener evidence requires `allow_public=true`; otherwise a public candidate is suppressed with `permission_policy_denied`.
- Version messages get a sender address only from `advertise_candidate`, preserving the zero-address behavior when no candidate passes.

## Deviations from Plan

### Execution Adjustments

**1. AGENTS-driven TDD commit adjustment**
- **Found during:** Task 1 and Task 2 TDD flow
- **Issue:** The plan requested TDD commits, but repo rules require format, clippy, build, and tests before every commit, and the user required normal hooks.
- **Adjustment:** RED tests were written and run to a failing signal, then kept uncommitted until the corresponding GREEN implementation passed verification.
- **Impact:** Preserved TDD evidence without creating commits that violate repo pre-commit requirements.

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added coverage for hook-required uncovered branches**
- **Found during:** Task 1 commit hook
- **Issue:** `scripts/verify.sh` coverage rejected uncovered invalid-port and unsupported-future-network branches.
- **Fix:** Added tests for zero-port inputs and unsupported future-network classification.
- **Files modified:** `packages/open-bitcoin-network/src/address/tests.rs`
- **Verification:** Focused address tests, full workspace tests, and normal commit hooks passed.
- **Committed in:** `6cb8b91`

**Total deviations:** 1 execution adjustment, 1 auto-fixed blocking issue
**Impact on plan:** No scope creep. The added tests were required to satisfy repo verification and strengthen the planned boundary coverage.

## Issues Encountered

- The repo has no root `Cargo.toml`, so Rust verification used `--manifest-path packages/Cargo.toml`, matching the repo-local workspace layout.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network address --no-fail-fast`
- `bun run scripts/check-parity-breadcrumbs.ts --write`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- Forbidden discovery-term scan for `address/advertisement.rs`
- Normal git hooks ran `scripts/verify.sh` for both task commits.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Later Phase 92 plans can reuse the exported address vocabulary and `LocalAdvertisementDecision` shape for address book, getaddr, status, CLI, and docs work. No blockers remain from this plan.

## Self-Check: PASSED

- Found summary file and all created/modified plan files.
- Verified task commits `6cb8b91` and `3065950` exist.
- Confirmed `STATE.md` and `ROADMAP.md` were not modified by this executor; the only unrelated dirty file remains pre-existing `.planning/config.json`.

---
*Phase: 92-address-advertisement-and-discovery-boundaries*
*Completed: 2026-06-26*

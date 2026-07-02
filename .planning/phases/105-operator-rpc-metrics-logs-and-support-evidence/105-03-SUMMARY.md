---
phase: 105-operator-rpc-metrics-logs-and-support-evidence
plan: 105-03
subsystem: support-redaction
tags:
  - rust
  - relay-evidence
  - support-bundles
  - redaction
requires:
  - 105-01
  - 105-02
provides:
  - Support bundle relay and mempool evidence rendering from the shared sanitized status contract.
  - Relay-specific support redaction for transaction, peer, endpoint, permission, credential, cookie, secret, suspicious-hex, and dynamic-label material.
  - Shared JSON and Markdown support projection with bounded local troubleshooting and parity-review guidance.
affects:
  - support-bundles
  - operator-status
  - parity-breadcrumbs
tech-stack:
  added: []
  patterns:
    - Support bundles consume sanitized `MempoolStatus.relay` evidence instead of reconstructing relay internals.
    - Relay support rendering lives in a child module to keep support rendering below the file-length guard.
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/render/relay.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Support bundle relay evidence is rendered from the same sanitized support status projection used by JSON output."
  - "Relay-specific redaction treats suspicious relay reason strings as sensitive while preserving safe implemented/unavailable/deferred field states."
  - "Markdown support guidance explicitly limits relay and mempool evidence to bounded local status and local troubleshooting/parity-review evidence."
patterns-established:
  - "Support redaction sanitizes relay evidence before both JSON serialization and Markdown rendering."
  - "Relay support guidance carries the D-15 no-claim boundary in one renderer so support bundles do not imply public propagation or production readiness."
requirements-completed:
  - OBS-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-02T01:19:32Z
duration: 27m
completed: 2026-07-02
---

# Phase 105 Plan 03: Support Redaction Summary

**Support bundles now render relay and mempool evidence through the shared sanitized status projection, with relay-specific redaction before JSON or Markdown output can persist support evidence.**

## Performance

- **Duration:** 27m
- **Started:** 2026-07-02T00:52:00Z
- **Completed:** 2026-07-02T01:19:32Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added a support bundle `Relay and Mempool Evidence` Markdown section sourced from `MempoolStatus` and `RelayEvidenceStatus`.
- Extended support redaction to sanitize relay outcome, mempool admission, local submission, fanout, serving, rebroadcast, and public-relay reason fields before bundle rendering.
- Added relay-specific safeguards for raw transaction hex, txids, wtxids, peer endpoints, socket-address evidence, peer identifiers, permission strings, credentials, cookies, secrets, suspicious hex, and dynamic labels.
- Preserved implemented relay evidence counters in fixed fields while replacing sensitive relay reasons with `redacted_relay_mempool_evidence`.
- Added support tests proving JSON and Markdown share the sanitized projection and that Markdown carries the full bounded local troubleshooting and parity-review next-action guidance.

## Task Commits

Plan 105-03 was committed as one verification-backed implementation commit:

1. **Tasks 105-03-01 through 105-03-02: Support redaction and summary roots** - `b079d23e` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support/render/relay.rs` - Renders support Markdown relay and mempool evidence from sanitized shared status fields.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Delegates the relay/mempool support section to the focused child module.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Adds relay-specific redaction of sensitive reason strings and updates redaction safeguards.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Covers shared support projection, D-15 bounded guidance, and the full OBS-04 sensitive-material set.
- `docs/parity/source-breadcrumbs.json` - Records the new support relay renderer as Open Bitcoin-only support infrastructure.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metrics after Rust source changes.

## Commands Run

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support -- --nocapture`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/check-file-lengths.sh`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `git diff --check`
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check`
- Pre-commit hook via `git commit`, including `bash scripts/verify.sh`, passed.

## Decisions Made

- Relay support redaction runs in `support_status_for_bundle` before rendering so JSON and Markdown cannot diverge on sensitive relay evidence.
- Safe implemented fields and counters remain visible because they are fixed, low-cardinality evidence; free-form reason strings receive relay-specific inspection.
- The support Markdown next action names bounded local status and local troubleshooting/parity-review evidence while excluding public propagation, compact-block relay, release-validation, public-network proof, production-service proof, production full-node readiness, and production-funds wallet safety.
- The renderer split follows the existing `foo.rs` plus `foo/` Rust module pattern and keeps `support/render.rs` within the repo file-length guard.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated the existing redaction summary expectation after expanding omitted relay classes**
- **Found during:** Task 105-03-01 (Sanitize relay and mempool evidence in support bundles)
- **Issue:** The support redaction summary test still expected the previous omitted-material wording and safeguard list.
- **Fix:** Updated the summary expectation to include raw transaction identifiers, permission strings, credentials, and dynamic relay labels plus the new relay/mempool safeguard.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/tests.rs`
- **Verification:** Focused support tests, full cargo verification, and the pre-commit hook passed.
- **Committed in:** `b079d23e`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The update keeps the redaction summary aligned with the stricter OBS-04 support-bundle contract.

## Issues Encountered

- Focused support tests initially surfaced the stale redaction summary expectation described above; the expectation was updated and the focused rerun passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 105-04 can use `105-01-SUMMARY.md`, `105-02-SUMMARY.md`, and this support-redaction summary as evidence roots for docs, parity records, the Phase 105 checker, and final closeout.

## Self-Check

- Complete: OBS-04 evidence is implemented, tested, summarized, and committed in `b079d23e`.
- Passed: focused support tests, full cargo verification, parity breadcrumb check, file-length check, diff whitespace check, LOC freshness check, and hook-backed repository verification all passed.

*Phase: 105-operator-rpc-metrics-logs-and-support-evidence*
*Completed: 2026-07-02*

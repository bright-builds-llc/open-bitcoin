---
phase: 126-compact-relay-residual-hardening
plan: "01"
subsystem: compact-relay
tags: [bip152, compact-blocks, network-adapters, entropy, rust]
requires:
  - phase: 119
    provides: live mempool and bounded extra-transaction compact receive candidates
  - phase: 122
    provides: compact announcement provenance and serving evidence
provides:
  - peer-neutral fail-closed generic compact-block dispatch
  - explicit fact construction for every factful compact receive path
  - lazy fallible node-shell compact announcement entropy
  - truthful safe fallback, provenance, and evidence semantics
affects: [126-02, compact-download, compact-relay, network-adapters]
tech-stack:
  added: [getrandom 0.3.4]
  patterns: [explicit adapter facts, lazy fallible entropy closure, achieved-effect evidence]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/error.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/Cargo.toml
    - packages/open-bitcoin-node/BUILD.bazel
    - packages/Cargo.lock
    - MODULE.bazel.lock
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Treat factless generic CompactBlock dispatch as a local peer-neutral routing error before reconstruction."
  - "Acquire nonce entropy only after AnnounceCompactBlock is selected and reuse the peer's existing headers/inventory fallback when entropy fails."
  - "Record compact provenance and achieved-effect evidence only from an actually emitted CompactBlock."
requirements-completed: []
duration: 31m
completed: 2026-07-18
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T19:38:08Z
---

# Phase 126 Plan 01: Compact Relay Runtime Seam Hardening Summary

Compact receive adapters now require explicit live facts, while compact announcements acquire a random nonce lazily in the node shell and fall back truthfully without false compact provenance or evidence.

## What Changed

- Added `NetworkError::CompactBlockReceiveFactsRequired` as a stable peer-neutral routing-contract error.
- Made generic `PeerManager::handle_message` reject `CompactBlock` before reconstruction instead of inventing empty candidate facts.
- Removed `Default` from `CompactBlockReceiveFacts` and converted test callers to explicit candidate and extra-transaction slices.
- Preserved both managed receive entrypoints as the production shells that intercept compact blocks and call the factful receive seam.
- Added a private lazy `FnOnce` nonce seam after compact announcement action selection.
- Replaced deterministic block-hash-derived nonces with eight bytes from `getrandom::fill`, converted using little-endian ordering.
- Kept headers, inventory, and suppression paths entropy-free; entropy failure reuses the peer's existing safe headers/inventory announcement behavior.
- Based compact provenance and announcement evidence only on the actual outbound message.
- Declared `getrandom 0.3.4` consistently in Cargo and Bazel and refreshed generated Cargo/Bazel lock metadata.

## Task Commits

| Task | Commit | Result |
| --- | --- | --- |
| 1. Make compact receive routing fail closed | `5106f5ad` | Added peer-neutral routing failure, removed default facts, and preserved explicit managed receive paths |
| 2. Generate compact announcement nonces lazily in the node shell | `a553174b` | Added fallible OS entropy, safe fallback, achieved-effect accounting, and matching build dependencies |

## Verification

| Check | Result |
| --- | --- |
| Network compact-block focused tests | 29 passed, 0 failed |
| Node compact receive focused tests | 55 passed, 0 failed |
| Node announcement focused tests | 13 passed, 0 failed |
| Focused node Clippy | Passed with warnings denied |
| Focused Bazel node build | Passed |
| Workspace formatting | Passed |
| Workspace Clippy, all targets/features | Passed with warnings denied |
| Workspace build, all targets/features | Passed |
| Workspace tests, all features | Passed |
| Task 1 full repository verifier | Passed in 4m 52.549s |
| Task 2 full repository verifier | Passed in 3m 0.145s |
| Final formatting and diff checks | Passed |

## Decisions Made

- Factless compact dispatch is a local adapter integration failure, not malformed peer behavior, so the error contains no peer identity and triggers no disconnect or misbehavior evidence.
- `handle_compact_block_download` remains the factful pure-core seam; live mempool and bounded extra candidates stay owned by the managed node shell.
- The entropy closure is invoked exactly once only for `AnnounceCompactBlock`; other selected actions use a placeholder nonce that is never consumed by compact construction.
- Entropy failure delegates to the existing peer-preference fallback rather than creating a second policy implementation in the node shell.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Restored direct coverage for no-matching compact transaction responses**

- **Found during:** Task 1 pre-commit verification
- **Issue:** Removing the generic factless compact receive path exposed that it had incidentally covered the `NoMatchingInFlight` action mapping.
- **Fix:** Strengthened the factful regression to keep one compact download in flight, send a response for another hash, verify silence, and verify the original state remains intact.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Commit:** `5106f5ad`

**2. [Rule 3 - Blocking] Refreshed the generated Bazel module lock**

- **Found during:** Task 2 focused Bazel verification
- **Issue:** Adding the direct node-shell Cargo dependency changed crate-universe input digests, leaving `MODULE.bazel.lock` stale.
- **Fix:** Accepted the Bazel-generated lock refresh and verified the focused target and full repository build.
- **Files modified:** `MODULE.bazel.lock`
- **Commit:** `a553174b`

**3. [Rule 3 - Blocking] Preserved tracked LOC freshness generated by the commit hook**

- **Found during:** Both task commit hooks
- **Issue:** The repository verifier requires `docs/metrics/lines-of-code.md` to match first-party source changes.
- **Fix:** Allowed the managed hook to regenerate and stage the tracked report; the final count is 234,517 lines.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Commits:** `5106f5ad`, `a553174b`

## Authentication Gates

None.

## Known Stubs

None. Explicit empty candidate slices are intentional representations of genuinely empty live snapshots and are covered by managed receive tests.

## Security

No new endpoint, authentication path, filesystem trust boundary, or schema change was introduced. The planned OS-entropy boundary fails safely to headers/inventory and cannot produce compact success provenance or evidence.

## Handoff

Plan 126-02 can build on structurally explicit compact receive facts and randomized node-shell announcement nonces. This plan intentionally completes no milestone requirements.

## Self-Check: PASSED

- All created and modified key files exist.
- Task commits `5106f5ad` and `a553174b` exist in repository history.
- The summary has exactly two standalone YAML frontmatter delimiters.
- No known stub prevents the plan objective.

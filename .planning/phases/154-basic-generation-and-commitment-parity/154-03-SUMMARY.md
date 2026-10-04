---
phase: 154-basic-generation-and-commitment-parity
plan: "03"
subsystem: testing
tags: [basic, bip158, siphash, parity, rust, bun]
requires:
  - phase: 154-01
    provides: Pure BASIC construction and typed commitments
  - phase: 154-02
    provides: Strict historical inputs and validated spending fixtures
provides:
  - Independently reproducible ten-case pinned corpus and seventeen edge/branch vectors
  - Exact raw/display byte, hash, header and actual-history parity tests
  - Tested provenance and scoped-claim guards in the default native verifier
affects: [compact-filter-index, prune-aware-filter-serving]
tech-stack:
  added: []
  patterns: [Bun orchestration of pinned test-only Python protocol helpers]
key-files:
  created:
    - scripts/basic-filter-oracle.py
    - scripts/generate-basic-filter-vectors.ts
    - packages/open-bitcoin-consensus/testdata/basic_filter_vectors.rs
    - packages/open-bitcoin-consensus/tests/basic_filter.rs
    - scripts/check-phase154-basic-filters.ts
    - scripts/check-phase154-basic-filters.test.ts
    - docs/parity/catalog/basic-compact-filters.md
  modified:
    - AGENTS.md
    - packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs
    - docs/parity/source-breadcrumbs.json
    - docs/parity/index.json
    - docs/parity/catalog/README.md
    - README.md
    - scripts/verify.sh
key-decisions:
  - "Expected results come only from the source-audited, corpus-cross-validated pinned Python helper oracle; Bun owns orchestration."
  - "Corpus tests use low-level script facts because upstream sparse synthetic undo is not a strict historical input shape."
  - "Raw block round trips stay in consensus tests using its existing codec dependency; chainstate compares validated body-bound identity and exact commitments."
  - "Keep requirements, state updates and commits behind root full verification."
patterns-established:
  - "Check mode regenerates in memory and fails without modifying tracked files."
requirements-completed: [CFIL-01, CFIL-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 154-2026-10-04T04-36-23
generated_at: 2026-10-04T06:17:58Z
duration: 37min
completed: 2026-10-04
---

# Phase 154 Plan 03: Independent BASIC parity Summary

**Pinned Python SipHash/block helpers independently reproduce ten Knots corpus cases, seventeen edge/branch commitments, and four byte-length SipHash boundaries, with exact Rust and tested native provenance guards.**

## Performance

- Recorded implementation start: 2026-10-04T05:41:01Z (oracle file creation; context loading preceded this).
- Completed: 2026-10-04T06:17:58Z.
- Duration: 37min, including shared verification waits.
- Tasks: 3.
- Implementation paths: 14.

## Accomplishments

- Verified the clean exact Knots commit `a9aee730466ac67d35a3c03ee24676be5e045878`, all ten block hashes/encoded filters/headers before rendering edge expectations, independently derived filter hashes, and froze deterministic collision provenance.
- Checked in dependency-free Rust constants for empty/duplicate/collision/OP_RETURN/malformed/count-252/253 behavior, witness corpus data, the corrected validated genesis/funding/spending fixture, replacement spend and both branch successors, plus SipHash lengths 64/255/256/257.
- Preserved exact historical and same-block undo assertions while comparing actual block identity, filter bytes/hash/header and branch predecessor commitments against independent constants.
- Added 26 temporary-tree mutation/failure tests, structural source/parity/scope checking, deterministic oracle check in the default verifier, and honest README/catalog navigation.

## Verification

- `bun test ./scripts/check-phase154-basic-filters.test.ts`: 26 passed.
- `bun run scripts/generate-basic-filter-vectors.ts --check`: ten pinned agreements and all frozen vectors reproduced; mutation test confirms altered expectations fail without rewriting the file.
- `bun run scripts/check-phase154-basic-filters.ts`: passed.
- Timing-wrapped `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --test basic_filter`: 8 passed; compilation succeeded after dependency enumeration delay, then the binary started after its loader wait.
- Scoped consensus Clippy: `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --test basic_filter -- -D warnings` passed.
- Historical executor's latest timing-wrapped chainstate filter run: 28 passed with these golden assertions; all-target/all-feature chainstate Clippy passed; all 212 chainstate unit tests under coverage passed, including 140/140 body-bound input lines and 59/59 staged lines.
- `bun scripts/bright-builds-check.ts all`: zero findings.
- `git diff --check` and `bash -n scripts/verify.sh`: passed.
- Full native verification, overall coverage, Bazel smoke, security closure and lifecycle-valid verification remain root gates. CFIL-01/02 above identify the delivered scope; requirement checkboxes remain pending until those gates pass.

## Task Commits

No commits were created. The strict outer gate requires root to pass full phase verification before committing; implementation changes remain unstaged. Root owns STATE.md, ROADMAP.md, requirements, task tracking and final commits.

## Files Created/Modified

The frontmatter inventories all fourteen owned implementation paths. The generated fixture has 27 BASIC cases and four SipHash expectations; its hashes and headers never come from Rust-under-test. The parity page records algorithm anchors, independent mechanism/caveats, reproduction commands and deferred capabilities.

## Decisions Made

Followed the plan's narrow pinned-Python compatibility exception, documented outside managed AGENTS text. No dependencies, crates, baseline binary builds, runtime endpoints or deferred capabilities were introduced. An explicit simplification review kept selection/Rice protocol logic in the small test-only oracle, filesystem/pin/bounds/rendering in Bun, and checked fixed evidence paths rather than scanning broad source trees.

## Deviations from Plan

- **[Rule 3 - Blocking] Isolated pinned Bun bootstrap.** Installed Bun was 1.4.2 while the repository pins 1.3.9. Downloaded and verified the pinned executable at `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun`; preserved installed defaults and `.bun-version`.
- **[Rule 3 - Blocking] Explicit Bun test path.** Bare `bun test scripts/...` stalled in directory discovery; process sampling showed `__getdirentries64`. The new verifier call uses `./scripts/...`, which runs the 26 tests immediately. Legacy literal verifier contracts remain untouched; root separately owns Plan 04's runtime argv repair.
- **[Rule 3 - Blocking] Avoided a chainstate codec backedge.** An initial test-only raw serialization assertion produced E0433 because chainstate has no codec dependency. Removed that assertion, retained body-bound identity/exact commitments, and proved raw serialization in consensus using its existing dependency.
- **[Rule 2 - Correctness alignment] Rebound edited synthetic fixture.** The concurrently added historical body-Merkle check requires the ordering test's intentionally modified body to recompute Merkle root and rebind position hash/header and undo tag. Actual validated fixtures remain unchanged.
- The breadcrumb checker scans tracked paths; unmatched new groups before staging are a staging-gate limitation, not a breadcrumb omission. Root will make new source paths visible before full verification.

## Issues Encountered

Initial oracle check correctly refused the absent generated fixture, then explicit oracle generation established all pinned agreements. No independent filter expectation was rewritten to agree with Rust. Quiet Rust compilation was sampled in dependency directory enumeration and allowed to continue rather than being treated as a timeout.

## Known Stubs

None. Empty scripts/empty filter bytes are intentional protocol vectors, and deferred runtime services are explicitly outside this plan.

## User Setup Required

None. Reproduction requires repo-pinned Bun, Python 3's standard library and the initialized pinned Knots submodule.

## Next Phase Readiness

Pure BASIC evidence is ready for root review and full phase verification. Index activation/storage, filter prune retention, RPC/peer serving, catch-up, V0/BIP37, GUI and production/funds capabilities remain deferred. Root handles commits and planning status only after the final gate.

## Self-Check: PASSED

All fourteen owned implementation paths and this summary exist. The focused Rust,
historical, Bun and lint evidence above passed. No commit hashes are claimed:
commits are intentionally pending the strict root verification gate.

## Hook-boundary addendum

The normal commit hook exported `GIT_INDEX_FILE=.git/index`, which broke foreign
Git calls in submodule/temp roots. Two new real-fixture regressions reproduced
the relative-index failure and broader selector/config contamination (RED:
26 passed, 2 failed). Following [Git's hook guidance](https://git-scm.com/docs/githooks),
the generator now discovers canonical local names with
`git rev-parse --local-env-vars` and clears only those names in child environment
copies. The name-only probe excludes Git bootstrap contamination; actual foreign
commands retain global configuration/auth and system settings, and the parent
environment is unchanged. Child duration/output bounds and visible errors remain.

GREEN: all 28 tests pass normally, with the hook's relative index, and with broader
repository selectors plus invalid local configuration. Oracle `--check` passes
under contamination; pin mismatch, no-write drift and corpus disagreement
diagnostics remain specific. Golden Rust expectations were not regenerated.
Only the generator, its test file and this addendum changed; root owns the normal
commit-hook retry and full verification gate.

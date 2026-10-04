---
phase: 154-basic-generation-and-commitment-parity
reviewed: 2026-10-04T08:13:56Z
depth: standard
files_reviewed: 31
files_reviewed_list:
  - packages/open-bitcoin-primitives/src/hash.rs
  - packages/open-bitcoin-primitives/src/lib.rs
  - packages/open-bitcoin-codec/src/block_filter.rs
  - packages/open-bitcoin-codec/src/block_filter/tests.rs
  - packages/open-bitcoin-codec/src/lib.rs
  - packages/open-bitcoin-consensus/src/block_filter.rs
  - packages/open-bitcoin-consensus/src/block_filter/tests.rs
  - packages/open-bitcoin-consensus/src/block/tests/genesis_header_fixture_passes_pow_check.rs
  - packages/open-bitcoin-consensus/src/crypto.rs
  - packages/open-bitcoin-consensus/src/crypto/siphash.rs
  - packages/open-bitcoin-consensus/src/lib.rs
  - packages/open-bitcoin-chainstate/src/block_filter.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/fixtures.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - AGENTS.md
  - scripts/generate-basic-filter-vectors.ts
  - scripts/basic-filter-oracle.py
  - packages/open-bitcoin-consensus/testdata/basic_filter_vectors.rs
  - packages/open-bitcoin-consensus/tests/basic_filter.rs
  - docs/parity/source-breadcrumbs.json
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/index.json
  - docs/parity/catalog/README.md
  - README.md
  - scripts/check-phase154-basic-filters.ts
  - scripts/check-phase154-basic-filters.test.ts
  - scripts/verify.sh
  - scripts/verify/helpers.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: final-source-completed-plans-01-02-03-04
diff_base: 0be7aac9
resolved_findings: [WR-01]
---

# Phase 154: Code Review Report

**Reviewed:** 2026-10-04T08:13:56Z
**Depth:** standard
**Files Reviewed:** 31
**Status:** clean; final source review

## Summary

Reviewed all completed Plans 01–04: Rust implementation, independent oracle/generator, fixtures/integration tests, scoped parity documentation/registration, verifier wiring, helper argv repair and the final legacy Merkle test correction, including the hook-boundary child-environment isolation and scoped worktree changes against base `0be7aac9`. All reviewed files meet quality standards. No unresolved issues found.

Source review is complete and clean for all completed Plans 01–04. Root's full native verification, lifecycle-valid phase verifier, coverage and Bazel gates remain pending before phase completion; these are separate execution gates. Subsequent source changes require review.

The BASIC implementation matches pinned `blockfilter.cpp` for output-only first-byte OP_RETURN exclusion, empty-script exclusion, raw-script deduplication, retained mapped collisions, little-endian SipHash keys, high-half range mapping, count-prefixed MSB-first Golomb-Rice encoding and raw SHA256d commitment composition. Count/range/order/output checks bound encoder arithmetic and unary writing. Explicit predecessor identity and height checks preserve branch correspondence. Arbitrary-byte SipHash correctly handles tails and wrapped lengths. The Merkle mutation repair checks real pairs before padding, matching pinned `consensus/merkle.cpp`.

The historical adapter distinguishes missing non-genesis undo from explicit complete empty undo and requires exact transaction/input cardinality. Actual staged tests establish prior-block and same-block spent-script provenance before comparing filters, and retained history works after current coins disappear. The shared constructor now binds the supplied transaction body to its header and refuses actual Merkle mutation.

Local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and architecture, code-shape, testing, verification, Rust and TypeScript/JavaScript standards informed this review. The narrow pinned-Python-helper exception is documented outside managed AGENTS text. No project skills were present. Both active lesson inputs were read completely (7,188 bytes; conservative estimate 2,397 tokens).

## Independent proof review

The oracle imports pinned Knots Python block/SipHash helpers and implements audited selection, mapping and Rice composition without executing or importing Rust-under-test. Bun checks the exact submodule commit and tracked worktree cleanliness, validates the ten-row corpus shape, and compares all ten independently computed block identities, bytes and headers before rendering any accepted edge expectations. The source correctly distinguishes this corpus-cross-validated oracle from a directly linked Knots binary.

Child execution has explicit input/output bounds, a ten-second timeout, and visible executable/nonzero-exit failures. `--check` renders in memory and compares exact tracked bytes; it does not regenerate tracked files or run the collision search. The frozen distinct-script collision retains both values. Fixtures include all ten upstream cases, 252/253 counts, OP_RETURN asymmetry, malformed/empty/duplicate scripts, wrapped SipHash lengths and six ordered/replacement blocks. Generated bytes and commitment ordering are consistent with the source-audited oracle.

Rust integration tests compare raw serialization, block identity, encoded bytes, filter hashes and headers against those independently frozen constants. Updated chainstate tests retain direct actual-undo assertions and compare validated/retained/replacement commitments to the same fixtures. Corpus tests deliberately use low-level script facts because upstream flattened synthetic undo is not complete production history.

The checker and its temporary-tree mutation tests cover missing pins, source anchors/evidence, removed executable verifier steps, fixture drift, wrong submodule pin, corpus-count/oracle disagreement, child errors and scoped overclaims. Source-string checks are accurately described as supplementary structural evidence. Current README/parity claims remain limited to pure BASIC construction and commitments; later runtime services and production claims remain deferred. The three new executable verifier steps precede Rust checks without changing existing command ordering.

## Native verifier helper review

The Plan 04 delta copies post-label argv into a Bash array and only normalizes exact `bun test` commands. Existing relative `.test.ts` regular files before the first option receive `./`; flags and values, directories, absent paths, absolute paths, explicit `./`/`../`, spaces and non-test commands retain their argv boundaries. Stopping at the first option is deliberate and documented: this covers actual native callers without inventing a general Bun CLI parser.

Execution remains a quoted array invocation, with no eval or command-string reconstruction. Timing start/end, disabled/restored errexit, captured child exit status, timing recording, cleared current-step fields and returned status follow the original sequence. The small shfmt pipeline whitespace change leaves pipeline semantics intact. Plan 04 does not change literal verifier calls, help/heredoc strings or their ordering. Executor evidence reports preserved argv and actual child exit 37 being returned/recorded with cleanup; source inspection confirms the failure path is unchanged.

## Final legacy Merkle test delta

Reviewed `packages/open-bitcoin-consensus/src/block/tests/genesis_header_fixture_passes_pow_check.rs`, including its changed duplicate-transaction test and the helper transaction/validation call chain. The former three-entry `[coinbase, spend, spend]` fixture relied on odd padding being incorrectly counted as mutation. The corrected `[coinbase, spend, distinct_duplicate, distinct_duplicate]` fixture changes the third transaction's lock time to 1, making the first three transaction identities distinct, then supplies an actual equal final pair. It commits the three-entry root and asserts the four-entry root remains equal, true mutation is reported, and `check_block` rejects with `bad-txns-duplicate`.

This strengthens the pinned Knots duplicate-leaf defense proof: it exercises a genuine root-preserving mutation after the correct odd-padding production repair. No production validation behavior changed, and no filler was added to the actual historical/same-block three-transaction fixture. No finding arises from this test-only delta. The core executor owns its full consensus unit execution; this reviewer performed source/diff review and retains the native aggregate gate as pending.

## Hook-boundary child environment review

The actual commit hook exposed inherited `GIT_INDEX_FILE=.git/index` selecting the wrong index after foreign submodule/temp-repository cwd changes. The generator now discovers Git's canonical repository-local names with `git rev-parse --local-env-vars` and deletes those names only from a copied child environment. This follows [Git's official hook guidance](https://git-scm.com/docs/githooks) for invoking Git in foreign repositories.

The name-only bootstrap probe excludes inherited `GIT_*` selectors so poisoned directory/object/config-count values cannot prevent discovery. Actual children remove only the canonical local names, preserving global/system configuration, authentication settings, PATH and unrelated environment. Parent environment remains unchanged; test-side mutations use finally restoration. The probe and actual child execution share the ten-second timeout, bounded stdout/stderr and visible nonzero/error handling. Input bounds remain checked before child execution. No expectation/oracle algorithm or golden fixture was changed.

The two added regressions prove relative-index isolation, broader directory/worktree/common-dir/object/config-count selector isolation, global/auth preservation, unchanged parent state and exact deterministic fixture reproduction. This reviewer independently ran the 28-test suite on pinned Bun 1.3.9 in three outer environments: normal, relative `GIT_INDEX_FILE=.git/index`, and broader repository selectors plus invalid `GIT_CONFIG_COUNT`. Each run passed 28 tests with zero failures. No unresolved finding arises from this delta; root's normal full hook retry remains the aggregate completion gate.

## Resolved finding

### WR-01: Supplied transaction bodies now bind to the validated header

**Original location:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/block_filter.rs:44-46`
**Related accessor:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/engine/stage.rs:22-29`
**Original severity:** Warning
**Disposition:** Closed on source and regression review; root/executor owns Rust execution evidence.

The original constructor checked header/hash identity but accepted changed output scripts or input outpoints under that unchanged header when cardinalities matched. This could generate altered filter commitments using genuine staged undo and the original validated hash.

The fix recomputes `block_merkle_root` after structural/genesis guards and rejects `MutatedBody` and `BodyMerkleMismatch` before constructing inputs. `BodyEncoding(CodecError)` preserves typed encoding failure and Error source through the minimal existing consensus re-export; the enum no longer derives Copy. Tests cover unchanged success, output-only and input-outpoint substitutions without live-state mutation, duplicate leaves and repeated real subtrees with root-preserving bodies. Synthetic ordering/shape fixtures now rebuild header/position identity when their bodies change. This closes the identified gap without claiming authentication of fabricated same-shape undo or imposing unrelated witness binding.

## Review verification and limits

With the repo-pinned Bun 1.3.9 at `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun`, this reviewer independently ran:

- `bun run scripts/generate-basic-filter-vectors.ts --check`: passed all ten pinned corpus agreements and frozen edge reproduction.
- `bun run scripts/check-phase154-basic-filters.ts`: passed provenance, evidence and scoped claim checks.
- `bun test ./scripts/check-phase154-basic-filters.test.ts`: 26 passed, zero failed.
- Sourced `run_step` with the original `bun test scripts/check-phase154-basic-filters.test.ts` operand: 26 passed, zero failed; recorded duration 466 ms, status 0, current-step cleanup confirmed.
- `bash -n scripts/verify/helpers.sh`, `shfmt -d -i 2 scripts/verify/helpers.sh`, `git diff --check`: passed.
- Report formatting uses the installed mdformat GFM/frontmatter extensions with exactly two standalone YAML delimiters.

An initial supplemental run used PATH Bun 1.4.2; all three checks were then repeated successfully with the pinned 1.3.9 binary, so the pinned results above are the review evidence. No source files were modified and no Cargo commands or commits were run by this reviewer, as requested. Prior Rust results in implementation summaries are executor evidence, not independently rerun review results; WR-01's summary explicitly distinguishes pre-fix results from follow-up execution. Root retains final native verification, complete Rust execution, coverage, Bazel smoke and phase closure.

______________________________________________________________________

_Reviewed: 2026-10-04T08:13:56Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_

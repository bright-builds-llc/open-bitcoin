---
phase: 160-explicit-bounded-peer-filter-serving
plan: "01"
subsystem: networking
tags: [bip157, basic-filters, wire-codec, bounded-parsing]
requires:
  - phase: 159
    provides: Existing persisted BASIC filter commitments and guarded authority
provides:
  - Six raw-byte BIP157 message codecs with bounded response allocation
  - Typed nonfatal short-CF-request outcome and request-only trailing-byte compatibility
  - COMPACT_FILTERS service flag without default activation
affects: [160-02, 160-05, 160-08, 160-09, 160-10, 160-11]
tech-stack:
  added: []
  patterns: [Checked response envelope arithmetic before allocation, request extraction distinct from frame validation]
key-files:
  created:
    - packages/open-bitcoin-network/src/message/filter.rs
    - packages/open-bitcoin-network/src/message/filter/tests.rs
  modified:
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs
key-decisions:
  - Preserve filter type as raw u8 and filter commitments as raw Hash32 bytes until semantic policy.
  - MalformedCFRequest is receive-only and cannot be encoded as an outgoing message.
  - Only the three CF request parsers ignore trailing handler bytes; responses and other commands stay strict.
requirements-addressed: [CFNET-02, CFNET-03, CFNET-04, CFNET-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T19:08:40Z"
duration: 7min
completed: 2026-10-10
---

# Phase 160 Plan 01: BASIC Peer Wire Families Summary

**Six BIP157 codecs preserve pinned digest order, bound CF response payloads to 4,000,000 bytes, and retain nonfatal request extraction semantics.**

## Accomplishments

- Added GetCFilters/GetCFHeaders/GetCFCheckpt and CFilter/CFHeaders/CFCheckpt structs, exports, command names, payload codecs and framed-message integration. Request payloads are exactly 37/37/33 bytes when encoded.
- Preserved unsupported filter types including 2 and 255 for later semantic authorization. Added COMPACT_FILTERS = 1 << 6 without advertising it by default.
- Applied checked count multiplication, CompactSize prefix arithmetic, the pinned payload envelope, and remaining-payload bounds before vector allocation. The index codec's separate capacity is unchanged.
- Added MalformedCFRequest(CFRequestKind), consumed by ordinary peer dispatch with no response or disconnect action. All 107 short request lengths across the three commands are covered. Successful request fields tolerate trailing bytes; response/non-CF payloads remain strict.

## Acceptance Criteria

- [x] Every family has binary command/encode/decode integration without JSON/RPC conversion.
- [x] Exact vectors cover pinned testnet genesis filter bytes, empty checkpoint vectors, predecessor headers and CompactSize transitions at 252/253 and 65,535/65,536.
- [x] Non-palindromic block/filter/header digests have direct byte assertions in addition to round trips.
- [x] Short complete requests earn no response/disconnect; subsequent ordinary peer dispatch remains usable.
- [x] Frame header, frame length and checksum failures stay errors; strict non-CF decoding remains covered.

## TDD and Verification

Task 1 RED executed one test and failed with UnknownCommand("getcfilters"). After codec integration it passed. Expanded codec coverage passed while Task 2 RED failed specifically with UnexpectedEof for short complete requests and TrailingData for valid request fields plus suffix bytes. The narrow extraction outcome and request parser changes made both pass.

All Cargo commands used pinned Bun 1.3.9 and the cooperative command-timings runner with key phase160-codecs; no Cargo jobs overlapped within this executor.

| Check | Evidence |
| -- | -- |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_wire -- --nocapture` | 14 focused tests passed, nonzero; 10 codec tests and 4 dispatch tests |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_wire_dispatch -- --nocapture` | 4 passed, nonzero |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --quiet` | 560 unit tests, 2 parity tests and 2 property tests passed |
| `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` | Passed |
| Scoped `rustfmt --edition 2024 --check` | Passed for every owned source/test path |
| `git diff --check` | Passed |

Full native verification, workspace build, independent security/source review and final lifecycle verification belong to the parent consolidated gate and have not been claimed here.

## Task Commits

1. Task 1: Define and implement all six binary contracts — pending outer consolidated commit.
2. Task 2: Preserve pinned CF handler parse outcomes in normal dispatch — pending outer consolidated commit.

No staging, commits, pushes, branch/worktree changes, STATE/ROADMAP/REQUIREMENTS/todo changes or breadcrumb-manifest writes were performed by this executor. Requirements remain incomplete until parent verification.

## Deviations from Plan

**1. [Rule 1 - Bug] Updated the stale deferred-command regression assertion.**

- Found during full network-crate regression: 559 tests passed and one failed because CF commands were still asserted to be unknown.
- Removed only the four now-implemented CF command entries from `peer/tests/peer_action_cases.rs`; retained all `mempool` and BIP37 absence checks.
- This additional modified path is directly caused by the new supported command surface. The rerun passed all 560 unit tests and four integration tests.
- Commit deferred to the outer gate.

No scope reduction or architectural deviation occurred. Authentication gates: none.

## Simplification and Security Review

The explicit simplification pass retained one shared checked payload-length function, one bounded count parser and one raw-hash vector codec for all responses. Request shapes share their ranged field codec without introducing a generic message framework. The changes preserve all prior message branches; no production dependency or I/O was added.

- T-160-01: CompactSize counts are bounded against complete payload capacity and available bytes before allocating; raw filter type survives decoding.
- T-160-02: A short complete CF payload becomes a typed nonfatal outcome, while corrupt framing/EOF/checksum remain transport errors. The outcome cannot be encoded and earns no response.
- T-160-03: Exact genesis bytes and direct non-palindromic predecessor/filter-hash/block-hash assertions protect raw digest order.

Stub scan found no TODO/FIXME/placeholder data source in the new files and no unfinished codec behavior. Successful CF request serving is intentionally left to Plan 160-08 and the later adapters; this plan supplies its typed wire seam without enabling a serving capability. No unmodeled endpoint, authentication, filesystem or schema trust boundary was introduced.

## Breadcrumb Registration and Next Plan Readiness

Parent must serially register these two new paths in `docs/parity/source-breadcrumbs.json`:

- `packages/open-bitcoin-network/src/message/filter.rs`
- `packages/open-bitcoin-network/src/message/filter/tests.rs`

Both source comments use the same ordered pinned anchors: `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/protocol.cpp`, `packages/bitcoin-knots/test/functional/test_framework/messages.py`. The pinned `src/net_processing.cpp` request handlers and ProcessMessages exception boundary were inspected directly; the genesis vector also follows its pinned `src/test/data/blockfilters.json` row.

AGENTS.md, its Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research and decision coverage materially informed the implementation. The parent's preloaded active lessons and synchronized baseline were honored. Existing breadcrumbs were retained on modified files.

## Self-Check: PASSED

Both new Rust files, all five modified Rust files and this summary exist. Focused tests, full crate regression, targeted Clippy, scoped formatting and diff whitespace checks passed. Commit checks are intentionally deferred because the outer wrapper forbids intermediate commits; no commit hash is asserted. Shared planning and unrelated executor edits were preserved.

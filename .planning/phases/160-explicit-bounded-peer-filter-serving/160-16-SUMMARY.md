---
phase: 160-explicit-bounded-peer-filter-serving
plan: "16"
subsystem: testing
tags: [bip157, knots, oracle, bun, python, proof, fixtures]
requires:
  - phase: "160-14"
    provides: Explicit separation of wire envelope, local resource policy and transport evidence
provides:
  - Pinned independent six-family payload and v1 mainnet-frame fixtures
  - Independent exact 256-bit proof and signed equivalent-time fixtures
  - Bounded Bun validation, deterministic generation and read-only checking
affects: ["160-11", "160-12"]
tech-stack:
  added: []
  patterns: [Pinned helper serializers, lossless decimal strings, validate-before-replace]
key-files:
  created:
    - scripts/generate-peer-filter-fixtures.ts
    - scripts/generate-peer-filter-fixtures.test.ts
    - docs/parity/fixtures/peer-filter-wire.json
    - docs/parity/fixtures/peer-filter-proof.json
  modified:
    - scripts/basic-filter-oracle.py
key-decisions:
  - Keep every filesystem, process, pin, corpus, bound and final rendering concern in Bun; extend only the existing test-only Python oracle.
  - Treat truncated-request refusal as C++ DataStream source evidence because the Python uint256 reader is permissive; helper-check trailing consumption separately.
  - Use exact pinned serializers for payloads and the pinned v1 frame formula; no first-party Rust supplies expected values.
requirements-addressed: [CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T01:45:57Z"
duration: 8 min
completed: "2026-10-11"
---

# Phase 160 Plan 16: Independent Peer Filter Fixtures Summary

**Pinned Knots serializers and independent exact integer formulas reproduce 88 wire, handler, malformed-input and proof cases through a bounded Bun generator.**

## Performance

- **TDD implementation started:** 2026-10-11T01:37:15Z
- **Completed:** 2026-10-11T01:45:57Z
- **Tasks:** 2/2
- **Files modified:** Five owned implementation/golden paths, plus this summary
- **Final fixtures:** 1,458,391 combined bytes; 42 wire, 13 handler, six malformed-input and 27 proof cases

## Accomplishments

- Implemented exact local pin validation for `a9aee730466ac67d35a3c03ee24676be5e045878`, tracked-input cleanliness checks and canonical repository path/symlink guards before helper invocation or fixture publication.
- Bun bounds subprocess input to 8 MiB, stdout/stderr capture to 16 MiB, helper execution to ten seconds, corpus to 256 cases, each wire payload to 4,000,000 bytes and combined rendered goldens to 16 MiB. Case identity, exact keys, lossless work/proof strings, raw digests, canonical CompactSize, payload-field correspondence, frame length/checksum and signed time limits are validated before replacement.
- Both output paths and documents are validated first; complete temporary documents are staged before atomic per-file rename. Failed pin/schema/output controls leave prior goldens unchanged. Check mode preserves golden bytes and modification times. CLI breadcrumbs and final run reports use the existing ignored `.local/open-bitcoin-dev/peer-filter-fixtures/last-run.json` path.
- Extended the existing Python oracle with explicit `--peer-wire-proof`, using all six pinned `messages.py` serializers and source-derived v1 framing. Default BASIC mode remains compatible with all ten pinned corpus agreements and frozen edge commitments.
- Wire goldens include non-palindromic raw hashes, CompactSize 252/253 and 65,535/65,536 transitions, genesis/predecessor distinction, 2,000 cfheaders entries, and positive checkpoint intervals including 0/999/1,000/1,001/2,000/3,001 and 252/253 checkpoint-vector counts. Handler assertions preserve semantic precedence, inclusive range limits, absent backend/record outcomes and distinct malformed/trailing contracts without inventing peer error-response bytes.
- Proof goldens include nonuniform compact targets, invalid negative/zero/overflow targets, future signed age, strict 30-day timestamp/proof boundaries, negative/equal work, 256-bit multiplication wrap, exact INT64_MAX and signed saturation. Invalid-target equivalent time is explicitly undefined/null rather than evaluating division by zero.

## Task Evidence

1. **Task 1 — pinned bounded Bun orchestration:** TDD RED failed on the missing generator module; GREEN ran five tests with eight assertions and zero failures. Final expanded suite verifies its pin/path/corpus/process/output/schema/publication controls.
2. **Task 2 — authentic independent goldens:** TDD RED ran five passing controls and one failing real-helper test because Python lacked the explicit mode (`KeyError: cases`). GREEN added the mode and goldens. Final suite: **54 pass, zero fail, 102 assertions**, 10.70 seconds; the ten-second stalled-helper control accounts for most runtime.

Commands used with `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH` (Bun 1.3.9):

```bash
bun test ./scripts/generate-peer-filter-fixtures.test.ts
bun scripts/generate-peer-filter-fixtures.ts --write
bun scripts/generate-peer-filter-fixtures.ts --check
bun scripts/generate-basic-filter-vectors.ts --check
bun scripts/bright-builds-check.ts all
bun build ./scripts/generate-peer-filter-fixtures.ts --target=bun --external '*' > /dev/null
```

The driver invokes `python3 scripts/basic-filter-oracle.py --peer-wire-proof` with `PYTHONDONTWRITEBYTECODE=1`; BASIC checking invokes the same existing Python file without arguments. Repeated real generation, normal writes and read-only checking reproduce the exact pair. Tests reject a self-consistent payload/frame/checksum mutation that changes a declared raw digest, as well as independent byte mutations, schema violations, wrong pin, dirty tracked source, oversized stdout/input and stalled subprocess work. Both golden paths have mutation checks.

Managed starter checks reported zero findings. Scoped `git diff --check` passed. Bun build validated standalone script syntax; no dedicated TS formatter or TS project configuration applies to these standalone existing-style scripts. No Cargo, Bazel or full-native command was run in this isolated worker; the parent owns those gates.

## Task Commits

Commits deferred to the authorized outer strict workflow. No staging, commit, push, branch, worktree or shared planning-state mutation occurred in this executor. The parent owns STATE/ROADMAP/requirements and final verification.

## Decisions Made

Material guidance: repo AGENTS.md and sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability and TS/JS standards, active global/repo lessons, the phase context/research and decision coverage. Active lessons were measured at 7,188 bytes / 2,397 conservative tokens and loaded without archives; the parent reported no audit trigger.

The 4,000,000-byte wire singleton envelope is explicitly anchored to `src/net.h:MAX_PROTOCOL_MESSAGE_LENGTH`; it does not substitute for local ancestry/copy/queue policy or achieved transport evidence. Goldens record their exact pin and source authorities.

## Deviations from Plan

None. The planned malformed-input source distinction required a correction to the initial test assumption: pinned `messages.py:deser_uint256` accepts short reads, whereas `streams.h:DataStream::read` throws at end of data. The final fixture labels truncation as source-derived refusal, preserves exact truncated bytes and independently checks trailing consumption through helpers. No product behavior or ownership scope changed.

## Simplification and Diff Review

Reviewed the five owned paths for scope, imports, unchanged BASIC computation and source formulas. Kept the six-family corpus in one driver and reused the existing Git environment sanitizer. Payload validation decodes bounded fields rather than adding a second serializer/oracle. New source files remain below 400 lines; no dependencies, production Rust, endpoints, mutable schemas or additional authority were introduced. No unresolved stubs were found. T-160-31 is addressed at the planned helper-output/fixture boundary; no additional threat surface was introduced.

## Issues Encountered

None remaining. Atomicity is per output file: both complete documents are staged before renaming, but the filesystem does not provide a transaction across two renames. Interrupted publication is detected by the next read-only check and converges on regeneration.

## Next Plan Readiness

Plan 11 can consume immutable independent wire/proof goldens. These fixtures establish helper/source expectations only: comparison against first-party Rust, actual inbound/outbound transport, near-maximum valid block resource behavior and achieved complete/partial writes remain later-plan and parent-verification work. No CFNET requirement is marked complete here.

## Self-Check: PASSED

All five owned implementation/golden files and this summary exist. Both real fixture checks and BASIC compatibility passed, the focused suite executed 54 nonzero tests, bounded subprocess controls passed, and the pinned submodule has no tracked modifications. Commit existence checks are intentionally inapplicable under the plan's explicit deferred-commit contract.

## Post-review compatibility repair | 2026-10-11

The unchanged historical Phase154 source guard expects the original BASIC helper import line. The parent retained that exact import and put only the added peer-message imports in a separate statement. No helper behavior, corpus, golden bytes or historical guard changed. Phase154 and independent peer fixture --check passed after the repair; the outer native contract reruns the full tests.

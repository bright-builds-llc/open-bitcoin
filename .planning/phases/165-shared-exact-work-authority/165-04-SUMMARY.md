---
phase: 165-shared-exact-work-authority
plan: "04"
subsystem: network
tags: [proof-work, header-ancestry, strict-ordering, import-refusal, parity]
requires:
  - phase: 165-03
    provides: Atomic ProofWork carriers and explicit untrusted numerical imports
provides:
  - Accepted-parent target-derived cumulative header work
  - One strict greater-work live header selector and direct proof projections
  - Hash-checked unbound admission and immutable selected powLimit admission
  - Explicit unproved fixture projection without live or recovery authority
affects: [165-05, 165-06, 165-09, 165-13, 165-14]
tech-stack:
  added: []
  patterns: [Private accepted ancestry, direct accepted-entry proof projection, explicit unproved imports]
key-files:
  created:
    - packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs
  modified:
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-network/src/header_store/proof.rs
    - packages/open-bitcoin-network/src/header_store/tests.rs
    - packages/open-bitcoin-network/src/header_store/proof/tests.rs
    - packages/open-bitcoin-network/src/filter_serving/policy/tests.rs
    - packages/open-bitcoin-network/src/peer/tests/block_fixtures.rs
    - packages/open-bitcoin-network/src/peer/tests/block_announcement_fallback_cases.rs
    - packages/open-bitcoin-network/src/peer/tests/block_sync_request_cap_cases.rs
    - packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs
    - packages/open-bitcoin-network/tests/parity.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Unbound insertion verifies actual hash/target and never silently selects regtest parameters."
  - "Selected insertion binds powLimit and refuses unbound ancestry promotion or mismatched contexts."
  - "Imports retain explicitly unproved numbers; they cannot supply accepted parents or live best tips."
  - "Proof facts are direct accepted-entry projections, with no cache or second selector."
patterns-established:
  - "Mine cheap fixtures and re-admit actual headers instead of trusting seed metadata."
  - "Imported/restart tie order remains unavailable; raw numerical sorting is never claimed as Knots activation parity."
requirements-completed: ["CWOR-02", "CWOR-03"]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T18:38:31Z
duration: 16min
completed: 2026-10-14
---

# Phase 165 Plan 04: Accepted Header Work Authority Summary

**Hash-checked accepted ancestry derives exact cumulative work, and one strict work selector drives live tips, locators, branch queries and proof diagnostics.**

## Performance

- First recorded test: 2026-10-14T18:22:16.786Z; initial context loading was not separately timed.
- Final verification and summary: 2026-10-14T18:38:31Z.
- Duration: 16min from the first recorded test.
- Tasks: 2/2; 12 source/mapping files, plus this summary.

## Accomplishments and APIs

- `insert_header(header)` checks normalized target validity and actual header hash without asserting a selected network. It derives height and work from a private accepted parent, using `checked_add` for height and `ProofWork::wrapping_add(block_proof(bits))` for work. Missing/unproved parents, invalid target/hash and additional roots fail before mutation. Accepted duplicates preserve the original work and first best-header admission.
- `insert_header_with_params(header, &ConsensusParams)` additionally checks selected powLimit. A selected store refuses mismatched limits and subsequent unbound insertion. A store with existing unbound ancestry refuses selected promotion, including duplicate promotion; Plan05 must bind the actual managed owner explicitly.
- `best_tip`, `best_height`, `best_chain_entries`, locators and outgoing header queries use one accepted best header. Only strictly greater unsigned full-width work replaces it. Hash and height do not break accepted ties.
- `maybe_proof_facts(hash)` and `maybe_best_proof_header()` directly project the accepted entry's cumulative work and own block proof. The old proof cache, independent best-proof selector, reconstructed import authority and count score were removed.
- `from_entries`, `seed_from_chain` and `record_position` are unproved imports. They cannot supply accepted parents, live best tips or proof facts. `record_position` cannot overwrite an accepted receipt. The explicitly named `maybe_unproved_fixture_tip()` retains raw numerical visibility only; map/restart order has no earned reception-order claim. Ancestry queries are bounded for raw cycles.
- Twelve new behavioral tests cover shorter stronger branches, equal-work real admission order, taller equal-work refusal, duplicate preservation, raw duplicate refusal, consistent/inflated/forged imported parents, normalized zero/hash/root/missing-parent refusals, selected context transitions, accepted receipt preservation, cyclic imports, a missing selected query entry and a >128-bit fixed corpus number that remains unproved.

## Task Commits

Both task commits, RED/GREEN commits and the metadata commit are **deferred to the parent strict finalization** after entire-phase native verification, independent review/security/goal checks and normal hooks. No files were staged, committed or pushed by this executor. Shared STATE, ROADMAP, REQUIREMENTS, config and todo files were not edited by this executor.

## Executed Evidence

Every Cargo command ran sequentially through command-timings with Rust 1.94.1 and command-local Bun 1.3.9. Prefix each command below with:

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Command                                                                                                                                                                                                                | Actual result                                                                                                                                                                                                                                        |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run scripts/command-timings.ts run --key phase165-header-authority -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features phase165`                                             | RED: all 3 initial tests failed for the intended selector/import/order defects. Final GREEN: 12 passed, zero failed/ignored.                                                                                                                         |
| `bun run scripts/command-timings.ts run --key phase165-header-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features header_store`                                        | Final 28 passed, zero failed/ignored.                                                                                                                                                                                                                |
| `bun run scripts/command-timings.ts run --key phase165-header-network-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --quiet`                                     | Final 617 unit, 2 parity and 2 property tests passed; zero failed/ignored.                                                                                                                                                                           |
| `bun run scripts/command-timings.ts run --key phase165-header-workspace-check -- cargo check --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`                                             | Exit 0, 18.51s; all consuming crates compile.                                                                                                                                                                                                        |
| `bun run scripts/command-timings.ts run --key phase165-header-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`                          | Final exit 0, 4.30s.                                                                                                                                                                                                                                 |
| `bun run scripts/command-timings.ts run --key phase165-header-format -- cargo fmt --manifest-path packages/Cargo.toml --all` and subsequent `phase165-header-format-check` with `--check`                              | Passed.                                                                                                                                                                                                                                              |
| `bun run scripts/command-timings.ts run --key phase165-header-coverage -- cargo llvm-cov --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --lib --show-missing-lines --summary-only --quiet` | After clean instrumentation, 617 tests passed; no `Uncovered Lines` section. LLVM aggregate reported 99.71% lines, header-store summary 99.00%, proof module 100.00%; those raw summary percentages are not claimed as 100%. Final duration 21.554s. |
| `bun scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                                                      | Passed, 1,165 Rust files covered.                                                                                                                                                                                                                    |
| `bun scripts/bright-builds-check.ts all`                                                                                                                                                                               | Zero findings, no exceptions added.                                                                                                                                                                                                                  |
| Owned diff review, stub/threat scan, `git diff --check`                                                                                                                                                                | Passed; no goal-blocking stub or new unregistered security surface.                                                                                                                                                                                  |

The coverage cleanup command used `phase165-header-coverage-clean` around `cargo llvm-cov clean --manifest-path packages/Cargo.toml --workspace`. The first report identified three header-store paths; focused controls and a clean final report resolved its reported uncovered lines. No full workspace test/native/Bazel acceptance is claimed here: Plan14 and the parent own the complete final contract.

## Decisions and Simplification

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/Rust/TypeScript guidance and both complete active lesson files (7,188 bytes;2,397 estimated tokens) informed execution. The parent owned initial sync/bootstrap. The simplification pass removed the entire advisory cache, making proof diagnostics a read-only projection of the selected entry rather than a second ancestry computation.

## Deviations from Plan

**[Rule3 - Blocking] Expanded the fixture inventory with root approval.** The required hash checks and unproved-seed boundary exposed affected fixture setup outside the original six paths. The parent approved all six exact paths recorded in `.local/open-bitcoin-dev/phase165-plan04-extra-paths.json` before their edits and updated the plan inventory. Repairs mine cheap headers or re-admit existing mined genesis/child headers into fresh stores, preserving cap, peer, wire/block and policy assertions. No production fixture bypass was added.

The nonuniform filter-policy case needs proof-age beyond its existing30-day threshold. Its first bounded target produced8192 work and failed that assertion; a bounded16384-work target preserves the exact assertion and cheap mining. Signed timestamp cases use bounded8192-work targets. No frozen corpus, historical receipt, source attribution hash or prior qualification outcome was regenerated.

## Issues Encountered

Affected network discovery initially found19 unit failures from the intentional admission/import changes; approved fixture repairs reduced these to one proof-age fixture boundary, then all unit cases passed. The complete suite subsequently exposed the sixth raw-seeded wire parity fixture; it now re-admits the same mined blocks and retains all assertions. A627-line fixture file passes the628-line managed rule after a small import simplification; no exemption or checker weakening was used.

## Next Plan Readiness

Plan05 must use `insert_header_with_params` for every actual selected managed owner and preserve canonical-root/history gating. It must not consume raw `seed_from_chain` positions as accepted ancestry, silently promote an unbound store, or infer authority from `HeaderProofFacts`. Exact-work checkpoint/all-handle reopening and canonical bootstrap remain Phase166; contextual prerequisites remain Phase167; actual pinned-daemon differential qualification remains Phase168. Ordinary support/refusal classes and default-off networking remain unchanged.

## Parent Test-Placement Revalidation

Before Plan 07, the parent moved the cfg-test module into `packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs` for the existing panic-policy scanner and preserved its logical module name. Only the relative fixture include gained one parent component, retaining the same work-vector corpus. Normalizing that include recovers the original SHA-256 `f276566c31babc1dac11eb67e3e992ca5afcf1bf2e83a8e14dd1540f2cb2b1a5`; test logic and assertions are unchanged. The pinned timed network phase165 run passed 29 tests, including the 12 header authority controls and 17 Plan 06 pure eligibility controls. Formatting, panic policy, breadcrumbs (1,171), managed checks and current source guards passed. No panic allowlist or scanner rule changed; whole-phase native acceptance remains pending.

## Self-Check: PASSED

All 12 owned source/mapping files and this summary exist. Focused and affected regressions, compilation, clippy, formatting, breadcrumbs and managed checks pass. No active executor verification sessions remain and the Git index is empty. Commit existence and shared tracker updates are intentionally deferred to the authorized parent wrapper.

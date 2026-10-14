---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "06"
subsystem: cli
tags: [rust, canonical-chain, nonmutating-refusal, wallet-backup, soak]
requires:
  - phase: 164-02
    provides: Non-opening ordinary storage compatibility refusal
  - phase: 164-05
    provides: Shared selected_canonical_chain selector resolution
  - phase: 164-13
    provides: Coupled GREEN and explicit synthetic RPC fixture contracts
provides:
  - Selected-network guards on all four CLI ordinary store opener sites
  - Typed bounded CLI refusal and truthful unavailable soak bounds
  - Physical source and backup destination preservation regressions
affects: [164-07, 164-08, 165, 166, 167]
tech-stack:
  added: []
  patterns: [Non-opening compatibility before effects, live RPC ownership separate from local storage]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility.rs
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
    - packages/open-bitcoin-cli/src/operator/wallet/name_resolution.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime/chain_compatibility.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/src/operator/wallet.rs
    - packages/open-bitcoin-cli/src/operator/wallet/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs
    - packages/open-bitcoin-cli/tests/operator_binary/compatibility_wallet.rs
    - packages/open-bitcoin-cli/tests/operator_binary/status_sync_soak.rs
    - packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse the RPC canonical selector with consistent mapped config selectors and the RuntimeConfig product default.
  - Live wallet sends pass the explicit optional wallet name to authenticated RPC without local registry opening.
  - Ordinary soak bounds remain unavailable; synthetic ledger projection regressions do not authorize ordinary startup.
requirements-completed: [CVLD-05]
requirements-addressed: [CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T05:09:09Z"
duration: 18min
completed: 2026-10-14
---

# Phase 164 Plan 06: Operator Store Compatibility Summary

**Offline sync, wallet lookup/export and soak enrichment refuse unproved sources before backend effects while preserving authenticated live RPC ownership.**

## Accomplishments and Effect Ordering

All three tasks completed in approximately 18 minutes. Four new Rust files and ten modified artifacts implement and verify the plan. Requirements remain incomplete pending parent-owned whole-phase verification.

| Ordinary surface | Guard and visible result |
| --- | --- |
| Offline sync Status/Pause/Resume | Resolved canonical selection is passed explicitly to `FjallNodeStore::open_ordinary` before runtime metadata load or sync-control persistence. Reachable RPC authentication/operation errors remain visible failures; transport unavailability can reach only the guarded offline path. |
| Managed wallet name lookup | Ordinary opener precedes registry load; compatibility errors retain `OperatorRuntimeError::CompatibilityRefused`. Unconfigured datadir also fails; failed opening/loading never yields a guessed wallet name. |
| Wallet backup | Ordinary opener precedes wallet-name validation, destination inspection, registry reads and export writes. Explicit wallet names and JSON format do not bypass the guard. |
| Soak enrichment/start | Ordinary opener precedes metadata reads. Refusal produces `FieldAvailability::Unavailable` with `chain_compatibility_unavailable` and bounded selected identity facts, preserving preexisting sync/peer observations. Startup propagates this reason before run index/ledger/report creation. |
| Live status/index/send/control | Existing authenticated RPC paths retain separate ownership. Send uses the existing explicit optional wallet endpoint selection without local name lookup. No local store opening exemption or fixture mode is introduced. |

`canonical_chain_from_resolution(&OperatorConfigResolution) -> Result<CanonicalChain, OperatorRuntimeError>` builds consistent existing address/sync selectors and calls `open_bitcoin_rpc::config::selected_canonical_chain`. An absent operator selector uses `RuntimeConfig::default`; no identity is inferred from the path or stored contents. Testnet maps to Testnet3. CLI typed refusal Display delegates to the fixed node diagnostic; JSON errors expose `chain_compatibility_refused` plus that safe diagnostic. Selected genesis/network and identity-only scope remain separate from unavailable observed network/root/work.

## Physical Preservation Evidence

Owned fixtures use real canonicalized temporary roots and closed sources: missing, empty, raw synthetic durable Fjall, malformed sentinels, canonical-looking text, unsafe files and symlinks. Durable fixtures contain runtime metadata plus wallet/registry/selected-wallet records. Raw opening is confined to test arrangement.

Manifests compare complete relative names, file types, bytes, lengths, modification times, permissions/modes and symlink targets, excluding access time. Tests exercise actual production helpers:

- Offline controls: 168 attempts across seven shapes, four networks, three operations and two formats; typed selected identity and unchanged physical roots after every attempt.
- Wallet-name lookup: 56 source attempts across seven shapes, four networks and explicit/absent names, plus two unconfigured-datadir failures; no guessed success.
- Backup: 336 attempts across seven shapes, four networks, explicit/absent names, nonexistent/existing/absent-parent destinations and both formats. Existing backup bytes and directories stay unchanged; absent outputs/parents stay absent.
- Soak enrichment: 28 attempts across seven shapes and four networks; bounded unavailable bounds, preserved existing sync/network/peer observations and unchanged physical roots.
- Live failures: actual local HTTP adapters receive auth and operation failures for sync and both wallet endpoint selections; unproved local sources remain unchanged. Existing binary live-success and auth regressions also pass.
- Actual binary ordinary controls/export/start prove refusal and source/destination preservation. Both elapsed-time and target-height soak startup fail before ledger creation. JSON offline refusal is parsed as JSON.

## Actual Verification

All Cargo commands ran serially through `bun run scripts/command-timings.ts run --key <key> --`. Logs are `/tmp/open-bitcoin-phase164-cli-<suffix>.log`.

| Check | Session | Actual result |
| --- | --- | --- |
| Offline RED, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib chain_compatibility` | 24922 | Exit 101: missing selected-chain helper/typed error/interface, plus corrected test visibility/Debug plumbing. No behavioral pass claimed. |
| Offline GREEN, key `phase164-cli-offline` | 37594 | Exit 0; 3 passed, 0 failed, 410 filtered. |
| Wallet RED, key `phase164-cli-wallet-red`, filter `chain_compatibility_wallet` | 38470 | Exit 101: absent guarded name/backup interfaces and extracted registry helper. |
| Wallet GREEN, key `phase164-cli-wallet` | 19437 | Exit 0; 2 passed, 0 failed, 413 filtered. |
| Soak RED plumbing, key `phase164-cli-soak-red` | 21842 | Exit 101: test visibility and explicit StatusRequest initialization corrected before behavioral RED. |
| Soak behavioral RED, key `phase164-cli-soak-red-behavior`, filter `chain_compatibility_soak` | 54414 | Exit 101; 1 failed: unproved-source resource bounds remained available under the old raw enrichment. |
| Soak GREEN, key `phase164-cli-soak` | 40505 | Exit 0; 1 passed, 0 failed, 415 filtered. |
| Combined canonical focused filter, key `phase164-cli-guards` | 40648 | Exit 0; 8 passed, 0 failed, 410 filtered. Includes live HTTP boundary tests. |
| Initial `--test operator_binary`, key `phase164-cli-operator-binary` | 69315 | Exit 101; 22 passed, 8 failed: three macOS symlink-path expectation mismatches and five tests requiring superseded ordinary synthetic soak enrichment. |
| Authorized binary adaptations, key `phase164-cli-operator-binary-recheck` | 13444 | Exit 0; 30 passed, 0 failed. A now-unused target-height fixture warning was subsequently eliminated by testing that ordinary refusal path too. |
| Full CLI library, key `phase164-cli-lib` | 60200 | Exit 0; 418 passed, 0 failed. |
| Final coupled regressions initial attempt, key `phase164-cli-final-regressions` | 4284 | Exit 101; 417 library tests passed, one owned TCP fixture failed from an inherited nonblocking socket; binary stage was not reached. |
| Final coupled regressions after socket fix, key `phase164-cli-final-regressions-recheck` | 48621 | Exit 0; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib --test operator_binary`: 418 library tests and 30 binary tests passed, zero failures/ignores; 5.63s and 11.27s respectively. |
| CLI all-target/all-feature Clippy, key `phase164-cli-clippy` | 58490 | Exit 0; `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings`; 14.20s. Final fixture-change recheck recorded below. |

Scoped Rust formatting/check mode passed for all 13 owned Rust paths. Breadcrumb verification passed for 1,160 Rust paths; managed checks reported zero file-length/lesson findings; `git diff --check` passed. Full native verifier, phase checker, independent review/security and normal-hook Git finalization remain parent-owned. No mainnet runtime, canonical bootstrap/reopen, external Knots differential run or production readiness is claimed.

## Deviations from Plan

**[Rule 2 - Cohesive file boundaries] Added three narrow child modules.** Parent authorized `runtime/chain_compatibility/tests.rs` for cfg-test physical fixtures, `wallet/name_resolution.rs` for guarded registry selection, and `soak/runtime/chain_compatibility.rs` for unavailable enrichment. Existing wallet/soak parents were near 628 lines; splits preserve coherent behavior without length exceptions. All four new files have registered breadcrumbs. No product fixture helper exists.

**[Rule 2 - Preserve separate live ownership] Avoided local name lookup before live Send.** Parent authorized direct existing optional wallet-name delegation to authenticated RPC. Actual root/named endpoint auth tests and both unchanged binary send-success tests pass. Backup and local registry resolution remain guarded.

**[Rule 3 - Directly affected regression setup] Adapted obsolete ordinary success expectations and scoped synthetic ledger setup.** Parent authorized the three modified binary child files. Ordinary sync/backup/start tests now prove the intentionally refused Phase 164 contract. Report/stop/resume/support tests arrange an explicitly synthetic four-event completed ledger and index, then execute the actual report projection; their behavior assertions remain intact. The external-wallet-destination pure validation regression remains unchanged and passing. Historical receipts, source identities, corpus digests and product defaults were not rewritten. Only owned datadir arguments are canonicalized to avoid macOS temporary-directory symlink classification; production unsafe-path checks are unchanged.

**[Rule 1 - Owned test fixture bug] Set accepted test RPC sockets to blocking mode.** The final combined library run exposed macOS inheritance of the nonblocking listener mode. The socket now explicitly becomes blocking before its bounded read timeout, matching the repository's existing HTTP fixture practice. Final coupled regressions pass; no production networking behavior changed.

## Simplification, Audit and Remaining Work

Kept one canonical resolver and the existing typed node refusal renderer. Removed redundant raw store setup from pure offline mutation-policy regressions; retained their original policy assertions after the earlier compatibility boundary. Extracted only cohesive name/enrichment responsibilities, with no new dependency, config switch, environment override, service action, public networking activation, storage decoder, migration or automatic repair.

The complete CLI raw-opener inventory now contains only `wallet/tests.rs` and `runtime/chain_compatibility/tests.rs`, each under existing `cfg(test)` ownership. All four product sites call `open_ordinary`; no production fixture factory is added. Live status/index are RPC consumers and do not open local Fjall. Owned diffs and new files were reviewed for effects, redaction, forbidden shortcuts and goal-blocking stubs; none were found. Empty collections in the explicitly synthetic test ledger are narrow fixture data, not product stubs. No unplanned endpoint/auth/schema/file-access security boundary was introduced beyond the plan's known local adapter surfaces.

Local AGENTS guidance, the Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards and both complete active lesson files informed execution. No authentication gate occurred.

## Task Commits and Shared State

Tasks 1, 2 and 3: deferred parent-owned strict phase finalization. No staging, commits, pushes, worktrees, hook changes or shared STATE/ROADMAP/REQUIREMENTS/todo edits were performed. No commit hashes or requirement completions are claimed. Parent owns final native verification, review/security, shared bookkeeping and Git finalization.

## Self-Check: PASSED

All 14 listed implementation artifacts and this summary exist. All prescribed GREEN logs exist; final source regressions pass 418 library and 30 binary tests. Final all-target/all-feature Clippy session 95875 exited zero in 3.48s after the socket fixture repair. Scoped rustfmt check, breadcrumb verification (1,160 paths), managed standards (zero findings), source audit and whitespace checks passed. Frontmatter has only its two leading delimiters. Commit verification is intentionally inapplicable under parent-owned finalization.

## Parent Accepted Requirement Closure | 2026-10-14

CVLD-05 has unique completion ownership in this summary after full default native session41473 exited zero (1h0m21.973s), clean 146-file review, 30 closed security controls and lifecycle-valid 27/27 phase verification. The original execution date and earlier evidence remain preserved. All four ordinary histories remain refused pending Phases165–167; no current ordinary qualification is asserted. Wrapper-owned normal-hook commit and push remain the final transaction.

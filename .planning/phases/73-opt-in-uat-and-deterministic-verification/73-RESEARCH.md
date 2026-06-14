# Phase 73: Opt-In UAT and Deterministic Verification - Research

**Researched:** 2026-06-13
**Domain:** Rust deterministic verification, Bun checker wiring, operator UAT documentation, parity evidence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

## Implementation Decisions

### Default Hermetic Verification

- **D-01:** Keep `bash scripts/verify.sh` as the repo-native deterministic
  verification contract. Extend it with a Phase 73 Bun checker only if that
  checker remains local, short-running, public-network-free, service-manager
  free, and timing-stable.
- **D-02:** Follow the existing Phase 61 through Phase 72 checker pattern:
  explicit required files, required test/doc needles, ordered checker wiring,
  and scoped forbidden-default-verification strings.
- **D-03:** The Phase 73 checker must guard against accidental default
  invocation of live-mainnet smoke, manual peers, `--restart-after-progress`,
  real `systemctl` or `launchctl`, `-openbitcoinsync=mainnet-ibd`, and
  current-tip or wall-clock release gates in `scripts/verify.sh`.
- **D-04:** Do not add strict Cargo/Bazel offline flags to the normal
  `scripts/verify.sh` path unless a post-bootstrap offline audit mode is
  deliberately documented. Fresh contributors should not be blocked by cache
  state while the default contract still forbids public-network runtime checks.

### Deterministic Coverage Scope

- **D-05:** Create a Phase 73 coverage map, implemented either inside the Phase
  73 checker or as a small local manifest consumed by it, that maps VER-02 to
  explicit existing or new deterministic tests for durable UTXO/undo writes,
  block connect/disconnect/reorg across restart, best-chain header selection,
  peer response failures, crash recovery, duplicate connect prevention, and
  resource bounds.
- **D-06:** Audit Phase 68 through Phase 72 tests before adding new tests. If a
  VER-02 behavior already has explicit assertions, reference it from the
  coverage map instead of duplicating fixtures.
- **D-07:** Add narrow hermetic Rust gap tests only where the audit finds missing
  explicit coverage. Prefer existing `DurableSyncRuntime`, Fjall temp-store,
  scripted transport, chainstate, block reconcile, and synthetic long-chain
  fixtures over a new process-level crash harness.
- **D-08:** Treat crash recovery as deterministic durable reopen/recovery
  evidence for this phase unless planning proves an actual process-level crash
  harness can be short, hermetic, platform-stable, and worth the added moving
  parts.

### Opt-In Public-Mainnet UAT Commands

- **D-09:** Add a central Phase 73 opt-in UAT matrix in
  `docs/operator/runtime-guide.md`, with links or short pointers from nearby
  workflow sections rather than scattering the authoritative command list.
- **D-10:** Every operator-facing UAT workflow in the matrix should include
  copy-pasteable repo-local Cargo and Bazel command forms where the workflow is
  served by the operator CLI. Use:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- **D-11:** The matrix should cover full-sync activation/review,
  stay-current/status review, same-datadir restart/resume review, status-surface
  comparison, live-smoke report collection, and support-bundle collection.
  `bun run scripts/run-live-mainnet-smoke.ts` remains the repo-owned wrapper for
  live public-mainnet evidence and must be labeled opt-in UAT.
- **D-12:** Command descriptions must state what evidence each workflow proves
  and what it does not prove. Bundle existence, daemon startup, elapsed time, or
  peer reachability alone are not sync-to-tip proof.

### Parity And Evidence Auditability

- **D-13:** Prefer the existing checker-plus-breadcrumb approach for a narrow
  Phase 73 closeout. Introduce a small phase-scoped evidence manifest only if
  planning adds enough non-Rust UAT/report/fixture surfaces that plain checker
  constants become hard to audit.
- **D-14:** Keep `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts` as the required path for any new
  first-party Rust source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.
- **D-15:** The Phase 73 checker should verify that new or referenced UAT,
  fixture, compatibility-harness, support-bundle, live-smoke, and deterministic
  checker surfaces are documented as local evidence and not as production-node,
  inbound-serving, relay, production-wallet, migration-apply, packaging, GUI,
  hosted-dashboard, or public-network CI claims.
- **D-16:** Do not introduce SLSA, in-toto, signed attestations, or generated
  provenance systems in Phase 73. Those are future release-engineering scope
  unless a later milestone explicitly adopts them.

### the agent's Discretion

- The planner may split Phase 73 into coverage audit/gap tests, UAT command
  matrix docs, deterministic checker wiring, and parity/auditability closeout.
- The executor may keep Phase 73 implementation mostly in docs and Bun checker
  code if the coverage map proves existing deterministic tests already satisfy
  VER-02.
- The executor may add a small manifest such as `docs/parity/v1.6-evidence.json`
  or an embedded checker constant table if that makes evidence mapping clearer,
  but should avoid broad generated artifacts or new dependencies.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Public-network CI, release-blocking live sync, current-tip timing thresholds,
  production-node/inbound serving/relay claims, production-funds wallet use,
  migration apply mode, packaging distribution, hosted dashboards, GUI, Windows
  service support, signed attestations, SLSA/in-toto provenance, and generated
  release attestation systems remain future scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| VER-01 | Contributor can run `bash scripts/verify.sh` without internet access, public peers, real service managers, long-running sync, or current-tip timing. [VERIFIED: .planning/REQUIREMENTS.md] | Add a Phase 73 checker after Phase 72 that requires ordered wiring and rejects live-mainnet smoke, manual peers, `--restart-after-progress`, service managers, `-openbitcoinsync=mainnet-ibd`, and current-tip/timing gates in `scripts/verify.sh`. [VERIFIED: scripts/verify.sh; scripts/check-phase72-observability-evidence.ts; 73-CONTEXT.md] |
| VER-02 | Contributor can run deterministic tests for durable UTXO/undo writes, block connect/disconnect/reorg across restart, best-chain header selection, peer response failures, crash recovery, duplicate connect prevention, and resource bounds. [VERIFIED: .planning/REQUIREMENTS.md] | Build an explicit coverage map from named existing tests, then add only narrow gap tests if a required behavior lacks an anchor. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-chainstate/tests/parity.rs; 68-VERIFICATION.md; 69-VERIFICATION.md; 70-VERIFICATION.md; 71-VERIFICATION.md] |
| VER-03 | Operator can run copy-pasteable repo-local Cargo and Bazel commands for opt-in public-mainnet full-sync, stay-current, restart/resume, and support-bundle UAT. [VERIFIED: .planning/REQUIREMENTS.md] | Centralize an opt-in UAT matrix in `docs/operator/runtime-guide.md` and use exact repo-local command forms for operator CLI workflows. [VERIFIED: 73-CONTEXT.md; AGENTS.md; docs/operator/runtime-guide.md; packages/open-bitcoin-cli/src/operator.rs] |
| VER-04 | Contributor can audit parity breadcrumbs, fixtures, compatibility harness reports, and deterministic checkers for every new v1.6 source, test, and operator-evidence surface. [VERIFIED: .planning/REQUIREMENTS.md] | Keep `scripts/check-parity-breadcrumbs.ts --check` as the Rust source/test audit mechanism and have the Phase 73 checker assert fixture, live-smoke, support-bundle, compatibility-harness, docs, and checker evidence anchors. [VERIFIED: scripts/check-parity-breadcrumbs.ts; docs/parity/source-breadcrumbs.json; scripts/check-phase66-compatibility-wrapper.ts; scripts/check-phase72-observability-evidence.ts] |
</phase_requirements>

## Summary

Phase 73 should be planned as a verification closeout and evidence-mapping phase, not as a broad sync-runtime implementation phase. [VERIFIED: 73-CONTEXT.md; .planning/ROADMAP.md] The strongest path is to add `scripts/check-phase73-uat-verification.ts`, wire it after `scripts/check-phase72-observability-evidence.ts` in `scripts/verify.sh`, and make it validate a VER-02 coverage map plus UAT/evidence documentation anchors. [VERIFIED: scripts/verify.sh; scripts/check-phase72-observability-evidence.ts; packages/open-bitcoin-node/src/sync/tests.rs]

Existing deterministic tests already cover most or all VER-02 behaviors through `DurableSyncRuntime`, Fjall temp stores, scripted transports, chainstate snapshots, branch/reorg reconciliation, same-datadir reopen, and synthetic long-chain fixtures. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-chainstate/tests/parity.rs] Planning should start with a coverage audit and only add narrow Rust tests if a required behavior cannot be named and checked by the Phase 73 map. [VERIFIED: 73-CONTEXT.md; 68-VERIFICATION.md; 69-VERIFICATION.md; 70-VERIFICATION.md; 71-VERIFICATION.md]

The operator-facing work should be a central UAT matrix in `docs/operator/runtime-guide.md` with Cargo and Bazel command forms for CLI-backed review surfaces, plus Bun live-smoke commands labeled as explicit public-mainnet opt-in UAT. [VERIFIED: 73-CONTEXT.md; AGENTS.md; docs/operator/runtime-guide.md; scripts/run-live-mainnet-smoke.ts] The matrix must state what each workflow proves and what it does not prove so elapsed time, daemon startup, peer reachability, and support-bundle existence are not mistaken for sync-to-tip proof. [VERIFIED: 73-CONTEXT.md; docs/operator/runtime-guide.md; scripts/check-phase72-observability-evidence.ts]

**Primary recommendation:** Implement Phase 73 as four plans: coverage map and gap-test audit, UAT matrix docs, Phase 73 deterministic checker wiring, and parity/evidence closeout. [VERIFIED: 73-CONTEXT.md]

## Project Constraints (from AGENTS.md)

- Read `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant pinned Bright Builds standards before planning or implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; Bright Builds standards URLs]
- Use `rust-toolchain.toml` as the Rust source of truth; this repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml; cargo --version; rustc --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use repo-local Cargo and Bazel commands for UAT operator workflows instead of only naming an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts and prefer TypeScript for substantial script logic. [VERIFIED: AGENTS.md; .bun-version; scripts/check-phase72-observability-evidence.ts]
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Add parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts` for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`. [VERIFIED: AGENTS.md; .planning/PROJECT.md; docs/parity/index.json]
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; .planning/ARCHITECTURE.md]
- Do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- No project skill directories were found at `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Library/Tool | Version | Purpose | Why Standard |
|--------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | Compile, lint, build, and test first-party Rust crates. | Pinned by `rust-toolchain.toml` and required by repo guidance. [VERIFIED: rust-toolchain.toml; cargo --version; rustc --version; AGENTS.md] |
| Cargo workspace | Workspace package version 0.1.0, edition 2024 | Owns `open-bitcoin-*` crates and deterministic Rust tests. | `packages/Cargo.toml` defines the workspace and member crates used by Phase 73 coverage. [VERIFIED: packages/Cargo.toml; cargo tree] |
| Bun | 1.3.9 | Runs TypeScript verification scripts and live-smoke fixtures. | Repo guidance names Bun as canonical for repo-owned automation; existing checkers are `#!/usr/bin/env bun`. [VERIFIED: .bun-version; bun --version; AGENTS.md; scripts/check-phase72-observability-evidence.ts] |
| TypeScript Bun checkers | Existing repo-owned scripts | Enforce deterministic evidence, docs, and default-verification boundaries. | Phase 68 through Phase 72 checkers use explicit file/needle assertions and ordered `scripts/verify.sh` wiring. [VERIFIED: scripts/check-phase68-active-chain-persistence.ts; scripts/check-phase72-observability-evidence.ts; scripts/verify.sh] |
| Fjall | 3.1.4 | Durable local store for headers, block index, chainstate, runtime metadata, metrics, and wallet state. | Existing deterministic tests use `FjallNodeStore::open` and same-datadir reopen evidence. [VERIFIED: packages/Cargo.lock; cargo tree -p open-bitcoin-node; packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-node/src/sync/tests.rs] |
| `serde` / `serde_json` | 1.0.228 / 1.0.149 | Stable JSON data shapes for status, support evidence, live-smoke reports, and checker parsing. | Existing node/CLI/RPC crates use these dependencies for stable operator evidence surfaces. [VERIFIED: packages/Cargo.lock; cargo tree -p open-bitcoin-node; cargo tree -p open-bitcoin-cli; cargo tree -p open-bitcoin-rpc] |
| Bazel | 8.6.0 | Top-level smoke build and repo-local UAT command form. | `scripts/verify.sh` builds Bazel smoke targets and repo docs require Bazel UAT forms. [VERIFIED: bazel --version; scripts/verify.sh; AGENTS.md] |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `clap` | 4.6.1 | Operator CLI command grammar. | Use existing `OperatorCommand`, `SyncCommand`, `ServiceCommand`, and `SupportCommand` forms when documenting UAT. [VERIFIED: packages/Cargo.lock; cargo tree -p open-bitcoin-cli; packages/open-bitcoin-cli/src/operator.rs] |
| `axum` / `tokio` | 0.8.9 / 1.52.1 | Local RPC server and async runtime. | Reference only for status/RPC surfaces; Phase 73 should not add runtime behavior unless a deterministic gap test demands it. [VERIFIED: packages/Cargo.lock; cargo tree -p open-bitcoin-rpc; 73-CONTEXT.md] |
| `ratatui` / `crossterm` | 0.30.0 / 0.29.0 | Terminal dashboard/status surfaces. | Use as existing evidence surfaces, not as new Phase 73 implementation targets. [VERIFIED: packages/Cargo.lock; cargo tree -p open-bitcoin-cli; scripts/check-phase72-observability-evidence.ts] |
| `cargo-llvm-cov` | 0.8.5 | Pure-core coverage gate in `scripts/verify.sh`. | Keep current verifier behavior; do not add public-network or timing gates. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |
| `git` | 2.53.0 | Breadcrumb checker tracks first-party Rust files through `git ls-files`. | Required by `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: git --version; scripts/check-parity-breadcrumbs.ts] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Embedded checker coverage table | `docs/parity/v1.6-evidence.json` manifest | Use an external manifest only if Phase 73 adds enough non-Rust UAT/report/fixture surfaces that checker constants become hard to audit. [VERIFIED: 73-CONTEXT.md] |
| Bun TypeScript checker | Bash checker | Existing Phase 61 through Phase 72 pattern uses Bun/TypeScript for structured file reads and assertions; Bash should stay thin orchestration. [VERIFIED: AGENTS.md; scripts/check-phase72-observability-evidence.ts; scripts/verify.sh] |
| Existing deterministic reopen tests | New process-level crash harness | Treat crash recovery as deterministic durable reopen/recovery evidence unless planning proves a process-level harness is short, hermetic, and platform-stable. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs] |
| Default offline Cargo/Bazel flags | Normal `scripts/verify.sh` behavior | Do not add strict offline flags to the normal verifier because fresh contributors may lack caches while runtime public-network checks still remain forbidden. [VERIFIED: 73-CONTEXT.md] |

**Installation:**

```bash
# No new packages are recommended for Phase 73.
# Use the pinned local tools already required by the repo.
bun --version
cargo --version
bazel --version
cargo llvm-cov --version
```

**Version verification:** Versions above were verified with local tool commands, `rust-toolchain.toml`, `.bun-version`, `packages/Cargo.toml`, `packages/Cargo.lock`, and `cargo tree --manifest-path packages/Cargo.toml -p ... --depth 1`. [VERIFIED: local command output]

## Architecture Patterns

### Recommended Project Structure

```text
scripts/
  check-phase73-uat-verification.ts    # deterministic Phase 73 evidence map and boundary checker
docs/operator/
  runtime-guide.md                     # central opt-in UAT matrix and command forms
docs/parity/
  source-breadcrumbs.json              # update only if new first-party Rust source/test files are added
  v1.6-evidence.json                   # optional only if checker constants become hard to audit
packages/open-bitcoin-node/src/sync/
  tests.rs                             # add narrow gap tests only if coverage map lacks explicit VER-02 anchors
```

This structure matches the existing checker-plus-docs-plus-Rust-test pattern from Phases 68 through 72. [VERIFIED: scripts/check-phase68-active-chain-persistence.ts; scripts/check-phase72-observability-evidence.ts; docs/operator/runtime-guide.md; packages/open-bitcoin-node/src/sync/tests.rs]

### Pattern 1: Deterministic Phase Checker

**What:** A Bun TypeScript checker reads required files, asserts required evidence needles, verifies checker ordering in `scripts/verify.sh`, and rejects forbidden default-verification strings. [VERIFIED: scripts/check-phase71-resource-restart.ts; scripts/check-phase72-observability-evidence.ts]

**When to use:** Use for Phase 73 closeout because the phase is mostly an evidence, default-verification, and docs audit. [VERIFIED: 73-CONTEXT.md]

**Example:**

```typescript
const phase72 = "bun run scripts/check-phase72-observability-evidence.ts";
const phase73 = "bun run scripts/check-phase73-uat-verification.ts";

requireContains(verifyScript, phase72, "scripts/verify.sh", failures);
requireContains(verifyScript, phase73, "scripts/verify.sh", failures);

if (verifyScript.indexOf(phase73) < verifyScript.indexOf(phase72)) {
  failures.push("scripts/verify.sh must run the Phase 73 checker after the Phase 72 checker");
}

for (const forbidden of [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
]) {
  requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
}
```

Source pattern: existing Phase 71 and Phase 72 checkers use this same ordered-checker and forbidden-string pattern. [VERIFIED: scripts/check-phase71-resource-restart.ts; scripts/check-phase72-observability-evidence.ts]

### Pattern 2: VER-02 Coverage Map Before New Tests

**What:** Map each VER-02 behavior to exact test names and files, then make the checker fail if those anchors disappear. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs]

**When to use:** Use before adding Rust tests because prior phases already passed deterministic verification for active-chain persistence, tip/stay-current, reorg/peer recovery, resource bounds, and support evidence. [VERIFIED: 68-VERIFICATION.md; 69-VERIFICATION.md; 70-VERIFICATION.md; 71-VERIFICATION.md; 72-VERIFICATION.md]

| VER-02 behavior | Existing deterministic anchor | Confidence |
|-----------------|-------------------------------|------------|
| Durable UTXO/undo writes | `connect_disconnect_and_reorg_preserve_phase_four_outcomes`; `ChainstateSnapshot` includes `utxos` and `undo_by_block`; snapshot codec serializes both. [VERIFIED: packages/open-bitcoin-chainstate/tests/parity.rs; packages/open-bitcoin-chainstate/src/types.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs] | HIGH |
| Block connect across restart | `connected_active_chain_progress_survives_runtime_reopen`; `first_non_genesis_block_connect_advances_downloaded_and_connected_height`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH |
| Disconnect/reorg across restart | `phase70_reorg_records_bounded_persisted_evidence`; `same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH |
| Best-chain header selection | `competing_header_branch_wins_after_restart_when_it_extends_farther`; `phase70_equal_or_lower_work_side_branch_does_not_replace_active_tip`; `prefer_candidate_tip`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-chainstate/src/engine.rs] | HIGH |
| Peer response failures | `block_notfound_is_peer_attributed_no_credit`; `phase70_notfound_releases_inflight_and_rotates_to_second_peer`; malformed/invalid/duplicate/disconnected/non-extending Phase 70 peer tests; `connect_failures_are_reported_as_peer_outcomes`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH |
| Crash recovery | `phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight`; `same_datadir_reopen_seeds_headers_from_durable_store`; `phase70_malformed_stored_chainstate_is_storage_blocker`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH for deterministic reopen/recovery; process-crash harness intentionally not required by current decisions. [VERIFIED: 73-CONTEXT.md] |
| Duplicate connect prevention | `same_datadir_reopen_does_not_duplicate_connected_block_getdata`; `duplicate_block_response_is_peer_attributed_no_credit`; `phase70_duplicate_block_releases_inflight_without_credit`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH |
| Resource bounds | `phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`; `bounded_unattended_cycles_preserve_resource_pressure_and_retention`; `bounded_block_requests_respect_per_peer_and_total_caps`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | HIGH |

### Pattern 3: Central Opt-In UAT Matrix

**What:** Put the authoritative Phase 73 public-mainnet UAT matrix in `docs/operator/runtime-guide.md`, then link or point to it from nearby sections instead of duplicating scattered command lists. [VERIFIED: 73-CONTEXT.md; docs/operator/runtime-guide.md]

**When to use:** Use for VER-03 because `docs/operator/runtime-guide.md` already has many commands spread across mainnet sync, live smoke, v1.4, v1.5, and support sections. [VERIFIED: docs/operator/runtime-guide.md]

**Required workflows:** full-sync activation/review, stay-current/status review, same-datadir restart/resume, status-surface comparison, live-smoke report collection, and support-bundle collection. [VERIFIED: 73-CONTEXT.md]

### Anti-Patterns to Avoid

- **Adding public-network checks to `scripts/verify.sh`:** The default verifier must remain deterministic and public-network-free. [VERIFIED: 73-CONTEXT.md; scripts/verify.sh]
- **Duplicating existing deterministic tests:** Existing named tests already cover most VER-02 behaviors; duplicate fixtures increase maintenance cost without improving auditability. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; 73-CONTEXT.md]
- **Treating daemon startup or bundle existence as proof:** Operator docs must distinguish evidence fields from mere process or file presence. [VERIFIED: 73-CONTEXT.md; docs/operator/runtime-guide.md]
- **Introducing provenance systems:** SLSA, in-toto, signed attestations, and generated release provenance are deferred beyond Phase 73. [VERIFIED: 73-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Durable test harness | New process-level crash framework by default | Existing Fjall temp stores, `DurableSyncRuntime::open`, same-datadir reopen, scripted transports | Current decisions define crash recovery as deterministic reopen/recovery evidence unless a better short hermetic harness is proven. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs] |
| Public-mainnet evidence runner | New network UAT wrapper | `bun run scripts/run-live-mainnet-smoke.ts` | Existing wrapper already handles explicit opt-in daemon launch, manual peers, restart-after-progress, polling, JSON/Markdown reports, and deterministic fixture tests. [VERIFIED: scripts/run-live-mainnet-smoke.ts; scripts/test-run-live-mainnet-smoke.sh] |
| Operator CLI command examples | Installed alias-only docs | Repo-local Cargo and Bazel command forms | Repo guidance requires copy-pasteable repo-local commands for UAT. [VERIFIED: AGENTS.md; 73-CONTEXT.md] |
| Source/test audit | Manual checklist for Rust breadcrumbs | `docs/parity/source-breadcrumbs.json` plus `scripts/check-parity-breadcrumbs.ts --check` | The checker validates tracked in-scope Rust files, mapping shape, breadcrumb target existence, and source comments. [VERIFIED: scripts/check-parity-breadcrumbs.ts; docs/parity/source-breadcrumbs.json] |
| Evidence redaction | New ad hoc support sanitizer | Existing support bundle and live-smoke summary allowlists | Phase 72 support evidence already avoids raw logs, raw peer tables, credentials, wallet material, and raw live-smoke input. [VERIFIED: scripts/check-phase72-observability-evidence.ts; packages/open-bitcoin-cli/src/operator/support.rs; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |

**Key insight:** Phase 73's risk is not missing a library; it is accidentally expanding the default verification contract or making unverifiable public-mainnet claims. [VERIFIED: 73-CONTEXT.md; .planning/REQUIREMENTS.md]

## Common Pitfalls

### Pitfall 1: Accidentally Making Default Verification Live

**What goes wrong:** A checker, docs command, or script call pulls `run-live-mainnet-smoke`, manual peers, service-manager actions, `-openbitcoinsync=mainnet-ibd`, or timing thresholds into `bash scripts/verify.sh`. [VERIFIED: 73-CONTEXT.md; scripts/check-phase72-observability-evidence.ts]

**Why it happens:** Live UAT commands are valid operator workflows but invalid default verifier steps. [VERIFIED: docs/operator/runtime-guide.md; 73-CONTEXT.md]

**How to avoid:** Keep live commands only in docs and opt-in scripts; make the Phase 73 checker reject forbidden strings in `scripts/verify.sh`. [VERIFIED: scripts/check-phase71-resource-restart.ts; scripts/check-phase72-observability-evidence.ts]

**Warning signs:** `scripts/verify.sh` contains `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl`, `launchctl`, `openbitcoinsync=mainnet-ibd`, "current tip", or wall-clock pass/fail thresholds. [VERIFIED: 73-CONTEXT.md]

### Pitfall 2: Overbuilding Gap Tests

**What goes wrong:** New fixtures duplicate Phase 68 through Phase 72 behavior rather than making existing evidence auditable. [VERIFIED: 68-VERIFICATION.md; 69-VERIFICATION.md; 70-VERIFICATION.md; 71-VERIFICATION.md; 72-VERIFICATION.md]

**Why it happens:** VER-02 is broad, but the broad behavior was mostly implemented and verified in prior phases. [VERIFIED: .planning/REQUIREMENTS.md; prior verification files]

**How to avoid:** Make a coverage map first and add only missing, named, hermetic assertions. [VERIFIED: 73-CONTEXT.md]

**Warning signs:** A planned test creates new public-network, real-service-manager, sleep/timing, or process-crash infrastructure before the coverage map identifies a gap. [VERIFIED: 73-CONTEXT.md]

### Pitfall 3: UAT Commands Without Proof Semantics

**What goes wrong:** Docs list commands but do not say whether they prove sync-to-tip, stay-current behavior, restart/resume, a diagnosed blocker, or only local artifact collection. [VERIFIED: 73-CONTEXT.md; docs/operator/runtime-guide.md]

**Why it happens:** Existing commands are spread across older v1.4/v1.5 sections and support sections. [VERIFIED: docs/operator/runtime-guide.md]

**How to avoid:** Centralize the matrix and add an "proves / does not prove" column for every workflow. [VERIFIED: 73-CONTEXT.md]

**Warning signs:** Wording treats elapsed time, daemon startup, peer reachability, or bundle existence as correctness proof. [VERIFIED: 73-CONTEXT.md]

### Pitfall 4: Breadcrumb Drift

**What goes wrong:** New Rust test/source files are added without updating `docs/parity/source-breadcrumbs.json` and source breadcrumb comments. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]

**Why it happens:** Docs and scripts do not use source breadcrumbs, but Rust files under the scoped paths do. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

**How to avoid:** If Phase 73 adds Rust files, update the mapping and run `bun run scripts/check-parity-breadcrumbs.ts --check`. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]

**Warning signs:** The breadcrumb checker reports missing mapping, duplicate mapping, missing target, or source comment drift. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

## Code Examples

Verified patterns from existing source:

### Coverage Map Constant

```typescript
const VER02_COVERAGE = [
  {
    behavior: "durable_utxo_undo_writes",
    files: [
      "packages/open-bitcoin-chainstate/tests/parity.rs",
      "packages/open-bitcoin-node/src/storage/snapshot_codec.rs",
    ],
    needles: [
      "connect_disconnect_and_reorg_preserve_phase_four_outcomes",
      "undo_by_block",
      "utxos",
    ],
  },
  {
    behavior: "resource_bounds",
    files: ["packages/open-bitcoin-node/src/sync/tests.rs"],
    needles: [
      "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
      "max_blocks_in_flight_total",
      "max_sync_rounds",
    ],
  },
] as const;
```

Use this pattern to make VER-02 auditable through explicit source/test anchors. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-chainstate/tests/parity.rs]

### UAT Matrix Command Forms

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/tmp/open-bitcoin-mainnet \
  --manual-peer=HOST:8333 \
  --restart-after-progress \
  --timeout-seconds=180 \
  --poll-seconds=10
```

These forms match repo guidance and existing operator docs; the Bun command must remain labeled opt-in public-mainnet UAT. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md; scripts/run-live-mainnet-smoke.ts]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Default verification as generic local test runner | `scripts/verify.sh` as a repo-native contract with deterministic TypeScript checkers through Phase 72 plus Rust/Bazel/coverage gates | Existing by Phase 72 | Phase 73 should add one more deterministic checker, not a new verification system. [VERIFIED: scripts/verify.sh; scripts/check-phase72-observability-evidence.ts] |
| Public-network evidence as default pass/fail | Public-mainnet live smoke as explicit local UAT with JSON/Markdown reports | Existing by v1.4/v1.5 and Phase 72 | Keep `run-live-mainnet-smoke.ts` out of `scripts/verify.sh` and document it as opt-in review evidence. [VERIFIED: docs/operator/runtime-guide.md; scripts/run-live-mainnet-smoke.ts; scripts/test-run-live-mainnet-smoke.sh] |
| Renderer-specific status meanings | Shared full-sync truth contract across CLI, dashboard, RPC sync status, metrics/logs, live-smoke, and support bundles | Phase 72 | Phase 73 should validate the evidence surfaces rather than redefine the status model. [VERIFIED: 72-VERIFICATION.md; scripts/check-phase72-observability-evidence.ts] |
| Manual source/test parity review | `source-breadcrumbs.json` plus checker-managed source comments | Existing repo mechanism | New Rust source/test files require breadcrumb mapping; docs/scripts require checker evidence anchors instead. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts] |

**Deprecated/outdated:**

- Using ASVS v4 numbering for new security references is outdated because OWASP lists ASVS 5.0.0 as the latest stable version and ASVS 5.0 renumbered/reorganized chapters. [CITED: https://owasp.org/www-project-application-security-verification-standard/; VERIFIED: https://api.github.com/repos/OWASP/ASVS/contents/5.0/en?ref=v5.0.0]
- Treating public-network checks as release gates is out of scope for v1.6 Phase 73. [VERIFIED: .planning/REQUIREMENTS.md; 73-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified or cited in this session; no `[ASSUMED]` claims are present. [VERIFIED: source audit and cited docs]

## Open Questions (RESOLVED)

1. **Should Phase 73 use an external evidence manifest or embedded checker constants?**
   - What we know: Context allows either embedded checker constants or a small manifest such as `docs/parity/v1.6-evidence.json`. [VERIFIED: 73-CONTEXT.md]
   - What's unclear: The exact amount of Phase 73 non-Rust evidence surface is unknown until the planner chooses task granularity. [VERIFIED: 73-CONTEXT.md]
   - Recommendation: Start with embedded constants; add a manifest only if the checker becomes hard to scan. [VERIFIED: scripts/check-phase72-observability-evidence.ts; 73-CONTEXT.md]
   - RESOLVED: Use embedded checker constants for the VER-02 coverage map and Phase 73 evidence map unless execution proves a manifest is needed for auditability. [VERIFIED: 73-01-PLAN.md; 73-03-PLAN.md; 73-04-PLAN.md]

2. **Are additional Rust gap tests needed?**
   - What we know: The audit found named existing anchors for every VER-02 behavior. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-chainstate/tests/parity.rs]
   - What's unclear: The planner may require an even more explicit assertion for durable UTXO/undo persistence across a node restart rather than chainstate parity plus snapshot codec coverage. [VERIFIED: 73-CONTEXT.md]
   - Recommendation: Plan a first task that writes the coverage map and runs the Phase 73 checker; only add a narrow test if the map exposes an unambiguous missing assertion. [VERIFIED: 73-CONTEXT.md]
   - RESOLVED: Add no new Rust gap tests unless execution proves an existing deterministic anchor is missing. The planned default is checker-only coverage mapping against existing anchors. [VERIFIED: 73-01-PLAN.md; 73-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust/Cargo | Rust tests, builds, clippy | yes | `cargo 1.94.1`; `rustc 1.94.1` | Blocking if missing. [VERIFIED: cargo --version; rustc --version; rust-toolchain.toml] |
| Bun | Phase 73 checker and existing TypeScript scripts | yes | `1.3.9` | Blocking for checker work; no package install step exists. [VERIFIED: bun --version; .bun-version; AGENTS.md] |
| Bazel | Verify smoke build and UAT command form | yes | `8.6.0` | Blocking for full `scripts/verify.sh`; docs can still include Bazel form. [VERIFIED: bazel --version; scripts/verify.sh] |
| cargo-llvm-cov | Existing pure-core coverage gate | yes | `0.8.5` | Blocking for full `scripts/verify.sh`. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |
| Git | Breadcrumb checker tracked-file scan | yes | `2.53.0` | Blocking for `check-parity-breadcrumbs.ts`. [VERIFIED: git --version; scripts/check-parity-breadcrumbs.ts] |
| Public internet / public peers | Opt-in UAT only | not required for default verification | n/a | Use deterministic fixtures and local status/support checks by default. [VERIFIED: 73-CONTEXT.md; scripts/verify.sh] |
| Real service managers | Opt-in service UAT only | not required for default verification | n/a | Use docs and fake-manager deterministic tests by default. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-cli/src/operator/service.rs; packages/open-bitcoin-cli/src/operator/service/tests.rs] |

**Missing dependencies with no fallback:**

- None found for research and planning. [VERIFIED: local tool probes]

**Missing dependencies with fallback:**

- Public internet, public peers, and real service managers are intentionally not required for default verification; they remain opt-in UAT surfaces. [VERIFIED: 73-CONTEXT.md; scripts/verify.sh]

## Security Domain

`security_enforcement` is absent from `.planning/config.json`, so the security section is included. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS 5.0 Category | Applies | Standard Control |
|-------------------|---------|------------------|
| V1 Encoding and Sanitization | limited | Keep checker string matching scoped to repo file content and avoid shell interpolation in new scripts. [VERIFIED: OWASP ASVS v5 file list; scripts/check-phase72-observability-evidence.ts] |
| V2 Validation and Business Logic | yes | Validate UAT proof semantics and business claim boundaries in checker/docs. [VERIFIED: OWASP ASVS v5 file list; 73-CONTEXT.md] |
| V4 API and Web Service | limited | Do not change RPC behavior unless a deterministic test gap is found; preserve Open Bitcoin-specific sync status separately from baseline `getblockchaininfo`. [VERIFIED: OWASP ASVS v5 file list; packages/open-bitcoin-rpc/src/dispatch/tests.rs; 72-VERIFICATION.md] |
| V5 File Handling | yes | Support bundles and live-smoke reports must remain local, bounded, and redacted. [VERIFIED: OWASP ASVS v5 file list; scripts/check-phase72-observability-evidence.ts; packages/open-bitcoin-cli/src/operator/support.rs] |
| V6 Authentication | no new auth | Phase 73 should not alter RPC auth or cookie behavior. [VERIFIED: 73-CONTEXT.md; packages/open-bitcoin-cli/src/operator/status.rs] |
| V7 Session Management | no | No web session surface is in Phase 73. [VERIFIED: .planning/ROADMAP.md; 73-CONTEXT.md] |
| V8 Authorization | limited | Do not add new mutating UAT commands to default verification; keep service and public-mainnet actions opt-in. [VERIFIED: 73-CONTEXT.md; docs/operator/runtime-guide.md] |
| V11 Cryptography | no new crypto | Do not add cryptographic provenance or attestation systems in Phase 73. [VERIFIED: 73-CONTEXT.md] |
| V13 Configuration | yes | Docs must keep `open-bitcoin.jsonc`, daemon flags, manual peers, and datadir ownership explicit. [VERIFIED: docs/operator/runtime-guide.md; AGENTS.md] |
| V15 Secure Coding and Architecture | yes | Keep functional-core/imperative-shell boundaries and use existing typed Rust domain helpers. [VERIFIED: AGENTS.md; Bright Builds Rust standard; packages/open-bitcoin-chainstate/src/engine.rs] |
| V16 Security Logging and Error Handling | yes | Evidence docs/checkers should preserve typed blockers and redaction boundaries. [VERIFIED: OWASP ASVS v5 file list; scripts/check-phase72-observability-evidence.ts] |

### Known Threat Patterns for Phase 73

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Default verifier contacts public peers or real services | Information Disclosure / Denial of Service | Forbidden-string guard in Phase 73 checker and ordered `scripts/verify.sh` wiring. [VERIFIED: 73-CONTEXT.md; scripts/check-phase72-observability-evidence.ts] |
| UAT docs overclaim production readiness | Spoofing / Repudiation | Matrix "proves / does not prove" fields and parity deferred-scope needles. [VERIFIED: 73-CONTEXT.md; docs/parity/catalog/p2p.md; docs/parity/catalog/chainstate.md] |
| Support/live-smoke evidence leaks raw logs, credentials, peers, or wallet material | Information Disclosure | Reuse existing support/live-smoke allowlists and have Phase 73 checker assert redaction boundary text. [VERIFIED: scripts/check-phase72-observability-evidence.ts; packages/open-bitcoin-cli/src/operator/support.rs] |
| New Rust source/test files lack parity anchors | Repudiation | Update `docs/parity/source-breadcrumbs.json` and run `scripts/check-parity-breadcrumbs.ts --check`. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md` - locked Phase 73 decisions, scope, and canonical refs. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - VER-01 through VER-04 and v1.6 out-of-scope boundaries. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 73 goal, dependency, success criteria, and Phase 74 boundary. [VERIFIED: file read]
- `.planning/STATE.md` - current milestone state and prior decisions. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo and Bright Builds workflow constraints. [VERIFIED: file read]
- `scripts/verify.sh` - deterministic verifier ordering and current tool gates. [VERIFIED: file read]
- `scripts/check-phase68-active-chain-persistence.ts` through `scripts/check-phase72-observability-evidence.ts` - existing checker pattern. [VERIFIED: file reads]
- `packages/open-bitcoin-node/src/sync/tests.rs` and `packages/open-bitcoin-chainstate/tests/parity.rs` - deterministic coverage anchors. [VERIFIED: file reads and rg]
- `docs/operator/runtime-guide.md`, `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`, `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` - operator and parity evidence roots. [VERIFIED: file reads]
- Prior phase verification files for Phases 68 through 72. [VERIFIED: file reads]
- Local tool commands for Rust, Bun, Bazel, cargo-llvm-cov, Git, and Cargo dependency trees. [VERIFIED: command output]
- OWASP ASVS 5.0.0 official project and GitHub file listing. [CITED: https://owasp.org/www-project-application-security-verification-standard/; VERIFIED: https://api.github.com/repos/OWASP/ASVS/contents/5.0/en?ref=v5.0.0]

### Secondary (MEDIUM confidence)

- Bright Builds canonical standards pages loaded from raw GitHub URLs because no local `standards/` directory exists. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/core/verification.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/core/testing.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/languages/rust.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/languages/typescript-javascript.md]

### Tertiary (LOW confidence)

- None. [VERIFIED: source audit]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - no new dependencies are recommended; versions were verified from local tool commands, Cargo files, and Cargo lock/tree output. [VERIFIED: command output; packages/Cargo.lock]
- Architecture: HIGH - Phase 68 through Phase 72 already establish the checker/docs/test structure and Phase 73 decisions lock that direction. [VERIFIED: 73-CONTEXT.md; scripts/check-phase72-observability-evidence.ts]
- Pitfalls: HIGH - forbidden default-verification strings and opt-in UAT boundaries are explicit in the context and prior checkers. [VERIFIED: 73-CONTEXT.md; scripts/check-phase71-resource-restart.ts; scripts/check-phase72-observability-evidence.ts]

**Research date:** 2026-06-13
**Valid until:** 2026-07-13 for codebase-local verifier architecture; re-check ASVS and dependency versions before security or dependency policy changes. [VERIFIED: local codebase state; CITED: OWASP ASVS project]

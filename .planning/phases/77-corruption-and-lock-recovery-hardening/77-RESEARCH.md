# Phase 77: Corruption and Lock Recovery Hardening - Research

**Researched:** 2026-06-15  
**Domain:** Rust durable storage recovery diagnostics, Fjall lock handling, shared operator status contracts  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md`. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]

### Locked Decisions

### D-01: Add a Probe-Only Lock Evidence Path
Implement a read-only/probe-only lock evidence path for status/support/recovery inspection.

This path must not:
- create missing schema records
- write recovery markers
- clear stale artifacts
- repair stores
- delete lock files
- otherwise mutate the datadir

Normal runtime store open may still follow the existing adapter behavior when the daemon explicitly starts, but operator-facing inspection must not hide mutation.

### D-02: Distinguish Three Lock/Datadir Cases
Phase 77 must distinguish at least:
- active lock contention
- stale-lock evidence
- concurrent datadir use

Concurrent datadir use may combine lock evidence with service status, same-datadir service evidence, live RPC availability, or other existing repo-local signals. Do not depend on non-portable process scans as the only proof.

### D-03: Preserve Adapter Error Mapping
Backend open failures still map through `StorageError` and `SyncRecoveryCategory::StorageLockContention` when the real storage adapter reports lock/contention.

The read-only evidence path complements adapter errors. It must not replace real adapter error mapping or mask backend failures.

### D-04: Defer Owner Heartbeat/PID Sentinels
Do not add a required owner heartbeat or PID sentinel contract in Phase 77.

It can be mentioned as a future enhancement only if the implementation leaves a clean place to add it later.

### D-05: Preserve Stable Recovery Category Labels
Keep existing stable labels intact:
- `incompatible_schema`
- `store_corruption`
- `storage_lock_contention`
- `storage_backend_failure`
- `resource_exhaustion`

Do not rename existing categories. Add richer evidence beside them instead.

### D-06: Add Typed Recovery Evidence
Add typed recovery evidence beside the category/action summary.

Evidence should capture causes/details such as:
- schema mismatch
- corruption marker
- partial/interrupted write
- unreadable namespace/store
- backend open failure
- active lock
- stale lock evidence
- concurrent datadir evidence

### D-07: Add Guidance Action Classes
REC-07 guidance must use action classes that separate:
- `safe_retry`
- `read_only_inspection`
- `backup_then_rebuild`
- `stop_and_escalate`

Existing `StorageRecoveryAction` may feed compatibility text, but the new action class is the durable safety contract.

### D-08: Centralize Classification
Centralize mapping from storage errors, lock evidence, recovery markers, and status collectors into one pure classifier.

Renderers/reports should consume classifier output instead of each re-implementing string matching.

### D-09: Fallback Backend Failures
Use `storage_backend_failure` as the fallback for unreadable/unavailable stores when no more precise signal exists.

Preserve Phase 76 / Phase 71 resource-pressure precedence: disk/resource-pressure cases remain `resource_exhaustion` / `FreeDisk`, not generic backend failure.

### D-10: Prefer Top-Level Status Evidence
Prefer a top-level field such as:

```rust
recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>
```

on `OpenBitcoinStatusSnapshot`, because store-open or stopped-node failures can happen before sync state exists.

### D-11: Preserve Compatibility Summaries
Keep existing `sync.recovery_category` and `sync.recovery_action` as compatibility summaries.

Richer evidence should include category, action class, evidence basis, affected namespace/path when relevant, unavailable reason, and next action.

### D-12: Reuse Across Surfaces
The same recovery evidence fields should feed:
- CLI status
- dashboard status
- stopped-node status
- support evidence
- soak checkpoint/report summaries
- live-smoke summaries if they already include recovery summaries
- operator docs

Missing evidence should remain explicit `Unavailable: reason`, not omitted silently.

### D-13: Preserve Soak Outcome Semantics
Soak schema/corruption/lock/backend categories continue to map to `recovery_stop`.

Resource exhaustion remains `resource_stop`.

Do not redefine soak outcome classes in this phase.

### D-14: Rust Tests Are Primary
Use Rust unit/integration tests as the primary verification path for classifier and storage/status behavior.

### D-15: Use Temp Datadirs and Fjall Fixtures Where Practical
Prefer deterministic temp datadir fixtures and Fjall-backed tests where practical, especially for:
- schema mismatch
- corruption marker
- partial write / recovery marker
- backend open failure mapping

### D-16: Lock Contention May Use a Helper/Subprocess
For lock contention, an in-process fixture is preferred if reliable.

If Fjall or OS lock behavior makes that unreliable, use a small test helper/subprocess that holds the store open while the test probes or opens the same datadir.

The test must remain deterministic and not require public network access.

### D-17: Test-Only Seam Allowed for Platform-Sensitive Failures
A small test-only seam is allowed if a storage-open failure is too platform-sensitive to induce portably.

The seam must still exercise the real classifier/status mapping.

### D-18: Bun Checkers Are Supplementary
Bun scripts may supplement validation for docs/artifacts/status schema consistency.

They should not replace Rust tests for core recovery classification.

### the agent's Discretion

The agent may decide:

1. Exact enum/type names for recovery evidence and action classes.
2. Whether the classifier lives under `status`, `sync`, `storage`, or a new small recovery module.
3. Exact JSON shape of `RecoveryEvidenceSnapshot`, as long as labels are stable and readable.
4. Whether to add a dedicated status helper module or extend existing status/recovery files.
5. Whether lock evidence is collected by CLI status code, node status code, storage adapter helper, or a shared probe helper.
6. Whether deterministic lock contention uses an in-process open-store fixture or a subprocess helper.
7. Exact UAT command examples, provided they are repo-local Cargo/Bazel commands and not alias-only.
8. Whether docs updates live in runtime guide, status snapshot docs, storage decision docs, or all three.

### Deferred Ideas (OUT OF SCOPE)

1. Automatic repair, automatic reindex, automatic store deletion, automatic lock cleanup, or automatic datadir relocation.
2. Owner heartbeat/PID sentinel protocol as a required runtime contract.
3. Public-network validation, long soak requirements, service-manager mutation, or CI-only large-disk scenarios.
4. Changing existing stable recovery labels.
5. Rewriting the Fjall storage adapter beyond what is needed for recovery evidence and safe probes.
6. Wallet-specific salvage or key recovery flows.
</user_constraints>

## Summary

Phase 77 should introduce a top-level typed `recovery_evidence` status contract, a single pure classifier, and a probe-only lock evidence path that never calls the normal mutating store-open path for operator inspection. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] The key planning risk is that `FjallNodeStore::open` currently creates or recovers databases, opens or creates keyspaces, and writes a schema record when missing, so status/support code that opens the store for inspection can mutate an otherwise untouched datadir. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]

The implementation should keep `sync.recovery_category` and `sync.recovery_action` as compatibility summaries while adding richer evidence at the snapshot top level, because stopped-node and failed-store-open cases can occur before usable sync state exists. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] The classifier should consume typed `StorageError` values, recovery markers, non-mutating lock probe results, and existing service/RPC/datadir evidence, then emit stable recovery category labels plus REC-07 action classes. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs]

**Primary recommendation:** Use the existing Rust/Fjall/status stack, add no new dependencies, and plan Phase 77 around a pure classifier plus read-only evidence collectors that feed every operator surface from the same `RecoveryEvidenceSnapshot`. [VERIFIED: packages/Cargo.lock] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REC-05 | Detect lock contention, stale lock evidence, and concurrent datadir use without hidden source datadir mutation. [VERIFIED: .planning/REQUIREMENTS.md] | Use a probe-only lock evidence path and combine lock results with same-datadir service/RPC evidence; do not use `FjallNodeStore::open` in status/support probes. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] [VERIFIED: packages/open-bitcoin-cli/src/operator/status/service_status.rs] |
| REC-06 | Detect corruption markers, schema mismatches, partial writes, and unreadable runtime stores with typed recovery categories. [VERIFIED: .planning/REQUIREMENTS.md] | Existing `StorageError`, `RecoveryMarker`, schema validation, and snapshot decode paths already model these inputs and should feed one classifier. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/snapshot_codec.rs] |
| REC-07 | Generate recovery evidence separating safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate guidance. [VERIFIED: .planning/REQUIREMENTS.md] | Add REC-07 action classes beside the existing `StorageRecoveryAction` compatibility text; renderers should consume the classifier output. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| REC-08 | Provide deterministic tests for lock contention, stale lock, corruption marker, schema mismatch, partial write, and storage-open failure. [VERIFIED: .planning/REQUIREMENTS.md] | Existing Fjall tests already cover schema mismatch, corrupted JSON, recovery markers, and clean shutdown clearing; add classifier/probe/status tests and a subprocess or test seam only where OS lock behavior requires it. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/tests.rs] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Use root `AGENTS.md`, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages before planning or implementation. [VERIFIED: AGENTS.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md] [VERIFIED: rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: AGENTS.md]
- Keep migration/recovery behavior dry-run-first and backup-aware; do not imply source datadir, service, config, or wallet mutation without an explicit future plan. [VERIFIED: .planning/CONVENTIONS.md]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`. [VERIFIED: AGENTS.md]
- Keep pure domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.bright-builds.md] [VERIFIED: standards/core/architecture.md]
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add the required parity breadcrumb block through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`; use `none` only when no defensible Knots source anchor exists. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts and keep Bash for thin orchestration wrappers and simple shell checks. [VERIFIED: AGENTS.md]
- Apply Arrange/Act/Assert comments in non-trivial unit tests and test one behavior per test. [VERIFIED: standards/core/testing.md]
- Prefer early returns, typed domain models, and semantic names; optional Rust/TypeScript values should use the `maybe_` naming convention. [VERIFIED: standards/core/code-shape.md] [VERIFIED: standards/languages/rust.md]

## Standard Stack

### Core

| Library / Component | Version | Purpose | Why Standard |
|---|---:|---|---|
| Rust / Cargo | `1.94.1` | First-party implementation and tests. | The repo pins this toolchain in `rust-toolchain.toml` and Cargo metadata uses the 2024 edition. [VERIFIED: rust-toolchain.toml] [VERIFIED: packages/Cargo.toml] |
| Fjall | `3.1.4` | Durable key-value storage backend for node state. | The existing `open-bitcoin-node` storage adapter is Fjall-backed, and `packages/Cargo.lock` pins `fjall 3.1.4`. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| serde / serde_json | `1.0.228` / `1.0.149` | Stable snapshot, status, marker, and report serialization. | Existing storage snapshots, runtime metadata, status DTOs, and CLI reports already use serde-derived shapes. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-node/src/snapshot_codec.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| `FieldAvailability<T>` | repo-local | Explicit available/unavailable status fields. | Existing status snapshots use `FieldAvailability` for resource bounds and other availability-sensitive data; Phase 77 should reuse it for `recovery_evidence`. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| `SyncRecoveryCategory` | repo-local | Stable recovery category labels across status, support, soak, and docs. | Existing labels include `incompatible_schema`, `store_corruption`, `storage_lock_contention`, `storage_backend_failure`, and `resource_exhaustion`; Phase 77 must preserve them. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| Bun | `1.3.9` | Repo-owned TypeScript automation and supplementary schema/doc checkers. | Use only for supplementary artifact checks; Rust tests remain primary for classifier/storage behavior. [VERIFIED: .bun-version] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Bazelisk / Bazel | `1.28.1` / `8.6.0` | Top-level smoke build through Bzlmod. | `scripts/verify.sh` invokes Bazel smoke verification for first-party targets. [VERIFIED: bazel version] [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | `0.8.5` | Coverage verification used by repo verification. | `scripts/verify.sh` expects coverage tooling for the repo-native contract. [VERIFIED: cargo-llvm-cov --version] [VERIFIED: scripts/verify.sh] |
| fs4 | `1.1.0` | Existing filesystem stat/resource helper dependency in CLI. | Use only through existing resource-bound paths; do not add it as the lock evidence primitive unless existing code already requires that boundary. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Probe-only lock evidence using standard filesystem APIs and existing service/RPC signals | `lsof`, `/proc`, platform-specific process scans | Non-portable process scans conflict with the locked decision that concurrent-use detection must not depend on them as the only proof. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Existing Fjall adapter plus typed `StorageError` mapping | Raw Fjall/journal/keyspace parser | Fjall exposes no public read-only database-open path in the pinned source, and hand-parsing storage internals would be brittle and outside Phase 77. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/builder.rs] [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/db.rs] |
| Add typed evidence beside stable categories | Rename or expand existing stable category labels | Existing labels are consumed across status/support/soak/docs and are explicitly locked for compatibility. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |

**Installation:**

No new package installation is recommended for Phase 77. [VERIFIED: packages/Cargo.lock] If the plan adds a new dependency, it should justify why existing Rust standard library APIs, Fjall typed errors, and repo-local status types are insufficient. [VERIFIED: AGENTS.md]

**Version verification:**

| Component | Verified Version | Verification Method |
|---|---:|---|
| Rust | `1.94.1` | `rustc --version`; `rust-toolchain.toml`. [VERIFIED: rustc --version] [VERIFIED: rust-toolchain.toml] |
| Cargo | `1.94.1` | `cargo --version`. [VERIFIED: cargo --version] |
| Fjall | `3.1.4` | `packages/Cargo.lock`; local registry source. [VERIFIED: packages/Cargo.lock] |
| Bun | `1.3.9` | `bun --version`; `.bun-version`. [VERIFIED: bun --version] [VERIFIED: .bun-version] |
| Bazel | `8.6.0` | `bazel version`. [VERIFIED: bazel version] |

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
├── storage.rs                  # Existing StorageError, StorageRecoveryAction, RecoveryMarker inputs. [VERIFIED: packages/open-bitcoin-node/src/storage.rs]
├── storage/fjall_store.rs      # Adapter boundary; improve typed Fjall error mapping here. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]
├── status.rs                   # Add top-level recovery_evidence field to OpenBitcoinStatusSnapshot. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
├── status/recovery.rs          # Existing stable SyncRecoveryCategory labels; likely home for evidence DTOs or re-export. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]
└── recovery.rs                 # Candidate new pure classifier module if status/recovery.rs would grow too large. [VERIFIED: standards/core/code-shape.md]

packages/open-bitcoin-cli/src/operator/
├── status.rs                   # Collect shared status; avoid normal store open for probe-only evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
├── status/service_status.rs    # Combine same-datadir service evidence with lock/concurrent-use classification. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/service_status.rs]
├── status/render.rs            # Render classifier output, not renderer-local string matching. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs]
├── dashboard/model.rs          # Consume shared status recovery evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]
├── support/evidence.rs         # Include the same evidence in support reports. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs]
└── soak/outcome.rs             # Preserve recovery_stop/resource_stop mapping. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs]
```

### Pattern 1: Pure Recovery Classifier

**What:** Centralize storage errors, recovery markers, probe-only lock evidence, service/RPC/datadir evidence, and unavailable reasons into one pure function that emits `RecoveryEvidenceSnapshot`. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**When to use:** Use for status snapshots, stopped-node status, dashboard models, support evidence, soak summaries, and compatibility `sync.recovery_category` / `sync.recovery_action` summaries. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs]  
**Why:** Existing recovery mapping is spread across `StorageError::recovery_category`, sync error string matching, durable runtime status projection, service restart status, support evidence, and soak outcome code. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status/service_status.rs]

### Pattern 2: Probe-Only Lock Evidence

**What:** Inspect filesystem facts and advisory lock state without creating, deleting, opening the full database, creating keyspaces, clearing markers, or writing schema records. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**When to use:** Use in operator status/support/recovery inspection before any normal `FjallNodeStore::open` would run. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]  
**Important detail:** Fjall uses a database-level `lock` file, attempts advisory locking, and does not remove the lock file on normal unlock, so lock-file existence alone is not stale-lock proof. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/file.rs] [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]

### Pattern 3: Typed Adapter Error Mapping

**What:** Improve `FjallNodeStore` backend mapping to use typed `fjall::Error` variants where possible before falling back to string details. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/error.rs]  
**When to use:** Use only at real storage adapter boundaries such as daemon startup, runtime store writes, and explicitly requested operations that may open the store. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]  
**Why:** Pinned Fjall exposes a typed `Error::Locked` variant, while the current `StorageError::recovery_category` lock path also falls back to scanning words such as `lock`, `locked`, and `contention` in backend messages. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/error.rs] [VERIFIED: packages/open-bitcoin-node/src/storage.rs]

### Pattern 4: Compatibility Summaries Derived From Evidence

**What:** Keep `sync.recovery_category` and `sync.recovery_action` as compatibility summaries and derive them from the richer classifier output when possible. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**When to use:** Use when updating `OpenBitcoinStatusSnapshot`, CLI renderers, support summaries, live-smoke output, and soak reports. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs]  
**Why:** Existing docs and surfaces already refer to stable recovery categories, so Phase 77 should add evidence without breaking consumers. [VERIFIED: docs/architecture/status-snapshot.md] [VERIFIED: docs/operator/runtime-guide.md]

### Anti-Patterns to Avoid

- **Normal store open for operator inspection:** `FjallNodeStore::open` can create databases/keyspaces and write missing schema records, so do not use it for probe-only status/support evidence. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]
- **Lock-file existence equals stale lock:** Fjall leaves the lock file after unlock, so classify stale-lock evidence only when other evidence supports it. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]
- **Renderer-local string matching:** Rendering code should display classifier output instead of duplicating category detection from error strings. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]
- **Swallowing store-open failures as `None`:** Current `.ok()?` patterns in status/support collectors can lose REC-06 evidence and should become typed unavailable/evidence results where relevant. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]
- **Resource precedence regression:** Low-disk/resource cases must remain `resource_exhaustion`/`FreeDisk`, not generic backend failures. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/storage.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Fjall database inspection | Raw LSM/journal/keyspace parser | Existing Fjall adapter errors plus explicit `Unavailable` when read-only inspection cannot safely read deeper records | Pinned Fjall does not expose a read-only database open in the reviewed source, and normal open can mutate stores. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/db.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| Lock ownership detection | Portable process detector or `lsof` dependency | Advisory lock probe plus same-datadir service/RPC evidence | The phase explicitly rejects non-portable process scans as the only proof of concurrent use. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Recovery mapping | Separate string mappers per renderer/report | One pure classifier and shared DTOs | Existing duplicate mapping already appears in storage, sync recovery, service status, support, and soak code. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs] |
| Repair behavior | Automatic delete, repair, reindex, compaction, or lock cleanup | Typed guidance action classes and explicit backup-before-rebuild text | Automatic destructive recovery is out of scope and conflicts with the no-hidden-mutation requirement. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| New recovery taxonomy | Renamed category labels | Stable `SyncRecoveryCategory` labels plus richer evidence | Stable labels are locked and already used across status, support, soak, and docs. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] [VERIFIED: docs/architecture/status-snapshot.md] |
| Secrets or wallet salvage | Raw wallet/key inspection in support evidence | Redacted typed evidence with affected namespace/path only | Wallet-specific salvage and key recovery are out of scope for this phase. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |

**Key insight:** Phase 77 is classification and evidence hardening, not storage repair; plans should make unsafe or mutating paths explicit and unavailable rather than trying to infer or fix Fjall internals from status code. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] [VERIFIED: docs/architecture/storage-decision.md]

## Common Pitfalls

### Pitfall 1: Probe Code Accidentally Mutates the Store

**What goes wrong:** A status/support command calls `FjallNodeStore::open` to inspect runtime metadata and silently creates or updates database state. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]  
**Why it happens:** `FjallNodeStore::open` delegates to Fjall database open, opens keyspaces, and calls `ensure_schema`, which writes the current schema when missing. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]  
**How to avoid:** Add a probe-only path for lock/evidence collection and report `FieldAvailability::Unavailable` when deeper metadata cannot be read without normal store open. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Warning signs:** Status code uses `.open(data_dir).ok()?`, `load_runtime_metadata`, or `load_metrics_status` from an operator inspection path without classifying open failures. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs]

### Pitfall 2: Misclassifying Persistent Lock Files

**What goes wrong:** Any existing `lock` file is reported as stale or active lock evidence. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/file.rs]  
**Why it happens:** Fjall keeps the `lock` file after unlocking and uses advisory locking on that file to determine active contention. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]  
**How to avoid:** Treat the lock path as one evidence item, then distinguish active contention from stale-lock evidence using an advisory probe result plus existing service/RPC/datadir signals. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Warning signs:** Tests assert stale lock based only on file existence. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]

### Pitfall 3: Duplicated String Matching Drifts

**What goes wrong:** CLI, support, soak, and sync code disagree about the same storage failure. [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status/service_status.rs]  
**Why it happens:** Current mapping includes typed `StorageError` mapping and separate string-detail mapping paths. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]  
**How to avoid:** Move decision logic into one pure classifier and let renderers display the result. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Warning signs:** New code checks substrings such as `schema`, `corrupt`, `lock`, or `disk` outside the classifier. [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]

### Pitfall 4: Backend Failures Lose Specificity

**What goes wrong:** Fjall lock, version, unrecoverable, or journal failures all collapse into generic `storage_backend_failure`. [VERIFIED: packages/open-bitcoin-node/src/storage.rs]  
**Why it happens:** The existing backend failure path stringifies `fjall::Error` and relies on action/message fallback. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]  
**How to avoid:** Match typed `fjall::Error` variants at the adapter boundary and then feed the classifier; keep generic backend failure only as fallback. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/error.rs]  
**Warning signs:** Adapter tests only assert formatted messages, not resulting categories/action classes. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/tests.rs]

### Pitfall 5: Recovery Evidence Is Hidden Under Sync Only

**What goes wrong:** Store-open and stopped-node failures cannot surface because no sync state exists yet. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Why it happens:** `OpenBitcoinStatusSnapshot` currently has `sync.recovery_category` and `sync.recovery_action`, but no top-level `recovery_evidence` field. [VERIFIED: packages/open-bitcoin-node/src/status.rs]  
**How to avoid:** Add top-level `FieldAvailability<RecoveryEvidenceSnapshot>` and derive compatibility summaries from it where possible. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Warning signs:** Stopped status reports “unavailable” without structured cause/action evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]

### Pitfall 6: Tests Depend on Unstable Runtime State

**What goes wrong:** Lock/recovery tests pass on one OS or timing setup and fail elsewhere. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Why it happens:** Advisory locks and file permissions can be platform-sensitive, while default verification must remain deterministic and public-network-free. [VERIFIED: .planning/STATE.md]  
**How to avoid:** Use pure classifier tests for mapping, temp datadir Fjall fixtures for real storage cases, and a subprocess/helper or test seam only where platform behavior makes in-process locking unreliable. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/tests.rs] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]  
**Warning signs:** Tests require service managers, public network, large disk, long sleeps, or real operator datadirs. [VERIFIED: .planning/REQUIREMENTS.md]

## Code Examples

Verified patterns from codebase and pinned dependency sources:

### Pure Classifier Shape

```rust
// Source: existing status/recovery/storage contracts.
// [VERIFIED: packages/open-bitcoin-node/src/status.rs]
// [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]
// [VERIFIED: packages/open-bitcoin-node/src/storage.rs]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    SafeRetry,
    ReadOnlyInspection,
    BackupThenRebuild,
    StopAndEscalate,
}

pub fn classify_recovery(
    input: RecoveryClassifierInput<'_>,
) -> FieldAvailability<RecoveryEvidenceSnapshot> {
    let Some(signal) = input.strongest_signal() else {
        return FieldAvailability::Unavailable {
            reason: input.unavailable_reason().to_owned(),
        };
    };

    FieldAvailability::Available(RecoveryEvidenceSnapshot::from_signal(signal))
}
```

### Probe-Only Lock Evidence Boundary

```rust
// Source: Fjall uses a database-level "lock" file and advisory try_lock.
// [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/file.rs]
// [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]
pub fn probe_fjall_lock(datadir: &Path) -> LockEvidence {
    let lock_path = datadir.join("lock");

    if !lock_path.exists() {
        return LockEvidence {
            kind: LockEvidenceKind::NoLockArtifact,
            lock_path: lock_path.display().to_string(),
            detail: "no Fjall lock artifact found".to_string(),
        };
    }

    let maybe_file = std::fs::File::open(&lock_path);
    let Ok(file) = maybe_file else {
        return LockEvidence {
            kind: LockEvidenceKind::ProbeUnavailable,
            lock_path: lock_path.display().to_string(),
            detail: "lock probe unavailable: lock file could not be opened".to_string(),
        };
    };

    match file.try_lock() {
        Ok(()) => LockEvidence {
            kind: LockEvidenceKind::StaleLockEvidence,
            lock_path: lock_path.display().to_string(),
            detail: "Fjall lock artifact is present but not currently held".to_string(),
        },
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => LockEvidence {
            kind: LockEvidenceKind::ActiveContention,
            lock_path: lock_path.display().to_string(),
            detail: "Fjall lock is currently held by another opener".to_string(),
        },
        Err(_) => LockEvidence {
            kind: LockEvidenceKind::ProbeUnavailable,
            lock_path: lock_path.display().to_string(),
            detail: "lock probe unavailable: advisory lock failed".to_string(),
        },
    }
}
```

### Status Projection

```rust
// Source: OpenBitcoinStatusSnapshot currently uses FieldAvailability for availability-aware fields.
// [VERIFIED: packages/open-bitcoin-node/src/status.rs]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    pub config: ConfigStatus,
    pub service: ServiceStatus,
    pub sync: SyncStatus,
    pub recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>,
    pub resource_bounds: FieldAvailability<ResourceBoundSnapshot>,
    // existing fields omitted
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Recovery state mainly appears as `sync.recovery_category` and `sync.recovery_action`. [VERIFIED: packages/open-bitcoin-node/src/status.rs] | Add top-level `recovery_evidence` and keep sync fields as compatibility summaries. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] | Phase 77 plan target. [VERIFIED: .planning/STATE.md] | Store-open and stopped-node failures can be represented before sync state exists. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Backend lock detection partly relies on message strings. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] | Match typed `fjall::Error::Locked` at adapter boundaries and use message fallback only for unknown variants. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/error.rs] | Phase 77 plan target. [VERIFIED: .planning/STATE.md] | Lock contention evidence becomes less brittle. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/error.rs] |
| Status/support metadata collection can normal-open Fjall stores. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Probe-only inspection reports evidence or explicit unavailable state without hidden mutation. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] | Phase 77 plan target. [VERIFIED: .planning/STATE.md] | Operator diagnostics become safe for unknown or damaged datadirs. [VERIFIED: .planning/REQUIREMENTS.md] |
| Soak already maps storage/schema/corruption/lock/backend categories to `recovery_stop` and resource exhaustion to `resource_stop`. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs] | Preserve soak outcome classes while adding richer evidence summaries. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] | No semantic change required in Phase 77. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] | Existing soak reports keep stable outcome vocabulary. [VERIFIED: docs/architecture/status-snapshot.md] |

**Deprecated/outdated for this phase:**

- Treating `lock` file existence as stale-lock evidence is outdated for Fjall because the lock file persists after unlock. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]
- Opening the full store from status/support paths is unsafe for probe-only inspection because normal open can create/recover records. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]
- Duplicating recovery classification in renderers or report generators is outdated because Phase 77 locks a centralized classifier requirement. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]

## Assumptions Log

All claims in this research were verified from project files, pinned dependency source, local tool output, or cited public documentation. No `[ASSUMED]` claims are used. [VERIFIED: research log]

## Open Questions (RESOLVED)

1. **Exact stale-lock evidence threshold**
   - What we know: Fjall lock-file existence alone is not enough because the file remains after unlock. [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]
   - Resolution accepted for Phase 77: classify `stale_lock_evidence` only when the Fjall lock artifact exists, the probe can acquire the advisory lock, and same-datadir service/live RPC evidence does not indicate a running daemon. This is evidence, not proof of owner death, and the action class is `read_only_inspection` with wording that forbids automatic lock deletion or cleanup. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]
   - Plan impact: Plan 77-01 encodes the classifier precedence and Plan 77-02 implements the probe result as `LockEvidence { kind, lock_path, detail }`; concurrent datadir evidence outranks stale-lock evidence. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-01-PLAN.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-02-PLAN.md]

2. **Whether to expose a dedicated recovery inspection command**
   - What we know: Required consumers include CLI status, dashboard status, stopped-node status, support evidence, soak summaries, live-smoke summaries if applicable, and docs. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]
   - Resolution accepted for Phase 77: do not add a standalone `recovery inspect` command. The shared top-level status contract, support bundle projection, dashboard rows, soak reports, live-smoke summary projection, and docs satisfy the required operator surfaces without expanding the CLI command set. [VERIFIED: .planning/REQUIREMENTS.md]
   - Plan impact: Plans 77-03 through 77-05 project the shared evidence into existing status/support/dashboard/soak/live-smoke surfaces; Plan 77-06 documents the existing status and support commands as the bounded operator workflow. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-03-PLAN.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-04-PLAN.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-05-PLAN.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-06-PLAN.md]

3. **How broad metrics-store recovery evidence should be**
   - What we know: Metrics status currently opens a Fjall store and can fail separately from the main runtime store. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
   - Resolution accepted for Phase 77: status/support inspection must not open Fjall metrics stores for recovery diagnosis. When metrics evidence is not already collected safely, surfaces emit explicit unavailable reasons such as `metrics history unavailable: probe-only status does not open Fjall stores` or `metrics history unavailable: probe-only support bundle does not open Fjall stores`. The shared recovery classifier keeps affected namespace/path fields for future uniform metrics/runtime/wallet evidence without adding metrics-specific action classes now. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]
   - Plan impact: Plans 77-03 and 77-04 remove store-backed status/support metrics inspection from probe-only paths and preserve explicit unavailable evidence. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-03-PLAN.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-04-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---:|---|
| Rust | Implementation and tests | yes | `rustc 1.94.1` | None required. [VERIFIED: rustc --version] |
| Cargo | Workspace tests and metadata | yes | `cargo 1.94.1` | None required. [VERIFIED: cargo --version] |
| Bun | Supplementary repo-owned scripts | yes | `1.3.9` | Rust tests remain primary. [VERIFIED: bun --version] |
| Bazelisk / Bazel | Repo smoke build | yes | `1.28.1` / `8.6.0` | None required. [VERIFIED: bazel version] |
| cargo-llvm-cov | Repo verification | yes | `0.8.5` | `scripts/verify.sh` is the source of truth if coverage tooling requirements change. [VERIFIED: cargo-llvm-cov --version] |
| Git | Source and submodule state | yes | `2.53.0` | None required. [VERIFIED: git --version] |
| Bitcoin Knots submodule | Parity evidence and breadcrumbs | yes | `v29.3.knots20260210` pinned commit present | Run `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status packages/bitcoin-knots] [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found for research and planning. [VERIFIED: command probes]

**Missing dependencies with fallback:** None found for research and planning. [VERIFIED: command probes]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---|---|
| V2 Authentication | no | Phase 77 does not change authentication flows; preserve existing RPC/operator authentication boundaries and do not add credential material to recovery evidence. [VERIFIED: .planning/REQUIREMENTS.md] |
| V3 Session Management | no | Phase 77 does not introduce sessions or browser state. [VERIFIED: .planning/REQUIREMENTS.md] |
| V4 Access Control | yes | Keep operator diagnostics read-only/probe-only unless the user explicitly performs a future mutating recovery plan. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| V5 Input Validation | yes | Parse storage/probe/service inputs into typed enums and DTOs at boundaries; renderers should not classify raw strings. [VERIFIED: standards/core/architecture.md] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| V6 Cryptography | no | Phase 77 does not add cryptographic operations; do not hand-roll wallet/key recovery or crypto. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| V7 Error Handling and Logging | yes | Emit structured, bounded, redacted recovery evidence instead of raw logs or unbounded backend messages. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Hidden status/support mutation of damaged datadirs | Tampering | Use probe-only evidence paths and explicit unavailable states; never repair/delete/clear/open-mutating from inspection. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Concurrent datadir use misreported as safe | Tampering / Denial of Service | Combine advisory lock evidence with same-datadir service status and live RPC availability instead of relying on one weak signal. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/service_status.rs] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Raw backend error disclosure in support evidence | Information Disclosure | Keep typed cause/action/basis fields and redact or bound raw detail strings before support/report rendering. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs] [VERIFIED: standards/core/architecture.md] |
| Ambiguous guidance leads to destructive recovery | Tampering / Repudiation | Separate `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and `stop_and_escalate` action classes. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |
| Category drift between CLI, support, and soak | Repudiation | Use one pure classifier and shared status DTOs; preserve stable category labels. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md` - locked decisions, scope, test expectations, and out-of-scope items. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - REC-05 through REC-08 and out-of-scope constraints. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/STATE.md` - current phase state and deterministic verification constraints. [VERIFIED: .planning/STATE.md]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/` - repo-specific and Bright Builds planning, verification, architecture, and testing rules. [VERIFIED: AGENTS.md]
- `packages/open-bitcoin-node/src/storage.rs` - `StorageError`, `StorageRecoveryAction`, `RecoveryMarker`, and category mapping. [VERIFIED: packages/open-bitcoin-node/src/storage.rs]
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` and `tests.rs` - Fjall adapter behavior, schema writes, recovery marker behavior, and existing storage tests. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]
- `packages/open-bitcoin-node/src/status.rs` and `status/recovery.rs` - shared status shape, `FieldAvailability`, and stable recovery labels. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` and `sync/runtime_state.rs` - current runtime recovery mapping and projection paths. [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]
- `packages/open-bitcoin-cli/src/operator/status*`, `support*`, `dashboard*`, and `soak*` - downstream consumers that need shared evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/{db.rs,builder.rs,error.rs,file.rs,locked_file.rs}` - pinned Fjall behavior for open/create/recover, errors, and locks. [VERIFIED: local cargo registry]
- `docs/architecture/storage-decision.md`, `docs/architecture/status-snapshot.md`, `docs/operator/runtime-guide.md` - operator/documentation contracts and no-hidden-repair guidance. [VERIFIED: docs/architecture/storage-decision.md]

### Secondary (MEDIUM confidence)

- `https://docs.rs/fjall/latest/fjall/struct.Database.html` - public Fjall database API documentation for builder/open and keyspaces. [CITED: https://docs.rs/fjall/latest/fjall/struct.Database.html]
- `https://docs.rs/crate/fjall/latest` - public Fjall crate documentation noting internal synchronization and separate-process loading warning. [CITED: https://docs.rs/crate/fjall/latest]
- `https://owasp.org/www-project-application-security-verification-standard/` and `https://github.com/OWASP/ASVS` - ASVS applicability and current 5.0.0 project context. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: research log]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - verified from `rust-toolchain.toml`, `packages/Cargo.lock`, local command probes, and repo scripts. [VERIFIED: rust-toolchain.toml] [VERIFIED: packages/Cargo.lock]
- Architecture: HIGH - driven by locked Phase 77 decisions and existing code paths; stale-lock threshold remains MEDIUM because the exact product threshold is not fully specified without a heartbeat/PID sentinel. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]
- Pitfalls: HIGH - verified from current code paths, pinned Fjall source, and existing tests/docs. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/locked_file.rs]
- Security domain: MEDIUM - ASVS applicability is verified, but exact redaction field policy should be finalized during planning against the new DTO shape. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]

**Research date:** 2026-06-15  
**Valid until:** 2026-07-15, or sooner if the Fjall dependency, status snapshot contract, or Phase 77 locked decisions change. [VERIFIED: packages/Cargo.lock] [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md]

---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T16:25:00Z
---

# Phase 155: Recoverable Index and Pre-Prune Startup Protection - Research

**Researched:** 2026-10-04
**Domain:** BASIC index integrity, Fjall durability, recovered coins authority and startup prune safety
**Confidence:** HIGH for existing behavior; MEDIUM for proposed implementation until reopen tests pass

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions and scope are copied verbatim. [VERIFIED: 155-CONTEXT.md]

### Locked Decisions

### Storage compatibility and integrity
- **D-01:** Use additive versioned index records in the existing Fjall database, preserving current schema-2 coins and old datadir compatibility. Add no production crate, dependency or storage engine.
- **D-02:** Keep immutable BASIC records keyed by block hash, with block/parent/height identity, filter bytes/hash/header and predecessor commitment. Validate encoding, bounded lengths and commitment/ancestry consistency; identical rewrites may be idempotent, conflicting immutable records must refuse.
- **D-03:** Distinguish immutable records from active height projection and durable resume authority. Preserve valid displaced or ahead records on recovery/refusal; never erase a prefix or fabricate a gap to recover.

### Chainstate fence and fault ordering
- **D-04:** Fence the safe cursor by recovered coins best-block plus compatible durable active-chain metadata and ancestry. Header tip, SyncProgress, status counters and persisted filter height alone are insufficient authority.
- **D-05:** When records precede a successful coins/metadata flush, retain those records but rewind/reconcile active projection and resume cursor to the recovered branch/checkpoint. A missing/inconsistent authority refuses explicitly.
- **D-06:** Persist index records/checkpoint/protection through concrete atomic durable batches where the same-database preconditions permit it. Separate effects must publish valid records/checkpoint before relaxing protection. Failures may retain extra history; they must not create a phantom cursor or release required input.

### Pre-prune startup protection
- **D-07:** After coins recovery establishes the durable authority, validate/reconcile saved index metadata and index-owned protection before `initialize` can call `resume_prune_intent`. Manager construction after initialize is too late.
- **D-08:** Protect the earliest still-required body/undo input conservatively using a reserved internal index identity. Missing, corrupt or unsafe protection/checkpoint combinations refuse before deletion and preserve payloads, live intent and immutable prefix. Retain the existing finish-or-Repair refusal behavior.
- **D-09:** Require the production `DurableSyncRuntime` reopen path to consume this guard. A test-installed lock after runtime construction or an unused helper does not satisfy CFPR-03. Absent index metadata preserves legacy startup behavior.

### Evidence and scope
- **D-10:** Prove successful/interrupted writes, ahead-of-chainstate rows, wrong branch, corrupt record/checkpoint/protection, and unsafe live prune intent using real Fjall close/reopen and production runtime startup. Include record/checkpoint/protection persistence fault seams; memory-only or source-string proof is supplementary.
- **D-11:** Preserve pure recovery/transition decisions outside I/O, with thin store/startup shells and behavior tests using Arrange/Act/Assert. Register all new Rust parity breadcrumbs and run the full native verifier before finalization.

### the agent's Discretion

Exact additive keyspace/envelope types, bounded record limits, explicit fault-injection seams and plan decomposition are researcher/planner choices. Prefer small modules and reuse existing typed BASIC commitments, Fjall batches, coins authority and prune locks. Do not expand into public activation, scheduler or operator product surfaces.

### Deferred Ideas (OUT OF SCOPE)

Phase 156 owns reserved operator CRUD, actual manual/automatic prune coordination and disable/re-enable transitions. Phase 157 owns public option parsing, activation/history preflight and scheduled catch-up. Phase 158 owns validated runtime reorg orchestration. Phases 159–162 own RPC, peers, operator surfaces and integrated retained-history proof. No pending todos matched this phase.
</user-constraints>

<phase-requirements>
## Phase Requirements

Descriptions are verbatim requirements. [VERIFIED: .planning/REQUIREMENTS.md]

| ID | Description | Research Support |
| --- | --- | --- |
| CFIX-02 | An operator can reopen the real Fjall datadir after successful or interrupted index writes and recover valid filter records and safe progress, or receive a fail-closed diagnostic without phantom cursor advancement. | Versioned bounded parser, immutable identity checks, atomic publication and explicit reopen fault matrix |
| CFIX-04 | An operator can restart after index work runs ahead of a coins/chain-metadata flush without trusting a resume cursor beyond the recovered durable chainstate checkpoint; immutable filter records and active progress are reconciled to the recovered branch. | Coins-B plus compatible chain_meta authority; common-ancestor reconciliation; immutable rows independent from projection |
| CFPR-03 | An operator can reopen a datadir with interrupted prune intent without recovery deleting inputs required by the index: index protection is validated before resumed deletion, and unsafe combinations retain protection or refuse with a diagnostic. | Guard inside initialize before resume_prune_intent; direct early-height protection; production runtime reopen |
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Work inside the parent-invoked GSD plan-phase lifecycle. This research owns only this file; parent owns commits and the clean gate. [VERIFIED: AGENTS.md; orchestrator assignment]
- Preserve pinned Knots `29.3.knots20260210`, minimal dependencies, single Fjall chainstate, no Rust Bitcoin production dependency, and functional core / imperative shell boundaries. [VERIFIED: AGENTS.md]
- Use Rust `1.94.1`, edition 2024; Bun for substantial automation and thin Bash wrappers. No package.json bootstrap is needed. [VERIFIED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml]
- Use small `foo.rs` plus `foo/` modules, typed invariant constructors, `maybe_` optional names, early returns, propagated errors, no `unwrap()`, and Arrange/Act/Assert behavior tests. Pure business logic requires near-total coverage. [VERIFIED: supplied global instructions; AGENTS.bright-builds.md; standards/core/{architecture,code-shape,testing}.md; standards/languages/rust.md]
- Read sidecar and relevant local standards; overrides are placeholder-only. Both active lessons total 7,188 bytes and were loaded; no archive was loaded. [VERIFIED: startup byte measurements; AGENTS.bright-builds.md; standards-overrides.md; active lessons]
- Register new Rust source/test parity breadcrumbs through `docs/parity/source-breadcrumbs.json`; document intentional Fjall/storage/protection differences in parity docs. Review relevant README status after substantial changes. [VERIFIED: AGENTS.md]
- Serialize Cargo/Bazel work through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`. Full `bash scripts/verify.sh`, including Bazel smoke, is finalization contract; `--fast` is local iteration only. LOC artifact refresh is required freshness. Poll quiet resumable checks at least every 60 seconds. [VERIFIED: AGENTS.md]
- Before commits run required Rust format/lint/build/tests in order, native verification and managed checks; parent should coordinate hook evidence to avoid unnecessary duplicated heavy runs. [VERIFIED: supplied global instructions; AGENTS.md; standards/core/verification.md]
- Preserve literal `initialize(` in `sync/open_runtime.rs`, and the `Chainstate::from_coins_cache` / `ManagedChainstate::from_chainstate` construction inside `open_with_runtime_activation`; existing Phase 123/135 checkers inspect these anchors. Integrating inside initialize preserves them. [VERIFIED: scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts; scripts/check-phase135-snapshot-recovery.ts]

## Summary

The concrete production startup seam is `DurableSyncRuntime::open_with_runtime_activation` → `initialize` → H/B coins recovery → load prune locks → `resume_prune_intent` → readiness → manager construction. Protection installed by the manager arrives after deletion. Insert one early index-recovery shell into initialize after recovered best-block is known and before prune resume; legacy absence remains the existing path. [VERIFIED: packages/open-bitcoin-node/src/sync/open_runtime.rs; chainstate/flush_lifecycle.rs]

Coins best-block and chain_meta are separate persisted effects: `complete_coins_write` flushes/syncs cache, then persists active-chain metadata. Fjall's batch atomically publishes across same-database keyspaces, but cannot retroactively make these existing effects a single checkpoint. Immutable filter rows therefore describe completed derivations; only a verified branch checkpoint against recovered coins and compatible chain_meta authorizes resume/protection release. [VERIFIED: chainstate/flush_lifecycle.rs::complete_coins_write; storage/fjall_store/coins.rs; installed fjall-3.1.4/src/batch/mod.rs]

**Primary recommendation:** Implement additive versioned BASIC records, a typed pure recovery plan, an atomic checkpoint/protection adapter, and the pre-resume startup consumer; prove each through real reopen. This is a prescribed design under D-01–D-11, not a claim the index already exists. [VERIFIED: 155-CONTEXT.md]

## Standard Stack

Use the existing pinned stack without upgrades or installations. Current registry releases are irrelevant to the locked no-dependency-change scope; versions below are verified project selections, not claims of ecosystem latest versions. [VERIFIED: 155-CONTEXT.md D-01; packages/Cargo.lock]

| Component | Verified version | Purpose | Source |
| --- | --- | --- | --- |
| Rust / Cargo | 1.94.1 / edition 2024 | Existing workspace, pure policy and adapters | [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; rustc/cargo probes] |
| Fjall / lsm-tree | 3.1.4 / 3.1.4 | Same-database batches and durable reopen | [VERIFIED: packages/Cargo.lock; installed dependency source] |
| First-party BASIC types/generator | Workspace 0.1.0 | FilterHash/FilterHeader, SHA256d and historical evidence | [VERIFIED: primitives; consensus/block_filter.rs; chainstate/block_filter.rs] |
| Bun | Repo pin 1.3.9; current PATH 1.4.2 | Verification/checkers/timing shell | [VERIFIED: .bun-version; bun probe] |

Package publication dates were not needed or verified because no package is selected for addition or upgrade. No npm installation step applies. [VERIFIED: D-01; repo has no package.json per AGENTS.md]

## Architecture Patterns

### Recommended decomposition

Proposed paths are planner choices within existing crates; preserve the crate graph. [VERIFIED: D-01; D-11; standards/languages/rust.md]

```text
open-bitcoin-codec/src/block_filter/validation.rs      bounded BASIC encoding validation
open-bitcoin-chainstate/src/filter_index.rs           pure identities, checkpoint/recovery policy
open-bitcoin-chainstate/src/filter_index/              focused recovery/protection tests
open-bitcoin-node/src/storage/filter_index.rs          versioned binary envelope codec
open-bitcoin-node/src/storage/fjall_store/filters.rs    same-db persistence and recovery reads
open-bitcoin-node/src/storage/fjall_store/filters/      batches, startup shell, tests
open-bitcoin-node/src/chainstate/flush_lifecycle.rs      call early guard before prune resume
open-bitcoin-node/src/sync/tests/filter_index_reopen.rs production open tests
```

### 1. Additive schema, independent authorities

Prescribe fixed `basic_filter:v1:` prefixes in the existing BlockIndex keyspace. This avoids expanding every StorageNamespace match or changing global schema 2 while permitting local envelope version rejection. Existing BlockIndex already contains body keys, snapshots, have_pruned, intent and locks; prefixes must not overlap those keys. [VERIFIED: storage.rs; fjall_store.rs; fjall_store/prune.rs; D-01]

| Key / record | Proposed content and invariant |
| --- | --- |
| `basic_filter:v1:record:<64 raw-byte hex>` | Version/type, height, block hash, parent hash, predecessor header, filter hash, filter header, bounded filter length/bytes. Raw hash encoding follows existing block/undo keys, never display-order strings. |
| `basic_filter:v1:active:<8 hex height>` | Version plus `(height, block hash)`; height duplicated in payload detects misplaced keys. Only rows in the checkpoint's visible prefix are active. |
| `basic_filter:v1:state` | Explicit empty-prefix or committed-prefix enum; committed endpoint `(height,hash,header)`, durable chain fence `(height,hash)`, and protection earliest-required height. This metadata means a saved internal index exists, not public option activation. |
| Existing `prune_locks` record | Reserved identity such as `__open_bitcoin_basic_index`; updated in the same database batch with state/projection publication while preserving other names. |

These are prescribed envelopes, not existing storage facts. They implement the distinctions required by D-02/03/06. Empty-prefix is distinct from genesis-complete; unknown versions/types and partial/trailing envelopes refuse with `StorageError::Corruption { namespace: BlockIndex, action: Repair }`. A failure diagnostic does not execute repair. [VERIFIED: D-02/03/06; storage/fjall_store.rs::corruption]

Use checkpoint-bounded projection visibility: rewind atomically changes the visible endpoint. Persisted rows beyond it become unclaimed candidates and must never be returned as active. No full-prefix erase or unbounded suffix-delete batch is needed; later ordered append overwrites projection rows while immutable records remain intact. Existing prefix rows must match recovered ancestry up to the selected endpoint; do not silently invent missing height entries. [VERIFIED: D-03/05; Knots blockfilterindex.cpp::CustomRewind; design recommendation]

### 2. Cheap-first bounded integrity checks

Phase 154 has a bounded encoder but no BASIC byte decoder in `codec/block_filter.rs`; stored bytes need a new validation-only decoder, not regeneration from current coins. Use canonical CompactSize, streaming Golomb-Rice reads, checked delta accumulation, N*M bounds, exact terminal padding and no trailing bytes. Validate empty filter `00` explicitly; reject an empty byte vector. [VERIFIED: codec/block_filter.rs; consensus/block_filter.rs; D-02]

Prescribe the existing codec `MAX_SIZE` ceiling (32 MiB) for filter bytes and count, checked envelope overhead, and bounded batch record/byte budgets. Decoder must validate minimum bit capacity before N iterations, account unary work by available bits, and allocate no vector proportional to N. This reuses Phase 154 limits without claiming a production block-capacity guarantee. [VERIFIED: codec/block_filter.rs::basic_filter_range / checked_output_size; D-02; design recommendation]

Then compute SHA256d(bytes) and header from typed hash/predecessor, check raw key hash and embedded identity, genesis null-parent/zero-predecessor correspondence, and non-genesis `(parent hash, parent height+1, parent header)` equality. Hash self-consistency alone does not prove ancestry or historically correct script provenance. Immutable writes originate from the validated historical-input API; identical full records are idempotent, any differing field at the same block hash refuses before publication. [VERIFIED: consensus/block_filter.rs; chainstate/block_filter.rs; D-02]

Stream startup records rather than collect the filter database into a Vec/HashMap. Read one record and predecessor at a time; return explicit corruption/backend errors. Validate the saved checkpoint prefix contiguously against durable positions. A full integrity scan has bounded additional record memory but linear work; do not advertise bounded total startup time or avoid acknowledging existing chain_meta/undo hydration. [VERIFIED: D-02; fjall_store/coins.rs::load_all_undo_records; design recommendation]

### 3. Coins-B plus chain_meta fence and pure recovery plan

Prepare an in-memory verified authority from the recovered coins best-block and durable chain_meta. Validate each position's stored hash against its header, sequential heights, unique identities and previous-hash links from genesis. `decode_chain_meta` constructs positions but does not itself check sequence continuity or agreement with B. [VERIFIED: storage/snapshot_codec/chain_meta.rs; D-04]

Prescribe strict matching of B to the compatible metadata tip for initial implementation. If B is absent with saved index state, metadata is absent/malformed, or B disagrees with metadata tip, refuse before pruning. The existing finish-or-Repair contract permits refusal; avoid silently taking a numeric minimum across inconsistent chains or modifying coins/meta. Valid immutable ahead rows with old *consistent* B/meta remain recoverable. [VERIFIED: D-04/05/08; prune.rs::ensure_intent_may_finish; v2.4-MILESTONE-AUDIT.md]

The pure function receives validated checkpoint/record identities, recovered ancestry, saved protection and optional intent facts. Return typed `LegacyAbsent`, `Keep`, `Reconcile`, or `Refuse` facts. It must not read Fjall, clocks, runtime counters or sockets. On ahead/wrong-branch checkpoint find the verified common ancestor with recovered active ancestry, limited by recovered durable tip; validate its complete indexed prefix, select that endpoint, and retain every immutable record. No record-only opportunistic cursor advancement. No common prefix or missing predecessor refuses rather than fabricating genesis. [VERIFIED: D-03–05; standards/core/architecture.md; Knots base.cpp::Rewind / ChainStateFlushed; design recommendation]

Protection is derived from safely resumable work, not latest stored-record height: earliest required body is genesis for empty prefix, otherwise safe endpoint + 1; non-genesis undo at the same next height is required. Use checked next-height arithmetic and an explicit finished/max-height state rather than overflow/sentinel ambiguity. A previously stronger lock is acceptable; a missing, malformed or weaker checkpoint/protection combination refuses before attempting automatic lock recreation. A valid saved ahead checkpoint may need *stronger* protection after rewind; commit it before resume. [VERIFIED: D-08; chainstate/block_filter.rs; design recommendation]

### 4. Same-database publication, explicit uncertainty

Fjall 3.1.4 `WriteBatch` supports cross-keyspace atomic commit and `durability(Some(SyncAll))`. Its persist error poisons the database; read/compare then batch commit is not a compare-and-swap transaction. Keep publication under the existing serialized authority, and do not retry a failed commit in the same live instance or treat its result as proven rollback. Reopen and validate persisted state. [VERIFIED: installed fjall-3.1.4/src/batch/mod.rs]

For a bounded append whose chain fence is already verified, atomically insert immutable rows, affected projection rows, state and updated full lock map in one SyncAll batch. If generated records lie beyond verified B/meta, allow durable record-only storage while safe state/protection remains unchanged. If persistence is split, publish rows first; publish valid checkpoint and protection together later. A failed earlier step cannot relax protection. Do not change the existing multi-batch coins H/B protocol to satisfy this phase. [VERIFIED: D-04/06; coins_view.rs; design recommendation]

Prescribe private per-store/test-only fault points immediately before record-only batch, before state/checkpoint batch, before protection publication, and after an actual successful batch to simulate caller uncertainty. Exercise the production adapter against real Fjall; no global environment fault switch or alternate memory store. A pre-commit injected refusal proves no write; a post-commit injected error proves recovery when the caller cannot rely on return status. These are deterministic software fault seams, not hardware torn-write proof. [VERIFIED: D-10; existing raw test seams in fjall_store.rs; design recommendation]

### 5. Guard inside initialize before resumed deletion

Required order: existing coins recovery → recovered B → read/validate/reconcile index state and protection → durably persist safe reconciliation → load resulting effective locks → inspect intent against index's direct earliest-required rule → existing `resume_prune_intent` → readiness. Keep the runtime's existing initialize call and constructor body intact. [VERIFIED: flush_lifecycle.rs::initialize; sync/open_runtime.rs; D-07/09]

If every index key is absent, preserve legacy behavior. State absent with reserved lock or index records/projection present is a partial/corrupt saved index, not legacy absence. Safe already-indexed prune intent may proceed through existing resume rules; intent at/above earliest-required refuses and retains both remaining payload mates, intent, immutable rows and protection. Existing stale-meta/keep-window/operator-lock refusals remain unchanged. [VERIFIED: D-08/09; prune.rs; design recommendation]

**Critical early-height detail:** `height_forbidden_by_lock` excludes heights <=1 when first<=11 and adds `height_last + 10` without saturation. Do not put `u32::MAX` into that existing helper. A bounded internal endpoint can use `u32::MAX - PRUNE_LOCK_BUFFER`, with checked/saturating policy for any new arithmetic; startup also needs the explicit earliest-required intent check to protect genesis/height 1. This phase does not authorize broad public CRUD or ordinary deletion changes. [VERIFIED: chainstate/prune/locks.rs; Knots node/blockstorage.cpp::DoPruneLocksForbidPruning; D-08; deferred scope]

## Don't Hand-Roll

| Problem | Use instead | Evidence |
| --- | --- | --- |
| Hash/header implementations | Existing SHA256d, FilterHash/Header, compute_filter_header | [VERIFIED: consensus/block_filter.rs] |
| Historical spent scripts | BasicFilterInputs plus authoritative undo | [VERIFIED: chainstate/block_filter.rs] |
| Ad hoc database/transaction log | Existing Fjall SyncAll batches | [VERIFIED: D-01; installed Fjall batch source] |
| Second deletion owner | Existing initialize/resume and serialized runtime authority | [VERIFIED: D-07/09; startup source] |
| Repair through erase/rebuild or current coins | Explicit typed refusal, immutable retention | [VERIFIED: D-03/05/08] |

The new bounded BASIC encoding validator is owned protocol work consistent with this project's dependency policy; introducing a Rust Bitcoin decoder dependency would contradict the locked scope. [VERIFIED: AGENTS.md; D-01/02]

## Runtime State Inventory

This phase is an additive storage evolution, not an OS/service rename. Categories are explicit so implementation does not silently assume source edits migrate runtime state. [VERIFIED: D-01]

| Category | Found / action |
| --- | --- |
| Stored data | Schema-1/2 Fjall, coins B/H, chain_meta, block/undo, prune_locks and live prune_intent. Add versioned index rows without global schema bump; preserve old migration behavior. Reconcile saved projection/checkpoint, never rewrite existing coins or immutable filters. [VERIFIED: fjall_store.rs; coins.rs; prune.rs] |
| Live service config | No phase-owned external service/config migration identified in canonical scope; public activation belongs to 157. No external user service was inspected or mutated. [VERIFIED: context deferred scope] |
| OS-registered state | None required by this phase's scope; no service registration change is proposed. [VERIFIED: ROADMAP Phase 155; context] |
| Secrets/env vars | None required by the scoped datadir/index schema; do not add fault-control environment variables or inspect secret values. [VERIFIED: context; design recommendation] |
| Build artifacts | Existing Rust/Bazel outputs rebuild through native verification; no installed binary/package rename or image migration. [VERIFIED: AGENTS.md; context scope] |

## Common Pitfalls

| Pitfall | Concrete prevention / warning sign | Evidence |
| --- | --- | --- |
| Height comparison accepts wrong fork | Compare hash/parent/header and contiguous durable ancestry; equal heights are not equal checkpoints | [VERIFIED: D-04/05; Knots base.cpp] |
| Record existence advances resume | Record-only commits keep checkpoint/protection; ahead rows remain unclaimed | [VERIFIED: D-03–06] |
| Manager installs lock too late | Guard inside initialize before resume; test production open with live intent | [VERIFIED: sync/open_runtime.rs; flush_lifecycle.rs] |
| Corruption becomes missing/empty | Distinguish absence, corruption, backend error; bounded parser rejects malformed empty bytes | [VERIFIED: Knots CustomInit / ReadFilterFromDisk; D-02] |
| Rewind erases valuable data | Change projection visibility/checkpoint only; hash-addressed displaced/ahead rows survive | [VERIFIED: D-03/05; Knots CustomRewind] |
| Lock endpoint overflow / early-height hole | Checked bounds and direct earliest-required intent check; test 0/1 and upper endpoint | [VERIFIED: chainstate/prune/locks.rs] |
| Test leaves hidden Fjall clones live | Drop runtime, view, cache and store handles before reopen; assert actual database open succeeds | [VERIFIED: clone-owned Database/Keyspace handles in fjall_store.rs / open_runtime.rs] |
| Source guards become completion evidence | Static checks supplement actual durable reopen/fault tests | [VERIFIED: D-09/10] |

## Code Examples

This exact existing pattern demonstrates supported same-database durability, not proposed index API names. [VERIFIED: fjall_store/prune.rs::commit_paired_delete_inner; installed Fjall batch source]

```rust
let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
batch.remove(&self.block_index, block_key(block_hash));
batch.remove(&self.chainstate, undo_key(block_hash));
batch.insert(&self.block_index, HAVE_PRUNED_KEY, vec![HAVE_PRUNED_VALUE]);
batch.remove(&self.block_index, PRUNE_INTENT_KEY);
batch.commit()
    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
```

This existing typed commitment helper should be reused after bounded record parsing. [VERIFIED: consensus/block_filter.rs]

```rust
pub fn compute_filter_header(hash: FilterHash, previous: FilterHeader) -> FilterHeader {
    let mut bytes = [0_u8; 64];
    bytes[..32].copy_from_slice(hash.as_bytes());
    bytes[32..].copy_from_slice(previous.as_bytes());
    FilterHeader::from_byte_array(double_sha256(&bytes))
}
```

## Validation and Scoped Plan Decomposition

Nyquist Validation Architecture is intentionally omitted: `workflow.nyquist_validation` is false. The behavior tests below are still required by context and standards. [VERIFIED: .planning/config.json; D-10/11]

| Plan | Ownership | Required evidence |
| --- | --- | --- |
| 155-01 | Pure bounded BASIC validator, index/checkpoint/protection types and recovery policy | Encoding truncation/noncanonical count/padding/range/overflow, genesis and wrong-parent commitments; B/meta mismatch, ahead and fork recovery, empty versus genesis, lock 0/1/max; near-total pure coverage |
| 155-02 | Versioned additive Fjall codec/adapter, immutable conflict rules and durable fault seams | Real close/reopen successful/idempotent/conflicting record batches; malformed envelopes; record/checkpoint/protection failures before and after actual commits; immutable retention |
| 155-03 | initialize pre-resume consumer, conservative reconciliation and actual production reopen tests | Unsafe lagging checkpoint + live intent retains body/non-genesis undo/intent; corrupt/missing/weak protection refuses; safe old indexed intent resumes; absent index remains legacy |
| 155-04 | Cross-boundary production restart matrix, parity/docs and native verifier integration | Validated branch/ahead scenarios on real Fjall, raw corruption fixtures, H/B replay interaction; deterministic guard checker tests; source breadcrumbs/README/parity updates and full verification |

Decomposition is a recommendation under researcher discretion. Plans 02 and 03 depend on the policy; final evidence depends on both. Avoid duplicating runtime fixture infrastructure or broad module cleanup. [VERIFIED: context discretion; standards/core/code-shape.md]

Tests must inspect safe cursor, visible projection, immutable records, durable protection, body presence, non-genesis undo presence and live intent separately before and after real reopen. For protected intent at eligible historical height, prepare enough chain height to pass existing keep-window rules; also isolate genesis/height-1 direct-protection cases. Reuse real validated Phase 154 spend/branch fixtures for commitment/authority evidence and existing initialize prune fixtures for deletion ordering, labeling sparse codec-valid cases honestly. [VERIFIED: chainstate/block_filter/tests/validated.rs; flush_lifecycle/tests/initialize.rs; v2.4 audit limitations; D-10]

Suggested targeted commands follow existing timing wrapper, with final test module names chosen during implementation. [VERIFIED: AGENTS.md; existing cargo workspace test layout]

```bash
bun run scripts/command-timings.ts run --key phase155-core-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-chainstate filter_index
bun run scripts/command-timings.ts run --key phase155-storage-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node filter_index
bun test ./scripts/check-phase155-filter-index.test.ts
bash scripts/verify.sh
```

The first command needs a separate `block_filter` filter for decoder tests if those tests do not include `filter_index` in their names. Commands are planned validation, not passes reported by this research. Full native verifier remains the release/pre-commit contract. [VERIFIED: AGENTS.md; Cargo test filtering]

## State of the Art

The relevant comparison is the pinned baseline and current first-party seams, not an unrequested dependency modernization. Knots separates appended filter data from committed best block, flushes file data before index metadata, retains displaced rows and avoids committing a rewind beyond flushed chainstate. Open Bitcoin should implement those observable integrity/progress properties on existing Fjall batches while documenting its layout and stricter corruption/protection checks. [VERIFIED: Knots index/base.cpp::Commit/Rewind/ChainStateFlushed; blockfilterindex.cpp::CustomCommit/CustomRewind; D-01]

## Security Domain

Security enforcement is not explicitly false, so include applicable controls. This is a local storage/startup phase with no new authentication/session/network endpoint; chapter mappings use ASVS 5.0.0, whose numbering differs from the older V2-authentication template. This is engineering guidance, not a compliance certification. [VERIFIED: .planning/config.json; context scope; CITED: https://owasp.org/projects/asvs]

| ASVS 5.0.0 area | Applies here | Control |
| --- | --- | --- |
| V2 Validation and Business Logic | Yes | Bounded parsing, typed authority, fail-closed transitions |
| V6 Authentication / V7 Session Management | No new surface | Keep this phase internal; existing RPC remains later scope |
| V8 Authorization | Internal ownership boundary | Reserved identity and mandatory pre-delete startup gate |
| V11 Cryptography | Hash commitments | Reuse existing SHA256d; hashes detect corruption, do not authenticate historical provenance |
| V16 Security Logging and Error Handling | Yes | Explicit failure categories, no secret/path/script dumps or swallowed storage error |

Chapter names are verified from the official structured standard; applicability is an inference from the locked local-storage scope. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/docs_en/OWASP_Application_Security_Verification_Standard_5.0.0_en.json; VERIFIED: 155-CONTEXT.md]

| Threat | STRIDE | Standard mitigation |
| --- | --- | --- |
| Corrupt/oversized envelope consumes memory or panics | Denial of service | Bounded bytes/count/work, checked arithmetic, streaming decoder |
| Wrong fork or manipulated progress releases history | Tampering | B/meta/ancestry validation, checkpoint-derived protection, fail closed |
| Persistence error credited as successful progress | Repudiation / tampering | Explicit commit error, conservative protection, real reopen validation |
| Runtime construction bypasses internal protection | Elevation of privilege | Mandatory initialize gate before deletion, direct intent check |

Threat classification and mitigations are design inferences from verified fault/control paths. [VERIFIED: D-02/04/06–10; inspected startup and store source]

## Environment Availability

| Dependency | Available | Version / evidence | Required action |
| --- | --- | --- | --- |
| Rust/Cargo | Yes | 1.94.1, probes match pin | Existing timing wrapper; no overlapping Cargo jobs |
| Fjall source | Yes | Installed 3.1.4 source and lockfile | Read pinned API, no upgrade |
| Knots submodule | Yes | a9aee730466ac67d35a3c03ee24676be5e045878 | Keep pinned baseline |
| Bun | Yes, PATH differs from pin | PATH 1.4.2; pin 1.3.9 | Use repo verifier's pinned environment; direct checker runs must respect contract |
| Bazel/Bazelisk | Executables found | /opt/homebrew/bin; version not probed | Verify through native smoke command |
| External service | None required | Local real-Fjall fixtures only | No credentials or public-network run |

Availability facts are from command probes and scope; no version/minimum claim is inferred for Bazel. No blocking missing dependency was identified. [VERIFIED: environment probes; D-01/10; AGENTS.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |

No factual claims rely solely on training knowledge. Proposed designs are explicitly recommendations grounded in context/source evidence, with MEDIUM confidence until implemented. Exact batching budgets and startup profiling remain execution measurements, not invented targets. [VERIFIED: source inventory; context discretion]

## Resolved Research Questions

1. **RESOLVED — Total startup integrity-scan cost:** full streamed validation bounds extra record memory but scales with retained history. Plan 155-04 Task 1 measures a deterministic larger corpus before any latency claim; correctness is not weakened based on an unmeasured performance guess. [VERIFIED: proposed scan algorithm; PITFALLS.md performance traps]
2. **RESOLVED — Coins/meta disagreement after replay:** strict refusal is sufficient for Phase 155. Supporting reconciliation of an advanced B against stale metadata would need separate validated provenance and persistence semantics; do not invent it here. [VERIFIED: D-05/08; separate coins/meta writes; v2.4 audit]
3. **RESOLVED — Hardware fault claims:** before/after durable-batch injection and process-close/reopen prove software boundaries. Actual hardware/power-loss simulation is excluded from this phase's claims. [VERIFIED: D-10; prescribed fault seam scope]

## Sources

Primary local sources were inspected on 2026-10-04. Official dependency implementation is authoritative for the locked version; Context7 is not exposed in this session. [VERIFIED: tool inventory]

- `155-CONTEXT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/PROJECT.md`, `.planning/STATE.md` — locked scope and owning requirements.
- `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder overrides, `standards/core/{architecture,code-shape,testing,verification,local-guidance}.md`, `standards/languages/rust.md`, both active lessons — project constraints.
- `.planning/research/ARCHITECTURE.md`, `.planning/research/PITFALLS.md`, Phase 154 context and `.planning/milestones/v2.4-MILESTONE-AUDIT.md` — canonical startup, recovery hazards and retained advisories.
- `packages/bitcoin-knots/src/index/base.cpp`, `index/blockfilterindex.cpp`, `node/blockstorage.cpp` at `a9aee730466ac67d35a3c03ee24676be5e045878` — flush fence, immutable displaced lookup, checksums and lock buffer.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `fjall_store/{coins,prune}.rs`, `prune/records.rs`, `snapshot_codec/chain_meta.rs`, `storage/coins_view.rs` — concrete schema, batches, authority and recovery.
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`, `sync/open_runtime.rs`, initialize/storage/sync test modules — actual guard insertion and reopen test seams.
- `packages/open-bitcoin-{codec,consensus,chainstate}/src/block_filter.rs`, `chainstate/prune/locks.rs` — bounded producer, historical inputs, typed commitments and early-height/overflow caveat.
- `/Users/peterryszkiewicz/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/batch/mod.rs` — actual WriteBatch API/poisoning/order.
- [OWASP ASVS project](https://owasp.org/projects/asvs) and [official 5.0.0 structured standard](https://github.com/OWASP/ASVS/blob/master/5.0/docs_en/OWASP_Application_Security_Verification_Standard_5.0.0_en.json) — versioned chapter mappings.

## Metadata

| Area | Confidence | Reason |
| --- | --- | --- |
| Standard stack / existing seams | HIGH | Pinned source/manifests/probes |
| Proposed architecture | MEDIUM | Source-grounded design; real reopen implementation pending |
| Pitfalls | HIGH | Concrete source and durability/prune ordering |
| Security applicability | MEDIUM | Official categories mapped to local phase scope |

**Research date:** 2026-10-04
**Valid until:** Source/toolchain changes; otherwise recheck 2026-11-03. This is a review recommendation, not an external policy. [VERIFIED: pinned-source research scope]

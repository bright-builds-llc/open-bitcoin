---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: 2026-10-18T22:43:39.543Z
---

# Phase 166: Canonical Durable Bootstrap and Genesis BASIC - Research

**Researched:** 2026-10-18
**Domain:** Canonical genesis, pre-effect durable admission, exact-work recovery, optional BASIC
**Confidence:** HIGH for existing behavior; MEDIUM for the proposed concurrency/publication integration until implemented and fault-tested.

<user-constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Canonical genesis and authority

- **D-01:** Reuse the exact pinned genesis header and complete body for all four existing network selectors. A selected network, file shape or marker alone never proves stored identity. Reject mismatched network/root/body before accepting authority or mutating incompatible original storage.
- **D-02:** A fresh successful initialization has height zero, canonical genesis as best header/block and coins best block, internally derived exact genesis proof and no spendable genesis coinbase outputs. Preserve the serialized genesis transaction and its output script for BASIC input; do not treat maturity or script unspendability as the genesis UTXO exclusion mechanism.
- **D-03:** Separate genesis bootstrap/reopen capability from ordinary-history qualification. Keep all missing contextual prerequisites explicit, including Signet challenge validation; genesis support does not authorize ordinary history, candidate connects, reorgs or public-network activation.

### Durable initialization and reopen

- **D-04:** Interrupted initialization either resumes from independently proved complete authority or explicitly refuses. Do not publish partial genesis, coins, header, work or index progress as accepted. Prefer the smallest robust atomic/publication protocol supported by current storage; no automatic repair, import, destructive conversion or legacy reinterpretation.
- **D-05:** Preserve the Phase 164/165 pre-effect compatibility boundary for existing unproved, foreign-network, noncanonical or legacy synthetic-work state. Design a proved compatible reopen route rather than bypassing refusal via raw Fjall open. Refusal diagnostics remain bounded and redacted, with unchanged incompatible original data.
- **D-06:** Checkpoint and actual same-datadir reopen after releasing every storage handle must preserve every bit of exact 256-bit work. Recovered authority binds selected network, complete stored header ancestry, internally recomputed proof/accumulation, active tip and coins best block. Decoded numerical work, metadata, fixture snapshots or cloned live handles cannot confer authority. Test independently expected above-u128 encoding/validation separately from small-work genesis histories.
- **D-07:** Inconsistent header/body/network/work/coins bindings, missing records and interrupted checkpoint states recover only to a proved durable prefix or explicitly refuse; never synthesize accepted progress. Preserve pinned modulo-2^256 arithmetic and existing finite-resource policies.

### BASIC and ordinary operator integration

- **D-08:** Explicit BASIC enablement on fresh canonical storage produces the exact pinned genesis BASIC filter and commitment with zero predecessor. Reuse validated lineage, accepted-state owner and durable publication/fencing; no seeded history or fixture-authority route in the product path.
- **D-09:** Genesis indexing and reopen integrate into existing durable startup and configured operator/daemon routes. Ordinary index continuation uses the same owner once validated history is available; this phase does not bypass Phase 167 to manufacture it. Keep disabled BASIC free of index initialization and keep public networking default-off.
- **D-10:** Test actual durable bytes, all-handle release/reopen, boundary corruption and failure injection. Preserve prior receipts and distinguish source-derived genesis/filter expectations, software fault controls, arithmetic vectors and future actual-daemon differential evidence.


### Agent Discretion

Choose focused typed modules and restricted constructors within existing crates, using the existing Fjall, coins, work and BASIC infrastructure. Add no production dependency, workspace crate or external service. Use pure data transformations for authority checks and thin shell adapters for I/O. Research must resolve pre-effect reopen proof and publication ordering before plans are accepted.

### Deferred Ideas (OUT OF SCOPE)

Phase 167 contextual ordinary-history rules; Phase 168 actual pinned Knots differential runner; Phase 169 fork/recovery qualification; Phase 170 canonical pruning integration. No new networks, implicit download/import/repair, public-default activation or production/service/funds claim. No pending todos matched this phase.
</user-constraints>

The constraints above are copied verbatim from the current context; their provenance is Phase166 context, not a new research assertion. [VERIFIED: 166-CONTEXT.md]

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CBOT-02 | An operator can initialize and reopen an empty durable datadir with canonical genesis as best block, while the genesis coinbase contributes no spendable UTXOs and interrupted initialization never publishes partial authority. | One SyncAll cross-keyspace genesis batch; restricted pure genesis proof; original-versus-scratch inspection; actual all-handle reopen and interruption controls. |
| CBOT-03 | An operator can enable BASIC indexing on a fresh canonical datadir, obtain its genesis filter and zero-predecessor commitment, and continue ordinary indexing without seeded history or implicit public-network activation. | Bootstrap before configured index startup; existing accepted-state owner, zero predecessor and canonical body-derived scripts; preserve candidate-history gate. |
| CWOR-04 | An operator can checkpoint and reopen exact cumulative work without truncation, and recovered work is bound to stored headers, network identity and the recovered coins best block. | Recompute proof from stored actual headers, compare exact records and active/coins endpoints, then issue a store-bound recovery capability; separate above-u128 codec vectors. |

Descriptions are copied from the approved requirements. [VERIFIED: .planning/REQUIREMENTS.md]
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Preserve Knots29.3.knots20260210 at a9aee730466ac67d35a3c03ee24676be5e045878; own the production Bitcoin domain and keep dependencies minimal. No new production crate or dependency is needed. [VERIFIED: AGENTS.md; 166-CONTEXT.md]
- Keep pure validation/authority construction in core crates and filesystem/database/process effects in node/adapters. Use restricted types, early returns, maybe_ optional names, foo.rs plus foo/, thiserror/anyhow where appropriate, and tracing for logging. Never add unwrap. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/core/architecture.md; standards/core/code-shape.md; standards/languages/rust.md]
- Work through the active GSD lifecycle; this research owns only this file and makes no commit. Normal hooks and the wrapper own final Git changes. Do not edit managed standards or checker files to silence findings. [VERIFIED: AGENTS.md; 166-CONTEXT.md Specific Ideas; parent research assignment]
- Before phase completion run bash scripts/verify.sh, including formatting/lint/build/tests/coverage/Bazel and managed checks; fast is iteration only. Ad-hoc Cargo/Bazel runs must use scripts/command-timings.ts, with no overlapping Cargo target jobs. Preserve generated LOC freshness. [VERIFIED: AGENTS.md; standards/core/verification.md]
- Add parity breadcrumbs and manifest entries for new first-party Rust source/tests; record intentional differences in docs/parity; review relevant READMEs. Test behavior with focused Arrange/Act/Assert cases. [VERIFIED: AGENTS.md; standards/core/testing.md]
- Use Bun for substantial repo automation; no package.json/bootstrap install exists. Give copy-pasteable repo-local Cargo/Bazel operator commands. Preserve default-off networking, one active Fjall chainstate, explicit evidence limits and original receipts. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens; no project skill directories were found. The research skill's primary-source/file-output workflow was used within this already-delegated researcher. [VERIFIED: session filesystem probes; /Users/peterryszkiewicz/.agents/skills/research/SKILL.md]

## Summary

Use a new restricted canonical-genesis bootstrap/reopen path, leaving open_ordinary's existing unproved-state refusal behavior and every ordinary-history admission gate intact. Bootstrap all four canonical selectors; expose genesis support separately from candidate-history support and Signet challenge support. The current source has exact genesis facts and full-width numbers, but generic imports deliberately lack live work authority. [VERIFIED: consensus/src/chain_spec.rs; node/src/storage/chain_compatibility.rs; node/src/network/canonical_admission.rs; chainstate/src/engine/work_authority.rs; network/src/header_store.rs]

The smallest robust initialization is a single atomic SyncAll Fjall batch containing all genesis authority rows, followed by optional BASIC startup. Reopen needs a bounded private scratch copy: the pinned Fjall does not expose a read-only open, and recovery can persist/flush before node code runs. A retained application operation lease across every own opener closes the lock handoff gap for supported Open Bitcoin callers; the real Fjall lock protects copying. Independently validate logical records on scratch, then revalidate original logical records before node recovery/migration/index/prune. [VERIFIED: pinned fjall src/builder.rs, db.rs, locked_file.rs, batch/mod.rs; node/src/storage/fjall_store.rs; accepted parent research concurrency contract]

**Primary recommendation:** Build one sealed genesis proof, one atomic bootstrap publication and one leased inspection/open owner; connect those to existing BASIC and operator adapters without enabling descendant acceptance. This is a proposed design based on the inspected seams, not an assertion that the feature already exists. [VERIFIED: 166-CONTEXT.md D-03–D-09; sources above]

Current STATE frontmatter says Phase166 context gathered/research next; body Current Position still describes Phase165 finalization. Use the active Phase166 lifecycle ID and current context, not that stale narrative as permission to rewrite earlier verification. Nyquist is explicitly false, so this document intentionally has no Validation Architecture section. [VERIFIED: .planning/STATE.md; .planning/config.json; phase-op166 output]

## Standard Stack

Keep the existing pins; this is no dependency-upgrade phase. Registry latest versions are not recommendations here, and no npm package is proposed. [VERIFIED: 166-CONTEXT.md Agent Discretion; packages/Cargo.lock]

| Component | Verified version/source | Purpose |
| --- | --- | --- |
| Rust | 1.94.1, Rust2024 | std::fs file locks, typed proofs and existing workspace. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; rustc --version] |
| Fjall | 3.1.10, git aa30dca811399a201e0b9595da93a4582dcb2b57; commit dated2026-09-27 | Existing durable database, atomic cross-keyspace batch, recovery on scratch. [VERIFIED: packages/Cargo.lock; node/Cargo.toml; cached exact git source/log] |
| serde / serde_json | 1.0.228 / 1.0.149 | Existing bounded DTOs and classified work. [VERIFIED: packages/Cargo.lock; storage/snapshot_codec/work.rs] |
| ProofWork / Sha256 | Existing first-party consensus modules | Exact 32-byte numbers, target-derived proof, ephemeral inspection digests. [VERIFIED: consensus/src/chain_proof.rs; consensus/src/crypto.rs] |
| Bun | 1.3.9 pinned | Repo-native oracle/guard/test orchestration. [VERIFIED: .bun-version] |
| Bazel | 8.6.0 pinned | Existing first-party smoke/provenance build. [VERIFIED: .bazelversion; bazel --version] |

**Installation:** none. Use the cached pinned dependencies and pinned Bun executable identified below; do not bootstrap npm, replace Fjall, or add fs-lock dependencies. Existing fs4 is present, but Rust std locking already matches pinned Fjall's locking primitive. [VERIFIED: node/Cargo.toml; fjall/src/locked_file.rs; session Rust lock probe]

## Architecture Patterns

### Proposed focused module ownership

| Owner | Responsibility |
| --- | --- |
| consensus/chain_spec.rs and new chain_spec/bootstrap.rs | Pure canonical genesis proof: exact complete body/header, selected chain, header proof and zero predecessor facts; distinct support class. |
| chainstate/engine plus new canonical_genesis.rs | Consume proved genesis into an empty coins owner or independently validate recovered genesis; no normal transaction application and no unchecked live-origin setter. |
| node/storage/chain_compatibility.rs plus canonical_bootstrap.rs | Existing refusal route plus new narrow product opener; bounded diagnostics and phase-specific compatibility decisions. |
| node/storage/fjall_store.rs plus inspection.rs / open_lease.rs | Central lifetime lease, private scratch reader, atomic publication, original logical recheck, sealed store-bound recovery result. |
| node/sync/open_runtime.rs | Construct canonical-scoped managed runtime only from the bootstrap/recovery capability; run optional BASIC afterward. |
| node/chainstate/fjall_store.rs and filter_index.rs | Preserve genuine work origin; attach BASIC lineage from the proved canonical owner and existing append proof. |
| rpc/bin/open-bitcoind.rs and CLI operator runtime adapters | Route existing durable startup/explicit config through narrow bootstrap opener; retain history/network activation preflights. |

These are proposed small modules within existing crates, chosen from current owner boundaries. Names are illustrative; preserve the responsibilities and restricted constructor contracts. [VERIFIED: current listed modules; 166-CONTEXT.md Agent Discretion]

### Pattern1: sealed genesis proof, never a global height-zero shortcut

Derive genesis position from ChainSpec's exact block, its actual hash and block_proof(header.bits), with median-time metadata derived from that header. Require the selected full body and no non-genesis ancestry. Do not call ordinary contextual transaction connection: current overlay_apply applies the coinbase outputs, whereas Knots ConnectBlock special-cases genesis and sets best block without applying transactions. Keep the exact serialized transaction/output script for the filter. [VERIFIED: consensus/src/chain_spec.rs and genesis.rs; chainstate/src/engine/overlay_apply.rs and apply.rs; Knots validation.cpp:2733]

Introduce a private-field proof whose only constructor validates selected canonical facts. A cross-crate public API may accept this unforgeable core proof and a CoinsCache, but must compare its actual complete coins snapshot, empty heads, genesis best block and absence of all coin rows before setting live origin. Bind selected network/params, current owner/generation and tip. Raw ChainPosition, snapshot imports, exact numeric records or bool flags must never create this proof. [VERIFIED: chainstate/src/engine/work_authority.rs; coins.rs; node/src/storage/coins_view.rs; D-02/D-06]

For recovered ancestry, validate continuous height/parent/hash and exact stored work against cumulative target-derived proof starting at canonical genesis. In Phase166 the accepted product proof must contain exactly the genesis position; reject any descendant until Phase167 provides validated contextual-history authority. Write the pure accumulator generically enough for later qualified ancestry, but do not mint acceptance from a self-consistent arbitrary header list. [VERIFIED: D-03/D-06; network/src/header_store.rs; consensus/src/chain_proof.rs]

### Pattern2: one atomic fresh-genesis authority publication

Prepare and validate all bytes before commit. In one database batch with SyncAll publish: selected canonical identity/version; complete genesis body; matching headers and block-index snapshots; chain_meta with genesis position/exact work; coins B best-block=genesis with no H and no C coin records; appropriate complete validation coverage/genesis accepted identity; current schema. Genesis needs no transaction undo. Keep BASIC rows absent unless explicitly enabled later through its own lifecycle. [VERIFIED: fjall/src/batch/mod.rs; node/storage/coins_codec.rs; snapshot_codec.rs; fjall_store/coins.rs and validation_history.rs; Knots validation.cpp:2733; chainstate/block_filter.rs]

Add a narrowly owned writer rather than chaining save_block/save_header_entries/save_chain_meta/coins writes: public raw sinks invalidate validation coverage or append authority and do not provide atomic compound bootstrap. On any commit uncertainty, publish no runtime capability; release all handles. Next open validates a complete committed state or refuses. An engine-created but authority-empty directory after precommit interruption may explicitly refuse; the requirement does not demand automatic reconstruction. [VERIFIED: fjall_store.rs raw save methods; fjall_store/coins.rs; fjall/src/batch/mod.rs poison behavior; D-04]

Finish bootstrap proof before initialize_configured: that function otherwise recovers coins, changes BASIC ownership and resumes prune intent. If optional index initialization fails after a complete genesis batch, runtime startup fails; subsequent startup must use existing index recovery or refuse, never report a partially published index checkpoint as accepted. [VERIFIED: chainstate/flush_lifecycle.rs:227; fjall_store/filters/startup.rs]

### Pattern3: pre-effect reopen using a supported-app lifetime lease

**Resolved concurrency contract:** every own generic, ordinary, canonical and fixture FjallNodeStore opener must pass through a central retained operation lease before backend construction. Keep it in an Arc-owned field shared by Store clones, released only after all database/keyspace owners are gone. Do not expose a constructor that bypasses the lease. Use an externally located stable per-source lease file, not a newly created entry inside an incompatible source tree. Normalize the selected source consistently and retain existing unsafe-ancestor/symlink refusal. [VERIFIED: parent accepted research contract; FjallNodeStore Clone/ownership in fjall_store.rs; chain_compatibility.rs ancestor inspection]

1. Acquire application lease, then inspect source filesystem shape. Missing/empty targets follow fresh initialization; existing source must have an existing regular Fjall lock file.
2. Open that existing lock without create/truncate and take std::fs::File::try_lock. Busy/unsupported/error refuses with a bounded diagnostic. This prevents a supported live Fjall instance from being copied.
3. Under both leases, inventory and copy regular engine files/directories into a private owned scratch directory. Use fresh independent files, never hardlinks, symlinks or an engine open on original. Record an ephemeral closed inventory/digest of copied relative paths/content; recheck source identity/content before handoff. This inventory is inspection evidence, not durable authority metadata.
4. Open only the scratch database with existing Fjall. Enumerate existing namespace names before opening handles, then decode original copied rows using read-only application helpers. Do not call ensure_schema, recover_validation_history, initialize_configured or default-producing load wrappers that could fill missing proof.
5. Validate all canonical logical authority and configured index/prune preconditions on scratch. Refuse unknown schema, missing identities, legacy work, noncanonical body, extra genesis coins, H markers, inconsistent exact work or any unexpected authority-bearing record. Fully drop every scratch DB/keyspace/snapshot and remove scratch before opening original.
6. While retaining the application lease, release the temporary actual-Fjall inspection lock and open original at the lowest unrecovered node layer. Every supported own competing opener remains excluded. Immediately rerun the same logical validator before node recovery/schema/index/prune effects; compare the recovered result to scratch's proof.
7. Only then construct the store-bound canonical capability and proceed to configured runtime/BASIC startup; keep the application lease for the entire live store lifetime.

This protocol is a proposed implementation based on exact pinned APIs and the accepted supported-app contract. Fjall recovery itself can mutate original engine files after step6; no claim is made that ordinary backend reopen is byte-preserving. Incompatible original inputs are refused from scratch before step6. [VERIFIED: fjall/src/db.rs recover; locked_file.rs; node/fjall_store.rs; accepted parent research contract]

**Exact limit of the guarantee:** all own APIs cooperate and competing Fjall instances honor the engine lock. An arbitrary unsupported direct-Fjall opener could race during lock handoff; arbitrary filesystem writers can ignore every advisory lock. This design does not claim protection against either. Detect source/path/inode/content changes and refuse; do not advertise a hostile-writer guarantee or pretend an after-open check retroactively prevents engine effects. Existing code has no public preacquired-lock handoff or before-recovery callback. [VERIFIED: complete pinned builder.rs/locked_file.rs/lib.rs inspection; session lock probe; accepted parent contract]

**Scratch bounds/security:** use a private directory (0700 on Unix) and private files (0600), checked creation and an owned cleanup guard. Reject symlinks/special files, traversal, file-identity changes, oversized counts/bytes/depth and disk-copy failures. Keep finite configurable-in-code inspection limits and fail closed above them; choose/test the smallest envelope that admits fresh/reopened genesis with enabled BASIC plus existing supported metadata, rather than promising archive-scale copies. Cleanup failures must be visible and block successful admission before original open; do not swallow remove errors, log payloads or retain wallet-containing copies silently. Cleanup only the known owned scratch root. [VERIFIED: AGENTS.md error rules; D-05/D-07; existing physical preservation tests; proposed protocol]

A sibling or stable external lease may persist empty after a failed attempt; never include private data in its name/content and never delete/recreate an active lease inode. Source preservation tests compare the original source tree, including its files/modes/mtimes; reads may alter atime and existing tests intentionally exclude it. [VERIFIED: chain_compatibility/tests/preservation.rs physical_manifest; proposed lease lifetime]

### Pattern4: explicit BASIC after authority, through the existing owner

Use BasicFilterInputs::from_historical(canonical_body, proved_position, None) and FilterHeaderPredecessor::Genesis. Its generator selects output scripts from the complete body; absence of genesis UTXOs does not remove the output script. Knots CustomAppend uses empty undo at height0 and its previous header starts null. Generate independent canonical vectors for every selector because filter keys depend on block hash. [VERIFIED: chainstate/src/block_filter.rs; consensus basic-filter/header implementation; Knots blockfilter.cpp; index/blockfilterindex.cpp:268 and blockfilterindex.h]

Use existing BasicFilterStartupMode and enable/recover lifecycle, then initialize_basic_index_owner and drive_basic_filter_index_turn. Attach maybe_validated_lineage only from canonical recovery capability plus a genuine store append proof. Do not alter raw imports or use attach_fixture_basic_owner_after_flush. Disabled mode must not initialize filter owner/state/checkpoint records; PreserveSaved remains an explicitly separate request. [VERIFIED: sync/open_runtime.rs; chainstate/filter_index.rs; network/runtime_authority/filter_index.rs and catch_up.rs; storage/fjall_store/filters/startup.rs]

### Operator integration and minimal history-guard changes

Keep open_ordinary as the existing broad refusal API; add a clearly named genesis-only opener returning the restricted store/capability result. Existing foreign/legacy/synthetic refusal tests and fixture APIs should remain valid. Daemon open_runtime_store/open_authoritative_network_runtime and the canonical runtime constructor select the new path only for configured durable genesis startup; sync/inbound/relay activation still checks unsupported history before any network worker. Update CLI offline sync/wallet adapters only where canonical genesis observation is needed, and do not make ordinary queries initialize missing storage implicitly. [VERIFIED: rpc/bin/open-bitcoind.rs; node/sync/open_runtime.rs; CLI operator/runtime/support.rs, wallet.rs and wallet/name_resolution.rs; D-03/D-09]

Review every changed guarded function, then renew only its corresponding source-files.ts EDGES / shared SOURCE_CONTRACTS row. Add a small Phase166 current-scope successor for genesis-only claims instead of weakening Phase164/165 refusal assertions or rewriting their receipts. Exact verify.sh registration changes also need the existing164 historical-tooling inverse and165 baseline binding reviewed. Preserve155 startup ordering and157 daemon anchors. No broad digest regeneration, skip list, whole-repository inverse expansion or giant all-file mutation loop. [VERIFIED: scripts/check-phase164-canonical-identity.ts; its source-files.ts/test-modules.ts/historical-tooling.ts; check-phase165-shared-exact-work.ts; parent guard-scout findings]

The old154 case named actual-genesis is synthetic and cannot become this phase's canonical vector by relabeling it. Add separate166 source-bound canonical body/filter/proof expectations; preserve old inputs and original failed/native/history receipts. [VERIFIED: parent guard-scout findings; scripts/basic-filter-oracle.py source-derived recipe; 166-CONTEXT.md D-10]

## Don't Hand-Roll

| Problem | Use | Avoid |
| --- | --- | --- |
| Storage decoding | Existing pinned Fjall on owned scratch | Custom Fjall/WAL/SST reader or assumed read-only open. [VERIFIED: pinned builder/db source] |
| Work arithmetic | Existing ProofWork, block_proof, wrapping_add and bytes_le codecs | Parallel bigint, u128 authority or numeric-to-live promotion. [VERIFIED: consensus/chain_proof.rs; storage/snapshot_codec/work.rs] |
| Filter contents/header | Existing BasicFilterInputs, BASIC codec and explicit predecessor | Hardcoded empty filter, UTXO scan or production-generated oracle expectations. [VERIFIED: chainstate/block_filter.rs; Knots blockfilter.cpp] |
| Index recovery/turns | Existing lifecycle/fencing/accepted-state owner | New bootstrap-only index engine or fixture authority. [VERIFIED: node chainstate/filter_index.rs; filters/startup.rs] |
| Locking | std File locks plus one shared own-opener lease | New dependency, source lock replacement, or an unprotected unlock/open gap among supported APIs. [VERIFIED: pinned Fjall locked_file.rs; Rust1.94.1 probe] |

## Common Pitfalls

- **Blessing imports:** load_header_store uses from_entries, which imports unproved metadata. from_coins_cache is untrusted. Recompute and seal explicitly; a decoded exact record is still numerical data. [VERIFIED: node/fjall_store.rs; network/header_store.rs; chainstate/engine.rs and work_authority.rs]
- **Wrong genesis coins:** normal add_transaction_outputs would add the P2PK output. Canonical bootstrap must skip all genesis transaction application; test the exact outpoint absent before and after reopen. [VERIFIED: engine/apply.rs; Knots validation.cpp:2733]
- **Empty-filter assumption:** the genesis output script belongs in BASIC although it has no spendable coin. Its previous filter header is zero. [VERIFIED: chainstate/block_filter.rs; Knots blockfilter.cpp and blockfilterindex.cpp]
- **Partial publication:** separate raw sinks invalidate provenance and can leave B/meta/header/index mismatches. Use one authority batch; faults before/after uncertain commit return no capability. [VERIFIED: fjall_store.rs; fjall_store/coins.rs; batch/mod.rs]
- **False reopen:** a cloned store/network/snapshot holds Fjall handles and cannot prove recovery. Drop runtime, all network handles, Store clones and snapshots before same-path reopen. [VERIFIED: FjallNodeStore Clone; ManagedNetworkHandle; D-06]
- **History leakage:** simply changing require_ordinary_history to Ok would permit unsupported descendant paths. Bootstrap capability and missing candidate_history/Signet challenge facts remain separate. [VERIFIED: consensus/chain_spec.rs; network/canonical_admission.rs]
- **Scratch defaults hiding missing rows:** generic open creates keyspaces and application open writes coverage/schema. Enumerate actual source namespaces/records and validate copied bytes without default-producing wrappers. [VERIFIED: Fjall db.rs keyspace/create paths; fjall_store.rs; validation_history.rs]
- **Unbounded confidentiality footprint:** scratch can copy wallet/runtime metadata. Bound disk work, use private permissions and surface cleanup errors. [VERIFIED: FjallNodeStore namespaces; AGENTS.md failure rules; proposed inspection contract]
- **Historical-checker churn:** a changed digest is a prompt for source review, not evidence that copying the new digest proves behavior. Keep semantic mutation regressions and full native acceptance. [VERIFIED:164/165 checker diagnostics and source contracts]

## Code Examples

Existing pinned Fjall API, not a new storage implementation: [VERIFIED: pinned fjall/src/batch/mod.rs]
~~~rust
let mut batch = db.batch().durability(Some(fjall::PersistMode::SyncAll));
batch.insert(&headers, "snapshot", encoded_headers);
batch.insert(&block_index, "snapshot", encoded_index);
batch.insert(&chainstate, "chain_meta", encoded_chain_meta);
batch.insert(&coins, encoded_best_block_key, encoded_genesis_hash);
// Add the exact body, schema and canonical/validation identity in this same batch.
batch.commit()?;
~~~

Existing first-party filter API; use only after proving canonical position/body: [VERIFIED: chainstate/src/block_filter.rs; consensus filter predecessor type]
~~~rust
let inputs = BasicFilterInputs::from_historical(&genesis, &position, None)?;
let (filter, filter_header) = inputs.generate(FilterHeaderPredecessor::Genesis)?;
~~~

Existing std lock primitive used by the pinned engine: [VERIFIED: pinned fjall/src/locked_file.rs; actual Rust1.94.1 local executable probe]
~~~rust
let source_lock = std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .open(source.join("lock"))?; // No create or truncate on source.
source_lock.try_lock()?;
// Hold while copying and validating scratch; application lease outlives this guard.
~~~

These examples omit application-specific error mapping and capability assembly; they are API patterns, not complete ready-to-paste production functions. [VERIFIED: inspected API signatures; proposed design above]

## State of the Art

| Existing boundary | Phase166 successor | Retained limit |
| --- | --- | --- |
| Filesystem-only unconditional ordinary refusal | Separate bounded proved canonical-genesis opener | Unknown/foreign/legacy states still refuse. [VERIFIED: chain_compatibility.rs; D-05] |
| IdentityOnly plus missing bootstrap/work/context facts | Truthful genesis bootstrap/work evidence class | candidate_history and Signet challenge still unavailable. [VERIFIED: chain_spec.rs; D-03] |
| Full-width ExactNumber storage | Recomputed, store-bound recovered genesis authority | Arbitrary numeric imports and synthetic fixtures remain untrusted. [VERIFIED: snapshot_codec/work.rs; work_authority.rs] |
| BASIC requires seeded validated history | Fresh canonical genesis supplies proved starting authority | Descendant validation remains Phase167. [VERIFIED: filters/startup.rs; roadmap166/167] |

No package deprecation or dependency replacement is proposed. [VERIFIED: locked existing-stack scope in166-CONTEXT.md]

## Implementation Order and Verification

1. Implement the pure canonical genesis/work/coins proof and independent canonical vectors first; test wrong body/header/network/work/coins and separate wide-number vectors. [VERIFIED: D-01/D-02/D-06]
2. Centralize the lifetime operation lease for every own opener; add bounded scratch copy/probe and physical-preservation/concurrency tests before allowing any existing-state admission. [VERIFIED: accepted supported-app research contract; D-05]
3. Add one atomic fresh-genesis writer and restricted recovered constructor; fault every publication boundary, include commit-uncertain failure and actual all-handle reopen. [VERIFIED: D-04/D-07/D-10]
4. Wire optional BASIC through existing recovered lineage/lifecycle/owner; compare independently expected filter bytes/hash/header and enabled/disabled/reopen behavior for all four selectors. [VERIFIED: D-08/D-09]
5. Integrate configured daemon/operator paths, then minimal reviewed guard/claim/registration successors; prove descendant header/body/connect/reorg and network activation remain refused. [VERIFIED: D-03/D-09; existing adapters and guards]
6. Review diff and parity breadcrumbs/README changes, run semantic guard regressions and full default native verification with pinned tools. Preserve original receipts; actual daemon differential is not required or claimed here. [VERIFIED: AGENTS.md; D-10; roadmap168]

Suggested focused Cargo command: bun run scripts/command-timings.ts run --key phase166-node-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase166. Add equivalent consensus/chainstate tests and a focused Bun166 guard test with explicit ./ path. The full gate remains bash scripts/verify.sh; there is no new Nyquist matrix because nyquist_validation=false. [VERIFIED: AGENTS.md; config.json; existing workspace conventions]

Operator UAT examples must use cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ... and bazel run //packages/open-bitcoin-cli:open_bitcoin -- ... with the actual implemented command/options; do not invent a bootstrap subcommand or present an installed alias as the primary route. [VERIFIED: AGENTS.md; active repo lessons]

## Environment Availability

| Dependency | Available | Version | Required action/fallback |
| --- | --- | --- | --- |
| Rust/Cargo | Yes | 1.94.1 | Matches pin. [VERIFIED: session commands; rust-toolchain.toml] |
| Fjall source | Yes | aa30dca /3.1.10 | Cached exact git checkout, not cached registry3.1.4. [VERIFIED: Cargo.lock; source filesystem] |
| Bun on current PATH | Yes, wrong version | 1.4.2 | Prepend pinned executable directory below. [VERIFIED: bun --version; .bun-version] |
| Cached pinned Bun | Yes | 1.3.9 | /Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin. [VERIFIED: direct executable --version] |
| Bazel/Bazelisk | Yes | Bazel8.6.0 | Matches .bazelversion. [VERIFIED: bazel --version] |
| Knots source | Yes | a9aee730466ac67d35a3c03ee24676be5e045878 | Source-derived oracle only; no daemon required by166. [VERIFIED: submodule rev-parse; roadmap166/168] |

Prepend the pinned Bun bin directory to PATH for every Bun/Cargo automation/native verifier and Git-hook invocation; children inherit it. No global install change is needed. Missing required dependencies with no fallback: none identified. No external service is introduced. [VERIFIED: direct version probes; parent pinned-runtime finding;166 scope]

## Security Domain

Security enforcement is enabled by default because config does not explicitly disable it. ASVS is used only as a scoped control vocabulary; this headless storage phase does not claim web-app certification. Use version-qualified ASVS5.0 categories, not the older template's V2-auth/V3-session numbering. [VERIFIED: .planning/config.json; CITED: https://github.com/OWASP/ASVS/tree/v5.0.0/5.0]

| ASVS5.0 category | Applies | Scoped control |
| --- | --- | --- |
| V2 Validation and Business Logic | Yes | Parse bounded copied records into sealed canonical facts. |
| V5 File Handling | Yes | Trusted scratch paths, no source symlinks/special files, bounded copy/cleanup. |
| V6 Authentication / V7 Session Management | No new surface | Preserve existing configured RPC controls. |
| V8 Authorization | Yes, internal capability | Private recovered lineage; raw numbers cannot publish authority. |
| V11 Cryptography | Existing hashing | Reuse first-party work/hash code; no new cryptosystem. |
| V14 Data Protection | Yes | Private scratch and redacted errors; no wallet payload logging. |
| V16 Security Logging and Error Handling | Yes | Fixed categories, visible cleanup failures and no invented progress. |

Category names are from ASVS5.0; applicability and controls are research recommendations derived from inspected storage/RPC boundaries. [CITED: https://github.com/OWASP/ASVS/tree/v5.0.0/5.0; VERIFIED: node/storage.rs; chain_compatibility.rs; rpc/bin/open-bitcoind.rs]

| Threat | STRIDE | Mitigation to prove |
| --- | --- | --- |
| Forged canonical marker/exact scores | Spoofing/Tampering | Full body/header/coins/network/work recomputation before capability. |
| Concurrent own opener or retained live handle | Tampering | Central lifetime lease plus actual Fjall lock during inspection. |
| Copy exhaustion/path alias | Denial of service/Tampering | Bounded inventory/bytes/depth, checked identity, no link following. |
| Scratch wallet exposure or diagnostic leak | Information disclosure | Private owned copy, explicit cleanup and bounded redaction. |
| Partial batch/index state becomes accepted | Elevation/Tampering | Single genesis batch, capability after success, conservative index recovery/refusal. |

Threats and mitigations are phase-specific design recommendations grounded in the identified ingress/effect boundaries. [VERIFIED: D-04–D-10; inspected Fjall/store/coins/index modules]

## Assumptions Log

No unverified factual claims are used as locked decisions. Proposed APIs, module names and copy envelope are explicitly recommendations; their behavior must be established by implementation tests. The supported-app concurrency scope was explicitly accepted by the parent during research and excludes unsupported direct-Fjall/filesystem writers. [VERIFIED: parent research coordination;166-CONTEXT.md Agent Discretion]

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| None | No claims tagged ASSUMED | Entire document | All factual claims cite current source or probes. |

## Open Questions (RESOLVED)

- **Inspection envelope values — RESOLVED by plan:** Plan 166-04 selects initial limits of 4096 entries, 512 MiB aggregate bytes and depth 16, with bounded path lengths/arithmetic and positive fresh/reopened genesis+BASIC sizing tests. Any adjustment requires observed sizing evidence; larger datadirs refuse without archive-scale promises or repair. The policy is resolved; implementation and test results remain pending. [VERIFIED: 166-04-PLAN.md Task 1 action; D-07; Agent Discretion]
- **Exact adapter entrypoints — RESOLVED by plan:** Plan 166-09 routes distinct configured durable genesis startup through daemon open_runtime_store/open_authoritative_network_runtime and restricts CLI execute_offline_sync_command to existing-only canonical observation. No new command or implicit missing/empty-datadir status/query bootstrap is added; sync/inbound/relay history preflights remain. Actual invocation and process tests are planned, not claimed passed. [VERIFIED: 166-09-PLAN.md integration context and Tasks 1–2; D-03/D-09]
- **Future ordinary ancestry — RESOLVED by locked scope:** Phase 167 must supply contextual validation provenance before descendants can be admitted; Phase 169 owns broader fork/recovery qualification. Plans 166-04/09 do not expand that boundary or grant history authority from genesis. No user choice remains open. [VERIFIED: 166-04-PLAN.md; 166-09-PLAN.md; roadmap 167/169; locked D-03/D-06]

## Sources

### Primary (HIGH confidence)

- Local exact Knots pin a9aee730466ac67d35a3c03ee24676be5e045878: kernel/chainparams.cpp; validation.cpp:2733 and5395; chain.cpp; arith_uint256.h; blockfilter.cpp; index/blockfilterindex.cpp and .h. [VERIFIED: submodule source/rev-parse]
- Exact Fjall3.1.10 aa30dca811399a201e0b9595da93a4582dcb2b57: cached src/builder.rs, db.rs, locked_file.rs, snapshot.rs, batch/mod.rs, journal/recovery.rs, lib.rs; upstream source https://github.com/fjall-rs/fjall/tree/aa30dca811399a201e0b9595da93a4582dcb2b57. [VERIFIED: Cargo.lock; cached pinned source and git log]
- First-party consensus/chain_spec/genesis and chain_proof; chainstate engine/work_authority/overlay_apply/apply/block_filter; network/header_store; node store/coins/validation-history/BASIC/runtime/adapters referenced inline. [VERIFIED: local source reads]
- Phase166 context, approved requirements/roadmap, STATE/config, Phases164/165 context and verification; original history remains attributable. [VERIFIED: local artifact reads]
- Rust1.94.1 isolated lock probe: first file locks; separate open returns WouldBlock; clone locks successfully; separate file locks after unlock. This is software evidence on this host, not a universal filesystem guarantee. [VERIFIED: research session executable exit0]
- [OWASP ASVS5.0.0](https://github.com/OWASP/ASVS/tree/v5.0.0/5.0): category vocabulary only. [CITED: same URL]

### Secondary (MEDIUM confidence)

- Parent accepted supported-app lock contract and focused guard-scout findings; these informed the proposed integration, not a claim that production implementation has passed. [VERIFIED: research coordination messages]

### Tertiary (LOW confidence)

- None used. [VERIFIED: source inventory above]

## Metadata

**Confidence breakdown:** standard stack HIGH (pins/source/version commands); current architecture HIGH (actual source); proposed lease/scratch/publication integration MEDIUM (API primitives verified, integration/fault behavior still to build); pitfalls HIGH (pin and current owner behavior). [VERIFIED: cited sources and session probes]

**Research date:** 2026-10-18. **Valid until:** pinned source or phase decisions change; recheck environment before execution. No production code, historical receipt, staging or Git commit was changed by this researcher. [VERIFIED: task scope; file-only research output]

## Bounded amendment: process-isolated scratch recovery | 2026-10-19T23:48:00Z

The earlier Pattern3 step4 is superseded narrowly: raw scratch Fjall recovery/read must execute in a child process, while source locks, source recheck and owned-root cleanup remain in the parent. The logical proof, original post-admission revalidation, arithmetic/genesis/BASIC protocols and later phase ownership are unchanged. This amendment is based on an actual implementation failure rather than an alternative dependency choice. [VERIFIED: current inspection.rs/reader.rs;166-04/15 revised plans;166-CONTEXT.md D-04/D-05/D-07/D-10]

The controlled receipt `.local/open-bitcoin-dev/phase166/plan04/malformed-engine-child-red-final.log` records zero passed/one failed test: both nonnumeric and u64-overflow engine directory names terminate the raw recovery child with SIGABRT6, retain scratch and preserve source. Pinned Fjall recovery.rs:47 expects a valid integer; DatabaseInner::drop at db.rs:82 subsequently expects an unpoisoned lock, so a destructor panic during unwinding aborts. Parent catch_unwind cannot contain this outcome. Earlier16focused/504storage green suites did not exercise it. The non-UTF8 fixture could not be created on this APFS host (OS92), which is setup failure, not a runtime result; Linux-supporting filesystems may test it separately. Existing exact-owned-root receipts permitted parent cleanup after confirmed child completion. [VERIFIED: named final RED and original malformed-engine-red.log; executor04 evidence report; exact pinned db.rs Drop source]

Use the same current executable, with one exact libtest-compatible reserved argv: `--exact __open_bitcoin_storage_inspection_worker --nocapture --test-threads=1`. Product mains dispatch before parser/config/logging/runtime; daemon main must be plain synchronous, with its existing Tokio-decorated async application helper called only after dispatch declines. Cargo libtest harnesses require one explicitly registered root test with this exact name, invoking the same node dispatcher only for this exact argv. A private result file avoids libtest's stdout prelude; child stdout/stderr are null. No sibling availability, CARGO_BIN_EXE in library tests, environment-selected executable, PATH lookup or mutable global registration is assumed. [VERIFIED: current CLI/daemon mains/Cargo manifests/Bazel targets; bootstrap_scout independent harness inventory;166-15 planned contract]

Required existing roots are node/src/lib.rs, CLI/src/lib.rs, CLI/src/bin/open-bitcoin.rs and RPC/src/bin/open-bitcoind.rs. Existing integration harnesses either use generic/raw fixture stores or spawn product binaries; they do not directly enter the new canonical inspection API. A future direct canonical-opener integration test must register the root macro in its own owned harness. The baseline RPC-client CLI main has test=false and no local canonical opener. Product binary Bazel targets need direct existing node-library dependencies, already present transitively but not declared as direct rust_binary deps. [VERIFIED: current package manifests/BUILD files and harness roots; bootstrap_scout inventory]

The parent retains its original application lease and existing Fjall engine lock. Child receives only the versioned16KiB-capped scratch job/nonce, checks canonical fixed-/tmp random private root identity,0700/0600 layout and its own scratch operation lease, and opens only scratch/database. Use a bounded length-prefixed detached-record protocol, preserving exact names/keys/values/absence; this is not a Fjall file parser. Reuse4096logical units/512MiB payload, checked result framing cap `MAX_BYTES + MAX_ENTRIES*24 +64`, and a centralized30-second child deadline. The deadline must include stdin transfer and all child waiting; avoid blocking unbounded pipe reads/writes. Parent validates result type/identity/private mode/link count/cap, exact framing/version/nonce and complete output AND exit0. Child-only panic hook immediately exits fixed nonzero without formatting payload or unwinding; every successful raw engine scope drops all database/keyspace owners first. Pinned DatabaseInner::drop stops/joins workers before clearing engine handles; failure in any such teardown cannot yield accepted success. No parent hook is changed. [VERIFIED: pinned db.rs Drop; existing logical Budget and copy constants;166-15 executable recommendation, values to be measured by tests]

On timeout/output overflow/spawn/protocol/nonzero/signal failure, kill/reap any live child, visibly preserve child-lifecycle failures, recheck source and apply checked recovered-tree privacy/caps before checked exact-root removal. Do not remove scratch while child completion is uncertain. Failures in any stage block proof callback and original admission; never fall back to parent raw recovery. Root ownership is recorded before any backend work; no scan or cleanup of other directories. Original-source raw logical read after admitted backend open remains immediately validated in166-06. Supported-app/advisory-lock scope remains; arbitrary filesystem/direct-Fjall writers are outside it. Parent/system crash, forced termination, OOM and OS crash-dump handling are outside the live-parent cleanup guarantee. [VERIFIED: current ScratchGuard/source inventory/open lease;166-15 planned failure contract; accepted supported-app scope]

04 remains unfinished at amendment time. Its honest foundation-only acceptance requires no parent raw-recovery/admission path; full original inspection goal is preserved in mandatory15, inserted04→15→05 with subsequent waves shifted. Existing behavior cases, genuine RED receipts, physical preservation and actual fresh/reopened canonical+BASIC sizing stay intact; macro no-op/no-job tests are not storage proof.15 verifies valid/corrupt/protocol/lifecycle/concurrency paths and fresh Cargo/Bazel dispatch availability;09 owns actual canonical product reopen,11/12 own only reviewed changed current source anchors/contracts,14 owns full native and independent final acceptance. No original receipt or historical pin is renewed in this amendment. [VERIFIED: revised04/05/09/11/12/15 plans and decision coverage]


## Bounded amendment: sealed genesis RPC context | 2026-10-20

The actual 166-09 product startup test first refused ordinary bootstrap, then reached the proved durable runtime but refused at ManagedRpcContext::from_runtime_config_with_network_handle in context/network.rs. Receipts are .local/open-bitcoin-dev/phase166/plan09/daemon-red-valid-config.log and daemon-green.log; the earlier daemon-red.log is configuration setup failure. The existing ordinary context constructor deliberately requires ordinary history before wallet/mempool recovery. It must remain unchanged. [VERIFIED: actual daemon receipts reported by executor09; current context/network.rs and canonical_chain.rs]

A DurableSyncRuntime reference or immutable selected-chain field alone cannot substitute for a sealed capability: fixture factories and fixture-to-canonical scope binding exist. The resolved adapter seam is a specialized context factory consuming OpenedCanonicalGenesis by value and internally constructing the existing canonical runtime, returning the context and runtime together. It derives its own actual handle/store, accepts no separately supplied raw store/work/DTO/fixture owner, and retains selected-config/history activation checks before effects. This avoids a new core proof API or ordinary-constructor bypass. [VERIFIED: current node sync factory/handle APIs;06/07 capability contracts; executor09 source inspection]

There is no existing disabled-wallet state; Local permits wallet mutation. The genesis context must therefore use an explicit unavailable wallet state with fixed refusal behavior, perform no wallet registry or mempool snapshot recovery, and preserve every existing ordinary wallet gate. Exhaustive state matches in wallet_state.rs, canonical_chain.rs, network.rs and rescan.rs require narrow successors. No wallet permission, transient fixture authority, network worker, dependency, schema or new RPC endpoint is authorized. [VERIFIED: current context wallet/network/rescan sources;09 inspection]

Mandatory166-16 is planned after08 and before09 completion for the context seam and behavioral guards. Its new RPC-library tests directly reopen canonical storage, so that root must explicitly register the existing same-executable inspection worker macro; helper no-job success is distinct from real reopen proof.09 remains paused with its actual partial adapter implementation and failed receipts preserved; it resumes only after checked16 execution/acceptance. Future10–14 shift one wave;15 process isolation remains mandatory. Original decisions and all four roadmap outcomes remain intact; final native/code/security/goal/normal-hook Git gates are unchanged. [VERIFIED: amended workflow ownership; existing worker registration contract]

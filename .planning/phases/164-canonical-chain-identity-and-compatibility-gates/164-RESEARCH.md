---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: 2026-10-14T02:12:33Z
---

# Phase 164: Canonical Chain Identity and Compatibility Gates - Research

**Researched:** 2026-10-13 local / 2026-10-14 UTC. **Domain:** canonical chain identity, nonmutating compatibility refusal, validation capability gates. **Confidence:** HIGH for inspected constants and mutation paths; MEDIUM for recommended integration. [VERIFIED: session date; sources below]

<user-constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Canonical root identity

- **D-01:** Only exact pinned genesis header/body for mainnet, testnet3, signet and regtest; reject any other null-parent root.
- **D-02:** Use one pure existing-network specification with inspectable genesis and support facts reused by adapters.

### Compatibility refusal

- **D-03:** Inspect before schema/coins migration, initialization, validation-history recovery, prune resume and BASIC writes across ordinary daemon/operator paths.
- **D-04:** Fail closed with a bounded typed diagnostic when root/network proof is absent, contradictory or noncanonical; preserve prior durable records and provide a seam for Phase 165 work classification.
- **D-05:** Describe the reason and safe separate-datadir/review steps; no automatic repair, conversion or destructive reindex.

### Validation support and evidence

- **D-06:** Identity selection is separate from validation support. Classify all four existing modes and refuse unsupported ordinary history before acceptance, especially Signet without challenge validation.
- **D-07:** Only implemented and verified capabilities; missing canonical bootstrap, exact work and candidate-history prerequisites remain explicit gates until Phases 165–167.
- **D-08:** Keep synthetic/source-derived fixture evidence explicitly separate from ordinary canonical runtime acceptance; do not rewrite historical receipts or silently bypass production gates.

### Agent Discretion

Choose small typed modules, exact diagnostics, storage inspection mechanics and focused tests after research; preserve first-party domain ownership, one active Fjall chainstate, existing dependency set and default-off public networking.

### Deferred Ideas (OUT OF SCOPE)

Phase 165 owns exact work and legacy synthetic-work policy. Phase 166 owns canonical durable bootstrap, exact-work reopen and genesis BASIC. Phase 167 owns candidate-history consensus rules. Actual differential qualification belongs to Phase 168. No new networks, automatic repair, public-default activation or production-readiness claim.

[VERIFIED: 164-CONTEXT.md; copied decision, discretion and deferred text verbatim]
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CBOT-01 | An operator's selected existing network binds the accepted root to pinned canonical genesis header/body identity; a different null-parent root is rejected. | Exact four-network vectors; network-bound root checks before header/body admission. |
| CBOT-04 | An operator opening a foreign-network or noncanonical-root datadir receives a bounded diagnostic before automatic coins/schema migration, initialization, prune resume or index mutation; prior durable data remains intact. | Filesystem-only fail-closed inspection, complete opener inventory, byte-preservation tests. |
| CVLD-05 | An operator can identify the exact supported validation/evidence scope of existing mainnet, testnet3, signet and regtest selectors; any mode lacking required consensus checks, including Signet challenge validation, refuses before accepting unsupported blocks rather than using another network's defaults. | Pure support matrix, missing-prerequisite refusals, ordinary-path gates and explicit fixture separation. |

[VERIFIED: .planning/REQUIREMENTS.md; descriptions copied verbatim]
</phase-requirements>

## Summary

Use one pure chain specification in an existing core crate and one filesystem-only compatibility gate in the node shell. The first answers which root is canonical; the second refuses any existing store whose identity cannot be proved before opening Fjall. Identity recognition must not imply ordinary body acceptance: all four modes still lack milestone prerequisites, and Signet additionally lacks its challenge validation. Keep those gates explicit until their later owners deliver evidence. [VERIFIED: 164-CONTEXT.md D-01–08; ROADMAP.md phases 164–167; sync/types.rs; sync/open_runtime.rs]

The actual backend is **Fjall 3.1.10 at git revision aa30dca811399a201e0b9595da93a4582dcb2b57**, not registry 3.1.4. `Database::recover` persists the active journal; keyspace recovery can remove directories. A database open followed by reads, even without Open Bitcoin schema migration, cannot satisfy unchanged source bytes. The smallest defensible Phase 164 policy is to refuse all nonempty/unproved datadirs without opening their backend, reporting observed identity as unavailable. This explicitly postpones positive canonical reopen to Phase 166. [VERIFIED: packages/Cargo.lock; open-bitcoin-node/Cargo.toml; pinned Fjall src/db.rs:555–578 and src/recovery.rs:58,70; ROADMAP.md scope gates]

**Primary recommendation:** Implement canonical identity predicates, bounded non-opening refusal, ordinary acceptance gates and truthful operator diagnostics; do not implement a second database reader, sidecar-based authorization, bootstrap, work conversion or historical validation in this phase. [VERIFIED: CONTEXT.md phase boundary and deferred ownership; recommendation derived from inspected backend effects]

## Project Constraints (from AGENTS.md)

- Loaded AGENTS.md, Bright Builds sidecar, placeholder-only overrides, standards index and architecture/verification/testing/code-shape/Rust pages. No project skills directories exist; parent loaded the complete 7,188-byte active lessons set. [VERIFIED: local reads/probes; parent handoff]
- Preserve pure core/shell separation, first-party Bitcoin ownership, one active Fjall chainstate, pinned Knots source and default-off networking. Add no production dependency/crate. Add parity breadcrumbs and intentional-difference documentation for changed Rust behavior. [VERIFIED: AGENTS.md; ROADMAP.md]
- Use typed invariants, early returns, `maybe_` option names, `foo.rs` plus `foo/`, no unwrap, focused Arrange/Act/Assert tests. Keep existing historical receipts and evidence directories unchanged. [VERIFIED: supplied global rules; sidecar; relevant standards; D-08]
- Use Bun automation and timing-wrapped Cargo/Bazel jobs without overlapping target directories; default completion contract is `bash scripts/verify.sh`, including pure-core coverage and Bazel. Keep generated LOC current; poll long verification at least every 60 seconds. UAT uses repo-local Cargo/Bazel commands. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Research is inside parent GSD lifecycle; write only this file and make no commit before parent final verification. Use frontmatter delimiters only at file top. [VERIFIED: parent assignment; AGENTS.md; supplied global instructions]

## Standard Stack

| Component | Exact project version | Purpose and recommendation |
| --- | --- | --- |
| Rust / Cargo | 1.94.1, edition 2024 | Existing core/shell implementation; retain pin. [VERIFIED: rust-toolchain.toml; Cargo.toml; local version probes] |
| Fjall | 3.1.10, git aa30dca811399a201e0b9595da93a4582dcb2b57 | Existing single durable database; do not open unproved sources. Registry 3.1.4 code is not the resolved dependency. [VERIFIED: Cargo.lock; node manifest; local Cargo git checkout] |
| Existing codec/consensus/primitives | Workspace versions | `Block`, `BlockHeader`, typed hashes, `encode_block_header`, `encode_block`, `parse_block`, `block_hash`, `transaction_txid`; use for identity checks. [VERIFIED: codec/block.rs; consensus/crypto.rs; primitives/hash.rs] |
| Bun | Repo pin 1.3.9 | Existing test/check automation. PATH currently resolves 1.4.2; cached 1.3.9 executable is present. Use verified pin for final checks. [VERIFIED: .bun-version; `bun --version`; local cache inventory] |
| Bazel | 8.6.0 | Existing first-party smoke build; no new target family. [VERIFIED: .bazelversion; `bazel --version`; AGENTS.md] |
| serde / serde_json | Existing manifest and lock | Typed diagnostic projection where needed; do not introduce a new serialization dependency. [VERIFIED: node Cargo.toml] |

**Installation:** none. This is a pinned integration phase; latest-registry upgrades and npm version checks are inapplicable because no package is recommended for addition. [VERIFIED: CONTEXT.md discretion; absence of package.json; inspected manifests]

## Exact Canonical Genesis and Test Vectors

All four constructors use version 1, a zero previous hash, one identical transaction, 50 BTC (5,000,000,000 satoshis), and display merkle root/txid `4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b`. Their wire merkle bytes are reversed: `3ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a`. [VERIFIED: pinned kernel/chainparams.cpp:40–77,137–140,295–298,529–532,633–636; independent Python struct/hashlib computation in this research]

| Existing selector | Canonical name | Time | Nonce | Bits | Display genesis hash |
| --- | --- | --- | --- | --- | --- |
| Mainnet | mainnet | 1231006505 | 2083236893 | 0x1d00ffff | `000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f` |
| Testnet | testnet3 | 1296688602 | 414098458 | 0x1d00ffff | `000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943` |
| Signet | default signet | 1598918400 | 52613770 | 0x1e0377ae | `00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6` |
| Regtest | default regtest | 1296688602 | 2 | 0x207fffff | `0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206` |

[VERIFIED: pinned chainparams.cpp constructors/assertions; independently reproduced all four hashes from serialized fields]

Canonical transaction bytes (204 bytes); append `01` plus these bytes to the 80-byte selected header to obtain the exact 285-byte genesis block. The coinbase script is 77 bytes; output script is 67 bytes; sequence and previous-output index are `ffffffff`, locktime is zero, with no witness. [VERIFIED: pinned CreateGenesisBlock; computed byte lengths]

```text
01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000
```

Exact header wire bytes, each 80 bytes. These are source-derived vectors, not an executed Knots daemon receipt. [VERIFIED: independent struct.pack little-endian/hashlib computation; pinned hash assertions]

```text
mainnet:
0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c
testnet3:
0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae18
signet:
0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a008f4d5fae77031e8ad22203
regtest:
0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff7f2002000000
```

For a second serialization guard, SHA-256 of the complete block bytes is respectively `5299fac924b5a2fc19a88876a0042c19ac4d11fe69c3f66e47516e26185f9e99`, `8e83a1ce1b5985bd639984e474cb5f01273f6884c6aab920d67c109eb37a276c`, `c8abc5554d8a4e1134b27050f0a6a8fc616196bfdd381fb49f49fdea6a811675`, `a4edaa2bf93d233994bf2d2560809ad89a2e9552c08d6e511116f186a985100f`. This auxiliary digest is not the Bitcoin block hash. [VERIFIED: independent hashlib computation]

Rust `block_hash` stores the raw double-SHA256 result in `BlockHash`; compare raw bytes or typed values consistently, reversing display literals once at the boundary. Do not install display-order hashes directly with `from_byte_array`. Match all header fields and the entire canonical body, not only the null parent, merkle root or shared transaction. Preserve existing operator `testnet` spelling but document it as testnet3. [VERIFIED: consensus/crypto.rs:80; primitives/hash.rs; codec/block.rs; sync/types.rs; recommended guard derived from D-01]

Default Signet challenge is `512103ad5e0edad18cb1f0fc0d28a3d4f1f3e445640337489abb10404f2d1e086be430210359ef5021964fe22d6f8e05b2463c9540ce96883fe3b278760f048f5189f2e6c452ae`. Knots derives magic from serialized challenge hash; genesis is shared across custom Signets. Store a default-profile identity fact, not an implication that genesis alone proves a Signet challenge. Custom Signet and testnet4 remain excluded. [VERIFIED: pinned chainparams.cpp:455–531; CONTEXT.md deferred scope]

## Architecture Patterns

### One pure specification, explicit authority levels

Recommended structure: `open-bitcoin-consensus/src/chain_spec.rs` plus focused child/test files for an enum of the four existing canonical identities, exact genesis construction/bytes and immutable support facts; `open-bitcoin-node/src/storage/chain_compatibility.rs` plus a filesystem adapter for inspection/refusal; focused adapter changes rather than another workspace crate. `SyncNetwork` and RPC/CLI chain selectors map exhaustively into this core enum. Keep network I/O and filesystem logic out of consensus. [VERIFIED: D-02; existing core dependency direction in research/ARCHITECTURE.md; recommendation]

Model identity recognition separately from ordinary authority. Proposed types are `CanonicalChain`, `GenesisIdentity`, `ValidationScope`, `MissingValidationPrerequisite`, `DatadirObservation`, `CompatibilityDecision`, and `CompatibilityRefusal`; these names are recommendations, not existing APIs. A successful exact-root predicate grants identity only. A raw `HeaderStore`, `ChainPosition` or `ValidationCoverage::Complete` must not mint canonical runtime authority. [VERIFIED: HeaderStore networkless interface; ChainPosition::genesis null-parent check; storage validation history; D-06–08; recommendation]

### Complete root and acceptance seams

| Seam | Current behavior / required placement |
| --- | --- |
| `network/header_store.rs`: `insert_header`, `from_entries`, `record_position`, `seed_from_chain` | Generic networkless carriers accept roots/entries. Bind production construction/import to a selected spec; retained fixture mechanisms must remain explicitly classified and never authorize an ordinary runtime. [VERIFIED: inspected file] |
| Node `network/header_sync.rs`: `validate_header_for_sync`, `build_header_validation_context` | First null-parent header gets height zero with default previous header; only a second root is refused. Check exact selected root before contextual processing and any insertion. [VERIFIED: inspected file] |
| Node `network/validation_history.rs`: `handle_validated_headers` | Validation closure inserts headers, then publishes admitted identities; valid prefix can survive rejected suffix. Perform chain scope/root checks inside admission before either in-memory insertion or durable receipt publication. [VERIFIED: inspected file] |
| Node `network/mempool_lifecycle.rs`: local/stored connects and reorg | Local connect uses caller flags/params; stored connect caches duplicate/nonextending/disconnected bodies. Check ordinary scope and root before cache, inventory, chainstate or accepted-lifecycle effects; apply to reorg replacement branches too. [VERIFIED: inspected file] |
| Node `network/runtime_authority.rs`: handle connect entrypoints | Handle dispatch delegates to the shared owner; propagate the same bound scope, do not create a second policy. [VERIFIED: inspected file] |
| Chainstate `engine/overlay_apply.rs` and `types.rs` | Empty state accepts any zero-parent shape; `ChainPosition::genesis` checks only zero parent. Keep generic core fixtures honest, but require spec-bound ordinary construction and guard before overlay output insertion. [VERIFIED: inspected files] |
| Node `sync/open_runtime.rs`, header-store seeding and body reconciliation | Initialization currently precedes header reconstruction and config params. A runtime-only root check is too late; additionally require ordinary support before constructing accepted authority. [VERIFIED: inspected file; sync/block_reconcile.rs] |
| RPC `context/network.rs`: runtime-config construction and local connection | Memory fallback uses default consensus params plus wallet maturity; durable/memory ordinary paths both need bound scope. A memory fallback must not evade a durable refusal or unsupported-network gate. [VERIFIED: inspected file] |

The simplest intermediate behavior is to recognize exact canonical roots as identity facts while refusing ordinary connected history until bootstrap/work/context prerequisites exist. Do not route canonical genesis through generic transaction connection: Knots skips its transactions, whereas existing overlay connection adds outputs. That special bootstrap belongs to Phase 166, so Phase 164 must not advertise it as operational. [VERIFIED: pinned validation.cpp:2733–2739; overlay_apply.rs; ROADMAP.md Phase 166; recommendation]

## Read-Only Compatibility Mechanism and Proof

### Why a normal backend open fails the contract

Observed order: Fjall `Database::builder(...).open()` → version recovery/create → active-journal recovery/persist and keyspace recovery → Open Bitcoin opens/creates nine namespaces → `recover_validation_history()` → `ensure_schema_and_migrate_coins()` → runtime `initialize_configured()` → coins recovery → BASIC configure/recover → prune-intent resume → managed reconstruction → BASIC owner initialization/index turn. [VERIFIED: fjall_store.rs:85–116; pinned Fjall db.rs:376–403,555–578,804–825; recovery.rs; validation_history.rs:174; coins.rs:235; flush_lifecycle.rs:226; sync/open_runtime.rs:71–117]

`recover_validation_history` writes Complete coverage on a genuinely empty validation store; schema setup writes missing version metadata; schema 1 can migrate coins, undo, chain metadata and version. Schema 2 treats coins as authority and does not remigrate a leftover snapshot. Initialization may replay coins recovery, reconcile BASIC and resume paired deletion. These effects must all be after accepted compatibility; putting a check after `open_unrecovered` still permits backend/namespace mutation. [VERIFIED: validation_history.rs:174–233; coins.rs:235–332; flush_lifecycle.rs:246–276]

The inspected pinned Builder/Config has no read-only-open option; `manual_journal_persist`, snapshots, `create_or_recover`, and no-background-thread behavior do not make recovery read-only. Recovery calls journal persist and can remove incomplete/deleted keyspace directories. Do not hand-roll an SST/WAL parser or change dependency version to solve Phase 164. [VERIFIED: pinned Fjall builder.rs, db_config.rs, db.rs, recovery.rs, journal/recovery.rs; recommendation]

### Recommended bounded filesystem-only policy

1. Resolve selected network and immutable validation scope as data before any store open. Use `symlink_metadata`, propagate permission/I/O errors, and check directory shape with at most the first `read_dir` entry. Do not recursively inventory or decode arbitrary files. [Recommendation grounded in VERIFIED: D-03–05; existing Rust std filesystem interfaces and backend effects]
2. Classify `Missing`, `EmptyDirectory`, `ExistingUnproved`, `UnsafePath` (file/symlink/special), or `InspectionIoFailure`. Nonempty includes apparently canonical, foreign, legacy, partial, wallet-only and malformed stores. Refuse with observed identity **unavailable**, not a fabricated foreign-network/hash diagnosis. Unproved root/network is itself the locked refusal condition. [VERIFIED: D-04; recommended conservative policy]
3. Refuse nonempty/unproved state before `Database::builder`, before creating a lock/marker/log, and before schema/history/coins/index/prune methods. Do not create an identity sidecar to authorize it. Missing/empty means no existing identity, not permission to initialize canonical authority: report bootstrap/work/context prerequisites and leave the path unchanged until Phase 166. [VERIFIED: D-03,07; ROADMAP.md phases 165–167; recommended policy]
4. Require the decision through a network-bound ordinary opener used by every daemon/operator durable path. For this phase no accepted existing-store capability is constructible. Keep inspection types extensible with separate root/network facts and work-profile classification for Phase 165; preserve refusal of unproved exact-work state. [VERIFIED: D-04; ROADMAP.md Phase 165; recommendation]
5. Do not introduce an `allow_legacy`, environment override, automatic reset, ordinary config switch or catch-and-retry through raw open. A trusted sidecar alone cannot establish that stored headers, coins, bodies and provenance agree. Future positive reopen needs a design that proves those records before source mutation, rather than weakening byte preservation. [VERIFIED: D-04–08; research/ARCHITECTURE.md compatibility table; recommendation]

This mechanism is genuinely non-opening and therefore cannot run Fjall recovery, namespace creation or Open Bitcoin migration. It trades temporary refusal of all existing unproved sources for preservation and truthful capability scope. It does **not** determine the actual stored genesis, support canonical reopen, or repair an existing datadir. Those limitations must be explicit in docs and tests. [VERIFIED: recommended call order against inspected mutation paths; ROADMAP.md Phase 166 ownership]

Proof should combine (a) injected/open-spy tests showing zero backend-open and migration/recovery/index/prune calls, (b) source-level call-site coverage, and (c) physical before/after tree equality on real closed Fjall fixtures. Compare file names, types, lengths and bytes/digests, including version, WAL, keyspace metadata, lock and unrelated sentinel files; compare permissions/mtime where stable. Ordinary read access-time changes are not database writes and should not be misrepresented as a byte change. Test missing paths remain absent and empty directories remain empty. [Recommendation grounded in VERIFIED: source backend mutation inventory; CBOT-04 preservation contract]

### Ordinary durable opener inventory

| Path | Required treatment |
| --- | --- |
| RPC binary `src/bin/open-bitcoind.rs::open_runtime_store` | Gate before `FjallNodeStore::open`; runtime triggers include sync, inbound, prune and explicit BASIC. Preserve selected network even when public networking is off. [VERIFIED: lines 393–425] |
| Same binary `open_authoritative_network_runtime` | Runtime construction must consume gate/scope outcome, before initialize/configure/index; no fallback accepted memory runtime following refusal. [VERIFIED: lines 300–330; sync/open_runtime.rs] |
| RPC binary `open_bitcoind/inbound_metrics.rs::start_inbound_metrics_worker` | Gate the no-handle fallback open; metrics-only paths can otherwise migrate/create source state. [VERIFIED: line 60] |
| RPC `context/wallet_state.rs::build_wallet_state_with_store` | Change swallowed `let Ok(open)` fallback into a visible compatibility decision; do not quietly swap an incompatible durable wallet/chain source for memory. [VERIFIED: lines 304–335] |
| RPC `context/mempool_recovery.rs::recover_mempool_snapshot_from_store_handle` | Gate no-handle open before recovery; typed compatibility refusal is distinct from benign unavailable mempool history. [VERIFIED: lines 67–90] |
| RPC `context/network.rs::from_runtime_config_with_store` | Central caller must propagate refusal and selected scope through wallet/mempool construction; its current memory-only `expect` assumption needs reconsideration when inspection can fail. [VERIFIED: lines 80–155] |
| CLI `operator/runtime/support.rs::execute_offline_sync_command` | Gate status as well as pause/resume; status currently opens a mutable backend before reading metadata. Pass network from config resolution rather than inferring mainnet. [VERIFIED: lines 26–76] |
| CLI `operator/wallet.rs::execute_backup_command`, `resolve_managed_wallet_name` | Gate both backend opens; compatibility failure must not become a fallback guessed wallet name or write a backup destination. [VERIFIED: lines 190,479] |
| CLI `operator/soak/runtime.rs::enrich_soak_resource_bounds_from_store` | Gate optional metadata enrichment; project typed unavailable reason rather than causing backend effects or hiding identity mismatch. [VERIFIED: lines 168–185] |

Run a final `rg 'FjallNodeStore::open|Database::builder|open_unrecovered'` call-site audit excluding test modules, then classify every result. Generic `FjallChainstateStore::from_store` already receives an opened store; it does not replace the earlier admission boundary. [VERIFIED: repository searches; chainstate store wrapper; recommended audit]

## Validation Support and Evidence Matrix

| Selector | Identity facts deliverable now | Ordinary history acceptance after Phase 164 | Missing prerequisites / honest evidence |
| --- | --- | --- | --- |
| Mainnet | Exact pinned genesis/header/body and profile | Refused until verified prerequisites exist | Canonical durable bootstrap, exact work, history-derived rules; later finite historical fixtures are not whole-mainnet replay. |
| Testnet/testnet3 | Exact pinned testnet3 root; preserve selector spelling | Refused | Same prerequisites plus network/history difficulty, minimum-difficulty recovery and deployments; never borrow mainnet modern defaults. |
| Signet/default | Exact root plus pinned default challenge/profile facts | Refused, with explicit Signet challenge reason | Non-genesis challenge/solution validation absent from inspected first-party path; root/PoW knowledge is insufficient. No custom challenge scope. |
| Regtest/default | Exact pinned root and default-profile identity | Refused until verified prerequisites exist | Bootstrap, exact work, normal maturity 100, subsidy interval 150, candidate-history flags/rules; current runtime uses 210000 and P2SH-only. |

[VERIFIED: sync/types.rs; sync/open_runtime.rs; consensus/context.rs; first-party source search; pinned chainparams.cpp; pinned validation.cpp:4371; pinned signet.cpp:125–150; ROADMAP.md and D-06–08. Matrix is recommended Phase 164 policy, not a claim those later prerequisites are implemented.]

Expose the exact selected mode, canonical genesis, evidence class, ordinary acceptance status and fixed missing-prerequisite codes through existing operator diagnostics/current docs. Do not add arbitrary deployment switches. Gate inbound header/body, local/stored connect, startup/recovery seeding and reorg paths, including memory-backed ordinary construction; default-off networking alone is not consensus enforcement. [VERIFIED: enumerated production seams; D-06–08; recommendation]

### Preserving historical fixtures without a production bypass

Existing v2.5 fixtures intentionally use synthetic roots, caller work, static flags and sometimes custom maturity. They remain valuable, narrower regression evidence; they cannot become canonical runtime authorization. Keep old receipts immutable. Route active synthetic tests through visibly named fixture constructors/owners, leaving ordinary constructors gated; do not set a production boolean to silently accept them. [VERIFIED: PROJECT.md current-state limits; historical fixture APIs; D-08; recommendation]

There is an existing `ManagedNetworkHandle::from_network_fixture` boundary. Prefer extending explicit lower-level fixture construction and using internal/unit-test helpers over changing networkless arithmetic/data carriers globally. Cross-crate tests may need a carefully scoped explicit fixture API; it must never be selected by daemon/CLI config or grant the ordinary compatibility capability. If it remains publicly callable as a library mechanism, document its evidence scope and add checks that ordinary production call sites cannot invoke it. Do not label successful fixture reopen as canonical reopen. [VERIFIED: context/network.rs:64,143; runtime_authority fixture interface; recommendation]

## Don't Hand-Roll

| Problem | Use instead |
| --- | --- |
| Genesis encoding/hash | Existing codecs and consensus hash functions plus audited pinned vectors. [VERIFIED: codec/block.rs; consensus/crypto.rs] |
| Read-only legacy DB reader | Filesystem-only unproved refusal in this phase; do not build an SST/WAL parser or copy/recovery system. [VERIFIED: backend hazards; D-04; recommendation] |
| Support/default tables | One pure spec with exhaustive adapter mappings. [VERIFIED: D-02; recommendation] |
| New work/conversion logic | Existing ProofWork and Phase165 ownership through the compatibility seam. [VERIFIED: ROADMAP.md] |

## Runtime State Inventory

This phase introduces compatibility refusal at existing storage boundaries; it does not rename a runtime identity or authorize a data migration. [VERIFIED: CONTEXT.md boundary]

| Category | Items found / evidence | Action required |
| --- | --- | --- |
| Stored data | Fjall headers/index snapshots, chain_meta, coins B/H and outpoints, undo/body keys, validation-history coverage, BASIC and prune state are identity-sensitive; schema 1 migration and recovery effects inspected. | Code gate only; preserve every existing record, including unsupported/unproved data. No data migration. [VERIFIED: storage/fjall_store*.rs and child modules] |
| Live service config | Ordinary daemon configs/selectors and operator paths select the storage boundary; no live service was contacted or modified in this research. | Enforce selected mode at code boundary; do not claim an audit of external running instances. [VERIFIED: opener inventory; tool-call scope] |
| OS-registered state | No rename/re-registration required by phase; service-manager registrations were not queried. | None within authorized scope; do not change registrations. [VERIFIED: phase boundary; tool-call scope] |
| Secrets/env vars | No secret or environment-variable rename is part of this phase; secret values were not read. | No credential change; no environment legacy bypass. [VERIFIED: phase boundary; tool-call scope] |
| Build artifacts | Existing Cargo/Bazel outputs and pinned dependency checkout; no package rename. | Normal rebuild and final native verification; no installed-package migration. [VERIFIED: manifests; phase boundary] |

## Common Pitfalls

- Opening before inspection already permits backend mutation. Root checks after `open_unrecovered` fail CBOT-04. [VERIFIED: pinned Fjall recovery; fjall_store.rs]
- Empty filesystem state is not canonical authority. Do not implement partial bootstrap or genesis coin insertion in Phase164. [VERIFIED: D-07; validation.cpp; Phase166 ownership]
- Shared genesis tx/merkle cannot distinguish networks; raw and display hash orders differ. Test pairwise foreign roots and body near-misses. [VERIFIED: vectors; crypto.rs]
- Header-only gates miss local/stored connects, snapshot seeding, reorg and RPC memory fallback. Swallowed compatibility errors in wallet/soak paths must remain visible failures/unavailable facts. [VERIFIED: seam/opener inventories]
- Existing Complete validation coverage and synthetic fixture receipts do not certify canonical work/context. Preserve old receipt meaning and prohibit production fixture bypasses. [VERIFIED: validation_history.rs; PROJECT.md; D-08]
- Non-opening inspection cannot name actual stored genesis/network: report proof unavailable. [VERIFIED: proposed mechanism; D-04]

## Code Examples

Proposed ordering (names are recommendations, not existing APIs): `inspect_datadir_without_backend(path) -> decide_compatibility(spec, observation) -> proved ordinary capability -> mutable backend open`. Phase164 cannot construct the last capability while prerequisites are missing. [VERIFIED: D-03,07; recommendation]

For pure identity tests, parse the selected 285-byte vector with existing `parse_block`, compare `encode_block_header(&block.header)`, `encode_block(&block)`, and `block_hash(&block.header)` against independent pinned literals. Identity success must not create chainstate authority. [VERIFIED: codec/block.rs; consensus/crypto.rs; vectors; recommendation]

## Verification Design

Nyquist-generated Validation Architecture is disabled (`workflow.nyquist_validation: false`), so this report does not prescribe a Nyquist/Wave-0 artifact. Native Rust tests and focused Bun checker tests still belong in implementation and the default verifier. [VERIFIED: .planning/config.json; scripts/verify.sh]

| Obligation | Meaningful checks | Suggested focused command/filter |
| --- | --- | --- |
| CBOT-01 | Four exact byte/header/txid/hash vectors; round-trip encoding; each selected mode rejects the other three roots; one-field and one-body-byte mutations; arbitrary valid-PoW null root; root check precedes insertion/publication | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus canonical_chain` plus node gate tests |
| CBOT-04 | Real closed Fjall schema1/schema2 fixtures, malformed/foreign/synthetic state, prune intent, BASIC records, wallets/metrics sentinels; snapshot full tree before/after ordinary attempts; open spy always zero; no schema/coins/history/index/prune invocation | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chain_compatibility` |
| CVLD-05 | Exhaustive selectors → canonical identity/support facts; Signet challenge reason; missing work/bootstrap/context gates; no ordinary memory fallback or callable production config bypass | node/RPC `canonical_chain` or `chain_compatibility` filters |
| Ordinary entrypoints | Daemon runtime, metrics fallback, wallet/mempool context fallback, CLI offline status/pause/resume, wallet backup/name resolution and soak enrichment preserve source | RPC binary and CLI operator focused filters |
| Evidence isolation | Synthetic tests use explicit fixture construction; production call-site checker rejects fixture authorization; old receipts unchanged; current docs state narrowed ordinary capability | New scoped Bun checker tests plus diff review |

[VERIFIED: requirements and inventoried seams; commands are recommended test filters to create, not evidence of existing test names or passed execution]

Use the timing wrapper, for example `bun run scripts/command-timings.ts run --key phase164-node-compatibility -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chain_compatibility`. Do not run those commands concurrently against the same Cargo target directory. Final gate is the full `bash scripts/verify.sh`, parent-owned review/security checks and `git diff --check`; pure-core coverage must include new decision branches. Research itself ran no Cargo/Bazel build or runtime acceptance test. [VERIFIED: AGENTS.md verification/timing contract; tool-call scope]

### Recommended plan slices

1. **Pure identity and vectors:** Add the four-mode spec and exact header/body checks, exhaustive selector conversion and support facts; add bytes/hash/near-miss tests and breadcrumbs. No bootstrap/UTXO/work effects. [VERIFIED: D-01,02,06; recommended slice]
2. **Non-opening compatibility seam:** Add bounded typed filesystem observation/refusal and ordinary opener admission shape; add backend spy plus physical byte-preservation fixtures; reserve separate work classification for Phase 165. [VERIFIED: D-03–05; recommended slice]
3. **Daemon/RPC/operator durable wiring:** Route every inventoried opener through selected-network inspection, propagate typed refusal, remove swallowed/memory fallback authorization, verify before all namespace/recovery/migration/index/prune effects. [VERIFIED: opener inventory; recommended slice]
4. **Ordinary header/body gates and fixture boundary:** Bind shared managed admission/connect/reorg paths to identity/support; guard imported snapshots and in-memory ordinary paths; adapt active synthetic test construction explicitly, without ordinary flags/environment escape. [VERIFIED: root seam inventory; D-07,08; recommended slice]
5. **Operator truth and closeout:** Current README/parity companion scope, bounded diagnostics, repo-local UAT, phase checker/native registration and default verification. Preserve all historical receipts; mark only Phase 164 owners after lifecycle-valid verification. [VERIFIED: AGENTS.md; REQUIREMENTS.md traceability; recommended slice]

Slices 3 and 4 depend on 1–2 and should be coordinated around the same authority type. Tests and implementation must ship green together under existing hooks. The simplification pass should specifically reject introducing a bespoke DB reader or global fixture-mode switch where early typed refusal suffices. [VERIFIED: repo native pre-commit contract; phase boundary; recommendation]

## State of the Art

The intended transition is null-parent inference to exact selected identity; mutable-open inspection to non-opening refusal; selector defaults to explicit prerequisite gates; and implicit fixture reuse to named evidence classes. Canonical bootstrap/reopen, exact work and history-derived validation remain later owners. [VERIFIED: inspected current source; D-01–08; ROADMAP.md; recommendation]

## Environment Availability

| Dependency | Required by | Available | Observed / fallback |
| --- | --- | --- | --- |
| Pinned Knots source | Canonical authority | Yes | Submodule HEAD exactly a9aee730466ac67d35a3c03ee24676be5e045878. |
| Rust/Cargo | Implementation tests | Yes | Both 1.94.1. |
| Fjall pinned source | Backend audit | Yes | Cargo git checkout aa30dca, package 3.1.10. |
| Bun | Automation | Yes, PATH differs | PATH 1.4.2; cached 1.3.9 at `/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin/bun`. |
| Bazel | Native smoke | Yes | 8.6.0. |
| Actual Knots executable | Phase168 differential | Not audited here | Not required to implement/refuse in Phase164; no fake oracle fallback. |

[VERIFIED: version/path/submodule probes; manifests; later-phase ownership]

No missing blocking runtime was identified for this research. Parent should select pinned Bun before final native verification. Host tool availability does not prove tests/builds pass. [VERIFIED: probes; AGENTS.md pin contract; recommendation]

## Security Domain

Security enforcement is not explicitly disabled in config. The applicable scope is hostile/unproved disk input, peer/local candidate admission and truthful diagnostics; Phase164 changes no authentication/session protocol. The category labels below use the ASVS4 taxonomy expected by the phase template; current OWASP ASVS has a newer release and this is a control lens, not a compliance claim. [VERIFIED: .planning/config.json; phase boundary; https://owasp.org/projects/asvs]

| ASVS category | Applies | Standard control / mapping |
| --- | --- | --- |
| V2 Authentication | No new auth | Preserve existing authenticated RPC boundaries; chain identity is not user authentication. |
| V3 Session management | No new session state | Preserve existing session/peer owners. |
| V4 Access control | Yes, internal authority | Nonconstructible ordinary compatibility capability; fixture/data carriers cannot authorize source mutation. |
| V5 Input validation | Yes | Typed exhaustive selectors, exact bounded root bytes, early shape validation, errors propagate. |
| V6 Cryptography | Identity hashing only | Existing SHA256d/typed-hash implementation; no bespoke cryptography or secret storage. |

[VERIFIED: inventoried code and D-01–08; applicability and controls are recommended engineering mappings]

| Threat | STRIDE | Mitigation to test |
| --- | --- | --- |
| Foreign/synthetic root admitted | Spoofing/tampering | Exact selected root plus support gate before mutation/publication. |
| Backend recovery alters refused source | Tampering | Filesystem-only refusal; zero backend calls; physical tree comparison. |
| Very large/unreadable/symlink datadir | Denial of service/tampering | Constant-space first-entry observation, no recursion/read-all, fixed diagnostic codes, explicit I/O failure. |
| Fixture API or fallback grants ordinary authority | Elevation of privilege | Separate fixture owner, no ordinary config/env escape, production call-site tests. |
| Raw paths/backend text leak or expand diagnostics | Information disclosure/DoS | Fixed bounded reason + selected identity + unavailable observed fields; cap optional path detail; no wallet/file contents. |
| Old receipt relabeled canonical | Repudiation | Preserve original receipts/source identity and label current evidence classes. |

[VERIFIED: phase requirements; inspected mutation/admission/fallback hazards; mitigations are recommendations]

Resource bounds should be explicit local implementation policy, not claimed Knots consensus limits. Recommend fixed-size enums and at most a small fixed diagnostic message (for example 1 KiB), one directory entry and no source-file reads for the initial gate. Test bound enforcement and permission errors; never claim an exact stored network/hash without reading/proving it. [VERIFIED: D-04; recommended bounds, no external performance target assumed]

## Assumptions Log

| # | Claim | Section | Risk if wrong |
| --- | --- | --- | --- |

No training-only factual claim is used. Implementation signatures, policies, budgets and plan slices above are explicitly recommendations grounded in locked scope and inspected source, not assertions that those APIs exist or tests passed. [VERIFIED: research method/tool-call scope]

## Open Questions — RESOLVED for Phase 164

1. **RESOLVED — Future positive record-bound inspection:** Phase 164 refuses every existing unproved source before backend open. Positive canonical record-bound reopen remains deferred to Phase 166; this resolution does not claim its design is solved, and a sidecar alone remains insufficient. [VERIFIED: backend effects; D-04,07; ROADMAP.md Phase166]
2. **RESOLVED — Exact fixture API visibility:** Use explicitly named fixture constructors where cross-crate historical tests require visibility, because dependency crates do not inherit their caller's `cfg(test)`. Plans assign the named adapters and smallest test adaptations; ordinary call-site audits and supplied-handle refusal tests prevent an operator runtime-config bypass. [VERIFIED: inspected cross-crate RPC fixture usage; existing fixture owner; Phase 164 plans]
3. **RESOLVED — Intermediate operator behavior:** Offline wallet/status enrichment reports compatibility refusal or unavailable identity without mutable reads of legacy source state. Live RPC against an already separately owned process remains a separate path and grants no local-store opening exemption. [VERIFIED: opener inventory; D-03–05; Phase 164 plans]

## Sources

### Primary (HIGH confidence)

- Local `164-CONTEXT.md`, REQUIREMENTS.md, ROADMAP.md, STATE.md, PROJECT.md; milestone research SUMMARY.md, ARCHITECTURE.md and PITFALLS.md: locked phase boundary, downstream ownership and historical evidence limits. [VERIFIED: local reads]
- Local AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, standards/core/{architecture,verification,testing,code-shape}.md and standards/languages/rust.md: material project constraints. [VERIFIED: local reads]
- [Pinned Knots chain parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp): local constructors/assertions and official source page verified; exact genesis, profiles, default Signet challenge. [VERIFIED: local source; CITED: linked pinned official source]
- Local pinned `validation.cpp` AcceptBlockHeader, CheckBlock, ConnectBlock; `signet.cpp` CheckSignetBlockSolution: root/parent order, genesis UTXO special case and required Signet body challenge. [VERIFIED: local pinned source]
- Local first-party sources listed in root/opener/mutation inventories; `codec/block.rs`, `consensus/crypto.rs`, `primitives/hash.rs`: exact APIs and current mutation order. [VERIFIED: local source]
- [Pinned Fjall db.rs](https://github.com/fjall-rs/fjall/blob/aa30dca811399a201e0b9595da93a4582dcb2b57/src/db.rs), local Cargo checkout `builder.rs`, `db_config.rs`, `locked_file.rs`, `recovery.rs`, `journal/recovery.rs`: absence of read-only option in inspected pinned API and concrete open/recovery effects. [VERIFIED: Cargo git source; CITED: linked official source]
- Independent Python stdlib struct/hashlib computation in this research: exact canonical transaction/header/block bytes and four hash assertions reproduced. This is source-derived vector evidence only. [VERIFIED: session output]
- [OWASP ASVS](https://owasp.org/projects/asvs): current security standard project; category application above is a local engineering mapping, not certification. [CITED: owasp.org/projects/asvs]

### Secondary and Tertiary

No community/unverified ecosystem source was needed; version-specific source was available locally. Context7 was not available among session tools. No package upgrade or external library recommendation required registry research. [VERIFIED: available tools; local source inventory]

## Metadata

**Confidence breakdown:** Standard stack HIGH (manifest/lock/pins/probes); constants HIGH (pinned assertions plus independent serialization); mutation hazards HIGH (concrete source calls); recommended architecture MEDIUM (implementation/test proof remains); later positive reopen mechanism unresolved and explicitly deferred. [VERIFIED: findings above]

**Validity:** pin-specific findings hold for the inspected revisions; rerun opener/backend audits after source/dependency changes. **Execution evidence:** no implementation, Cargo test, Bazel build, actual Knots daemon run or source-datadir mutation performed. **Commit:** intentionally left to parent strict workflow after clean phase verification. [VERIFIED: tool-call scope; parent assignment]

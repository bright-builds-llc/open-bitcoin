---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-14T16:23:00Z
---

# Phase 165: Shared Exact Work Authority - Research

**Researched:** 2026-10-14
**Domain:** First-party compact target arithmetic, earned work provenance, candidate eligibility, pre-mutation compatibility
**Confidence:** HIGH for inspected behavior; MEDIUM for proposed integration

<user-constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Exact arithmetic and target admission

- **D-01:** Extend the existing first-party consensus `ProofWork`; no additional dependency or parallel arithmetic implementation. Numerical work alone is not validation provenance.
- **D-02:** Derive block proof as floor(2^256 / (target + 1)), accumulate modulo 2^256, compare all unsigned 256 bits, and use lossless byte/hex boundaries. No extra cumulative-overflow refusal; any future refusal requires an intentional parity difference.
- **D-03:** Match pinned compact normalization, sign, zero and overflow boundaries, including tiny mantissas truncated to zero and equivalent encodings. Header admission additionally requires selected-network powLimit and hash satisfaction. Share decoding validity between proof and admission without claiming contextual difficulty completeness.
- **D-04:** Independently expected vectors cover values beyond 128 bits, mixed targets, carry/wrap and negative/zero/overflow edges. Keep arithmetic/source-derived evidence separate from actual-daemon differential qualification.

### Header and connected branch ordering

- **D-05:** Accepted headers derive cumulative work from accepted parent plus own target proof. Update the best header only on strictly greater work; equal work preserves the first accepted best header, and a shorter higher-work branch wins.
- **D-06:** Connected candidate ordering derives the same work internally, followed by pinned body/ancestor eligibility arrival order. Header arrival is not body eligibility order. Consider eligible retained side branches as well as the best-header branch; an equal-work first-header branch must not suppress the earlier eligible connected candidate.
- **D-07:** Enforce work/eligibility preference at actual connect and reorg owners before mutation. Inflated caller numbers, forged position metadata, direct managed reorg entrypoints and staged previews cannot authorize a stronger active branch. Preserve explicit fixture boundaries where historical tests need synthetic roots.
- **D-08:** Define known equal-work activation cases from the pin; document truly ambiguous internal/restart ties as bounded evidence. Do not describe a deterministic hash/pointer fallback as proven Knots activation parity.

### Safe legacy compatibility and scope

- **D-09:** Extend Phase 164's filesystem-only ordinary gate with an explicit work-compatibility decision. Refuse unprovable legacy synthetic-work state before Fjall recovery, coins/schema migration, initialization, prune resume, BASIC writes or live authority changes. Report observed work as unavailable when bytes have not been proved; never infer a profile from a forged marker or file shape.
- **D-10:** Keep explicit legacy codec representation/classification only where the shared-work transition requires it for historical snapshot fixtures. Decoded u128 scores are legacy numbers, never silently zero-extended into verified authority. No automatic repair, destructive conversion or reinterpretation.
- **D-11:** Prove refusal through closed source-manifest preservation, backend-spy and migration/prune controls, malformed/forged inputs and ordinary operator/daemon routes. Preserve all four selector support/refusal classifications and default-off networking.
- **D-12:** Phase 166 owns canonical durable bootstrap, exact-work checkpoint/reopen acceptance and genesis BASIC; Phase 167 owns contextual consensus prerequisites; Phase 168 owns actual pinned-daemon differential qualification. Representation changes needed to compile Phase 165 are permitted, but do not claim those later outcomes.

### Agent Discretion

Choose small modules, restricted authority constructors, bounded diagnostics and focused behavioral tests. Reuse existing networking, chainstate and storage seams, preserve one active Fjall chainstate and pure-core/shell boundaries. Research must distinguish compatibility number carriers from earned runtime authority and identify all call sites that can select an active branch.

### Deferred Ideas (OUT OF SCOPE)

Canonical bootstrap, exact durable recovery authority, contextual history validation and actual-daemon differential acceptance retain Phases 166–168 ownership. No new networks, automatic legacy repair, public-default activation or production/funds claims. No pending todos matched this phase.
</user-constraints>

The preceding decisions and discretion are copied verbatim from the phase context. [VERIFIED: .planning/phases/165-shared-exact-work-authority/165-CONTEXT.md]

<phase-requirements>
## Phase Requirements

| ID      | Description                                                                                                                                                                                                                                                                                                                      | Research Support                                                                                                                                   |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| CWOR-01 | A contributor obtains exact target-derived 256-bit block proof and cumulative work with pinned compact-target validity and cumulative-arithmetic semantics, including values beyond 128 bits; any additional overflow refusal is documented as an intentional difference rather than silently changing the reference arithmetic. | Reuse ProofWork, correct compact normalization, independent fixed vectors, lossless boundaries.                                                    |
| CWOR-02 | An operator's live header best-chain selection follows verified cumulative proof of work rather than header count; defined equal-work arrival/activation behavior matches the pinned reference, while genuinely ambiguous internal ties remain explicitly bounded.                                                               | Merge actual header selection with earned arithmetic; strict best-header updates; body/ancestor eligibility sequence and retained side candidates. |
| CWOR-03 | An operator's connected-chain and reorg selection consume the same internally verified work authority as accepted headers; caller-supplied work cannot authorize a stronger active branch.                                                                                                                                       | Restrict provenance/stages, validate parent lineage, derive connect work, guard direct managed reorg before preview.                               |
| CWOR-05 | An operator opening legacy synthetic-work state receives an explicit compatibility decision before automatic coins/schema migration or live/durable authority changes; unprovable state refuses without silent reinterpretation, destructive conversion or automatic repair.                                                     | Typed explicit work decision in filesystem-only gate; legacy classification without source Fjall open; closed manifest and backend spy.            |

Requirement descriptions are copied from the active milestone requirements. [VERIFIED: .planning/REQUIREMENTS.md]
</phase-requirements>

## Summary

The repository already contains the exact arithmetic needed: consensus `ProofWork([u64; 4])`, unsigned ordering, wrapping addition/subtraction and target-derived block proof. However, `HeaderStore::best_tip` still follows caller-compatible `u128` count scores, while `header_store/proof.rs` maintains separate exact arithmetic facts and another best tip. Promoting the advisory map without retiring the count authority would leave two competing branch selectors. [VERIFIED: packages/open-bitcoin-consensus/src/chain_proof.rs; packages/open-bitcoin-network/src/header_store.rs; packages/open-bitcoin-network/src/header_store/proof.rs]

Numerical correctness is separate from validation provenance. Raw `ChainPosition` values, `HeaderStore::from_entries`, `Chainstate::from_snapshot/from_parent/from_coins_cache`, and public connect-stage fields can supply plausible or inflated metadata. Current managed reorg preparation accepts anchored caller scores and installs a preview before mempool and accepted-state publication. Put earned-parent and preference checks at the owners of those effects, not solely in sync reconciliation. [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs; packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-chainstate/src/engine/stage.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

**Primary recommendation:** Make `ProofWork` the sole exact number carrier, keep legacy-number decoding explicitly untrusted, and mint restricted live authority from accepted ancestry and checked bodies. Add a separate eligibility registry for all retained candidate branches and an explicit refusal decision before ordinary backend opening. Canonical initialization/recovery acceptance, contextual completeness and actual daemon qualification stay deferred. [VERIFIED: 165-CONTEXT.md D-01, D-06–D-12; .planning/ROADMAP.md Phases 166–168]

## Project Constraints (from AGENTS.md)

These sources materially constrain the plan: root `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture/code-shape/verification/testing pages and Rust/TypeScript pages. No `.claude/skills` or `.agents/skills` directory was present. Both active lesson files were loaded within their combined 7,188-byte / 2,397-estimated-token budget; repo lessons reinforce explicit Cargo/Bazel operator commands. [VERIFIED: local instruction files; active lesson files; filesystem probes]

- Use the pinned Knots source and preserve auditable parity breadcrumbs. Add every new first-party Rust source/test file to `docs/parity/source-breadcrumbs.json`; update `docs/parity/index.json` and companion documents for intentional behavior differences. [VERIFIED: AGENTS.md]
- Keep domain arithmetic and preference decisions pure; node adapters own filesystem, database, networking and lifecycle effects. Keep one active Fjall chainstate. No production Rust Bitcoin library or new dependency for this phase. [VERIFIED: AGENTS.md; standards/core/architecture.md; 165-CONTEXT.md D-01]
- Use Rust 1.94.1 / edition 2024, existing Cargo workspace and Bazel/Bzlmod. Bun owns substantial automation; direct Bun test paths require `./`. [VERIFIED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml]
- Use `foo.rs` plus `foo/`, early returns, optional internal `maybe_` names, documented public APIs and explicit errors. Functions around 161 lines/files around 628 lines trigger decomposition. No new unwrap/panic shortcuts or swallowed failures. [VERIFIED: AGENTS.bright-builds.md; standards/core/code-shape.md; standards/languages/rust.md; scripts/check-panic-sites.sh]
- Test one concern per unit test with Arrange/Act/Assert, preserve meaningful business-logic coverage and verify observable refusal/state preservation. [VERIFIED: standards/core/testing.md; AGENTS.md]
- The default `bash scripts/verify.sh` is the completion/pre-commit contract. Ad-hoc Cargo/Bazel commands must use `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`; do not overlap Cargo against the same target directory. Keep hooks enabled and tracked LOC freshness changes. [VERIFIED: AGENTS.md]
- Preserve historical artifacts consumed by native verifiers. Scoped current checker-contract renewals need review and fresh evidence; never rewrite old receipts to imply canonical qualification. [VERIFIED: .planning/STATE.md Pending Todos; scripts/check-phase164-canonical-identity.ts]
- This research is inside the parent `gsd-plan-phase 165 --auto` lifecycle. Write only this research artifact; the strict wrapper owns all commits after clean verification. No source, Git or config mutation in this research task. [VERIFIED: assigned task; 165-CONTEXT.md Specific Ideas]

## Standard Stack

### Core

| Component               | Verified version / identity                                  | Purpose                                          | Action                                                                                     |
| ----------------------- | ------------------------------------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| First-party `ProofWork` | Current checkout, four u64 limbs                             | Proof, unsigned order, modulo arithmetic         | Extend; do not replace. [VERIFIED: consensus/src/chain_proof.rs]                           |
| Rust                    | 1.94.1; installed probe matches pin                          | Core and adapters                                | Existing toolchain. [VERIFIED: rust-toolchain.toml; rustc --version]                       |
| Knots source oracle     | 29.3.knots20260210; a9aee730466ac67d35a3c03ee24676be5e045878 | Exact reference formulas and activation behavior | Use local source pin. [VERIFIED: git -C packages/bitcoin-knots rev-parse HEAD; PROJECT.md] |
| Cargo / Bazel           | Existing workspace and root Bzlmod                           | Compile/verify carrier changes                   | No new build stack. [VERIFIED: packages/Cargo.toml; MODULE.bazel; AGENTS.md]               |

### Supporting

| Component          | Locked version                                              | Purpose / restriction                                                                                                                                            |
| ------------------ | ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fjall              | 3.1.10 at revision aa30dca811399a201e0b9595da93a4582dcb2b57 | Existing durable adapter; ordinary compatibility must refuse before opening it. [VERIFIED: packages/Cargo.lock; node/Cargo.toml; storage/chain_compatibility.rs] |
| serde / serde_json | 1.0.228 / 1.0.149                                           | Node-owned DTOs; exact bytes/hex plus explicit legacy classification. [VERIFIED: packages/Cargo.lock; storage/snapshot_codec.rs]                                 |
| Bun                | Pin 1.3.9; shell probe 1.4.2                                | Repo automation and behavioral checker tests; ensure pinned verifier environment. [VERIFIED: .bun-version; bun --version; AGENTS.md]                             |

**Installation:** None. This phase adds no dependencies; retain locked versions rather than upgrading to registry latest. Package publish-date research is not applicable to an internal carrier change. [VERIFIED: 165-CONTEXT.md D-01; assigned task]

### Alternatives Considered

| Instead of                         | Alternative                                          | Decision                                                                           |
| ---------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Existing ProofWork                 | Big integer crate or independent work implementation | Rejected by D-01.                                                                  |
| Shared authoritative header map    | Promote advisory exact map alongside count-based tip | Retains conflicting selectors found in current source; retire competing authority. |
| Explicit legacy DTO/classification | Zero-extend all old u128 fields                      | Rejected by D-10; numbers are not earned provenance.                               |

Decision basis: locked context and inspected current header-store design. [VERIFIED: 165-CONTEXT.md; network/src/header_store.rs; network/src/header_store/proof.rs]

## Architecture Patterns

### Recommended Ownership and Structure

Use small child modules in the existing owners; the following names are recommendations, not existing APIs. [VERIFIED: standards/core/code-shape.md; standards/languages/rust.md]

```text
consensus/chain_proof.rs + chain_proof/
  fixed-width boundaries and arithmetic vectors
consensus/crypto.rs
  normalized compact target validity reused by proof/admission
network/header_store.rs + header_store/
  accepted ancestry/exact work and strict best-header update
  transaction/body ancestry eligibility and candidate ordering
chainstate/engine.rs + engine/
  earned-parent derivation, restricted stages, fixture compatibility
node/network/mempool_lifecycle.rs
  direct managed connect/reorg authorization before lifecycle effects
node/sync/block_reconcile.rs
  all eligible retained branches, no best-header-only activation
node/storage/chain_compatibility.rs
  filesystem-only work decision, no source backend opening
node/storage/snapshot_codec.rs + snapshot_codec/
  lossless exact carrier plus explicitly classified legacy DTOs
scripts/check-phase165-shared-exact-work*.ts
  current evidence boundaries and mutation-tested guard
```

### Pattern 1: Normalize compact decoding once

Knots `SetCompact` first shifts the mantissa for exponents \<=3, then derives sign/overflow flags from that normalized word. The Rust decoder currently tests the raw sign before truncation and can return zero bytes for a tiny nonzero mantissa. Consequently, zero-after-truncation and negative-zero error classification require deliberate tests. Equivalent positive encodings must be accepted as equal targets; do not impose canonical-encoding uniqueness. [VERIFIED: bitcoin-knots/src/arith_uint256.cpp:175; consensus/src/crypto.rs:125]

Separate the normalized decode result from admission policy: block proof accepts a positive nonzero nonoverflow target; admission additionally compares selected-network powLimit and verifies the header hash. `GetBlockProof` returns zero on invalid compact input in Knots, while the existing first-party Result API returns an error; retain a documented admission/error mapping rather than claiming identical internal API shape. [VERIFIED: bitcoin-knots/src/chain.cpp:131; bitcoin-knots/src/pow.cpp:146; consensus/src/chain_proof.rs]

The current generic `check_block_header` checks hash versus compact target; existing `ConsensusParams` carries `pow_limit_bits`. Route selected-network admission through an explicit range check; do not infer contextual difficulty completeness from it. The chain specification's missing prerequisites and canonical admission refusals remain authoritative. [VERIFIED: consensus/src/block.rs:88; consensus/src/context.rs; node/src/network/header_sync.rs; node/src/network/canonical_admission.rs; 165-CONTEXT.md D-03, D-12]

### Pattern 2: One exact numeric carrier, restricted earned provenance

Add stable `[u8; 32]` boundaries and lossless hex rendering to the existing ProofWork, declaring byte endianness in the API. Bytes can represent a number without proving acceptance. A private-field accepted-parent/lineage value must come from checked admission or actual managed absorption, not `ChainPosition::new`, a decoded snapshot, a matching header hash, or an arbitrary coins best-block. [VERIFIED: chain_proof.rs documentation; chainstate/src/types.rs; chainstate/src/engine.rs constructors; node/src/chainstate/validation_history.rs]

For live connect derive `parent_earned_work.wrapping_add(block_proof(bits))` internally. For reorg derive every suffix position from the genuine retained ancestor; verify parent links, hash/header identity, heights and authority lineage. Recomputing a candidate's proof while still trusting a forged parent's accumulated number is insufficient. A whole imported vector's arithmetic consistency still does not establish that its bodies or UTXOs were validated. [VERIFIED: 165-CONTEXT.md D-01, D-05, D-07; chainstate/src/engine/overlay_apply.rs; node/src/chainstate/validation_history.rs]

Restrict connect-stage construction/mutation as well as reorg-stage construction. `StagedChainstateReorg` already has private fields and compile-fail proofs, but `StagedChainstateConnect` currently exposes overlay, position and next-chain fields publicly. Audit commit/absorb/preview consumers and bind stages to the source generation/tip before publication. Use existing accepted-state receipts as a pattern, without claiming their existing BASIC lineage receipts certify work. [VERIFIED: chainstate/src/engine.rs; chainstate/src/engine/stage.rs; node/src/chainstate/validation_history.rs]

Keep synthetic roots and legacy score fixtures behind an explicit compatibility boundary that cannot construct ordinary earned authority. Existing lower-level raw snapshots/stores are historical-fixture seams; do not globally remove them or quietly bless them by recomputing numbers. A fixture boundary may exercise the real transaction engine while retaining explicit evidence classification. [VERIFIED: network/canonical_admission.rs AdmissionScope; 165-CONTEXT.md D-07, D-10–D-12; scripts/check-phase164-canonical-identity.ts]

### Pattern 3: Header preference differs from activation eligibility

Knots `AddToBlockIndex` derives cumulative work from parent plus GetBlockProof and updates the best header only on strictly greater cumulative work. Header ties preserve the first accepted best header, regardless of hash or height. Both current Rust selectors add height/hash tie rules; remove those rules from live header preference. Recovered order from BTreeMap iteration is not original acceptance order. [VERIFIED: bitcoin-knots/src/node/blockstorage.cpp:252–280; network/src/header_store.rs; network/src/header_store/proof.rs]

Connected candidate preference is work first, then smaller eligibility sequence. Knots assigns that sequence only when full checked data and transaction-valid ancestry become available; header reception does not mint it. `ReceivedBlockTransactions` releases previously unlinked descendants through a FIFO queue, so a body received before its ancestor becomes eligible at ancestor arrival. Preserve pending-child order rather than sorting descendants by hash. [VERIFIED: bitcoin-knots/src/node/blockstorage.cpp:194; bitcoin-knots/src/validation.cpp:4205–4265]

Maintain an eligibility registry owned by admission/runtime, with idempotent body acceptance, pending children, eligibility sequence, failed/unavailable status and candidate enumeration. Selection must inspect all eligible retained branches and required suffix bodies, not only `best_chain_entries()`. A best-header A accepted first must not suppress equal-work B whose bodies became eligible first. A higher-work header without bodies must not block an available eligible lower-work candidate that outranks active state. [VERIFIED: 165-CONTEXT.md D-06; bitcoin-knots/src/validation.cpp:3585; node/src/sync/block_reconcile.rs:163]

For equal work, compare the active tip's earned eligibility as well as the candidate's. Later eligible equal-work candidates do not replace an earlier eligible active tip; eligibility, rather than header arrival, governs the known runtime case. Knots' final pointer-address tie breaker for shared disk sequence IDs makes restart/internal ties bounded evidence, not hash-order parity. Do not serialize a deterministic hash fallback and label it proven activation parity. [VERIFIED: bitcoin-knots/src/node/blockstorage.cpp:194–210; validation.cpp TryAddBlockIndexCandidate/PruneBlockIndexCandidates; 165-CONTEXT.md D-08]

### Pattern 4: Guard before preview or mutation

The direct chain is `ManagedNetworkHandle::reorg_to_branch -> ManagedPeerNetwork::reorg_to_branch -> prepare_reorg -> stage_reorg -> install_prepared_reorg_preview`, followed by peer/mempool state and commit. Sync's caller-side candidate comparison is bypassed by direct calls. Preference/provenance refusal must happen before preview and before accepted lineage, BASIC, mempool, coins, block publication and tip notifications. [VERIFIED: node/src/network/runtime_authority.rs:403; node/src/network/mempool_lifecycle.rs:140; node/src/chainstate.rs:388; node/src/network/runtime_authority/filter_index/reorg.rs]

Pure chainstate can retain an explicit mechanical staging/fixture seam for disconnect-only and historical short-branch tests. It must not provide a freely forged stage that a managed live owner can absorb. Test each entrypoint: local connect, stored connect, direct managed reorg, stage/preview/commit, forged snapshot/parent and normal sync candidate reconciliation. Keep genuine owner checks together; avoid requiring every caller to remember the same predicate. [VERIFIED: 165-CONTEXT.md D-07; chainstate/src/engine/stage.rs; node/src/network/mempool_lifecycle.rs; node/src/sync/tests/filter_index/reorg/branches.rs]

### Pattern 5: Explicit pre-backend compatibility refusal

Phase 164 `open_ordinary` always refuses based on bounded filesystem shape; `WorkProfileObservation` currently has only Unavailable. Raw Fjall open reaches Database::open, recovery and schema handling. Add a work-compatibility decision separate from observed work, preserving Unavailable when no bytes have been proved. An existing-state decision can state work is unproved/unsupported without claiming to have positively identified a legacy profile. [VERIFIED: node/src/storage/chain_compatibility.rs; node/src/storage/fjall_store.rs]

Preserve all existing observation precedence, redaction and selector scope. Forged marker names, exact-looking JSON or Fjall directory shape cannot turn Unavailable into verified exact work. Do not open Fjall to classify the store, add file-content inspection merely to distinguish legacy, or repair/reencode on refusal. Source payloads, recovery markers, coins/B/H, schema, prune intent, BASIC rows and wallet/config state must remain unchanged. [VERIFIED: 165-CONTEXT.md D-09–D-11; node/src/storage/chain_compatibility/tests/preservation.rs; 164-VERIFICATION.md]

### Anti-Patterns to Avoid

- Public `From<u128>` or generic integer constructors treated as earned work.
- Separate synthetic best-tip and exact best-proof authorities.
- Inferring accepted parent state from hash/height, arithmetic consistency or matching coins B alone.
- Assigning candidate order at header arrival or resetting it on duplicate body arrival.
- Restricting activation to the best-header branch.
- Authorizing reorg only in sync; preview can mutate before downstream preference checks.
- Silently zero-extending legacy DTO numbers or trying source Fjall open to inspect them.
- Changing ordinary support classifications after adding only arithmetic.

These are prohibited or contradicted by locked D-01/D-05–D-12 and inspected source seams. [VERIFIED: 165-CONTEXT.md; sources cited in Patterns 1–5]

## Don't Hand-Roll

| Problem                 | Do not build                                                     | Use instead                                                              |
| ----------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Exact arithmetic        | Another limb implementation, big integer production dependency   | Existing ProofWork and shared normalized compact decoder.                |
| Proof of acceptance     | Marker string, public score wrapper or recomputed imported score | Restricted authority minted from checked ancestry and actual absorption. |
| Storage inspection      | An alternate Fjall reader or recovery-driven classifier          | Existing bounded filesystem-only gate.                                   |
| Runtime lifecycle       | Alternate mempool/BASIC publication owner                        | Existing staged managed lifecycle with earlier guards.                   |
| Differential acceptance | Self-comparison or formula runner described as daemon parity     | Source/vector evidence now; actual pinned daemon owner Phase 168.        |

Basis: locked context, existing arithmetic, gate and managed owner. [VERIFIED: 165-CONTEXT.md; chain_proof.rs; storage/chain_compatibility.rs; network/mempool_lifecycle.rs]

## Runtime State Inventory

The inspected Rust `chain_work` surface spans 96 files (including fixtures). Plan mechanical carrier work as a workspace task with compile feedback, not an isolated network change. [VERIFIED: rg -l chain_work packages --glob '\*.rs' count]

| Category                             | Items found                                                                                                                                                              | Action required                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stored data                          | u128 ChainPositionDto and HeaderEntryDto; snapshots, split chain metadata, headers/block index; synthetic fixture datadirs and interrupted replay.                       | Version/classify legacy DTOs and add a lossless wide representation needed for compile/runtime recording. Refuse ordinary unproved state. No automatic data migration; Phase 166 owns earned checkpoint/reopen acceptance. [VERIFIED: storage/snapshot_codec.rs; snapshot_codec/chain_meta.rs; chainstate/replay.rs; D-09–D-12] |
| Live service config                  | Ordinary daemon/operator network selectors and datadir routing; four explicit scope classifications. No work-specific external service setting found in inspected scope. | Preserve selectors/default-off networking and diagnostic propagation; no live external-service mutation. [VERIFIED: chain_spec.rs; canonical_admission.rs; 164-VERIFICATION.md; assigned scope]                                                                                                                                 |
| OS-registered state                  | No work-carrier OS registration found in inspected source; no OS registry inspection performed because this is internal authority migration.                             | None planned; do not rename installed aliases/service definitions. [VERIFIED: AGENTS.md architecture and phase boundary; scoped source search]                                                                                                                                                                                  |
| Secrets/env vars                     | No chain-work-specific secret/env rename identified in inspected source. Existing credentials are outside work encoding.                                                 | No credential mutation; keep refusal diagnostics redacted. [VERIFIED: scoped source search; D-09–D-11; 164-VERIFICATION.md]                                                                                                                                                                                                     |
| Build artifacts / installed packages | Cargo/Bazel binaries compiled against changed public types; installed alias is not used for UAT evidence.                                                                | Rebuild workspace/Bazel through native verification. No package reinstall or image-tag migration identified; no toolchain upgrade. [VERIFIED: AGENTS.md; packages/Cargo.toml; scripts/verify.sh]                                                                                                                                |

This inventory covers repository-visible work-carrier/runtime boundaries; it does not claim an audit of unrelated user services or datadirs. No runtime system is opened or modified during this research. [VERIFIED: research tool actions; assigned task]

### Concrete blast-radius checklist

| Surface                                                                          | Required planning action                                                                                                                                                                                                                                    |
| -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ChainPosition`, `AnchoredBlock`, engine connect/overlay/stage methods           | Exact carrier, internally derived work, restricted stages and explicit legacy fixture handling. [VERIFIED: chainstate/src/types.rs; engine.rs; engine/stage.rs]                                                                                             |
| `HeaderEntry`, `InsertedHeader`, from_entries/seed/record/insert and proof facts | Unify exact work; raw import cannot mint earned provenance; preserve scoped fixture/recovered ambiguity. [VERIFIED: network/src/header_store.rs; header_store/proof.rs]                                                                                     |
| `ManagedChainstate`, network handle/direct lifecycle methods                     | Guard accepted parent and actual preference; caller score cannot authorize branch. [VERIFIED: node/src/chainstate.rs; network/runtime_authority.rs; network/mempool_lifecycle.rs]                                                                           |
| Header validation + sync reconcile/request/runtime progress                      | Explicit powLimit/hash validity; eligible branches; lossless string projections. [VERIFIED: network/header_sync.rs; sync/block_reconcile.rs; sync/runtime_state.rs; sync/tip.rs]                                                                            |
| DTOs, chain meta, replay, Fjall snapshots                                        | Separate exact encoded numbers from earned recovery; preserve explicit legacy classifiers. [VERIFIED: storage/snapshot_codec.rs; chainstate/replay.rs]                                                                                                      |
| Bench/test harness, historical BASIC/reorg/prune fixtures                        | Replace literal scores or use explicit compatibility fixture builders; inflated scores must no longer drive live branch choice. [VERIFIED: bench/src/fixtures.rs; node/src/sync/tests/filter_index/reorg/fixtures.rs; reorg/branches.rs]                    |
| RPC/status/CLI/dashboard/support                                                 | Existing work strings avoid JSON integer truncation; adapt carrier rendering and preserve Unavailable/provenance distinctions. [VERIFIED: sync/tip.rs; sync/runtime_state.rs; rpc/src/dispatch/tests/sync_status.rs; cli/src/operator/sync_truth_render.rs] |

Do not convert every old integer fixture into an earned authority fixture mechanically. Pin-derived work and accepted-parent provenance are required for new Phase 165 authority tests; old synthetic historical fixtures retain their evidence label. [VERIFIED: D-07, D-10, D-12; 164-CONTEXT.md and historical tooling contract]

## Common Pitfalls

### 1. Normalization happens after the wrong check

**Failure:** Tiny positive mantissas produce zero target bytes; negative-zero is classified from raw sign. **Avoidance:** Normalize first, share zero/sign/overflow facts, cover boundary equivalence. **Warning:** `compact_target_bytes` returns all-zero bytes while admission succeeds. [VERIFIED: crypto.rs; pinned arith_uint256.cpp]

### 2. Parent provenance is assumed

**Failure:** Target-derived child proof is added to an inflated raw parent or forged restored position. **Avoidance:** Restricted accepted lineage and explicit untrusted import state; test raw constructors and forged stage fields. **Warning:** A decoded snapshot or record_position creates live accepted work without admission. [VERIFIED: engine.rs constructors; header_store.rs; D-07]

### 3. A best-header-only candidate loop hides valid branches

**Failure:** Header A arrived first, equal-work body B became eligible first, yet A blocks B; unavailable higher-work headers block complete side branches. **Avoidance:** Enumerate eligible retained candidates with pinned sequence policy. **Warning:** Activation still calls only `best_chain_entries()`. [VERIFIED: sync/block_reconcile.rs; pinned validation.cpp; D-06]

### 4. Guard runs after preview

**Failure:** A rejected weak/forged reorg changes live coins, tip, mempool, cache or BASIC intent before refusal. **Avoidance:** Guard the direct owner before prepare/preview and prove full unchanged-state observations. **Warning:** Preference checking occurs only in sync. [VERIFIED: mempool_lifecycle.rs; chainstate.rs; D-07]

### 5. Legacy detection itself mutates

**Failure:** Database::open/recovery/schema migration runs merely to inspect a source store. **Avoidance:** Keep filesystem-only classification; distinguish unproved observed work from explicit refusal decision. **Warning:** `open_ordinary` calls raw `open`, `initialize`, migration or replay. [VERIFIED: storage/fjall_store.rs; chain_compatibility.rs; D-09]

### 6. Compatibility assertions pin obsolete implementation spelling

Phase 68 explicitly requires `chain_work: u128`, `Some("2".to_string())` and `assert_eq!(active_tip.chain_work, 2)`. Phase 164 verifies exact source-region digests/ownership for sensitive opener/admission edges and current fixture wiring. Legitimate new guards/fields can fail these contracts before compilation. Update only the reviewed current contract and its negative tests; preserve historical artifact hashes/receipts and intentional classifications. Run these checkers early. [VERIFIED: scripts/check-phase68-active-chain-persistence.ts:105–134; scripts/check-phase164-canonical-identity.ts:42–89]

### 7. Wide number persistence is mistaken for recovery acceptance

A lossless exact DTO can be necessary to keep writes and compilation coherent in Phase 165. It does not certify stored headers/network/coins binding, canonical genesis initialization or all-handle reopen authority; those are CWOR-04/Phase 166. Classify decoded numbers explicitly and keep ordinary recovery gated. [VERIFIED: D-10, D-12; ROADMAP.md Phase 166; REQUIREMENTS.md CWOR-04]

## Code Examples

### Existing shared proof and wrapping accumulation

```rust
// Verified current API; numbers alone imply no accepted-parent provenance.
// Source: packages/open-bitcoin-consensus/src/chain_proof.rs
use open_bitcoin_consensus::chain_proof::{ProofWork, block_proof};

fn numeric_cumulative(
    parent: ProofWork,
    bits: u32,
) -> Result<ProofWork, open_bitcoin_consensus::crypto::CompactTargetError> {
    Ok(parent.wrapping_add(block_proof(bits)?))
}
```

Do not expose this arithmetic helper as a constructor for validated parent authority. [VERIFIED: chain_proof.rs docs; D-01, D-07]

### Independently expected arithmetic values

| Compact    | Expected proof hex (32-byte unsigned big-endian display)         |
| ---------- | ---------------------------------------------------------------- |
| 0x207fffff | 0000000000000000000000000000000000000000000000000000000000000002 |
| 0x1d00ffff | 0000000000000000000000000000000000000000000000000000000100010001 |
| 0x01010000 | 8000000000000000000000000000000000000000000000000000000000000000 |
| 0x01020000 | 5555555555555555555555555555555555555555555555555555555555555555 |

The existing independent-vector tests encode these expectations as limbs and the pinned formula independently specifies floor(2^256/(target+1)); summing the two 0x01010000 proofs wraps to zero. Record Phase 165 expectations as fixed corpus values with separate `source-derived-arithmetic` evidence labels, not production helper-generated expectations. [VERIFIED: consensus/src/chain_proof/tests.rs; pinned chain.cpp:131; arith_uint256.h; D-04]

Expand vectors for equivalent encodings, mantissa truncation, negative-zero, exponent 32/33/34 boundaries, selected powLimit equality/exceedance, mixed target sums, carry into bits >=128, final wrap, high-limb ordering and bytes/hex roundtrip. Mining astronomical-difficulty headers is unnecessary for the arithmetic vector class; live ordering tests use mineable bounded fixture targets and clearly identified contextual fixtures. [VERIFIED: D-03–D-04; existing chain_proof/tests.rs; header_store/proof/tests.rs]

### Reference body/ancestor eligibility algorithm

```text
checked body arrives:
  mark body checked and available
  if transaction-valid ancestry is not linked:
    append block to pending children of parent
  else:
    queue = [block]
    while queue has items:
      pop front
      establish linked transaction ancestry
      assign next eligibility sequence
      add eligible candidate
      append waiting children in their received order

choose candidate:
  greatest exact cumulative work
  then earliest earned eligibility sequence
  verify suffix data/ancestry still available and nonfailed
  keep genuinely ambiguous internal/restart ties bounded
```

This is a source-derived outline of `ReceivedBlockTransactions`, `CBlockIndexWorkComparator` and `FindMostWorkChain`; it is not an executed daemon receipt. [VERIFIED: pinned validation.cpp:3585,4205; node/blockstorage.cpp:194]

## Recommended Plan-Sized Sequence

The following sequence is prescriptive planning guidance grounded in the inspected seams; split each into focused tasks with behavioral tests and explicit file ownership. Keep mechanical migration and real authority proof distinct. [VERIFIED: D-01–D-12; sources above]

1. **165-01 — Shared arithmetic and admission validity (CWOR-01).** Extend ProofWork boundaries/rendering; normalize compact decoding; add independent vectors and selected-limit/hash tests. Reuse arithmetic; preserve modulo semantics and Result diagnostics. Cover changed pure-core branches immediately.
1. **165-02 — Exact carriers and explicit compatibility boundary (CWOR-01/03/05 foundations).** Migrate ChainPosition/HeaderEntry/AnchoredBlock and sync/DTO signatures together; add lossless wide encodings and classified legacy DTOs; adapt historical fixture builders/bench/RPC projections. Restrict imported numbers/stage construction from earning authority. Renew Phase 68 current checks with test controls. Do not open ordinary stores or claim CWOR-04.
1. **165-03 — Accepted header ancestry owns live selection (CWOR-02).** Unify header entries/proof facts and best-tip authority; accepted-parent admission only, strict greater-work best-header update, duplicate/missing/forged parent tests, shorter-higher-work live branch. Preserve canonical missing-prerequisite gates.
1. **165-04 — Body/ancestor eligibility and candidate enumeration (CWOR-02/03).** Own pending descendants and sequence at checked body linkage; enumerate eligible retained branches; separate header tie from body eligibility order; cover out-of-order bodies, duplicate body idempotence, unavailable/invalid ancestry and equal-work side-branch cases.
1. **165-05 — Actual connect/reorg work guards (CWOR-03).** Derive exact work inside engine/managed owner from earned lineage; guard direct connect/reorg before preview/mempool/coins/BASIC publication; restrict stage forging/replay; wire sync to eligible candidates. Cover maximal caller score, forged parent/position/height/hash, weak candidate, genuine stronger candidate and unchanged rejected state. Preserve explicit mechanical historical reorg fixture boundary.
1. **165-06 — Explicit ordinary legacy-work refusal (CWOR-05).** Add typed work decision to existing filesystem-only gate, bounded ordinary daemon/operator diagnostics and backend spy; closed-source manifest, schema-one migration/prune controls, forged marker/malformed/path cases across all four selectors. Renew only changed Phase 164 source regions/fixture attribution after review.
1. **165-07 — Native guard, parity/docs/UAT and closure (all four).** Add checker plus negative mutation tests, wire offline verifier, breadcrumbs/README/current parity evidence, execute default native verification and coverage, review every branch-selection and backend-opening owner. Keep D-12 nonclaims explicit and all old receipt source identities intact. Wrapper commits only after clean verification.

Recommended dependency order: 01 → 02 → 03 → 04 → 05; 06 depends on 02 and runs before 07; 07 integrates all. Prefer a green workspace at each plan boundary; future-plan symbols cannot appear as uncompilable partial conversions. [VERIFIED: assigned lifecycle contract; AGENTS.md verification rules; inspected cross-crate public types]

## Validation and Historical Verification Compatibility

`workflow.nyquist_validation` is explicitly false, so a Nyquist Validation Architecture section is intentionally omitted. Security enforcement is not explicitly disabled and is addressed below. [VERIFIED: .planning/config.json]

Default verification includes whole-workspace fmt/clippy/build/tests, CLI binary tests, benchmarks, Bazel smoke/provenance, pure-core dependency/size/panic guards and pure-core llvm coverage. The coverage helper rejects any `Uncovered Lines:` output; new pure-core code needs complete line coverage, including normalization/encoding/refusal branches. `--fast` skips full build/bench/Bazel/coverage and cannot close the phase. [VERIFIED: scripts/verify.sh:392–432; scripts/verify/helpers.sh:269–289]

Focused commands for planner tasks (choose stable keys and relevant test filters; these are planned checks, not executed research receipts):

```bash
bun run scripts/command-timings.ts run --key phase165-consensus -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features chain_proof
bun run scripts/command-timings.ts run --key phase165-headers -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features header_store
bun run scripts/command-timings.ts run --key phase165-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase165
bun run scripts/check-phase68-active-chain-persistence.ts
bun test ./scripts/check-phase164-canonical-identity.test.ts
bun run scripts/check-phase164-canonical-identity.ts
bash scripts/verify.sh
```

Command shape comes from repo guidance and current verifier/test ownership. New `phase165` tests/checkers must be registered by the implementing plans. [VERIFIED: AGENTS.md; scripts/verify.sh]

| Requirement | Mandatory evidence                                                                                                                                                                                                                                                          |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CWOR-01     | Independent fixed wide proof/carry/wrap/compact vectors; byte/hex boundaries; proof/admission consistency; selected-limit/hash failure leaves state unchanged.                                                                                                              |
| CWOR-02     | Actual live managed header admission chooses shorter-higher-work branch; strict equal-header retention; body eligibility independent of header order; orphan descendants and retained side candidates; restart ambiguity label.                                             |
| CWOR-03     | Same internally derived exact work through local/stored connect and managed direct reorg; malformed parent/stage imports cannot earn authority; inflated scores fail/no longer affect selection; before/after coins/tip/undo/mempool/BASIC/lineage/projection observations. |
| CWOR-05     | Ordinary paths refuse before backend spy and migration/prune/index initialization; full closed source manifest preservation; forged marker/schema/malformed source tests; four selectors and bounded Unavailable diagnostics.                                               |

Evidence expectations derive directly from the requirement and locked context. [VERIFIED: REQUIREMENTS.md CWOR-01/02/03/05; 165-CONTEXT.md D-04–D-11]

Historical checkers are part of the acceptance surface, not incidental cleanup. Inventory current source-spelling dependencies early with `rg`; renew reviewed current expectations and their failing controls in the same plan as carrier/edge changes. Preserve immutable historical evidence, manifests and scope labels, particularly Phase 164's region-digest contract and v2.5 synthetic BASIC/reorg histories. Do not disable checks or add broad exclusions to obtain green verification. [VERIFIED: scripts/check-phase68-active-chain-persistence.ts; scripts/check-phase164-canonical-identity.ts; .planning/STATE.md; 164-VERIFICATION.md]

## Security Domain

This phase adds no authentication/session feature; its security seam is untrusted branch/metadata input reaching active state and refusal inspection reaching backend effects. Treat ASVS as a scoped control vocabulary rather than a claim of web-application certification. OWASP currently identifies ASVS 5.0.0 as stable and recommends version-qualified identifiers; the older V2–V6 template numbering must not be presented as current 5.0 categories. \[VERIFIED: inspected phase surfaces; CITED: https://owasp.org/projects/asvs]

### Applicable ASVS Control Areas

| Area                           | Applies                 | Standard control for this phase                                                                            |
| ------------------------------ | ----------------------- | ---------------------------------------------------------------------------------------------------------- |
| Authentication                 | No new mechanism        | Preserve existing RPC/operator configuration; do not add credentials or auth exceptions.                   |
| Session management             | No new mechanism        | Preserve current runtime ownership; no token/session feature scoped.                                       |
| Access control / authorization | Yes, internal authority | Restricted provenance constructors and stage ownership; actual owner guards before publication.            |
| Input validation               | Yes                     | Shared normalized compact parser, powLimit/hash checks, accepted ancestry, bounded DTO/diagnostic parsing. |
| Cryptography                   | Existing hash/PoW path  | Existing first-party hash/proof code and pinned arithmetic; no parallel cryptographic implementation.      |

Applicability is the phase's scoped architectural assessment based on the inspected files and locked decisions. [VERIFIED: 165-CONTEXT.md; chain_proof.rs; canonical_admission.rs; chain_compatibility.rs]

| Threat pattern                                        | STRIDE                            | Mitigation / test                                                                          |
| ----------------------------------------------------- | --------------------------------- | ------------------------------------------------------------------------------------------ |
| Inflated work / forged parent / mutable connect stage | Tampering, elevation of privilege | Restricted earned lineage, internal derivation and direct owner negative tests.            |
| Header withholding changes equal-work winner          | Tampering                         | Body/ancestor linkage sequence, never header sequence.                                     |
| Unavailable or invalid side ancestry selected         | Tampering, denial of service      | Candidate eligibility plus bounded required-data refusal; preserve existing resource caps. |
| Source inspection triggers recovery/schema/prune      | Tampering                         | Filesystem-only always-refuse gate, backend spy and closed manifest.                       |
| Forged profile marker gains authority                 | Spoofing                          | Unavailable observed work plus explicit refusal decision; no profile inference.            |
| Diagnostics expose paths/payload/credentials          | Information disclosure            | Typed fixed categories and redacted ordinary adapters.                                     |
| Formula evidence claimed as daemon parity             | Repudiation / evidence integrity  | Separate source-derived labels and Phase 168 actual-process owner.                         |

Threats and controls derive from inspected boundaries and locked D-04/D-07/D-09–D-12. [VERIFIED: 165-CONTEXT.md; network/mempool_lifecycle.rs; storage/chain_compatibility.rs]

## State of the Art

| Existing approach in this checkout                     | Required Phase 165 approach                                          | Meaning                                                 |
| ------------------------------------------------------ | -------------------------------------------------------------------- | ------------------------------------------------------- |
| Count-score live headers + separate exact advisory map | Shared exact number and accepted ancestry drive live selection       | Eliminates split selector authority.                    |
| u128 caller chain work                                 | Exact ProofWork carrier with internally earned lineage               | Numeric width and trust both change.                    |
| Height/hash tie in live selection                      | Strict best-header work and separate eligibility sequence            | Distinguishes header reception from activatable bodies. |
| Unavailable observed work only                         | Unavailable observation plus explicit compatibility refusal decision | Safe classification without backend mutation.           |

These describe inspected current source and the locked destination, not external library deprecations. [VERIFIED: header_store.rs; header_store/proof.rs; chainstate/types.rs; chain_compatibility.rs; 165-CONTEXT.md]

## Assumptions Log

No training-only factual claims are used. Proposed module names and sequence are recommendations, with MEDIUM integration confidence; they are not locked new decisions or delivered evidence.

| #   | Claim               | Section | Risk if wrong |
| --- | ------------------- | ------- | ------------- |
| —   | None tagged ASSUMED | —       | —             |

All behavioral facts are local-source verified or explicitly cited; new runtime evidence must still be produced by implementation. [VERIFIED: sources listed below]

## Open Questions

The following are **RESOLVED planning decisions**, not claims of implemented behavior or successful verification. Task references use the current fourteen-plan set, which refines the earlier recommended seven-part sequence. [VERIFIED: 165-01-PLAN.md through 165-14-PLAN.md]

1. **Minimum restricted provenance API — RESOLVED.** 165-07 Task 1 chooses private earned-lineage state at the actual chainstate owner, explicitly untrusted raw snapshot/parent/cache/replay origins, private connect-stage fields with borrowed accessors, and owner-identity/source-tip/monotonic-generation binding for both stages. 165-07 Task 2 owns compile-fail, cross-owner, stale-generation and duplicate-absorption controls; 165-08 Tasks 1–2 require every managed connect/reorg/preview/commit owner to enforce these boundaries before its first effect. This resolves the authority contract without locking unnecessary constructor names: raw numeric consistency or a BASIC receipt cannot mint work provenance, and named mechanical fixtures cannot promote ordinary authority. [VERIFIED: 165-07-PLAN.md Tasks 1–2; 165-08-PLAN.md Tasks 1–2; D-07]
1. **Wide DTO compatibility granularity — RESOLVED.** 165-02 Task 1 chooses a node-owned tagged/versioned work-record boundary classifying LegacySynthetic(u128) versus ExactNumber(ProofWork), with exact fixed 32-byte LE encoding and neither variant constituting accepted lineage. Task 2 checks historical fixed bytes, wide/zero/max exact values, missing/contradictory tags and malformed lengths. 165-03 Task 2 wires this classification into chain/meta/header writes and preserves only explicitly named untrusted fixture adapters for old low-level readers. No Fjall global schema bump, automatic migration or legacy-to-authority conversion is authorized; earned checkpoint/reopen acceptance remains Phase 166. [VERIFIED: 165-02-PLAN.md Tasks 1–2; 165-03-PLAN.md Task 2; D-10/D-12]
1. **Ambiguous tie evidence — RESOLVED.** 165-04 Tasks 1–2 choose strict greater-work best-header updates and bounded/unavailable imported or restart order. 165-06 Tasks 1–2 choose checked-body/transaction-ancestor linkage, FIFO pending-child release and exactly-once earned eligibility sequence; greatest exact work then smallest earned sequence decides defined runtime activation cases, including the active tip. 165-08 Task 2 and 165-09 Task 2 own managed and actual sync arrival-order tests; 165-13 Task 1 documents ambiguous internal/restart ties without deterministic hash/pointer parity claims. Actual paired daemon/restart qualification remains Phase 168. [VERIFIED: 165-04-PLAN.md Tasks 1–2; 165-06-PLAN.md Tasks 1–2; 165-08-PLAN.md Task 2; 165-09-PLAN.md Task 2; 165-13-PLAN.md Task 1; D-08/D-12]
1. **Pinned Bun execution — RESOLVED.** Keep .bun-version=1.3.9 as the source of truth; the observed shell 1.4.2 is an environment mismatch to reconcile through the existing parent-owned preflight/tool bootstrap, not permission to change the pin or accept a different verifier runtime. The common execution constraints in the plan set require pinned Rust/Bun and assign initial tool bootstrap to the parent; 165-14 Task 2 requires the full native contract with normal runtime-pin bootstrap and actual successful receipts. Reconcile the executable selection before dependent automation, then let native verification establish the selected runtime; an unreconciled mismatch fails the execution gate rather than reopening the planning decision. No successful runtime reconciliation is asserted here. [VERIFIED: .bun-version; research probe; 165-01-PLAN.md Execution Constraints; 165-14-PLAN.md Task 2]

## Environment Availability

| Dependency           | Required by                 | Available                         | Verified version / scope                    | Fallback                                          |
| -------------------- | --------------------------- | --------------------------------- | ------------------------------------------- | ------------------------------------------------- |
| Pinned Knots source  | Formula / ordering evidence | Yes                               | Exact a9aee730... HEAD materialized         | None needed.                                      |
| Rust                 | Compile/tests               | Yes                               | rustc 1.94.1                                | None needed.                                      |
| Bun                  | Automation                  | Yes, pin mismatch in shell        | 1.4.2 shell; 1.3.9 repo pin                 | Existing bootstrap/pinned execution.              |
| Node                 | GSD tooling                 | Yes                               | v24.13.0                                    | None needed.                                      |
| Bazel command        | Smoke build                 | Yes, command located              | /opt/homebrew/bin/bazel; version not probed | Native verifier owns executable/version check.    |
| cargo-llvm-cov       | Pure-core coverage          | Yes, command located              | Version not probed                          | Native verifier prerequisite check.               |
| Source Fjall datadir | Refusal tests               | Must remain closed                | Owned generated fixtures only               | Backend spy / closed filesystem manifest.         |
| Actual Knots daemon  | Differential qualification  | Not required/probed for Phase 165 | Phase 168 owner                             | Source/formula evidence only, distinctly labeled. |

Availability follows direct command/source probes and phase scope; no expensive builds or runtime stores were opened during research. No missing tool was shown to block planning. [VERIFIED: research probes; D-04/D-09/D-12; AGENTS.md]

## Sources

### Primary (HIGH confidence)

- [Pinned arithmetic](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/arith_uint256.cpp): locally inspected SetCompact; arithmetic header for wrapping.
- [Pinned block proof](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp): locally inspected GetBlockProof and independent formula.
- [Pinned target admission](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/pow.cpp): locally inspected DeriveTarget/CheckProofOfWorkImpl.
- [Pinned block index](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp): locally inspected AddToBlockIndex and comparator; web fetch was rate-limited, local exact pin remains primary.
- [Pinned activation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp): locally inspected ReceivedBlockTransactions, TryAddBlockIndexCandidate and FindMostWorkChain.
- Local `chain_proof.rs`, tests and `crypto.rs`: existing full-width implementation, vector expectations and normalization gap.
- Local `header_store.rs/proof.rs`, `engine.rs/stage.rs/overlay_apply.rs`, `types.rs`: split header authority and raw/staged carrier/provenance seams.
- Local node `network/mempool_lifecycle.rs`, `runtime_authority.rs`, `header_sync.rs`, `chainstate.rs/validation_history.rs/replay.rs`, `sync/block_reconcile.rs/runtime_state.rs/tip.rs`: effects and integration owners.
- Local node `storage/chain_compatibility.rs`, preservation tests, `snapshot_codec.rs/chain_meta.rs`, `fjall_store.rs`: read-only gate, legacy representation and backend effects.
- Local `scripts/check-phase68-active-chain-persistence.ts`, `check-phase164-canonical-identity.ts`, `verify.sh`, `verify/helpers.sh`: historical/current contracts and coverage.
- `165-CONTEXT.md`, active requirements/roadmap/state, Phase 164 context/verification, milestone architecture/pitfalls: approved scope and inherited evidence.
- Local instructions, standards, lessons, toolchain/manifests/lock files: plan constraints and locked stack.
- [OWASP ASVS](https://owasp.org/projects/asvs): verified current version/identifier guidance; scoped security vocabulary.

### Secondary / Tertiary

None required. Library discovery/Context7 is unnecessary for this locked first-party implementation phase; local pinned primary source outranks unpinned ecosystem advice. [VERIFIED: assigned scope; D-01]

## Metadata

| Area                                    | Confidence  | Basis                                                                                                     |
| --------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------- |
| Standard stack                          | HIGH        | Existing manifests, locked versions and first-party APIs inspected.                                       |
| Arithmetic/reference semantics          | HIGH        | Exact local Knots pin plus existing independent tests.                                                    |
| Integration architecture                | MEDIUM      | Concrete bypass/owner seams verified; new restricted authority API still needs implementation validation. |
| Pitfalls / verifier compatibility       | HIGH        | Current source-spelling and digest checks directly inspected.                                             |
| Actual daemon parity / canonical reopen | Not claimed | Explicit later owners and unexecuted evidence.                                                            |

**Research date:** 2026-10-14\
**Valid until:** Source/context/pin changes invalidate affected integration advice; stable reference formulas remain tied to the exact Knots commit.\
**Completion:** Research artifact only; no source edits, tests/builds, database opens, commits or Git/config mutations performed.

## Execution source clarifications | 2026-10-14

Read-only follow-up by the same GSD researcher resolved two integration details at the pinned Knots commit. These are source-derived findings; no daemon qualification is claimed.

- Ordinary Headers handling prechecks whole-message PoW and internal continuity (`net_processing.cpp:2518–2576`) and requires the first parent to connect (`2931`), including a known genesis announcement. A contextual-valid prefix is retained when a later continuous, PoW-valid header fails contextual admission (`validation.cpp:4518/4693/4767`). Best header updates immediately only for strictly greater work (`node/blockstorage.cpp:272`). Plan 05 implements both failure classes without blanket rollback.
- `ReceivedBlockTransactions` assigns ordinary sequence after checked/stored TRANSACTIONS-valid body and transaction-linked parent ancestry (`validation.cpp:4205`; preceding `AcceptBlock:4833–4905`). Pending descendants release by FIFO breadth-first traversal, with each parent's multimap insertion order (`4233`); this is not a global body-arrival sort. Historical linkage (`chain.h:255`) survives pruning, while failed or missing-data nonactive suffixes are excluded separately (`validation.cpp:3585`). Plan 06 preserves these distinct facts before spend/script connection.
- Ordinary present-body duplicates preserve sequence. Requested pruned-body redownload can assign again; `preciousblock` uses negative sequence. Restart pointer ties, deferred prune/operator surfaces and actual daemon behavior retain their bounded scope. Knots' signed counter starts at 2 without an exhaustion check here; managed caps/refusal must be attributed to Open Bitcoin resource policy.

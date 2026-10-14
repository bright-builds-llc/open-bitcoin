---
phase: 164-canonical-chain-identity-and-compatibility-gates
reviewed: 2026-10-14T05:18:14Z
review_updated: 2026-10-14T09:33:31Z
depth: standard
review_scope: partial_core_node
diff_base: 6cf34f1b96cdb7e719752395b2c52c8f531b559c
files_reviewed: 22
files_reviewed_list:
  - packages/open-bitcoin-consensus/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/header_sync.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-consensus/src/chain_spec.rs
  - packages/open-bitcoin-consensus/src/chain_spec/genesis.rs
  - packages/open-bitcoin-consensus/src/chain_spec/tests.rs
  - packages/open-bitcoin-node/src/network/canonical_admission.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
source_snapshot_sha256: c237c7b7ca73909d279c8687fff1a2712d6b844cd954b1a8eec8a327632adbf5
initial_source_snapshot_sha256: fbf475104b939a910d34efb0a4bc35a19d20e7ab7367c8b4e53073be0bddca01
source_sha256:
  packages/open-bitcoin-consensus/src/lib.rs: 3eb75ad506bb6eef7aa0001dd2ec2ba9dea5a88722aaff0ab7e0c038fe6a3cfe
  packages/open-bitcoin-node/src/network.rs: 99ad30449677ee40a9569642bf42d52a743176061f11b41f1bdcaaa0afed116f
  packages/open-bitcoin-node/src/network/action_translation.rs: 4bb07a11761b5c84ef6903c8643a82d775988f1df0ed0d730a3b202bedeb1e62
  packages/open-bitcoin-node/src/network/header_sync.rs: 6b546e406c8ff851c2c089da4445e5f999230e7695cf80170ff6bd64df386731
  packages/open-bitcoin-node/src/network/mempool_lifecycle.rs: e71ff25d0facf5c109a87e9f04de6513e5a1a41d933203e7183332e0d4df2496
  packages/open-bitcoin-node/src/network/peer_network_clone.rs: 5073bd88917a73cb17ea66a6b2584b308d02c097a4aa658e174d2bb6b1fb1dbf
  packages/open-bitcoin-node/src/network/relay_serving.rs: 3f53591281799ba040289f9b20bac53f656861cb0352464d113f32d7c8c8916a
  packages/open-bitcoin-node/src/network/runtime_authority.rs: 3923b0e316fbd66c049d236b0843558950ebdffc968901d3622a996dcc8561e5
  packages/open-bitcoin-node/src/network/runtime_authority/tests.rs: 47ecf6975c17d50becd69a3c2d5f1d5cd55097027497aed5a77f771198ab16cb
  packages/open-bitcoin-node/src/network/validation_history.rs: 65681c65fd7e84b3acd5ebd6e8076254ecca9586cd0f827a6d1f59eab34c3b49
  packages/open-bitcoin-node/src/storage.rs: 9794e1aa3aae44819ccfd2685d992caf4c82d8d6145df58c162aedce1ec26afa
  packages/open-bitcoin-node/src/storage/fjall_store.rs: a5910d96a5fcfc136c89cb16cb57ab2c98b91d56dc0ee14524fc80f2a094e51a
  packages/open-bitcoin-node/src/sync/open_runtime.rs: 80207b6e70bdd9f1f8ccbfb2adcdd6787f560cca64e33455a0faa704efc0dfe1
  packages/open-bitcoin-consensus/src/chain_spec.rs: 80e301612d039afa72125c16a895f248a155305410348acce09376325634302e
  packages/open-bitcoin-consensus/src/chain_spec/genesis.rs: 6203adbae928028fbad72732374afd422a371180021b65045afa4273dfc2f1e9
  packages/open-bitcoin-consensus/src/chain_spec/tests.rs: 262d8d89930115c3e2ed5cd60e654225edaf4713880f63f2352a39d13e9f2bdb
  packages/open-bitcoin-node/src/network/canonical_admission.rs: 07f69af9a17270538194859fb73089088db28f96b8a5bdc1d8f4e1a8a2374b83
  packages/open-bitcoin-node/src/network/canonical_admission/tests.rs: 4977020a4421500c9c629d4787abdd421e0bce28f777f1c56a9cc2b18a461e00
  packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs: 3b0d4a20790a7d299fa7a3723e618cb65dc0e6197a7da4a4b7f0cc2b8928d69a
  packages/open-bitcoin-node/src/storage/chain_compatibility.rs: a40d4663343c9937329873461ae0721a8acd758ffc0468b0a18c2fdea539598a
  packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs: 3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630
  packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs: a127cd9cd6cecdadda5e0d998fec7ff6948615d1dbe4b35c06f82f8b5826f9cf
---

# Phase 164: Core and Node Code Review Report

**Reviewed:** 2026-10-14T05:18:14Z (initial review 2026-10-14T05:17:17Z)
**Depth:** standard
**Files Reviewed:** 22
**Status:** clean (original documentation Info closed after targeted verification)

## Scope and Summary

This is the independent partial core/node review, not the final phase aggregate. The exact assigned 22-file worktree scope includes tracked modifications and new untracked files, compared with HEAD `6cf34f1b96cdb7e719752395b2c52c8f531b559c`. Every assigned file was read in full; changed behavior and immediate constructor/dispatch/error boundaries were traced. Source hashes above identify the reviewed bytes. RPC, CLI, checker and contributor-documentation reviews remain separate parent-owned scopes.

The review used AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, core architecture/code-shape/testing/verification and Rust standards, both active lesson files, Phase 164 CONTEXT/RESEARCH and Plans/Summaries 01–03. No project skills directories or .claudeignore were present. The active lessons total 7,188 bytes and were loaded completely. Ignored build/dependency outputs were excluded from source review; explicitly requested existing verification timing receipts were consulted as evidence.

No actionable correctness, security or regression defect was found in the reviewed implementation. The original retained-API documentation Info was corrected by the parent and verified below; no findings remain open.

## Closed Finding History

### IN-01: Transient fixture constructor claimed production authority — closed

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority.rs:592`

**Issue:** `transient_runtime` is documented as creating a “transient production authority.” Its constructor calls `ManagedPeerNetwork::new_with_block_relay_activation`, which initializes `AdmissionScope::Fixture`, then wraps it with `Self::new` without canonical binding. Consequently `maybe_canonical_chain()` returns `None`, and this constructor's synthetic validation behavior is lower-level fixture/infrastructure evidence. The description conflicts with the Phase 164 D-08/Plan 03 requirement to explicitly classify retained lower-level APIs. Current call-site search found only tests; this is a documentation issue, not an observed product bypass.

**Fix:** Replace that description with explicit lower-level fixture/infrastructure scope, such as “Creates a transient lower-level fixture authority without canonical admission.” State that ordinary product construction must use the selected-chain gated boundary. Preserve behavior.

**Resolution:** The parent replaced the original line with two documentation lines describing an unbound transient authority for lower-level fixtures and requiring canonical binding plus supported validation for ordinary use. The original issue text above records the initial finding; it is not an open claim about the corrected source.

## Reviewed Behavior and Evidence

- CanonicalChain covers exactly Mainnet, Testnet3, default Signet and Regtest. The fields match the locally pinned Knots CreateGenesisBlock constructors/assertions; exact equality checks every header field and the complete transaction body, including witness. Frozen tests distinguish the four 80-byte headers, four 285-byte bodies and common 204-byte transaction, and reject all twelve foreign pairs plus mutations.
- Private ChainSpec/ValidationScope facts have no caller-settable support capability. All four ordinary-history modes refuse bootstrap, exact work and candidate-history prerequisites; Signet also reports challenge validation. Identity recognition creates no UTXO, work, BASIC or bootstrap effects.
- FjallNodeStore::open_ordinary has no successful/backend branch. Filesystem inspection checks borrowed ancestor prefixes root first, rejects symlink/nondirectory components, uses metadata calls proportional to path depth, advances directory iteration once, and reads no source-file contents. Permission/other I/O errors remain distinct from absence. Every result refuses, so concurrent path changes cannot authorize backend access.
- Compatibility diagnostics contain only fixed enums, selected identity facts and fixed guidance. All selector/observation combinations are tested below 1,024 UTF-8 bytes, including JSON serialization; candidate paths/backend error text never enter the refusal. Observed identity and work remain explicitly unavailable.
- Exact-root and ordinary support checks precede header insertion/publication, imported header assignment, local/stored-body preparation, Duplicate/NonExtending/Disconnected cache and inventory changes, and replacement preview/mempool effects. Empty header batches and empty reorg replacements also require ordinary support. Fixture seeding cannot overwrite an already bound canonical store.
- General/sync/durable message dispatch gates Headers, Block, CompactBlock and BlockTxn before peer bookkeeping, reconstruction state, observed relay evidence and timeout cleanup. ReceivedBlock actions check before compact-slot cleanup. Shared-handle connect/reorg dispatch delegates to the same owner guard. The first canonical selection survives owner clones, repeated consuming binding and fixture/canonical/shared wrappers.
- Ordinary runtime startup rejects the selected scope before initialize_configured and therefore before coins recovery, BASIC configuration/initialization, prune resume or recovered owner reconstruction, including supplied raw-store handles. Raw startup/open APIs remain explicitly lower-level mechanisms.
- Physical preservation tests drop every Fjall owner before comparing recursive test-only manifests containing names, types, lengths, exact bytes, link targets, permissions and mtime. They cover schema 1/2, foreign/synthetic roots, malformed backend/history/BASIC, interrupted coins/recovery, prune intent and unrelated/wallet/metrics sentinels. Atime is properly excluded. Separate raw-open/migration/prune controls show these fixtures would otherwise change. The backend spy is proven live before Database::builder.
- Durable admission/startup tests compare all nine namespace records; managed snapshots include chainstate, headers/best tip, peer/compact state, mempool, caches and effect evidence. These application-record tests correctly make a narrower claim than the separate closed-store physical tests.

## Validation and Limits

No Cargo/Bazel job, source edit or commit was performed during this review, as requested to avoid concurrent use of the shared target directory. `git diff --check` passed. Existing timing receipts at `.local/open-bitcoin-dev/command-timings/` independently record exitStatus 0 and outcome success for the latest canonical-genesis/spec-lib, compatibility-core/preservation/Clippy, admission-owner/headers/bodies/network-fixture-regressions/Clippy runs at the stated HEAD with a dirty worktree.

The execution summaries report 31 canonical-chain tests, 227 consensus regression tests, 24 combined compatibility tests (13 physical-preservation tests), 3 owner/5 header/5 body tests, and 523 affected network regressions passing. The timing receipts substantiate successful runs but do not independently retain per-test counts or bind those runs to this final source SHA-256 snapshot; the counts are attributed to the execution summaries. Final default `bash scripts/verify.sh`, source-stability reconciliation and final aggregate review remain parent-owned.

The temporary refusal of all four ordinary modes and absence of positive canonical reopen are approved Phase 164 behavior. Phases 165–167 remain the owners of exact work, canonical bootstrap/reopen and candidate-history consensus; this review does not reclassify fixture success as ordinary acceptance or claim actual Knots differential qualification.

## Targeted Recheck — 2026-10-14T05:18:14Z

Verified the correction at runtime_authority.rs:592–593 against the reviewed constructor behavior. Reversing only those two new documentation lines to the original one-line description reproduces the exact original reviewed file SHA-256, proving this follow-up introduced no implementation change. The other 21 assigned source hashes remain identical to the original snapshot. The corrected runtime_authority.rs hash is now recorded in frontmatter; its original hash was `4234cb0edc2f06b327325ef82b5cc3ae1c4d01d70c79be18927e9e62ece5ad6e`.

IN-01 is closed and all open counts are zero. `git diff --check` passed again. No Cargo/Bazel job, source edit or commit was performed during this recheck; the parent retains final native verification ownership.

## Native 4 Targeted Core Recheck — 2026-10-14T08:28:45Z

**Status:** clean; original 22-file scope and closed IN-01 history retained. The parent reassigned this narrow relocation/invariant recheck to the automation reviewer; the original full core review remains attributed above.

Native 4 failed the panic-sites policy after **24m49.836s (1,489,836ms)**, after its Bun suites passed. The failure at `/tmp/open-bitcoin-phase164-native-fourth.log:8571` is preserved. The lexical policy classified a cfg-test helper by its old nonconventional disk path and also matched a lint attribute and three closed genesis constructor expectations. This was a policy failure, not an observed production crash or full native acceptance.

The preservation module moved to `storage/chain_compatibility/tests/preservation.rs`. Direct comparison with `/tmp/phase164-panic-preservation-before.rs.txt` confirms all **20,556 bytes** remain identical, SHA `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`. The old `storage/chain_compatibility/preservation.rs` path is absent. No function, fixture, assertion, conditional, physical-manifest bound or byte-preservation premise changed.

The parent adds only `#[path = "chain_compatibility/tests/preservation.rs"]` between `#[cfg(test)]` and `mod preservation;`. Removing that line recreates prior parent SHA `d1f917824b611e79d3e4ddbf2c170fac146835b1455444dd9d4ab5c71fe74c0c`; final parent SHA is `a40d4663343c9937329873461ae0721a8acd758ffc0468b0a18c2fdea539598a`. The unchanged module identifier retains the existing `super` namespace. The initial shorter `tests/preservation.rs` literal failed compilation: `couldn't read open-bitcoin-node/src/storage/tests/preservation.rs`, parent line 281, recorded in `/tmp/phase164-panic-node.log`. That diagnostic remains attributed. The corrected literal resolves from `src/storage` to the actual moved file.

Current checker ownership selects the unique executable top-level declaration and its actual masked attribute span, then requires exact raw cfg-test/compiler-valid path bytes. Wrong path, disabled cfg plus comment decoy, and commented declaration each refuse. Function fingerprints retain the same exact bytes at the new path; no blanket test allowance or missing-input fallback was added.

Genesis source SHA remains `6203adbae928028fbad72732374afd422a371180021b65045afa4273dfc2f1e9`. Independently read the actual Amount/ScriptBuf constructors: private zero-argument `coinbase()` supplies only fixed constants. Eight fixed prefix bytes plus a 69-byte ASCII literal form a **77-byte** script; `OUTPUT_SCRIPT` is a fixed **67-byte** array. Both are below ScriptBuf's **10,000-byte** maximum. Fixed **5,000,000,000 satoshis** lie in Amount's inclusive **0..=2,100,000,000,000,000** range. No candidate input enters these calls. The `#[expect(clippy::expect_used, ...)]` hit is a lint attribute, not an executable panic. Four exact genesis path/needle/rationale entries are narrowly justified; the existing allowlist entry remains.

All other core source hashes remain unchanged. Original aggregate `fbf475104b939a910d34efb0a4bc35a19d20e7ab7367c8b4e53073be0bddca01` and renewed aggregate `68a7194199af4dba81c7ad5f852c47cda8f1d495e52da24fdbcb287c739e15ce` hash ordered frontmatter `file:sha256` rows joined with LF without trailing LF. Scope remains 22 files. Original review/finding history, historical receipts and ordinary refusal boundaries remain intact.

Parent/executor evidence records corrected node **24/24**, consensus **31/31**, whole guards **355/355**, panic policy, breadcrumbs (**1,160 paths**), managed checks and scoped rustfmt green. This reviewer ran no tests, Git, Cargo or native verifier and changed only review artifacts. Native 5 remains active under parent ownership, session **8391**, `/tmp/open-bitcoin-phase164-native-fifth.log`; no full native success is inferred.


## Coverage Preflight Targeted Recheck — 2026-10-14T09:33:31Z

**Status:** clean; CORE remains 22 files, with one test-file hash renewed. Original review, closed finding and native-repair histories above remain intact.

Separate coverage preflight **91777** exited **1** because `chain_spec.rs:218` was the only uncovered instrumented line: the initial diagnostic write's error-propagation branch. That preflight failure remains distinct from the five failed full-native attempts.

The sole source delta adds `canonical_chain_support_refusal_propagates_formatter_failure` to `chain_spec/tests.rs`. A local `std::fmt::Write` sink returns `Err(fmt::Error)`; `write!(&mut output, "{error}")` formats the actual `UnsupportedValidationScope` and asserts the returned formatter error. It exercises the real Display contract without I/O, timing, token copying or a fake production implementation. The test has one concern and explicit Arrange/Act/Assert. Removing only its one inserted block recreates all prior test bytes, SHA `c1eec96c79f26447b1ef2a786f6b1bca51a60eb9a3485af536f519116c8e46c7`; final test SHA is `262d8d89930115c3e2ed5cd60e654225edaf4713880f63f2352a39d13e9f2bdb`. Existing assertions, vectors, bounds and test registrations remain unchanged.

Production `chain_spec.rs` remains SHA `80e301612d039afa72125c16a895f248a155305410348acce09376325634302e`. Formatting behavior, error text, prerequisite facts, coverage packages, exclusions and thresholds were not changed. The helper adds no sensitive opener or authority constructor, so the existing **93-input/246-edge** guard corpus remains byte-identical; no new exemption or contract renewal was required.

Parent/executor serial verification **82469** exited **0**: formatting, full workspace all-target/all-feature Clippy with warnings denied, focused consensus **32 passing / zero failing**, eight-package pure-core coverage **1,706 passing / zero failing**, and whole guards **355 passing / zero failing, 421 assertions, 65.82s**. Coverage has no `Uncovered Lines:` section and no zero-count rendered rows; the formerly uncovered line **218** now has count **1**. This is **100% executed-line coverage**, not a branch-coverage or full-native claim. Logs and exact source evidence are `/tmp/phase164-coverage-{fmt,clippy,consensus,clean,fixed,guards}.log`, `/tmp/phase164-coverage-tests-before.rs.txt`, `/tmp/phase164-coverage-repair.diff` and `/tmp/phase164-coverage-hashes.json`.

All other CORE rows retain their reviewed source SHA. Previous aggregate `68a7194199af4dba81c7ad5f852c47cda8f1d495e52da24fdbcb287c739e15ce` remains attributed; renewed CORE aggregate `c237c7b7ca73909d279c8687fff1a2712d6b844cd954b1a8eec8a327632adbf5` hashes ordered frontmatter `file:sha256` rows joined with LF without trailing LF. The separately completed SHELL assertion binding remains `e6fb1285c6716328b75a55774dc460555f08a574eb58c89437150cfbad63f434`; AUTOMATION's 56 source hashes remain unchanged.

This reviewer ran no tests, Git, Cargo, full native verification or source edits. Native 6 is active under parent ownership, session **41473**, `/tmp/open-bitcoin-phase164-native-sixth.log`; its success is not inferred from direct prechecks. Source is frozen for final review/security binding and parent verification.


***
_Reviewer: gsd-code-reviewer_
_Depth: standard_

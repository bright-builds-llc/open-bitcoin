---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-25T18:13:00.349Z
---

# Phase 132: Typed Package Vocabulary and Staged Admission - Research

<user-constraints>
## User Constraints (from CONTEXT.md)

The following locked decisions, discretion areas, and deferred ideas are copied verbatim from the phase context. [VERIFIED: .planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md]

### Locked Decisions

### Package shape, identity, and ordered results

- **D-01:** Parse raw transaction vectors at the package boundary into an opaque well-formed package type. Its fallible constructor proves non-empty input, at most 25 transactions, at most 404,000 total weight units, unique txid and wtxid identities, topological order, and no internal input conflicts before expensive validation.
- **D-02:** Refine a well-formed package into a distinct child-with-unconfirmed-parents submission type. Submission-only shape rules must not be caller booleans or repeated checks deep inside admission.
- **D-03:** Compute canonical txid/wtxid member pairs once and preserve request order in private ordered storage. Keyed maps may exist only as lookup projections; they must never define response order.
- **D-04:** Return one input-index-aligned package report with a package-wide status, exactly one typed member result per request member, and explicit non-empty effective-fee groups whose membership is an ordered wtxid list. Avoid optional-field combinations that can represent impossible result states.
- **D-05:** Keep a package fingerprint or package hash separate from request order and per-member admission identity so Phase 133 can reuse package identity without changing the local result contract.

### Dry-run, partial acceptance, and staged commit

- **D-06:** Use distinct dry-run and submission command types over shared package primitives. Dry-run evaluates the complete pipeline and returns the same ordered vocabulary while leaving mempool entries, rolling fee, relay, persistence, and evidence state byte-for-byte unchanged.
- **D-07:** Preserve pinned individual-first behavior. Evaluate members in input order; retain successful singleton admissions in the prospective view; retry only eligible reconsiderable or missing-input members as the remaining subpackage; and allow a valid parent to remain finally accepted when its child fails.
- **D-08:** Implement a typed prospective overlay rather than repeatedly calling the live single-transaction mutator or cloning the entire mempool. Each accepted singleton or package group produces a checked coherent sub-delta; compose those facts into one package transition and perform one guarded live apply after final trimming.
- **D-09:** Bind a prepared transition to the exact base state it evaluated. Applying it to a changed mempool must fail before mutation, and any validation, replacement, limit, script, trim, or delta-composition failure must discard the overlay and rolling-fee changes.
- **D-10:** Keep attempt vocabulary in ordered package/member results and committed facts in `MempoolLifecycleDelta`. Witness aliases and failed candidates never appear as admitted or removed lifecycle members.

### Effective fee and final policy boundaries

- **D-11:** Preserve Phase 130 fee-role separation. Every ordinary member must satisfy the static relay floor independently; an eligible non-empty package aggregate may satisfy the active rolling floor. Incremental relay fee remains only a replacement/pressure input.
- **D-12:** Implement the unchanged PACK-06/PACK-07 surface rather than silently narrowing it. Phase 132 must cover the pinned limited package-RBF, TRUC inheritance/topology and explicit enforced-TRUC fee exception, ephemeral-dust spend checks, same-txid/different-witness handling, and reconsiderable-failure classification needed by the selected local package modes.
- **D-13:** Follow one explicit policy order: context-free shape checks; exact-mempool/witness-alias/new-candidate classification; individual evaluation; residual reconsiderable grouping; ordinary static-floor checks; TRUC checks; aggregate rolling-floor assessment; ancestor/descendant limits and limited replacement; ephemeral-dust checks; scripts; coherent staged commit; one Phase 131 pressure trim; then final-membership result rewriting.
- **D-14:** Model `SameTxidDifferentWitness` with the existing wtxid explicitly, keep aliases out of effective-fee groups and lifecycle deltas, and distinguish reconsiderable failures from hard rejects without adding peer-origin state in this phase.
- **D-15:** Rewrite every initially successful member result from authoritative post-trim membership. A member removed by replacement or final pressure cannot remain reported as accepted merely because earlier preparation succeeded.
- **D-16:** Package admission itself does not enqueue relay, write persistence, mutate serving/compact/orphan/retry caches, or publish operator evidence. It emits the typed results and semantic lifecycle facts later phases consume.

### the agent's Discretion

- Exact Rust names and module split, provided the opaque refinements, ordered-report invariant, mode separation, and prospective-apply guard remain explicit.
- The internal overlay representation and base-state token/version, provided it avoids whole-mempool cloning on the normal path and has a recomputation oracle in tests.
- Exact enum granularity for package-wide and per-member failure reasons, provided hard, reconsiderable, witness-alias, already-present, finally-present, and post-trim-absent states cannot be confused.
- Whether dry-run and submission share private helper functions or sealed internal stages, provided their public command types make invalid capability combinations unrepresentable.

### Deferred Ideas (OUT OF SCOPE)

- Peer-originated reconsiderable caching and bounded same-peer 1P1C assembly — Phase 133.
- Applying package lifecycle facts to serving, relay, compact reconstruction, orphan/reject, retry, persistence, and evidence caches — Phase 134.
- Snapshot schema, checkpointing, and recovery — Phase 135.
- Receive-independent maintenance, fanout, and transport receipts — Phase 136.
- RPC/CLI/dashboard/status methods and sanitized operator evidence — Phase 137.
- Cross-phase adversarial pressure/restart/release proof — Phase 138.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| PACK-01 | Node validates package shape before expensive work, including non-empty input, the pinned 25-transaction and 404,000-weight limits, unique identities, topological order, and no internal input conflicts. | Opaque `WellFormedPackage`, one-pass identity/weight/index construction, exact Knots shape anchors, and boundary test matrix below. [VERIFIED: .planning/REQUIREMENTS.md:29; packages/bitcoin-knots/src/policy/packages.cpp:17-117] |
| PACK-02 | Operator can dry-run package admission and receive ordered per-transaction results without mutating mempool, relay, persistence, or evidence state. | Distinct `DryRunPackage` capability, common prospective engine, no live apply, exact-state snapshot tests, and adapter exclusion below. [VERIFIED: .planning/REQUIREMENTS.md:30; 132-CONTEXT.md D-06/D-16] |
| PACK-03 | Operator can submit a child-with-unconfirmed-parents package and receive package-wide status plus ordered final per-transaction outcomes and effective-fee membership. | `SubmissionPackage` refinement, input-aligned report, explicit non-empty fee groups, and post-trim rewrite below. [VERIFIED: .planning/REQUIREMENTS.md:31; packages/bitcoin-knots/src/validation.cpp:1960-2021,2104-2145] |
| PACK-04 | Package admission preserves pinned individual-first partial-acceptance behavior instead of treating the entire call as globally atomic. | Ordered singleton pass, residual retry classification, prospective retention, and hard-failure behavior below. [VERIFIED: .planning/REQUIREMENTS.md:32; packages/bitcoin-knots/src/validation.cpp:2031-2096] |
| PACK-05 | Each accepted subpackage is staged and committed through one coherent mempool delta, with no partial mutation when validation, replacement, limits, or commit preparation fails. | Revision-bound owned sparse `MempoolPatch`, checked delta composition, revision-first guarded apply, and stale-base tests below. [VERIFIED: .planning/REQUIREMENTS.md:33; packages/bitcoin-knots/src/txmempool.h:824-918] |
| PACK-06 | Package fee evaluation applies the pinned effective-fee grouping rules while preserving the static relay floor and evaluating the active rolling floor correctly. | Static/member versus rolling/group decision table and typed fee-group vocabulary below. [VERIFIED: .planning/REQUIREMENTS.md:34; packages/open-bitcoin-mempool/src/fee.rs:71-181; packages/bitcoin-knots/src/validation.cpp:1097-1112,1831-1856] |
| PACK-07 | Package outcomes reflect final post-trim membership and match the pinned replacement, TRUC, ephemeral-dust, same-txid/different-witness, and reconsiderable-failure boundaries selected for the scoped surface. | Exact replacement/TRUC/dust/witness anchors, final rewrite algorithm, and parity test matrix below. [VERIFIED: .planning/REQUIREMENTS.md:35; packages/bitcoin-knots/src/validation.cpp:1320-1423,1960-2145] |
</phase-requirements>

## Summary

Phase 132 should be planned as a typed refinement and transition-engine phase, not as a package wrapper around `Mempool::commit_transaction_with_context`. The current single-transaction path computes identities late, collapses static and rolling floors into one effective floor, clones the entire entry map, fully recomputes topology/resource state, trims, then assigns live fields. Calling it once per package member would violate the locked no-full-clone/no-repeated-live-mutation decisions and would make the individual-first semantics hard to stage coherently. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209]

The implementation should first extract a shared, non-mutating **pre-script** candidate preparation path over a read-only mempool view and a separate contextual script checker, then introduce a revision-bound sparse patch. Package evaluation can retain singleton successes in that overlay, retry only reconsiderable/missing-input members as a residual group, run static/TRUC/rolling/limits/limited-replacement/ephemeral policy, invoke scripts only at the explicit late script stage, trim once, rewrite results from the final prospective view, and either discard the transition for dry-run or apply it once for submission. The legacy single-admission adapter must call the same pre-script preparer and late script checker in its existing single-transaction order. The existing lifecycle builder, topology helpers, accounted-memory primitives, rolling-fee state, and full `recompute_state` are reusable, but `recompute_state` is a test oracle only: normal preparation and apply operate solely on checked sparse facts. [VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs:175-337; packages/open-bitcoin-mempool/src/resource.rs:149-319; packages/open-bitcoin-mempool/src/pool.rs:430-525; packages/bitcoin-knots/src/validation.cpp:1772-1889]

PACK-07 also requires foundations absent from the current `PolicyConfig`: the three-state Knots TRUC policy, an independent dust relay fee plus scoped ephemeral permissions, explicit pay-to-anchor classification, and the complete limited package-RBF checks. Current Open Bitcoin rejects every dust output during standardness, classifies the Knots `OP_1 0x02 0x4e 0x73` pay-to-anchor script as an unknown witness program, has no TRUC vocabulary, and lacks the 100-candidate/feerate-diagram package-replacement rules. These are plan prerequisites, not optional polish. [VERIFIED: packages/open-bitcoin-mempool/src/types.rs:34-84; packages/open-bitcoin-mempool/src/policy/output.rs:17-89; packages/open-bitcoin-consensus/src/classify.rs:37-77,250-269; packages/open-bitcoin-mempool/src/pool.rs:164-247; packages/bitcoin-knots/src/script/script.cpp:208-223; packages/bitcoin-knots/src/policy/rbf.h:25-27,121-126]

**Primary recommendation:** Build one pure `open-bitcoin-mempool` prospective admission engine around opaque package refinements and a revision-bound sparse transition; migrate legacy single admission onto the same prepare/apply seam before adding ordered package orchestration and the PACK-07 policy modules. [VERIFIED: 132-CONTEXT.md D-01 through D-16; AGENTS.md Project/Architecture constraints]

## Project Constraints (from AGENTS.md)

- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior on in-scope surfaces, and keep intentional differences auditable in `docs/parity/`. [VERIFIED: AGENTS.md Project/Conventions sections]
- Keep pure domain behavior in functional-core crates; filesystem, process, network, terminal, RPC, service-manager, persistence, and evidence effects belong in shell adapters. [VERIFIED: AGENTS.md Project/Architecture/Conventions sections; standards/core/architecture.md]
- Do not add an existing Rust Bitcoin library to the production path; the project owns its domain model. Minimize dependencies. [VERIFIED: AGENTS.md Project constraints; AGENTS.bright-builds.md]
- Use Rust `1.94.1`, Rust 2024, `foo.rs` plus `foo/`, typed newtypes/enums, `let...else`, no `unwrap()`, and `maybe_` prefixes for optional values. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; standards/languages/rust.md]
- Keep unit tests behavior-oriented, one concern each, and Arrange/Act/Assert. [VERIFIED: AGENTS.md Testing; standards/core/testing.md]
- Add every new first-party Rust source/test file to `docs/parity/source-breadcrumbs.json` with defensible Knots anchors; use explicit `none` only when no source anchor exists. [VERIFIED: AGENTS.md Repo-Local Guidance; scripts/check-parity-breadcrumbs.ts:1-23]
- Use `bash scripts/verify.sh` as the completion contract. Route ad hoc Cargo/Bazel commands through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` and do not overlap Cargo jobs against one target directory. [VERIFIED: AGENTS.md Repo-Local Guidance; standards/core/verification.md]
- Use standalone `---` only as the top YAML-frontmatter delimiters in parsed Markdown; body separation uses headings or `***`. [VERIFIED: AGENTS.md Frontmatter-Parsed Markdown]
- Update the relevant README/parity catalog after substantial parity or operator-surface changes, and treat generated `docs/metrics/lines-of-code.md` freshness changes as intentional. [VERIFIED: AGENTS.md Repo-Local Guidance]

## Standard Stack

### Core

| Component | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| Rust toolchain | `1.94.1` | Typed package vocabulary, pure policy, transition engine, tests | Repo-pinned source of truth; installed `rustc` and Cargo both report `1.94.1`. [VERIFIED: rust-toolchain.toml; local `rustc --version`; local `cargo --version`] |
| Rust edition | 2024 | First-party crate language edition | Workspace and Bazel mempool target both pin edition 2024. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-mempool/BUILD.bazel] |
| `open-bitcoin-mempool` | workspace `0.1.0` | Package types, admission, policy, lifecycle facts | Existing pure mempool authority and the phase’s owned implementation surface. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-mempool/Cargo.toml; AGENTS.md Architecture] |
| `open-bitcoin-primitives` | workspace `0.1.0` | `Transaction`, `Txid`, `Wtxid`, `OutPoint` | Existing project-owned identities and transaction model. [VERIFIED: packages/open-bitcoin-mempool/Cargo.toml; packages/open-bitcoin-mempool/src/types.rs:1-12] |
| `open-bitcoin-consensus` | workspace `0.1.0` | txid/wtxid, weight, script/consensus validation, first-party SHA-256 | Already supplies canonical transaction identities and `Sha256::digest`; no new crypto dependency is needed for package fingerprints. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs:23-76; packages/open-bitcoin-mempool/Cargo.toml] |
| `open-bitcoin-chainstate` | workspace `0.1.0` | Immutable chainstate snapshot/input lookup | Already supplies contextual input facts for admission and child-with-unconfirmed-parents refinement. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-105,265-285; packages/open-bitcoin-mempool/Cargo.toml] |

### Supporting

| Component | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| Bun | `1.3.9` installed | Repo-owned TypeScript checks and timed-command wrapper | Breadcrumb checks, verifier, and ad hoc Cargo/Bazel timing. [VERIFIED: local `bun --version`; AGENTS.md Repo-Local Guidance] |
| Bazelisk/Bazel | Bazel `8.6.0` installed | Root smoke build | Use through repo verification; the mempool Bazel target is a `rust_library` over `src/**/*.rs`. [VERIFIED: local `bazelisk --version`; packages/open-bitcoin-mempool/BUILD.bazel] |
| Pinned Bitcoin Knots submodule | `v29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878` | Behavioral oracle and breadcrumb source | Use for exact shape, admission, replacement, TRUC, dust, and result parity. [VERIFIED: local `git -C packages/bitcoin-knots describe`; local `git -C packages/bitcoin-knots rev-parse HEAD`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Private checked non-empty vector wrapper | Add a `nonempty` crate | Adds a dependency for a 25-member bounded invariant already naturally enforced by the opaque constructor; do not add it. [VERIFIED: 132-CONTEXT.md D-01; AGENTS.md Dependencies Philosophy] |
| First-party `Sha256` and existing IDs | Add `bitcoin`, `bitcoin_hashes`, or another crypto crate | Violates the production-path dependency constraint and duplicates existing functionality; do not add it. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs:23-76; AGENTS.md Project constraints] |
| Sparse overlay plus test-only recomputation | Clone/recompute the entire mempool per member | Simpler initially but directly violates D-08 and scales work by package size; keep it only as an oracle. [VERIFIED: 132-CONTEXT.md D-08; packages/open-bitcoin-mempool/src/pool/admission.rs:138-161] |
| Ordered result vector plus lookup projection | `HashMap`/`BTreeMap` as the report | Loses request order or makes ordering an incidental key order; prohibited by D-03/D-04. [VERIFIED: 132-CONTEXT.md D-03/D-04] |

**Installation:** No dependency installation is required or recommended. The phase should remain within the existing workspace dependency graph. [VERIFIED: packages/open-bitcoin-mempool/Cargo.toml; AGENTS.md dependency constraints]

**Version verification:** This is a Rust workspace with no `package.json`; npm version checks are not applicable. Tool and workspace versions above were verified from the pinned files and installed commands on 2026-07-25. [VERIFIED: packages/Cargo.toml; rust-toolchain.toml; local version probes]

## Current Integration Points and Gaps

| Seam | Reuse | Required change |
| --- | --- | --- |
| `pool/admission.rs` | Existing validation, replacement, lifecycle, trim, and final assignment sequence | Extract non-mutating preparation over a view and one guarded apply. The current path clones `self.entries` and recomputes the whole state. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209] |
| `fee.rs` / candidate preparation | Correct role types and a starter `PackageFeeFloorAssessment`; current preparation exposes one consensus-derived fee | Generalize from one member boolean to a non-empty ordered group, and carry typed base/modified fee facts through policy. Production values remain equal until a separately owned prioritization surface exists, but pure policy tests must cover unequal values because ephemeral dust requires both to be zero. [VERIFIED: packages/open-bitcoin-mempool/src/fee.rs:71-181; packages/open-bitcoin-mempool/src/pool/admission.rs:103-116; packages/bitcoin-knots/src/policy/ephemeral_policy.cpp:23-30] |
| `outcome.rs` | Existing attempt-versus-fact separation and stable single labels | Add package-specific sum types rather than optional fields; current duplicate lacks wtxid and current rejection categories do not distinguish reconsiderable versus hard. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs:16-115] |
| `pool/lifecycle.rs` | Checked txid/wtxid identity pairs, admitted insertion order, deterministic removals/final membership | Add a checked “record/merge delta facts” seam if needed; do not put the large package engine in this already-large module. [VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs:175-337; local line count] |
| `pool/topology.rs` and `recompute_state` | Ancestor/descendant/conflict helpers and independent full-state oracle | Introduce overlay-aware view operations and touched-entry stat updates; retain full recomputation for oracle comparisons. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs:430-525; packages/open-bitcoin-mempool/src/pool/topology.rs] |
| `resource.rs` | Deterministic accounting and independent recompute oracle | Add checked subtract/replace primitives so a sparse patch can account for removals and changed parent/child sets without rebuilding the whole ledger. [VERIFIED: packages/open-bitcoin-mempool/src/resource.rs:149-319] |
| `pool/pressure.rs` | Correct Phase-131 victim selection, rolling bump, and removal facts | Separate selection/application from owned `MempoolState` so the package overlay can trim once without first materializing a full clone. [VERIFIED: packages/open-bitcoin-mempool/src/pool/pressure.rs:26-73] |
| `types.rs` / `policy/output.rs` | Existing RBF, standardness, and fixed dust thresholds | Add `TrucPolicy::{Reject, Accept, Enforce}`, `DustRelayFeeRate`, scoped ephemeral permissions, and output classification that permits the selected ephemeral form before package spend checks. [VERIFIED: packages/open-bitcoin-mempool/src/types.rs:34-84; packages/open-bitcoin-mempool/src/policy/output.rs:17-89; packages/bitcoin-knots/src/kernel/mempool_options.h:20-21,64-96] |
| `open-bitcoin-consensus/src/classify.rs` and script policy | Existing witness-program parsing and unknown-version consensus handling | Add a distinct `PayToAnchor` classification for witness version 1 program `0x4e73`, keep its consensus treatment equivalent to an upgradable witness program, and reject non-empty anchor witness at standardness. [VERIFIED: packages/open-bitcoin-consensus/src/classify.rs:37-77,250-269; packages/open-bitcoin-consensus/src/script/witness.rs:256-267; packages/bitcoin-knots/src/script/script.cpp:208-223; packages/bitcoin-knots/src/policy/policy.cpp:351-371] |
| `docs/parity/source-breadcrumbs.json` | Existing `mempool-policy`, `mempool-entry-context`, and `mempool-lifecycle` groups | Register every new package/policy/test file. Extend `mempool-policy` breadcrumbs with `validation.cpp`, `validation.h`, `truc_policy.*`, `ephemeral_policy.*`, and relevant tests. [VERIFIED: docs/parity/source-breadcrumbs.json:583-666] |

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-mempool/src/
├── package.rs                    # Public opaque package/report/command vocabulary
├── package/
│   ├── shape.rs                 # WellFormedPackage + SubmissionPackage refinements
│   ├── report.rs                # Ordered, impossible-state-free outcomes and fee groups
│   └── fingerprint.rs           # Order-independent Knots-compatible package hash
├── policy.rs
├── policy/
│   ├── output.rs                # Dust relay rate + standard output classification
│   ├── ephemeral.rs             # Zero-fee dust and all-parent-dust spend checks
│   ├── replacement.rs           # Limited package-RBF and diagram checks
│   └── truc.rs                  # TRUC inheritance/topology/size rules
├── pool.rs
├── pool/
│   ├── admission.rs             # Legacy public single-admission adapters
│   ├── candidate.rs             # Shared non-mutating candidate preparation
│   ├── prospective.rs           # Overlay view, patch, revision, oracle seam
│   ├── package_admission.rs     # D-13 orchestration and one apply
│   ├── pressure.rs              # Overlay-aware final trim using Phase-131 selection
│   └── tests/
│       ├── package_shape_cases.rs
│       ├── package_admission_cases.rs
│       ├── package_policy_cases.rs
│       └── prospective_oracle_cases.rs
└── ...
```

This split follows the repo’s `foo.rs` plus `foo/` convention, keeps public vocabulary separate from pool internals, and avoids further growth in `pool.rs`/`pool/lifecycle.rs`. [VERIFIED: standards/languages/rust.md; local module line counts]

### Pattern 1: Opaque Refinement Pipeline

**What:** Convert caller-owned `Vec<Transaction>` into `WellFormedPackage`, then refine it into a mode-specific command input. No admission stage should accept raw vectors plus booleans. [VERIFIED: 132-CONTEXT.md D-01/D-02/D-06]

**Recommended invariants:**

1. `WellFormedPackage::try_from(Vec<Transaction>)` rejects empty, count over 25, total weight over 404,000, serialization/identity failure, duplicate txid, duplicate wtxid, child-before-parent ordering, zero-input members, and cross-member input conflicts. Within-one-transaction duplicate inputs remain a consensus-validation error, matching Knots’ batch insertion rationale. [VERIFIED: 132-CONTEXT.md D-01; packages/bitcoin-knots/src/policy/packages.cpp:52-117]
2. Each private `PackageMember` owns `transaction`, `MempoolMemberIdentity`, `weight`, and original input index. The constructor computes txid/wtxid exactly once. [VERIFIED: 132-CONTEXT.md D-03; packages/open-bitcoin-consensus/src/crypto.rs:60-76]
3. `SubmissionPackage` is an enum with `Single(WellFormedPackageMember)` and `ChildWithUnconfirmedParents(ChildWithParentsPackage)`. For the multi-member case, every preceding member must be a direct parent of the final child, and each child input not backed by a package parent must exist in the supplied immutable chainstate snapshot. Do not globally require `IsChildWithParentsTree`; parent independence is a separate topology predicate and TRUC enforcement supplies its own stricter rules. [VERIFIED: packages/bitcoin-knots/src/policy/packages.cpp:119-149; packages/bitcoin-knots/src/validation.cpp:1967-2021]
4. Keep contextual chainstate lookup in the submission refinement constructor, before prospective mempool classification. This proves the “all unconfirmed parents supplied” property without smuggling a mutable mempool dependency into the type. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1988-2021; 132-CONTEXT.md D-02/D-13]

**Package fingerprint:** Match `GetPackageHash`: copy wtxids, sort by conventional displayed-hex lexicographic order (implemented over Open Bitcoin’s raw digest bytes by comparing reversed byte iterators), concatenate the canonical 32-byte wtxid encodings in that order, and apply single SHA-256. Do not sort by the hash wrapper’s derived raw-byte `Ord` or numeric `uint256` order; the pinned fixed vectors distinguish those orders. This value is permutation-independent and must not reorder members or reports. [VERIFIED: packages/open-bitcoin-primitives/src/hash.rs:50-125; packages/bitcoin-knots/src/policy/packages.cpp:151-169; packages/bitcoin-knots/src/test/txpackage_tests.cpp:75-130]

### Pattern 2: Ordered Report as Sum Types

**What:** Model final prospective truth, not intermediate call history, in one input-aligned vector. A practical shape is:

```rust
pub struct PackageReport {
    pub fingerprint: PackageFingerprint,
    pub status: PackageStatus,
    pub members: Vec<PackageMemberResult>,
    pub effective_fee_groups: Vec<EffectiveFeeGroup>,
}

pub enum PackageMemberResult {
    FinallyPresent(NewlyPresent),
    AlreadyPresent(ExistingMember),
    SameTxidDifferentWitness(WitnessAlias),
    HardRejected(HardMemberFailure),
    Reconsiderable(ReconsiderableMemberFailure),
    PostTrimAbsent(PostTrimAbsence),
}
```

`PackageMemberResult` must carry its requested identity in every variant, and accepted/reconsiderable fee-assessed variants must carry a required `EffectiveFeeGroupId`, not an optional fee rate/list pair. `EffectiveFeeGroup` should have a private non-empty ordered wtxid vector plus checked total fee/vsize/rate. `PostTrimAbsence` should retain a typed prior origin (`NewlyPresent`, `AlreadyPresent`, or `SameTxidDifferentWitness`) so final rewriting is explicit. [VERIFIED: 132-CONTEXT.md D-04/D-14/D-15; packages/bitcoin-knots/src/validation.h:111-233]

Dry-run and submission should return the same `PackageReport` semantics: “finally present” means present in the evaluated post-trim prospective state. The command type tells the caller whether that state was forecast and discarded or guardedly committed. [VERIFIED: 132-CONTEXT.md D-06]

### Pattern 3: Revision-Bound Sparse Prospective View

**What:** Add a private monotonic `MempoolRevision(u64)` and prepare changes in a `ProspectiveMempool<'base>` that borrows immutable base maps and owns only touched additions, removals, updated cached entries, spent-outpoint edits, resource deltas, prospective rolling state, and lifecycle facts. Lookup is overlay-first, then base unless removed. [VERIFIED: 132-CONTEXT.md D-08/D-09; packages/bitcoin-knots/src/txmempool.h:824-918]

The revision must cover every state component used by evaluation: entries, spent-outpoint index, cached resource/topology facts, and all rolling-fee fields. Increment it after membership-changing single/package admission, block/expiry/reorg changes, pressure changes, rolling test injection, block decay-gate changes, and materialized decay whenever the state actually changes; an all-already-present no-op need not advance it. Current mutation sites are distributed across `pool.rs`, `pool/admission.rs`, `pool/expiry.rs`, and `pool/lifecycle.rs`; centralize them behind checked apply helpers so a path cannot forget the revision. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs:58-130; packages/open-bitcoin-mempool/src/pool/admission.rs:197-200; packages/open-bitcoin-mempool/src/pool/expiry.rs:35-64; packages/open-bitcoin-mempool/src/pool/lifecycle.rs:440-526]

The prepared transition is the owned sparse `MempoolPatch` itself. It contains `base_revision`, `next_revision`, and only touched entry upserts/removals, spent-index edits, topology/aggregate replacements, checked accounted-resource deltas, final rolling facts, and the checked lifecycle delta; there is no wrapper containing a full `MempoolState`. `apply_prepared(&mut Mempool, patch)` first compares revisions and returns a typed stale-base error before mutation, then applies only the patch and assigns `next_revision` last. After the revision check, apply must contain no policy, whole-state materialization, `recompute_state`, allocation-heavy recomputation, or fallible delta composition; all fallible work belongs in preparation. [VERIFIED: 132-CONTEXT.md D-08/D-09; packages/bitcoin-knots/src/txmempool.h:824-918]

### Pattern 4: Individual-First State Machine

The orchestration should be an explicit state machine, in locked D-13 order:

1. Construct/refine the package outside mempool mutation. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1973-2021]
2. At one base revision, classify each member in request order as exact mempool entry, same-txid/different-witness alias, or new candidate. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2034-2064]
3. Evaluate each new member alone against the evolving prospective view through pre-script preparation, then invoke the separate script checker in legacy single-admission order before retaining singleton success and assigning its singleton fee group. `prepare_candidate` itself never invokes scripts. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2065-2072,1508-1514]
4. Keep only missing-input or typed reconsiderable failures for residual evaluation. A hard failure sets package-wide failure/skip-residual but does not stop singleton evaluation of later request members. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2073-2096]
5. Evaluate the non-empty residual group with ordinary per-member static checks, pre-replacement TRUC using direct conflicts plus eligible sibling-eviction intent, aggregate rolling, topology/limits, limited replacement, and ephemeral dust; only then invoke the separate script checker for each member. [VERIFIED: 132-CONTEXT.md D-11 through D-13; packages/bitcoin-knots/src/validation.cpp:1772-1889]
6. Compose each successful singleton/group patch into one prospective package transition. A failed residual group does not erase prior singleton successes. [VERIFIED: 132-CONTEXT.md D-07 through D-10]
7. Run Phase-131 pressure trim exactly once over the final prospective view and rolling clone. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2099-2106; packages/open-bitcoin-mempool/src/pool/pressure.rs:26-73]
8. Rewrite every prior-success/alias/existing outcome from authoritative final prospective membership; build the final lifecycle delta only from actual newly admitted and removed members. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2108-2145; 132-CONTEXT.md D-10/D-15]
9. Dry-run discards the prepared transition; submit performs one guarded apply. [VERIFIED: 132-CONTEXT.md D-06/D-09]

### Pattern 5: Fee Roles and Effective Groups

| Case | Static relay floor | Active rolling floor | Effective group |
| --- | --- | --- | --- |
| Ordinary singleton | Must pass independently | Singleton must pass | Exactly its requested wtxid. [VERIFIED: 132-CONTEXT.md D-11; packages/open-bitcoin-mempool/src/fee.rs:152-181] |
| Ordinary residual group | Every member must pass independently | Aggregate fee/vsize may pass | Exactly the new residual members in request order; excludes exact existing and witness aliases. [VERIFIED: packages/bitcoin-knots/doc/policy/packages.md:100-159; packages/bitcoin-knots/src/validation.cpp:1831-1856,2040-2063] |
| TRUC version 3 under `Enforce` | Explicitly exempt | Aggregate/singleton rolling still applies | Same non-empty grouping rules. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1097-1112; packages/bitcoin-knots/test/functional/mempool_truc.py:640-684] |
| TRUC version 3 under `Accept` | Must pass normally | Normal rules | `Accept` does not enforce TRUC rules and does not grant the static exception. [VERIFIED: packages/bitcoin-knots/src/kernel/mempool_options.h:20-36; packages/bitcoin-knots/src/validation.cpp:843-850,1097-1107,1208-1209] |
| TRUC version 3 under `Reject` | Not reached | Not reached | Hard non-standard version rejection. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:843-845] |
| Existing exact member / witness alias | Not re-evaluated | Not counted | No effective-fee group membership. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2040-2063] |

The static exception can be selected from transaction version plus `TrucPolicy::Enforce` before the deeper TRUC topology stage; failure of the later TRUC checks remains a hard policy outcome. This preserves locked D-13 ordering without accidentally granting the exception under `Accept`. [VERIFIED: 132-CONTEXT.md D-13; packages/bitcoin-knots/src/validation.cpp:1097-1107,1208-1209]

Carry `CandidateFees { base, modified }` (exact name discretionary) through preparation, effective-group arithmetic, replacement, reports, and ephemeral policy. Open Bitcoin currently has no fee-delta/prioritization state, so production preparation initializes both from the consensus-derived fee; Phase 132 should not add an operator prioritization or persistence surface. Keeping the pair distinct nevertheless prevents the policy API from silently encoding `base == modified` and permits direct parity tests of the pinned dust rule. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:103-116; packages/bitcoin-knots/src/validation.h:147-164; packages/bitcoin-knots/src/validation.cpp:1078-1086; 132-CONTEXT.md D-16]

### Pattern 6: Scoped Advanced Policies

**Limited package RBF:** Only a two-member 1-parent-1-child residual package is eligible; neither new member may have an in-mempool ancestor. Before allocating the descendant union or diagram, iterate direct conflicts in deterministic order and checked-add each conflict entry's descendant count, deliberately counting shared descendants once per direct conflict exactly like `GetEntriesForConflicts`; fail immediately when the conservative running sum exceeds `MAX_REPLACEMENT_CANDIDATES=100`. Only after that guard may the implementation construct the deterministic union of direct conflicts and descendants. Then require replacement fees to exceed originals plus the incremental-relay fee for the package vsize, package feerate strictly above parent feerate, and a strictly improved feerate diagram. Stage the resulting union as removals in the same patch before post-removal topology/limit revalidation. [VERIFIED: packages/bitcoin-knots/src/policy/rbf.cpp:60-86; packages/bitcoin-knots/src/validation.cpp:1320-1423; packages/bitcoin-knots/src/policy/rbf.h:25-27,98-126]

**TRUC:** Represent `Reject`, `Accept`, and `Enforce`, with pinned default `Accept`. Under `Enforce`, version 3 means TRUC; enforce 10,000-vB maximum, two-member ancestor/descendant limits, 1,000-vB child maximum, version inheritance in both directions, no sibling/parent-and-child topology, and replacement-aware descendant handling. Because D-13 places TRUC before limited replacement, the evaluator receives the candidate's direct conflicts plus an explicit eligible sibling-eviction intent derived from the unmodified prospective graph. It evaluates child replacement and eligible sibling eviction hypothetically without assuming any removal has been staged; actual limited-RBF removal staging remains later. Do not bring peer-origin behavior into this module. [VERIFIED: packages/bitcoin-knots/src/kernel/mempool_options.h:20-36; packages/bitcoin-knots/src/policy/truc_policy.h:38-64; packages/bitcoin-knots/src/policy/truc_policy.cpp:202-262; packages/bitcoin-knots/src/validation.cpp:1208-1224]

**Ephemeral dust:** Add a `DustRelayFeeRate` role (pinned baseline 3,000 sat/kvB) rather than continuing fixed 330/546 constants as the policy source. Add explicit pay-to-anchor classification for witness version 1 program `0x4e73`; preserve its upgradable-witness consensus behavior but reject witness stuffing as non-standard. Apply the three permissions as exact independent predicates: `anchor` gates P2A outputs, `send` gates dusty non-anchor outputs, and `dust` gates nonzero-valued dust. Defaults are `anchor=true`, `send=false`, `dust=false`; `dust` does not permit non-anchor forms, and `send` is not a vague future seam. A transaction containing any permitted dust must have both base and modified fee equal to zero; then every child spending any in-package or in-mempool parent with dust must spend all of that parent’s dust outputs. The spend check runs after topology/limited replacement and before the separate script stage. [VERIFIED: packages/bitcoin-knots/src/policy/policy.h:70-76,96-106; packages/bitcoin-knots/src/script/script.cpp:208-223; packages/bitcoin-knots/src/policy/policy.cpp:185-199,351-371; packages/bitcoin-knots/src/policy/ephemeral_policy.cpp:23-94; packages/bitcoin-knots/src/validation.cpp:1864-1889]

The current Open Bitcoin output validator unconditionally rejects below-threshold outputs, so implementing only `CheckEphemeralSpends` would produce dead code. Refactor standardness and add policy configuration in the same plan wave. [VERIFIED: packages/open-bitcoin-mempool/src/policy/output.rs:17-89; packages/open-bitcoin-mempool/src/types.rs:41-84]

### Anti-Patterns to Avoid

- **Raw `Vec<Transaction>` plus flags:** allows invalid shape/capability combinations to leak deep into admission. Use opaque refinements. [VERIFIED: 132-CONTEXT.md D-01/D-02/D-06]
- **Per-member calls to the live mutator:** changes rolling/membership between members and cannot produce one guarded transition. Use the common candidate preparer over the overlay. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209; 132-CONTEXT.md D-08]
- **Whole-mempool clone as “atomicity”:** makes up to 25 full clones/recomputations and hides touched-state correctness. Use sparse patches; keep clone/recompute test-only. [VERIFIED: 132-CONTEXT.md D-08; packages/open-bitcoin-mempool/src/pool/admission.rs:138-161]
- **One effective minimum fee:** `max(static, rolling)` is correct for ordinary single admission but wrong as the only package abstraction. Keep static/member, rolling/group, and incremental/replacement roles separate. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:110-114; packages/open-bitcoin-mempool/src/fee.rs:71-181; 132-CONTEXT.md D-11]
- **Result map as authority:** Knots uses maps internally, but Phase 132 explicitly improves the local contract to input-index alignment. Keyed views are lookup-only. [VERIFIED: packages/bitcoin-knots/src/validation.h:238-260; 132-CONTEXT.md D-03/D-04]
- **Reporting preparation success:** pressure or replacement can remove a previously successful member. Final prospective membership is authoritative. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2104-2145]
- **Alias as admission:** same-txid/different-witness means the requested witness was not validated or inserted; return the existing wtxid and emit no lifecycle/fee-group facts for the alias. [VERIFIED: packages/bitcoin-knots/src/validation.h:111-167; packages/bitcoin-knots/src/validation.cpp:2052-2063]
- **Package engine publishing side effects:** relay, persistence, caches, and evidence are Phase 134/137 concerns. Return semantic facts only. [VERIFIED: 132-CONTEXT.md D-16 and Deferred Ideas]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Transaction identities/weight | Alternate serializer or hash implementation | Existing `transaction_txid`, `transaction_wtxid`, and weight helpers | Canonical project-owned encoding already exists and is parity tested. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs:54-76; packages/open-bitcoin-mempool/src/pool/admission.rs:85-93] |
| Package fingerprint hashing | New crypto crate | Existing first-party `Sha256::digest` over exact Knots preimage | Avoids dependency and endian mistakes. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs:23-24; packages/bitcoin-knots/src/policy/packages.cpp:151-169] |
| Lifecycle fact conflict resolution | Parallel package-only delta structure | `MempoolLifecycleDeltaBuilder`, extended with checked composition if needed | It already enforces identity coherence, deterministic ordering, and final-membership completeness. [VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs:175-337] |
| Topology oracle | Separate graph library | Existing topology helpers and test-only `recompute_state` | Keeps one definition of ancestry/descendant stats and provides an independent oracle. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs:430-525] |
| Pressure policy | A second package-only eviction algorithm | Phase-131 victim selection/rolling bump factored over the prospective view | Package trim must match existing accounted-memory pressure semantics. [VERIFIED: packages/open-bitcoin-mempool/src/pool/pressure.rs:26-99; 131-CONTEXT.md]
| Non-empty package/group dependency | Third-party non-empty collection | Private checked wrapper with `first`, `last`, `iter`, `len` only | Bounds are tiny and the invariant is phase-specific. [VERIFIED: 132-CONTEXT.md D-01/D-04; AGENTS.md dependency constraints] |
| Feerate-diagram comparison | Boolean “new fee > old fee” shortcut | Port the pinned chunk/diagram dominance rule with exact tests | Limited package RBF requires economic dominance, not only total fee increase. [VERIFIED: packages/bitcoin-knots/src/policy/rbf.h:121-126; packages/bitcoin-knots/src/validation.cpp:1400-1414] |

**Key insight:** The complex part is not parsing 25 transactions; it is maintaining one coherent hypothetical mempool across partial successes, replacement removals, topology/resource cache updates, rolling pressure, and final truth. Existing identity, lifecycle, resource, and validation primitives should be composed behind a typed overlay rather than reimplemented. [VERIFIED: current Open Bitcoin seams cited above; 132-CONTEXT.md D-07 through D-15]

## Common Pitfalls

### Pitfall 1: Revision Guard Covers Entries but Not Rolling State

**What goes wrong:** A prepared package applies after a block gate, pressure bump, or decay changed the rolling floor, even though its fee decision used older state. [VERIFIED: packages/open-bitcoin-mempool/src/fee/rolling.rs:24-135]

**How to avoid:** Revision every mutation of entries, indexes, ledger, topology caches, and `RollingFeeState`; compare before any apply mutation. Test stale transitions caused independently by membership and rolling-only changes. [VERIFIED: 132-CONTEXT.md D-09]

**Warning sign:** `base_revision` increments only in `admission.rs`, or rolling setters remain direct public mutations. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs:112-130]

### Pitfall 2: Sparse Patch Produces Stale Resource Accounting

**What goes wrong:** Adding/removing a child changes the parent/child sets and therefore accounted memory of existing entries; updating only the new/removed entry makes the Phase-131 ledger drift. [VERIFIED: packages/open-bitcoin-mempool/src/resource.rs:215-266]

**How to avoid:** Treat every entry whose direct relations or aggregate stats change as touched; subtract its old accounted value and add its updated value with checked arithmetic. Compare every prepared transition against `recompute_state` and `recompute_resource_ledger` in tests. [VERIFIED: packages/open-bitcoin-mempool/src/resource.rs:268-319; packages/open-bitcoin-mempool/src/pool.rs:430-525]

### Pitfall 3: Static Floor Is Accidentally Package-Bumpable

**What goes wrong:** A high-fee child lets an ordinary zero/low-fee parent bypass the anti-free-relay floor. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1097-1107]

**How to avoid:** Check each ordinary new member against `StaticRelayFeeRate` before aggregate rolling assessment; exempt only version 3 under `TrucPolicy::Enforce`. Keep static failure hard, not reconsiderable. [VERIFIED: 132-CONTEXT.md D-11/D-13; packages/bitcoin-knots/src/validation.cpp:1097-1112]

### Pitfall 4: Hard Failure Stops Later Singleton Evaluation

**What goes wrong:** A valid later member is never individually admitted, contradicting pinned partial acceptance. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2073-2087]

**How to avoid:** Set `skip_residual = true` but continue the input-order singleton loop. Only the residual group is suppressed. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2031-2096]

### Pitfall 5: Existing/Alias Members Pollute Fees or Lifecycle

**What goes wrong:** Existing fees are double counted, an unvalidated witness alias is reported as admitted, or a later cache projection sees a phantom member. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2040-2063]

**How to avoid:** Exclude both classifications from residual groups and admitted/removed lifecycle facts. `SameTxidDifferentWitness` must carry the actual existing wtxid. [VERIFIED: 132-CONTEXT.md D-10/D-14]

### Pitfall 6: Ephemeral Spend Check Is Added Behind Unconditional Dust Rejection

**What goes wrong:** No ephemeral package can reach the spend checker, so PACK-07 appears implemented but is behaviorally dead. [VERIFIED: packages/open-bitcoin-mempool/src/policy/output.rs:79-88]

**How to avoid:** First model dust relay rate and permitted ephemeral output form in standardness, then enforce zero base/modified fee, then all-parent-dust spending after topology bounds. [VERIFIED: packages/bitcoin-knots/src/policy/policy.cpp:185-199; packages/bitcoin-knots/src/policy/ephemeral_policy.cpp:23-94]

### Pitfall 7: TRUC `Accept` Is Confused with `Enforce`

**What goes wrong:** Default `Accept` either rejects valid version-3 transactions, applies topology restrictions unexpectedly, or grants the static-floor exception. [VERIFIED: packages/bitcoin-knots/src/kernel/mempool_options.h:20-36; packages/bitcoin-knots/src/validation.cpp:843-850,1097-1107,1208-1209]

**How to avoid:** Use a three-variant enum and branch each responsibility explicitly: `Reject` rejects version 3, `Accept` treats it as ordinary, `Enforce` applies TRUC rules and the fee exception. [VERIFIED: same sources]

### Pitfall 8: Final Report Uses Intermediate Success

**What goes wrong:** A transaction removed by package replacement or the final trim remains “accepted.” [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2104-2145]

**How to avoid:** Rewrite by requested wtxid for new/exact members and by txid for witness aliases, using the final prospective view after the one trim. Emit `PostTrimAbsent` when the prior origin disappeared. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2108-2137; 132-CONTEXT.md D-15]

### Pitfall 9: Apply Still Has Fallible Work After First Mutation

**What goes wrong:** Allocation, delta building, topology recompute, or trim fails halfway through live mutation. [VERIFIED: 132-CONTEXT.md D-09]

**How to avoid:** Preparation owns all new entries/collections and completes all checked calculations. Apply does revision check, then deterministic moves/replacements only, with revision increment last. [VERIFIED: packages/bitcoin-knots/src/txmempool.h:824-918]

### Pitfall 10: Fingerprint Sorting Reorders the Report

**What goes wrong:** The order-independent hash’s sorted wtxids become the report order. [VERIFIED: packages/bitcoin-knots/src/policy/packages.cpp:151-169]

**How to avoid:** Compute fingerprint from a temporary copy only; the package’s private member vector never changes order. Add permutation-equality hash tests plus request-order report tests. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:52-130; 132-CONTEXT.md D-03/D-05]

## Code Examples

These sketches are prescriptive shapes, not copy-paste-complete implementations.

### Opaque Package Boundary

```rust
pub struct WellFormedPackage {
    members: Vec<PackageMember>,
    fingerprint: PackageFingerprint,
}

impl TryFrom<Vec<Transaction>> for WellFormedPackage {
    type Error = PackageShapeError;

    fn try_from(transactions: Vec<Transaction>) -> Result<Self, Self::Error> {
        let members = validate_and_index_package(transactions)?;
        let fingerprint = PackageFingerprint::from_members(&members);
        Ok(Self { members, fingerprint })
    }
}
```

The public API exposes ordered iteration and length, but no mutable access or constructor bypass. [VERIFIED: 132-CONTEXT.md D-01/D-03/D-05; source behavior: packages/bitcoin-knots/src/policy/packages.cpp:79-169]

### Guarded Transition

```rust
struct MempoolPatch {
    base_revision: MempoolRevision,
    next_revision: MempoolRevision,
    entry_upserts: BTreeMap<Txid, MempoolEntry>,
    entry_removals: BTreeSet<Txid>,
    spent_updates: BTreeMap<OutPoint, Option<Txid>>,
    topology_updates: BTreeMap<Txid, TopologyUpdate>,
    resource_delta: MempoolResourceDelta,
    rolling_fee_state: RollingFeeState,
    lifecycle_delta: MempoolLifecycleDelta,
}

impl Mempool {
    fn apply_prepared(
        &mut self,
        patch: MempoolPatch,
    ) -> Result<MempoolLifecycleDelta, MempoolError> {
        if self.revision != patch.base_revision {
            return Err(MempoolError::StalePreparedTransition);
        }
        let (next_revision, delta) = self.apply_infallible_patch(patch);
        self.revision = next_revision;
        Ok(delta)
    }
}
```

The actual implementation must prepare the next revision without leaving overflow as a post-mutation failure; revision exhaustion should be checked before mutation. [VERIFIED: 132-CONTEXT.md D-09; project fail-fast/error-propagation rules]

### Final Membership Rewrite

```rust
for (member, result) in package.members().zip(report.members_mut()) {
    let still_present = match result {
        PackageMemberResult::SameTxidDifferentWitness(_) => {
            prospective.contains_txid(member.identity.txid)
        }
        _ => prospective.contains_wtxid(member.identity.wtxid),
    };

    if result.was_prior_success() && !still_present {
        *result = PackageMemberResult::PostTrimAbsent(
            PostTrimAbsence::from_prior(member.identity, result),
        );
    }
}
```

Querying witness aliases by txid matches the pinned final rewrite while keeping the requested wtxid out of admission facts. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2108-2137]

## Exact Parity Test Matrix

### Shape and Identity

- Empty input; exactly 25 versus 26 members; exactly 404,000 versus 404,001 weight; duplicate txid including same-txid/different-witness members; duplicate wtxid; child before parent; cross-member double spend; zero-input member; and within-one-transaction duplicate input reaching consensus rather than package-conflict classification. [VERIFIED: packages/bitcoin-knots/src/policy/packages.cpp:52-117; packages/bitcoin-knots/src/test/txpackage_tests.cpp:133-201]
- Fingerprint fixed vectors and all permutations of the same members; prove report order remains request order. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:52-130]
- Multi-member submission rejects unrelated members, three generations, and a missing unconfirmed parent; single-member submission remains valid. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:258-355,356-496,497-587]

Locked D-01 requires the 404,000 bound at the opaque package boundary without the Knots single-member diagnostic exception at `packages.cpp:89-91`; tests should follow D-01. [VERIFIED: 132-CONTEXT.md D-01; packages/bitcoin-knots/src/policy/packages.cpp:87-91]

### Ordered Partial Admission and Dry-Run

- Valid parent plus invalid child leaves the parent finally present; a hard failure skips residual evaluation but later valid singletons are still evaluated. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:420-458; packages/bitcoin-knots/src/validation.cpp:2073-2096]
- Reconsiderable low-fee/missing-input parent and fee-paying child succeed as a residual group; exact existing members are deduplicated. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:824-1034; packages/bitcoin-knots/src/validation.cpp:2040-2096]
- Dry-run and submit produce the same ordered prospective report from the same base; dry-run leaves entries, spent index, topology stats, ledger, full rolling state, and revision exactly equal to the pre-call snapshot. [VERIFIED: 132-CONTEXT.md D-06]
- A transition prepared at revision N fails unchanged after membership-only and rolling-only state changes. [VERIFIED: 132-CONTEXT.md D-09]

### Fees, Witnesses, and Final Trim

- Every ordinary member fails below static floor even when aggregate is high; an eligible group passes/fails active rolling as an aggregate; enforced TRUC version 3 exercises the sole static exception. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1097-1112,1831-1856; packages/bitcoin-knots/test/functional/mempool_truc.py:640-684]
- Effective groups preserve request-order wtxids and exclude existing exact/witness-alias members. [VERIFIED: packages/bitcoin-knots/src/test/txpackage_tests.cpp:588-823; packages/bitcoin-knots/src/validation.cpp:2040-2063]
- Same txid/different witness returns the existing wtxid, does not validate/admit the requested witness, and emits no admitted/removed lifecycle fact. [VERIFIED: packages/bitcoin-knots/src/validation.h:111-167; packages/bitcoin-knots/src/test/txpackage_tests.cpp:588-823]
- Force final pressure trim to remove a newly accepted member and an already-present/alias target; verify every prior success becomes `PostTrimAbsent`, rolling bump occurs once, and lifecycle facts match final truth. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:2099-2145; packages/open-bitcoin-mempool/src/pool/pressure.rs:26-73]

### Limited RBF

- Accept basic 1P1C replacement; reject wrong package size/topology, any new in-mempool ancestor, conflict clusters larger than two, conservative direct-conflict descendant-count sum over 100 before union/diagram allocation, insufficient absolute/incremental fee, package feerate not above parent, and non-improving diagram. Include the overlap case where the deduplicated removal union is at most 100 but the conservative sum exceeds 100 and must reject. [VERIFIED: packages/bitcoin-knots/src/policy/rbf.cpp:60-86; packages/bitcoin-knots/test/functional/mempool_package_rbf.py:99-554; packages/bitcoin-knots/src/validation.cpp:1346-1415]
- Verify all staged removals and additions appear in one lifecycle delta, while any policy/script failure leaves both membership and rolling state unchanged. [VERIFIED: 132-CONTEXT.md D-09/D-10; packages/bitcoin-knots/src/txmempool.h:824-918]

### TRUC and Ephemeral Dust

- Cover all three TRUC modes, 10,000-vB max, 1,000-vB child max, ancestor/descendant count two, inheritance both directions, in-package ancestor calculations, sibling/parent-and-child rejection, and pre-replacement child-replacement plus eligible sibling-eviction handling from direct-conflict/intent inputs. [VERIFIED: packages/bitcoin-knots/src/policy/truc_policy.h:18-64; packages/bitcoin-knots/src/policy/truc_policy.cpp:202-262; packages/bitcoin-knots/test/functional/mempool_truc.py:56-684]
- Cover zero-fee dust parent success with complete sweep, nonzero base fee, injected nonzero modified fee at the pure policy seam, multiple dust outputs, missing any parent dust, multiple dusty parents, and in-mempool dusty parents. Add combination tests proving `anchor` alone admits only P2A zero-valued dust, `send` is additionally required for dusty non-anchor output, and `dust` is additionally required for nonzero-valued dust; defaults remain `true/false/false`. [VERIFIED: packages/bitcoin-knots/src/policy/policy.cpp:185-199; packages/bitcoin-knots/src/test/txvalidation_tests.cpp:126-283; packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py:82-442]
- Use `mempool_ephemeral_dust.py` and `txvalidation_tests.cpp` as the direct dust anchors; `mempool_truc.py` mainly anchors TRUC and the enforced-TRUC min-relay exception. [VERIFIED: local source search and cited files]

### Overlay Oracle and Bounds

- For deterministic generated graphs, apply singleton success, residual group, replacement, and trim patches to the sparse overlay; materialize a clone only in the test oracle; compare entries, parents/children, ancestor/descendant aggregates, spent index, resource ledger, rolling state, and lifecycle final membership against full recomputation. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs:430-525; packages/open-bitcoin-mempool/src/resource.rs:268-319; 132-CONTEXT.md discretion]
- Instrument or benchmark the max 25-member case to prove the production path performs zero whole-mempool clones/recomputes per member and one final trim. [VERIFIED: 132-CONTEXT.md D-08/D-13; MAX_PACKAGE_COUNT in packages/bitcoin-knots/src/policy/packages.h:18-24]

## Recommended Plan Decomposition

The repo uses fine planning granularity; use the following dependency order. [VERIFIED: .planning/config.json `granularity: fine`]

### Plan 132-01 — Opaque Package Vocabulary and Ordered Reports

- Add shape/refinement/fingerprint/report types, exact bounds, canonical identity caching, input-order lookup projections, and fixed fingerprint vectors.
- Add parity breadcrumbs for every new file.
- Covers PACK-01 and the vocabulary portions of PACK-03/PACK-07. [VERIFIED: requirements and architecture above]

### Plan 132-02 — Shared Candidate Preparation and Mempool Revision

- Extract non-mutating pre-script candidate preparation plus a separate contextual script checker from `commit_transaction_with_context`.
- Add `MempoolRevision`, owned sparse `MempoolPatch`, revision-first apply helpers, stale-base error, and revision coverage for rolling/block/expiry/admission paths.
- Migrate legacy single admission onto pre-script preparation → legacy-order script check → sparse prepare/apply without changing external behavior.
- This is the highest regression-risk plan and should land before package orchestration. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209; mutation inventory above]

### Plan 132-03 — Sparse Prospective Overlay and Accounting Oracle

- Add overlay-first entry/spent lookups, touched topology/stat updates, checked resource subtraction/replacement, patch composition, and test-only materialization/recompute oracle.
- Factor Phase-131 pressure selection/application so final trim can operate on the overlay.
- Establish “no whole-mempool clone on normal path” tests/instrumentation.
- Covers the infrastructure of PACK-05. [VERIFIED: 132-CONTEXT.md D-08/D-09; current resource/topology seams]

### Plan 132-04 — Individual-First Dry-Run and Submission Engine

- Add distinct command types, exact/existing/alias classification, input-order singleton evaluation, residual retry classification, skip-residual-on-hard behavior, coherent patch composition, and dry-run discard.
- Integrate the child-with-unconfirmed-parents contextual refinement and ordered package-wide report from Plan 132-01.
- Covers PACK-02, PACK-03, and PACK-04. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1960-2096]

### Plan 132-05 — Effective Fee Groups and Final Membership Rewrite

- Add typed base/modified candidate fee facts and generalize fee-floor assessment to all ordinary group members plus aggregate rolling.
- Add the three-state TRUC policy vocabulary/default, explicit non-empty ordered fee groups, and the enforced-TRUC static exception seam; defer full TRUC topology checks to Plan 132-07.
- Run one final trim, rewrite all member results, and finalize lifecycle delta from actual final membership.
- Covers PACK-06 and final-truth portions of PACK-07. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1097-1112,1831-1856,2099-2145]

### Plan 132-06 — Limited Package RBF

- Port the exact 1P1C/no-ancestors/conservative descendant-count-before-union/incremental-fee/parent-versus-package/diagram rules over the overlay.
- Add targeted Knots unit/functional parity cases and lifecycle-removal assertions.
- Covers replacement portions of PACK-07. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1320-1423; packages/bitcoin-knots/test/functional/mempool_package_rbf.py]

### Plan 132-07 — TRUC, Pay-to-Anchor, and Ephemeral-Dust Policy

- Complete the exact TRUC topology/inheritance/size checks using pre-replacement direct-conflict plus sibling-eviction-intent inputs and the policy vocabulary introduced in Plan 132-05.
- Add explicit pay-to-anchor script classification with empty-witness standardness, independent dust relay rate, exact `anchor`/`send`/`dust` predicates and defaults, and zero-fee/complete-spend checks over the fee facts introduced in Plan 132-05.
- Add direct parity tests from `mempool_truc.py`, `mempool_ephemeral_dust.py`, and `txvalidation_tests.cpp`.
- Covers the remaining PACK-07 surface. [VERIFIED: packages/bitcoin-knots/src/script/script.cpp:208-223; packages/bitcoin-knots/src/policy/policy.cpp:351-371; exact policy anchors above]

### Plan 132-08 — Parity Closure and Full Verification

- Run the max-bound oracle matrix, dry-run equality, stale-revision, partial-acceptance, final-trim, and advanced-policy suites.
- Update `docs/parity/source-breadcrumbs.json`, `docs/parity/catalog/mempool-policy.md`, relevant README status, and tracked LOC freshness.
- Run `bash scripts/verify.sh` and review the diff. Do not claim Phase-133 peer assembly, Phase-134 cache projection, Phase-137 adapters, or Phase-138 adversarial soak. [VERIFIED: AGENTS.md Repo-Local Guidance; 132-CONTEXT.md Deferred Ideas]

## Verification Commands

Use non-overlapping commands in this order during implementation:

```bash
bun run scripts/command-timings.ts run --key phase132-mempool-tests -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool

bun run scripts/check-parity-breadcrumbs.ts

bun run scripts/command-timings.ts run --key phase132-bazel-smoke -- \
  bazel build //packages/open-bitcoin-mempool:open_bitcoin_mempool_lib

bash scripts/verify.sh
```

The package-specific Cargo command is the fastest behavior gate; the root verifier remains the required completion contract. The Bazel target is a library smoke build, while Rust unit/integration tests run through Cargo. [VERIFIED: packages/open-bitcoin-mempool/BUILD.bazel; AGENTS.md Repo-Local Guidance; standards/core/verification.md]

## State of the Art

| Old/current approach | Phase-132 approach | Why it matters |
| --- | --- | --- |
| One single-tx mutator clones/recomputes/trims and commits per call. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209] | Shared candidate preparation over a sparse revision-bound view, one package trim, one apply. [VERIFIED: 132-CONTEXT.md D-08/D-09/D-13] | Enables bounded partial acceptance without false global atomicity or repeated full clones. |
| `max(static, rolling)` is enforced on each single transaction. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:110-114] | Ordinary static floor remains per member; active rolling floor can use an eligible non-empty aggregate; incremental stays replacement/pressure only. [VERIFIED: 132-CONTEXT.md D-11] | Prevents both static-floor bypass and package-CPFP failure. |
| Outcome categories omit witness alias and reconsiderable distinction. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs:16-115] | Input-aligned sum types distinguish existing, alias, hard, reconsiderable, final present, and post-trim absent. [VERIFIED: 132-CONTEXT.md D-04/D-14/D-15] | Makes impossible result combinations unrepresentable and supports later adapters. |
| Fixed dust thresholds are unconditional rejection, and pay-to-anchor is only an unknown witness program. [VERIFIED: packages/open-bitcoin-mempool/src/policy/output.rs:17-89; packages/open-bitcoin-consensus/src/classify.rs:37-77,250-269] | Explicit pay-to-anchor classification, role-typed dust relay rate, scoped ephemeral permission, zero-fee rule, then complete-spend rule. [VERIFIED: packages/bitcoin-knots/src/script/script.cpp:208-223; packages/bitcoin-knots/src/policy/policy.cpp:55-81,185-199; packages/bitcoin-knots/src/policy/ephemeral_policy.cpp:23-94] | Makes the required ephemeral package surface reachable and auditable. |
| No TRUC policy vocabulary. [VERIFIED: packages/open-bitcoin-mempool/src/types.rs:34-84] | Exact `Reject`/`Accept`/`Enforce` with pinned default `Accept`; only `Enforce` grants the version-3 static exception. [VERIFIED: packages/bitcoin-knots/src/kernel/mempool_options.h:20-36; packages/bitcoin-knots/src/validation.cpp:843-850,1097-1107] | Avoids silent policy changes and closes PACK-07. |

**Deprecated/outdated for this phase:**

- Treating `PackageFeeFloorAssessment` as a complete package engine is outdated; it assesses only one member plus one aggregate. Extend it rather than deleting the fee-role types. [VERIFIED: packages/open-bitcoin-mempool/src/fee.rs:152-181]
- Treating `Mempool::commit_transaction_with_context` as the reusable package primitive is outdated; its validation logic is reusable only after preparation/apply extraction. [VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs:77-209]
- Using `mempool_truc.py` as the only ephemeral-dust source is incomplete; direct dust coverage lives in `mempool_ephemeral_dust.py` and `src/test/txvalidation_tests.cpp`. [VERIFIED: local pinned source search]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| — | None. All recommendations derive from locked context, current repository code/instructions, installed tool probes, or the pinned Knots tree. | All | No user confirmation is required before planning. |

## Resolved Decisions

1. **Ephemeral permission defaults and meanings — RESOLVED**
   - `PolicyConfig::default()` uses the pinned source defaults `anchor=true`, `send=false`, `dust=false`.
   - The predicates are exact and cumulative: `anchor` gates P2A, `send` gates dusty non-anchor outputs, and `dust` gates nonzero-valued dust. Core combination tests exercise all gates; Phase 132 adds no operator configuration surface. [VERIFIED: packages/bitcoin-knots/src/policy/policy.h:70-76; packages/bitcoin-knots/src/policy/policy.cpp:185-199; 132-CONTEXT.md D-12/D-16]

2. **Revision/prepared-transition visibility — RESOLVED**
   - `MempoolRevision` and `MempoolPatch` remain crate-private; no full-state `PreparedMempoolTransition` wrapper exists.
   - Public callers receive only the narrow command result/report/lifecycle surface already required by D-04/D-10/D-16. Tests may observe the typed stale-transition error through the narrowest crate-private/test seam; no revision counter or prepared transition becomes public evidence. [VERIFIED: 132-CONTEXT.md D-09/D-16; functional-core boundary in AGENTS.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust/Cargo | Core implementation and tests | ✓ | `1.94.1` | None needed. [VERIFIED: local version probes] |
| Bun | Timed commands and TypeScript parity checks | ✓ | `1.3.9` | Thin direct commands only for diagnosis; repo contract prefers Bun wrapper. [VERIFIED: local version probe; AGENTS.md] |
| Bazelisk/Bazel | Root smoke build | ✓ | Bazel `8.6.0` | Cargo can diagnose crate behavior, but full verifier still requires Bazel. [VERIFIED: local version probe; scripts/verify.sh] |
| Pinned Knots submodule | Exact parity sources/tests | ✓ | `v29.3.knots20260210` / `a9aee730...` | None; run `git submodule update --init --recursive` if later missing. [VERIFIED: local git probe; AGENTS.md] |

**Missing dependencies with no fallback:** None. [VERIFIED: environment probes]

**Missing dependencies with fallback:** None. [VERIFIED: environment probes]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| V2 Authentication | No | Pure local mempool core has no authentication boundary; adapters remain deferred. [VERIFIED: 132-CONTEXT.md D-16/Deferred Ideas] |
| V3 Session Management | No | No session state exists in this phase. [VERIFIED: phase boundary] |
| V4 Access Control | No | No user/role authorization is implemented in the pure package core. [VERIFIED: phase boundary] |
| V5 Input Validation | Yes | Opaque bounded package refinements, checked arithmetic, exact policy sum types, and fail-closed errors. [VERIFIED: 132-CONTEXT.md D-01/D-02/D-09] |
| V6 Cryptography | Yes, non-secret hashing only | Reuse first-party `Sha256` for the Knots-compatible package fingerprint; do not implement or import cryptography. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs:23-24; packages/bitcoin-knots/src/policy/packages.cpp:151-169] |

### Known Threat Patterns for Package Admission

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Oversized/expensive package before cheap checks | Denial of Service | Enforce non-empty/count/weight/identity/topology/conflict bounds before chainstate, scripts, or mempool mutation. [VERIFIED: packages/bitcoin-knots/src/policy/packages.cpp:79-117; 132-CONTEXT.md D-13] |
| Reusing a stale prepared transition | Tampering | Revision-bind every prepared patch and fail before mutation. [VERIFIED: 132-CONTEXT.md D-09] |
| Partial live mutation after failure | Tampering | Complete all fallible preparation/delta composition first; one guarded apply. [VERIFIED: 132-CONTEXT.md D-08/D-09] |
| Static-floor bypass through aggregation | Denial of Service | Independent static floor for every ordinary member; aggregate only the rolling obligation. [VERIFIED: 132-CONTEXT.md D-11; packages/bitcoin-knots/src/validation.cpp:1097-1112] |
| Witness-alias identity confusion | Spoofing/Tampering | Preserve requested txid/wtxid pair, return existing wtxid explicitly, and exclude alias from fees/lifecycle. [VERIFIED: packages/bitcoin-knots/src/validation.h:111-167; 132-CONTEXT.md D-14] |
| Replacement amplification | Denial of Service | Exact limited 1P1C topology, no mempool ancestors, 100-candidate bound, incremental fee, and diagram improvement. [VERIFIED: packages/bitcoin-knots/src/validation.cpp:1346-1415; packages/bitcoin-knots/src/policy/rbf.h:25-27] |
| Ephemeral dust mined/retained without sponsor | Denial of Service | Zero base and modified fee for dusty parent, plus complete dust spending by every child relation. [VERIFIED: packages/bitcoin-knots/src/policy/ephemeral_policy.cpp:23-94] |
| Cached topology/resource drift | Tampering/Denial of Service | Checked sparse accounting plus full recomputation oracle on deterministic generated graphs. [VERIFIED: packages/open-bitcoin-mempool/src/resource.rs:268-319; packages/open-bitcoin-mempool/src/pool.rs:430-525] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md` — locked decisions, discretion, canonical sources, and phase boundaries.
- `.planning/REQUIREMENTS.md` — PACK-01 through PACK-07.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/{architecture,code-shape,verification,testing}.md`, `standards/languages/rust.md` — project workflow, architecture, style, test, and verification constraints.
- `packages/open-bitcoin-mempool/src/{fee,outcome,types,resource,policy/output}.rs` and `src/pool{,/admission,/pressure,/lifecycle,/expiry,/topology}.rs` — current implementation seams and gaps.
- `packages/open-bitcoin-consensus/src/{crypto,classify}.rs` and `src/script/witness.rs` — existing identity/SHA-256 helpers, witness classification, and the pay-to-anchor prerequisite.
- `packages/bitcoin-knots/src/policy/packages.{h,cpp}` — exact shape, topology, child-with-parents, and fingerprint behavior.
- `packages/bitcoin-knots/src/validation.{h,cpp}` — result vocabulary, individual-first processing, grouping, replacement, trim, and final rewrite.
- `packages/bitcoin-knots/src/txmempool.h` — staged changeset model.
- `packages/bitcoin-knots/src/policy/{rbf,truc_policy,ephemeral_policy}.{h,cpp}` and `src/kernel/mempool_options.h` — advanced policy rules and defaults.
- `packages/bitcoin-knots/src/test/txpackage_tests.cpp`, `src/test/txvalidation_tests.cpp`, and `test/functional/{mempool_truc,mempool_package_rbf,mempool_ephemeral_dust}.py` — exact parity cases.

### Secondary (MEDIUM confidence)

- `.planning/research/{ARCHITECTURE,FEATURES,PITFALLS,SUMMARY}.md` — milestone synthesis cross-checked against current code and pinned Knots.
- Phase 130/131 context and summaries — predecessor fee/resource/pressure decisions cross-checked against current implementation.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — pinned workspace files and installed commands agree; no new dependency is recommended.
- Architecture: HIGH — locked decisions, current code seams, and Knots staged/individual-first behavior converge on the same refinement/overlay design.
- Policy behavior: HIGH — exact pinned source and direct unit/functional tests were traced for package RBF, TRUC, dust, witnesses, and trim rewriting.
- Plan decomposition: HIGH — ordered by explicit dependencies discovered in current code; the single-admission extraction and dust standardness prerequisites are unavoidable.
- Pitfalls: HIGH — each is evidenced by current code or a pinned Knots guard/test.

**Research date:** 2026-07-25

**Valid until:** 2026-08-24, or until the pinned Knots submodule, Phase 132 context, or mempool core changes.

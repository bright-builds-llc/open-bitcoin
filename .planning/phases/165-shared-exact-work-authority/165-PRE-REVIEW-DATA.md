---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T13:19:05Z
depth: standard
review_scope: preliminary-data-and-evidence-integrity-only
base_commit: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
knots_commit: a9aee730466ac67d35a3c03ee24676be5e045878
files_reviewed: 4
files_reviewed_list:
  - docs/parity/fixtures/phase165-work-vectors.json
  - packages/testdata/phase165/legacy-c-fixtures.json
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
phase_qualified: false
native_qualification: false
pinned_daemon_qualification: false
---

# Phase 165: Preliminary Data and Evidence Review

**CLEAN for the data scope below.** No actionable integrity, arithmetic, manifest-path, provenance or qualification-label defect was identified. This report does not clear the fixture warning in `165-PRE-REVIEW-FIXTURES.md`, review later source edits or qualify the phase.

## Scope and method

Discovery combined tracked changes against the exact resolved base `d0104a997c0362bf65b81de1c8fec8ddc3012a1b` with untracked, nonignored paths. The only changed/new file under `packages/testdata/` is `packages/testdata/phase165/legacy-c-fixtures.json`. The only additional changed non-Rust/non-script fixture payload found is `docs/parity/fixtures/phase165-work-vectors.json`. The changed parity index and breadcrumb JSON were also inspected as supporting evidence metadata, rather than runtime fixture payloads.

The review read the actual Plan 01, 12 and 13 summaries, the arithmetic corpus consumers and bounded corpus/legacy manifest loader, and the materialized pinned Knots compact/proof/pow-limit sources. The original ignored seven-source pre-C capture and separate Git-base capture were read only to independently authenticate the tracked mixed-provenance bundle. No project module was imported or executed.

Guidance and active lessons were carried from the preceding production and fixture audits: `AGENTS.md`, the Bright Builds sidecar/index, placeholder-only overrides, relevant architecture/code-shape/verification/testing/Rust standards, locked D-01 through D-12, and both fully read active lesson files (7,188 bytes / summed conservative estimate 2,397 tokens). No archive lessons or project skill directories were loaded.

Completed review steps:

- [x] Discover all changed/new fixture data and supporting JSON evidence metadata.
- [x] Recompute fixed arithmetic independently using Python standard-library integers.
- [x] Compare the ordered bridge rows and metadata with the original capture artifacts and exact Git blob.
- [x] Check fixed loader path/order/digest bindings and source-versus-runtime labels.
- [x] Check catalog/breadcrumb metadata and historical-data preservation without running project tooling.
- [x] Record exact full-file identities and review limitations.

## Independent arithmetic results

An independent, read-only Python calculation used integer arithmetic and the local pinned formulas. It did not call production Rust, import a project helper or run Bun.

| Corpus component | Actual independent result |
| --- | --- |
| 11 target/proof vectors | Every decoded target and `floor(2^256 / (target + 1))` proof matches the fixed 64-digit hex expectation. |
| Six addition vectors | Every sum matches modulo `2^256`, including bit64/bit128/higher carries, MAX + 1, two unit-target proofs wrapping to zero, and the mixed-target sum. |
| 14 invalid compact vectors | Every fixed Zero/Negative/Overflow expectation matches normalized mantissa classification. |
| Width and representation | All target/proof/sum values use exactly 64 lowercase hex digits; names are unique within each vector section. Six proof vectors are at least `2^128`, including target 1, target 2 and target 65535. |
| JSON integrity | Payload and supporting JSON parse without duplicate object keys. Reviewed data files and capture artifacts are regular files, not symlinks. |
| Source pin and evidence class | Corpus commit is exactly `a9aee730466ac67d35a3c03ee24676be5e045878`; evidence is explicitly `source-derived-arithmetic`, with no executed pinned-daemon or chain-validation provenance claim. |

The materialized Knots checkout resolves to the exact stated pin. Comparisons used `arith_uint256.cpp::SetCompact`, `chain.cpp::GetBlockProof`, `pow.cpp::DeriveTarget/CheckProofOfWorkImpl` and the canonical powLimits in `kernel/chainparams.cpp`.

Normalization shifts small-exponent mantissas before zero/sign/overflow classification. Tiny positive and signed mantissas truncated to zero therefore produce Zero; the positive equivalent unit encodings all produce target 1. Invalid Knots proof maps to zero internally, while the first-party corpus deliberately expects typed errors. Addition wraps rather than adding an overflow refusal.

The named mainnet/regtest proof vectors are arithmetic expectations, not network-admission receipts. Selected-network hash/powLimit behavior is covered in Rust source/tests reviewed separately; this corpus does not contain header/hash admission cases. As an additional numerical check, a bounded independent calculation of the maximum representable valid compact target under each actual canonical powLimit gives:

| Selected powLimit | Largest representable accepted compact target |
| --- | --- |
| Mainnet / Testnet3 | `00000000ffff0000000000000000000000000000000000000000000000000000` (`1d00ffff`) |
| Signet | `00000377ae000000000000000000000000000000000000000000000000000000` (`1e0377ae`) |
| Regtest | `7fffff0000000000000000000000000000000000000000000000000000000000` (`207fffff`) |

This checks representable compact range boundaries against the primary constants; it does not execute PoW admission or establish contextual validation.

## Ordered legacy bridge integrity

The tracked bundle has eight rows and correctly distinguishes its two provenance sources:

1. Its first seven **complete row objects**, including every path, source string, SHA-256 and order, exactly equal the original 124,139-byte pre-C capture. Capture time, base-context field and scope metadata also match. The original capture remains unchanged and retains its original seven-row scope.
2. The eighth complete row exactly equals the separate captured base-Git parent. Independent `git show BASE:path` and `git rev-parse BASE:path` confirm the 20,234 source bytes, SHA-256 `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` and blob `31dc52b18c51ee4a03e0aba0412443111fd153d7`. It is not mislabeled an eighth pre-C capture.
3. Every source SHA-256 and UTF-8 byte count was recalculated independently. All eight paths are unique, canonical repository-relative paths with no absolute prefix or `..` component. Row keys are exactly `path`, `sha256`, `source`.
4. The actual corpus loader's fixed `LEGACY_METADATA` and ordered `LEGACY_VERSIONS` match the data. Full-bundle byte count and SHA-256 match the loader constants and current bridge-data identity. Both current parent/relocated-child bindings recorded in the separate header capture match current source at review time.
5. Data rows do not choose file reads. The loader uses fixed code-owned inputs, authenticates whole bundle bytes before parsing and requires exact metadata/row order/path/source length/hash. Embedded strings feed only an explicitly labeled historical structural view, with current source versions separately bound. This static inspection does not claim execution of the loader's adversarial matrix.

| Ordered embedded source path | UTF-8 bytes | Source SHA-256 |
| --- | --- | --- |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs` | 14611 | `623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs` | 11654 | `3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs` | 11533 | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | 18737 | `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs` | 22182 | `abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 15320 | `933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs` | 23886 | `1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs` | 20234 | `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` |

The `phase165-legacy-function-contract-bridge` label and metadata explicitly exclude runtime qualification. These inert historical source strings are not current compiled tests, accepted chainstate, source authority, durable recovery evidence or new Phase 162/163 runtime receipts. Older source expectations are not promoted into current behavior by a successful structural projection.

## Supporting catalog and breadcrumb metadata

The current parity index adds exactly one `v2-6-shared-exact-work` row. Its status remains `in_progress`; requirement order is exactly CWOR-01/02/03/05. All 93 previous checklist surface objects remain unchanged and in their original relative order, and other top-level catalog fields remain unchanged. Every new evidence/upstream file reference exists. Its rationale, intentional differences and gaps retain the boundaries between numerical evidence, actual local refusal processes, whole-phase gates and later pinned-daemon qualification.

Breadcrumb metadata has 23 added/changed groups relative to the base, with unique group labels and existing referenced source/Knots anchor paths. This confirms mapping integrity only, not the completeness or correctness of tests in those paths. Current Plan 24/30 source changes retain separate code-review ownership.

Read-only `git diff --exit-code BASE` comparison found no changes in `packages/testdata/phase162`, `packages/testdata/phase163`, or `packages/testdata/phase164`, nor in the specifically compared Phase 162/163 native evidence and verification receipts. No immutable corpus, recipe, manifest, snapshot or receipt was rewritten by this review.

## Full-file identity inventory

The first four rows are the reviewed changed payload/metadata files. The final two are read-only original provenance artifacts used for comparison, not newly generated review files or runtime receipts.

| Path | Bytes | Full-file SHA-256 |
| --- | --- | --- |
| `docs/parity/fixtures/phase165-work-vectors.json` | 6785 | `3261c2d2c73cbd6977a30691b40514e4d2940abfbfc500b145ccdcc6b10d7451` |
| `packages/testdata/phase165/legacy-c-fixtures.json` | 146752 | `1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c` |
| `docs/parity/index.json` | 297621 | `03e2c79ee693e607d274fc9e2ee8fb7d4bc0026152939421ba866ad72f12a40d` |
| `docs/parity/source-breadcrumbs.json` | 168557 | `4fe89200d14b415d2ad4d14e842d7281e1af3d19dd376b0460bb31e1fad35061` |
| `.local/open-bitcoin-dev/phase165-pre-c-source-snapshots.json` | 124139 | `4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981` |
| `.local/open-bitcoin-dev/phase165-plan26-header-legacy-source.json` | 22002 | `2f15d035bfc1e57bc9ef2a9e93f69607d8c46b24ba441938ae5236b24a9bf783` |

The corpus and bundle identities matched the initial discovery and final independent checks. Supporting metadata also retained its discovered identities. Future changes require renewed data or metadata review rather than borrowing this result.

## Exclusions and qualification limits

Rust implementation/test/benchmark bodies and TypeScript/Bash/tooling behavior remain under their separate production, fixture and tooling reviews. The loader was read as the data consumer; this report does not replace its code review or execute its source-mutation controls. Planning documents and ignored capture artifacts were context/provenance inputs, not new runtime proof. No other changed non-Rust/non-script runtime-fixture payload was discovered.

Only lightweight read-only Python standard-library arithmetic/JSON/hash/path checks and Git reads/comparisons were run. No Cargo, Bazel, Bun, project import, test, build, source edit, Git mutation, shared metadata edit or source-guard renewal occurred. Only this new report was written. Read-only `git diff --check` passed.

All four ordinary canonical selectors remain refused and networking remains default off. CWOR-01/02/03/05 remain Phase 165's scope. Canonical bootstrap, exact all-handle checkpoint/reopen and genesis BASIC (CWOR-04) remain Phase 166; contextual completeness remains Phase 167; actual pinned-Knots daemon qualification remains Phase 168. Arithmetic matches, authentic captured source and fixed manifest integrity confer none of those runtime qualifications.

***

_Reviewer: independent gsd-code-reviewer. Preliminary data/evidence integrity audit only._

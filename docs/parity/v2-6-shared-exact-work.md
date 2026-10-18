# Shared Exact Work and Earned Live Authority

Surface id: `v2-6-shared-exact-work`. Phase 165 contributes CWOR-01, CWOR-02,
CWOR-03 and CWOR-05. This page records the implemented arithmetic, live-owner
contracts and focused evidence. Full phase acceptance remains a separate gate.
Ordinary history is currently refused for all four selectors; networking remains
default off. [Canonical identity and refusal](v2-6-canonical-chain-identity.md)
retains the exact mainnet, testnet3, default Signet and default regtest matrix.

## Pinned source and independent numerical evidence

The baseline is Bitcoin Knots `29.3.knots20260210` at
`a9aee730466ac67d35a3c03ee24676be5e045878`. The materialized source anchors are
[`GetBlockProof`](../../packages/bitcoin-knots/src/chain.cpp),
[`arith_uint256::SetCompact` and wrapping arithmetic](../../packages/bitcoin-knots/src/arith_uint256.cpp),
[`base_uint` operators](../../packages/bitcoin-knots/src/arith_uint256.h),
[`CheckProofOfWorkImpl`](../../packages/bitcoin-knots/src/pow.cpp),
[`BlockManager::AddToBlockIndex` and `CBlockIndexWorkComparator`](../../packages/bitcoin-knots/src/node/blockstorage.cpp),
and [`ReceivedBlockTransactions` / `FindMostWorkChain`](../../packages/bitcoin-knots/src/validation.cpp).
These links identify source-derived behavior; they do not report an executed
Knots process comparison.

For a positive normalized compact target `T`, the block proof is
`floor(2^256 / (T + 1))`. Accumulation and subtraction wrap modulo `2^256`;
comparison uses all 256 unsigned bits. There is no cumulative-overflow rejection.
The sole numeric carrier is the existing consensus
[`ProofWork`](../../packages/open-bitcoin-consensus/src/chain_proof.rs), with
private limbs, lossless 32-byte little-endian import/export and exactly 64
most-significant-first hexadecimal digits. Rendering uses lowercase hex.
Parsing refuses signs, prefixes, whitespace, incorrect width and non-hex digits.
Numerical exactness never establishes validation provenance.

[`compact_target_bytes`](../../packages/open-bitcoin-consensus/src/crypto.rs)
normalizes the mantissa before sign/zero decisions, including truncation of tiny
mantissas to zero and equivalent encodings. It rejects normalized zero, negative
and overflow targets. Actual header admission additionally checks the selected
parameters' powLimit and hash satisfaction; a syntactically valid compact target
or an easy valid hash alone cannot grant admission. Full contextual difficulty
completeness remains deferred.

The frozen [independent work corpus](fixtures/phase165-work-vectors.json) contains
11 proof vectors and six addition vectors, including above-u128 values, mixed
targets, carries, wrap and invalid compact boundaries. Expected constants are
source-derived independently of the production work helper and are not regenerated
from it. Actual [proof vector tests](../../packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs)
and [compact/admission tests](../../packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs)
are recorded in [Plan 01 evidence](../../.planning/phases/165-shared-exact-work-authority/165-01-SUMMARY.md).
No actual pinned-Knots daemon differential qualification is claimed.

## Header preference and body eligibility are separate

The accepted [header owner](../../packages/open-bitcoin-network/src/header_store.rs)
derives work from an accepted parent and the child's target proof. Best-header
replacement requires strictly greater work; equal work preserves the first
accepted best header. A shorter, harder branch can beat a longer, easier branch.
Both best and side block requests require accepted header proof facts and the
full selected parameter identity, rather than a raw seeded header projection.
[Header tests](../../packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs)
and [Node header tests](../../packages/open-bitcoin-node/src/network/tests/phase165_headers.rs)
exercise the actual owners; [Plan 04](../../.planning/phases/165-shared-exact-work-authority/165-04-SUMMARY.md)
and [Plan 05](../../.planning/phases/165-shared-exact-work-authority/165-05-SUMMARY.md)
separate proof/continuity rejection from contextual-prefix behavior.

Connected candidates use the same exact work plus checked body/ancestor linkage.
Known body order follows per-parent insertion-FIFO breadth-first release when
ancestry becomes eligible. Header arrival does not define body eligibility order,
and linked body history is not accepted coins history. An earlier eligible side
candidate remains visible even when an equal-work branch won header arrival.
The owner considers all retained eligible candidates, not only the best-header
projection. Present-body duplicates do not claim pruned-body redownload or
preciousblock parity.

Cached bytes received before their headers undergo actual full body/context
checks when readmitted. Their ordering remains unknown through ancestry, copies,
duplicates, publication and authorization; checks cannot reconstruct historical
arrival order. Strictly greater work can activate a fully checked candidate with
unknown order; equal-work activation requires known eligibility order.
No deterministic restart tie parity is claimed. Knots' internal pointer fallback
and disk sequence initialization are source observations, not evidence for a
deterministic hash ordering or persisted restart tie equivalence here.
[Eligibility tests](../../packages/open-bitcoin-network/src/header_store/eligibility/tests.rs),
[cached-order controls](../../packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs)
and [Plan 06 evidence](../../.planning/phases/165-shared-exact-work-authority/165-06-SUMMARY.md)
bound the known cases.

## Authority before effects

Immutable checked-body linkage uses an opaque shared seal; mutable eligibility
uses a separate owner epoch with weak grants. Invalidating current eligibility
revokes grants without rewriting past linkage. A copied sequence number cannot
replace seal identity. Private, non-Clone stages bind source origin, tip,
generation and owner. Source body and coins full identity must match before a
consuming rebind; raw snapshots and decoded numbers cannot earn lineage.

The [core authority](../../packages/open-bitcoin-chainstate/src/engine/work_authority.rs),
[managed authority](../../packages/open-bitcoin-node/src/chainstate/work_authority.rs)
and [network authorization](../../packages/open-bitcoin-node/src/network/work_authority.rs)
check actual owner preference before staged effects or BASIC preparation.
Unqualified direct lower managed reorg entrypoints refuse. Inflated work,
foreign/stale stages, caller-supplied positions and preview state cannot grant
activation. [Core adversarial tests](../../packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs)
and [freshness/history tests](../../packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs)
are linked from [Plan 07](../../.planning/phases/165-shared-exact-work-authority/165-07-SUMMARY.md)
and [Plan 08](../../.planning/phases/165-shared-exact-work-authority/165-08-SUMMARY.md).

The [sync reconciliation owner](../../packages/open-bitcoin-node/src/sync/block_reconcile.rs)
propagates local/storage/stale/BASIC failures. Only typed consensus failures allow
fallback to freshly queried and authorized candidates. A genuinely accepted
prefix reports progress and stops the current turn before new ancestry is acquired.
BASIC storage checks the actual active identity before writes.
[Actual candidate controls](../../packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs)
and [Plan 09 evidence](../../.planning/phases/165-shared-exact-work-authority/165-09-SUMMARY.md)
include stronger shorter branches, live equal-order replacement, fatal failures
with viable siblings and accepted-prefix termination.

## Explicit genuine fixture sources and complete-prefix validation

The public [genuine fixture constructor](../../packages/open-bitcoin-node/src/sync/live_fixture.rs)
consumes the same genuinely validated, flushed Fjall-backed managed source with
its complete selected consensus parameters. Before startup it independently
replays every actually retained active body and checks derived positions, undo,
coins, B/heads, validation coverage and physical body identity. All four canonical
root hashes are refused. Initial source refusals precede startup; later startup
or concurrent I/O errors do not promise rollback or return the consumed source.
This explicit fixture entrypoint never makes an ordinary opener eligible.

After a true all-handle close, a raw reopened observation remains unearned.
An explicit fixture may load EVERY actually available prefix body into fresh
Memory validation with the full selected parameters and original script flags,
derive its own positions/undo/logical counts, and consume that genuine owner
through strict rebind into the same physical Fjall target before using the public
constructor. Missing body/undo, incomplete validation or physical identity must
refuse; no missing data is restored and no raw number or partial prefix is promoted.
Physical None counts remain None rather than being normalized or persisted by
this handoff. The [normal-maturity public proof](../../packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs)
and [Phase127 consumer](../../packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs)
exercise the complete selected Regtest maturity100 path and first real extensions.
Older explicit custom-parameter recipes retain their own narrower scope.

The constructor builds checked header/body ancestry from those validated bytes.
Historical nonroot body order remains UNKNOWN; a later real arrival may earn new
order, but cannot reconstruct an earlier tie history. A strictly greater checked
candidate remains distinct from an equal-work candidate requiring known order.
One fresh live early-body BASIC fixture earns order in the same owner; reopening
does not replay that order. Fixture-only missing-count enrichment after real flush
remains a separate narrow live-owner seam. No fabricated durable root grants
authority. This is not automatic raw reopen, canonical Phase166 recovery or
deterministic restart-tie qualification.

## Legacy representation and non-opening refusal

The [number codec](../../packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs)
classifies historical bare/tagged u128 scores as `LegacySynthetic`, separately
from tagged `ExactNumber` bytes. Version, tag, fields and width are strict. The
512-byte numerical record bound includes whitespace; compact serialization also
honors it inside pretty snapshots. Neither classification, exact import nor
recomputation returns an acceptance token. No repair, reencode or automatic
reinterpretation is authorized. [Plan 02](../../.planning/phases/165-shared-exact-work-authority/165-02-SUMMARY.md)
and [Plan 03](../../.planning/phases/165-shared-exact-work-authority/165-03-SUMMARY.md)
record the representation and atomic carrier transition.

The [ordinary filesystem gate](../../packages/open-bitcoin-node/src/storage/chain_compatibility.rs)
retains observed network/root/work as unavailable. Its fixed guidance is followed
by the explicit `work_decision` suffix:

| Filesystem observation | Work decision           |
| ---------------------- | ----------------------- |
| Missing or empty       | `bootstrap_unavailable` |
| Existing unproved      | `existing_unproved`     |
| Unsafe path            | `unsafe_path`           |
| Inspection I/O failure | `inspection_io_failure` |

The gate never opens Fjall, migrates coins/schema, initializes history, recovers
state, resumes pruning, writes BASIC records or changes live authority. Selected
profiles still report missing `canonical_bootstrap`, `exact_work` and
`candidate_history` (plus Signet challenge validation). Phase 165's numeric and
live-fixture foundation does not supply those ordinary canonical prerequisites.
Preserve existing data and review it separately. A separate datadir also refuses
until the missing prerequisites are implemented and verified.

[Plan 10 evidence](../../.planning/phases/165-shared-exact-work-authority/165-10-SUMMARY.md)
separates six new physical/effect controls from 13 reused original cases in the
19-case module; they are not 19 independent new controls. The original preservation
source remains attributable to Git blob `0345bab1bfbd76be3078453a6b9201fdf73941cc`
(20,556 bytes, SHA-256 `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`).
Its current 20,688-byte source has only the mechanical carrier migration, SHA-256
`e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`.

[Plan 11](../../.planning/phases/165-shared-exact-work-authority/165-11-SUMMARY.md)
records one normal Open Bitcoin daemon integration test with 32 actual children
(four selectors × four source shapes × metrics off/on), three configured factory
tests and six CLI cases. These are real local refusal processes and adapter
evidence, not an executed pinned-Knots qualification matrix. Closed manifests,
backend-builder controls and unchanged source/destination bytes bound the claims.

## Intentional internal differences and remaining ownership

Knots `GetBlockProof` maps invalid compact targets to zero. Open Bitcoin's
`block_proof` returns a typed decode `Result`; admission refuses invalid targets.
`proof_equivalent_time` also returns typed errors for invalid spacing or zero best
proof. These internal error contracts do not add a cumulative-overflow refusal or
an external qualification claim. Ordinary refusal is an intentional current
product difference from Knots' canonical startup.

Canonical bootstrap and exact checkpoint/all-handle reopen acceptance remain
deferred to Phase 166, including genesis BASIC. Contextual consensus completeness
remains deferred to Phase 167. Actual pinned-daemon differential qualification
remains deferred to Phase 168. No canonical bootstrap or reopen acceptance is
claimed. No whole-chain, production-service, production-funds, public-default
networking or deterministic restart ordering claim follows from this work.

Historical synthetic roots, u128 source IDs, receipt bytes, hashes, dates and
outcomes remain preserved in [archive-safe provenance](v2-5-archive-safe-provenance.md).
Current exact carriers do not retroactively upgrade them. The
[native source guard and 837-case predecessor matrix](../../.planning/phases/165-shared-exact-work-authority/165-12-SUMMARY.md)
bind reviewed source/test owners, independent expectations and historical receipts;
source checks alone do not execute Rust. [The current guard](../../scripts/check-phase165-shared-exact-work.ts)
also checks this page for overclaims. Complete source breadcrumbs remain in
[the attribution inventory](source-breadcrumbs.json).

[Plan28](../../.planning/phases/165-shared-exact-work-authority/165-28-SUMMARY.md)
records the actual combined post-close public normal100 proof;
[Plan32](../../.planning/phases/165-shared-exact-work-authority/165-32-SUMMARY.md)
retains actual full daemon/coins consumer execution; and
[Plan31](../../.planning/phases/165-shared-exact-work-authority/165-31-SUMMARY.md)
records genuine1002-block Dense/root-inclusive undo accounting, all original
Phase127 socket/RPC outcomes and precise missing-body/physical-identity refusals.
Its separately executed Plan31 checkpoint matrices passed1012 Phase164 and1934 Phase165
cases under a closed658-entry fixed read window. These remain scoped repair
receipts, not a replacement for the final default native/review/formal gates.
The original35-item historical CFIX evidence set and every archived receipt retain
their original source, date, count, maturity and outcome scope.

[Plan30](../../.planning/phases/165-shared-exact-work-authority/165-30-SUMMARY.md)
closes the parent-only preparation regression with an expectation-only
UntrustedWorkLineage RED, then exact second-input MissingCoin GREEN on the same
genuine closed-flush core. Its full 14-case module, strict Node checks and 37
focused source/owner/inventory cases passed under an unchanged 709-entry window.
At the dated Plan30 checkpoint, the guard inventory was 158 source owners, 88
executable test owners and 277 fixed inputs; the Plan31 matrix counts above are predecessor
receipts. Full final native, code/security and formal acceptance remain separate.

[Plan34](../../.planning/phases/165-shared-exact-work-authority/165-34-SUMMARY.md)
repairs a source-expectation mismatch reached by the first final default verifier.
Four live observability comparisons now require canonical 64-digit hexadecimal
work, while complete raw support/smoke fixture rows keep their decimal values.
All eleven original Phase73 tests remain, with six added cases independently
checking four live fields and two complete raw rows. The six-case RED was 2/4 and
the full-file RED 13/4; GREEN passed 6/0 and 17/0 and the ordinary Phase72 CLI
passed. These are overlapping scoped receipts, not aggregate native coverage.

The current post-34 guard inventory is 160 source owners, 88 executable Rust test
owners and 279 ordered fixed inputs. Both new TypeScript sources have ordinary
source ownership; no Rust TEST row was invented. Twelve proportional commands,
including four actual new-owner controls, closed all 4,829 declared raw entries.
The declared static entry union is 39 modules, current CLI 14; Phase72's 30 roots
cover 133 source-corpus files. This is not an all-child or compiler graph. The
first full native attempt remains exit 1; the entire final default verifier,
whole-phase code/security and formal acceptance still require actual completion.

## Current source-guard repair checkpoint

The later [Plan 35](../../.planning/phases/165-shared-exact-work-authority/165-35-SUMMARY.md)
preserves an exact source-table relocation. [Plan 36](../../.planning/phases/165-shared-exact-work-authority/165-36-SUMMARY.md),
[Plan 37](../../.planning/phases/165-shared-exact-work-authority/165-37-SUMMARY.md),
[Plan 38](../../.planning/phases/165-shared-exact-work-authority/165-38-SUMMARY.md) and
[Plan 39](../../.planning/phases/165-shared-exact-work-authority/165-39-SUMMARY.md)
repair current source-policy seams with independent semantic controls, genuine
fixture-helper evidence and exact historical restoration. Current 164/165 gates
bind supplied adapter generations before imported source classification.
Original manifests, snapshots and receipts retain their original identities.

At the accepted Plan 39 checkpoint, the inventory is 190 source owners, 88 executable
Rust test owners and 307 ordered fixed inputs. Plan 39's full 159/161/history runs
passed 654 / 149 / 118 cases; its separate 164/165 selections passed 180 / 68, including 31
copied CLI controls and seven nonempty 165 evidence families. These overlapping
source-policy scopes are not Rust execution or summed native coverage. Both
earlier entire native attempts remain failures at Phase 72 and Phase 73; later
Rust, fresh coverage and Bazel were not reached. Actual final execution is
recorded separately in the [acceptance evidence](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md).
Source passes do not supply ordinary canonical bootstrap/reopen, contextual
completeness or later pinned-daemon qualification.

## Accepted Plan 40 source checkpoint

[Plan 40](../../.planning/phases/165-shared-exact-work-authority/165-40-SUMMARY.md)
repairs the copied missing-root loader boundary with complete bounded executable
copying and explicit evidence-only fixture membership. The strict diagnostic and
all 900 original reorg cases remain. Shared PASSED-generation qualification and
two complete copier inverses retain the existing historical restoration chain;
unknown historical inputs remain unmodified until qualification refuses them.
No production Rust capability was added by these source-policy repairs.

Its actual 101-command final closure passed complete Phase158/162/native163/history
files with 900 / 2620 / 160 / 203 cases and separate selected164/165 scopes with
169 / 68. All 85 eligible source CLIs passed with their actual wrappers and flags.
The actual native registry contains 87 CLIs; benchmark-report and Bazel-provenance
are the exact two generated-prerequisite exclusions. The old 81-executed subset
remains incomplete, and overlapping test selections are not summed.

The accepted inventory is now 192 source owners, 88 executable Rust test owners
and 307 ordered fixed inputs, retaining pin
`625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb`.
Root closed all 7,693 declared raw states and retained earlier failures, including
the first Plan40 898-pass/two-failure evidence-membership replay. Prior checkpoint
counts above retain their dated scopes.

The third entire default native attempt failed at the Phase158 copied loader with
899 passes and one failure. All three native attempts remain failed receipts;
later Rust, fresh coverage and Bazel were not reached. Actual R4 preflight and the
entire fresh default native run remain separate, as do final code/security/formal
acceptance and normal-hook finalization. Instrumentation smoke supplies no native
coverage claim. The [acceptance evidence](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md)
records exact outcomes and identities. Ordinary canonical startup/reopen still
refuses all four selectors, networking stays default off, and Phase166/167/168
ownership and qualification limits remain unchanged.

## Accepted Plan 41 independent history diagnostic checkpoint

[Plan 41](../../.planning/phases/165-shared-exact-work-authority/165-41-SUMMARY.md)
restores the independent manifest/history diagnostic required by the unchanged
four-owner regression. The existing historical data-view check now runs even when
the shared current copier-generation adapter refuses. Its result grants no
current-classifier credit: the adapter and historyBound gate remain unchanged.
Only the existing Phase165 entrypoint and its one ordinary ownership row change.
Historical snapshots, manifests, inverses, copiers, test assertions and production
Rust retain their complete reviewed bytes.

The actual four-owner regression passed 4 cases and 96 assertions. Complete
Phase165, Phase164 main and historical files passed 1966 / 1179 / 203 cases with
7351 / 2787 / 661 assertions respectively. All 94 bounded commands exited zero,
including all 85 eligible source CLIs in actual native order. Root authenticated
8,978 declared canonical states and accepted the actual summary. These separate
source-policy scopes are not summed native coverage. The inventory remains
192 source owners, 88 executable Rust test owners and 307 ordered fixed inputs,
with the same ordered pin recorded above.

The fourth entire default native attempt permanently remains failed: 1965 passes,
one failure and 7351 assertions in its Phase165 matrix, actual exit 1 after
4,088,320 ms. Its earlier 85-CLI preflight passed on that generation; later Rust,
benchmark, fresh coverage and Bazel stages were not reached. The missing
manifest-attributed diagnostic prompted Plan41; none of R1–R4 is relabeled by the
later source repair. Synthetic instrumentation smoke supplies no native coverage.

The [acceptance evidence](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md)
retains exact failed receipts, source-first/ordinary-row chronology and the
accepted Plan41 closure. An entire fresh default native R5, actual fresh
eight-crate coverage, final code/security/formal/lifecycle review and normal-hook
main-branch finalization remain pending. Phase165 is incomplete and Phase166 is
unstarted. Ordinary canonical startup/reopen still refuses all four selectors,
networking remains default off, and later166–168 qualification limits remain
unchanged.

## Full default native execution

The actual full default repository verifier completed with exit 0 on pinned
Bun 1.3.9 and Rust 1.94.1 after the accepted Plan41 source repair. Its internal
duration was 5,872,826 ms. The 76 single-file Bun run summaries contain 11,130
passes and zero failures; repeated execution scopes are not unique-case totals.
All Rust result occurrences succeeded, with six baseline ignored entrypoints
retained and no new Phase165 ignored test. Benchmark, Bazel/provenance and the
fresh eight-crate llvm coverage stage were reached. The retained text report
contains no Uncovered Lines: section; no aggregate percentage is inferred.

The [acceptance record](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md)
keeps the actual outcome and original false capture-postprocessor validation
separate. That error is a tuple/JSON-array comparison in evidence postprocessing;
Root separately accepted the complete actual native/input/output/FD/report
closure through an independently reviewed data-only audit. The original false
validation and all earlier failed attempts retain their original outcomes.
[Plan 14](../../.planning/phases/165-shared-exact-work-authority/165-14-SUMMARY.md)
records this native evidence and its scope. Phase status follows the canonical
verification records linked from the acceptance evidence. All four ordinary
selectors still refuse, networking stays default off, and later166–168
capabilities remain unclaimed.

## Phase165 bounded acceptance (2026-10-18)

All41 plans are complete. The full default native run and independent capture audit support CWOR-01, CWOR-02, CWOR-03 and CWOR-05. Canonical [code review](../../.planning/phases/165-shared-exact-work-authority/165-REVIEW.md), [security verification](../../.planning/phases/165-shared-exact-work-authority/165-SECURITY.md) and [goal verification](../../.planning/phases/165-shared-exact-work-authority/165-VERIFICATION.md) record zero unresolved findings,184 closed threat dispositions and all146 plan truth entries plus the four mandatory roadmap criteria.

Only those four requirement IDs are closed here. CWOR-04 and ordinary canonical bootstrap/reopen remain Phase166 work; contextual completeness and actual pinned-daemon differential qualification remain with167–168. All four ordinary selectors still refuse and networking stays default off. This record precedes its own normal-hook Git transaction; repository history and its actual hook outcome establish saving and push.

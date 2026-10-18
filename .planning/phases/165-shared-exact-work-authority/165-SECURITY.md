---
phase: 165-shared-exact-work-authority
status: secured
verified: "2026-10-18T12:53:58.820505+00:00"
threats_total: 184
threats_closed: 184
threats_open: 0
threats_mitigate: 182
threats_accept: 2
threats_transfer: 0
unregistered_flags: 0
asvs_level: 1
block_on: open
---

# Phase 165 — Security Verification

**SECURED: all 184 declared threats are CLOSED by disposition: 182 verified mitigations and the same two explicitly planned accepted risks. No transfer or new risk acceptance is introduced.**

This report verifies the Phase165 threat register against the current implementation and authenticated actual evidence. It does not certify later-phase behavior, confer ASVS compliance, or claim that the future normal-hook Git transaction has already occurred.

## Review scope and method

All 41 plan identities match the completed prior security review. Independent register reconciliation found 184 declared IDs and 184 traced IDs, with no omission or extra threat: 175 explicit disposition rows (173 mitigate, two accept) and nine mitigation/evidence rows in Plans38/39 (four/five) explicitly classified as mitigate. IDs T-165-32-04/05 retain their names when their tasks move to Plan33; no ID is counted twice. Each row below records its category, disposition, expected mitigation, current source anchor and CLOSED result.

The prior complete-body review is `.local/open-bitcoin-dev/phase165-final41-security-prepass/SECURITY.md`, SHA256 `af850bd11848e8058388afff10e3d90a3b9cb68b91e6b1310319fdaec262ed7b`. This continuation independently rehashed all 335 reviewed source states: zero drift, current source mapping `7354f4d8a5dd9378a7d2fde02e8dc975cd708fb2fbc59324b24ce03cd3c4345e`. All 184 source-evidence anchors match; three displayed snippets normalize source backticks to single quotes for Markdown readability (13-02,29-01,41-04), with actual source and line anchors unchanged. Prior scoped review/execution records retain their original attribution; fresh native R5 is separately qualified below.

Repo-local AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and relevant architecture, verification, testing, Rust and TypeScript standards materially informed this review. Both active lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens; no lesson archive was loaded. The applied controls include parsed authority boundaries, pre-effect guards, fixed bounded regular-file readers, source-first ownership review, exact historical inverses, redacted diagnostics and truthful source-versus-runtime claims.

Secure-phase configuration is `asvs_level:1`, `block_on:open`. Level1 supplies scoped mapping vocabulary only; no web application, authentication/session mechanism or ASVS certification is claimed. This phase supports CWOR-01/02/03/05. CWOR-04 and Phases166–168 remain later work. Ordinary networking remains default-off, and all four ordinary fixture-selector refusals remain required.

## Accepted actual native evidence

The entire default `bash scripts/verify.sh` R5 ran as PID87342 from 2026-10-18T10:55:11.685402Z and actually exited zero at 12:33:05.263120Z. Durations are 5,873,552ms at the launcher, 5,873,529ms in the normal timing wrapper and 5,872,826ms in the verifier's terminal line. The accepted run includes Rust formatting checks, Clippy, all-target build, workspace tests/doctests, CLI/benchmark/report checks, Bazel smoke/provenance and fresh llvm coverage for all eight pure-core crates. It was the full default verifier, with no fast/filtered replacement, native waiver or hook bypass.

The retained native log contains 76 Bun summary occurrences: 11,130 passes and zero failures. All 59 Rust summary occurrences report zero failures; repeated coverage execution is not presented as a deduplicated unique-test total. Six pre-existing ignored Rust cases remain explicitly attributed to public-network opt-in, measurement and bounded child-process controls. This report does not imply those ignored bodies were independently run as ordinary tests or broaden fixture evidence into public Bitcoin network qualification.

Root's comprehensive data-only auditor and acceptance authenticate all 9,504 native input states in three complete before/after/current frames: 28,512 exact Raw proof rows, zero source or generated LOC deltas, all ten declared actual outputs, timing, native outcome, and the report's recorded writer/FD binding. This security continuation rechecked all 28,512 saved pair equalities and their counts, hashed all 30 unique retained input/output reference buffers, and rehashed the 335 current source states. The verified native closure is `d464b6d9…`; Root final native acceptance is `abc23902…` (complete identities below).

The fresh actual coverage report is 2,431,082 bytes, SHA256 `44107b740baee0fe758c585da40dec865005174d85a09082a6005178ae59bcf3`, with 161 annotated file sections across all eight declared crates and no `Uncovered Lines:` section. Binding records authenticate actual writer/native PID births, argv and ancestry, regular FD1, same device/inode, native completion, writer completion and complete same bytes retained across unlink. This is the actual completed run's report. No synthetic smoke result replaces it, and no percentage, aggregate table or 100% branch-coverage claim is inferred.

### Preserved postprocessor refusal

The original validation remains unchanged and false at SHA256 `31b559aec8578c1c792ebf5b8ad814b10f5b9f15215862512ab5d85b9d81253d`, with its sole `complete report/binding buffer identity mismatch` error. Independent diagnoses established that a Python equality comparison incorrectly compares decoded JSON lists with runtime tuples at `binding.plugin.argv` and `binding.contract_sources`. Every other report/binding predicate passes and the saved result's capture equals the saved receipt as canonical JSON.

A separately reviewed data-only audit checks the genuine retained native/source/output/timing/writer/report evidence and the normalized same-buffer JSON comparison. Root accepted that complete proof. No executed native tool, helper, source, failed validation, receipt or coverage report was rewritten, and the entire native command was not relabelled or synthetically rerun. The original postprocessing refusal stays visible; its serialization defect does not change the independently authenticated actual native exit0. Prior R1–R4 native failures remain failures under their own source generations and receipts.

## Procedural mitigation boundaries

T-165-14-02 is CLOSED by the configured strict wrapper and normal pre-commit contract. `.githooks/pre-commit` generates LOC from the index, stages the generated report and runs `OPEN_BITCOIN_LOC_REPORT_SOURCE=index bash scripts/verify.sh`. Root's actual commit/push remains a later authorized transaction after canonical code/security/formal/lifecycle gates. Closing this structural mitigation neither bypasses that transaction nor falsely claims it already happened; making its future receipt a precondition for its own mandatory security review would be circular.

T-165-41-04 is CLOSED by permanent failedR4 preservation, the independently accepted source-before-data two-owner repair, actual expected stale refusal, complete accepted94-command/1,723,968-row closure, unchanged full test identities and actual85 eligible source-CLI exits. T-165-41-05 is CLOSED by the accepted41 summary's bounded claims, the accepted entire nativeR5, explicit later-phase limits and retained mandatory independent final-review/lifecycle gates. Root retains ownership of final14 document publication and final metadata binding; this report makes no premature Phase165-complete or Git-success claim.

## Accepted risks log

### T-165-15-06 — Caller backend/startup errors

**Disposition: accept. Status: CLOSED by this canonical log.** Constructor consumes source; post-init concurrency/I/O errors can follow initializer effects. Do not promise rollback/zero effects or ABA/revision-history detection when all captured facts are identical. This is the exact existing bounded acceptance in Plan15:286. No new waiver, rollback promise or risk expansion is introduced.

### T-165-16-06 — Explicit local fixture policies

**Disposition: accept. Status: CLOSED by this canonical log.** Serving activates only through existing explicit arguments in a test runtime; default-off ordinary networking remains unchanged and no external network qualification is inferred. This is the exact existing bounded acceptance in Plan16:262. No public-network, ordinary-selector or unattended-daemon qualification follows.

## Per-threat verification

All 184 rows are classified and CLOSED by their declared disposition. Source anchors are considered with the complete body review, exact current binding and actual scoped execution/provenance records; a matching code string alone is not substituted for a required actual command receipt.

### Plan 01

Plan: [.planning/phases/165-shared-exact-work-authority/165-01-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-01-PLAN.md>); complete raw SHA256 `029d0a5ea923f601c3211d058f71cf438e95f14c9eae13a129d5a66182ee05ab`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-01-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-01-SUMMARY.md>); SHA256 `11e6e6b181ca2957aa54ac0d9933c5322ab506ceb9d4ebfcd6bc09a7a56f9d07`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-01-01 | T | mitigate | CLOSED | Normalize before flags and test zero/sign/overflow/powLimit/hash boundaries. | [packages/open-bitcoin-consensus/src/crypto.rs:130](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-consensus/src/crypto.rs:130>) — `pub fn compact_target_bytes(bits: u32) -> Result<[u8; 32], CompactTargetError> {` |
| T-165-01-02 | R | mitigate | CLOSED | Fixed source-derived expected corpus; no helper-generated expectations or daemon qualification. | [packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs:11](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs:11>) — `fn cases(section: &str, next_section: &str) -> Vec<&'static str> {` |

### Plan 02

Plan: [.planning/phases/165-shared-exact-work-authority/165-02-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-02-PLAN.md>); complete raw SHA256 `da1b351ad008bdc8587bb4e0935fc802f5a8119c892046378647bfb6076326f4`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-02-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-02-SUMMARY.md>); SHA256 `e0adc62dde8ed3f6d24fb83d5ae927dc02e908e4bfe420e4649a6a1c58b86c5d`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-02-01 | S | mitigate | CLOSED | Separate legacy and exact numeric classification; neither decoder constructs authority. | [packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs:20](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs:20>) — `LegacySynthetic(u128),` |
| T-165-02-02 | T | mitigate | CLOSED | Fixed byte width/version validation and malformed/mixed-format controls. | [packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs:104](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs:104>) — `fn check_version(version: u32) -> Result<(), &'static str> {` |

### Plan 03

Plan: [.planning/phases/165-shared-exact-work-authority/165-03-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-03-PLAN.md>); complete raw SHA256 `1195ae03103bfe8433215961fdde2bb5cc205a75b40d59afac16717c6581ea69`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-03-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-03-SUMMARY.md>); SHA256 `427814cc7dad82a45d7384b788b8d263c7b9cf49eff5f50c46340e42b8689073`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-03-01 | E | mitigate | CLOSED | Explicit untrusted fixture/decode boundaries; width is never accepted provenance. | [packages/open-bitcoin-chainstate/src/engine/work_authority.rs:290](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/engine/work_authority.rs:290>) — `pub fn validate_live_origin(&self) -> Result<(), ChainstateError> {` |
| T-165-03-02 | R | mitigate | CLOSED | Scoped reviewed renewal plus new negative controls; historical manifests/receipts fixed. | [scripts/check-phase164-canonical-identity/contracts.ts:46](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:46>) — `export function maybeRestoreReviewedSource(source: string, inverse: ExactSourceInverse): string \| undefined {` |
| T-165-03-03 | T | mitigate | CLOSED | Exact bytes/hex and full workspace check; no truncation/automatic migration. | [packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs:276](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs:276>) — `fn phase165_exact_snapshot_and_chain_meta_preserve_high_work_limbs() {` |

### Plan 04

Plan: [.planning/phases/165-shared-exact-work-authority/165-04-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-04-PLAN.md>); complete raw SHA256 `a34ee6e05ec1c43bdef58d379a8f948c8283ce8c7eeb003aef25ac0fe4bb6d62`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-04-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-04-SUMMARY.md>); SHA256 `28133ffe8073de83009c6c01fca7ca907053241be39ca987e262b73b34e1419f`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-04-01 | S | mitigate | CLOSED | Accepted lineage separate from raw import; parent/header/hash/height controls. | [packages/open-bitcoin-network/src/header_store/proof.rs:22](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store/proof.rs:22>) — `pub fn maybe_proof_facts(&self, hash: BlockHash) -> Option<HeaderProofFacts> {` |
| T-165-04-02 | T | mitigate | CLOSED | Strict > work only; duplicates retain first admission. | [packages/open-bitcoin-network/src/header_store.rs:294](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store.rs:294>) — `.is_none_or(\|current\| candidate.chain_work > current.chain_work)` |

### Plan 05

Plan: [.planning/phases/165-shared-exact-work-authority/165-05-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-05-PLAN.md>); complete raw SHA256 `035918b91b8fa888d9f6de9a9231949c441398ee767c193773e504881c68d341`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-05-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-05-SUMMARY.md>); SHA256 `73d46460f59da43ba9deffb52bb67b4380988acc70bda54eacc0c08553b56706`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-05-01 | T | mitigate | CLOSED | Shared validity/powLimit/hash, canonical guard first, managed negative tests. | [packages/open-bitcoin-node/src/network/header_sync.rs:14](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/header_sync.rs:14>) — `/// Contextual fixture validation only; the managed owner checks canonical scope first.` |
| T-165-05-02 | E | mitigate | CLOSED | No mutable scope bypass; four-selector actual product refusal controls. | [packages/open-bitcoin-node/src/network/canonical_admission.rs:11](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/canonical_admission.rs:11>) — `consensus::{CanonicalChain, RootIdentityError, UnsupportedValidationScope},` |

### Plan 06

Plan: [.planning/phases/165-shared-exact-work-authority/165-06-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-06-PLAN.md>); complete raw SHA256 `14b92224f593707805e60521720ee058b780604c35062cd2e9e37a83a18bad41`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-06-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-06-SUMMARY.md>); SHA256 `d6e1c1d70bc6e24c01042445f047d4d11816d6c86a72fdccdea54ceaadd732e5`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-06-01 | E | mitigate | CLOSED | Owner-only checked-body minting; no public setters/import authority. | [packages/open-bitcoin-network/src/header_store/eligibility.rs:111](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store/eligibility.rs:111>) — `pub fn check_body_eligibility(` |
| T-165-06-02 | D | mitigate | CLOSED | Preserve existing finite caps, FIFO idempotence and explicit exhaustion. | [packages/open-bitcoin-network/src/header_store/eligibility.rs:276](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store/eligibility.rs:276>) — `return Err(invalid_header("body-eligibility-capacity"));` |
| T-165-06-03 | T | mitigate | CLOSED | Body/ancestor linkage sequence with opposing-arrival tests. | [packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs:71](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs:71>) — `fn phase165_eligibility_opposing_header_and_body_order_retains_earlier_active_tie() {` |

### Plan 07

Plan: [.planning/phases/165-shared-exact-work-authority/165-07-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-07-PLAN.md>); complete raw SHA256 `75b97282200631bf9e27c4ace64d4f0494163525c83f57f7dbc881ca0c659292`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-07-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-07-SUMMARY.md>); SHA256 `861881f5c414ac22bd8e254a45b34b7957ae937fafd8e415e7e78010ea47b798`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-07-01 | E | mitigate | CLOSED | Private earned lineage, sealed fields, origin/tip/generation validation, consume-once. | [packages/open-bitcoin-chainstate/src/engine/work_authority.rs:93](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/engine/work_authority.rs:93>) — `pub(super) fn check(` |
| T-165-07-02 | T | mitigate | CLOSED | Raw imports remain untrusted; pre-effect checks and full unchanged-state assertions. | [packages/open-bitcoin-chainstate/src/engine/work_authority.rs:241](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/engine/work_authority.rs:241>) — `pub fn rebind_into<W: CoinsView>(` |
| T-165-07-03 | E | mitigate | CLOSED | Live source only; complete identity checks; normal managed guards remain mandatory. | [packages/open-bitcoin-node/src/chainstate/validation_history.rs:210](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/validation_history.rs:210>) — `.validate_staged_connect(prepared.staged)?` |
| T-165-07-04 | T | mitigate | CLOSED | Explicit unsupported default, complete overlays/markers, coherent snapshot and existing writer/runtime serialization. | [packages/open-bitcoin-chainstate/src/coins.rs:49](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-chainstate/src/coins.rs:49>) — `fn complete_snapshot(&self) -> Result<CompleteCoinsSnapshot, ChainstateError> {` |

### Plan 08

Plan: [.planning/phases/165-shared-exact-work-authority/165-08-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-08-PLAN.md>); complete raw SHA256 `dcae8a98304c4250bf66e90f49b5a73564b11afd4d6a384de26a1a521402a114`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-08-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-08-SUMMARY.md>); SHA256 `94a24a6512c5ec5584a3f7c92cce2c27622451cd948f9d0efea4eeee98bedb7e`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-08-01 | E | mitigate | CLOSED | Central owner guard before first effect; lower-owner lineage/stage checks too. | [packages/open-bitcoin-node/src/chainstate/work_authority.rs:93](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/work_authority.rs:93>) — `pub(crate) fn prepare_authorized_reorg(` |
| T-165-08-02 | T | mitigate | CLOSED | Derive full suffix from retained earned ancestor; maximal-score tests. | [packages/open-bitcoin-node/src/network/work_authority.rs:196](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/work_authority.rs:196>) — `pub(crate) fn authorize_reorg(` |
| T-165-08-03 | R | mitigate | CLOSED | Before/after complete managed/coins/mempool/BASIC/durable spy evidence. | [packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs:16](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs:16>) — `fn phase165_work_authority_weaker_max_score_refuses_before_preview() {` |
| T-165-08-04 | T | mitigate | CLOSED | Immutable genuine linkage seal, lower accepted evidence, padded reseed/clone regressions before effects. | [packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs:8](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs:8>) — `fn phase165_work_authority_genuine_earlier_body_tie_survives_post_active_clone() {` |
| T-165-08-05 | T | mitigate | CLOSED | Pre-store saved/current active-identity guard; scheduled stale negative and matching-identity positive. | [packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs:134](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs:134>) — `fn phase165_work_authority_matching_basic_identity_can_resume_without_live_work() {` |
| T-165-08-06 | T | mitigate | CLOSED | Unique-strong registry epoch with opaque Weak grants, clone isolation and pre-effect freshness checks. | [packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs:8](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs:8>) — `fn phase165_work_authority_connect_grant_checks_epoch_before_preparation_and_commit() {` |

### Plan 09

Plan: [.planning/phases/165-shared-exact-work-authority/165-09-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-09-PLAN.md>); complete raw SHA256 `f88579c86f31ff3a1139ed15b8e69c8dcd7d16bcc91726af3c642f7b046a487f`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-09-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-09-SUMMARY.md>); SHA256 `78ba2dc4c1167cafb2ba30ce42fe1e14abb9711c25c75c4edfd59801ec1b59b4`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-09-01 | T | mitigate | CLOSED | Enumerate all eligible retained candidates with integration cases. | [packages/open-bitcoin-node/src/sync/block_reconcile.rs:179](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/block_reconcile.rs:179>) — `let mut candidates: VecDeque<_> = runtime.network.eligible_body_candidates()?.into();` |
| T-165-09-02 | D | mitigate | CLOSED | Bounded requests and required-data refusal; continue viable candidates without mutation. | [packages/open-bitcoin-node/src/sync/block_reconcile.rs:40](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/block_reconcile.rs:40>) — `pub(super) fn request_missing_blocks(` |
| T-165-09-03 | T | mitigate | CLOSED | Keep checked linkage separate from unknown arrival order; strictly greater work only and no duplicate/copy/descendant order promotion. | [packages/open-bitcoin-network/src/header_store/eligibility.rs:152](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store/eligibility.rs:152>) — `pub fn check_cached_body_eligibility(` |
| T-165-09-04 | T | mitigate | CLOSED | Genuine core plus exact fully flushed physical state and current actual proof; reject raw/mechanical/mismatch before mutation; no production opening. | [packages/open-bitcoin-node/src/chainstate/live_fixture.rs:90](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/live_fixture.rs:90>) — `pub(crate) fn attach_fixture_basic_owner_after_flush(` |
| T-165-09-05 | T | mitigate | CLOSED | Typed consensus-only recovery, immediate local failure, accepted-prefix progress and stop/reacquire semantics with actual fault/notification controls. | [packages/open-bitcoin-node/src/sync/block_reconcile.rs:216](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/block_reconcile.rs:216>) — `Err(ActivationFailure::Fatal(error)) => return Err(error),` |
| T-165-09-06 | D | mitigate | CLOSED | Actual accepted proof-facts-only scheduling through read-only owner accessor; raw seed excluded with real outbound/budget controls. | [packages/open-bitcoin-node/src/network/candidate_eligibility.rs:58](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/candidate_eligibility.rs:58>) — `.maybe_proof_facts(block.header.previous_block_hash)` |

### Plan 10

Plan: [.planning/phases/165-shared-exact-work-authority/165-10-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-10-PLAN.md>); complete raw SHA256 `e145093d346d12de05356c740e2c72a0272d276dff3b7aadc5f7e008929e4586`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-10-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-10-SUMMARY.md>); SHA256 `e46fe320198bd44862dd17c6ea775e2eba818da293ffcd67fb28061a904e2231`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-10-01 | T | mitigate | CLOSED | Filesystem-only always-refuse ordinary gate; builder spy plus closed manifests. | [packages/open-bitcoin-node/src/storage/chain_compatibility.rs:15](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/chain_compatibility.rs:15>) — `pub fn open_ordinary(` |
| T-165-10-02 | S | mitigate | CLOSED | Observed unavailable; separate typed unproved/refused decision. | [packages/open-bitcoin-node/src/storage/chain_compatibility.rs:164](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/chain_compatibility.rs:164>) — `pub const fn work_profile(&self) -> WorkProfileObservation {` |
| T-165-10-03 | I | mitigate | CLOSED | Fixed categories/selected facts, bounded \<1024 bytes, hostile-input redaction tests. | [packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs:21](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs:21>) — `assert!(diagnostic.len() < 1024);` |

### Plan 11

Plan: [.planning/phases/165-shared-exact-work-authority/165-11-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-11-PLAN.md>); complete raw SHA256 `6fe3a71bd3559343d776e410beb31e7b47203665c86a10f9e2745cd3fcd338b9`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-11-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-11-SUMMARY.md>); SHA256 `13db987c862ebae4330130ca6381ac8a7c06cc12342b0c4c6ae6f0f48b445b47`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-11-01 | E | mitigate | CLOSED | Real daemon/operator factory tests and sensitive-edge negative controls. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs:11](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs:11>) — `fn assert_work_diagnostic(message: &str, selected: &str, decision: &str) {` |
| T-165-11-02 | I | mitigate | CLOSED | Bounded shared diagnostic, secret/path/payload sentinels. | [packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs:12](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs:12>) — `pub(crate) fn forged_source(shape: &str) -> SourceFixture {` |

### Plan 12

Plan: [.planning/phases/165-shared-exact-work-authority/165-12-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-12-PLAN.md>); complete raw SHA256 `e2ff6f3f02059b9ed8620bc4d453732017be7ded30b246ea5e67a1c6eff55539`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-12-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-12-SUMMARY.md>); SHA256 `bd6a50e8da9a1275fd94d00696f2ecb02bf6866393c55f63ecd99299c9b6d4e8`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-12-01 | R | mitigate | CLOSED | Real loaded-source negative controls; exact registration; execution evidence separately required. | [scripts/check-phase165-shared-exact-work.ts:192](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:192>) — `function checkEnabledTests(text: (file: string) => string, fail: Fail): void {` |
| T-165-12-02 | T | mitigate | CLOSED | Bounded fixed regular paths and immutable historical view. | [scripts/check-phase165-shared-exact-work.ts:36](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:36>) — `function readBounded(file: string): string {` |

### Plan 13

Plan: [.planning/phases/165-shared-exact-work-authority/165-13-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-13-PLAN.md>); complete raw SHA256 `2b9f14dcb8e1f1d633926019905c6203dd7572a57c0b199545be3336e325f41c`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-13-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-13-SUMMARY.md>); SHA256 `cbdbfe56fc85038a959605e9ac66f84e90c731d8f31bd2627a52746de6bb59f1`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-13-01 | R | mitigate | CLOSED | Explicit formula/source versus real execution labels;166/167/168 ownership and bounded ties. | [docs/parity/v2-6-shared-exact-work.md:48](</Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/v2-6-shared-exact-work.md:48>) — `No actual pinned-Knots daemon differential qualification is claimed.` |
| T-165-13-02 | I | mitigate | CLOSED | Sanitized typed examples only, no source path/credential dumps. | [docs/operator/runtime-guide.md:19](</Users/peterryszkiewicz/Repos/open-bitcoin/docs/operator/runtime-guide.md:19>) — `'existing_unproved' for existing unproved sources, 'unsafe_path' for unsafe paths,` |

### Plan 14

Plan: [.planning/phases/165-shared-exact-work-authority/165-14-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-14-PLAN.md>); complete raw SHA256 `e8a33d5c4ed99980fd1c2d1a96406de66beb36fd0855184ec50d563daa3c0983`.

No SUMMARY exists at capture; completion is pending.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-14-01 | R | mitigate | CLOSED | Actual complete default verifier exit/test counts/coverage/Bazel logs; no --fast or fake receipts. | [scripts/verify.sh:438](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/verify.sh:438>) — `run_step "cargo llvm-cov pure core" run_coverage_report`; accepted actual R5 native closure `d464b6d9…` and Root native acceptance `abc23902…` below. |
| T-165-14-02 | E | mitigate | CLOSED | Normal hooks and wrapper-only commits after clean verification/review/security. | [.githooks/pre-commit:10](</Users/peterryszkiewicz/Repos/open-bitcoin/.githooks/pre-commit:10>) — `OPEN_BITCOIN_LOC_REPORT_SOURCE=index bash scripts/verify.sh`; structural normal-hook/wrapper gate verified; actual Git transaction remains future. |

### Plan 15

Plan: [.planning/phases/165-shared-exact-work-authority/165-15-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-15-PLAN.md>); complete raw SHA256 `de54b38f05b8dd0a4cca02d100ff36c884ea823710017a7c19d6b07778de63a3`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-15-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-15-SUMMARY.md>); SHA256 `e913fb17a7534ea2003bc9864771304542eeb86ed43e510b623df742a6c26b57`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-15-01 | S/E | mitigate | CLOSED | Actual private origin/full-parameter binding, no raw reconstruction, no separate destination, no canonical owner input, all-four canonical-root refusal before initialization. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:55](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:55>) — `fn pure_source_gate(` |
| T-165-15-02 | T | mitigate | CLOSED | Existing engine replays full actual retained prefix into independent Memory state; compare independent/source/physical coins+B+heads/meta/undo/counts/body/validation facts. Missing history refuses; masked and public clean-cache alias controls. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:133](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:133>) — `let mut replay = Chainstate::new();` |
| T-165-15-03 | E/T | mitigate | CLOSED | SAME owned core through existing normal initialize/from_recovered/owner path; count and attachment/adoption helpers remain cfg-test; explicit fixture parameters throughout. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:95](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:95>) — `let managed = ManagedChainstate::from_recovered_chainstate(sink, core, lifecycle)?;` |
| T-165-15-04 | T/D | mitigate | CLOSED | Pure genuine/params/all-four canonical gate before reads; full replay capture before initializer; release/reacquire publication lock; final check against captured immutable content. Actual stage interleave plus separately labeled source-order controls; no delegate reuse. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:88](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:88>) — `let captured = capture_live_fixture_source(&source, consensus_params)?;` |
| T-165-15-05 | R/I | mitigate | CLOSED | Fixed bounded regular-file inventory, actual owner-attributed mutations/nonzero external tests, immutable historical hashes and six default registrations retained. | [scripts/check-phase165-shared-exact-work.ts:36](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:36>) — `function readBounded(file: string): string {` |
| T-165-15-06 | D | accept | CLOSED | Constructor consumes source; post-init concurrency/I/O errors can follow initializer effects. Do not promise rollback/zero effects or ABA/revision-history detection when all captured facts are identical. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:77](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:77>) — `/// Initial source refusals precede startup. Later startup/concurrent I/O errors`; local accepted-risk log above; canonical log pending.; canonical accepted-risks log below. |
| T-165-15-07 | S/E/T | mitigate | CLOSED | Validated captured headers once, checked unique root, cached/unknown-order historical nonroots, no accepted-tip binding mint; actual populated extension/unknown-equal refusal and NEW A/B preference controls. Final captured identity check follows graph before handle publication, with no publication lock across graph/readiness mutation. | [packages/open-bitcoin-node/src/sync/live_fixture.rs:247](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/live_fixture.rs:247>) — `pub(super) fn initialize_live_fixture_source_graph(` |

### Plan 16

Plan: [.planning/phases/165-shared-exact-work-authority/165-16-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-16-PLAN.md>); complete raw SHA256 `23457ba216ffca1733c82f7e8b6b91ca4c153ffb235a018a9603a5025ab425d2`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-16-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-16-SUMMARY.md>); SHA256 `fae1781a607a47753afb2e442150eb197c204a0f7b8eaf165465ff49448c42c6`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-16-01 | S/R | mitigate | CLOSED | Genuine empty Disabled source, no preconnected genesis, explicit root outside P2P, actual final tip/work/counters/persistence and repeated execution. | [packages/open-bitcoin-bench/src/cases/sync_runtime.rs:47](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-bench/src/cases/sync_runtime.rs:47>) — `let runtime = DurableSyncRuntime::from_live_fixture_source(` |
| T-165-16-02 | T/E | mitigate | CLOSED | Full maturity1 params, real managed genesis/flush, original live owner, checked pending B400 before A children, accepted A400 binding, same-height equal derived work and no score/order setters. | [packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:147](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:147>) — `fn open_basic_source(` |
| T-165-16-03 | S/E | mitigate | CLOSED | Move legitimate mutation before final drop, preserve raw read-only/physical evidence and explicit unchanged-state work mutation refusal; no replay restoring ties. | [packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:226](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:226>) — `pub(super) fn assert_raw_reopen_refuses_work(runtime: &open_bitcoin_node::DurableSyncRuntime) {` |
| T-165-16-04 | T | mitigate | CLOSED | Preserve serving positive, actual prune20 body+undo absence, wallet refusal, NotFound/no DurableBlock and immutable displaced row before/after all-handle closure; never restore inputs. | [packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs:489](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs:489>) — `open_bitcoin_node::ManagedInboundResponsePlanItem::Immediate(WireNetworkMessage::NotFound(` |
| T-165-16-05 | R/I | mitigate | CLOSED | Actual loaded owner-attributed mutations and positive baselines, enabled tests, fixed bounded input pin, unchanged original identities and six native registrations. | [scripts/check-phase165-shared-exact-work/test-support.ts:31](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/test-support.ts:31>) — `["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "empty_runtime", "DurableSyncRuntime::from_live_fixture_source(", "DurableSyncRuntime::open("],` |
| T-165-16-06 | D | accept | CLOSED | Serving activates only through existing explicit arguments in a test runtime; default-off ordinary networking remains unchanged and no external network qualification is inferred. | [packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:204](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs:204>) — `DurableSyncRuntime::from_live_fixture_source(`; local accepted-risk log above; canonical log pending.; canonical accepted-risks log below. |

### Plan 17

Plan: [.planning/phases/165-shared-exact-work-authority/165-17-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-17-PLAN.md>); complete raw SHA256 `7734e64842e7f5b02924f260c64af73a17703b91dbead52e65c75b7f31fbcdbf`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-17-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-17-SUMMARY.md>); SHA256 `01bb0f81d2e709510054a4937507a9ffd9a638ed7eb74279905a0d4be580fab9`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-17-01 | S/E | mitigate | CLOSED | Tasks1/2 construct genuine actual prefixes; preserve raw origin refusal and full explicit params. | [packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:36](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:36>) — `Chainstate::new()` |
| T-165-17-02 | T/E | mitigate | CLOSED | Task2 reaches actual acceptance before pending capture; raw rows cannot become ScriptsValid or Pending. | [packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:33](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:33>) — `Default::default(),` |
| T-165-17-03 | T | mitigate | CLOSED | Only declared lock caller uses existing reopen_live_fixture with actual prefix/count/physical equality; ordinary reopen untouched. | [packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:21](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs:21>) — `let dir = std::env::temp_dir().join(format!(` |
| T-165-17-04 | R | mitigate | CLOSED | Task3 renews reviewed changed owners with actual loaded mutations; immutable historical bytes retained. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-17-05 | D | mitigate | CLOSED | Four/five-file bounded source tasks, cap628 and no29-consumer fanout. | [packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs:576](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs:576>) — `bytes[134..166].copy_from_slice(header.as_bytes());` |

### Plan 18

Plan: [.planning/phases/165-shared-exact-work-authority/165-18-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-18-PLAN.md>); complete raw SHA256 `cc60412c879cf0e74dddba973b8ebbfff7ae4b046cfee3dc1a356fd286489521`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-18-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-18-SUMMARY.md>); SHA256 `a5f3742b38b6992f7e380bb04ebfb9891308b6e5b95f73bd85f88b33cf9fc683`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-18-01 | S/E | mitigate | CLOSED | Task1 actual selected old-prefix body acceptance and SAMEcore assembly precede faults; raw clone remains raw. | [packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs:307](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs:307>) — `let runtime = DurableSyncRuntime::from_live_fixture_source(` |
| T-165-18-02 | T | mitigate | CLOSED | Fault arming follows full fixture assembly; existing counters and typed first-effect assertions prove intended trigger. | [packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs:18](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs:18>) — `mod rpc_faults;` |
| T-165-18-03 | T | mitigate | CLOSED | Task2 preserves accepted20..400, durable19 and actual paired-delete/Periodic/Always controls. | [packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs:18](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs:18>) — `mod rpc_faults;` |
| T-165-18-04 | S/I | mitigate | CLOSED | Task2 preserves NeverConnected/legacy facts, physical snapshots and raw all-handle observations. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-18-05 | R | mitigate | CLOSED | Task3 real positive baseline plus attributed raw-owner/fault-placement/pressure-source mutations. | [scripts/check-phase164-canonical-identity/contracts.ts:2](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:2>) — `* they do not claim runtime execution or canonical qualification. Changes require review.` |

### Plan 19

Plan: [.planning/phases/165-shared-exact-work-authority/165-19-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-19-PLAN.md>); complete raw SHA256 `87c0448c047af15e0f5df37c138e60e5a9f96fb75040c2150c96be8acabbec03`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-19-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-19-SUMMARY.md>); SHA256 `f3738aca433cb6ac61dbac222055a3e40f9f0bfe3f24a1b10ac0b00ea64b2277`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-19-01 | S/E | mitigate | CLOSED | Task1 uses actual empty→Fjall managed genesis/flush and public seven-argument constructor; no Node cfg-test API exposure. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs:205](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs:205>) — `Chainstate::new()` |
| T-165-19-02 | T | mitigate | CLOSED | Task2 requires two additional complete exact fragments plus prior exact import/context inverse and unchanged original whole-source SHA; reject partial/mixed/unknown edits. | [packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs:76](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs:76>) — `..Default::default()` |
| T-165-19-03 | R/E | mitigate | CLOSED | Tasks2/3 independently pin actual current adapter in both guards; keep raw/current hash distinct, historical input map copied, original manifests/receipts/snapshots unchanged. | [packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs:61](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs:61>) — `.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")),` |
| T-165-19-04 | E | mitigate | CLOSED | Task2 returns only path/recipe/provenance after actual handle close; original physical denied-family effects remain and raw reopens cannot mutate. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-19-05 | D/T | mitigate | CLOSED | Shared recognized tests/support helper with immediate breadcrumb/current ownership, \<=628 files, unique present anchors and fixed read-set/loader/registration regressions. | [scripts/check-phase164-canonical-identity/contracts.ts:2](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:2>) — `* they do not claim runtime execution or canonical qualification. Changes require review.` |

### Plan 20

Plan: [.planning/phases/165-shared-exact-work-authority/165-20-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-20-PLAN.md>); complete raw SHA256 `fdd1c28a5bd32efb711911638ec28c426f61872b70daee504a83df774dfe9957`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-20-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-20-SUMMARY.md>); SHA256 `76612c45216136000bdaf8df2c43988fb1231d86cf694029b73ee0a8d9e379a2`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-20-01 | S/E | mitigate | CLOSED | Task1 validates real entire prefix under original maturity1; raw serving admission remains header-only. | [packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs:130](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs:130>) — `let mut state = Chainstate::new();` |
| T-165-20-02 | T/E | mitigate | CLOSED | Task2 explicit actual fork bodies arrive before original suffix on same owner; no copied sequence/seal or universal preference. | [packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs:113](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs:113>) — `Default::default(),` |
| T-165-20-03 | T | mitigate | CLOSED | Exact13→15 body-count premise gives actual greater work; original wake/latch/physical assertions remain. | [packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs:113](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs:113>) — `Default::default(),` |
| T-165-20-04 | T/I | mitigate | CLOSED | No complete replay or header-registry replacement on already bound live owners; unknown cached order and removed payloads retained. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-20-05 | R | mitigate | CLOSED | Repair shared owners and two exact approved callers only;29 unchanged consumers stay unchanged, loaded mutations attributed. | [packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs:516](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs:516>) — `.canonicalize()` |

### Plan 21

Plan: [.planning/phases/165-shared-exact-work-authority/165-21-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-21-PLAN.md>); complete raw SHA256 `04c941c6568e6af8a700f8e3b36d4b77b49e93e83618f59c186203ac35fe9c9b`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-21-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-21-SUMMARY.md>); SHA256 `a8885ee6afe4e89aab386f52a8c3cec95abb06e3d577e13dec9f56c3a0d9c094`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-21-01 | S/T/E | mitigate | CLOSED | Use actual same-Fjall source/complete prefix; retain strict count/coins/B/heads/positions/undo binding and raw negatives. | [scripts/check-phase164-canonical-identity/contracts.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:11>) — `["        let config = SyncRuntimeConfig {\n            network: SyncNetwork::Regtest,\n            ..Default::default()\n        };\n        let params = config.network.consensus_params();\n        let recipe = recipe::Recipe::ne` |
| T-165-21-02 | T/R | mitigate | CLOSED | Real admitted root/child bodies and authorized operations drive exact original counters/projections; no fake receipts, generic error weakening or saved-tip substitution. | [packages/open-bitcoin-node/src/network/tests/runtime_projection.rs:20](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/runtime_projection.rs:20>) — `MemoryChainstateStore::default(),` |
| T-165-21-03 | I/D | mitigate | CLOSED | Preserve redaction, real resource budgets, pending/abort/join and liveness assertions; retain local storage failures as fatal. | [packages/open-bitcoin-node/src/network/tests/runtime_projection.rs:20](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/runtime_projection.rs:20>) — `MemoryChainstateStore::default(),` |
| T-165-21-04 | T/E | mitigate | CLOSED | Actual loaded-source mutations, enabled registration and exact input pins; no blanket allowlist/digest renewal. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-21-05 | R | mitigate | CLOSED | Keep four ordinary selectors refused/default-off and all166–168 ownership explicit; configured fixture evidence is accurately labeled. | [packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs:33](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs:33>) — `canonical_members: BTreeSet::new(),` |

### Plan 22

Plan: [.planning/phases/165-shared-exact-work-authority/165-22-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-22-PLAN.md>); complete raw SHA256 `c5ab47ea56879e6ca374a62a9282032041587010b69cd7c15d222e773aa71d8f`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-22-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-22-SUMMARY.md>); SHA256 `b0997b9995d3f5884139f93223a3956ff2d25fb2ed22ec8347ac9e74e95b27f6`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-22-01 | S/T/E | mitigate | CLOSED | Use actual same-Fjall source/complete prefix; retain strict count/coins/B/heads/positions/undo binding and raw negatives. | [packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:224](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:224>) — `.adopt_live_fixture_owner(open_bitcoin_core::chainstate::Chainstate::new())` |
| T-165-22-02 | T/R | mitigate | CLOSED | Real admitted root/child bodies and authorized operations drive exact original counters/projections; no fake receipts, generic error weakening or saved-tip substitution. | [packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:36](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:36>) — `let first = build_block(BlockHash::default(), 0);` |
| T-165-22-03 | I/D | mitigate | CLOSED | Preserve redaction, real resource budgets, pending/abort/join and liveness assertions; retain local storage failures as fatal. | [packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:36](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs:36>) — `let first = build_block(BlockHash::default(), 0);` |
| T-165-22-04 | T/E | mitigate | CLOSED | Actual loaded-source mutations, enabled registration and exact input pins; no blanket allowlist/digest renewal. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-22-05 | R | mitigate | CLOSED | Keep four ordinary selectors refused/default-off and all166–168 ownership explicit; configured fixture evidence is accurately labeled. | [scripts/check-phase164-canonical-identity/contracts.ts:2](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:2>) — `* they do not claim runtime execution or canonical qualification. Changes require review.` |

### Plan 23

Plan: [.planning/phases/165-shared-exact-work-authority/165-23-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-23-PLAN.md>); complete raw SHA256 `ea23f17c02c6cc532e7efad9a7c62337bf4d0b971cffe32fc853068b31619763`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-23-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-23-SUMMARY.md>); SHA256 `e409973b8623eab90b28bc854aaf91985b5e3f9df50113b432815f8b97b2d361`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-23-01 | S/T/E | mitigate | CLOSED | Use actual same-Fjall source/complete prefix; retain strict count/coins/B/heads/positions/undo binding and raw negatives. | [scripts/check-phase164-canonical-identity/contracts.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:11>) — `["        let config = SyncRuntimeConfig {\n            network: SyncNetwork::Regtest,\n            ..Default::default()\n        };\n        let params = config.network.consensus_params();\n        let recipe = recipe::Recipe::ne` |
| T-165-23-02 | T/R | mitigate | CLOSED | Real admitted root/child bodies and authorized operations drive exact original counters/projections; no fake receipts, generic error weakening or saved-tip substitution. | [packages/open-bitcoin-node/src/sync/tests/durable_tip.rs:23](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/durable_tip.rs:23>) — `..VersionMessage::default()` |
| T-165-23-03 | I/D | mitigate | CLOSED | Preserve redaction, real resource budgets, pending/abort/join and liveness assertions; retain local storage failures as fatal. | [packages/open-bitcoin-node/src/sync/tests/durable_tip.rs:23](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/durable_tip.rs:23>) — `..VersionMessage::default()` |
| T-165-23-04 | T/E | mitigate | CLOSED | Actual loaded-source mutations, enabled registration and exact input pins; no blanket allowlist/digest renewal. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-23-05 | R | mitigate | CLOSED | Keep four ordinary selectors refused/default-off and all166–168 ownership explicit; configured fixture evidence is accurately labeled. | [scripts/check-phase164-canonical-identity/contracts.ts:2](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:2>) — `* they do not claim runtime execution or canonical qualification. Changes require review.` |

### Plan 24

Plan: [.planning/phases/165-shared-exact-work-authority/165-24-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-24-PLAN.md>); complete raw SHA256 `df6ed883c67c3f07801b03e42758a8b4cea40ae01ea6eac681a12633cb7b5f7f`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-24-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-24-SUMMARY.md>); SHA256 `7297574b7f36932b96ac77e9da8b4bf4408abe0c8aadfce1995a6a7baa40845a`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-24-01 | S/T/E | mitigate | CLOSED | Use actual same-Fjall source/complete prefix; retain strict count/coins/B/heads/positions/undo binding and raw negatives. | [packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:302](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:302>) — `Chainstate::new()` |
| T-165-24-02 | T/R | mitigate | CLOSED | Real admitted root/child bodies and authorized operations drive exact original counters/projections; no fake receipts, generic error weakening or saved-tip substitution. | [packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:8](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:8>) — `loopback_config_with_permission_classes(max_peers, 0, PeerPermissionClassRegistry::default())` |
| T-165-24-03 | I/D | mitigate | CLOSED | Preserve redaction, real resource budgets, pending/abort/join and liveness assertions; retain local storage failures as fatal. | [packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:8](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs:8>) — `loopback_config_with_permission_classes(max_peers, 0, PeerPermissionClassRegistry::default())` |
| T-165-24-04 | T/E | mitigate | CLOSED | Actual loaded-source mutations, enabled registration and exact input pins; no blanket allowlist/digest renewal. | [scripts/check-phase165-shared-exact-work.ts:11](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:11>) — `import { CORPUS_FILE, CORPUS_SHA, HISTORICAL_IDENTITIES, CURRENT_BRIDGE_DATA, SOURCE_CONTRACTS, TEST_OWNERS, VERIFY_BASE_SHA, WORK_CLAIM_FACTS, WORK_CLAIM_FILE } from "./check-phase165-shared-exact-work/contracts.ts";` |
| T-165-24-05 | R | mitigate | CLOSED | Keep four ordinary selectors refused/default-off and all166–168 ownership explicit; configured fixture evidence is accurately labeled. | [scripts/check-phase164-canonical-identity/contracts.ts:2](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:2>) — `* they do not claim runtime execution or canonical qualification. Changes require review.` |

### Plan 25

Plan: [.planning/phases/165-shared-exact-work-authority/165-25-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-25-PLAN.md>); complete raw SHA256 `b13e8ee0fc8a8cd25d685a63af199151e2107c872bd1cb872e79d878f45fa977`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-25-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-25-SUMMARY.md>); SHA256 `e5b346a1748d58177592db41e0f34a79f000dd9a8eae74e0e62e9f613845ad12`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-25-01 | S/E | mitigate | CLOSED | Task1 retains actual accepted genesis receipt and SAME core through public constructor; no raw promotion/count repair. | [packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:55](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:55>) — `pub(in crate::sync::tests) fn at(` |
| T-165-25-02 | T | mitigate | CLOSED | Task2 verifies existing checked body/reorg header insertion at accepted parents and retains both caller files byte-identical with actual13-case GREEN. | [packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:215](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs:215>) — `fn admit_recipe_headers(` |
| T-165-25-03 | E | mitigate | CLOSED | Preserve existing caller carriers and physical assertions; trace authorize_reorg deriving proof independently, with no lower bypass or early canonical connection. | [packages/open-bitcoin-node/src/network/work_authority.rs:196](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/work_authority.rs:196>) — `pub(crate) fn authorize_reorg(` |
| T-165-25-04 | R | mitigate | CLOSED | Preserve exact original physical fault/assertion/snapshot boundaries and separate actual RED/GREEN from source mutations. | [packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs:119](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs:119>) — `fn run(case: Case) {` |
| T-165-25-05 | D/T | mitigate | CLOSED | Only five changed owners plus two read-only callers, fixed bounded input additions and unique present anchors, \<=628 lines and unchanged loader/registration checks. | [scripts/check-phase165-shared-exact-work.ts:15](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:15>) — `export const MAX_FILE_BYTES = 512_000;` |

### Plan 26

Plan: [.planning/phases/165-shared-exact-work-authority/165-26-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-26-PLAN.md>); complete raw SHA256 `55b38732385d1dae9907651b1fc1b3ba90a52feb2f88d02ebcd3689577fd1c6e`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-26-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-26-SUMMARY.md>); SHA256 `a80179d07dddbff7ff8adc0d5fc1faa1dc92f52d3261e260b65b843d94d898d8`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-26-01 | S/E | mitigate | CLOSED | Task1 authenticates actual fixed current versions and whole pinned source bundle, refuses every unknown version with no fallback. | [scripts/check-phase164-canonical-identity/fixture-source.ts:167](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/fixture-source.ts:167>) — `export function legacyFunctionContractView(sources: ReadonlyMap<string, string>): Map<string, string> {` |
| T-165-26-02 | T | mitigate | CLOSED | Task1 fixed eight legacy paths plus an existing child dependency, distinct exact provenance and whole hash before parse; Tasks2/3 bounded regular no-follow loader and one-entry pre-read pin growth. | [scripts/check-phase164-canonical-identity/fixture-source.ts:145](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/fixture-source.ts:145>) — `function authenticatedLegacySources(sources: ReadonlyMap<string, string>): readonly { path: string; sha256: string; source: string }[] {` |
| T-165-26-03 | R/T | mitigate | CLOSED | Task2 reverses only exact complete reviewed wiring and requires existing manifest current_sha; original snapshots/manifests/receipts unchanged. | [scripts/check-phase164-canonical-identity/historical-tooling.ts:173](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/historical-tooling.ts:173>) — `export function legacyToolingBase(file: string, source: string): string {` |
| T-165-26-04 | E | mitigate | CLOSED | Task2 separates authentic eight-source test baselines and historical-only command comparison from raw current/native validation; current165 independently refuses changed current fixtures/parent/child and all-six removal. | [scripts/check-phase164-canonical-identity/fixture-source.ts:185](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/fixture-source.ts:185>) — `export function legacyFunctionSources(sources: ReadonlyMap<string, string>): Map<string, string> {` |
| T-165-26-05 | D | mitigate | CLOSED | Existing14-module topology, no eager JSON import/new module, bounded mixed8 bundle, \<=628 files and copied-root missing/symlink/size tests. | [scripts/check-phase165-shared-exact-work.ts:38](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:38>) — `const descriptor = openSync(file, constants.O_RDONLY \| constants.O_NOFOLLOW);` |

### Plan 27

Plan: [.planning/phases/165-shared-exact-work-authority/165-27-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-27-PLAN.md>); complete raw SHA256 `3b211a49732cba8d8f15c9ca75e925c262b58b830ff5ef2e9cdfc9b5b54adf38`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-27-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-27-SUMMARY.md>); SHA256 `c016eacaf2831dced241ada54a346f9c53b3f4a480d7a3e7c077dde600d9defd`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-27-01 | E/T | mitigate | CLOSED | Task1 checks authorization/owner/generation-stage gates before effect callees and explicit unqualified refusal on actual current Rust. | [scripts/check-phase158-validated-reorg.ts:135](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.ts:135>) — `/** Only the exact evidence-authorized sentence earns a current-document exemption. */` |
| T-165-27-02 | E | mitigate | CLOSED | Task1 binds actual stage fields to engine scope and requires enabled parent/child/test registrations with substantive changed-anchor negatives. | [scripts/check-phase158-validated-reorg.ts:166](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.ts:166>) — `row(CORE + "engine/stage.rs", "validate_staged_reorg", ["self.check_staged_reorg(&staged)?", "Ok(ValidatedChainstateReorg", "owner: self", "staged"]),` |
| T-165-27-03 | R/T | mitigate | CLOSED | Tasks1/2 preserve the complete original evidence requirement set and exact385b/bebdd/CAF/784e targets; new dependencies never borrow old receipt qualification. | [scripts/check-phase164-canonical-identity/historical-tooling.ts:205](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/historical-tooling.ts:205>) — `export function historicalToolingView(sources: ReadonlyMap<string, string>): Map<string, string> {` |
| T-165-27-04 | S/T | mitigate | CLOSED | Task2 maps only two158 fingerprints through complete contextual recovery, keeps provenance raw and requires owner-specific tamper findings. | [scripts/check-phase162-integrated-filter-proof.ts:410](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase162-integrated-filter-proof.ts:410>) — `function checkFrozenGuards(text: (file: string) => string, failures: string[]): void {` |
| T-165-27-05 | R/D | mitigate | CLOSED | Task3 freezes the complete fixed factory/module union before full runs, checks all after, preserves failed310/140-20 receipts and fits existing628-line owners without exclusions. | [scripts/check-phase165-shared-exact-work.ts:30](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:30>) — `export function assertInputInventory(files: readonly string[]): void {` |

### Plan 28

Plan: [.planning/phases/165-shared-exact-work-authority/165-28-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-28-PLAN.md>); complete raw SHA256 `a8d6d1158e4d688aa43d1321b2babd2abf1584e290a2ae6df3be03be9bc1e697`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-28-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-28-SUMMARY.md>); SHA256 `b94d8b56ee68206769da4f6206707372012dac2d76e2425e8d3b02411f61c4c1`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-28-01 | S/E | mitigate | CLOSED | Task1 drops all original owners and demonstrates raw Untrusted before fresh complete Memory replay and strict consuming handoff. | [packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:105](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:105>) — `drop((source, store));` |
| T-165-28-02 | T | mitigate | CLOSED | Task1 compares complete coins/B/heads/positions/undo/bodies and savedNone; logical Some counts derive only from genuine snapshot with no writer. | [packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:28](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:28>) — `fn assert_saved_prefix(` |
| T-165-28-03 | E/T | mitigate | CLOSED | Task1 asserts checked available/nonfailed prefix and nonroot sequenceNone; first child is direct connect with no header/cache bypass. | [packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:50](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:50>) — `fn assert_prefix_graph(runtime: &DurableSyncRuntime, prefix: &ChainstateSnapshot) {` |
| T-165-28-04 | R | mitigate | CLOSED | Preserve old14 canonical/tie controls, record real exact combined receipt and require parent checker before24;166–168 limits unchanged. | [packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:76](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs:76>) — `fn phase165_live_fixture_requalified_prefix_extends_with_normal_maturity() {` |
| T-165-28-05 | D/T | mitigate | CLOSED | Task2 uses explicit child registration/current owner, data/loop reuse, one fixed input addition, \<=628 and complete stable matrices with source review before pins. | [scripts/check-phase164-canonical-identity/test-modules.ts:370](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:370>) — `["packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs", 1],` |

### Plan 29

Plan: [.planning/phases/165-shared-exact-work-authority/165-29-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-29-PLAN.md>); complete raw SHA256 `4cf40fa4f86a20df17fb4ef67d491f817b0bde4317b7337a0e52e05a523fe898`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-29-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-29-SUMMARY.md>); SHA256 `b86444ef822651549c29c7d61ca6957e414207cff8c6f07e99909e06bb865541`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-29-01 | T/E | mitigate | CLOSED | Task1 requires unchanged64 imported/AST ordered values, exact three-file inverse and identical existing loops/module edges before independent review. | [scripts/check-phase165-shared-exact-work.test.ts:601](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:601>) — `const imported = Bun.spawnSync([process.execPath, "-e", 'const module = await import(${JSON.stringify(moduleUrl)}); console.log(JSON.stringify(module.INPUT_FILES));']);` |
| T-165-29-02 | T/R | mitigate | CLOSED | Tasks1/2 compare every ordered source/test/input array/pin and complete actual dependency hashes; no renewal or missing literal module. | [scripts/check-phase165-shared-exact-work.ts:30](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:30>) — `export function assertInputInventory(files: readonly string[]): void {` |
| T-165-29-03 | R | mitigate | CLOSED | Task2 executes actual64 affected cases/live checks, records filter exclusions and confines the receipt to the data move;24/14 full gates remain. | [scripts/check-phase165-shared-exact-work.test.ts:19](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:19>) — `for (const [file, digest] of LEGACY_BRIDGE_ROWS) {` |
| T-165-29-04 | D | mitigate | CLOSED | Only three existing owners, physical\<=628, no new module/dependency/logic extraction or exceptions; failure returns bounded replan. | [scripts/check-phase165-shared-exact-work/test-support.ts:9](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/test-support.ts:9>) — `export const CLI_MODULES = [` |

### Plan 30

Plan: [.planning/phases/165-shared-exact-work-authority/165-30-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-30-PLAN.md>); complete raw SHA256 `486ad9b2cbfefe5b7b77ad07dbea73d9188f3e304dbb27fd0ab6aaae66344b0e`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-30-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-30-SUMMARY.md>); SHA256 `726fee0021cc73b88d5d53767a618e5584366f1f1b761cfb3635adc256838b61`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-30-01 | S/E | mitigate | CLOSED | Task1 retains genuine into_parts core through CoinsFlush/from_chainstate; exact origin/bind checks, no coins_mut/raw/mechanical import. | [packages/open-bitcoin-node/src/chainstate/tests.rs:379](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/tests.rs:379>) — `core.prepare_current_best_block()` |
| T-165-30-02 | T/R | mitigate | CLOSED | Task1 real Untrusted-versus-MissingCoin RED then exact second-outpoint GREEN, first-input identity and count0 before/after. | [packages/open-bitcoin-node/src/chainstate/tests.rs:441](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/tests.rs:441>) — `open_bitcoin_core::chainstate::ChainstateError::MissingCoin {` |
| T-165-30-03 | T/E | mitigate | CLOSED | Task2 actual unique loaded-source negatives and target test disable controls, independent exact row/pin reversal and no old164 SOURCE_FILES/EDGES/TEST_MODULES/other policy changes; only the current165 TEST row in its shared leaf may change. | [scripts/check-phase165-shared-exact-work.test.ts:15](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:15>) — `import { loadSources as loadLegacyBridgeInputs, checkPhase162IntegratedFilterProof, checkPhase162LegacyFunctionControls, assertionSpans, factSpans, NATIVE_COMMANDS, SHARED_FORMAT } from "./check-phase162-integrated-filter-proof.ts` |
| T-165-30-04 | D/R | mitigate | CLOSED | Five owners\<=628 (fifth only the current165 TEST data host), full affected Node/strict plus focused guard closure; no coverage owners or unexecuted full/native/future qualification claims. | [packages/open-bitcoin-node/src/chainstate/tests.rs:373](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/tests.rs:373>) — `fn prepare_connect_then_drop_leaves_parent_unpopulated() {` |
| T-165-30-06 | T/R | mitigate | CLOSED | Task2 authentic actual completed31 before values/body, appends one actual current TEST row only, preserves SOURCE_FILES/TEST_MODULES/other helpers and includes fifth owner in independent prepin/raw closure; no forecast/new scope. | [scripts/check-phase164-canonical-identity/test-modules.ts:387](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:387>) — `["packages/open-bitcoin-node/src/chainstate/tests.rs",11],` |

### Plan 31

Plan: [.planning/phases/165-shared-exact-work-authority/165-31-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-31-PLAN.md>); complete raw SHA256 `806d863b14d91d323cfa27370851b5438e5aef5bfd7606e3b725796fcab09376`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-31-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-31-SUMMARY.md>); SHA256 `80e31f68945552db34b7d67b76b2bbc63e632dfcbf8d5187d93b09df0ec6c2c5`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-31-01 | S/E/T | mitigate | CLOSED | Task1 mines only actual headers, asserts typed UntrustedWorkLineage for all raw calls and unchanged cache/physical facts; no promotion. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs:237](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs:237>) — `pub(super) fn assert_raw_cache_refuses(` |
| T-165-31-02 | T/R | mitigate | CLOSED | Task1 actual0..1001 full params/flush/ActiveEmpty/gen0, ALL1002 derived undos including root, independent per-height bytes and cache0/1/713/714 exact eviction; counters remain. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs:194](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs:194>) — `fn assert_payloads(&self, store: &FjallNodeStore, first_retained: usize) {` |
| T-165-31-03 | S/E/T | mitigate | CLOSED | Task2 complete actual replay/derived metadata/strict consuming identity and real28 prerequisite, physicalNone/UNKNOWN preserved. | [packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:483](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:483>) — `fn phase127_requalify(` |
| T-165-31-04 | I/D | mitigate | CLOSED | Task2 retains exact missing/identity negatives, all-handle cleanup, actual auth/redaction/serving budgets and joins. | [packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs:108](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs:108>) — `pub(super) async fn receive(&mut self) -> WireNetworkMessage {` |
| T-165-31-05 | R/T | mitigate | CLOSED | Task3 independent source-before-pins/inverse/full stable read union and actual full matrices; no historical/ordinary/future borrowing. | [scripts/check-phase165-shared-exact-work.ts:30](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:30>) — `export function assertInputInventory(files: readonly string[]): void {` |
| T-165-31-06 | T/R/E | mitigate | CLOSED | Task3 authentic imported BEFORE values, whole typed declarations/ordered values and four-file inverse preserve every helper/TEST_MODULES byte; acyclic14/36closure and151/83/140/522/273+c118 unchanged before separate source-reviewed activation. | [scripts/check-phase164-canonical-identity/test-modules.ts:393](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:393>) — `export const SOURCE_CONTRACTS: readonly (readonly [string, string, string])[] = [` |
| T-165-31-07 | T/E/R | mitigate | CLOSED | Task2 only moves three exact pure predicates with original callsites/helper prefix/type/binding/inverse/caps; Task3 whole sharedhelper/root ownership and meaningful hash/length/true-return/root route controls, all12-source prepin review/native/matrix/raw closure. | [scripts/check-phase165-shared-exact-work.test.ts:96](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:96>) — `expect(checkPhase162LegacyFunctionControls(legacy).some(failure => failure.includes(file + ":stale_runtime_configured: exact executable " + (kind === "assertion" ? "assertion" : "protocol/matrix fact")))).toBe(true);` |

### Plan 32

Plan: [.planning/phases/165-shared-exact-work-authority/165-32-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-32-PLAN.md>); complete raw SHA256 `f648e8022b4098ab8a8eda4b8711d890e348ecbbc4bfe5aacbd8a846f2afed7b`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-32-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-32-SUMMARY.md>); SHA256 `38c5a37828663722dc1d97947a6a0befed318cb45f48d566fd6e4aaaab77843a`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-32-01 | S/E | mitigate | CLOSED | Tasks1/2 full params, actual stronger suffix and same genuine normal100 owner/public prefix replay. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs:109](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs:109>) — `pub fn open_genuine(` |
| T-165-32-02 | T/R | mitigate | CLOSED | Task1 real endpoints/lag/all immutable rows and unchanged old waiter/corruption/prune assertions. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs:39](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs:39>) — `assert_eq!(response.status(), StatusCode::OK);` |
| T-165-32-03 | E/T | mitigate | CLOSED | Task2 replay before body20 deletion, physicalNone/UNKNOWN preserved; header-only helper cannot grant live core/body facts. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs:75](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs:75>) — `pub(super) fn admit_retained_header_proof(` |
| T-165-32-06 | I/D | mitigate | CLOSED | Tasks2/3 retain all20frames/oracles/denials/body/wallet/budgets/joins, seven owners\<=628 and all full modules/strict Rust. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:108](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:108>) — `fn exchanges(oracle: &Value) -> Vec<Exchange> {` |
| T-165-32-07 | S/T/E | mitigate | CLOSED | Task2 SAME actual full-param checked header-only helper in both raw callers; no pruned body restoration or physical publication. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:429](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:429>) — `super::fixtures::admit_retained_header_proof(` |

### Plan 33

Plan: [.planning/phases/165-shared-exact-work-authority/165-33-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-33-PLAN.md>); complete raw SHA256 `7a53f1405221220074570947f12273afd9f21b2530b65b7cbeeb72adedc2bf3d`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-33-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-33-SUMMARY.md>); SHA256 `d385bc67ad696e4cbec0fba84e94e479d583b711c864f6bea108b457573cbe67`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-32-04 | T/R | mitigate | CLOSED | Entire original Task3 transferred byte-exact: reviewed source2d0/whole-adapterb92 inverses with raw map binding; original history independent. | [scripts/check-phase164-canonical-identity/contracts.ts:46](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:46>) — `export function maybeRestoreReviewedSource(source: string, inverse: ExactSourceInverse): string \| undefined {` |
| T-165-32-05 | T/E | mitigate | CLOSED | Entire original Task3 transferred byte-exact: fresh copied-module positives/tampering/refusal, exact current pins/full read union/fresh matrices. | [scripts/check-phase164-canonical-identity.ts:106](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity.ts:106>) — `export function historicalFixtureAdapter(inputs: ReadonlyMap<string, string>): string {` |
| T-165-33-01 | S/E/R | mitigate | CLOSED | Task1 exact seven-source/full Rust receipt/root acceptance before pins; original failures stay failed, Summary32 source-only and33 actual guard closure. | [packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:429](</Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs:429>) — `super::fixtures::admit_retained_header_proof(` |
| T-165-33-02 | T/E | mitigate | CLOSED | Task1 actual additional raw SOURCE/TEST_OWNERS/input/unique helper/caller controls with1 actual enabled direct test registered; original no-body/physical source assertions retained. | [scripts/check-phase164-canonical-identity/test-modules.ts:293](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:293>) — `"packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs",` |
| T-165-33-03 | T/R | mitigate | CLOSED | Transferred Task3 canonical ORIGINAL unique nonempty nonoverlapping ordered spans/right-to-left, current/restored SHA, unconditional paired PASSED-map binding and all negatives. | [scripts/check-phase164-canonical-identity/contracts.ts:46](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:46>) — `export function maybeRestoreReviewedSource(source: string, inverse: ExactSourceInverse): string \| undefined {` |
| T-165-33-04 | I/D | mitigate | CLOSED | Nine existing owners\<=628, exact14 modules and complete frozen raw union/logs/full matrices; no projection or old receipt qualifies new current bytes. | [scripts/check-phase165-shared-exact-work/test-support.ts:19](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/test-support.ts:19>) — `export function copiedInputs(root: string, realInputs: ReadonlyMap<string, string>): void {` |
| T-165-33-05 | T/R/E | mitigate | CLOSED | Task1 requires valid executable copied modules/intended findings for all13 cases, explicit sparse/empty/noop controls, exact old helper/tables/finally cleanup and complete ninth-owner tooling/raw review; syntax/import failure never qualifies. | [scripts/check-phase165-shared-exact-work/test-support.ts:211](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/test-support.ts:211>) — `const stdout = child.stdout.toString();` |
| T-165-33-06 | T/E/R | mitigate | CLOSED | Task1 authenticates five unchanged actual whole sources, appends5ownership+4actualTEST rows and9unique route controls; exact13table relocation preserves all old helpers/cases; new actual9owner/proposal review precedes pins. | [scripts/check-phase164-canonical-identity/test-modules.ts:371](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:371>) — `["packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs",8],` |

### Plan 34

Plan: [.planning/phases/165-shared-exact-work-authority/165-34-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-34-PLAN.md>); complete raw SHA256 `17f47519e3e30af1ea4b2eb59a226b269a86d9182a1e4a17b0edb6a1363c0975`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-34-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-34-SUMMARY.md>); SHA256 `c067a5850a82b84a9f6bd0badfe5c68c1a9c19b699e5e2bce21e630852416fcd`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-34-01 | T/E | mitigate | CLOSED | Task1 exact complete64hex arrays, independently computed expected values and four real failing assertions before fix; all other fields remain enforced. | [scripts/check-phase72-observability-evidence.ts:150](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase72-observability-evidence.ts:150>) — `const CANONICAL_VALIDATED_WORK = "00000000000000000000000000000000000000000000000000000000000cd145";` |
| T-165-34-02 | T/R | mitigate | CLOSED | Task1 unchanged shared validatedWork/connectedWork and two complete decimal support/smoke row regressions; actual raw sources reviewed independently. | [scripts/check-phase72-observability-evidence.ts:123](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase72-observability-evidence.ts:123>) — `connected_work: fixture.connectedWork,` |
| T-165-34-03 | D/T | mitigate | CLOSED | Task1 silent pure import plus actual directCLI diagnostics before fix and success afterward; full original main/checks byte-preserved. | [scripts/check-phase72-observability-evidence.ts:549](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase72-observability-evidence.ts:549>) — `if (import.meta.main) {` |
| T-165-34-04 | T/E/R | mitigate | CLOSED | Task2 source-before-digest review, exact two ordinary rows,88TEST/history unchanged, actual stale-pin refusal and full-file inverse/root acceptance. | [scripts/check-phase164-canonical-identity/test-modules.ts:552](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:552>) — `["scripts/check-phase72-observability-evidence.ts","ownership","46b34b0cd3e6f5c0ae326ba9fd17e5d46f980099e582224903ed6187ae582f11"],` |
| T-165-34-05 | I/D/R | mitigate | CLOSED | Task2 complete root-approved raw union/commands/log closure, four\<=628 owners, no added source/network/credential behavior; final14 remains full fresh gate. | [scripts/check-phase165-shared-exact-work.ts:15](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:15>) — `export const MAX_FILE_BYTES = 512_000;` |

### Plan 35

Plan: [.planning/phases/165-shared-exact-work-authority/165-35-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-35-PLAN.md>); complete raw SHA256 `20b92f9d0a9355d16da1bb961dd291c355111b1e2c445666c99a629b1c3fe99f`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-35-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-35-SUMMARY.md>); SHA256 `447b7c00bfef4c0ae32ea235c68ed654049caf14902ea71f37a9a4b6a2a983be`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-35-01 | T/E | mitigate | CLOSED | Task1 exact typed literal block; Task2 all actual runtime exports/ordered JSON equality and unchanged inventory pin. | [scripts/check-phase164-canonical-identity/test-modules.ts:393](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:393>) — `export const SOURCE_CONTRACTS: readonly (readonly [string, string, string])[] = [` |
| T-165-35-02 | T/R | mitigate | CLOSED | Task1 authentic complete before and whole two-file inverse; old host prefix and all non-move contracts exact, no historical digest renewal. | [scripts/check-phase165-shared-exact-work/contracts.ts:6](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/contracts.ts:6>) — `export { SOURCE_CONTRACTS, TEST_OWNERS } from "../check-phase164-canonical-identity/test-modules.ts";` |
| T-165-35-03 | D/T | mitigate | CLOSED | Task1 existing edge only; Task2 actual graph and fresh copied CLI/bounded-loader tests with no helper change. | [scripts/check-phase165-shared-exact-work/contracts.ts:6](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work/contracts.ts:6>) — `export { SOURCE_CONTRACTS, TEST_OWNERS } from "../check-phase164-canonical-identity/test-modules.ts";` |
| T-165-35-04 | I/D | mitigate | CLOSED | Task2 reviewed canonical raw union including absence/bytes/SHA and all observed reads; both files\<=628, no new network or credential path. | [scripts/check-phase165-shared-exact-work.ts:38](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:38>) — `const descriptor = openSync(file, constants.O_RDONLY \| constants.O_NOFOLLOW);` |
| T-165-35-05 | R/E | mitigate | CLOSED | Task2 retain both native failures and12 known source failures; final14 retains entire fresh acceptance and later-phase ownership. | [scripts/check-phase165-shared-exact-work.ts:111](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:111>) — `function checkWorkClaims(source: string, fail: Fail): void {` |

### Plan 36

Plan: [.planning/phases/165-shared-exact-work-authority/165-36-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-36-PLAN.md>); complete raw SHA256 `3040d19e226e1a77038c7f97978b7ef280cbc4f4aa733d5c9f08413f69f36d90`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-36-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-36-SUMMARY.md>); SHA256 `c3514a28a716387317198fb89a3c9d945fa407c694333ada5938c735fa3f1af7`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-36-01 | T/E | mitigate | CLOSED | Task1 exact current coverage, omission/obsolete-only negatives, separate escaped/unescaped doc contracts; all other guards exact. | [scripts/check-phase73-uat-verification/constants.ts:68](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase73-uat-verification/constants.ts:68>) — `"## VER-02 deterministic coverage map",` |
| T-165-36-02 | T/E | mitigate | CLOSED | Task2 caller/helper/finish order and exact seven-argument constructor, actual mutation/default/comment negatives. | [scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts:34](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts:34>) — `export function verifyProductionActivation(` |
| T-165-36-03 | T/R | mitigate | CLOSED | Task2 executable helper variant/hash plus served/mixed assertion ownership and independent wrong-helper/hash/missing-assertion cases. | [scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts:13](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts:13>) — `const blockPredicate = "matches!(message, WireNetworkMessage::Block(block) if block_hash(&block.header) == expected_hash)";` |
| T-165-36-04 | T/E/R | mitigate | CLOSED | Task3 independent complete source/inverse review before digest; exact one renewed/eight new ordinary rows,88TEST/history unchanged and actual stale-pin refusal. | [scripts/check-phase164-canonical-identity/test-modules.ts:561](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:561>) — `["scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts","ownership","4bce72cde7265b9ebf851a407b40d0b9529baa99e02b17759a4b858eaec19c7c"],` |
| T-165-36-05 | I/D/R | mitigate | CLOSED | Task3 complete raw union/log closure, physical628 caps, existing native test registration, remaining failures and full final14 gates retained. | [scripts/check-phase165-shared-exact-work.ts:15](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:15>) — `export const MAX_FILE_BYTES = 512_000;` |

### Plan 37

Plan: [.planning/phases/165-shared-exact-work-authority/165-37-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-37-PLAN.md>); complete raw SHA256 `d383153ecf174623ab20b6d45ae823b76ba6cad953e2924383f3a46e5170a914`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-37-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-37-SUMMARY.md>); SHA256 `1ee45f1471756ae66c79b2718e8828414a878d848abb42137a808a1f6bfd45e1`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-37-01 | T/E | mitigate | CLOSED | Task1 owning-function save prefix, failure early return/no accepted effects, requested Connected actual-active-tip order and independent side/pending/error negatives. | [scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts:9](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts:9>) — `export function hasDurableSaveBeforeTip(response: string): boolean {` |
| T-165-37-02 | T/E | mitigate | CLOSED | Task1 exact acquisition/consumption/order in both seams; authorization/bypass/effect negatives, existing aggregate and persistence rules unchanged. | [scripts/check-phase134-apply-boundaries/aggregate-roots.ts:7](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-apply-boundaries/aggregate-roots.ts:7>) — `chainstatePreparation: "prepare_authorized_connect",` |
| T-165-37-03 | R/T | mitigate | CLOSED | Task1 retarget actual stale anchors with changed-byte assertions; full old and new registered cases execute without ignored tests. | [scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts:277](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts:277>) — `".prepare_authorized_connect(",` |
| T-165-37-04 | T/E/R | mitigate | CLOSED | Task2 independent complete source/absence/inverse acceptance before five ordinary rows/pin; actual stale refusal; old arrays/history unchanged. | [scripts/check-phase164-canonical-identity/test-modules.ts:563](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:563>) — `["scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts","ownership","90a131b2707648385d5fc15e329a777bb1bb65619062bcce5b14e6bdab2793a5"],` |
| T-165-37-05 | I/D/R | mitigate | CLOSED | Task2 complete canonical raw union/new helper graph/log closure, readable628 caps, actual seven source CLIs and retained final14/later-filter gates. | [scripts/check-phase165-shared-exact-work.ts:30](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:30>) — `export function assertInputInventory(files: readonly string[]): void {` |

### Plan 38

Plan: [.planning/phases/165-shared-exact-work-authority/165-38-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-38-PLAN.md>); complete raw SHA256 `ff27041bc9bc512d08dd2950c287e9fd352a751966f1d519859fb296438eca45`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-38-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-38-SUMMARY.md>); SHA256 `9236313fecc00e326131ec7ce149d6ebbe4103e7fca8168f437018b37fa773ad`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-38-01 | T/E | mitigate | CLOSED | Require actual typed validation/genuine constructor/helper/order; evidence: Specific changed-source negative controls and all3 CLIs | [scripts/check-phase155-filter-index.ts:140](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase155-filter-index.ts:140>) — `function checkImmutableAdmission(text: (file: string) => string, failures: string[]): void {`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-38-02 | T/R | mitigate | CLOSED | Preserve original bodies/loops and assertion/error obligations; evidence: Full three files and exact before diff/inverses | [scripts/check-phase156-prune-coordination.test.ts:33](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase156-prune-coordination.test.ts:33>) — `test("genuine payload comparison refuses an assert-true helper stub", () => {`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-38-03 | T/R | mitigate | CLOSED | Independent source acceptance before reviewed rows and derived pin; evidence: Actual stale-pin refusal and ordered fixed inventory | [scripts/check-phase164-canonical-identity/test-modules.ts:567](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:567>) — `["scripts/check-phase155-filter-index.ts","ownership","9f38bd65a10a64cf987926bb639876ea80945ec23db637cce0cd7707f1ffd803"],`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-38-04 | I/D | mitigate | CLOSED | Fixed existing helpers, bounded reads and complete closed raw union; evidence: Loader diff, module/cap checks and retained actual receipts | [scripts/check-phase157-index-catch-up/contracts.ts:136](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase157-index-catch-up/contracts.ts:136>) — `[RPC + "bin/open_bitcoind/tests/filter_index/fixtures.rs", "genuine_runtime", ["Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None)", "Chainstate::new()", ".rebind_into(&mut core)", "ManagedChainstate::fr`; mitigate explicitly inferred from Mitigation/Evidence columns. |

### Plan 39

Plan: [.planning/phases/165-shared-exact-work-authority/165-39-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-39-PLAN.md>); complete raw SHA256 `17d4abedf0d4ecb8e11cc80627949bc9e17510d6f5c59f9732ba0640eee7e942`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-39-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-39-SUMMARY.md>); SHA256 `107337577fcefdb9b58504ddb6f7dd468018dbcc8ed299cb704b33b4fe59d6b3`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-39-01 | T/E | mitigate | CLOSED | Require actual typed validation, unconditional authorization and genuine helper order; evidence: Independent actual-edge mutations, no-op prevention and both159/161 CLIs | [scripts/check-phase159-filter-rpcs.ts:75](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.ts:75>) — `function checkSource(text: Texts, failures: string[]): void {`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-39-02 | T/R | mitigate | CLOSED | Fixed whole current/restored hashes, canonical exact hunks and immutable snapshots; evidence: Complete source inverses and old/unknown/forged/partial controls | [scripts/check-phase164-canonical-identity/contracts.ts:46](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:46>) — `export function maybeRestoreReviewedSource(source: string, inverse: ExactSourceInverse): string \| undefined {`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-39-03 | T/E | mitigate | CLOSED | Bind current H/E before classification at full164/165 only; evidence: Direct stale-map plus fresh copied164/165 cases | [scripts/check-phase164-canonical-identity.ts:106](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity.ts:106>) — `export function historicalFixtureAdapter(inputs: ReadonlyMap<string, string>): string {`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-39-04 | I/D | mitigate | CLOSED | Historical projections use only present named sources, no filesystem fallback; evidence: Filtered159 transition, missing manifest159 and absent161 cases | [scripts/check-phase164-canonical-identity/historical-tooling.ts:215](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/historical-tooling.ts:215>) — `const maybeSource = sources.get(file);`; mitigate explicitly inferred from Mitigation/Evidence columns. |
| T-165-39-05 | T/R | mitigate | CLOSED | Source acceptance before root current rows/pin and closed complete union; evidence: Stale-pin refusal, ordered inventory, raw closure and preserved receipts | [scripts/check-phase165-shared-exact-work.ts:30](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:30>) — `export function assertInputInventory(files: readonly string[]): void {`; mitigate explicitly inferred from Mitigation/Evidence columns. |

### Plan 40

Plan: [.planning/phases/165-shared-exact-work-authority/165-40-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-40-PLAN.md>); complete raw SHA256 `172366e9de847ee6d23cda234895824a2e3945d8c9ba04f4c735fec43a6e65c3`.

Scoped execution trace: [.planning/phases/165-shared-exact-work-authority/165-40-SUMMARY.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-40-SUMMARY.md>); SHA256 `80e36efda7d8a5419d1bb7d85aa2a89b139d480355c76971bad10c59c11cf40b`.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-40-01 | T/E | mitigate | CLOSED | Task1 real RED, actual fixed-root scanImports closure, loader-vs-category controls and unchanged strict missing-root assertion | [scripts/check-phase163-provenance/fixtures.ts:134](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance/fixtures.ts:134>) — `function discoverRuntime(entries: readonly string[], root: string): RuntimeGraph {` |
| T-165-40-02 | I/D | mitigate | CLOSED | Fixed two-root allowlist, regular contained paths, unsupported imports refused,32/400 bounds and owned cleanup | [scripts/check-phase163-provenance/fixtures.ts:50](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance/fixtures.ts:50>) — `function boundedPath(root: string, file: string): string {` |
| T-165-40-03 | T/R | mitigate | CLOSED | Independent copier CLEAN before descriptors; exact full current/before hashes and unchanged manifest/snapshots/wiring | [scripts/check-phase164-canonical-identity/contracts.ts:172](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/contracts.ts:172>) — `"import { createHash } from \"node:crypto\";\nimport { COPIED_CLI_INVERSES, FILTER_GUARD_INVERSES, maybeRestoreReviewedSource } from \"./contracts.ts\";\n\nexport const MANIFEST_FILE = \"packages/testdata/phase164/historical-tooli` |
| T-165-40-04 | T/E | mitigate | CLOSED | Required two-owner current binding in shared164 helper before classification, direct plus fresh164/165 tests | [scripts/check-phase164-canonical-identity.ts:121](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity.ts:121>) — `for (const [file, inverse] of COPIED_CLI_INVERSES) {` |
| T-165-40-05 | T/R | mitigate | CLOSED | H source CLEAN before external complete47bb inverse; strict helper unchanged; unknown generations cannot enter fragment fallback | [scripts/check-phase164-canonical-identity/historical-tooling.ts:184](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/historical-tooling.ts:184>) — `} else if (currentHash !== inverse.restoredRawSha && currentHash !== maybeCurrent?.[1]) {` |
| T-165-40-06 | T/R | mitigate | CLOSED | Carry5550 paths plus actual new reads/outputs; reviewed frozen tools; same-buffer auth; complete per-command closure and failed-run retention | [scripts/check-phase163-provenance/fixtures.ts:338](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance/fixtures.ts:338>) — `export function cleanupFixtures(): void {` |
| T-165-40-07 | E | mitigate | CLOSED | Root-only later14, formal code/security/goal/lifecycle gates and normal hooks; no task commits; stop before166 | [.githooks/pre-commit:10](</Users/peterryszkiewicz/Repos/open-bitcoin/.githooks/pre-commit:10>) — `OPEN_BITCOIN_LOC_REPORT_SOURCE=index bash scripts/verify.sh` |

### Plan 41

Plan: [.planning/phases/165-shared-exact-work-authority/165-41-PLAN.md](</Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/165-shared-exact-work-authority/165-41-PLAN.md>); complete raw SHA256 `4df35e97558c6b1e5f9222dcc3a3f4802f7e57963bc398dc68c56f7bc614a323`.

No SUMMARY exists at capture; completion is pending.

| Threat ID | STRIDE | Disposition | Terminal draft status | Expected mitigation | Current source evidence |
| --- | --- | --- | --- | --- | --- |
| T-165-41-01 | T | mitigate | CLOSED | Unchanged four-owner mutation loop must receive exact MANIFEST_FILE/history finding; retain manifest/snapshot/inverse validation unchanged. | [scripts/check-phase165-shared-exact-work.ts:238](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:238>) — `try { historicalToolingView(inputs); }` |
| T-165-41-02 | E | mitigate | CLOSED | Preserve historyBound return and caller condition byte-exact; independently review that historicalToolingView success cannot authorize current classification. | [scripts/check-phase165-shared-exact-work.ts:81](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:81>) — `if (historyBound) checkCurrentClaims(text, fail);` |
| T-165-41-03 | T | mitigate | CLOSED | Independent source CLEAN/Root acceptance precedes digest derivation; independently review complete one-row inverse and unchanged192/88/307 inventory/pin before activation. | [scripts/check-phase164-canonical-identity/test-modules.ts:582](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/test-modules.ts:582>) — `["scripts/check-phase165-shared-exact-work.ts","ownership","cc8fb2ac578eb1a271e148d4d58ca54ac7ad40229669137ddc1bee6486b1302f"],` |
| T-165-41-04 | R | mitigate | CLOSED | Actual terminal failure/full8322-state Root closure before edits; per-command full Raw/input/output receipts, original test identities and retained failures. | [scripts/check-phase165-shared-exact-work.test.ts:336](</Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:336>) — `for (const [file] of TOOLING_INVERSE_CASES) test('phase165 reviewed current158 history ${file}', () => {`; full accepted 94-command/1,723,968-row closure `dce27225…` and accepted41 summary `391fb4bd…`. |
| T-165-41-05 | I | mitigate | CLOSED | 41 summary describes bounded source evidence only; final14 R5/default verification and independent formal/security/code gates remain mandatory;166–168 stay deferred. | [docs/parity/v2-6-shared-exact-work.md:48](</Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/v2-6-shared-exact-work.md:48>) — `No actual pinned-Knots daemon differential qualification is claimed.`; bounded accepted41 summary and entire actual R5 accepted; independent final reviews/lifecycle remain mandatory before Root finalization. |



## Threat flags

At this publication, all 40 existing SUMMARY files were checked for exact `## Threat Flags` sections; none exists. The earlier complete review read the previous39 summary bodies, and this continuation read the accepted41 summary, which explicitly maps its bounded repair to T-165-41-01 through05 and introduces no unmapped attack-surface flag. Final14 SUMMARY publication is Root-owned and remains subject to final document rebinding; this report does not claim to have read an absent future file. No unregistered flag is currently found. The preserved native postprocessing serialization issue maps to T-165-14-01 and is resolved by authenticated separate proof, not a new accepted security risk.

## Authenticated evidence references

Paths are relative to `.local/open-bitcoin-dev/`. Complete raw identities are retained.

| Artifact | Bytes | SHA256 |
| --- | ---: | --- |
| `phase165-final41-security-prepass/SECURITY.md` | 132143 | `af850bd11848e8058388afff10e3d90a3b9cb68b91e6b1310319fdaec262ed7b` |
| `phase165-final41-code-final-binding-prepass.json` | 168502 | `7f5d995e4464b37cc189d2bebe6f08351a0b3678444618602d3cc2cd9e0e0888` |
| `phase165-plan41-root-green-stage-acceptance.json` | 1579 | `dce27225bd10460c4e8aa00974a781effb6b17826eb5f08663efc103f64ccac7` |
| `phase165-final41-root-summary-acceptance.json` | 1651 | `391fb4bdef3a43e4aa058ea1fe8ccc21dc5f042f8d04964179940ab13ce07ede` |
| `phase165-final41-root-r5b-preflight-closure.json` | 3522675 | `71667e7050360f0295bbbccdefb3bcba2de33e060dbfb94ee5a110ebafc7fa96` |
| `phase165-final41-root-r5b-native-closure.json` | 25571096 | `d464b6d90fb8f1cef1ea583bf22e631f4019a3ca83ae1ba30f1e59a04231e5bd` |
| `phase165-final41-root-native-final-acceptance.json` | 1929 | `abc239023ef67348b228b68315755f3424fd633d673567d789c0f9b780798b68` |
| `phase165-final14-r5b-native-outcome.json` | 1273 | `5cb92ce07fae76bd636965d8691649d785e4d974fef0073414c4953a525a8f30` |
| `phase165-final14-r5b-native-result.json` | 8774 | `7fab8ea907c3520485bc0f5b3dff81857fe63a4b2d501b063f9b2826dce31d20` |
| `phase165-final14-r5b-native-validation.json` | 1384 | `31b559aec8578c1c792ebf5b8ad814b10f5b9f15215862512ab5d85b9d81253d` |
| `phase165-final14-r5b-native-verify.log` | 2264250 | `2fd5e17b4456bd1ebed7cfee6b63cb1571bcbe730fab3c225e69ae9e922e0352` |
| `phase165-final41-root-native-log-counts.json` | 27141 | `60705465f64c910f22196c783faa31332b53389eed78d5054435ce0498d6e4f1` |
| `phase165-final14-r4-r5-native-capture.json` | 4698 | `181d74ccfdd88ff6040a4b261d1b7a5917f5bc67fb48e53ec422a0b95ae981b0` |

## Result and residual scope

SECURED: 184/184 CLOSED; 182 mitigate, two accept, zero transfer; threats_open=0. No implementation gap or unmapped threat flag was found within the declared register. The two accepted risks above are the only accepted entries. Default-off networking, four ordinary-selector refusals and Phases166–168 ownership remain unchanged. Actual normal-hook finalization remains Root-owned and future.

This reviewer changed only the canonical security report in this publication. Source, existing evidence, other reports, documentation, metadata, runtime behavior and Git index/history were not changed. No implementation tests, imports, CLI, Cargo, Bun, Bazel or Git operation was run by this reviewer. Verification consisted of read-only source/receipt/hash/complete-frame checks.

## Final publication binding — 2026-10-18T13:11:19.609224+00:00

This append records the later actual publication checkpoint. The frontmatter `verified` time, 2026-10-18T12:53:58.820505Z, dates the initial security publication; statements above about 40 summaries and future final14 publication describe that earlier checkpoint. All 41 summaries now exist. The actual final14 summary was read in full and its native/capture/postprocessing distinctions match the retained evidence. All 41 summaries were checked for exact Threat Flags sections; none is present and no unregistered flag is found. No earlier failed native or false postprocessor outcome is changed.

Root accepted the actual14 summary through `.local/open-bitcoin-dev/phase165-final41-root-summary14-acceptance.json`, SHA256 `35953c9180172bd4b338289bf38fdc926a41b13b1a1c0b5ca2aea507e2c3364a`. Actual terminal metadata publication is `.local/open-bitcoin-dev/phase165-final41-root-terminal-metadata-application.json`, SHA256 `ec3088c13dede3b4bc1f03961c52dd3ad627c906a7292ae7bfe7cdb577f9b501`. All eight recorded after-bodies match their current files. The catalog row now has status `done`, links all three real canonical reports and closes only CWOR-01/02/03/05. CWOR-04 and Phase166 remain unstarted. The actual canonical goal verification reports passed,150/150:146 plan truths and four mandatory roadmap criteria. This describes completed reviews/publication, while the actual normal-hook Git transaction remains Root-owned and future.

The five final14 document owners and both relevant saved summaries are bound below to their complete current bytes. The index-generated LOC report reflects actual successful generation/freshness checks after staging:1,620 included files and482,559 lines. Its change from the earlier worktree LOC identity is the authorized staged-index generation, not source drift.

| Published artifact | Bytes | SHA256 |
| --- | ---: | --- |
| `.planning/phases/165-shared-exact-work-authority/165-14-SUMMARY.md` | 10986 | `6c60c111662763139d82258308131663e866aff8a2b636e216710c63287eddce` |
| `.planning/phases/165-shared-exact-work-authority/165-41-SUMMARY.md` | 19979 | `132a0a5a541f513c7921ae1128bde25b305e4adcedbd81116d072c18e8f82a0d` |
| `.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md` | 66492 | `229ab4fbba9ad79f7226ff653beb4f69dafb8607dd9c2f8b72638b47813d5304` |
| `docs/parity/v2-6-shared-exact-work.md` | 26470 | `2869a4b711e1e3a5c0eee28c1c7c9638d7e91c83656d053060fc6bc33e0526f9` |
| `packages/open-bitcoin-node/README.md` | 5104 | `89e6f4cb33400b59ad24eb0ba0d530f0e44e805f1714a8edc70a7578330b731e` |
| `docs/parity/index.json` | 300274 | `bbcb82ab9359aa106c74e204630d3ea20f067fdcb0d4524c23754f3bd8f83d8c` |
| `docs/metrics/lines-of-code.md` | 27099 | `f872ffac693a2f88c9d5dde511f2ff54633f8da5ef5527b2cd632ac5b9d65493` |

All 335 source states were freshly rehashed and remain exact under source mapping `7354f4d8a5dd9378a7d2fde02e8dc975cd708fb2fbc59324b24ce03cd3c4345e`; all 41 plan identities remain unchanged. The complete register remains184/184 CLOSED:182 mitigations, exactly the two original accepted risks T-165-15-06 and T-165-16-06, and zero transfers or open threats. No waiver or implementation change is introduced. Networking remains default-off, all four ordinary selectors refuse and later166–168 support limits remain explicit. This appendix changes only this security report; Root will stage it for the normal mandatory hook.

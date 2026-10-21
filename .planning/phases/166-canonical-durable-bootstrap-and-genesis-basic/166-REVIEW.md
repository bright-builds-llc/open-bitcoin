---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
reviewed: "2026-10-21T10:57:05Z"
depth: standard
files_reviewed: 113
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
  - packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs
  - packages/open-bitcoin-cli/BUILD.bazel
  - packages/open-bitcoin-cli/src/bin/open-bitcoin.rs
  - packages/open-bitcoin-cli/src/lib.rs
  - packages/open-bitcoin-cli/src/operator/runtime.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase166_genesis.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs
  - packages/open-bitcoin-consensus/src/chain_spec.rs
  - packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs
  - packages/open-bitcoin-consensus/src/chain_spec/bootstrap/tests.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/canonical_genesis.rs
  - packages/open-bitcoin-network/src/header_store/canonical_genesis/tests.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis/tests.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network/canonical_admission.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/proof.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/qualification_tests.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/records.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/test_support.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/tests.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/basic.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests/copy.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/open_lease/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/checkpoint_metadata.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs
  - packages/open-bitcoin-rpc/BUILD.bazel
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/context/canonical_genesis.rs
  - packages/open-bitcoin-rpc/src/context/canonical_genesis/tests.rs
  - packages/open-bitcoin-rpc/src/context/canonical_genesis/tests/refusals.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/rescan.rs
  - packages/open-bitcoin-rpc/src/context/tests/construction.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/lib.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - packages/open-bitcoin-rpc/tests/phase165_work_startup.rs
  - packages/open-bitcoin-rpc/tests/phase166_canonical_startup.rs
  - scripts/check-current-documentation-reconciliation.test.ts
  - scripts/check-panic-sites.sh
  - scripts/check-panic-sites.test.ts
  - scripts/check-panic-sites.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/filesystem.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts
  - scripts/check-phase135-snapshot-recovery/source.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase163-provenance/native.test.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase164-canonical-identity/claims.ts
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/test-support.ts
  - scripts/check-phase166-canonical-bootstrap.test.ts
  - scripts/check-phase166-canonical-bootstrap.ts
  - scripts/check-phase166-canonical-bootstrap/contracts.ts
  - scripts/panic-sites.allowlist
  - scripts/phase166-genesis-vectors.test.ts
  - scripts/phase166-genesis-vectors.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-21T10:57:05Z"
diff_base: 1273b4fb
source_scope_sha256: 861a298d938014739c0f311a292e270bf268c10e6e635d7ad4520195ef516535
---

# Phase 166: Code Review Report

**Reviewed:** 2026-10-21T10:57:05Z
**Depth:** standard, with focused cross-module authority and effect tracing
**Files Reviewed:** 113
**Status:** clean — source-review PASS

## Summary

No actionable correctness, security or applicable mandatory-standards finding remains in the reviewed Phase166 source under the explicit accepted fixture/tooling exception described below. All reviewed files meet the scoped review requirements; no unaddressed issues remain. The complete official default verifier passes in manual05 on the unchanged113-file861basis, including the full workspace and fresh eight-package coverage. Earlier failed runs and audits remain failed. Canonical security/goal acceptance, lifecycle closure and normal-hook saving remain separate gates; no Git outcome is claimed here.

The scope combines actual tracked changes since `1273b4fb` with `git ls-files --others --exclude-standard`, including new modules omitted by a tracked diff alone. The exact113Rust, Bun, shell, Bazel and panic-allowlist paths are preserved above. New production modules, changed existing behavior, test reliability/assertions, executable registrations, current guard changes and narrow data/inverse changes were inspected in context. The scope digest is SHA-256 of sorted `path + space + SHA256(file bytes)` rows joined with newlines, without a trailing newline; the current basis was checked against the original review and the approved follow-up deltas below.

Supporting material includes the Phase166 context, decision coverage, sixteen-plan set, amendments in the latest PLAN-CHECK, relevant execution summaries, current README/node/CLI guidance, parity companion/catalog/vector/breadcrumb changes and actual local behavioral receipts. Planning artifacts, generated LOC and supporting documentation are not counted as source files above. Ignored local receipts were read as evidence, not treated as production source or immutable qualification inputs.

Guidance materially applied: repository `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` (including the subsequently approved existing-fixture panic exception), standards index, architecture, code shape, verification, testing, local guidance and Rust/TypeScript rules. Both canonical active lesson files were loaded in full: global6,898bytes plus repository1,958bytes, total8,856bytes and conservative estimate2,953tokens. No archive or project skill directory was loaded; neither project skill directory exists. The GSD code-review workflow is the review entrypoint; root retains all source fixes, acceptance and Git operations.

## Authority and Effect Review

| Boundary | Source assessment |
| --- | --- |
| Complete genesis identity | `chain_spec/bootstrap.rs::prove` compares the selected full canonical body before deriving target proof. The immutable proof has private fields and supplies no ordinary-history capability. Body/witness/output changes and foreign selectors cannot stand in for this proof. |
| Coins and headers | Core genesis construction requires an actual complete effective snapshot with B=genesis, no coins and no H markers. It bypasses transaction connection entirely. Header ownership uses the checked header insertion path with selected parameters; imported numerical entries remain unproved. |
| Durable publication | `storage/fjall_store/canonical_genesis.rs:87` prepares and validates encodings before one cross-keyspace SyncAll batch. B is the only coins row. A failed/uncertain commit returns no owner; later admission independently proves complete state or refuses. |
| Pre-effect reopen | `canonical_genesis.rs:132` obtains detached proof only through private-copy inspection, then opens the original under the retained application lease and repeats the same complete validator. No schema migration, history hydration, index or prune recovery precedes proof. The proof includes saved BASIC facts as well as genesis ancestry. |
| Live capability | `canonical_genesis.rs:44` consumes the sealed owner; `:52` binds the actual publication-owner instance, checks sequence on both sides of full reproof and refuses any changed store. The managed constructor rechecks before configuration and obtains core genesis authority from the actual coins view. |
| Operation-lease lifetime | The stable external lease is required by supported backend constructors. Store clones retain its Arc; the store declares it after engine/keyspace fields. The extracted coins view retains its store after its own database/keyspace fields. Actual snapshots remain borrowed/local or return detached domain data; no new public raw engine owner escapes. Partial construction destroys backend owners before the consumed lease. |
| Isolated recovery | The parent retains application and actual source engine locks while copying checked regular files into private independent scratch. The same executable receives one exact reserved argv and a capped job. Child raw recovery is confined to the private copy, with fixed panic refusal and detached bounded records. Parent success requires delivered complete result, exit0, source recheck, recovered-tree checks and checked exact-root cleanup before proof/admission. |
| Child uncertainty | `inspection/worker.rs:148` monitors job delivery and child completion under the centralized deadline; `:245` rejects nonzero/incomplete output. `:264` distinguishes confirmed reaping from uncertainty. Uncertain completion refuses admission and retains scratch. Confirmed completion joins the writer. The source contains no parent raw-recovery fallback or alternative worker lookup. |
| Runtime and BASIC | Canonical startup consumes its owner, compares the network and builds earned managed/header owners. BASIC reuses the existing accepted-state append/catch-up/publication engine. Fresh Disabled/unspecified absence stays absent. PreserveSaved maps saved Active to the existing Enabled path while retaining the saved generation; it does not create a second index engine. |
| Checkpoint counts and work | The shared metadata writer preserves an already-decoded count map only when the entire ancestry is equal, under the publication guard. Missing/legacy counts are not inferred empty; malformed bytes propagate before metadata invalidation. Complete 256-bit work is recomputed and compared across all stored endpoints. Raw public writes retain invalidation behavior. |
| Consuming RPC factory | `context/canonical_genesis.rs` accepts OpenedCanonicalGenesis by value and derives its own runtime, handle and store. Selector/peer-filter/activation preflights precede its sole runtime construction. Consensus parameters come from the proof; assembly effect paths bind to the admitted store. The ordinary supplied-handle constructor remains gated. |
| Wallet refusal | Explicit Unavailable has no wallet or registry owner. Direct queries/mutations, wallet selection and rescan entrypoints refuse before registry loads/callbacks; wallet_info is now fallible. Existing batched descriptor dispatch retains per-item failure semantics, and authenticated wallet scope refuses. No mempool recovery or usable empty-wallet substitute enters the factory. |
| Product and operator entrypoints | Daemon and CLI worker dispatch occurs before Tokio/config/clap startup. Bazel roots/dependencies include the extracted modules and direct node calls. All configured selectors retain matching selection. Every network activation request is refused before original store opening; no-trigger startup confers no transient authority. Offline Status uses existing-only canonical reopen and preserves saved mode; Pause/Resume/wallet/soak routes retain ordinary gates. |
| RPC hash representation | Both metadata-present and absent bestblockhash projections use display order. The existing block-filter normalizer converts that display string back to typed raw order. Independent four-selector roundtrip assertions cover both branches. |
| Deferred scope | Ordinary header/body ingress, imports, local/stored connects, reorg and sync activation remain refused, including missing Signet challenge validation. No Phase167 descendant qualification or actual Knots-daemon comparison is supplied. |

## Transaction-Count Consistency

The complete stored genesis body contains its one transaction. Its explicit empty `maybe_confirmed_txid_counts` map models the connected-transaction occurrence/recovery index, populated by ordinary transaction connection; genesis deliberately skips that operation. It is not a claim that the serialized genesis block contains zero transactions.

The currently registered RPC method enum exposes no chain transaction-total, getchaintxstats or getblockstats result that derives a total from that map. The reachable transaction-count observations in getmempoolinfo and operator resource status derive from the actual mempool's transaction count, independently of the genesis body and confirmed-occurrence map. No inconsistency was identified at this modeled boundary. This does not add a new statistics endpoint or qualify later history.

## Current Guards, Pins and Historical Evidence

The Phase166 checker uses a finite current input set, executable bodies/ownership, complete named predicates with required lexical depth, mandatory first operations and count-bound legitimate early-success branches. Its tests mutate unique real anchors, assert an actual source change and require the intended file/rule finding. Behavioral runtime qualification remains separately owned by Rust/product tests. This is bounded structural protection, not a general Rust verifier.

The reviewed current-owner relocations, process source identities, EDGES, SOURCE_CONTRACTS and enabled-test counts match the actual new opener/factory/observer routes. The Phase165 reader still verifies its exact allowlist before source I/O and retains regular-file/no-follow/size checks. The 307→312 membership change consists of the five named reviewed helpers; the actual copied-membership negative control rejects an unapproved sixth path before source-root I/O.

The exact verifier inverse removes only the approved current registrations and canonical TMPDIR preparation, preserving the earlier verifier base and historical snapshot as distinct generations. Original historical-tooling restoration remains exact. The two new current startup successors project to the original daemon owner solely for historical157 catalog membership; unrelated original evidence remains mandatory. No historical receipt, immutable provenance manifest or VERIFY_BASE_SHA renewal was found.

The final claim classifier retains the identified original164 subject/status ordering and paragraph domains and original165 flattened following-predicate domains, with the narrow whole-clause genesis exception. The reviewed follow-up checks every exact current166 section occurrence in README/support-matrix, retaining each section's existing next-H2 cutoff; a valid earlier section cannot hide a later offending section. The finite controls preserve bare reopen, generic whole-chain/production, phase, daemon and unrelated-negation/conditional restrictions. No broader language-policy guarantee is inferred.

## Behavioral Receipts Examined

Actual receipts were inspected alongside test assertions and source, including these representative acceptance boundaries:

| Receipt under `.local/open-bitcoin-dev/phase166/` | Observed evidence |
| --- | --- |
| `plan15/malformed-green.log` | Original malformed integer/overflow behavior test passes 1/1; both controlled children succeed with no signal, scratch absent and source preserved. The earlier raw-pin SIGABRT characterization remains a separate result. |
| `plan15/worker-final.log`, `protocol-final.log`, `harness-final.log` | Bounded process/codec/lifecycle controls and real valid records; no-job routing helpers remain distinguished from storage recovery. |
| `plan10/qualification-expanded.log` | 8/8 qualification tests; actual authority-binding corruption and all32 coordinated work-byte controls, plus four-selector Enabled/Disabled managed checkpoint and independent-child reopen. |
| `plan10/checkpoint-metadata-final.log` | 6/6 controls for identical empty/nonempty counts, changed ancestry, missing/legacy, malformed-before-invalidation and raw-writer revocation. |
| `plan16/focused-serialized-final.log` and affected final-source receipts | 21/21 focused cases, with actual sealed factory, wallet refusal, selector/activation preflights and all-handle reopen. The earlier ambiguous observer-lock failure is retained and has no invented proven cause. |
| `plan09/cargo-product-evidence.json`, `bazel-product-evidence.json` | Each actual Regtest product starts twice on its same path, reports the fixed display hash and BASIC filter/header, and exits0 through the supported SIGINT path. This is Open Bitcoin product evidence. |
| `plan12/process-final.log` | 2/2 final process tests: 28 retained refusal combinations and four admitted empty-directory selectors, each started twice with authenticated genesis observations and closed-store exact work. |
| `plan12/full-tests-audit.json` | The combined five-file command remains failed: 4579 passed /3 default-timeout failures, 4582 total. No cause is established. |
| `plan12/native-diagnosis-outcome.json`, `native-phase157.log`, `native-phase164.log` | Three exact unchanged/default-limit replays pass; complete unfiltered standalone157 passes771/771 and164 passes1190/1190. These are separate/overlapping diagnostics, not a relabeled passing aggregate. |
| `plan12/final-evidence-audit.json` and original full log | Complete accepted per-owner scope is161+771+1190+2020+440=4582 cases; all52 earlier reader-control labels actually pass. The report does not add the overlapping replays as coverage. |
| `plan13/uat-evidence.json` | Actual serial Cargo/Bazel existing-only status reports Regtest height0, exact64-digit work and one BASIC record; both missing-storage controls exit1 and retain absence. |
| `plan14/manual01/format-outcome.json`, `loc-outcome.json` | Pre-manual01 format and LOC generation both exit0 under pinned Bun1.3.9. The original source review followed the formatter; all72reviewed Rust files remained byte-identical through the prior107-file cutoff. The later single annotation-only delta is reviewed below. These receipts do not cover later native completion. |
| `plan14/manual01/native-outcome.json`, `native.log` | Actual default `bash scripts/verify.sh` exits1 after2265.244s. Native163 completes158/160 with two original55000ms aggregate-child timeouts; later164/165/166, Rust, coverage and Bazel are not reached. This run remains failed. |

No tests, Cargo, Bazel or second verifier were started by this reviewer alongside the owned native run. Read-only diff/whitespace and scope checks completed; no production, tests, pins, shared metadata, hooks or Git state were changed by this review.

## Approved Follow-up Review and Current Source Basis

The first refresh superseded the initial source cutoff at 2026-10-20T22:43:22Z. That92-file source set added only `scripts/check-phase163-provenance/native.test.ts` to the original91 paths. At that cutoff, five existing source paths changed: claims.ts, contracts.ts, historical-tooling.ts, test-modules.ts and the166 test. Substituting their exact saved pre-repair bytes into the current original91-file scope reproduces its recorded digest `c90d15d1cd1eb84313cc4f922071ac0340b3d50626d3147033de584aa76251a1`. Thus all72 Rust files and every other original source byte match the initial review. That prior92-file digest is `d283c52851ad174bbe1a53c416ccba77281fc3a03919709d616c9bf0734baff6`; the subsequent approved93-file refresh is described below.

The native repair was independently reviewed CODE-before-DATA against complete saved before sources. It splits the two historical-context aggregate tests into ten selected layouts for each archive-only/next-active context. Every child selects exactly one existing top-level case, checks its exact completed name,1pass/0fail and one-owned-root cleanup. The original two map, two filesystem and two CLI boundaries remain for each layout; original negative cases and55s child/60s outer/30s inner deadlines remain unchanged.160definitions become178; no coverage was dropped. The source contains no new fixture/source fallback.

The narrow CURRENT_TOOL_WIRING row restores the native file byte-for-byte to29613bytes/SHA `5297898680a15de12f79b7e6c9c8a814fdd0276522bfd071b96cd7b3a6f7ac16`. The complete tooling inverse changes only its current-generation identity plus one exact mapping-row removal, retains all prior hunk order and restores104671bytes/SHA `47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65`. Both verifier generations, copied-fixture inverse, immutable manifest and all historical snapshots remain exact. Independent read-only replacement reproduced the final DATA file using exactly the two approved existing SOURCE_CONTRACTS values; all other DATA bytes, row types/order/counts and312-path membership remain unchanged. The actual pre-renewal165 CLI refusal contained only those two stale identities.

The subsequent claim repair changes only first-occurrence current166 selection to iteration over all occurrences. Reversing that loop change reproduces the complete previous classifier. All447 pre-repair166 tests are retained verbatim, with42 named additions covering the real existing README paragraph and repeated README/support current sections, bad first/middle/last sections, bounded-good clauses and historical separators. No classifier vocabulary, preprocessing, genesis exception, DATA or inverse was broadened. Neither changed claim/test file has a SOURCE_CONTRACTS row; no further renewal was needed.

| Follow-up evidence under `plan14/native-timeout-fix/` | Actual result |
| --- | --- |
| `native-subset-receipt.json`, `native-subset.log` |20/20 selected context/layout cases pass; no new owned roots,172unchanged live witnesses, original deadlines. Targeted evidence only. |
| `regression.log`, `code-proof.json`, `data-audit.json` |7/7 restoration/tamper controls,44assertions; exact restoration and two-row DATA change. Actual copied missing/wrong-destination/altered-span mapping modules refuse. |
| `provenance163-complete.log`, `native163-complete.log` | Complete unfiltered124/124 provenance tests and178/178 native tests pass, without deadline override. |
| `bootstrap166-complete.log` | Intermediate full166 result remains failed437/447;10claim assertions exposed first-occurrence heading selection after the real README gained that heading. |
| `claim-sections/named-red.log`, `named-green.log` | Retained22pass/20fail RED, then42/42 new controls with90assertions. |
| `claim-sections/complete166-outcome.json`, `complete166.log` | Complete unfiltered final166 passes489/489,1014assertions, actual exit0 at2026-10-21T00:00:55.250Z, no timeout override. This is not a full default verifier result. |

Raw source hashes at the prior93-file cutoff (the later DATA renewal is recorded below):

| Source | SHA-256 |
| --- | --- |
| native.test.ts | `6ff915a4232152ab76822a2f5e87a87d956d0e5c61b63feb44b22cd7936f5fde` |
| historical-tooling.ts | `dfee49b1b0e8b78ebd967c82cf24c3e41c61872fc6fff29c6f708b12802c56fc` |
| contracts.ts | `057a7b326dabcd30a985863ee07374f13432ad737e9e89eb6050a3b73d109241` |
| claims.ts | `83ce51c844091e3902832632baf3d4618ab561ecaa65d0e46b8ca98b6e30493a` |
|166test, latest shared-writer repair | `e14a83658e3806bb9b7d92a199cf0d36c041829b2c2adeceaea4c039ebacf920` |
| test-modules.ts DATA, prior one-helper-row renewal | `0de185672a4ae33af0f727673b73b15685e80f7ca769e84f30d02d9e879659de` |

The tooling SOURCE identity equals its raw hash. The contracts SOURCE identity excludes leading masked commentary and is `f939d63203f9637dddd8cb614b4bfc8c7532dd93f1380c316a146bfc163e2fcb`; its raw hash is not the replacement DATA value.

## Documentation Setup Correction and Actual UAT

The initial supporting-document review missed the literal `mktemp /tmp` spelling: this host's /tmp points to /private/tmp, and the unchanged product lease correctly rejects a symlink ancestor. Earlier13 UAT used safe .local paths, so it did not exercise that snippet. README and the parity companion now use separate fail-fast mktemp and `cd -- "$genesis_review_dir" && pwd -P` assignments, followed by fail-fast mkdir/printf, each with literal `|| exit 1`. No export, global setting, parser or runtime exception was added.

Actual serial reproduction is retained under `plan14/setup-correction/results-v3/2026-10-20T23-57-44.245Z-61910/`, with `uat-terminal-audit.json` and entrypoint/log receipts. The old literal /tmp Cargo daemon exits1 with fixed unsafe_path refusal and an unchanged empty physical source. Its scenario wrapper exits0 because it successfully checks that expected refusal; wrapper0 is not product0. Concrete exit attribution is explicitly unavailable for that fast refusal.

The corrected /private/tmp path has actual documented Cargo fresh startup followed by independent Bazel same-path reopen. Authenticated genesis/BASIC facts match the fixed vectors. Both supported SIGINT shutdowns have actual entrypoint0 and directly waited concrete-daemon0, with no forced cleanup. Bazel's observed transient parent/inspection-worker pair before readiness is recorded; attribution occurs only after authenticated readiness and a fresh unique identity snapshot. Process disappearance is not converted to an unobserved grandchild exit code.

Both offline status commands exit0 with height0, all64work digits, BASIC enabled/safe height0 and one retained record. Both missing-storage controls exit1 and leave absence. The eight serial scenario outcomes and their scopes are explicit; these are macOS Regtest Open Bitcoin products, not Knots differential qualification.

The preceding23:56:46 harness attempt remains failed: its assertion reported the product compatibility diagnostic missing from the observed entrypoint output. Its actual product refusal1 is retained, and `first-refusal-source-postcheck.json` independently confirms the source remained empty after exit; that postcheck does not relabel the harness as passing. V1/v2 preparation and earlier rejected outcomes remain preserved. Final documentation reconciliation,1244-file breadcrumbs, managed checks and whitespace checks have actual exit0 receipts in `setup-correction/followup-*.log`.

Current supporting-document SHA-256: README `d4c71feeef59a46460ed229ccefa93a2136effc50a04a912062b3773307c514e`; parity companion `f714654047dba7eaf7e691a18857a71168673debb2306d4f9199523d14a04511`. These documents and UAT do not substitute for the required fresh default native run.

## Simplification Assessment

At the initial review cutoff, an independent simplification pass found no justified source change. The pure complete-record proof, store-bound sealed owner and consuming runtime factory represent distinct trust boundaries; combining them would let detached bytes or a borrowed fixture owner substitute for live authority. The single atomic publication path, one private inspection pipeline, existing runtime/context assembly and existing BASIC continuation owner avoid duplicate authority or index engines. The later redundant pure-genesis constructor check, exposed by fresh coverage, is separately reviewed below.

The same-executable worker avoids another binary/dependency while addressing the observed engine abort. Reaped versus uncertain child completion and checked cleanup are necessary distinctions. The small canonical PreserveSaved resolver preserves generic recovery behavior. Exact-ancestry count preservation fixes the actual shared checkpoint cause without an inferred-empty fallback or a new flush engine. These conclusions follow the reviewed source and behavior, not the coordinator's simplification notes alone.

## Manual02 Failure and Literal Copied-Writer Repair

Manual02 ran the complete default command on its recorded snapshot and exited1 at2026-10-21T00:57:42.279Z after3262.883seconds (verifier54m22.147s). Native163 completed178/178 with cleanup/source witnesses intact, then164 completed1183/1190 with seven copied-CLI inverse controls failing because generated contracts.ts was syntactically invalid.165, later166, Rust, fresh coverage and Bazel were not reached. All seven reported paired fixture roots are absent and the known owned commands are closed in `manual02/closure-audit.json`. Manual02 remains failed; neither earlier standalone passes nor later focused controls change that result.

The actual shared writer in `scripts/check-phase165-shared-exact-work/test-support.ts:187` inserted serialized replacement source through String.replace's replacement-string channel. The new quoted native mapping contains one dollar/backtick token. The eight-case diagnosis saved original, intended and actual copied bytes: in seven bad-inverse cases it expands the45909-byte source prefix and adds45907bytes, producing exactly the observed unterminated string. The missing-hunks control removes that token and stays valid. Read-only independent reconstruction matched every saved expanded output. Literal splice and callback insertion agree byte-for-byte; all14 affected actual164/165 CLI counterfactuals retain the required semantic raw-tooling refusal without parser errors. Diagnosis, exact seven-case RED0/7, eight causal controls8/8/281assertions and identity-checked cleanup/source witnesses remain under `plan14/tooling-mutator-diagnosis/`.

The independently reviewed CODE repair is exactly one callback replacement in that shared writer plus one named actual-helper regression in the existing166 test. The regression inserts literal dollar-dollar, dollar-ampersand, dollar-backtick and dollar-apostrophe tokens, compares the real copied file with independently spliced bytes through the existing loader seam before both imported checks, then retains both real CLI semantic-detail/exit checks. It is not an isolated demonstration of String.replace. Original semantic refusal assertions, the seven original cases, no-classifier checks, cleanup and deadlines remain unchanged. No public API, probe, module, input, fixture or historical normalization change was added. Removing the new15-line test and import expansion reproduces all489previous test bytes; the file remains626lines.

Actual `tooling-mutator-fix/regression-red.log` is0pass/1fail at the exact-byte assertion with the old writer; `regression-green.log` is1/1/20assertions with the callback. `original-seven-green.log` is7/7/105assertions. The normal165 pre-DATA CLI refuses exactly its existing helper ownership identity; this is retained in `helper-stale-cli-stderr.log`. Root subsequently authorized only that existing SOURCE_CONTRACTS value from `c36b88a02596291f1200cba6ffd6e02c5c304e83c1b027209894b2a570ca5372` to `a2d1dd28105468e4c82727c3aca5c293ccfb05ee650b428d31975c8a6fdf4c7f`. The helper begins at its import, so raw hash and normalized sourceIdentity are identical.

Independent read-only comparison confirms the final DATA file is exactly its prior bytes plus that one value replacement, with no row/type/order/count/membership changes. DATA SHA is now `0de185672a4ae33af0f727673b73b15685e80f7ca769e84f30d02d9e879659de`; `tooling-mutator-fix/data-audit.json` also records197SOURCE_CONTRACTS,148SOURCE_FILES,26TEST_MODULES,88TEST_OWNERS,617EDGES, unchanged312-path reader identity,13immutable files and unchanged complete historical native/tooling/fixture/verifier destinations. No inverse or historical pin renewal was needed.

At that prior cutoff, the complete source scope was93: prior92 plus the formerly unchanged `scripts/check-phase165-shared-exact-work/test-support.ts`. That93 SHA is `b49cf7d21a3e08f2898f0f79eb0e29a92e5ec1f89376daeb0d5c88e3421c5369`. Replacing only the166 test and DATA with their saved pre-helper-repair bytes, over the prior92 paths, reproduces `d283c52851ad174bbe1a53c416ccba77281fc3a03919709d616c9bf0734baff6` exactly. All72 Rust files and every other prior source byte were unchanged at that cutoff. The helper raw/normalized hash remains `a2d1dd28105468e4c82727c3aca5c293ccfb05ee650b428d31975c8a6fdf4c7f`; final166 test hash remains `e14a83658e3806bb9b7d92a199cf0d36c041829b2c2adeceaea4c039ebacf920`.

Fresh serial preflight receipts live under `tooling-mutator-fix/preflight01/`. The five current155/157/164/165/166 CLIs, vectors, current-documentation reconciliation,1244-file breadcrumbs, managed checks and whitespace have actual exit0. The complete unfiltered166 suite actually exits0 at01:15:06.623Z with490/490 and no timeout override. At this review's01:15:32Z cutoff, the unfiltered164 suite is running (started01:15:06.638Z);165 is queued serially. Neither is claimed to pass. Root requires all three complete owner suites before manual03, followed by a fresh full default native run. Manual03 has not started at this cutoff. Focused/owner preflight is not full native acceptance, and no hook or Git saving outcome is inferred.

## Manual03 Failure and Legacy Guard Compatibility Review

The earlier serial preflight subsequently completed on its93-file basis:166490/490,1641190/1190 and1652020/2020, all unfiltered with default limits,3700 cases and11335assertions. Actual164 exit0 is at01:30:54.598Z and165 exit0 at01:52:22.418Z in `tooling-mutator-fix/preflight01/complete{164,165}-outcome.json`. These historical owner passes do not establish acceptance of the later guard changes.

Manual03's actual default verifier exits1 at2026-10-21T03:10:15.440Z after4632.935seconds (verifier1h17m12.164s). After its earlier guards pass, the old123 suite reports39pass/63fail of102 with the live configured-activation baseline finding. The moved canonical opener was absent from that checker's fixed physical reader. Later legacy guards, Rust checks/tests, benchmarks, Bazel and fresh coverage were not reached. `manual03/native-outcome.json`, `native.log` and `closure-audit.json` retain that failed result; the exact-prefix cleanup inventory found no matching temporary roots but did not invent per-test names or completion evidence.

The serial17-CLI diagnosis at `legacy-cli-baseline01/results.json` reports12exit0/5exit1 on the frozen93-file basis. Failures are123activation,127production composition,128shared outboxes,129delegation of127/128, and135producer settlement. Actual original owner suites report12713/31,12819/99 and13559/90. The independent review traced each mismatch to the changed real owner and caller shape, retained ordinary behavior, and required CODE approval before any DATA renewal.

| Guard boundary | Independently reviewed final behavior |
| --- | --- |
|123 configured activation | The fixed reader requires declared `canonical_startup.rs` while retaining34logical targets,35physical live files and the original34-file synthetic fixture. The old ordinary three-argument constructor, lower configured/fixture activation contracts and original102 test concerns remain. The sealed GEN branch requires actual synchronous dispatch, selected production opener, both store bindings, exact ordinary None arguments, five activation flags, preflight/history refusal and consuming factory/assembly. |
|127 shared authority | Independent mode classification prevents a broken declared GEN branch from falling back to the old ordinary constructor. The selected real entry/application/serve owners remain mandatory. All physical production daemon helpers, including untracked files, are read once and retained in duplicate/transient/ManagedPeerNetwork constructor scans. The exact old ordinary context and original durable-serving, projection and integration concerns remain. |
|128 shared outboxes | The pure shared composition predicate proves the selected sealed factory, preflight, returned context and runtime assembly. Network, peer epoch and outboxes derive from the same sync runtime. The checker separately requires the exact real listener arguments; an arbitrary matching token does not replace owner composition. Old inline ordinary composition remains explicitly exercised. Original durable-save/active-tip proof bytes are unchanged. |
|135 producer quiescence | Optional retry is allowed only with the exact derived `canonical_genesis` predicate and ordinary worker constructors in the opposite branch. Declared GEN cannot choose the old mode as fallback. Selected real production bodies and the exact shutdown call preserve sync, coins, retry and final-checkpoint settlement before retained-error refusal and clean marking. Original persisted-input/source helper prefixes and90old concerns remain. |

The new shared predicate stays pure and acyclic in the existing128durable-trigger module. New127controls reside in its existing production-proof test owner; new135source predicates and mutation data use existing seams. The independent simplification pass supports sharing the common GEN proof while retaining phase-specific listener and settlement checks. Every changed file is at most628lines; no dense packing, new module/member, generic source-normalization fallback or acceptance exception was introduced.

The P127 runtime determinism scan now reads the named checker raw and explicitly includes its new runtime proof dependency. It excludes only the test-only proof module, whose owned local Git fixture uses actual Bun.spawnSync. Staging/untracked invariance, missing-helper refusal and a genuine forbidden runtime-helper effect token are exercised. Other corpus reader semantics remain unchanged.

Independent CODE review blocked an intermediate candidate because raw global path/Tokio attribute text could be supplied by a comment or literal while the real declaration was wrong. Final code binds each attribute to the selected real production declaration's offset, scans contiguous outer attributes with offset-preserving comment/literal masking, and compares the actual original attribute slice, including the path literal. All8combined decoy controls actually fail before the fix and pass after it. Original mode/owner controls remain; no reported finding is left open.

| Final repair receipt under `plan14/` | Actual evidence |
| --- | --- |
| `phase123-owner-decoy-fix/{decoy-red,decoy-green,full129-final,final-cli129}.log`, `code-audit.json` | Three selected-owner decoys RED0/3 then GREEN3/3; complete123129/129,461assertions and normalCLI0. Original fixture bytes and102 registration concerns remain; the intermediate Bun done-callback harness failure is retained separately. |
| `legacy-guard-batch01/final01/outcomes.json` | Original candidate component run exits0:12767/67,128137/137,135108/108,12912/12. These passes preceded the independent attribute-binding BLOCK and do not cover it. |
| `legacy-guard-batch01/attribute-{red,green}-{127,128}.log` | Exact4+4combined real-path/Tokio mutation plus expected comment/literal decoys:8RED failures, then8GREEN semantic refusals. |
| `legacy-guard-batch01/final02/outcomes.json` | Terminal complete/passed at04:22:48.928Z; all9commands exit0/no signal and changedCode empty. Full12771/71,281assertions;128141/141,381assertions;12912/12,12assertions. Four unchanged135source hashes independently match final01, retaining108/108,191assertions. Combined332cases/865assertions are component evidence only. Four live CLIs, managed checks and audit also exit0. |
| `combined-legacy-data01/{before,after}.json`, `eight.patch` | Actual pre-renewal165CLI refuses exactly8existing stale ownership identities. The fresh-process post-renewal audit has no findings. Two ignored audit-harness precondition errors remain in `harness-errors.json`; neither is relabeled a product or source failure. |

Root then authorized only the eight existing ordinary SOURCE_CONTRACTS ownership values:123checks/test,127checker/test/production-proof helper, and128checker/test/durable-trigger helper. Independent byte comparison authenticates the before file as SHA `0de185672a4ae33af0f727673b73b15685e80f7ca769e84f30d02d9e879659de` and reproduces the actual final file using exactly those eight approved replacements. Every other DATA byte is unchanged, including row/type/order/membership. No row was added for123filesystem or any135file. The current DATA SHA is `2af57987dcc455b12797d06a37c6be4dac168f1550681732b29e26d8db09aa87`.

Counts remain197SOURCE_CONTRACTS,148SOURCE_FILES,26TEST_MODULES,88TEST_OWNERS,617EDGES and312reader paths, with reader identity `a9dd9428ce03f38d35972f6b14a07daf48fb5da17062231ed229abe292bb4a56`. All13immutable files, complete historical native/tooling/fixture destinations and both verifier generations remain unchanged. All13approved CODE hashes match the actual frozen final source; raw hashes and normalized identities coincide for these import-leading files.

The post-legacy scope was exactly106: the prior93 plus the13legacy CODE paths listed above, with no removals. The initial post-DATA basis in `combined-legacy-data01/source-basis106.{json,txt}` has digest `4906d20d033065d2ef39654a33cdd2e1054d5c5130b157a201561dbae42fa44e`. On the prior93 paths, only the eight-row DATA file differs from the saved93-file witnesses; all72Rust files and every other prior source byte remain exact.

The actual post-DATA `fast-closure.json` records23Bun checks exit0, followed by git diff --check exit2 for one surplus EOF blank line in135source.ts. The failed aggregate remains failed. Root removed exactly that final LF under the saved `32d0eb5263e8cd0b2935175139991089d309724944747575d2184851d04ab215` precondition. Independent comparison proves before bytes equal final bytes plus one LF; final135source SHA is `e64cf7fc329800a6ad2c3c10ab31d3510e7f0a673192304672e43ac24eee82c1`. No runtime, DATA, inventory or substantive guard behavior changed, and no full-suite repetition is inferred. Actual `post-eof-outcomes.json` records135CLI0 and diff-check0 at04:30:01.354Z.

At04:30:41Z, independent recomputation of all106raw file hashes exactly matched `combined-legacy-data01/root-approved-source-basis106-v2.json` and `source-basis106-v2.txt`, yielding source digest `8fddb7a15c10c4838a710225ee15f4f40c1cbfadc98b6c376339a7f96f5e32fc`. Source review was clean on that basis. The subsequent component attempt and final107-file basis follow.

## Suffix01 Failure and Documentation Fixture Repair

`suffix-preflight/suffix01/32-outcome.json` records actual exit1 at2026-10-21T04:34:02.268Z: current documentation reconciliation tests report19pass/1fail of20. The old mutation needle no longer exists in the architecture Sync Boundary, so `replaceInFixture` fails its needle-presence precondition before the checker and intended negative assertion run. `suffix01/finite-audit.json` authenticates the recorded logs and unchanged106-file `8fddb...` basis. Only the prior31suffix steps pass on that dated basis; step32 and the component aggregate remain failed. Steps33–51, including Rust, benchmarks, Bazel and fresh coverage, were not reached. Known commands are closed; per-test root names were not logged and are not inferred.

The independently reviewed correction changes exactly one string literal at `scripts/check-current-documentation-reconciliation.test.ts:122`: it selects the complete current two-line clause, `These are not public defaults and do not authorize the` followed by `current canonical route.` The replacement remains `These remain later v1.2 phases:` and the exact `architecture current sync boundary` failure assertion remains. This still removes the required current public-default boundary and inserts the forbidden later-v1.2 language. Independent reconstruction from `doc-fixture-anchor01/test-before.ts` reproduces the final file with that one literal replacement; all other bytes,20cases, helper and cleanup remain unchanged. No new helper or normalization is justified for this stale fixture anchor.

The original focused RED is retained in `doc-fixture-anchor01/focused-red.log`. Actual `outcomes.json` and `full20.log` record full20/20,45assertions,164ms and exit0 at04:36:24.966Z, followed by current-documentation CLI0, diff-check0 and managed-check0. Test SHA changes from `40da6ffd3aee64483b8a8246dada5a98b83bcd59ec83140daf7cbf7a8f8795ff` to `2fbad76d3c61fd5bfa2a8324b01d1d375f5735093e6c9211115929231d8692bc`. This test has no SOURCE_CONTRACTS ownership row and is not a312reader member; checker, documentation, runtime, DATA, inventory and inverse bytes remain unchanged.

Independent tracked-plus-untracked recomputation proves the final scope is107: every prior106raw witness is unchanged, and only this test is added. Exact rows match `doc-fixture-anchor01/source-basis107-v3.txt` and `root-approved-source-basis107-v3.json`, yielding source digest `9cec5811d1fce25004ed26e9af040058bd72b119dec7b217ae0001e675fe3cc3`. Root independently accepted that basis. At this04:37:19Z cutoff, the remaining20component steps (32–51) have not started; fresh full default native04 remains mandatory and pending. The failed suffix01 attempt is not relabeled by the passing focused owner suite.

## Suffix02 Failure, Panic Classification and Accepted Fixture Boundary

The actual suffix02 records show steps32–37 exit0 on the107-file `9cec...` basis. Step38, the unchanged old panic shell guard, exits1 at2026-10-21T04:40:04.972Z after113.526seconds. Its192reported sites are all test code: node/lib.rs3, canonical bootstrap qualification_tests.rs34 and test_support.rs95, and canonical-genesis basic_tests.rs60. The split children inherit real cfg(test) owners; the three lib findings belong to the selected #[test] function inside the harness macro. No reported site among those192is ordinary production code. Suffix02 and its failed38remain failed; steps39–51 were not reached.

The old awk scanner also exits at the first cfg(test) attribute, hiding any later production item. The reviewed repair keeps the existing shell entrypoint as a thin checked `bun test` invocation followed by `exec` of its Bun classifier. Unit tests use the inner CLI/pure API, so the wrapper does not recurse. Classification masks actual balanced cfg(test)/test-owned item spans and resumes scanning afterward. Comment/literal text supplies no ownership. Lexically incomplete or unbalanced input, missing files/helper, unsupported ownership and read failures refuse instead of granting partial exclusions. Ordinary attributes stay scanned without requiring an irrelevant test-item grammar; generic commas, fields, lifetimes and adjacent production are covered by retained regression cases.

Only the three named split files receive new whole-file exclusions after their unique live cfg/path ownership chains are proved. The test-only intermediate canonical_bootstrap/tests.rs owner is also protected against ordinary aliases. Ordinary module/include aliases to any protected node refuse; unsupported absolute/Windows/UNC source paths and inline path overrides fail closed. Independent review blocked intermediate candidates for absolute aliases and aliases to that intermediate owner. Each exact2-case block has actual RED0/2 followed by GREEN2/2 receipts; none is left open. The existing tests.rs/tests-directory panic waivers remain an explicit legacy policy boundary. No new *_tests filename waiver or general Rust/module-graph guarantee is claimed.

The optimized source enumeration independently matches all1209regular first-party src files in the saved original find result. Default and explicit absolute allowlist paths retain their behavior; findings retain actual path, line and original source text. Allowlist matching remains exact path plus literal needle. The existing verify.sh command is unchanged; tests and classifier run through that ordinary entrypoint, with checked failures and no process-substitution error swallowing. No verifier registration, inverse, input membership or DATA renewal is needed for the guard.

Correct continuation exposes one genuine pre-existing production-compiled panic in the public explicit fixture constructor, formerly hidden after a test module. `build_fixture_wallet_state_with_store(config, None)` can open Fjall for a configured registry; network configuration and fixture recovery can also fail. The old memory-only rationale is not an infallibility proof. Current callers are test-only or the explicit benchmark fixture alias; ordinary and sealed-genesis startup use separate fallible constructors and do not call this API.

Root deliberately accepted only this existing fixture/tooling fail-fast contract: one exact path/expect-literal allowlist entry, the narrow standards-overrides.md row and threat `T-166-14-03`. The phase threat inventory is now42, including this accepted exception. Invalid fixture configuration, storage, network setup or recovery can still panic. The public Panics documentation names `from_fixture_runtime_config_with_store(config, None)` as the fallible alternative, and the old clippy rationale is corrected. This is a compatibility exception, not zero risk or permission for new panic sites. The pre-existing diagnostic literal is retained without treating its wording as proof.

Independent whole-file reconstruction proves that canonical_chain.rs changes only the clippy-reason string and five rustdoc lines outside function regions. All16signatures/bodies and the existing panic literal remain exact. Thus71of the original72Rust files retain their raw bytes, and the remaining file has only an attribute/documentation delta. Its new raw SHA is `339d21f030a5d971b13c9d6c137c1ba6c22a8bea7a8f2e8285ff5c4eb99bda5b`; function EDGES and all DATA remain unchanged.

| Evidence under `plan14/panic-authority01/` | Actual outcome |
| --- | --- |
| `diagnosis.json`, `original-source-set.txt`, retained candidate logs | Original192test-site failure and earlier parser/generic-span failures remain preserved. The corrected pre-policy corpus still reports the single genuine fixture panic; it is not silently classified as a test. |
| `absolute-alias-{red,green}.log`, `intermediate-alias-{red,green}.log` | Both independently reported2-case ownership gaps fail before their fixes, then pass with the required refusal. Windows/UNC and ordinary relative-alias controls remain in the final suite. |
| `final-outer-repository-green.log`, `final-outcomes.json`, `final-driver.log`, `final-cleanup.json` | Complete51/51unit tests,69assertions. Actual repository outer wrapper exits0 under the accepted exact fixture exception. All12recorded outcomes have their expected exits/no signals: explicit-boundary positive0; altered path/message, unlisted production before/after, malformed input, missing helper, absolute alias and intermediate include controls1;165CLI, managed and diff checks0. All eight owned scratch roots are absent. Negative exit1 outcomes are expected refusals, not successful product starts. |
| `final-code-audit.json`, `actual-six-path.diff`, `root-independent-final-audit.json` | All six reviewed path hashes match, the Rust annotation-only reconstruction is exact, and the single allowlist append plus single standards override are bounded and honest. All13immutable files, complete inverse destinations,197owners/617edges and ordered312inputs remain unchanged. |
| `root-fast-cli-check.json` | Root's actual164and166CLIs both exit0 on the111-file basis, separately from the recorded165CLI0. No runtime qualification is inferred. |

Final guard hashes are shell `09b8daf3f9d6dca84aa7d377ba727ec94bd8e50f5e16793e1016d55b9c131f81`, classifier `12fd59f7463b1e1b91fdb13f3cd984d070db7ed3108db69f29182a9befe14cef` and tests `73a0ab8e61737c2e6311d1efa4b6b45599003331feb8cbd072d08d266fef1887`. The panic allowlist is now counted as executable policy input, SHA `502fd50ceaf0a4d731679c698507f7e26cd25e6d583fccff2a9878b51dbe3693`; supporting standards-overrides.md was reviewed separately, SHA `d5039a0c7e0f708fb27efe5814c9d260cca8724c3a408ca7f298d289dfe3d42e`.

At05:08:42Z, independent scope recomputation was111: prior107plus the shell, classifier, test and allowlist, with no removals. Among the prior107paths, only canonical_chain.rs has the reviewed annotation delta. Exact rows match `panic-authority01/candidate-source-basis111.{json,txt}` and root's accepted audit, yielding `6a57141979a0c9cbd8b1ed441087a59f9786e318e21677db64a07e05e0e8fa5e`. DATA stays `2af57987dcc455b12797d06a37c6be4dac168f1550681732b29e26d8db09aa87`. The simplification review favors per-item classification and the existing exact exception mechanism over blanket file waivers, Rust test-module churn or a broad fixture-API migration. The subsequent component failure and final112-file basis follow.

## Suffix03 Cargo Failure and Checkpoint Test Needle Repair

Actual suffix03 steps38–42 exit0 on the111-file basis: the outer panic guard, Cargo format, shared-fixture format, Clippy and all-target build. Step43's workspace/all-features Cargo test command exits101 at2026-10-21T05:51:24.906Z after2410.162seconds. Its24executed target footers total4843passed,1failed and6ignored; this is a failed, incomplete workspace command. Node's2040/0/4ignored and RPC library's409/0/1ignored are passing target results within that failed aggregate. The daemon target reports117passed/1failed/1ignored. No compiled artifact or unexecuted target is credited as a test pass, and steps44–51 were not reached.

The four-selector local durable-trigger matrix actually reports ok in that daemon run. Its48operations cover four selectors, three trigger sets, missing/empty starting paths and two successive opens per path. These are48operations inside one executed test, not48additional Cargo test cases. The aggregate still fails solely at the checkpoint source-inspection test.

`tests::checkpoint::daemon_shutdown_orders_all_producer_joins_before_checkpoint_settlement` includes daemon source; it has no runtime/GEN fixture setup. Its line389searches the obsolete mandatory retry closure and fails its find/expect before ordering assertions. Current production uses the already-reviewed optional retry shutdown callback in the same sync/coins/retry/checkpoint settlement call. The independently reviewed repair changes only that one expected string literal to `|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),`. No product change, ordinary-join relaxation or GEN worker creation is justified by this failure.

Independent reconstruction from `checkpoint-order01/test-before.rs` proves all other bytes,13test registrations, HTTP/inbound/metrics/coordinator/sync/retry/checkpoint ordering assertions and error-before-clean gate remain exact. The existing behavior tests continue to assert all settlement callbacks despite errors, HTTP failure withholding clean, and clean marking only after successful settlement. The minimal source-needle rebind preserves those concerns without another helper or API.

Actual focused RED exits101 with0pass/1fail on the111basis. `checkpoint-order01/exact-green.log` records1/1exit0 at05:56:26.826Z; `module-green.log` records the complete checkpoint module13/13exit0 at05:56:29.418Z. Single-file rustfmt check and diff-check exit0. These focused/module results repair the owning concern and do not relabel the earlier workspace command. Test raw SHA changes from `b166ce368485a7359f3fc5df93d89cdade6448097daa3e7cffded5b20f06f829` to `28732d972c3a07c5c273cd9e2d1443f1456f6f4891ed6bf849179883193391ff`.

The final112scope adds only this formerly unchanged Rust test file to the prior111paths, with no removals; every prior111raw witness is unchanged. Independent recomputation matches `checkpoint-order01/candidate-source-basis112.{json,txt}` and root's accepted basis, SHA `21eab7cf953bb55bd17820ecd6e40e3751b9bb472bb27fbdcce592fa0f01c1a4`. The reviewed Rust scope is now73files:71original raw witnesses, one annotation-only canonical-chain delta and this single-literal test delta. All production runtime bodies remain unchanged by these follow-ups. The test has no SOURCE_CONTRACTS/EDGES/fixed312ownership; case counts, DATA, membership and inverse destinations remain unchanged and need no renewal.

At this05:59:02Z cutoff, the proposed inventory of remaining Cargo targets is awaiting independent/root coverage review. Only actual executed per-target footers count; compiled-only discovery supplies no pass credit. Remaining targets and workspace doctests require actual execution under the native Cargo flags, and any unresolved target mapping requires a complete step43 retry. Remaining component acceptance and whole default native04 remain pending; the full default verifier still requires its own fresh workspace run.

## RPC Display Expectation and Two-Value DATA Renewal

The read-only Cargo inventory has29default harness artifacts, while the original failed workspace log has24uniquely matched Running headers and result footers:23whole targets pass and one daemon target fails. Five whole harnesses and13workspace doctest candidates had not run. Compilation supplies no pass credit. Independent review accepted four compile gates requiring exact selected target/package/source/profile/features/executable bytes before native Cargo execution, with full step43 retry on any mismatch or unknown mapping. Cargo continues to own runtime environment and package working directories; no direct executable substitute or test filter earns whole-harness credit. Official native04 remains a fresh full-workspace gate.

Remaining01's first black-box compile gate actually matches the original workspace executable bytes. Its native Cargo run then exits101 at06:14:36.713Z with4pass/1fail of5, on the112basis. The sole assertion compares RPC bestblockhash with raw-order hex; actual and expected strings are exact32-byte reversals. Both reviewed RPC metadata branches already emit display order. Pinned Knots blockchain.cpp1792uses GetHex, and uint256.cpp11–18reverses internal bytes before hexadecimal rendering. This source proof supports the existing Phase166projection; no production revert is justified and no actual Knots-daemon differential is inferred.

The shared test encoded_hash helper remains raw because the internal sync-summary assertion also uses it. The final628-line test repair adds only a local32-byte copy and reverse, then independently formats fixed two-digit lowercase byte pairs for the existing HTTP expectation. Every other byte, all state/height/authority/schema/cleanup assertions and all three test registrations are preserved. The file contains seven function regions total: the changed composition test and six byte-identical functions. All21relevant existing mutation rows remain exact:13direct composition,7shared-fixture and1parent registration. Two earlier formatter-only candidates expanded to633and630lines; both are retained and were never applied. The final layout adds no helper/import, blank deletion or dense statement packing.

Actual `rpc-display01/focused-green.log` passes1/1, then `whole5-green.log` passes the complete unfiltered workspace/all-features black-box harness5/5 at06:26:21.036Z. The focused case overlaps that whole5 and is not additional coverage. Raw test SHA is `200b3f4d828431c410458f84ffb2ee7b54ecaebbfa0a0f8f53ea8df0f1ccfced`. The provisional CODE-only113basis is `993b5ce1a4373c8674dc4db28dc9cdaa5f3c3ad6529648660da328e6b9fd03d8`; the later DATA changes below supply the final basis. Remaining01 stays failed and is not relabeled by the subsequent whole-harness pass.

CODE approval preceded root's authorization of exactly two existing values: normalized SOURCE ownership `577591de8b0907916b4024d104369e4289f7390eaa7fa02c4c1b213c26e1847b` becomes `2b918c867d6cb525f4f11882a0f18eed5186a143268a04974826a865f22459e4`; the composition function EDGE `e468b4cc027ac9f888d7f6fc83a2e663edae527534fa4b1cd8f611d777f505d5` becomes `30c4f78f0fa00de0d1fb37139c87c3fd6c646911004464389a3edfd9e1d2daa3`. The SOURCE identity excludes leading parity commentary and is not the raw file SHA. The EDGE hashes the literal-bearing fn region; ordinal0and mechanicalfalse classification remain unchanged. Actual pre-DATA164and165CLIs refuse exactly that boundary and ownership respectively;166passes.

Independent whole-file reconstruction from `rpc-display01/data-before.json` proves one approved value replacement in each DATA file and no other byte change. SOURCE_CONTRACTS remain197, SOURCE_FILES148, TEST_MODULES26, TEST_OWNERS88, EDGES617and ordered inputs312. All13immutable files and complete inverse destinations remain unchanged. Final test-modules.ts SHA is `24ea4a2a6c03a62e80056eec4e51b07e9806e9e7a57141af35fa3205f231e328`; source-files.ts SHA is `4ee7948d3b50e1c03f2adaa8b614bc61a29f34cbd4959adbf1eedab95ab01f4b`.

The first ignored copied-control driver exits1 at06:36:29.782Z with21pass/1fail of22. It incorrectly requires a164finding for the parent registration row before invoking the existing helper. Inventory presence does not imply such a contract: the parent is in both inventories, but164has no EDGE or TEST_MODULES declaration row for it;165owns its source registration. This failed aggregate remains failed. Qualified evidence reuses the paired positive plus20actual paired negative rows (42CLI children), then adds an existing-helper165parent refusal and a separate actual copied164CLI0scope check (two more children). The targeted follow-up passes2/2with21assertions; it does not create a passing22/22aggregate. Total actual copied CLI children are44, with the distinct checker scopes preserved.

`rpc-display01/final-verification.json`, `parent-registration-*.json/log` and `final-closure-checks.json` record actual post-DATA164/165/166CLI0, managed0 and diff0, ending06:42:04.159Z. Original temporary roots remain unchanged and no new owned roots remain. Independent recomputation matches all113current raw files and exact `source-basis113-final.txt` rows, yielding `e0bd2fb6898655fab69032bc0f56a8a1f1c78ebf3e71698b70cfce19dd2eff4e`, accepted in root-final113-audit.json. The scope adds only phase127_composition.rs to the prior112paths; current Rust scope is74files:71original raw witnesses, one annotation-only delta and two test-only body deltas. Production runtime bodies are unchanged by these follow-ups. Four other harnesses,13doctest packages and components44–51have not run at this06:43:32Z cutoff. The remaining02proposal and whole native04 still require their actual outcomes.

## Completed Component Cargo Run and Final Pure-Genesis Simplification

Suffix04 executes the full original workspace/all-features Cargo43 command on the historical e0bd113basis and actually exits0 at2026-10-21T07:37:14.053Z after2472.959seconds. Independent raw attribution confirms29unique harnesses with29footers and13doctest package headers with14footers. Chainstate rustdoc legitimately emits two success batches,2+21. All43complete raw footer records match the supplemental corrected audit:4957passed,0failed,6ignored and0filtered, including actual doctest cases and zero-case package outcomes. Source113/native bindings, all13workspace manifests and known-process closure were verified. No unexecuted or compile-only target earns pass credit.

The original ignored suffix04postprocessor expected42footers and stopped despite Cargo exit0. Its failed audit/results remain untouched. `suffix-preflight/suffix04/corrected-workspace-scope-and-closure.json` corrects only data attribution, with raw log SHA `2fc5562c39e5554d7320ad18c4429034f4b3c6b9d9a264b2df3d34b9a41cfa94` and corrected receipt SHA `647628a847863a98019fca6576b2fa81f8e2c21e41aa828bb20bc9fb077301d4`. This is genuine component Cargo success, not whole native04 acceptance or a relabeled earlier failure.

Actual suffix05components44–50exit0 on that e0bdbasis: CLI RPC-client tests, benchmark list/smoke/report, Bazel build/provenance and llvm coverage clean. Component51's coverage process also exits0, but its fresh report retains one `Uncovered Lines:` entry at core engine/canonical_genesis.rs44, so the coverage acceptance check fails. The old report remains preserved, SHA `8ec29f0386f926f609ece96fd1a885053df4b51fff5cebf3472b7cc964bfce81`. Neither that source's workspace/component passes nor this process exit alone qualifies the final changed source.

Independent deep review proves the uncovered error branch unreachable through supported safe APIs. ChainSpec has a private closed four-profile selector; every pinned genesis header unconditionally has a null parent. CanonicalGenesisProof's sole constructor first compares the complete canonical header/body, owns a clone and exposes only immutable references/value copies. Fields are private, with no Deserialize, Default, mutable accessor, alternative constructor or interior-mutable header. Existing all-selector altered-parent tests reject nonzero parents at the proof boundary. ChainPosition::genesis merely calls new(header,0,work,time), then rejects a nonzero parent; that additional rejection cannot occur after this sealed proof.

The final two-file repair therefore calls `ChainPosition::new` with explicit height0 at this trusted boundary, with one why-comment, and adds a null-parent assertion to the existing four-selector earned-root test. Public Result, complete_snapshot()?, all three coins guards and their genuine errors, exact proof work/time/header, Live work binding/acceptance and all nine existing test cases remain. The general fallible ChainPosition::genesis API remains unchanged. This removes redundant validation rather than suppressing coverage, merging lines, fabricating invalid proofs or widening authority. Exact diff and frozen-file evidence live in `pure-genesis-coverage01/`.

Actual ordered checks on the final source pass: format; strict workspace/all-target/all-feature Clippy; workspace/all-target/all-feature build; core genesis9/9; consensus chain-spec40/40; current164/165/166CLIs; managed checks and diff-check. Fresh llvm clean exits0 immediately before the exact native eight-package coverage invocation. Coverage exits0 at07:56:59.398Z, with16actual test footers totaling1832passed,0failed,0ignored and0filtered. Independent report parsing confirms all eight requested package headers and no `Uncovered Lines:` heading. The fresh report is2,438,891bytes, SHA `fd042b00ee47b1649adcb598fe14a7cf06a01cbde613f9c2649cb1cf34c2e600`; no branch-coverage or whole-workspace claim is inferred from this line-coverage contract.

Final raw core SHA is `785e5abb9ddd00bf2c99e7ec627e3e4d7bcd3b77df37878b722279597bf6ce11`; its test SHA is `9dd8a2f971d6357fe117256fc5a1f6ffe0a496044086033930fb4ac527490766`. Independent recomputation confirms only those two of the prior113files change, giving final source SHA `861a298d938014739c0f311a292e270bf268c10e6e635d7ad4520195ef516535`, accepted in `pure-genesis-coverage01/root-independent-source-audit.json`. The path list/count remains113. No SOURCE_CONTRACTS/EDGES/ordered312/inverse renewal is needed or performed; both files were already in the reviewed scope, and current166complete-coins contracts remain intact.

Current Rust accounting is74reviewed files:69initial raw witnesses remain exact; one canonical-chain file has annotation-only changes; one core production constructor and its existing test have the reviewed simplification/assertion; two later-scoped legacy tests have expected-value changes. All other production code remains unchanged by these follow-ups. The accepted fixture/tooling panic exception remains bounded and separate. The earlier4957workspace pass belongs to e0bd; official manual04 must execute its full default contract on861. At this08:01:01Z cutoff, source review is clean, while full native04 acceptance and canonical security/goal verification remain pending.

## Official Manual04 Failure and Manual05 Completion

Manual04 runs the full default verifier on113/861and exits1 at2026-10-21T08:41:30.706Z after2391.930seconds. Phase163 native completes176/178with two next-active historical-builder failures: audit-in-progress reaches its selected child's30000ms deadline while the Phase158 CLI is running; final reaches the outer55000ms process timeout. The former log records a nested SIGTERM and the latter ETIMEDOUT. Other cases pass, including both layouts from archive-only controls. The final summary reports178owned roots removed and live source witnesses unchanged; it does not prove cleanup of an unprinted nested root. `manual04/native-outcome.json`, `native.log` and `terminal-audit.json` preserve the failed result and finite process/root observations. Three other prefix-matching temporary roots were unattributed and left untouched.

The two exact parent cases subsequently pass once each, serially, under unchanged30000/55000/60000ms limits and identical113/861source:10.429and9.341seconds externally. Each executes one selected case with177filtered and the same171-file/4,686,175-byte corpus. `manual04-timeout-diagnose01/results.json` and owned-ancestry observations establish these bounded reproductions; neither reaches the diagnostic stall threshold. The original cause remains unreproduced, not proven transient. No source, cache, timeout, selector, inverse or fixture-membership change follows. Static review identifies repeated finite provenance validation but does not attribute the timeout to it. Manual04 remains failed.

Manual05 then executes the unmodified complete default `bash scripts/verify.sh` on that same113/861basis, with pinned Bun1.3.9 and the isolated Cargo target. `manual05/native-outcome.json` records exit0, no signal, start2026-10-21T08:54:10.022Z and finish10:55:33.413Z, elapsed7283.397seconds. The verifier itself prints completion in7282565ms. Its raw log SHA is `76fabae24b50d48cb7a32cac53b255e3447b87239862f29cc491ea2283ec6db7`; outcome SHA is `02053c4f73dbedfb87ad5ffd0f86dc2f3bbb5c514bb4126e384a177edb3272e2`. Format and LOC preparation both exit0; the post-format audit and independent final raw recomputation preserve every reviewed byte, path and aggregate digest. There are no new source deltas to review.

Independent raw attribution confirms78Bun file-result footers with zero fail labels and zero nonzero-failure footers. This includes the full178-case native provenance suite,1190canonical-identity cases,2020shared-work cases,490canonical-bootstrap cases and the repaired legacy guards. Workspace/all-features Cargo has29harness headers and13doctest package headers with43result footers; chainstate rustdoc has two batches. Totals are4957passed,0failed,6ignored and0filtered. The additional CLI-client target passes7/7. These are actual complete-command results on the final source; overlapping focused or earlier component results are not added to these counts.

The same native run completes strict Clippy, all-target/all-feature build, benchmark list/smoke/report validation, Bazel build and Bazel provenance, then fresh coverage. The actual coverage invocation selects the exact eight pure-core packages and emits16test-result footers totaling1832passed,0failed,0ignored and0filtered. `manual05/coverage-observation.json` records captured=true and identifies the actual report producer/file; the retained `coverage-report.txt` independently matches2,438,922bytes and SHA `589fbac073c775c7cbd285c964c74a6c27e022006fda9b675408ef0fdbf9db41`. All eight requested package paths occur and there is no `Uncovered Lines:` heading. This proves the native line-coverage contract, without extending it to branch coverage or another platform.

The final independent tracked-plus-untracked inventory equals the113frontmatter paths, with74Rust files. All raw hashes match `pure-genesis-coverage01/candidate-source-basis113.json` and manual05's post-format audit, yielding unchanged `861a298d938014739c0f311a292e270bf268c10e6e635d7ad4520195ef516535`. Existing SOURCE/EDGE values, fixed312membership and historical inverse qualification remain the previously reviewed basis. The successful full native result closes the pending native gate from the08:01source-review cutoff; it does not change any earlier failed receipt or establish a normal-hook, commit or push result.

## Completion Boundary and Residual Limits

**Code-review PASS; zero actionable findings under the accepted tooling exception. Official manual05 full default native verification passes on the exact reviewed113-file861basis.** Earlier failed native/component/ignored-audit attempts retain their actual failed outcomes. The final source includes the bounded repairs, exact reviewed DATA renewals, documented fixture panic exception and justified pure-genesis simplification. Actual passing owner/target, fresh coverage and qualified copied-source evidence is bound above to its source basis and scope. Canonical security/goal acceptance remains separately recorded in [166-VERIFICATION.md](166-VERIFICATION.md) and [166-14-SUMMARY.md](166-14-SUMMARY.md); this source review does not replace those artifacts. Any later source fix requires affected review and verification refresh against the new source basis.

Evidence is finite source-derived genesis/BASIC, exact arithmetic, software storage/fault/process controls and local macOS product behavior. It provides no arbitrary-writer/hostile-lock-replacement protection, parent-crash cleanup guarantee, hardware power-loss qualification, whole-flush atomicity, ordinary descendant/Signet-history qualification, actual pinned Knots-daemon differential, public activation or production/funds-readiness claim. The explicit public fixture convenience API retains its documented panic-on-error risk, accepted only for tooling compatibility; it grants no ordinary canonical authority. The documented Unix support boundary is distinct from the actual macOS receipts. Normal-hook commit/push outcomes remain root-owned and unclaimed here. Work stops before Phase167.

______________________________________________________________________

_Reviewed: 2026-10-21T10:57:05Z_
_Reviewer: gsd-code-reviewer_
_Depth: standard_

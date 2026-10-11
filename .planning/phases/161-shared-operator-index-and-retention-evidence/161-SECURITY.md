---
phase: 161
slug: shared-operator-index-and-retention-evidence
status: verified
threats_total: 35
threats_closed: 35
threats_open: 0
asvs_level: 1
asvs_version: "5.0"
security_enforcement: enabled
approval: verified
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T10:56:33Z"
verified_at: "2026-10-11T10:47:21Z"
closing_summary_inspected_at: "2026-10-11T10:56:33Z"
native_verification: passed
native_attempts: 5
diff_base: a7963f943de6a779b6b1dcafd0f42bdd6ded9066
scope_paths: 113
---

# Phase 161 — Declared Security Mitigation Verification

All **35 declared threats** across Plans 01–11 have their required mitigation controls present in the frozen implementation and supporting regression evidence. All dispositions are `mitigate`; none were accepted or transferred. No required implementation gap remains in this audit scope after WR-01 and WR-02 remediation.

**Security approval is verified after the complete default native run passed and the current 113-path independent source review was clean.** The failed and aborted attempts are preserved below. Formal phase closure and actual hook-protected commit/push remain subsequent root-owned gates; this report makes no production certification claim.

## Scope and Method

This is an independent verification of the declared Phase 161 threat registers at ASVS 5.0 L1 / STRIDE scope, not new vulnerability hunting, an exhaustive ASVS assessment, certification, production-readiness approval or funds-safety evidence. Each declared threat was classified by its recorded disposition before verification. Mitigations were traced at their actual typed input, authority, storage publication, immutable read, recovery, RPC, renderer/export and claim-checker boundaries; tests were inspected for the declared behavior rather than accepted solely from names.

The baseline and changed-path scope were supplied by the root: `a7963f943de6a779b6b1dcafd0f42bdd6ded9066` and the refreshed `/tmp/phase161-review-scope.json` inventory of 113 paths. The appendix records that exact inventory. The initial 99 paths gained ten Plan 09 paths, derived LOC, one existing black-box test and two existing legacy guard files during authorized follow-ups. It includes production code, tests, fixtures, historical/new guards, documentation, parity manifest and generated LOC. Relevant unchanged HTTP authentication, hook and verification dependencies were also read to establish the declared boundaries. Inventory membership is not a claim of unrestricted whole-codebase review.

Current 113-path scope-manifest SHA256: `f67402aa0ee09b904c911e0e2434218f07a8c4a5fe184513a6fa1733179c112e`. Current native-attempt-5 path/content digest: `31828e43196124e3b6d5e23261006f1ef073b0387ba9e6d6ea8c942182fd260f`, computed by ordered UTF-8 path, NUL and raw SHA256(file bytes) concatenation. Compared with attempt 4, only the two legacy guard files and derived LOC changed. The 111-path attempt-4 manifest/content digests were `706ddd85d2b0f55d88a582b58e094938f40a98bdef18f595c2b49b10ef4e5c8d` / `e66c147c7527ec1daa1f3aea5169d0967173b705f260b296e093994477b3acd1`; the immediate post-black-box-correction content digest was `48f366b5d765accc6f3402888254bc26e08f5af3e90eb0d8a0d3b6a2667d6a56`. This is the pre-activation, pre-stage evidence snapshot, not an eventual commit digest. Native attempt 3 used 110-path manifest `70a77cfcbafaeaaa3b36387141718314c83d93236f58e9f1f8d8950aaa31be7e` and content digest `8fdbedd493a38fa8cd3598a9fcdf4cf97aaa79f988cc18d82713f8f336d275c9`. Native attempt 2 used content digest `751c85ba5ba9ea1d6318ac5e951a60cd0c376cef1b9f490beba2314024f5112b`; the immediately post-rename digest was `8b0ad6bf779a0e6d7f2e5e894f91d769622fdcdcdaf1596b37396699eed7f452`. The initial pre-rename manifest/content digests were `76b507134b26ff8209abff775e64882c76901e00ecc442b851b328e6a5170687` / `867720f59c82ed5f30d207b9f20dfb417237501798bc5ed8441310220a226c65`; they identify the first failed native attempt's source scope. Planning reports are outside that implementation inventory.

Two expected later changes are completion/derived metadata: root changes only the CFOP-01 parity row from `in_progress` to `done` after formal approval while preserving all known-gap and exclusion limits, and the hook regenerates `docs/metrics/lines-of-code.md` after new files enter the index. Runtime behavior remains frozen. The parity-row update may receive a pre-commit metadata-only reinspection; the mandatory hook reruns full native verification against the final index/LOC source. Neither an eventual commit digest nor actual hook success is asserted in this pre-commit report.

The pre-completion parity inventory has one CFOP-01 owner at `in_progress` with three known-gap entries. Its semantic SHA256 is `46d77f8fd3e60f5a95525f928003d238df24f9d5ae7a04018ca8eb69e9d2f0f7`, using Python JSON serialization with sorted keys, compact separators and default ASCII escaping. Restoring that one status before a later semantic comparison can verify a status-only completion change independently of formatting.

The original approval inputs include all 11 PLAN `<threat-model>` registers, the ten then-completed plan summaries, context, CALLER-EVIDENCE, CORE-REVIEW, GUARD-COMPATIBILITY, Plan 09 SOURCE-HANDOFF, final independent REVIEW and root UAT. The final Plan 09 SUMMARY was subsequently read in the closing metadata inspection below, completing all eleven summary inputs. The final source freeze includes the healthy-start disable correction. Applicable privacy, authentication/access-control, input/trust validation, safe error projection and finite resource controls were checked against those declarations; no undeclared ASVS coverage is asserted.

Material guidance: repository `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, standards index, architecture, verification, testing, local-guidance and Rust/TypeScript rules, security-auditor instructions and the gsd-secure-phase workflow. Both canonical active lesson inputs were read completely: 5,230 + 1,958 = 7,188 bytes; conservative estimate 1,744 + 653 = 2,397 tokens. No lesson archive was loaded or tracker/source modified.

Path shorthand in the register: `status/`, `filters/`, `network/` and `sync/` refer respectively to node `src/status/`, `src/storage/fjall_store/filters/`, `src/network/` and `src/sync/`; `cli/` means `packages/open-bitcoin-cli/src/operator/`; `rpc/` means `packages/open-bitcoin-rpc/src/`. Peer implementation references are in `packages/open-bitcoin-network/src/`. Full test and documentation paths are supplied where the shorthand would be ambiguous.

## Trust Boundaries

| Boundary                                        | Verified data/control crossing                                                                                                                                    |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Runtime/storage authority → BASIC observation   | Fixed safe enums, booleans, optional heights, counters and validated protection bounds; raw identifiers/errors stay internal.                                     |
| Recovery/publication → retained accounting      | Complete mandatory verification or proven empty initialization earns totals; checked unique-row deltas install only after achieved publication.                   |
| Immutable read/client refusal → sticky fault    | Genuine source-local backend/codec/required-parent observations revoke trust; ordinary absence, client/policy/resource/stale refusals do not manufacture a fault. |
| Authority/epoch → status and prepared transport | Coherent captured generation/branch/fence facts and revoked read epochs; old prepared work cannot earn current completion.                                        |
| Captured snapshot → RPC/CLI/dashboard/support   | Existing authentication, unavailable legacy/stopped defaults, one captured source and exact reason allowlist; sanitization preserves the caller.                  |
| Evidence/docs → lifecycle claim                 | Narrow executable mutation guards supplement genuine callers; root full native/review/security/formal gates precede hook-protected commit.                        |

STRIDE category keys: S=Spoofing, T=Tampering, I=Information Disclosure, D=Denial of Service, E=Elevation of Privilege; E/T preserves the plan's combined classification. No Repudiation entry was declared. Stable plan-qualified IDs are retained exactly.

## Threat Register

| Threat ID  | STRIDE | Component                                          | Disposition | Declared mitigation                                                                                                                               | Status | Verified implementation and regression evidence                                                                                                                                                                                                                                                                                                                             |
| ---------- | ------ | -------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-161-01-1 | I      | Shared serialized BASIC contract                   | mitigate    | No identifiers, raw backend errors, paths or lock names in new types; fixed guidance/reasons.                                                     | CLOSED | `status/basic_index.rs:26,61,147,235,255`: exact reason normalization, bounded scalar/enum schema and serde adapters; hostile serialize/deserialize and guidance tests at lines462/491/611.                                                                                                                                                                                 |
| T-161-01-2 | T      | Legacy/default and endpoint semantics              | mitigate    | Serde defaults are unavailable; empty prefixes are Option absence, not invented zero.                                                             | CLOSED | `status/basic_index.rs:94,116,327,355` and `status/tests/snapshot_projection.rs`: all unavailable defaults; available null differs from genesis/zero.                                                                                                                                                                                                                       |
| T-161-02-1 | T      | Recovery to trusted measurement                    | mitigate    | Publish aggregates only after complete immutable/projection verification; clear on failed recovery.                                               | CLOSED | `filters.rs:223`, `filters/startup.rs:69`, `filters/lifecycle.rs:50`, `filters/ownership.rs:319`: complete forest/projection validation before installation, prevalidation revocation and failure cleanup. Healthy-start WR-02 tests: `tests/retention/recovery.rs:9` and `serving/tests.rs:20`.                                                                            |
| T-161-02-2 | D      | Storage accounting                                 | mitigate    | Reuse one existing complete pass, checked counters and bounded per-record buffers; status inspection is O(1).                                     | CLOSED | `filters.rs:223`, `filters/retention.rs:23,79,146`: existing complete scan accumulates checked bounded scalars; optional budget exhaustion preserves mandatory checks; poll reads only cached totals/two metadata values. Startup scan-count and exact budget tests in `tests/retention/recovery.rs`.                                                                       |
| T-161-03-1 | T      | Publication success to accounting                  | mitigate    | Compute checked deltas pre-effect; install only after achieved commit, deduplicate from prepared.records.                                         | CLOSED | `filters/append.rs:328,346`, `filters/publication.rs:314,320,384,420,458`: checked prospective deltas from prepared unique missing rows, installed after successful batch; duplicate/conflict/retry tests in `tests/retention/publication.rs:61,87,120,145`.                                                                                                                |
| T-161-03-2 | T      | Raw writers, clones and stale epochs               | mitigate    | Clear on raw integrity invalidation; poison and stale generation invalidate observation; reopen is exclusive.                                     | CLOSED | `filters/publication/epoch.rs:57,73,81`, `filters/publication.rs:559`: raw integrity revocation, checked epochs/revision and poison; clone/stale/raw/reopen tests in `tests/retention/publication.rs:335,381,413,455` and `query/tests.rs`.                                                                                                                                 |
| T-161-03-3 | D      | Counters and copies                                | mitigate    | Checked arithmetic before effects and reuse encoded buffers; no new scans or uncharged payload copies.                                            | CLOSED | `filters/retention.rs:43,79,128,377`, `filters/publication.rs:458`: checked borrowed lengths and finite admission; existing admitted comparison deduplicates without extra scan or payload copy; overflow/no-effects tests at `tests/retention/publication.rs:284`.                                                                                                         |
| T-161-04-1 | T      | Authority/publication capture                      | mitigate    | One existing lock order, generation and integrity lifetime; status cannot mutate or clear failure.                                                | CLOSED | `network/operator_snapshot.rs:121,216`, `filters/retention.rs:146`: one existing authority/publication order and full branch/generation/fence/endpoint checks; genuine namespace/owner/counter equality in `sync/tests/filter_index/operator_status.rs:93,492,544`.                                                                                                         |
| T-161-04-2 | I      | Peer scope and protection                          | mitigate    | Fixed aggregate counts and bounds only; no identifiers/raw grants/lock names.                                                                     | CLOSED | `status/basic_index.rs:123,184,193`, `network/operator_snapshot.rs:109,224`: five aggregate counters and validated scalar protection bounds only; high-boundary test `operator_snapshot/tests.rs:106`.                                                                                                                                                                      |
| T-161-04-3 | D      | Status peer aggregation                            | mitigate    | Maintain fixed checked lifecycle counters; status copies in O(1), at most8 metadata reads/4KiB and zero peer/index/history traversal.             | CLOSED | `filters/retention.rs:146`: \<=8 metadata gets/4096 admitted bytes (actual two), no peer/index/history/lock traversal or filter copies; growing-history and bound controls in `operator_status.rs:93,544`.                                                                                                                                                                  |
| T-161-04-4 | E      | Configured capability versus operational authority | mitigate    | Explicit/all/implicit remain separate; service flags never grant index health or general configuration.                                           | CLOSED | `network/operator_snapshot.rs:121,216`, `network/limited_serve.rs:24,31,37`, `filter_serving/capability.rs:23`: configuration, offers and operational coherence independent; zero-lag unearned-latch and permissioned-general-false controls.                                                                                                                               |
| T-161-05-1 | I      | RPC/collector evidence boundary                    | mitigate    | Typed safe projection only; no Display(StorageError), paths or IDs.                                                                               | CLOSED | `rpc/context/inbound_status.rs:66`, `rpc/dispatch/node.rs:271`, `cli/operator/status.rs:238`: typed captured group forwarded; shared serde reason normalization and fixed failure collection fallback; `status/tests/basic_index.rs:220`.                                                                                                                                   |
| T-161-05-2 | T      | Legacy/stopped contract                            | mitigate    | Serde omission and failed live calls yield unavailable rather than healthy defaults.                                                              | CLOSED | `rpc/method/node.rs:127`, `cli/operator/status.rs:332,372`: serde-defaulted omission and stopped/failed collection unavailable; legacy/malformed/stopped tests in `status/tests/basic_index.rs:148,190,239`.                                                                                                                                                                |
| T-161-05-3 | E      | RPC authentication                                 | mitigate    | Reuse existing authenticated route and pinned method shapes; no new auth bypass or endpoint.                                                      | CLOSED | Supporting existing `rpc/http.rs:143` authorizes before parsing/dispatch; `dispatch/tests/network_status_schema.rs:63` and daemon `operator_evidence.rs:55` exercise denied/allowed existing route; pinned filter/index shapes retained.                                                                                                                                    |
| T-161-06-1 | I      | Live/support free-text reasons                     | mitigate    | Exact bounded allowlist plus fixed typed guidance; malicious JSON/Markdown/ANSI/privacy markers are absent.                                       | CLOSED | `status/basic_index.rs:26`, `cli/status/render/basic_index.rs:51`, `cli/support/redaction/basic_index.rs:8`: one exact reason allowlist; fourteen-boundary hostile marker matrix `support/tests/basic_index.rs:176`.                                                                                                                                                        |
| T-161-06-2 | T      | Captured support facts                             | mitigate    | Clone first, preserve safe scalar values, render one sanitized snapshot, assert source equality.                                                  | CLOSED | `cli/support/redaction.rs:74`, `cli/support/render/basic_index.rs:7`, `cli/status/render/basic_index.rs:13`: sanitize captured owned copy and share rows; source equality, typed roundtrip and actual export-file checks in `support/tests/basic_index.rs`.                                                                                                                 |
| T-161-06-3 | E      | Operator guidance                                  | mitigate    | No repair/download/protection-release/activation controls or instructions are introduced.                                                         | CLOSED | `status/basic_index.rs:235` and `cli/status/render/basic_index.rs:98`: fixed guidance only, separate observation fields and explicit no-auto-repair/history wording; no effect-capable collector in renderers.                                                                                                                                                              |
| T-161-07-1 | I      | Terminal text and status reasons                   | mitigate    | Render fixed scalar/enum values and allowlisted unavailable text; no escape-sequence-bearing private strings.                                     | CLOSED | `cli/status/render/basic_index.rs:51`, `cli/dashboard/app.rs:316`: normalized pure row strings; actual hostile terminal-cell test `dashboard/app/tests/basic_index.rs:309`.                                                                                                                                                                                                 |
| T-161-07-2 | T      | Narrow rendering evidence                          | mitigate    | Assert labels and adjacent distinguishing values in actual buffer; no healthy clipped summary.                                                    | CLOSED | `cli/dashboard/app.rs:263`: checks complete critical values, safe narrow fallback and full unavailable word; actual adjacent buffer values/failure categories and oversized-number fallback tests at `app/tests/basic_index.rs:133,162,334`.                                                                                                                                |
| T-161-07-3 | D      | Dashboard refresh                                  | mitigate    | Reuse one captured snapshot and existing cadence; no render-time RPC/storage scan.                                                                | CLOSED | `cli/dashboard/model/basic_index_section.rs:10` and `cli/dashboard/app.rs:263`: supplied captured rows only; existing cadence and refresh remain, no render-time RPC or store borrow.                                                                                                                                                                                       |
| T-161-08-1 | S      | Authenticated production caller tests              | mitigate    | Use real existing auth router/client; unauthorized request cannot disclose BASIC group.                                                           | CLOSED | `rpc/http.rs:143`, daemon `operator_evidence.rs:55`: production HTTP unauthorized401 with empty body; authenticated loopback harness gate `cli/tests/operator_flows/rpc_harness.rs:188` before replay transformation.                                                                                                                                                       |
| T-161-08-2 | I      | Exported facts from genuine failures               | mitigate    | Safe fixed categories only; JSON/human/support compare private markers and unchanged captured input.                                              | CLOSED | `sync/tests/filter_index/operator_status/captures.rs:56`, `catch_up/rpc_faults/captures.rs:9`, `cli/tests/operator_flows/basic_index/captured.rs:24,149`: genuine producer freshness, exact safe catalog/roundtrip/bounds, hostile actual exports and unchanged source.                                                                                                     |
| T-161-08-3 | T      | Durable observation and pruning                    | mitigate    | Real staged spend/Fjall/reopen proof; retained filters do not authorize restored body/wallet availability.                                        | CLOSED | Daemon `operator_evidence.rs:155` and `cli/tests/operator_flows/basic_index.rs:437`: genuine accepted history, protected filtering, paired prune, stale rows/all-handle reopen; independent body-serving and unchanged-wallet rescan negatives, with manual-owner-plan limits stated in CALLER-EVIDENCE.                                                                    |
| T-161-08-4 | D      | Repeated cross-surface collection                  | mitigate    | Structural counters prove bounded BASIC capture and no payload/history traversal.                                                                 | CLOSED | `sync/tests/filter_index/operator_status.rs:93` and `cli/tests/operator_flows/basic_index/captured.rs:104`: growing-history namespace/counter equality, two metadata gets, bounded JSON and exact six/five collection counts; no payload/history traversal.                                                                                                                 |
| T-161-09-1 | T      | Claim/evidence integrity                           | mitigate    | Mutation controls pin real production edges and reject unsupported claims; formal report names actual tests.                                      | CLOSED | `scripts/check-phase161-operator-index-evidence.ts:15,77,118,137` and `.test.ts:15`: 28 actual function seams, six assertive callers, module ownership, exact producer comparison, claim limits and native order; 34 positive/mutation tests recorded in `161-09-SOURCE-HANDOFF.md`. Lexical evidence is explicitly not runtime proof.                                      |
| T-161-09-2 | I      | UAT/support documentation                          | mitigate    | Disposable fixture paths and fixed safe categories; no private evidence or automatic repair.                                                      | CLOSED | `docs/operator/runtime-guide.md:3,42`, `docs/architecture/status-snapshot.md:9`, parity catalog line794 and `161-UAT.md`: disposable stopped inspection, fixed safe categories, captured-copy privacy, no automatic repair and explicit fixture/evidence exclusions; six docs retain Phase 162/v2.5/crash-window limits.                                                    |
| T-161-09-3 | E      | Lifecycle and commit boundary                      | mitigate    | Independent ASVS L1/STRIDE findings block finalization; full native/hooks mandatory and root alone owns final commit/push.                        | CLOSED | `scripts/verify.sh:150,340,384` and supporting `.githooks/pre-commit:10`: fail-fast full native test/checker/build/test/Bazel/coverage wiring and mandatory hook; Plan 09 Task3/root gate defers commit/push until independent source/security/formal/native approval. Actual native execution remains pending below; hook execution occurs later during root finalization. |
| T-161-10-1 | E      | Resolved configuration versus offered service      | mitigate    | Capture actual startup/setter intent separately; preserve explicit/all/implicit and prerequisite semantics.                                       | CLOSED | `sync/open_runtime.rs:76`, `network/limited_serve.rs:31,37`, `filter_serving/capability.rs:23`: actual startup/setter intent and explicit/all provenance separate from offered service; prerequisite/grant semantics retained.                                                                                                                                              |
| T-161-10-2 | T      | Session lifecycle counters                         | mitigate    | Checked old/new updates on add/replace/version/remove; underflow/overflow is unavailable.                                                         | CLOSED | `packages/open-bitcoin-network/src/peer/basic_operator.rs:53`, `peer.rs:514`, `peer/inbound_state.rs:71`, `peer/message_dispatch.rs:179`, `peer/inventory_state.rs:346`: checked old/new lifecycle transitions; all-five underflow/overflow, duplicate/remove/reconnect/version failure controls.                                                                           |
| T-161-10-3 | D      | Status capability capture                          | mitigate    | Constant-size lifecycle aggregates and no per-poll peer table/iteration.                                                                          | CLOSED | `packages/open-bitcoin-network/src/peer/basic_operator.rs:82`, `network/operator_snapshot.rs:224`: constant-size counter copy; intentional table-clearing getter/clone test demonstrates no poll traversal.                                                                                                                                                                 |
| T-161-11-1 | T      | Query fault to status                              | mitigate    | Source-local typed fault, fixed sticky merge, revoked epoch/append proof, operator precedence even over owner pause; genuine before/after tests.  | CLOSED | `filters/query.rs:223,300`, `query/peer.rs:45,153`, `publication/epoch.rs:17`, `operator_snapshot.rs:77,275`: source-local observation, sticky strongest fault and one revocation, overriding owner pause; real immediate/final/peer tests.                                                                                                                                 |
| T-161-11-2 | D      | Client-triggered poisoning                         | mitigate    | No blanket Corruption/string classification; absence/stale/readiness/resource/byte/wire controls supply no observation; executed negative matrix. | CLOSED | `filters/query.rs:300`, `query/peer.rs:123,153,299`: admission/policy/absence/stale outcomes outside fault observers; `query/peer/tests.rs:9` and `rpc_faults.rs` execute budget, Corruption-policy, oversize, cancelled/readiness and absence controls.                                                                                                                    |
| T-161-11-3 | I      | Failure projection                                 | mitigate    | Private enum only, existing fixed public categories/reasons, malicious backend marker excluded from BASIC JSON.                                   | CLOSED | `publication/epoch.rs:10`, `operator_snapshot.rs:77`, `status/basic_index.rs:147`: private fixed fault to existing fixed public categories; `rpc_faults.rs:33` excludes private backend marker/identifiers from bounded BASIC JSON.                                                                                                                                         |
| T-161-11-4 | E/T    | Prepared authority and recovery                    | mitigate    | Old epoch/lease cannot earn receipt; complete verified reset only; partial/failed recovery and metadata never heal.                               | CLOSED | `publication/epoch.rs:46`, `startup.rs:69`, `lifecycle.rs:50`, `ownership.rs:528`: successful complete reset only, prevalidation revocation and blocked work; healthy-start WR-02 tests plus `serving/tests.rs:71` invalidate old prepared receipt/suffix without invented success.                                                                                         |
| T-161-11-5 | D      | Status/query work and locks                        | mitigate    | Existing publication guard plus one scalar transient observation, no recursive lock/scans/copies, checked epoch, unchanged read/output budgets.   | CLOSED | `filters/query.rs:223`, `query/peer.rs:45`, `retention.rs:146`, `publication/epoch.rs:17`: existing guard + transient scalar Cell, checked one-time revocation, no recursive acquisition/scans/copies; repeated genuine read-only poll/counter tests.                                                                                                                       |

## Remediation Reverification

**WR-01 — discovered immutable read fault followed by healthy status:** the current source-local `Cell<Option<BasicFilterReadFault>>` settles under the same existing publication guard before returning. Genuine backend/codec/required-parent faults preserve their original caller errors, latch only a fixed category, revoke complete read/append/retention trust and advance the checked epoch once. Integrity dominates Backend; either takes precedence over missing-history/resource owner pauses. Rejected rereads do not repeatedly revoke or upgrade a previously observed Backend. Actual immediate RPC, final-barrier read and managed-peer preparation tests inspect subsequent sticky status, safe-fact preservation, private-marker exclusion, old-epoch/receipt rejection and unchanged namespace/owner/scan state over repeated polls.

**WR-02 — healthy-start failed disable retained trusted growth/read authority:** independently confirmed the correction at `filters/lifecycle.rs:50`: `invalidate_read_integrity` now runs before append invalidation and every fallible owner/forest/checkpoint operation, with the same single direct epoch advance. The eight-case direct/configured regression at `filters/tests/retention/recovery.rs:9` starts healthy, has complete growth and no discovered query fault, then injects forest, checkpoint, owner or BeforeDisable failure. It checks revoked totals/read integrity, denied old/new epochs, expected scan counts and no fabricated query-fault category. The genuine managed accepted/Fjall regression at `network/runtime_authority/filter_index/serving/tests.rs:20` verifies the actual operator projection. Existing successful complete validation/proven-empty boundaries alone restore trust; failed/partial work and metadata do not heal it.

The Plan 11 correction appendix records meaningful RED 1 fail → GREEN 2 pass plus 74 related regressions and strict lint. Independent timing JSON inspection confirms RED exit101, GREEN success/exit0 and strict node Clippy success/exit0. Exact test counts are attributed to executor results and source-visible assertions, not inferred from timing metadata. Root's 155/156 guard refresh additionally rejects substitution of the obsolete epoch-only disable call; GUARD-COMPATIBILITY records273 tests/544 assertions and both live guards passing. Those are root-run source/mutation checks, not extra runtime cases.

**Native attempt 2 startup diagnostic correction:** independently inspected `filters/startup.rs:132`, supporting `filters/ownership.rs:403` and `packages/open-bitcoin-chainstate/src/filter_index/lifecycle.rs:124`. Present saved state requires complete forest/projection measurement before owner/protection inspection. Absent state supplies only a private candidate zero: the subsequent unconditional owner/artifact loader refuses any lifecycle, checkpoint/protection, reserved lock, immutable row or projection artifact. Only its `Ok(None)` complete-absence proof permits installation, under the same already-invalidated publication guard. No orphan-only partial state earns empty trust, and no new recovery scan is added. Existing-state one-scan and reconciliation two-scan controls remain; genuine empty recovery now asserts zero scans at `filters/tests/retention.rs:94`.

The unchanged original `filter_index_production_reopen_refuses_each_lone_partial_artifact` covers record-only, projection-only and reserved-lock-only refusals with the original BlockIndex/Repair diagnostic, unchanged namespace snapshot and retained body/undo. Plan 02 records its reproduced RED then GREEN, 45 passing startup tests, 22 passing recovery controls and strict node Clippy. This auditor independently inspected their timing receipts: failure/101, then success/0 for the original test, both regression selectors and strict lint. The five refreshed Phase 155/156 script paths pin the exact state predicate, mandatory propagated measurement for present state and unconditional owner proof before installation. Ten new mutations reject false/inverted predicate, missing proof, early totals and early success. GUARD-COMPATIBILITY records the final 283 tests/564 assertions, historical/current live guards and managed checks passing. These counts overlap prior suites and are not additive distinct runtime tests.

**Native attempt 3 black-box correction:** independently inspected `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs:379`. All seven old network-status keys remain, `basic_index` is added in sorted position, and the previous block-relay equality remains intact. One existing authoritative snapshot now supplies both block-relay and BASIC JSON; the new assertion compares the response's BASIC value with that captured authority. No production code, endpoint/authentication behavior, collector or ignored case changed. This is additional production-composition/schema evidence for T-161-05-3, not a new genuine BASIC indexing proof.

Plan 05 records the exact corrected black-box target passing 3/3, remaining workspace preflight excluding the already-passed unchanged node crate passing 2,596 (2,577 ordinary plus nineteen doctests), the explicit seven-test CLI client binary passing, and strict RPC lint/format/parity passing. This auditor independently confirmed success/exit 0 timing receipts for the black-box target, remaining workspace, alias binary and strict lint; exact discovery counts remain executor-attributed. These are preflight receipts, not a substitute for the mandatory complete default run. The added existing test is the only path/content change from the attempt-3 source snapshot at this correction boundary. The independent REVIEW refreshed at 2026-10-11T10:04:34Z covers all 111 paths with zero open findings and verifies the preserved old keys/authority plus stronger BASIC comparison. This auditor read that refresh. Root's benchmark/Bazel/pure-core coverage tail preflight is active separately, followed by mandatory full native attempt 4. Final approval remains pending that actual full success.

**Native attempt 4 legacy proof-guard compatibility:** independently inspected the released exact status-key anchor at `scripts/check-phase127-authoritative-network-state-unification.ts:356`: it requires `basic_index` followed by all seven original keys. The existing authority composition, durable serving, projection, restart and privacy controls remain present. The new isolated test `fails_when_frozen_status_keys_omit_basic_index` removes only BASIC from a temporary source corpus, first asserts the live source is clean, and then requires the exact production-proof failure. Fixture replacement refuses a missing needle, preventing a no-op mutation. GUARD-COMPATIBILITY records 18 passing tests/19 assertions, the live guard and managed/whitespace checks passing. This is a two-existing-script compatibility update, with no production Rust, API, authentication or dependency change and no new threat surface.

## Evidence Attribution and Limits

This auditor performed read-only source/artifact/fixture/timing inspection and wrote only this report. No Cargo, Bazel, test job, source edit, tracker edit, staging, commit or Git operation was performed by this auditor. The recorded executed runs belong to their named executors/root. Independently inspected successful timing receipts include final read-fault, strict lint, generation-disabled capture freshness, actual CLI callers and daemon callers, plus the WR-02 correction receipts.

- Plan 08/CALLER-EVIDENCE records 111 distinct selected tests:88 node +11 CLI integration +3 new daemon +9 historical daemon. Generation, nine cases inside replay tests and subset reruns are not extra discovered tests. Plan 07's twelve real terminal-buffer cases are inherited presentation evidence.
- Plan 11's correction brings its documented selected set to 122 distinct cases, not an additive whole-phase count. The earlier startup double-scan claim is superseded by Plan02's correction and cumulative production-route scan controls.
- Plan 09 handoff records 34 positive/mutation tests, 64 assertions, 28 actual function seams and six assertive caller selectors, plus historical 154–160 live guard success. This source checker is lexical/function-aware evidence, not a Rust parser or independent runtime proof.
- Genuine public daemon/operator paths and separately fresh-checked private producer captures composed with authenticated replay are distinguished. Private faults were not injected through a live daemon socket. Managed-peer stale receipt tests use isolated acknowledgement/state-machine evidence, not live socket-write proof.
- Two metadata reads/91 admitted bytes and \<=4KiB serialization/no-scan evidence apply to the BASIC group. Whole-capture timing envelopes are not direct lock-hold measurements, latency guarantees or archive-scale bounds.
- The 402 immutable rows / 4,801 payload bytes / 108,115 logical key-value bytes include displaced/body-pruned rows and exclude projections, metadata and storage-engine/physical overhead. Payload bytes are a subset.
- Synthetic genesis/easy PoW/maturity-one/manual owner-plan fixtures, separate in-window body positive and out-of-window body/wallet negatives do not establish ordinary prune-RPC planning, automatic prune-after 1000 or same-hash serving causality. Phase 162 integrated proof, v2.5 completion, public defaults, repair/download, production/funds safety and stronger crash-window claims remain excluded.

## Summary Threat Flags

No summary contains an unmapped `## Threat Flags` entry. All eleven final summaries, including the completed Plan 09 SUMMARY inspected below, report no additional production trust boundary or unmapped flag. WR-01 and WR-02 map to existing declared threats and are recorded above as resolved mitigation gaps, not unregistered threats. The final-summary reinspection is metadata maintenance after the original formal report, not a self-referential prerequisite for the earlier security approval. **Unregistered flags: none.**

## Accepted Risks Log

No accepted risks. No required/high gap was auto-accepted, reclassified as a limit or transferred.

## Native and Hook Gate

**Verified pre-commit native receipt:** root's complete default `bash scripts/verify.sh` attempt 5 exited **0**, ending at **2026-10-11T10:43:52.732Z**. The log footer records **27m41.348s (1,661,348 ms)**. The independent `verify-full` JSON receipt records `outcome: success`, `exitStatus: 0`, `verifyMode: full`, default target and **1,661,866 ms** wrapper duration.

| Attempt | Actual outcome                       | Elapsed ms    | Evidence / disposition                                                                                                                                                                                                                                               |
| ------- | ------------------------------------ | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1       | Exit 1                               | 651,964       | Panic-policy classification of a test child; canonical byte-identical rename corrected it. Cargo had not run.                                                                                                                                                        |
| 2       | Exit 101                             | 1,440,476     | Node 1,631 pass / one fail / three existing ignores; original startup diagnostic precedence restored without weakening its test.                                                                                                                                     |
| 3       | Exit 101                             | 1,634,308     | Node 1,632, RPC library 348 and daemon 87 passed; legacy black-box expected seven keys instead of the intended eight. Exact keys and same-snapshot BASIC assertion corrected.                                                                                        |
| 4       | Controlled abort, exit 143           | 283,445       | Root terminated only verified live Bun child PID 52695 after a separate known legacy-127 preflight failure, before that native step. Not a timeout, stall, Rust failure or direct native-127 result. Exact eight-key guard and isolated omission-negative corrected. |
| 5       | **Complete default success, exit 0** | **1,661,348** | All required source/claim/mutation/architecture/parity/managed/panic, format/strict Clippy/all-target build, workspace/CLI tests, benchmark, six-target Bazel/provenance and pure-core zero-uncovered coverage stages passed.                                        |

Primary proof artifacts: `161-NATIVE-EVIDENCE.md`, `/tmp/phase161-native-full-5.log` and `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-11T10-16-10.866Z-21971786-6462-4a73-9cab-7adc41627b1e.json`. This auditor independently read the final receipt, log footer, current 113-path clean REVIEW and Plan 09 handoff. Hash comparison after completion confirms **all 113 source-scope bytes and paths unchanged** from the attempt-5 snapshot recorded above.

Before coverage reruns, root's final receipt records **4,247 primary Rust passing executions, zero failures, three existing opt-in ignores across 41 result groups**, plus **4,181 Bun passing records**. Coverage reruns are excluded from that Rust total. These are verifier execution counts, not a deduplicated whole-project test count or extra ASVS coverage. Separate successful remaining-workspace, explicit CLI and benchmark/Bazel/coverage preflights are supplemental; none was substituted for the complete default success. All three failed attempts and the controlled fourth abort remain preserved.

`scripts/verify.sh` contains ordered Phase 161 test/live guard execution in both its command-order contract and actual `run_step` list, plus every full-mode stage above. Supporting `.githooks/pre-commit` uses `set -euo pipefail`, generates/stages LOC and invokes the full verifier with the indexed LOC source. This is verified mandatory wiring. **Actual hook, commit and push execution occurs later under root ownership and is not preclaimed here.**

The current independent `161-REVIEW.md`, refreshed at **2026-10-11T10:17:57Z**, covers all **113 paths** against the supplied baseline with zero open critical/warning/info findings. It independently rechecks WR-01/WR-02, the corrected terminal selector, canonical test layout, absent-state proof, exact black-box schema and legacy guard additions. Its independently executed guard/parity/whitespace checks and correctly attributed runtime receipts were inspected. No required implementation or security finding remains.

**Security approval verified at 2026-10-11T10:47:21Z: 35/35 mitigations CLOSED, threats_open: 0.** This approves the declared Phase 161 mitigation scope after the current-source native and source-review gates. Formal goal/lifecycle closure follows under root ownership; the final Plan 09 SUMMARY is written after that formal report and can receive metadata-only threat-flag reinspection afterward. Neither later summary creation nor impossible pre-commit hook self-reference blocks this approval.

## Security Audit Trail

| Audit time (UTC)     | Total | Closed controls | Open implementation threats | Approval                                                                      |
| -------------------- | ----- | --------------- | --------------------------- | ----------------------------------------------------------------------------- |
| 2026-10-11T08:37:39Z | 35    | 35              | 0                           | Pending successful frozen-source native receipt and clean final source review |

Final approval receipt: **2026-10-11T10:47:21Z** — 35 total, 35 closed, zero open; full native attempt 5 and current 113-path source review verified. Earlier pending statements in remediation sections describe their historical evidence windows.

## Sign-Off

- [x] All 35 threats classified and verified by their declared disposition.
- [x] No accepted/transferred risks or unregistered summary flags.
- [x] WR-01 and healthy-start WR-02 correction independently reinspected.
- [x] Implementation files remained read-only.
- [x] Root frozen-source full native success receipt recorded.
- [x] Clean final independent source review refreshed for the 113-path legacy guard correction.
- [x] Final security approval issued.

## Frozen Changed-Path Inventory

```text
README.md
docs/architecture/operator-observability.md
docs/architecture/status-snapshot.md
docs/metrics/lines-of-code.md
docs/operator/runtime-guide.md
docs/parity/catalog/basic-compact-filters.md
docs/parity/index.json
docs/parity/source-breadcrumbs.json
packages/README.md
packages/open-bitcoin-bench/src/cases/operator_runtime.rs
packages/open-bitcoin-cli/src/operator/dashboard/app.rs
packages/open-bitcoin-cli/src/operator/dashboard/app/tests.rs
packages/open-bitcoin-cli/src/operator/dashboard/app/tests/basic_index.rs
packages/open-bitcoin-cli/src/operator/dashboard/model.rs
packages/open-bitcoin-cli/src/operator/dashboard/model/basic_index_section.rs
packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
packages/open-bitcoin-cli/src/operator/dashboard/model/tests/basic_index.rs
packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
packages/open-bitcoin-cli/src/operator/dashboard/model/tests/prune.rs
packages/open-bitcoin-cli/src/operator/soak/tests/runtime/runtime_fixtures.rs
packages/open-bitcoin-cli/src/operator/status.rs
packages/open-bitcoin-cli/src/operator/status/render.rs
packages/open-bitcoin-cli/src/operator/status/render/basic_index.rs
packages/open-bitcoin-cli/src/operator/status/render/tests.rs
packages/open-bitcoin-cli/src/operator/status/render/tests/basic_index.rs
packages/open-bitcoin-cli/src/operator/status/tests.rs
packages/open-bitcoin-cli/src/operator/status/tests/basic_index.rs
packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs
packages/open-bitcoin-cli/src/operator/status/tests/rendering_and_service.rs
packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
packages/open-bitcoin-cli/src/operator/status/tests/status_inputs.rs
packages/open-bitcoin-cli/src/operator/support/redaction.rs
packages/open-bitcoin-cli/src/operator/support/redaction/basic_index.rs
packages/open-bitcoin-cli/src/operator/support/render.rs
packages/open-bitcoin-cli/src/operator/support/render/basic_index.rs
packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
packages/open-bitcoin-cli/src/operator/support/tests.rs
packages/open-bitcoin-cli/src/operator/support/tests/basic_index.rs
packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs
packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
packages/open-bitcoin-cli/tests/operator_flows.rs
packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs
packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs
packages/open-bitcoin-network/src/filter_serving/capability.rs
packages/open-bitcoin-network/src/lib.rs
packages/open-bitcoin-network/src/peer.rs
packages/open-bitcoin-network/src/peer/basic_operator.rs
packages/open-bitcoin-network/src/peer/inbound_state.rs
packages/open-bitcoin-network/src/peer/inventory_state.rs
packages/open-bitcoin-network/src/peer/message_dispatch.rs
packages/open-bitcoin-node/src/network.rs
packages/open-bitcoin-node/src/network/limited_serve.rs
packages/open-bitcoin-node/src/network/operator_snapshot.rs
packages/open-bitcoin-node/src/network/operator_snapshot/tests.rs
packages/open-bitcoin-node/src/network/peer_network_clone.rs
packages/open-bitcoin-node/src/network/relay_serving.rs
packages/open-bitcoin-node/src/network/runtime_authority.rs
packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests.rs
packages/open-bitcoin-node/src/network/tests/filter_capability.rs
packages/open-bitcoin-node/src/network/types.rs
packages/open-bitcoin-node/src/status.rs
packages/open-bitcoin-node/src/status/basic_index.rs
packages/open-bitcoin-node/src/status/tests.rs
packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/query/peer/tests.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/retention.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs
packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/recovery.rs
packages/open-bitcoin-node/src/sync/open_runtime.rs
packages/open-bitcoin-node/src/sync/tests/filter_index.rs
packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults/captures.rs
packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status/captures.rs
packages/open-bitcoin-node/testdata/basic_operator_captures.json
packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs
packages/open-bitcoin-rpc/src/context/inbound_status.rs
packages/open-bitcoin-rpc/src/dispatch/node.rs
packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs
packages/open-bitcoin-rpc/src/method/node.rs
packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
scripts/check-phase127-authoritative-network-state-unification.test.ts
scripts/check-phase127-authoritative-network-state-unification.ts
scripts/check-phase155-filter-index.test.ts
scripts/check-phase155-filter-index.ts
scripts/check-phase156-prune-coordination.test.ts
scripts/check-phase156-prune-coordination.ts
scripts/check-phase156-prune-coordination/contracts.ts
scripts/check-phase157-index-catch-up.test.ts
scripts/check-phase157-index-catch-up.ts
scripts/check-phase157-index-catch-up/contracts.ts
scripts/check-phase159-filter-rpcs.test.ts
scripts/check-phase159-filter-rpcs.ts
scripts/check-phase159-filter-rpcs/contracts.ts
scripts/check-phase161-operator-index-evidence.test.ts
scripts/check-phase161-operator-index-evidence.ts
scripts/verify.sh
```

## Closing Summary Flag Inspection — 2026-10-11T10:56:33Z

Read the actual completed `161-09-SUMMARY.md` generated at **2026-10-11T10:54:58Z**. Its inspected SHA256 is `62bc4690ac39c0d696e9f8b488f2bdd596f7fe50bf31d2c4742e4cf1f84d87d5`. It records tasks `[1,2,3]`, `completion_gate: passed` and `requirements-completed: [CFOP-01]` after the prior actual native, source/security, goal and strict eleven-plan/eleven-summary lifecycle evidence. Its `## Threat Flags` section explicitly reports **None**, with no new production trust boundary, dependency, authentication change or unmapped control.

The completion scope remains CFOP-01. CFPR-02, CFGR-01/02, Phase 162 and v2.5 completion remain pending. Synthetic/private-capture/manual-owner-plan limits, BASIC-only work bounds, logical-byte exclusions, body/wallet negatives, unmodeled implicit origins and the existing crash-window/public/default/production/funds-safety exclusions remain intact. The informational relative compile-time fixture include is the previously reviewed private producer/freshness mechanism, not a new production surface. Final metadata freshness, global activation and actual hook/commit/push remain subsequent root-owned actions; no such outcome is preclaimed.

Independently recomputed the current 113-path manifest SHA256 `f67402aa0ee09b904c911e0e2434218f07a8c4a5fe184513a6fa1733179c112e` and ordered path/content digest `31828e43196124e3b6d5e23261006f1ef073b0387ba9e6d6ea8c942182fd260f`: both exactly match the frozen Native 5 evidence. This closing check introduced no implementation or tracker change and ran no Cargo, Bazel, test job or Git operation; only this security artifact was updated.

**Result:** no unregistered flag, new threat or scope expansion. **Original security approval remains verified at 2026-10-11T10:47:21Z; 35/35 controls remain CLOSED, threats_open: 0, accepted risks: none.** The document generation/inspection timestamp advances only to record this metadata appendix. The final goal-report freshness refresh may now follow; no renewed security approval or source rerun is required by this unchanged summary.

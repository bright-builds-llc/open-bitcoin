---
phase: 152-post-prune-wallet-rescan-eligibility
verified: 2026-10-03T02:26:55Z
status: passed
score: 11/11 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T02:26:55Z
lifecycle_validated: true
overrides_applied: 0
native_verification_status: passed
requirement_activation: root_pending
---

# Phase 152: Post-Prune Wallet Rescan Eligibility Verification Report

**Phase Goal:** Node and durable RPC wallet rescans admit entries only when their creating payloads are present, including midrange scans and chunk resume after actual pruning; leftover snapshots remain non-authoritative.
**Verified:** 2026-10-03T02:26:55Z
**Status:** passed
**Re-verification:** No — initial verification; no prior Phase 152 verification or overrides existed.

## Goal Achievement

The implementation satisfies the scoped goal. Both durable adapters consume one staged replacement gate, and real paired-prune regressions prove that missing earlier creating payloads refuse before wallet persistence. The complete default native verifier passed before this report was created. SNAP-01 is behaviorally satisfied; canonical requirement, completing-summary and phase-tracker activation remain ordered root closeout work.

### Observable Truths

All four ROADMAP success criteria are retained verbatim below. The additional seven truths merge the three PLAN contracts; adapter-specific restatements of the same roadmap behavior, repeated success controls and the duplicate native/lifecycle evidence condition are deduplicated without removing scope.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Both scan paths enforce eligibility for every entry admitted to a full replacement, including creating heights before the requested start. | VERIFIED | `wallet_registry/rescan.rs:126` stages the existing pure selection on a copied wallet; lines 136–165 gate the union of requested heights and selected UTXOs' creating heights, resolve actual height membership and deduplicate hashes. Node `sync/wallet_rescan.rs:114` and RPC `context/rescan.rs:117` invoke this same durable helper. |
| 2 | A post-prune midrange scan cannot silently replace the wallet with entries whose creating payload is absent; refusal preserves prior wallet state. | VERIFIED | Node `eligibility.rs:511` and RPC `rescan_eligibility.rs:31` perform actual paired deletion before midrange scan, then compare the complete persisted WalletSnapshot. Node saves only at `sync/wallet_rescan.rs:160` after preparation; RPC saves only at `context/rescan.rs:244`. Both corresponding native tests passed. |
| 3 | Chunk resume and same-datadir reopen preserve the same rule and reject leftover snapshot authority. | VERIFIED | Node `eligibility.rs:52` succeeds through 1, deletes creating payload 1, plants conflicting leftover bytes, drops the runtime and automatically resumes after reopen; it then reopens Failed evidence and compares checkpoint/wallet. RPC `rescan_eligibility.rs:92,128` covers poison reopen and later-request/reopen rechecks. No successful probe is cached as permission. |
| 4 | Runtime regressions using real paired deletion and the default verifier pass; source-string assertions alone do not prove the flow. | VERIFIED | Native log records all 8 shared, 10 node eligibility and 19 RPC eligibility tests as `ok`. Root session 87304 completed default `bash scripts/verify.sh` with exit 0, including workspace checks, benchmarks, Bazel and pure-core coverage; precise evidence below. |
| 5 | Refusal preserves the actual successful checkpoint and persists safe Failed evidence for identified jobs, including authority/probe errors; persistence errors propagate and target/freshness remain truthful. | VERIFIED | Node identifies jobs before authority reads and handles failures at `sync/wallet_rescan.rs:134–157`; `mark_failed` changes only state/error. RPC handles authority, missing-target and preparation refusal at `context/rescan.rs:152,199,235,321`; Pending construction at 277–309 retains checkpoint/next/MTP and recalculates freshness. Native tests cover safe details, Failed-save errors, missing stop with/without a job, new/reused/unknown checkpoints and all three freshness arms. |
| 6 | Retained creating payloads permit replacement, and unrelated older pruned coins do not prevent it in either adapter. | VERIFIED | Separate native success controls at node `eligibility.rs:148,176` and RPC `rescan_eligibility.rs:67,186` assert selected UTXOs, tip/balance and retained durable coins. Only selected wallet candidates extend the required-height set. |
| 7 | Interrupted two-head coins authority refuses at direct wallet loading, shared preparation and identified-job advancement without repairing or using leftovers. | VERIFIED | `storage/fjall_store/coins.rs:115,147` checks recovery markers before coin reads. Node `eligibility.rs:216` physically plants H=[new,old], removes B, retains coins/metadata/poison and asserts typed InterruptedWrite at all three seams, unchanged wallet/checkpoint and durable Failed after reopen. RPC injected typed errors exercise its actual refusal/persistence handler; they are not physical RPC H/B fixtures. |
| 8 | The durable direct rescan helper cannot bypass authority/eligibility; local fixture scans remain usable. | VERIFIED | `context/wallet_state.rs:213` routes DurableNamedRegistry to `rescan_wallet_range(Some(0), None)` and retains supplied-snapshot scanning for Local. Native RPC tests at `rescan_eligibility.rs:506,522` prove fabricated favorable durable input refuses after real prune and the local control succeeds. |
| 9 | Contributor/operator documentation explains tested full-replacement eligibility and safe post-prune refusal. | VERIFIED | `docs/parity/catalog/chainstate.md:45` documents the requested-plus-creating contract, preserved wallet/checkpoint, safe failure facts, fresh resume/reopen, actual runtime fixtures and repeatable repo-local Cargo/timing-wrapper commands. Claims match inspected code and assertions. |
| 10 | SNAP-01 parity evidence identifies Phase 152 closure while preserving Phase 146 foundations and pinned wallet anchors. | VERIFIED | Exactly one index surface and one checklist row own SNAP-01: `v2-4-wallet-leftover-snapshot-cutover`. Historical Phase 146 evidence remains, Phase 152 runtime evidence is linked, wallet.cpp/transactions.cpp anchors exist, and the catalog explicitly distinguishes Knots transaction scanning from the project's full UTXO replacement. |
| 11 | README status distinguishes implemented/tested evidence from ordered root lifecycle activation and pending Phase 153 retention; closure relies on runtime and lifecycle evidence. | VERIFIED | README's scoped status paragraph links the new evidence and retains the Pending activation boundary. Its exact existing v2.4 claim and deferred paragraph match HEAD. The index/catalog also distinguish historical `done` foundations from canonical activation. This passing report supplies the completed native evidence; root updates closeout status afterward. Phase 153 remains pending. |

**Score:** 11/11 truths verified. No verification overrides were needed.

### Required Artifacts

The GSD artifact CLI passed **11/11** across Plans 01–03. Manual inspection also verified substantive implementation and actual consumers; existence and string matches alone were not treated as proof.

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/wallet_registry/rescan.rs` | Shared staging, typed refusal, membership and payload gate | VERIFIED | Public exports, copied-wallet pure selection, explicit requested/candidate union, safe formatter, marker guard and shell probes; node and RPC consumers use it. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | InterruptedTwoHeads wallet-authority refusal | VERIFIED | Guard precedes direct loader reads and is called by shared durable preparation; no repair or leftover read in the wallet loader. |
| `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` | Job-first chunk refusal before replacement | VERIFIED | Existing job resolved first, inactive jobs return without chain reads, handled preparation persists Failed, success alone saves wallet/progress; enqueue and automatic resume reach this path. |
| `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs` | Real prune/resume/reopen/error/control evidence | VERIFIED | Registered by the parent test module; 10 native tests passed. Paired deletion and physical H/B fixtures assert actual durable state. |
| `packages/open-bitcoin-rpc/src/context/rescan.rs` | Shared durable range gate and safe job evidence | VERIFIED | Real RPC dispatch reaches the range path; live admission authority is preserved, target membership required, shared preparation gates persistence and errors are handled. |
| `packages/open-bitcoin-rpc/src/context/wallet_state.rs` | Guarded durable direct helper | VERIFIED | Durable helper delegates to range; Local retains pure supplied-snapshot behavior. |
| `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs` | Durable paired-prune/refusal/reopen matrix | VERIFIED | Registered in `context/tests.rs`; 19 native tests passed, including WR-01 freshness and missing-target regressions. |
| `docs/parity/index.json` | Stable unique SNAP owner and pinned roots | VERIFIED | JSON parses, exactly one SNAP owner, historical and new evidence resolve, wallet.cpp and wallet/rpc/transactions.cpp anchors retained. |
| `docs/parity/checklist.md` | One SNAP row with foundation/closure evidence | VERIFIED | Exactly one SNAP row, matching owner and accurate activation/concurrency boundaries. |
| `docs/parity/catalog/chainstate.md` | Eligibility/refusal/resume operator contract | VERIFIED | All scoped links resolve; real node markers and injected RPC errors are described accurately, with no broader algorithm or readiness claim. |
| `README.md` | Narrow contributor status and Phase 153 boundary | VERIFIED | Scoped diff preserves the exact prior claim/deferred text and links executable closure evidence. |

Supporting artifacts also pass substance/wiring checks: shared helper tests are registered by `rescan.rs`; RPC fixture helpers are consumed by the 19-case matrix; all new first-party Rust paths have parity breadcrumbs, with **911** paths verified. The tracked LOC report was current at native gate entry. No dependency or wallet/job schema change was introduced.

### Key Link Verification

All **9/9** PLAN key links passed the CLI and manual control-flow checks.

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| Node shared rescan helper | Pure `wallet/scan.rs` | Staged `rescan_chainstate` | WIRED | Existing descriptor selection produces the candidate UTXOs whose creating heights are checked. |
| Node chunk adapter | Shared helper | `prepare_durable_wallet_rescan` | WIRED | Returned candidate is required before `save_wallet` and progress mutation. |
| Node chunk adapter | Registry/job persistence | `mark_failed`, `save_rescan_job` | WIRED | Authority and preparation errors save safe Failed; failure-save errors escape visibly. |
| RPC range adapter | Shared helper | Fresh durable preparation | WIRED | Ordinary public range wrapper passes real guard/preparation/save callbacks; private injections reach the same handler. |
| RPC direct helper | RPC range adapter | Durable delegation | WIRED | Caller-supplied snapshot cannot replace durable authority. |
| RPC prune fixtures | Fjall paired-unlink owner | `commit_paired_unlink` | WIRED | Same supplied store really loses block and undo; durable coins/best-block and have-pruned are asserted. |
| Parity index | Pinned transactions.cpp | Existing surface upstream source | WIRED | Actual pinned `rescanblockchain` requested-range `hasBlocks` anchor. |
| Chainstate catalog | Node eligibility tests | Runtime evidence link | WIRED | Concrete paired-prune/resume/interruption behavior, not historical source-string proof. |
| Chainstate catalog | RPC eligibility tests | Runtime evidence link | WIRED | Concrete durable adapter/prune/reopen/error/control evidence. |

### Data-Flow Trace (Level 4)

| Artifact | Data variable | Source | Produces real data | Status |
| --- | --- | --- | --- | --- |
| Node replacement wallet | Authority UTXOs → staged wallet UTXOs | Guarded `wallet_scan_chainstate_snapshot` reads durable coins and chain metadata; existing pure selection filters through the replacement height | Yes; actual Fjall fixtures retain coins after deleting payloads, and success controls return 60,000 sats / selected entries | FLOWING |
| Durable RPC replacement | Live snapshot UTXOs → staged wallet | `blockchain_snapshot` → network export → `admission_snapshot` → parent coins plus pending overlay; durable marker and payload checks remain separate | Yes; fixture explicitly asserts DurableNamedRegistry and live/durable UTXO equality, then executes the real range adapter | FLOWING |
| Durable job/freshness evidence | Checkpoint, next height, MTP, target, state and freshness | Real registry/job persistence; Pending projection uses known/unknown retained checkpoint, and RPC status consumes persisted Pending/Scanning jobs | Yes; WR-01 test reads durable Pending/Partial plus RPC scanning=true/Partial after Failed-save error | FLOWING |

The wallet loader never reads the leftover snapshot key. Poison bytes remain physically present in reopen tests but cannot supply authority or replacement data. Empty snapshots/default collections in tests are deliberate local or refusal inputs; production candidates are populated from actual authority.

### Behavioral Spot-Checks

No duplicate Cargo/Bazel jobs or services were started by this verifier. Completed targeted commands were corroborated against local timing records, source assertions and the root's native test log. The verifier's log/JSON inspections completed in under 10 seconds and did not mutate runtime state.

| Behavior | Command/evidence | Result | Status |
| --- | --- | --- | --- |
| Shared full-replacement membership and probe rules | Node `cargo test --lib wallet_registry::rescan --all-features`, through the timing wrapper | Targeted 8/8; all 8 independently observed `ok` in the native log | PASS |
| Real node prune/resume/reopen and authority refusal | Node `cargo test --lib wallet_rescan_runtime::eligibility --all-features` | Targeted 10/10; all 10 independently observed `ok` in native log lines 4614–4629 | PASS |
| Real durable RPC refusal, controls, target and freshness | RPC `cargo test --lib context::tests::rescan_eligibility --all-features` | Final targeted 19/19, session 68125 exit 0; all 19 independently observed `ok` in native log lines 4747–4852 | PASS |
| Existing construction/range/freshness/wallet compatibility | Sequential timing-wrapper filters and scoped Clippy | Construction 5/5, range 1/1, freshness 4/4, node wallet 39/39; final RPC Clippy session 15019 exit 0; native workspace test/Clippy gates also passed | PASS |

Node actual prune reproducer and corrected durable RPC reproducer both had observed pre-change behavioral RED and subsequent GREEN. Setup/import failures and the initial Local RPC fixture are excluded from behavioral RED evidence. Independent timing records corroborate RED exit 101 and final scoped success exit 0; complete native execution independently confirms final behavior.

### Complete Native Verification

Root confirmed **session 87304, exit_code 0** for default `bash scripts/verify.sh`, with isolated pinned **Bun 1.3.9** first on PATH. The log ends:

`verify.sh completed in 34m 46.545s (2086545ms)`

The independently read timing record is `outcome: success`, `exitStatus: 0`, started `2026-10-03T01:49:47.260Z`, ended `2026-10-03T02:24:34.277Z`, wrapper duration **2,087,017 ms**. Script duration and wrapper duration are distinct observations.

Evidence log: `packages/target/phase152/verification/full-verify.log`.
Timing record: `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-03T01-49-47.260Z-908c9cdc-91d1-4300-8fc9-97b806b98bcf.json`.

Passed stages include hooks/LOC/breadcrumb freshness, deterministic parity/claim and managed checks, architecture/dependency/file-length/panic checks, workspace formatting check, Clippy, all-target/all-feature build, workspace tests, benchmark list/smoke/report validation, Bazel smoke build/provenance and pure-core LLVM coverage. Managed checks reported **1,222 files, zero findings, zero file-cap exceptions**; breadcrumbs covered **911 Rust files**. No `--fast` substitute, public-network gate or verifier interruption was used. `git diff --check` passed.

### Requirements Coverage

| Requirement | Source plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| SNAP-01 | 152-01, 152-02, 152-03 | Wallet rescan reads durable coins and payload-present blocks, and does not treat leftover snapshot bytes as chain truth. | SATISFIED behaviorally; canonical activation is root-owned | Truths 1–11, real node/RPC paired-prune and poison/reopen regressions, shared authoritative gate, unique parity owner, complete native success and final lifecycle check. |

All three plans claim SNAP-01. REQUIREMENTS maps only SNAP-01 to Phase 152, so no orphaned phase requirement exists. Its Pending checkbox/traceability and the summaries' intentionally empty `requirements-completed` arrays are the mandated activation order, not evidence of unmet runtime behavior. Root completes Summary 03 and canonical tracking after this report validates.

### Anti-Patterns Found

No blocking TODO/FIXME/placeholder, empty production implementation, swallowed new failure, orphaned gate, disconnected data or unchecked candidate-save path was found in the touched sources. New tests exercise behavior with actual stores and registered modules. The clean REVIEW has zero unresolved findings; WR-01 was fixed and its final native regressions passed. SECURITY verifies all 17 planned threat dispositions, including the explicitly accepted concurrency boundary.

### Disconfirmation and Scope Limits

Three failure hypotheses were checked: range-only admission of older coins, cached/leftover authority after prune/reopen, and failure paths that lose checkpoint or retain misleading target/freshness. Real deletion tests and inspected control flow disconfirm each within the locked scope. No partially met Phase 152 requirement was found.

Historical Phase 146 source-string checks alone would be misleading evidence for this flow, and are not used as closure proof. There is no dedicated new Phase 152 malformed-descriptor selection-error injection; the selection error mapping at shared `rescan.rs:129` and the same node/RPC handled refusal boundaries were inspected directly. The report does not invent a test for that path.

Presence probes and wallet persistence are separate operations. HTTP requests on the same context serialize under `http.rs:277`, manual prune uses the authority mutation mutex, and current automatic `flush_coins` supplies an empty prune plan. Arbitrary direct-library concurrent pruning can still delete between probe and save. This is the explicitly resolved completed-prune/fresh-resume contract, not an override or a new gap; a new concurrent owner or stronger presence-at-save claim requires replanning.

The later roadmap was checked: Phase 153 specifically owns automatic retained-payload usage/planner/lifecycle integration and its wallet eligibility regressions. That capability remains outside Phase 152; no failed current must-have was deferred to hide a gap. Incremental scanning, snapshot deletion, repair, archive serving, assumeutxo, public defaults and production-funds claims remain outside the closure.

### Human Verification Required

None. This phase's local storage, pure selection, job evidence and RPC adapter contracts are programmatically checkable and have executed regressions. No visual, live external service, public-network, or performance-feel claim is made. The human-verification section contains no pending tests.

### Lifecycle and Guidance

CONTEXT, all three PLANs and all three SUMMARYs share `lifecycle_mode: yolo` and `phase_lifecycle_id: 152-2026-10-03T00-26-52`; upstream CLI preflight returned `valid: true`, without direct-fallback provenance. The report timestamp follows all upstream generation times.

Final required CLI validation passed with exit 0, `valid: true` and `reasons: []`: context valid, 3/3 plans valid, 3/3 summaries valid and this verification valid with matching provenance.

`node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 152 --expect-id 152-2026-10-03T00-26-52 --expect-mode yolo --require-plans --require-verification`

The lifecycle CLI requires the candidate boolean to be true before it can validate the report. That schema bootstrap is not completion evidence: the flag is retained only after successful required CLI validation, and would be cleared on failure. No checker/source changes are made for validation.

Material guidance: repo-local AGENTS and Bright Builds sidecar, placeholder-only standards overrides, standards index, architecture, code-shape, testing, verification and Rust pages; verifier overrides/gates/thinking/calibration references. Both canonical active lessons were read completely: 5,230 global plus 1,958 repo bytes, **7,188 bytes / 2,397 estimated tokens**, with no archived lessons loaded. Project skill directories were absent. This verification is inside the parent GSD lifecycle; only this report is verifier-owned. No source, tracker or git edits were performed by the verifier.

### Gaps Summary

No blocking gaps or human verification items remain. All 11 merged must-haves, 11 required artifacts and 9 key links are verified. The complete native gate passed. Root can perform ordered SNAP-01 and phase activation, then git finalization; automatic retention remains Phase 153.

_Verified: 2026-10-03T02:26:55Z_
_Verifier: the agent (gsd-verifier)_

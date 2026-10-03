---
phase: 152-post-prune-wallet-rescan-eligibility
slug: post-prune-wallet-rescan-eligibility
status: verified
threats_total: 17
threats_closed: 17
threats_open: 0
asvs_level: 1
block_on: high
security_enforcement: true
native_verification_status: pending
requirement_completion: pending
created: 2026-10-03
audited_at: 2026-10-03T01:51:39Z
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T01:51:39Z
---

# Phase 152 — Security

All 17 declared threats have verified dispositions: 16 implemented mitigations and one explicitly accepted concurrency limitation. No declared mitigation gaps remain. `status: verified` records this scoped threat audit; it does not claim the pending default native verifier passed, complete SNAP-01, or establish broader security certification.

## Scope and Method

Input state B: no existing Phase 152 security report; all three plans and summaries exist. The auditor read the three PLAN threat-model registers, all summaries, CONTEXT, resolved RESEARCH, clean REVIEW, relevant implementation/test/documentation paths, local instructions and active lessons. Threat IDs T-152-01 through T-152-17 are unique across the plans. Dispositions are 16 `mitigate`, one `accept`, zero `transfer`.

For each mitigation, the auditor searched its declared patterns, inspected the relevant control flow and executable assertions, and recorded file/line evidence below. The accepted risk is recorded in this report. This audit verifies listed threats; it introduces no new vulnerability scan, implementation changes, dependency changes, commits, or Cargo/Bazel execution. Root owns full verification and lifecycle closeout.

ASVS L1 is the plans' scoped V2/V4 preservation, V5 typed eligibility/range validation and V7 safe evidence handling. Configuration has no explicit false `workflow.security_enforcement`, so enforcement is enabled. The delegated severity threshold is high: high-severity authority, unchecked replacement or misrepresented completion gaps would block closure. No such declared gap was found.

## Trust Boundaries

| Boundary | Required control | Data crossing |
| --- | --- | --- |
| Durable coins/chain metadata → wallet staging | Refuse interrupted heads and unavailable metadata; exclude leftover authority | Coins, active-chain positions, best-block/head markers |
| Authenticated RPC → named wallet/range | Authenticate before parsing; resolve existing wallet and validate bounds | Wallet name and requested heights |
| Staged replacement → payload store → wallet persistence | Check requested and selected creating heights before returning a candidate | Selected UTXOs and block hashes |
| Authority/eligibility failure → job/RPC evidence | Retain wallet/checkpoint; save safe Failed detail; propagate persistence errors | Boundary/category and known height/hash |
| Runtime evidence/history → parity/README claims | Unique requirement owner, accurate fixture descriptions, pending full gate | Observed targeted results and lifecycle provenance |

## Threat Register

Paths below are repository-relative; cited line numbers identify inspected control or assertion entry points.

| Threat ID | Category | Component | Disposition | Mitigation and evidence | Status |
| --- | --- | --- | --- | --- | --- |
| T-152-01 | T — Tampering | Shared preparation/node chunk | mitigate | Staged copy and pure selection; union requested/candidate creating heights; explicit height membership; all distinct hashes probed before return: `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:126`, `:136`, `:147`, `:167`. Node saves only after successful preparation: `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:152`. Actual outside-range paired-prune refusal: `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:511`. | CLOSED |
| T-152-02 | T — Tampering | wallet_scan_chainstate_snapshot | mitigate | Marker guard runs before coins reads; InterruptedTwoHeads returns typed InterruptedWrite; loader reads coins and chain_meta only: `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:115`, `:147`, `:495`. Real missing-B/two-H plus poison fixture refuses loader, shared preparation and existing-job advance: `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:216`. | CLOSED |
| T-152-03 | R — Repudiation | advance_wallet_rescan | mitigate | Existing job identified and inactive jobs returned before authority reads; handled authority/selection/preparation errors mark Failed and call fallible save before returning original error: `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:134`, `:143`, `:155`. `mark_failed` changes only state/error: `packages/open-bitcoin-node/src/wallet_registry.rs:131`. Checkpoint equality and Failed-save propagation: `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:35`, `:298`, `:335`, `:368`. | CLOSED |
| T-152-04 | I — Information disclosure | Failure details | mitigate | Safe formatter emits category/boundary and only known height/hash, without original backend text: `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:45`. Raw error retained for typed caller category; durable detail omits injected path: `packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs:123`, `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:298`. | CLOSED |
| T-152-05 | D — Denial of service | Shared probe set | mitigate | Required-height BTreeMap and probed-hash BTreeSet deduplicate presence work; old nonmatching coins are not candidates: `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:136`, `:146`; byte-presence probe only: `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs:13`. Deduplication and unrelated-old-coin controls: `packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs:78`, `:144`, `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:176`. | CLOSED |
| T-152-06 | T — Tampering | Probe/save sequencing | accept | Accepted risks log AR-152-06 below, implementing the explicit Plan 01/RESEARCH disposition. HTTP dispatch mutex and manual-prune owner verified; completed-prune and fresh resume/reopen evidence only, with no global atomic probe/save claim. | CLOSED |
| T-152-07 | E — Elevation of privilege | Named RPC wallet/range | mitigate | Existing authentication remains before JSON parse: `packages/open-bitcoin-rpc/src/http.rs:136`; wallet scope validated before dispatch: `:277`. Wallet resolution refuses missing/ambiguous names: `packages/open-bitcoin-rpc/src/context/wallet_state.rs:342`. Durable resolution precedes mutation and start≤stop≤tip checks remain: `packages/open-bitcoin-rpc/src/context/rescan.rs:140`, `:185`. No new endpoint/auth surface in scoped diff. | CLOSED |
| T-152-08 | T — Tampering | Durable range/direct helper | mitigate | Production range delegates to shared durable preparation; saves only on successful return: `packages/open-bitcoin-rpc/src/context/rescan.rs:108`, `:235`, `:244`. Durable direct helper ignores supplied snapshot authority and routes to range: `packages/open-bitcoin-rpc/src/context/wallet_state.rs:213`. Fabricated favorable snapshot refuses after real prune; local helper control preserved: `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:506`, `:522`. | CLOSED |
| T-152-09 | R — Repudiation | RPC refusal | mitigate | Identify registry/wallet before authority; safe Failed on known-job authority and preparation refusal; Failed-save errors propagate: `packages/open-bitcoin-rpc/src/context/rescan.rs:140`, `:152`, `:199`, `:237`, `:321`. Preserve actual checkpoint/next/MTP rather than requested-start progress: `:277`. Actual wallet/checkpoint equality, no invented target and save-error assertions: `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:31`, `:128`, `:292`, `:475`, `:549`, `:574`. | CLOSED |
| T-152-10 | I — Information disclosure | RPC/job diagnostics | mitigate | Eligibility refusal maps shared safe_detail to RPC and job text: `packages/open-bitcoin-rpc/src/context/rescan.rs:159`, `:238`, `:339`, using `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:45`. Probe and authority injection assertions exclude backend path/secret text: `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:225`, `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs:274`. | CLOSED |
| T-152-11 | T — Tampering | Interrupted/reopened store | mitigate | Real durable guard on each request and preparation; no cached successful eligibility: `packages/open-bitcoin-rpc/src/context/rescan.rs:113`, `:152`, `:235`, `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:105`. Actual paired-delete/reopen/poison/later-request tests: `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:92`, `:128`. RPC typed authority failures enter production handler: `:453`, `:466`; actual H/B fixture remains node-owned at `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:216`. | CLOSED |
| T-152-12 | D — Denial of service | Probe work | mitigate | Durable RPC reuses shared deduplicated presence gate: `packages/open-bitcoin-rpc/src/context/rescan.rs:117`, `:235`, `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:136`. Actual retained-payload and unrelated old paired-prune success controls: `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:67`, `:186`. | CLOSED |
| T-152-13 | T — Tampering | SNAP-01 index/checklist | mitigate | Exactly one stable SNAP-01 index surface and checklist row; historical Phase 146 verification retained, new runtime files/summaries linked, no premature Phase 152 verification link. `docs/parity/index.json` surface `v2-4-wallet-leftover-snapshot-cutover`; `docs/parity/checklist.md:91`; `docs/parity/catalog/chainstate.md:45`. Canonical requirement remains Pending: `.planning/REQUIREMENTS.md:13`, `:97`. | CLOSED |
| T-152-14 | R — Repudiation | README/closure summary | mitigate | Explicit pending root full gate and SNAP-01 status: `README.md:28`, `docs/parity/catalog/chainstate.md:52`, `:103`; exact repeatable commands: `:120`. All summaries have matching lifecycle ID and empty requirements-completed; executor result tables state their provenance and pending native gate. No full pass fabricated. | CLOSED |
| T-152-15 | I — Information disclosure | Catalog failure examples | mitigate | Operator contract limits evidence to safe boundary/category/known height/hash, excluding raw paths/snapshot contents: `docs/parity/catalog/chainstate.md:77`. No raw fixture backend strings or poison bytes reproduced in the Phase 152 catalog/README descriptions. | CLOSED |
| T-152-16 | T — Tampering | Behavioral/production claims | mitigate | Explicit Knots incremental-scan versus project full-replacement distinction: `docs/parity/catalog/chainstate.md:70`. Real node markers versus injected RPC authority tests accurately named: `:97`. Scoped concurrency and deferred retention/readiness boundaries: `:129`, `README.md:28`, `:39`. Exact existing README v2.4 claim/deferred paragraph preserved in scoped diff. | CLOSED |
| T-152-17 | T/R — Tampering/Repudiation | Reused Pending target/freshness evidence | mitigate | Unified new/reused job construction retains actual checkpoint/next/MTP, sets verified target, then recomputes freshness before Pending save: `packages/open-bitcoin-rpc/src/context/rescan.rs:224`, `:277`, `:300`. Three arms observed: reused Fresh→new Partial and Failed-save Pending/Partial readback (`packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:328`), new known checkpoint/Partial (`:31`), unknown/Scanning (`:390`), checkpoint-at-target/Fresh after actual prune refusal (`:426`). Later Failed/Partial persists before/after reopen (`:128`). | CLOSED |

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale and boundary | Accepted By | Date |
| --- | --- | --- | --- | --- |
| AR-152-06 | T-152-06 | Phase 152 proves eligibility after completed pruning and fresh chunk/request/reopen probes. Separately exported node/store callers have no common atomic payload-probe/wallet-save transaction; concurrent direct-library pruning can delete after a successful probe and before persistence. HTTP manual prune/rescan on the same context serialize, and current automatic flush submits no deletions. This is the explicit bounded scope, not an assertion of universal serialization. A new concurrent retention/rescan owner or stronger presence-at-save guarantee requires replanning. | Explicit Phase 152 Plan 01 accepted disposition and resolved RESEARCH scope, reaffirmed by root delegation; auditor records existing acceptance | 2026-10-03 |

Owner evidence: `packages/open-bitcoin-rpc/src/http.rs:277` holds the context mutex across synchronous dispatch; `packages/open-bitcoin-rpc/src/context/prune.rs:133` enters manual planned flush; `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs:55` calls the mutation owner, whose mutex is at `packages/open-bitcoin-node/src/network/runtime_authority.rs:106`. Current `flush_coins` passes `PrunePlan::default()` at `:544`. Shared preparation explicitly documents separate presence/save operations at `packages/open-bitcoin-node/src/wallet_registry/rescan.rs:97`; node advancement persists separately at `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:160`. Caller searches found no new concurrent production pruning owner in the Phase 152 scope.

No transferred risks. No additional risks were accepted by this audit.

## Threat Flags

The summaries have no literal `## Threat Flags` sections; their Threat Review/Stub and Threat Review sections were also inspected. Plan 02's `## Review Threat Flag` contains `threat_flag: job-evidence-freshness (T-152-17)`, now registered in Plan 02 and verified above. It is informational here. Plans 01/03 report no additional attack surface. **Unregistered flags: none.**

## Behavioral Evidence and Limits

The auditor inspected executable runtime assertions and executor/reviewer artifacts; the following pass counts are recorded executor results, confirmed by root delegation, not new security-agent test runs:

| Evidence | Recorded result / purpose |
| --- | --- |
| Plan 01 shared preparation filter | 8/8; metadata refusal, deduplication, safe detail, prior-wallet isolation and height filtering |
| Plan 01 final node runtime eligibility filter | 10/10; actual paired-prune midrange/in-range refusal, fresh automatic resume/reopen, real H/B, safe failure and retained/unrelated controls |
| Plan 01 coins-migration filter and scoped node Clippy | 15/15 and exit 0 |
| Plan 02 final RPC eligibility matrix | 19/19, session 68125 exit 0; includes final Fresh-arm actual prune/refusal test |
| Plan 02 freshness filter | 4/4; Pending/Partial readback and related projections |
| Plan 02 final review-fix scoped RPC Clippy | Session 15019 exit 0; later completion supersedes REVIEW's closeout-time pending note |
| Plan 02 compatibility filters | Construction 5/5, rescanblockchain 1/1, node wallet 39/39 |
| Plans 02/03 managed and breadcrumb checks | Zero managed findings; 911 first-party Rust paths verified |
| REVIEW | Clean; critical/warning/info 0. WR-01 resolved with actual stale Pending/Fresh RED and final GREEN evidence |
| Security agent static/diff verification | Declared patterns, control flow and fixture assertions inspected; `git diff --check` passed |
| Root default `bash scripts/verify.sh` | **Pending at this audit.** No full suite, coverage, Bazel smoke or final lifecycle success is claimed here |

Node fixture `paired_prune` at `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs:436` calls real `commit_paired_delete`, verifies both mates absent, have-pruned, surviving coins and unchanged tip/best-block. RPC fixture at `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs:156` calls real `FlushPersistSink::commit_paired_unlink` on the supplied store, verifies those same facts and all other payload mates retained. The underlying SyncAll batch deletes both keys and inserts have-pruned at `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:109`. These are deletion fixtures, not omitted-save or source-string substitutes.

Real interrupted H=[new,old]/missing-B assertions belong to node tests. RPC uses typed injected authority failures through the production refusal handler and real registry persistence; this audit does not describe those injections as physical RPC marker fixtures. The live RPC authority remains the manager admission snapshot including durable parent coins and overlay; no older disk-tip equality requirement was added.

The three summaries retain empty requirement-completion arrays. The stable parity surface's historical `done` status explicitly distinguishes Phase 146 foundations from canonical Pending SNAP-01 closure. Phase 153 retention stays pending. Root must finish the default verifier, create the passing lifecycle-valid Phase 152 report, and perform canonical closeout before claiming phase completion.

## Guidance and Audit Trail

Material guidance: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture, code-shape, testing, verification, local-guidance and Rust pages. Both canonical active lesson inputs were read completely: 5,230 global bytes plus 1,958 repository bytes, 7,188 total and 2,397 conservative estimated tokens. No archives loaded; no lesson/ledger files modified under this report-only ownership. Root owns workflow/tracker maintenance.

| Audit Date | Threats Total | Mitigated Closed | Accepted Closed | Open | Run By |
| --- | --- | --- | --- | --- | --- |
| 2026-10-03T01:51:39Z | 17 | 16 | 1 | 0 | gsd-security-auditor |

## Sign-Off

- [x] Every declared threat classified and verified by its disposition.
- [x] Explicit accepted concurrency scope recorded; no new risk acceptance.
- [x] Summary freshness flag mapped to T-152-17; no unregistered flags.
- [x] `threats_closed: 17`, `threats_open: 0`, scoped `status: verified`.
- [x] Implementation files untouched by auditor; only this security report written.
- [ ] Root default native verification and passing lifecycle report remain pending.

**Approval:** Scoped threat verification completed 2026-10-03; full phase completion remains root-owned and pending.

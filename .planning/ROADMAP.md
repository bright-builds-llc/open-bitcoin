# Roadmap: Open Bitcoin

## Active Milestone: v2.6 Canonical Chain Bootstrap and Validation Parity

**Status:** Approved 2026-10-13; Phases 164–167 verified complete through 2026-11-05; Phase 168 remains unstarted
**Granularity:** Fine
**Goal:** Start from an empty datadir, validate canonical Bitcoin histories with network-correct rules and cumulative proof of work, and demonstrate matching results against pinned Bitcoin Knots `29.3.knots20260210` (`a9aee730466ac67d35a3c03ee24676be5e045878`).

This user-approved roadmap maps all 30 requirements to eight phases, continuing after shipped Phase 163. Phases 164–167 close 15 requirements across 121 verified plans; 15 requirements and Phases 168–171 remain pending. Approval, phase execution and full ordinary-history qualification remain separate gates.

## Scope and Delivery Gates

- Preserve first-party Bitcoin ownership, functional core / imperative shell boundaries, one active Fjall chainstate and explicit/default-off public networking. Reuse the existing work, coins, lifecycle and BASIC facilities without a new production dependency or workspace crate.
- Phase 164 must place read-only identity inspection before automatic schema/coins migration, initialization, prune resume or index mutation. Phase 165 uses that seam for legacy synthetic-work decisions. Exact-work checkpoint/reopen and coins binding belong to Phase 166 once canonical durable bootstrap exists.
- Pure work and adapter integration may remain capability-gated until canonical bootstrap and validated candidate-history context are complete. Header validity, body validity, active coins, checkpointed state and recovered authority remain distinct; intermediate phases do not establish full canonical acceptance.
- Genuine default-regtest histories use canonical genesis, normal maturity 100, halvings at heights 150 and 300, identical serialized candidates and internally derived work. Independent arithmetic vectors beyond 128 bits remain a separate evidence class from small-work regtest chains. Preserve pinned modulo-`2^256` cumulative arithmetic; any additional overflow refusal requires an intentional-difference record.
- Mainnet historical fixtures must name ancestry, UTXO, hash, activation and time prerequisites, distinguishing seeded contextual checks from ordinary ancestry-complete validation. Existing testnet3 and signet selectors must explicitly refuse unsupported ordinary acceptance when checks are missing, including Signet challenge validation. This milestone adds no testnet4, custom Signet or arbitrary deployment overrides.
- Runner/provisioning design can begin after Phase 164; actual differential qualification cannot precede Phase 167 foundation correctness. Qualification requires an actual pinned Knots executable bound to its source tree, SHA-256 and successful build inputs/options. Source helpers, frozen replay and self-comparison retain their narrower meanings.
- Routine native verification remains offline. Affected production rules, runner/normalization, corpus or oracle build inputs invalidate qualification and require a renewed actual Knots run before closure. Public-network UAT, archive scale, automatic repair/import and production/service/funds claims remain excluded.

## Phases

- [x] **Phase 164: Canonical Chain Identity and Compatibility Gates** - Bind existing network selectors to canonical roots and refuse incompatible state before mutation. (completed 2026-10-14)
- [x] **Phase 165: Shared Exact Work Authority** - Select headers and connected branches using verified target-derived 256-bit work.
- [x] **Phase 166: Canonical Durable Bootstrap and Genesis BASIC** - Initialize and reopen canonical genesis, exact work and BASIC without seeded history.
- [x] **Phase 167: Candidate-History Validation Rules** - Apply pinned network and branch-history rules through ordinary durable validation.
- [ ] **Phase 168: Actual Pinned-Knots Differential Qualification** - Compare identical canonical regtest histories against an identity-verified Knots daemon.
- [ ] **Phase 169: Differential Fork Selection and Durable Recovery** - Compare competing branches and prove earned authority through disconnects and actual reopen.
- [ ] **Phase 170: Canonical Prune and Retained BASIC Integration** - Prove real pruning, retained clients and coherent operator facts on canonical history.
- [ ] **Phase 171: Classified Historical Contexts and Parity Closure** - Qualify bounded mainnet contexts and source-bound milestone evidence without broader claims.

## Phase Details

### Phase 164: Canonical Chain Identity and Compatibility Gates

**Goal**: Operators can identify the selected canonical chain and receive a nonmutating refusal for incompatible roots, networks or unsupported validation modes.
**Depends on**: Shipped v2.5 foundations through Phase 163; no earlier v2.6 phase.
**Requirements**: CBOT-01, CBOT-04, CVLD-05
**Success Criteria** (what must be TRUE):

1. A contributor can check the exact pinned genesis header/body identity for the selected existing network and observe rejection of a different null-parent root. [CBOT-01]
1. An operator opening foreign-network or noncanonical-root durable state gets a bounded diagnostic while prior data remains intact, before automatic coins/schema migration, initialization, prune resume or BASIC writes. The same read-only inspection boundary is available to the later legacy-work decision. [CBOT-04]
1. An operator can identify the supported validation/evidence scope of mainnet, testnet3, signet and regtest; missing consensus prerequisites cause explicit unsupported refusal before accepting history, including absent Signet challenge validation. [CVLD-05]

**Plans**: 13/13 plans verified complete

**UI hint**: no

### Phase 165: Shared Exact Work Authority

**Goal**: Contributors and operators can rely on one exact target-derived work authority for live header selection, connected candidates and reorg decisions, with safe legacy-state refusal.
**Depends on**: Phase 164 (canonical identity and pre-mutation inspection).
**Requirements**: CWOR-01, CWOR-02, CWOR-03, CWOR-05
**Success Criteria** (what must be TRUE):

1. A contributor can reproduce independently expected compact-target validity, block proof and cumulative-work vectors beyond 128 bits, preserving pinned modulo-`2^256` semantics and identifying any additional overflow refusal as an intentional difference. [CWOR-01]
1. An operator observes verified cumulative work determine the live best-header branch, including a higher-work branch that is not the longest; defined equal-work arrival/activation follows the pin and genuinely ambiguous internal ties are explicitly bounded. [CWOR-02]
1. Connected-chain and reorg decisions use the same internally derived work as accepted headers; supplying an inflated caller score cannot authorize a stronger active branch. Live canonical support remains gated on the later bootstrap/context prerequisites. [CWOR-03]
1. Opening legacy synthetic-work state produces an explicit compatibility decision through the Phase 164 read-only gate before migration or live/durable authority changes; unprovable state refuses without reinterpretation, destructive conversion or automatic repair. [CWOR-05]

**Plans:** 41/41 plans executed

**UI hint**: no

### Phase 166: Canonical Durable Bootstrap and Genesis BASIC

**Goal**: Operators can initialize and reopen an empty canonical datadir with proved genesis, coins, exact work and explicitly enabled BASIC authority.
**Depends on**: Phase 164 (identity/compatibility); Phase 165 (exact work representation and authority).
**Requirements**: CBOT-02, CBOT-03, CWOR-04
**Success Criteria** (what must be TRUE):

1. An operator initializes and reopens an empty durable datadir with canonical genesis as best block and no spendable genesis coinbase outputs. [CBOT-02]
1. Interrupted genesis initialization either resumes to proved complete authority or refuses explicitly; it never exposes partial genesis/coins authority as accepted progress. [CBOT-02]
1. Enabling BASIC on a fresh canonical datadir yields its genesis filter and zero-predecessor commitment and permits ordinary indexing without seeded history or implicit public-network activation. [CBOT-03]
1. A checkpoint/all-handle reopen preserves exact cumulative work without truncation, binding it to stored headers, network identity and recovered coins best block; inconsistent bindings cannot become authority. [CWOR-04]

**Plans:** 16/16 plans executed

### Phase 167: Candidate-History Validation Rules

**Goal**: Contributors observe pinned network- and candidate-history-derived validation decisions through ordinary header admission and durable body connection.
**Depends on**: Phase 164 (network scope); Phase 165 (verified work); Phase 166 (canonical durable root).
**Requirements**: CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06
**Success Criteria** (what must be TRUE):

1. A contributor observes network-correct subsidy and ordinary coinbase maturity, including genuine default-regtest maturity 100 and boundary cases at heights 149/150 and 299/300. [CVLD-01]
1. Ordinary durable connects derive BIP34 height, BIP65/BIP66 block-version constraints, BIP113 finality, sequence locks and Segwit commitments from validated candidate ancestry/deployment history; blanket caller activation switches cannot substitute for that context. [CVLD-02]
1. Actual durable script validation follows pinned `GetBlockScriptFlags`, including exact baseline flags, exact block-hash exceptions and deployment-dependent additions, rather than applying present-day rules to every history. [CVLD-03]
1. Header admission and body connection agree on selected-network difficulty, supported retarget/minimum-difficulty boundaries and contextual median-time decisions; missing required history refuses. [CVLD-04]
1. Supported duplicate-coinbase/BIP30 and coin-overwrite cases require the pinned ancestor/hash facts; disconnect restores the correct earlier coins, and unrelated histories receive no global permissive exception. [CVLD-06]

**Plans:** 51/51 plans executed

**Completion checkpoint (2026-11-05):** Plan 20 revised-config default native verifier v15 passed on unchanged product source in 2h 34m 55s, including nine original 16-thread stress families, complete native tests, benchmark, six Bazel smoke targets/provenance and fresh zero-uncovered coverage for the original eight pure-core packages. A checked one-line `.bazelrc` host-tool correction followed the retained failed saving hook; the fresh default six-target build and full v15 pass bind the revised configuration. Independent source review found no actionable issue; security closed 187 distinct declared threat concepts (181 mitigations, six accepted risks). The canonical Phase 167 goal report verifies all five bounded CVLD requirements and decisions D01–D10. Only the private local default-Regtest 301-descendant route is earned product history; Mainnet/Testnet3 seeded fixtures are non-authoritative and actual pinned-Knots daemon differential remains Phase 168. Strict normal-hook commit and push are the remaining saving step. Older checkpoints below are historical.

**Execution checkpoint (2026-10-30T22:12:36+00:00)**: Plan51 FINAL source acceptance `8688d2f4297fbb84c5ed7f7e6b88188ec235c081a4c0270e55a32fb663a9f3b1` follows accepted50. Unvalidated full Block receipt is sender-only; matching cleanup requires real retained full-linkage body and final public Accepted/Duplicate success after durable/achieved/physical checks. Genuine E2/E4 and failed-ancestor REDs led to narrow corrections; a root-only uncached proved genesis exception preserves nonroot cache and full-linkage guards. Checked E2 positive-control timing deviation and unreachable supported-surface terminal BASIC barrier branches are disclosed. Four task literals pass7/20/20/11; all20 fresh finals exit0, broad provider49/49 and node-all2,439 with four documented ignores. Full502-source/13-owner/489-protected, native201+Byteview1/five-Knots frame, original10body/three-absence inverse, independent whole review and canonical Summary QA passed. Phase167 now has46/51 source-accepted contributions; v2.6 has116/121 checked plans complete (96%) and3/8 phases verified. Full Phase167 source manifest/provenance review and Plan17 guards are next, then18/19/25/20 and global CVLD/full verifier/coverage/Bazel/normal-hook main commit/push. Stop before168. Older checkpoints below are historical.

**Execution checkpoint (2026-10-30T03:42:18+00:00)**: Plan50 FINAL source acceptance `466d737aa3584a67c4916a6db873087e14a7ed8166582af3933806930b80da18` follows accepted49. D1/D4 genuine compiled REDs precede arrival-ordered bounded First/Later/Absent compact failure, service-selected full fetch and real canonical compact-header admission before enrollment. Three checked test-only owner additions and a later source-review correction close the 16-owner controls; final task selectors pass6/64/97/5. All 16 fresh serial final commands pass; node-all passes2,408 with four documented ignores after13,004 seconds. Full495-source/native201+Byteview1/four-Knots frame, 14-body/two-absence inverse protecting479, independent whole review and canonical Summary QA passed. Phase167 now has45/51 source-accepted contributions; v2.6 has115/121 checked plans complete (95%) and3/8 phases verified. Plan51 stored final success is next, followed by guards17/18, parity19, root docs25, full20 and global CVLD/normal-hook main commit/push. Stop before168. Older checkpoints below are historical.

**Execution checkpoint (2026-10-28T04:26:00+00:00)**: Plan49 FINAL source acceptance `6132a484ed8a765d9d96b76fa3ef3842a5025c2d9b65ee0fefe179a7759b7e53` follows accepted16. C1/C2 genuine Merkle-mismatch REDs and C3's honest post-GREEN scratch sensitivity lead to one selected compact completion chain and thin explicit-fact public delegates. Four bounded strict-lint corrections precede 13 fresh serial final commands; the full node suite passed 2,403 tests with four documented ignores after 10,997 seconds. Complete 490-source/native201+one-supplement frame, 14-path original inverse protecting476, independent whole review and canonical Summary QA passed. Phase167 now has44/51 source-accepted contributions; v2.6 has114/121 checked plans complete (94%) and3/8 phases verified. Plan50 ordered retry/header admission and Plan51 stored success remain next, followed by guards17/18, parity19, root docs25, full20 and global CVLD/normal-hook main commit/push. Stop before168. Older checkpoints below are historical.

**Execution checkpoint (2026-10-27T23:49:03+00:00)**: Plan16 FINAL source acceptance `76c91bd857ad1735bd4a5d0ef31858be3e7ad23a9cee75c418312a3f92cc2229` follows accepted15. Task1 proves exact historical script bodies, seven derived mechanical ordinary flag-sensitive spends and creating-MTP kernel controls; real REDs led to narrow legacy sighash/IFDUP fixes. Task2 proves both exact BIP30 repeats, full prior-Coin physical restore/reconnect and 39 classified results. The final formatted source passes literal16/16 and7/7 commands, full482-source/native201+1 frame, 11-owner original inverse/471 protected and independent whole evidence/Summary QA. Task2 had no preimplementation behavioral RED; a checked Root disposition records post-GREEN restoration-mutation sensitivity without claiming one. Phase167 now has43/51 source-accepted contributions; v2.6 has113/121 checked plans complete (93%) and3/8 phases verified. A prior Plan14 strict consensus Clippy fixture lint needs repair before final native verification. Compact Plans49-51, guards17/18, parity19, root docs25, full20 and all global CVLD/normal-hook main commit/push remain open. Stop before168. Older checkpoints below are historical.

**Execution checkpoint (2026-10-27T21:24:56+00:00)**: Plan15 FINAL source acceptance `898a20a004047259b6def19e47cc550a84c5b1569c511043e76b83567ff219e8` follows accepted14. The independently frozen ordinary default-Regtest genesis-through301 corpus plus next302 passed32 registered Bun tests and full raw-byte check; the managed path passed7 named tests across19 negative identities, actual height201 creating history, branch MTP, complete-handle reopen at150/301 and next302 with work/coins/BASIC/provenance. A real BIP68 block-reason mismatch was repaired at the chainstate block boundary, preserving the mempool reason; actual public `submitblock` returned the pinned missing-input reason on the same frozen negative bytes. The corrected unified467-source/native201+1 frame, original nine-owner inverse/458 protected, independent reviews and canonical Summary QA passed. Phase167 now has42/51 source-accepted contributions, v2.6 has112/121 checked plans complete (93%) and3/8 phases verified. Plan16 seeded historical evidence is next; all global CVLD, later compact/guard/parity/docs work, full verifier/coverage/Bazel/final reviews and normal-hook main commit/push remain open. Stop before168. Older checkpoints below are historical.

**Execution checkpoint (2026-10-27T20:19:55+00:00)**: Plan14 FINAL source acceptance `4024761b6439e5ded5dfcec23f09853080646416b11e1fdbf196f6d71e3afe34` follows accepted13/48. The bounded default-Regtest support classification and parsed offline CLI status reopen actual301 descendants from the same physical datadir as the paired RPC owner; selected identity, local implementation and actual pinned-daemon qualification remain distinct. Genuine compiled old-route301 RED preceded final2/8 named original-command passes; amended source/native458/201+1 and seven-original inverse/451 protected paths, Root three-file CLI integration, independent source/evidence/inverse and canonical Summary QA passed. Phase167 now has41/51 source-accepted contributions; v2.6 has111/121 checked plans complete (92%) and3/8 phases verified. Reviewed Plan15 source-inert packets require fresh post-FINAL14 source lease and actual tests. All global CVLD, later Phase167, full verifier/coverage/Bazel/final reviews and normal-hook main commit/push remain open. Stop before168. Older dated checkpoints below are historical.

**Execution checkpoint (2026-10-27T17:59:12+00:00)**: Compact lifecycle [Plans 49–51](phases/167-candidate-history-validation-rules/167-49-51-COMPACT-LIFECYCLE-AMENDMENT.md) are now checked planning metadata after accepted Plan35. Independent review passed the exact reciprocal-link correction, conserved all existing task commands, and confirmed the 16→49→50→51→17 order. The three new plans add twelve tasks and three waves; all 61 planned command occurrences remain unrun and source execution is held. Phase167 remains40/51 source-accepted contributions and v2.6 remains110/121 checked plans complete (91%), with3/8 phases verified. Plan14 is executing; all global CVLD, later source work, full default verifier/coverage/Bazel/final reviews and normal-hook main commit/push remain open. Stop before168. Older dated checkpoints below are historical.

**Execution checkpoint (2026-10-27)**: Plan48 FINAL source acceptance `d44365e3e24c045bb50eddc5b9530a53c0805f4c2bed6166cf8a11fbb1f61540` follows accepted13. Its checked seven-owner source has440 present paths/433 protected and201 native plus one supplement. A genuine old-daemon one-child restart RED precedes the rewired GREEN; four original V12 commands pass2/5/1/1 named tests including full301 normal-product process with pinned Knots BASIC and wide txid/Merkle, followed by original4whole3absence scratch inverse and clean independent CORE/Summary QA. The early harness false RED/GREEN, V10 CORE HOLD and V5 inverse HOLD remain historical without final credit. Phase167 now has40/48 source-accepted contributions, v2.6 has110/118 currently planned summaries (93%) and3/8 verified phases. Plan14 support is next; all global CVLD, later Phase167, Plan19 parity mapping, full default verifier/coverage/Bazel/final reviews and normal-hook main commit/push remain open. Stop before168. Older dated checkpoints below remain historical.

**Execution checkpoint (2026-10-26T02:01:38+00:00)**: Plan35 R2 FINAL source acceptance `1d468f364ce14608ca7b81e58191087bf13139db7246e115b1556d17b7454dfc` follows the one-file changed-missing-transaction control, original eight-body/three-absence inverse proof, nine fresh corrected-source commands, CORE/independent review, canonical Summary and independent QA. The full R2 test command passed 3483 named cases with zero failures and four exact ignored cases without pass credit; all 284 source and 201+1 native bodies match. Phase167 now has37/48 accepted source contributions, v2.6 has107/118 currently planned summaries (91%) and3/8 verified phases. Plan47 next requires its own fresh 297-path frame, original three-body/one-absence inverse and reviewed v5 nine-command execution. All five global CVLD, later Phase167, full default verifier/coverage/Bazel/final reviews and normal-hook main commit/push remain open. Stop before168. Earlier dated checkpoints below remain historical.

**Execution checkpoint (2026-10-25T00:33:59.899906+00:00)**: Actual45 FINAL source acceptance 2026-10-25T00:33:59.899906+00:00: RootacceptedjointCOREc20/independent8cc, canonicalSummary45 8db78fd0/independentSummaryQA41d9 PASS followingactualFinal46 8fc/Summaryf19/bcf. Original2behavioralREDs beforeoriginal9writes andsuccessiveR8/R8.1/R8.2 fixes remainhonesthistory; actual20freshjobs1148overlapPASS/exact4ignorednopasscredit/strictNodeD0 andonejointd3c5/source284/native201+separateByteview1/originalc6648whole4absenceinverseprotect272verified. Latest2testnoncebuffersprotect282;productionprefixesunchanged. RootFINAL45 bc9710892e4217a04eedc887c7d04208745f7d3bf558b47926f4f11135547154 binds finalSourceFrame d3c5 andaccepted46 forfuture35realcomposite. Counts36/48sourceaccepted,48plans107tasks38waves,milestone106/11890percent,3of8verified. Full35 originalexclusive5b0before8/3 isunchanged/unaccepted; nextfreshcomposite/twodistinctoriginal35inverseproofs/rootreview BEFOREnewverification-onlyruntimelease/all6+3originalargv. AllfiveglobalCVLD/later167/wholeverifiercoverageBazel/normal-hookMAINcommitpushremainOPEN;STOP168.

**Plans**: 51 checked plans / 119 tasks / 41 waves. The [public-chain-information split](phases/167-candidate-history-validation-rules/167-13-PUBLIC-CHAIN-INFORMATION-AMENDMENT.md) proposes11-owner core13 then5-owner actual daemon48 before14; all original task/argv/process obligations remain. Planning review/publication/source execution and acceptance are separate gates. The [serialization ownership correction](phases/167-candidate-history-validation-rules/167-15-SERIALIZATION-REFINEMENT-AMENDMENT.md) explicitly assigns the pure helper as Task1 fifth/Plan15 ninth owner, preserving all tasks, commands, dependencies, waves and current contribution counts. The [earned-command observation prerequisite](phases/167-candidate-history-validation-rules/167-47-PLAN.md) and its [canonical contract](phases/167-candidate-history-validation-rules/167-47-SUBMISSION-OBSERVATION-AMENDMENT.md) add four node owners/two tasks after actual30/35 and before13; planning publication grants no source lease or another accepted contribution. The [peer authority amendment](phases/167-candidate-history-validation-rules/167-45-46-PEER-AUTHORITY-AMENDMENT.md) includes checked R7 typed owned-payload precision for Plan46 authority (seven owners) and Plan45 integration (nine owners) through an exact not-final dependency-ready SOURCE gate; all joint strict/source/tests precede Summary46 then45. Actual source application remains separately gated. The [full35 repair amendment](phases/167-candidate-history-validation-rules/167-44-45-FULL35-REPAIR-AMENDMENT.md) adds44 complete-family metadata fixture correction and45 atomic peer lifetime/request/accounting correction before fresh35; source execution and acceptance remain separately gated. The [opened-unlink prerequisite](phases/167-candidate-history-validation-rules/167-43-OPENED-UNLINK-AMENDMENT.md) adds checked43 after accepted40/42 and before fresh35: a known safe opened zero-link source can abort a full attempt as Changed only after all safety checks, retaining the original error for strict callers and every ledger/stamp/root/hash bound. All106tasks are concrete; actual43/44 are accepted and actual46/45 source contributions remain unaccepted. The older prerequisite notes below retain their historical execution stages. The [logical-freshness amendment](phases/167-candidate-history-validation-rules/167-37-39-LOGICAL-FRESHNESS-AMENDMENT.md) adds checked37 carrier refactor→38 complete tracking/canonical consumers→39 inventory adoption before30. Independent2542 checks/root acceptance preserve all original obligations; Plans37/38 are accepted after real native/logical controls and independent CLEAN reviews;39 source/API/inverse has0findings but required verification is held by a new accepted-physical-inventory512MiB failure in the original305-prefix. The [physical-inventory prerequisite](phases/167-candidate-history-validation-rules/167-40-PHYSICAL-INVENTORY-AMENDMENT.md) adds checked two-owner/two-task40 native64MiB sealed-journal pressure before39, with unchanged complete512MiB physical proof. Independent3895 checks/19 invariants and root acceptance preserve all original task contracts; 40 automatic retirement is observed but final physical recheck failed. The [live physical stability prerequisite](phases/167-candidate-history-validation-rules/167-41-LIVE-PHYSICAL-AMENDMENT.md) adds checked three-task/five-owner41 before40, preserving every Inventory field and full hashes under the original logical stamp/held root and unchanged cumulative work bounds. The [directory-link refinement](phases/167-candidate-history-validation-rules/167-41-DIRECTORY-LINK-AMENDMENT.md) classifies safe directory nlink changes as Changed while full Eq and file-hardlink/type/mode refusals remain, with separate root identity and actual per-walk second-pair proof. Independent7206checks/24invariants and root acceptance release41 alone on62 actual inputs; Plan41 source acceptance follows all7 fresh checks,108 test executions and independent CLEAN0 review; all fresh40/39/30 matrices remain open and full305 remains unproved. The [work-bounded live prerequisite](phases/167-candidate-history-validation-rules/167-42-WORK-BOUNDED-LIVE-AMENDMENT.md) adds checked three-owner/three-task42 after accepted41 and before40: unchanged complete pairs continue only on Changed under the same cumulative work bounds; all old authority, safety and full-hash proof remain. Independent4627checks/36invariants and root planning acceptance publish42; actual native/budget controls, eight fresh finals and source review remain unproved. Actual source acceptance is28. Plans31–34 and36 are accepted. Plan36 closes its15-owner/four-task resource projection and real guard contribution with actual operational refusal/safe-old-reopen and valid both-mode command controls. Plan30 remains held after the confirmed physical-counter freshness failure; its checked four-task/ten-owner contract and original14 plus2 resource commands remain required after37/38/39 acceptance; the [test-partition amendment](phases/167-candidate-history-validation-rules/167-30-TEST-PARTITION-AMENDMENT.md) adds the complete retained-bound child while preserving original task XML and command discovery. Plan17 guard ownership and Plan35 compact fixture/mutation planning are accepted; their source remains gated by actual dependencies, complete final-source review, compactC-E and the formal dependency correction. The resource planning check passes497 assertions, and root implementation acceptance follows all13 exact commands plus Core units and clean independent review. The [wire-boundary amendment](phases/167-candidate-history-validation-rules/167-WIRE-BOUNDARY-AMENDMENT.md) preserves all original obligations and adds serialized-script, complete-wire, typed-result and durable-body prerequisites; its BASIC scanner correction is independently checked. Plans 31/32/33/34/36 are accepted; Plan30 resumes its complete matrix only after the freshness prerequisites pass, with later execution gated by actual accepted dependencies. The [execution-scope amendment](phases/167-candidate-history-validation-rules/167-EXECUTION-SCOPE-AMENDMENT.md), [fixture-projection prerequisite](phases/167-candidate-history-validation-rules/167-26-FIXTURE-PROJECTION-AMENDMENT.md), [complete-undo budget amendment](phases/167-candidate-history-validation-rules/167-27-UNDO-BUDGET-AMENDMENT.md), [managed ingress provider amendment](phases/167-candidate-history-validation-rules/167-09-MANAGED-BRIDGE-AMENDMENT.md) and [ingress test helper split](phases/167-candidate-history-validation-rules/167-09-TEST-SPLIT-AMENDMENT.md) record preserved obligations, execution order and complete historical-corpus/data barriers; accepted summary counts are derived in the progress table below.

The [managed connect caller amendment](phases/167-candidate-history-validation-rules/167-10-CONNECT-CALLERS-AMENDMENT.md) preserves both integration tasks and records the two necessary callers. The [legacy refusal test amendment](phases/167-candidate-history-validation-rules/167-10-LEGACY-REFUSAL-AMENDMENT.md) records the additional test-only scope extension. The independently checked [durable rows and count-carrier amendment](phases/167-candidate-history-validation-rules/167-11-DURABLE-ROWS-AMENDMENT.md) adds Plans 28/29 before the unchanged full replay proof, preserving resource limits and the existing synthetic fixture contract.

Checked Plan13/48 planning allocation: [167-13](phases/167-candidate-history-validation-rules/167-13-PLAN.md) has2 retained tasks; [167-48](phases/167-candidate-history-validation-rules/167-48-PLAN.md) has the1 entire original daemon task. Phase counts48/106/38 describe planning allocation; accepted34 and completed104 remain unchanged. Milestone denominator117→118 gives104/11888% (floor),3/8 verified. Actual35/47 acceptance, fresh source/API/normal-fit/inverse review and separate Root leases remain required; no current46 lease or result is changed. All five globalCVLD and final whole167/normal-hookMAINpush/STOP168 gates remainOPEN.

**UI hint**: no

Historical fixture prerequisites are established here for Phase 171. Canonical ordinary-path acceptance is only claimed for support/evidence classes whose prerequisites are satisfied.

### Phase 168: Actual Pinned-Knots Differential Qualification

**Goal**: Contributors can reproduce finite ordinary-validation comparisons against an actual pinned Knots daemon with source-bound case-level outcomes.
**Depends on**: Phases 164–167 for actual qualification; isolated runner/provisioning design may begin after Phase 164.
**Requirements**: CDIF-01, CDIF-02, CDIF-03, CDIF-04, CDIF-05
**Success Criteria** (what must be TRUE):

1. A contributor runs an actual pinned Knots daemon whose executable SHA-256 is bound to its exact source tree and recorded successful build inputs/options alongside Open Bitcoin's ordinary managed runtime, using isolated local datadirs, identical serialized inputs, disabled external networking and disabled assumevalid shortcuts. [CDIF-01]
1. Both implementations traverse canonical regtest genesis, ordinary 100-block maturity and at least halvings 150 and 300 without synthetic roots, maturity overrides or caller-selected work. [CDIF-02]
1. Invalid blocks and spends produce comparable accept/reject results and unchanged authority on rejection; versioned normalization preserves meaningful rejection differences and does not turn transport/process failures into consensus decisions. [CDIF-03]
1. Case observations compare best-block identity, UTXO facts, consensus-visible exact work and supported BASIC commitments, with intended storage/operator differences separated from unexplained mismatches. [CDIF-04]
1. Bounded case-level results identify both sources, corpus digests and actual process outcomes; a missing/wrong oracle, timeout, incomplete execution or mismatch cannot yield a passing parity receipt. [CDIF-05]

**Plans**: 0 plans (not yet planned)

### Phase 169: Differential Fork Selection and Durable Recovery

**Goal**: Contributors and operators can trust branch-correct coins, tip and exact work through competing branches, disconnect/reconnect and actual durable recovery.
**Depends on**: Phase 165 (work/selection); Phase 166 (durable authority); Phase 167 (branch context); Phase 168 (actual qualified differential runner).
**Requirements**: CREC-01, CREC-02, CREC-03, CREC-04
**Success Criteria** (what must be TRUE):

1. A contributor compares valid competing branches and equal-work arrivals with Knots and observes matching choice for defined activation order; genuinely reference-ambiguous restart ties are separately classified. [CREC-01]
1. Disconnect/reconnect and replacement-spend cases preserve branch-correct UTXOs, while rejected candidates leave active state unchanged in both implementations. [CREC-02]
1. After stopping and releasing every storage handle, ordinary same-datadir reopen preserves earned active-tip, coins and exact-work authority; a shared or cloned live handle is not reopen evidence. [CREC-03]
1. Supported interrupted checkpoints and inconsistent branch/work records recover only to a proved durable prefix or explicitly refuse, without invented progress or silent history repair. [CREC-04]

**Plans**: 0 plans (not yet planned)

### Phase 170: Canonical Prune and Retained BASIC Integration

**Goal**: Clients retain branch-correct BASIC service after real canonical-chain pruning and reopen, while operators see coherent earned chain/index/retention facts.
**Depends on**: Phase 166 (fresh BASIC/bootstrap); Phase 167 (ordinary rules); Phase 168 (qualified corpus); Phase 169 (branch/recovery evidence).
**Requirements**: CINT-01, CINT-02, CINT-03
**Success Criteria** (what must be TRUE):

1. A contributor reproduces actual paired block/undo deletion on a continuous default-regtest canonical chain above pinned ordinary pruning eligibility, preserving the required keep window and index protection; synthetic or fastprune substitutes cannot satisfy the proof. [CINT-01]
1. After genuine pruning and all-handle reopen, retained BASIC RPC plus inbound/outbound peer responses remain branch-correct; removed bodies stay unavailable and filters do not restore wallet eligibility. [CINT-02]
1. Existing RPC, repo-local CLI, dashboard and redacted support consumers agree on canonical tip, exact work, index progress and retention/refusal facts, preserving explicit capability and earned-progress distinctions. [CINT-03]

**Plans**: 0 plans (not yet planned)

**UI hint**: yes

### Phase 171: Classified Historical Contexts and Parity Closure

**Goal**: Contributors can audit bounded historical-network evidence and reproduce current source-bound v2.6 parity claims with explicit residual limits.
**Depends on**: Phase 167 (historical prerequisites); Phase 168 (actual qualification); Phase 169 (fork/reopen proof); Phase 170 (canonical prune/BASIC integration).
**Requirements**: CINT-04, CPAR-01, CPAR-02
**Success Criteria** (what must be TRUE):

1. A contributor audits finite mainnet historical cases with independent pinned expectations and exact ancestry/UTXO/activation prerequisites; seeded contextual, sampled and ancestry-complete evidence are named separately and cannot establish whole-chain mainnet validation. [CINT-04]
1. Every v2.6 parity claim and intentional difference links to pinned source anchors, actual differential results and lifecycle-valid requirement owners; original v2.5 receipts and their synthetic-root/work/halving limits remain separately attributable. [CPAR-01]
1. A contributor runs `bash scripts/verify.sh` offline with deterministic v2.6 regressions and separately reproduces actual Knots qualification. Changed canonical rules, runner/normalization, corpus or oracle build inputs invalidate affected receipts and require renewed actual qualification before closure; claim checks reject unsupported whole-chain, public-default and production-readiness claims. [CPAR-02]

**Plans**: 0 plans (not yet planned)

## Progress

The requirements and roadmap were approved on 2026-10-13. The acceptance dependency order is 164 → 165 → 166 → 167 → 168 → 169 → 170 → 171. Early runner design and historical fixture selection do not remove those acceptance dependencies. Phase 164 passed full native, source/security and 27/27 goal verification. Phase 165 passed full native, independent code/security and all 146 plan truth entries plus four mandatory roadmap criteria. Phase 166 passed the complete default native contract, independent code/security review and all 50 plan truth entries plus four mandatory roadmap criteria; Phase 167 passed its bounded native/source/security/goal verification; Phases 168–171 remain unstarted.

| Phase                                                  | Plans Complete | Status      | Completed  |
| ------------------------------------------------------ | -------------- | ----------- | ---------- |
| 164. Canonical Chain Identity and Compatibility Gates  | 13/13          | Complete    | 2026-10-14 |
| 165. Shared Exact Work Authority                       | 41/41          | Complete    | 2026-10-18 |
| 166. Canonical Durable Bootstrap and Genesis BASIC     | 16/16          | Complete    | 2026-10-21 |
| 167. Candidate-History Validation Rules                | 51/51          | Complete    | 2026-11-05 |
| 168. Actual Pinned-Knots Differential Qualification    | 0/0            | Not started | -          |
| 169. Differential Fork Selection and Durable Recovery  | 0/0            | Not started | -          |
| 170. Canonical Prune and Retained BASIC Integration    | 0/0            | Not started | -          |
| 171. Classified Historical Contexts and Parity Closure | 0/0            | Not started | -          |

Coverage: 30/30 active requirements mapped exactly once; no orphaned or duplicate owners. Later integration phases exercise foundations without reassigning their ownership. Phase 164 contains 13 accepted plans/summaries, Phase 165 contains 41 and Phase 166 contains 16, with their canonical verification records. Phase 167 has 51 accepted bounded contributions across 51 checked plans, including complete default native/coverage/Bazel verification and independent final source/security/goal reports. The milestone has 4/8 verified phases and 121/121 planned contributions complete; Phases 168–171 await subsequent workflows.

## Historical Milestones and Guidance

Latest shipped v2.5 controls: [roadmap](milestones/v2.5-ROADMAP.md), [requirements](milestones/v2.5-REQUIREMENTS.md), [audit](milestones/v2.5-MILESTONE-AUDIT.md). [MILESTONES.md](MILESTONES.md) retains earlier milestone records. Historical `.planning/phases/` directories remain tracked because native verifiers consume selected evidence.

This roadmap follows repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, the placeholder-only `standards-overrides.md`, `standards/index.md` and the architecture, verification and testing standards. Pure domain rules remain separate from storage/process effects; actual receipts and offline replay remain distinct. Repo-local Cargo/Bazel forms are required in subsequent operator UAT guidance. Focused phase research is required for the compatibility seam, work carriers, bootstrap crash ordering, historical rule inventory, oracle provenance, ties/recovery, canonical prune corpus and qualification invalidation.

Phases 164, 165 and 167 use Bitcoin `header` as domain terminology, so the lexical match receives `UI hint: no` and does not trigger UI-design work. Phase 170 includes the existing terminal dashboard/CLI status consumers and receives `UI hint: yes`. The accepted headless scope takes precedence over generic keyword routing; GUI/frontend feature expansion remains deferred.

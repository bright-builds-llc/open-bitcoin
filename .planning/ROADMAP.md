# Roadmap: Open Bitcoin

## Active Milestone: v2.6 Canonical Chain Bootstrap and Validation Parity

**Status:** Approved 2026-10-13; Phase 164 verified complete 2026-10-14; Phase 165 verified complete 2026-10-18
**Granularity:** Fine
**Goal:** Start from an empty datadir, validate canonical Bitcoin histories with network-correct rules and cumulative proof of work, and demonstrate matching results against pinned Bitcoin Knots `29.3.knots20260210` (`a9aee730466ac67d35a3c03ee24676be5e045878`).

This user-approved roadmap maps all 30 requirements to eight phases, continuing after shipped Phase 163. Phases 164–166 close ten requirements across 70 verified plans; the remaining 20 requirements and Phases 167–171 remain pending. Approval, phase execution and full ordinary-history qualification remain separate gates.

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
- [ ] **Phase 167: Candidate-History Validation Rules** - Apply pinned network and branch-history rules through ordinary durable validation.
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

**Plans**: 0 plans (not yet planned)

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

The requirements and roadmap were approved on 2026-10-13. The acceptance dependency order is 164 → 165 → 166 → 167 → 168 → 169 → 170 → 171. Early runner design and historical fixture selection do not remove those acceptance dependencies. Phase 164 passed full native, source/security and 27/27 goal verification. Phase 165 passed full native, independent code/security and all 146 plan truth entries plus four mandatory roadmap criteria. Phase 166 passed the complete default native contract, independent code/security review and all 50 plan truth entries plus four mandatory roadmap criteria; Phases 167–171 have not started.

| Phase                                                  | Plans Complete | Status      | Completed  |
| ------------------------------------------------------ | -------------- | ----------- | ---------- |
| 164. Canonical Chain Identity and Compatibility Gates  | 13/13          | Complete    | 2026-10-14 |
| 165. Shared Exact Work Authority                       | 41/41          | Complete    | 2026-10-18 |
| 166. Canonical Durable Bootstrap and Genesis BASIC     | 16/16          | Complete    | 2026-10-21 |
| 167. Candidate-History Validation Rules                | 0/0            | Not started | -          |
| 168. Actual Pinned-Knots Differential Qualification    | 0/0            | Not started | -          |
| 169. Differential Fork Selection and Durable Recovery  | 0/0            | Not started | -          |
| 170. Canonical Prune and Retained BASIC Integration    | 0/0            | Not started | -          |
| 171. Classified Historical Contexts and Parity Closure | 0/0            | Not started | -          |

Coverage: 30/30 active requirements mapped exactly once; no orphaned or duplicate owners. Later integration phases exercise foundations without reassigning their ownership. Phase 164 contains 13 accepted plans/summaries, Phase 165 contains 41 and Phase 166 contains 16, with their canonical verification records. Phases 167–171 await subsequent workflows.

## Historical Milestones and Guidance

Latest shipped v2.5 controls: [roadmap](milestones/v2.5-ROADMAP.md), [requirements](milestones/v2.5-REQUIREMENTS.md), [audit](milestones/v2.5-MILESTONE-AUDIT.md). [MILESTONES.md](MILESTONES.md) retains earlier milestone records. Historical `.planning/phases/` directories remain tracked because native verifiers consume selected evidence.

This roadmap follows repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, the placeholder-only `standards-overrides.md`, `standards/index.md` and the architecture, verification and testing standards. Pure domain rules remain separate from storage/process effects; actual receipts and offline replay remain distinct. Repo-local Cargo/Bazel forms are required in subsequent operator UAT guidance. Focused phase research is required for the compatibility seam, work carriers, bootstrap crash ordering, historical rule inventory, oracle provenance, ties/recovery, canonical prune corpus and qualification invalidation.

Phases 164, 165 and 167 use Bitcoin `header` as domain terminology, so the lexical match receives `UI hint: no` and does not trigger UI-design work. Phase 170 includes the existing terminal dashboard/CLI status consumers and receives `UI hint: yes`. The accepted headless scope takes precedence over generic keyword routing; GUI/frontend feature expansion remains deferred.

---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "08"
subsystem: testing
tags: [fjall, publication-faults, immutable-corruption, readiness, normal-maturity]
requires:
  - phase: 162-07
    provides: Guarded actual accepted-prefix adapter and closed namespace observations
  - phase: 162-14
    provides: Selected trusted-header prefix persistence before coins effects
provides:
  - Eight actual undo/coins/metadata/atomic index publication failure boundaries
  - Six persisted immutable ancestry corruption cases through immediate/final reads and closed recovery
  - Legitimate unknown/Missing negative control without read poison
affects: [162-09, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Actual accepted active/stale immutable union, earned readiness completions, exact allowed startup effects]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
key-decisions:
  - Preserve failed-write pre-open facts separately from legitimate configured startup catch-up.
  - Derive expected active and retained stale row union from real source/undo rather than assuming rollback after commit.
  - Corrupt checkpoint ancestry must refuse decoding while persisted scalar identity and bytes remain unchanged.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T22:20:13Z
duration: 8 min recorded receipt window
completed: 2026-10-11
---

# Phase 162 Plan 08: Publication and Corruption Fault Summary

**Actual accepted Fjall histories prove eight concrete failed publication boundaries and six corrupt ancestry read/reopen refusals without fabricated safe progress or successful acknowledgement.**

## Performance and Ownership

- Tasks: 2/2. Exactly three assigned Rust paths changed; two are new test children. The existing parent gains only module declarations. No production source, API, visibility, dependency, shared recipe or Plan07 child changed.
- First recorded compile receipt starts 2026-10-11T22:12:25.778Z; final frozen corruption receipt ends 22:19:52.994Z. Mandatory context/source inspection preceded the first receipt. Summary clock is 22:20:13Z.
- Material guidance: AGENTS, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, active lessons, GSD executor/execute-plan reasoning/template, STATE, checked Plan08, phase context/research/UI contract and preceding 06/07/14 handoffs.
- Both active lesson inputs were read completely: 7,188 bytes and 2,397 conservative estimated tokens. No archives were loaded; parent-confirmed audit baseline has no new trigger. No project skills exist in the prescribed directories.
- Parent supplied synchronized main baseline a77edab276a9daa086055aa61e6143ce86bc1218 and materialized pinned Knots. Read-only GSD init confirms 16 plans, eleven prior summaries and five incomplete plans. Chain auto mode remains active.
- Root owns STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, independent source/security reviews, full native/formal gates and Git. No staging, commits, branches, pushes, hook bypass or requirement activation occurred.

## Genuine History and Observation Boundary

Every case uses `phase162-normal-maturity-1`, actual runtime parameters and normal maturity100. Ordinary accepted h0..101 establishes genuine settled B/meta/safe101. A genuine alternate h102 is accepted and indexed; its actual managed-snapshot undo is persisted with ordinary save_undo so the existing reorg preflight has its required mate. This does not change coins or metadata authority. Ordinary shallow reorg accepts shared-recipe replacement h102..104 and retains the displaced immutable hash. Final active positions number105; one distinct accepted stale row remains.

These are focused genuine histories without pressure rows or a copied pressure database. No MemoryOnly backend, seeded UTXO, maturity override, fabricated read barrier or caller-created safe authority is used. Source-derived expected Rust durability rows validate state preservation/retry; they are **not** an independent protocol oracle. Plan06 owns independent BASIC/client parity on the complete long lifecycle.

Live observations include actual coins B, independently decoded active metadata, owner, saved checkpoint/protection, accepted processed/safe facts, exact immutable active/stale bytes, all physical active body/undo mates and private pending-intent absence. Closed snapshots compare every block_index/chainstate/coins key/value row. Cleanup guards are declared before owners; readiness handles and network clones are explicitly dropped before configured reopen.

## Task 1: Eight Concrete Publication Boundaries

| Actual fault     | Failed-turn durable coins / metadata                 | Actual failed-turn BASIC facts                                                                                       | Actual closed outcome                                                                         |
| ---------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| BeforeUndo       | B101 / meta101; replacement undo not written         | Active105 + stale1 immutable rows, safe101, protection102 retained                                                   | Old real authority recovers; ahead hash rows remain inactive; all observed bytes stable       |
| BeforeCoins      | B101 / meta101; real undo through104 already written | Same exact immutable prefix/safe/protection                                                                          | Same truthful old-prefix recovery; completed undo and header prefix retained                  |
| BeforeChainMeta  | Actual B104 / old meta101; undo through104           | Same immutable rows, safe101/protection102                                                                           | Two typed incompatibility refusals; no B rewrite or pretend repair                            |
| BeforeRecords    | Settled B/meta104                                    | Active102 + stale1 rows, safe101/protection102; no partial durable batch                                             | Normal configured startup retries real three-block lag to safe104; exact allowed effects only |
| BeforeCheckpoint | Settled B/meta104                                    | Same no-partial-batch facts                                                                                          | Same actual-source startup retry and second stable cold open                                  |
| BeforeProtection | Settled B/meta104                                    | Same no-partial-batch facts                                                                                          | Same actual-source startup retry and second stable cold open                                  |
| BeforeCommit     | Settled B/meta104                                    | Same no-partial-batch facts                                                                                          | Same actual-source startup retry and second stable cold open                                  |
| AfterCommit      | Settled B/meta104                                    | Full active105 + stale1 batch durable, saved safe104/protection105, but no success receipt or memory acknowledgement | Reopen derives committed truth with no new effects; second cold open stable                   |

Each selector discovers one test and reaches its named real writer boundary immediately after test-only injection. The ordinary Always flush or ordinary driver returns the public typed LifecycleEffect error category. Poison then refuses another index turn, a stale-row query, private disable/release and operator reserved-lock clearing. Failed atomic turns leave in-memory processed/safe acknowledgement unchanged even when AfterCommit saved the full batch.

Before-index-commit faults compare the complete live namespaces byte-for-byte before any recovery. Configured startup may then legitimately catch up against actual settled B/meta104. Expected immutable union starts with every saved active/stale row and adds source/actual-undo-derived candidates by hash; no fixed total substitutes for set equality. Only the exact replacement record/projection keys and existing state/lock keys may change. Every old active/stale byte, owner, body, undo, metadata and coins byte remains; a second cold open is byte-stable. AfterCommit must already equal that union and requires no startup writes.

The selected Plan14 header prefix is explicitly observed independently of coins. BeforeUndo preserves the old paired header snapshots. BeforeCoins/BeforeChainMeta have actually saved both active and stale admitted header knowledge before their failure; the paired snapshots agree and survive cold inspection. Those header/undo effects never mint coins or BASIC safe authority.

## Task 2: Corruption, Real Final Reads and Negative Control

| Persisted damage                | Immediate-first case               | Final-read-first case                             | Closed configured recovery                                     |
| ------------------------------- | ---------------------------------- | ------------------------------------------------- | -------------------------------------------------------------- |
| Corrupt actual target104        | Typed BlockIndex corruption/Repair | Same typed failure on genuinely earned final read | Two refusals, every persisted damaged/unrelated byte unchanged |
| Corrupt required predecessor103 | Same                               | Same                                              | Same two unchanged refusals                                    |
| Remove required predecessor103  | Same; never successful Missing     | Same; never successful empty reply                | Same two unchanged refusals                                    |

Two real requests capture pending frontiers while the synchronized owner genuinely lags. Ordinary publication earns both opaque readiness completions before precise tampering through the existing private storage seam. Each damage/order case fails the first actual caller with typed corruption, rejects the other stale completion and subsequent query, and leaves corruption unmodified. Exact namespace comparison proves only the intended immutable row was damaged; all coins/meta/undo bytes and unrelated valid h0..101 plus stale rows remain unchanged.

Failure projection is fixed IntegrityOrInvalidFacts/Failed. Earned initial synchronization, configured indexing, accepted height and protection retain their captured scalar truth; processed, safe, lag, retention and retained-availability projections become Unavailable. Repeated bounded status captures remain identical, redacted and nonmutating. Raw backend keys, errors, scripts and identifiers remain test data; exported evidence uses categories, counts, heights and booleans only.

Saved checkpoint bytes/scalar state remain unchanged, but resolving its corrupted endpoint ancestry correctly returns corruption. The tests explicitly require that refusal, including cold inspection, rather than publishing a healthy decoded checkpoint. Configured reopen refuses the persisted corruption twice without clearing faults or repairing bytes. All active physical mates and real B/meta104 survive.

The seventh selector distinguishes unknown block identity (UnknownBlock) from ordinary absent known target (Missing). Both immediate and genuinely earned final Missing preserve ScriptsValid provenance and initial-synchronization truth without read poison or status mutation. Its dangling saved projection still cannot earn a fresh recovered epoch: configured cold open refuses unchanged persisted bytes. This does not reinterpret required predecessor loss as absence.

## Verification, Initial Failures and Checked Replan

Every Cargo command ran serially through pinned Bun1.3.9 PATH and the cooperative timing wrapper with Rust1.94.1. Tests use `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features SELECTOR -- --test-threads=1 --nocapture`. No long584-row pressure corpus or unrelated broad runtime suite was rerun.

| Timing key                                       | Actual result                                                              | Wrapper ms |
| ------------------------------------------------ | -------------------------------------------------------------------------- | ---------- |
| phase162-publication-corruption-compile          | No-run compile passed                                                      | 9,444      |
| phase162-publication-faults                      | 0 passed /8 failed; setup lacked required genuine stale undo;16.79s        | 17,090     |
| phase162-publication-fault-aftercommit-preflight | Exact selector1 passed;2.77s                                               | 9,994      |
| phase162-publication-faults-final                | 4 passed /4 failed; incorrect no-startup-catch-up expectation;22.36s       | 22,650     |
| phase162-publication-precommit-replan            | Exact checkpoint selector1 passed;3.08s                                    | 10,472     |
| phase162-publication-faults-verified             | 8 passed;24.13s                                                            | 24,413     |
| phase162-corruption                              | 1 passed /6 failed; incorrect healthy checkpoint-decode expectation;18.19s | 18,592     |
| phase162-corruption-verified                     | 7 passed;20.91s                                                            | 28,684     |
| phase162-publication-final-source                | Header/union strengthening:8 passed;23.25s                                 | 31,451     |
| phase162-publication-corruption-clippy           | Node all-targets/all-features with -D warnings passed;5.55s                | 5,675      |
| phase162-publication-faults-frozen               | Final frozen8 passed;22.84s                                                | 28,629     |
| phase162-corruption-frozen                       | Final frozen7 passed;20.48s                                                | 20,642     |

Initial failures were owned fixture/expectation errors, not production bugs or deliberate semantic RED of missing functionality. After the second publication attempt, the executor reported a concrete checked replan and parent acknowledged it: retain pre-open failed-write facts, derive exact source/undo active+stale union, whitelist only actual startup publication effects, then exact precommit selector, final matrices and strict lint. No blind rerun, waiver, ownership expansion or acceptance weakening occurred. The corruption correction requires stronger fail-closed checkpoint behavior while retaining exact persisted scalar/byte evidence.

Exact final receipts:

- Publication: `.local/open-bitcoin-dev/command-timings/phase162-publication-faults-frozen/2026-10-11T22-18-51.509Z-e61012de-369e-4e3f-9e74-f066d9cf15a3.json`.
- Corruption: `.local/open-bitcoin-dev/command-timings/phase162-corruption-frozen/2026-10-11T22-19-32.352Z-ad749016-e6d6-425c-9d60-bb60e9692491.json`.
- Strict lint: `.local/open-bitcoin-dev/command-timings/phase162-publication-corruption-clippy/2026-10-11T22-18-05.065Z-eb66f1f9-dfc6-4020-9e4d-801fc481f9b3.json`.

Scoped rustfmt passes. Parent-registered breadcrumbs verify1,139 Rust files. Managed all checks report0 findings. Phase158 validated-reorg, Phase159 RPC, Phase160 peer and Phase161 operator claim/source guards pass. Owned-source whitespace/stub scans and source/diff review pass. The summary passes scoped mdformat with installed GFM/frontmatter extensions after check-first normalization. Clippy provides compiled diagnostics; no semantic-navigation diagnostics tool was available. Resumable sessions were polled normally; no quiet yield was treated as timeout.

## Simplification, Source Handoff and Cleanup

The explicit simplification pass retains one small genuine active/stale setup, shared raw/mate observations, a source-derived candidate helper and named closed retry/header checks. Faults and corruption remain separate children. No generic fault framework, production seam, dependency or suppression is needed. Functions stay below the approximate161-line trigger; owned files are426/569/373 lines, below628. Parent module changes only declare the children.

Final matrices cover15 distinct guarded datadirs, excluding prior runs. Cleanup reopens the physical store only after owners close, then removes the datadir and asserts path absence. Final TMPDIR and/tmp inventory after all test processes exited found zero focused-fault directories. An inventory taken while the last case was still running showed its live directory; the final post-exit inventory is empty. Cargo is idle.

| Owned Rust file                                                                       | SHA256                                                           |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs                    | 623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580 |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs | a0d01c2dd4f6e4d61fd004e8c01aaf2fc6890f0a2a8d5d1e075636f72dbe108e |
| packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs         | 6ba3fc4cce7df60cdd0eefd9191250848fc02b5feff4a86e1d14cf100547941e |

Plan07 missing_history/recovery hashes still equal43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a and9deca92df80b1259c1dd7d04a592fd31058af6795127cb77315318a364d210b7. Both actual new child paths were immediately reported and parent-registered. Their exact ordered anchors are packages/bitcoin-knots/src/index/base.cpp; packages/bitcoin-knots/src/index/blockfilterindex.cpp; packages/bitcoin-knots/src/node/blockstorage.cpp.

Parent parity proposal: add this summary, both final selectors and final receipts alongside retained-filter durability/recovery evidence. Preserve distinctions between failed-write pre-open facts, actual committed AfterCommit state, legitimate configured startup retry, required ancestry corruption and ordinary Missing. No new production behavior difference or requirement owner is created.

## Declared Mitigations and Remaining Gates

| Threat     | Concrete executed evidence                                                                                                                                                                |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-08-1 | All eight actual effect boundaries, independent B/meta observations, exact no-partial precommit batches, truthful committed-after-error facts and incompatibility refusal without rewrite |
| T-162-08-2 | Three precise damages in both actual caller orders, genuinely earned stale completion revocation, repeated typed closed refusal and separate legitimate absence control                   |
| T-162-08-3 | Fixed redacted IntegrityOrInvalidFacts/Failed projection, bounded unavailable fields, preserved safe scalar facts and nonmutating captures                                                |

No blocking stub, unwired source, new production endpoint/auth/schema/dependency or unmodeled production trust surface was added. No authentication gate occurred. Independent source/security review and unresolved-High disposition, full default native verification, formal lifecycle checks and evidence-gated requirement activation remain parent gates; no ASVS certification is claimed.

Task1, Task2 and metadata commits are explicitly deferred by D-14. No shared state/roadmap/requirement/parity/LOC/Git update was performed by this executor. Ready for checked Plan09 actual successful/partial transport receipt proof.

Synthetic root, caller-supplied branch work and inherited Open Bitcoin210000 versus Knots Regtest150 halving remain explicit limits. These tests prove scoped accepted first-party histories and durable/query behavior, not whole-chain Knots consensus differential or most-work selection. BIP37/V0, assumeutxo/dual-chainstate, archive scale, public defaults/public-network CI, implicit repair/import/download, wallet acceleration, production-service/readiness/funds and milestone audit/archive/shipment remain excluded. Phase161 schemas/presentation/redaction and v2.4 advisories are unchanged.

## Self-Check: PASSED

All three owned Rust paths and this summary exist. Frozen source hashes match the final8/8 and7/7 runs and strict lint. Final timing receipts record exit0; initial failed receipts record exit101. Plan07 child hashes remain unchanged. Breadcrumb/managed/claim/rustfmt/whitespace checks pass. The unchanged baseline, required lifecycle identity, empty requirements-completed and exactly two top frontmatter delimiters are verified. There are no task commits to check because D-14 explicitly defers them to root. Final temporary-datadir inventory is empty and all owned Cargo sessions have exited. Parent shared lifecycle/registry/Git responsibilities remain pending.

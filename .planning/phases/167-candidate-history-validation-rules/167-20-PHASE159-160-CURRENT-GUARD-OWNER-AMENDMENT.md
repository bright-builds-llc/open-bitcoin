---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T08:04:07Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 Phase 159/160 current source and provenance guards

After Phase157's exact-argv suite passed, source-stable direct preflight found one Phase159 and three Phase160 findings before another full verifier attempt. Phase159's metadata row still selects `coins.rs:save_validated_chain_meta`, while the accepted wrapper delegates to the full guarded `chain_meta.rs:save_validated_chain_meta_with_counts`. Phase160's three source/action rows still expect old direct calls in `runtime_authority.rs`, the BASIC serving wrapper and the sync session; accepted source moved the receive owner into `peer_sessions.rs` and split serving and session work through real locked/delegated helpers and `PeerRequests`. The reviewed current product Rust behavior remains intact. These are current guard drifts, not authorization to restore old calls or alter production.

ROOT adds exactly **five present source files** to Plan20 Task1 ownership: `scripts/check-phase159-filter-rpcs/contracts.ts`, `scripts/check-phase159-filter-rpcs.test.ts`, `scripts/check-phase160-peer-filter-serving.ts`, `scripts/check-phase164-canonical-identity/contracts.ts`, and `scripts/check-phase165-shared-exact-work/contracts.ts`. The already-owned Phase164 `test-modules.ts` covers the two Phase159 whole-file SHA rows. Plan20 now has **15 unique direct owners**, Task1 **12** and Task2 **three**; of the 1,828 accepted product paths, **1,818 remain protected**. LOC is a separate generated owner and four reports are truly absent. Before any new owner changes, close jobs, capture its complete original/live bytes and the provisional LOC in a 15-owner stage frame, issue a supplementary ROOT release and obtain independent review. The earlier Phase159/160 direct findings and complete default attempts retain their raw diagnostic status, with no final-credit inference.

For Phase159, add the real `chain_meta.rs` to fixed source inventory, retarget D-14's full guard to `save_validated_chain_meta_with_counts` with filter-publication, validation-coverage, invalidate, fault and coins-view order plus counts-aware persistence, and add a wrapper delegate row requiring the real full-writer call. Retarget only the existing `raw flush sink coverage laundering` test mutation from old `coins.rs` to the full writer in `chain_meta.rs`; retain the separate raw-seed coins mutation and all other 624-line test behavior. Renew only the two corresponding current Phase159 contracts/test SHAs in already-owned Phase164 `test-modules.ts`.

For Phase160, bind the actual `peer_sessions.rs` receive→BASIC intent owner, serving wrapper→locked prepare/resize/bind/lease order, and session wrapper→run/process→PeerRequests dispatch/handoff→filter-plan send order. Include real continuation and module registration edges; use existing dynamic per-contract/per-edge tests to reject removed/reordered/disconnected calls. Add a strict exact Phase160 checker final-current→prior-current descriptor to Phase164 `FILTER_GUARD_INVERSES`, preserving the frozen Phase163 provenance digest through existing `historicalToolingView`. The checker starts with the required shebang for existing generic inverse mutation tests. Renew only Phase165 `CURRENT_TOOLING_ROWS` current SHA for Phase164 contracts; keep its prior SHA and Phase164 `test-modules.ts` historical Phase164-contracts prior SHA unchanged. Do not edit historical manifests, snapshots, reports or Phase163 code.

Run focused and full Phase159/160 checker/test suites, Phase161/162/163 provenance/native controls, Phase164/165 current and historical inverse suites and other affected guards on one frozen source frame. Regenerate provisional worktree LOC, perform fresh locked coverage clean and rerun the complete default `bash scripts/verify.sh`. Preserve source before/after, discovery, raw failures and liveness. Independent final source/security/goal review and ROOT staged-index LOC/hook reconciliation remain mandatory. No production Rust, verifier, unrelated source, target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.

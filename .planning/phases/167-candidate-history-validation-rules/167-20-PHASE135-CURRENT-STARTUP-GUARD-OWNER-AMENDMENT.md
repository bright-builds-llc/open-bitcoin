---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T14:50:08Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 Phase 135 current startup guard boundary

After the checked Phase127/128 focused suites passed, a source-stable preflight of every remaining default `bun run scripts/check-*.ts` command found only the Phase135 snapshot recovery checker red. It reports `P135 daemon: recovery and private 300-second checkpointing must precede publication`. The exact pre-edit Phase135 suite discovered 108 tests: 77 passed and 31 failed. Three clean/synthetic positives fail from the same stale startup assumption; 28 negative mutations acquire that baseline P135 finding in addition to their intended result and have no clean adversarial credit. The accepted ordinary RPC constructor now checks selected chain, held-network owner and ordinary scope before returning typed `UnprovedOwner`; it no longer recovers and publishes a general ordinary context. The separate fixture constructor still recovers before assembling a fixture context. The private daemon's 300-second checkpoint worker remains conditional, and the bounded canonical-history route uses a prebuilt context without the old mempool recovery helper. This is a stale current guard, not proof of canonical-history mempool recovery or public checkpoint behavior.

ROOT adds exactly two present files to Plan20 Task1: `scripts/check-phase135-snapshot-recovery/source.ts` and `scripts/check-phase135-snapshot-recovery.test.ts`. Plan20 now has **29 unique direct owners**, Task1 **26** and Task2 **three**. Of the 1,828 accepted product-source paths, **1,804 remain protected**; LOC remains a separate generated owner and four reports are truly absent. Before editing either new file, close jobs, capture its exact original/live whole body and current provisional LOC in a 29-owner stage frame, issue a supplementary ROOT source release and obtain independent review. The direct RED, 77/31 suite and earlier default attempts retain diagnostic status only.

Renew only `contextStartupIsOrdered` and its current-route mutation controls: require the real ordinary selected-chain/owner/scope typed refusal without recovery/publication, while preserving fixture recovery-before-assemble, canonical support-before-effects, private checkpoint worker ordering, shutdown and all historical snapshot controls. The existing Phase135 checker entrypoint remains protected; no Phase164/165 body-hash row or Phase163 inverse is renewed. The test file has only 13 lines of normal-formatted headroom (615/628), so refit existing rows or use a compact in-owner typed-refusal mutation; stop for a checked split if an extra owner is genuinely needed. Do not add dead old-route calls or infer that the Phase167 canonical-history route proves general mempool recovery.

Run the genuine clean-current and synthetic positives to GREEN, retain all 28 negative mutation meanings, and complete the exact Phase135 suite/direct checker plus adjacent Phase134/current guard controls. Then regenerate provisional worktree LOC, perform fresh locked coverage clean and rerun complete default `bash scripts/verify.sh` on a frozen source frame. Preserve raw attempts, discovery, source before/after and liveness. Independent source/security/goal reports and ROOT staged-index LOC/hook reconciliation remain mandatory. No Rust production, unrelated guard, historical report, target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.

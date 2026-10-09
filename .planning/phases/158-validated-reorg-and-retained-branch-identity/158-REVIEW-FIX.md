---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "07"
finding: WR-158-01
recorded: 2026-10-09T03:50:53Z
status: focused-repair-verified
requirements-addressed: [CFIX-03]
requirements-completed: []
git-finalization: deferred-to-root
---

# Phase 158 WR-158-01 Review Fix

The existing authenticated displaced-fence binding now permits a genuine second reorg after a shorter accepted-unflushed replacement. The processed-frontier guard remains independent and unchanged.

## Authorized scope and guidance

This is the root-authorized Plan 07 review-fix amendment, consolidating the identical finding from `158-REVIEW-STORAGE.md` and `158-REVIEW-RUNTIME.md`. Material guidance: AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification/Rust standards. Both active lesson inputs were read completely (7,188 bytes, 2,397 conservative estimated tokens); archives were excluded. GSD execution context was initialized for Phase 158. No project skill directories were present.

Only these Rust files changed for this amendment:

- `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs`
- `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs`

No fixture helper, dependency, production authority constructor, public interface, schema, endpoint or additional trust boundary was introduced. No stage, commit, push, STATE, ROADMAP, requirement activation or todo update was performed by this worker.

## Failure reproduced before the fix

The new production test executes genuine A23 → B14, completes ordinary catch-up, and then attempts B14 → A23 through the existing serialized network reorg caller. No own coins flush occurs between these transitions. Actual coins remain at A23; the shared safe prefix remains ancestor 10 during B acceptance.

Required B undo records are explicitly retained from the genuine accepted B snapshot with `save_undo(..., PersistMode::Sync)` before the return. This is a source-retention prerequisite, not coins or chain-metadata publication. Without that prerequisite the repaired guard correctly reaches the separate `missing BASIC required undo` refusal. That independent required-source check was preserved.

The fully retained regression was run with the original unconditional guard:

```bash
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-wr01-red-retained-sources -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase158_validated_reorg_shorter_unflushed_return_preserves_coins_fence_and_reopens -- --nocapture
```

Actual RED: exit 101; 0 passed, 1 failed, 0 ignored, 1,355 filtered. Failure at the production network reorg call in `fixtures.rs:223`:

```text
Operation(Chainstate(CoinsStorage { detail: "storage corruption in block_index: BASIC displaced endpoint behind progress; Run the storage repair flow before restarting normal operation." }))
```

An earlier initial regression run also reproduced that precise guard failure (0 passed / 1 failed / 0 ignored, 1,352 filtered), before the three new negative controls were present. The later fully retained RED above is the definitive isolated evidence.

## Narrow repair and assertions

After the existing same-store, live revision/generation/checkpoint, processed identity and actual-coins authentication, the durable-height guard now rejects a taller checkpoint unless `proof.durable_displaced()` succeeds. That existing method compares the achieved marker with the exact `(durable_height, durable_hash)` tuple. The independent `processed.height() > old.height` rejection, genuine stage/acceptance, checked generation/revision, common-ancestor identity, guarded publication and next-marker recomputation are unchanged.

The return regression asserts safe and processed height 10 plus conservative protection immediately after the second rewind. Ordinary catch-up then verifies the exact original A commitments and may earn safe height 23 against the already durable A23 coins checkpoint; this is the existing recovery/fence behavior. It checks unchanged actual coins, exact original active records, retained immutable B rows, subsequent ordinary own flush, full runtime/store-handle drop, configured reopen, exact A records/checkpoint and retained B rows after reopening.

Three negative tests start from genuinely achieved A2 → B1 authority, then use test-local corruption to remove or mismatch only the earned displaced tuple in the existing proof/control. They assert the other guarded proof checks still pass and that reorg preparation specifically refuses `BASIC displaced endpoint behind progress` before effects. No test helper mints a matching tuple or new accepted receipt.

Regression names:

- `phase158_validated_reorg_shorter_unflushed_return_preserves_coins_fence_and_reopens`
- `phase158_storage_reorg_taller_fence_without_displaced_marker_refuses`
- `phase158_storage_reorg_taller_fence_with_wrong_displaced_height_refuses`
- `phase158_storage_reorg_taller_fence_with_wrong_displaced_hash_refuses`

## Executed verification

All Cargo work ran serially through the checkout lock/timing runner with Bun 1.3.9 on PATH and the pinned Rust toolchain.

| Check | Actual result |
| --- | --- |
| `phase158-wr01-format`: Cargo fmt, full workspace | Exit 0 |
| `phase158-wr01-green`: production return regression | Exit 0; 1 passed / 0 failed / 0 ignored; 1,355 filtered |
| `phase158-wr01-negative`: `phase158_storage_reorg_taller_fence` | Exit 0; 3 passed / 0 failed / 0 ignored; 1,353 filtered |
| `phase158-wr01-all-phase-tests`: node `--lib phase158_ -- --nocapture` | Exit 0; 102 passed / 0 failed / 1 ignored / 1,253 filtered; 203.89 seconds test runtime |
| `phase158-wr01-clippy`: node `--all-targets --all-features -- -D warnings` | Exit 0; 4.96 seconds Cargo runtime |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | Exit 0; 997 Rust files verified |
| `git diff --check` | Exit 0 |

Exact local timing receipts (durations include Cargo/runner overhead):

- RED: `.local/open-bitcoin-dev/command-timings/phase158-wr01-red-retained-sources/2026-10-09T03-49-10.098Z-415d8569-da2d-4efc-b91d-25d332747457.json`, 7,425 ms, exit 101.
- Final focused GREEN: `.local/open-bitcoin-dev/command-timings/phase158-wr01-green/2026-10-09T03-50-06.643Z-ae5acc4c-548f-4c5b-ac1a-13fc39a07ff0.json`, 9,813 ms, exit 0.
- Negative controls: `.local/open-bitcoin-dev/command-timings/phase158-wr01-negative/2026-10-09T03-50-24.135Z-5644ceac-df8a-4479-bd69-59fcd6491f9f.json`, 2,552 ms, exit 0.
- All Phase 158 node tests: `.local/open-bitcoin-dev/command-timings/phase158-wr01-all-phase-tests/2026-10-09T03-50-33.662Z-ca3e4389-f70e-478c-bdfd-a332c9e744cd.json`, 204,170 ms, exit 0.
- Node Clippy: `.local/open-bitcoin-dev/command-timings/phase158-wr01-clippy/2026-10-09T03-54-12.740Z-19304702-ed2d-4a9b-9f59-95eab03ec432.json`, 5,208 ms, exit 0.

Test refinement also exposed the independent missing-B-undo prerequisite and an overly restrictive expectation that safe height remain 10 after exact A history had caught up to actual durable A23. The final test asserts 10 at rewind and 23 after ordinary catch-up. Neither observation required another production change. An earlier failed run shares the GREEN timing key; the exact successful receipt above disambiguates it.

The sole ignored test is the pre-existing explicit timing experiment; all required behavior tests executed. During the automatic-prune test's longer runtime the runner continued heartbeats, and process evidence showed the test executable in running state with substantial CPU activity; no command was interrupted. Full default native coverage/build/Bazel verification, independent changed-hash re-review, security consolidation and formal lifecycle verification remain root-owned gates; this focused repair is not phase completion.

## Simplification and limits

The repair reuses the existing exact fence predicate rather than introducing a second ancestry rule or capability path. Production code changes only one conditional and its explanation. The test helper shares three corruption-only negative setups; both test functions and files remain below the managed function/file refactor thresholds. No blanket claim on unavailable sources or unrestricted retained-fork depth follows from the regression. There are no introduced stubs or new security-relevant surfaces.

## Source fingerprints

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | `f85e0da039b66b24669b3a085cca2d5c70889d5d28ef5b0098439f37b581e715` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | `a9bcf1d5e3de9e719f29cbff8e6fac29511dc3571c1326b52cd880497e23654e` |

## Self-Check: PASSED

Both changed Rust files and this evidence file exist. The source fingerprints above match the tested final bytes. The files contain no introduced TODO/FIXME/placeholder stub; `git diff --check` and the 997-file breadcrumb check passed. The new tests are non-ignored and actually executed. All verification sessions completed before returning serial Cargo ownership to root. Source is frozen at these hashes; independent source/security reviewers must use this repaired snapshot. Git finalization remains deferred as explicitly required by Plan 07.

---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T16:04:51Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 uncached accepted genesis Duplicate regression

The restarted Plan 51 final command 11, the broad `network::runtime_authority::validation_history` selector, exited 101 after 45 named passes and four old failures. Every failure stopped at its **first actual accepted genesis/root Duplicate** with support stage `retained body eligibility unavailable`. Final commands 1–10 passed; 12–20 were not run. This is a product regression, not setup or an invented new RED. The exact failed command, nine-key probe, raw streams, source pre/post hashes and frozen frame remain diagnostic evidence; no pass-only retry is authorized.

Independent read-only diagnosis traced the failure to E1 `finish_bookkeeping`: after the E4 command loads the actual same-store retained body, E1 requires `blocks_by_hash[hash] == body` before request cleanup. Fresh canonical genesis construction and canonical-history reopen prove and link the root body in HeaderStore and durable storage but intentionally leave the root absent from the volatile `blocks_by_hash` cache. The new E1 cache condition therefore rejects legitimate root Duplicate, and E4 propagates it as support unavailable. The four old tests fail before their later descendant assertions. The E3 failed-ancestor guard and cached descendant Duplicate controls still pass and must remain protected.

ROOT permits a **root-only read-only qualification** in the already-owned `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` at the exact cache-equality conjunct. Permit an uncached body only when all of the following are derived from held owners: a selected canonical chain exists; the block equals that chain specification's **entire** proved genesis block and hash; the accepted active chain contains that exact height-zero root; and the existing current header/hash/work, body binding/epoch, available/not-failed, and managed `eligible_body_candidates` full-linkage checks all pass. Every non-root body still requires exact cache equality. Do not insert or seed a volatile cache entry, use height/hash alone, trust a caller witness/flag, widen a public authority API, edit HeaderStore/storage, or bypass the failed-ancestor eligibility projection. E4's prior same-held-store load and genuine StorageError/source versus None/no-source behavior remain unchanged.

The already compiled old named `phase167_local_submission_factories_duplicate_root_and_real_first_acceptance` failure is the focused regression RED. After the one-owner fix, rerun that exact named control (both Disabled and Enabled modes), the other three old root/reopen/witness Duplicate controls, E4 cached-descendant stored-body/witness positive and corrupt/missing error controls, and E3 failed-ancestor request-conservation test with nonzero discovery and no assertion weakening. Any later descendant Duplicate failure or lost negative control requires renewed diagnosis; do not claim all four fixed from only the first root pass.

Before editing, ROOT must capture the current complete E1 owner whole body/hash, verify the frozen 502-source/201-native/Byteview1/five-Knots frame and 489 protected source paths, confirm all owned jobs closed and issue an exact one-owner correction release with independent review. Then normal rustfmt/strict lint and the whole original-before BSD `patch -F0` inverse/forward must restore the ten original bodies and three true absences. Recapture a new complete final frame; rerun **all twenty** published final commands fresh and serially from format after independent source re-review. The earlier format failure and command-11 regression remain diagnostic only. No new owner, target/cache exception, Git action, global CVLD closure or Phase 168 work is granted.

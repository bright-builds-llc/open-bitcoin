---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T13:56:27Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 final format registrar order correction

The first Plan 51 final command, the published `cargo fmt --check`, exited 1 on one normal rustfmt module-order difference in the already-owned `packages/open-bitcoin-node/src/network/tests.rs`: `mod compact_stored_body_cases;` must follow `mod compact_receive_cases;`. The raw diff names no other file. This is a formatting failure, not a behavioral RED; the other nineteen final commands were not launched. The failed format run and its exact nine-key probe/source pre/post hashes remain diagnostic evidence.

ROOT grants a separate exact one-owner correction after the owned command is closed and the complete frozen source/native/Knots frame is rechecked. Move only that module declaration to rustfmt order, preserve every test registration and body, run normal format check and the already passing four task selectors as appropriate, then recapture a fresh complete final frame. The original-before whole Plan 51 BSD `patch -F0` inverse/forward must still restore the ten original owner bodies and three true absences, with 489 protected first-party source paths and all 201 native, Byteview1 and five pinned Knots anchors unchanged.

After the corrected frozen frame passes independent source recheck, rerun **all twenty** published final commands fresh and serially from format. The first failed format attempt supplies no final matrix credit; no pass-only retry, altered argv, target/cache override, Git action, global CVLD closure or Phase 168 work is granted.

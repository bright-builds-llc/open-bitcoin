---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T00:52:21Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 pinned Bun verification and raw checker evidence

The repo pins Bun `1.3.9` in `.bun-version`. The default user `~/.bun/bin/bun` currently resolves to `1.4.2`; the Phase162 focused 2,632-test GREEN stderr and some earlier Plan17 Bun runs show that unpinned version. The bundled `/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin/bun` reports exactly `1.3.9`. The saved unpinned tests remain truthful execution/diagnostic evidence for those bytes, but they do **not** satisfy the repo's pinned-runtime final gate. A Phase163 native test was already running under 1.4.2 when this mismatch was found; let it close naturally and classify it likewise. Do not kill an owned or unrelated process solely to switch versions.

After all owned Bun jobs close, create one explicit immutable PATH projection placing the pinned 1.3.9 bin first and record an actual child `bun --version` probe before each focused/final application or a shared launcher that proves the identical projection per command. Keep the four published Plan17 argv strings exactly `bun run ...` / `bun test ...`; only the environment PATH selection changes to the repo-pinned runtime. Capture raw argv, stdout/stderr, timing, exit, named discovery, ignored reasons and complete source pre/post hashes. No pass-only retry or zero-discovery credit.

Rerun the Phase162 checker/test, Phase163 provenance/native tests, Phase123 checker/test and Phase164/165/166 focused suites under Bun 1.3.9 on attributable source frames. The earlier standalone Phase162 checker GREEN was reported from the console but lacks saved argv/streams/timing, so it has no final credit until a fresh raw-recorded pinned run. After all guard source changes finish, freeze one complete 21-owner/1,797-protected final frame and run **all four** original Plan17 literal commands fresh under the same pinned Bun projection, with meaningful live mutation refusals after clean GREEN. Earlier mixed-time baseline, unpinned GREEN, and unsupported output-only observations remain separately retained without final gate credit.

This amendment changes no source owner, command argument, historical receipt, production Rust, verifier registration, target/cache, Git state, global CVLD acceptance or Phase 168 scope. Normal formatting, whole original-before inverse, independent CORE/source/evidence review and later full default verifier/coverage/Bazel/normal-hook main push remain required.

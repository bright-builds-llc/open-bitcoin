---
phase: 165-shared-exact-work-authority
plan: 34
reviewed: 2026-10-17T21:39:04Z
depth: deep
files_reviewed: 4
files_reviewed_list:
  - scripts/check-phase72-observability-evidence.ts
  - scripts/check-phase73-uat-verification.test.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: task1-source-before-new-ownership-digests-and-inventory-pin
phase_acceptance: false
native_qualification: false
---

# Plan 34 Task 1 independent source review

CLEAN for the exact four current files below. No actionable correctness, security, integration, or test-reliability finding was identified. The repair corrects four live validated-work source comparisons while preserving complete raw support and smoke expectations.

This review precedes derivation of new ownership identities or the ordered inventory pin. No such identity or pin was computed by this reviewer. Task 2 proposals, activation, proportional verification, and final Phase 165 acceptance remain separate gates. No source, pin, plan, shared metadata, or Git mutation was made; no Bun tests, native tests, Cargo, Bazel, or heavy matrix was run by this reviewer.

## Inputs and source identities

The active lifecycle is `165-2026-10-14T16-09-59Z`. Applicable AGENTS/Bright Builds guidance and standards were loaded. Both active lesson files were fully loaded: global 5,230 bytes and repository 1,958 bytes, total 7,188 bytes and 2,397 conservative estimated tokens; no archives.

Authenticated full-byte inputs:

| Input                                 | SHA-256                                                            |
| ------------------------------------- | ------------------------------------------------------------------ |
| Checked 165-34-PLAN.md                | `17f47519e3e30af1ea4b2eb59a226b269a86d9182a1e4a17b0edb6a1363c0975` |
| phase165-plan34-task1-before.json     | `4c5a592834a921cfb102498bf1b8421e99317dc9d8736799c4a74c13c3e791ae` |
| phase165-plan34-task1-pre-review.json | `952025f1f14c3eea06f2a97bd234c10688fcb1ca7a8f12cde4531cf89cae8939` |

All four complete current source bodies matched the actual packet:

| Source                                                | Lines | Full-file SHA-256                                                  |
| ----------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| scripts/check-phase72-observability-evidence.ts       |   551 | `46b34b0cd3e6f5c0ae326ba9fd17e5d46f980099e582224903ed6187ae582f11` |
| scripts/check-phase73-uat-verification.test.ts        |   545 | `91d61c382c08591d5cf004be63a2facf6d32dc524256eafa3af5d780f4c1d229` |
| scripts/check-phase165-shared-exact-work/contracts.ts |   618 | `64008967e4446d07b70e6514d2bdfa7e99ae44b911da2d2866992f016e854c8f` |
| scripts/check-phase165-shared-exact-work.ts           |   269 | `9adb6a82bbfe62f8624cc008000d3d0409a4e4d08079c8eb54c580250d13ac5c` |

All remain within the approved 628-line cap. The last two owners are byte-exact to their authentic before bodies. Thus their existing 158 SOURCE rows, 88 TEST owners, 277 INPUT paths, and pin `9d2fb8f19b943757a968574d6997b65350d29a8a20e4d57b37f834d468f3ae8c` remain unchanged at this review boundary.

## Semantics and complete inverses

Independent standard-library arithmetic gives `format(840005, "064x")` = `00000000000000000000000000000000000000000000000000000000000cd145`, exactly 64 hexadecimal digits. Each of the four new live tests independently computes this value from the number, asserts width 64, and compares the complete expected array using `toEqual`. None imports the implementation constant or derives expected text from checker source. RPC retains both its field-name needle and exact `json!("…")` shape.

The actual CLI human-status assertion at inbound_and_sync.rs:449, dashboard assertions at sync_and_recovery.rs:83/331, RPC assertion at sync_status.rs:412, and metrics/log assertion at summary/tests.rs:222 contain the same canonical value. Those Rust files are unchanged.

Only the four live validated-work interpolations use the new explicit constant: three text needles and one RPC JSON needle. Shared fixture `validatedWork` and `connectedWork` both remain decimal `840005`. The two new raw cases compare their complete row objects, including every available/unavailable field, paths, heights, hashes, peer values, support connected work, and smoke validated work. Actual support_sync_soak.rs:144/148/228/240 and smoke fixtures-daemon.sh:72 plus assert-report.ts:89 retain matching decimal evidence. No production representation was changed or raw authority promoted.

The matrix still has all eight rows. Every unrelated field, unavailable reason, file path, corpus traversal, and read call is exact. Its export and short JSDoc accurately describe a pure expectation builder. The existing main retains all seven verification calls in the same order, all diagnostics, exit 1, and success output. Only its final invocation gains `import.meta.main`. Static inspection of the checker and source-corpus module shows no checker verification or source-corpus function call on import. Actual import silence establishes exit 0 and zero output; it does not claim absence of Bun module-loading I/O.

Independent complete-byte inverses passed:

- Remove the one constant, reverse exactly four substitutions, reverse the JSDoc/export, and reverse the main gate: complete Phase 72 source returns to `82a238ae1320ae3f3b6ca4792742dadcd034a6f3009d59d92cad70fb3b2ee0c8`.
- Remove exactly one added import and the complete six-case suffix: complete Phase 73 source returns to `ee9a62a71d2f72c5607732e94bf2189f9028badfda8f6c37ae55e94518daa104`. All original 447 lines, eleven test bodies, helpers, assertions, and registrations remain exact.
- Both current-165 owners already equal their complete before sources.

The six appended cases use explicit Arrange/Act/Assert comments. There are no new skips, only selectors in source, mocks, relaxed equality, deleted assertions, or extra dependencies. The narrow export/gate plus one constant and four substitutions is sufficient; no generic conversion machinery is introduced.

## Authenticated actual RED and GREEN

Both receipt JSON files, every exact argv and expected/actual exit, all eight log bodies/lengths/hashes, and actual selected names/counts were authenticated. Commands use the absolute pinned Bun 1.3.9 executable. The focused argv is `bun test ./scripts/check-phase73-uat-verification.test.ts -t 'phase165 observability'`; full argv omits the selector. Import uses `bun -e 'await import("./scripts/check-phase72-observability-evidence.ts")'`; direct CLI uses `bun run scripts/check-phase72-observability-evidence.ts`.

| Actual command      | Result                              | Duration | Log SHA-256                                                        |
| ------------------- | ----------------------------------- | -------: | ------------------------------------------------------------------ |
| RED six new cases   | Exit 1; 2 pass, 4 fail, 11 filtered |    29 ms | `c25570026280294cc9f24fcc3a1b3bb359608946edfac3a7f4c9d903cf453652` |
| RED full Phase 73   | Exit 1; 13 pass, 4 fail             |   215 ms | `f7431595ba08c9774205c687a5376f99ca7a84dd2b10c3b4ca1d3eb625cf29a2` |
| RED import          | Exit 0; zero output bytes           |    14 ms | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| RED direct CLI      | Exit 1; same four work diagnostics  |   163 ms | `68e356a4f6c4533474d1c8b3547173e7f42f264f4e589ba3a4a4e62de3d200db` |
| GREEN six new cases | Exit 0; 6 pass, 0 fail, 11 filtered |    17 ms | `e8a997d5214cafe6ede8b0ed9fd1ddfd06375073bf058a9c4b13b984a52c2ad3` |
| GREEN full Phase 73 | Exit 0; 17 pass, 0 fail             |   238 ms | `dcee89e08d251cde320338ae4ccbaa5670334c64cb24af6762a6bfb4231d5443` |
| GREEN import        | Exit 0; zero output bytes           |    14 ms | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| GREEN direct CLI    | Exit 0; original success message    |   142 ms | `2c8df25ef0be7eb6aa2b7d5a5023b92041a01d167d8637b2518949290206eef2` |

The RED assertions are four actual decimal-versus-hex mismatches, with both raw cases and all eleven original tests passing. They are not compile/import/discovery failures. Focused and full runs overlap and must not be added as distinct coverage.

The authentic RED checker body hashes to `2eb22a434a6116b33567c0b6915c1f4cf13d6721b589f38a355c5b5a4e3c9fad`; the RED test source equals the reviewed final test source. RED and GREEN receipt JSON hashes are respectively `17cc93093a22e85ac8de72651858fa7b4828091646c37e678fafd9b1b0198951` and `0271a3eb081c3b7e299651aef5c40c0f09b2d53662cf120ecc52d419bd93e2a1`.

The original full-default native failure remains authentic exit 1, with repository wrapper 4,679 ms and verifier internal 4,163 ms. Its receipt SHA is `fdf2396d3f8603c144c5ef79c67020eb41df554f89f1b0ee54677371e669b914`, raw log SHA `0c021085f6a949259701fbcc27be80874e0f34a2a7ee072ed95a2dd164a416e1`, and planning-before SHA `7ecfc37a20b6bde7ee4d10741164356d997ec0a8a19982e212bcc4fd714e9cdb`. This focused repair does not turn that run green or establish later native gates.

## Read-only dependency identities and boundaries

The complete nine recorded dependency bodies matched live bytes; relevant assertions, raw fixtures, corpus behavior, and registration were inspected.

| Dependency                                                                        | Full-file SHA-256                                                  |
| --------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| packages/open-bitcoin-cli/src/operator/status/render/tests/inbound_and_sync.rs    | `dd0ff41fdac67dbc53913e4f7073a8e0b391ed836f40acedcdc66f5a8da2079d` |
| packages/open-bitcoin-cli/src/operator/dashboard/model/tests/sync_and_recovery.rs | `b9575a2780b352fcef84b172d74d416a90b328de55a285b6b0d38a13e2594ccd` |
| packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs                       | `571a8e9e5888da928eb4246f5c5f66a7657c5930f912962bda53abdde42b85e1` |
| packages/open-bitcoin-node/src/sync/types/summary/tests.rs                        | `3d01d113435af23e31f899270595261438bc192465223407bd383c96f1484018` |
| packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs              | `5ce0e414f57476d83bc27a09d5a42cead3c6ac3b0912bc6fe661e7ebac97ac05` |
| scripts/test-run-live-mainnet-smoke/fixtures-daemon.sh                            | `b928a12441df980fa7502f0dc26c93ef40af29e232a34d3fc1a7cdac213ba81d` |
| scripts/test-run-live-mainnet-smoke/assert-report.ts                              | `baf64a4406a64dee43549e9c29aabc363bb0e84f3af50efbdb9dd9f735b1bb53` |
| scripts/source-corpus.ts                                                          | `a37575853f5a0501b502882dcb898c04f429699216c1f4eb97512845845acb8f` |
| scripts/verify.sh                                                                 | `e1823a94c0394107ecdfbb194b32d93f290cc7b79195dac5583dd7cc7a811fb7` |

Task 2's two ordinary TypeScript SOURCE rows, actual input ordering, proposed pin, complete pin-file inverses, stale-pin refusal, new ownership controls, and complete read-union closure are not reviewed or accepted here. No Rust TEST ownership row is justified by these TypeScript files. Full final native, matrices, coverage, Bazel, documentation, code/security/formal review, normal hooks, and strict finalization remain root-owned gates.

This is not native qualification or phase completion. Phase 165 requirements remain CWOR01/02/03/05; ordinary canonical bootstrap/reopen and CWOR04, contextual validation, and actual Knots daemon qualification remain deferred to 166–168. Reuse this Task 1 review only for the exact identities recorded above.

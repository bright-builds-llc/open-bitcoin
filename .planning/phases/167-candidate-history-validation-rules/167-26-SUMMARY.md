---
phase: 167-candidate-history-validation-rules
plan: "26"
subsystem: historical-fixture-adapter
tags: [bun, bip30, typed-fixtures, offline, provenance]
requires:
  - phase: 167-02
    provides: Complete root-accepted historical corpus and checked wire decoders
provides:
  - Bounded source-bound tagged-data projection of both full BIP30 repeat/original pairs
  - Complete real history, raw MTP prefixes, old Coins, external inputs and detached BIP34 seed
  - Deterministic explicit generation and read-only offline corruption refusal
affects: [167-07, 167-18, 167-20]
tech-stack:
  added: []
  patterns: [Pure finite projection, Closed canonical record comparison, Fixed-path filesystem shell]
key-files:
  created:
    - scripts/phase167-bip30-fixtures.ts
    - scripts/phase167-bip30-fixtures.test.ts
    - packages/testdata/phase167/bip30-fixtures.txt
  modified: []
key-decisions:
  - "Reuse accepted02 decoders and full offline validation; never add another transaction wire encoder."
  - "Keep raw ten-header prefixes outside the155stored history rows and derive true eleven-sample clocks."
  - "Preserve shared coinbase txids across original/repeat roles; record identity includes case, role and ordinal."
requirements-completed: [CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T21:06:10Z"
commits: false
duration: 10min
completed: 2026-10-21
---

# Phase 167 Plan 26: Complete typed BIP30 fixture projection Summary

**Both real BIP30 repeat/original pairs now have complete source-bound typed fields, genuine clock windows and Coins in a deterministic offline tagged fixture.**

## Files Created/Modified

- Created: `scripts/phase167-bip30-fixtures.ts` — pure bounded projection and validation, accepted decoder reuse, fixed-path explicit CLI;339lines.
- Created: `scripts/phase167-bip30-fixtures.test.ts` —93focused actual-corpus reconstruction, corruption, offline and unchanged-output tests;350lines.
- Created: `packages/testdata/phase167/bip30-fixtures.txt` — documented ASCII/LF tagged fields;101,893bytes,421lines.

These are exactly the three source/data owners. This summary and ignored receipts are execution records. Baseline `1b472fb225a290df5cdb06eeea2dc67c624c648e` is the preexisting checkout commit, confirmed by git rev-parse HEAD; it is not a task commit. No staging, commit, push, worktree or hook bypass occurred. Shared metadata and downstream release remain root-owned.

## Completed Task

Task1 is complete within its producer boundary. loadProjectionInputs first calls accepted02 check(root) against all seven bodies,5263previous transactions and80,190,547corpus bytes. It then verifies all five frozen checker/data byte counts and digests, the fixed source pin, evidence class and twelve manifest-bound source facts. It parses the large context once for selection, returns a204-header finite carrier and reuses that compact carrier for pure tests; no test clones the80MiB corpus. Temporary shell roots use read-only symlinks and bounded local mutations.

validateProjectionInputs derives full typed headers and every transaction/input/output/witness field through accepted decodeHeader/decodeBlock and raw fixed-offset scalar reads. It verifies both independent literal original pairs and maybeBip30OriginalHeight, every header hash/network/height/link, original-parent windows, expiry windows and complete external spend set. Prior Coins derive from the actual paired coinbase outputs, original heights and original-parent MTP. No second CompactSize/transaction decoder or transaction wire encoder exists.

renderBip30Fixtures revalidates its finite input and emits the closed documented schema. Independent tag/arity/literal/hex/canonical-decimal/range validation precedes exact comparison with the full freshly derived record sequence. Exact comparison additionally fixes record order, identities, ordinals, counts and all fields; it cannot silently accept duplicate, omitted or extra data. SOURCE paths are a closed sorted allowlist, never file-open capabilities. Hashes are display-order; the documented later Rust conversion reverses hashes once, with scripts/witness unchanged. Empty byte fields use a dash. No escaping, JSON, Rust expressions or executable payload is emitted.

Explicit --write validates everything before writing only the owned fixture. --check validates inputs, reconstructs expected bytes and only reads/compares the output, refusing missing or changed data. Unknown, absent and combined modes, --acquire and arbitrary output paths refuse before I/O. A throwing fetch function covers actual generation and checking. No current clock, randomness or endpoint response enters the output;1792603828 is the frozen acquisition observation, distinct from candidate times.

## Case and Window Inventory

| Repeat→original | Repeat/original bytes | Full TX counts | Stored history, parent-first | Raw prefix, descending | Original-parent window, descending |
| --- | --- | --- | --- | --- | --- |
|91842→91812|214/616|1/2|91841..91687,155rows|91686..91677,10rows|91811..91801,11rows|
|91880→91722|472/214|2/1|91879..91725,155rows|91724..91715,10rows|91721..91711,11rows|

| Repeat | Parent MTP | Original creation MTP | Expiry MTP | Overwritten Coins | External Coins |
| --- | --- | --- | --- | --- | --- |
|91842|1289767733|1289756038|1289713700|1,5000000000sat,coinbase,created91812|0|
|91880|1289778307|1289721830|1289728088|1,5000000000sat,coinbase,created91722|1|

Every stored row has eleven genuine samples in history+raw-prefix; neither recursive fabricated prefix clocks nor global earned work are projected. Original parent clocks use the separate eleven-row window. Both real original bodies retain their complete noncoinbase contents where present. Same coinbase txid across original/repeat roles is intentional: d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599 for91842/91812 and e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468 for91880/91722.

91880 retains noncoinbase transaction bc9423e0a8b3571181d8545c0091600d8fc421c380457c45c1ca8c0698f765b8 and external outpoint cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3:1. The complete external Coin is4480000000sat, script76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac,coinbase=false,created_height90644,created_mtp1289277562. Tests fix these expectations independently.

The separately labeled detached_bip34 header is actual mainnet227931, hash000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8. Its typed80bytes independently reconstruct through accepted decoding. It grants neither branch membership nor canonical authority.

## Exact Input and Output Identities

| File | Bytes | SHA256 |
| --- | --- | --- |
|scripts/phase167-bip30-fixtures.ts|25298|e06f7675c50adda042ff64f3e95bf42a17953b77aad457cd13fc1510c36ffb63|
|scripts/phase167-bip30-fixtures.test.ts|23773|651d58b781192e0137677a4fbabcba3f92de2311d127b9bf348e08d4af90dc6b|
|packages/testdata/phase167/bip30-fixtures.txt|101893|7b91df3ceee0f702c46f98911f4dc92a85c85a0327697e424810c46a379fff5a|
|scripts/phase167-historical-corpus.ts|52343|5f9031294493bd42be0ff4a29bf6c4501fc9fb0b1ad8c7d0a8b720ffe7d0459b|
|packages/testdata/phase167/manifest.json|6034|16649d2cee73bfd769632bd8b59a06af50ee2d8bb69a2a71dcaad8be88d53b09|
|packages/testdata/phase167/blocks.json|2892399|6e81f7f06037d8c1e18b75ddef67d16aa6ab09a505ca616db612de1f29e0c335|
|packages/testdata/phase167/contexts.json|77269907|3067e98fb235c97226482a6c1b9fad2484fe58610f5ed1c25e8860a0a970fd5a|
|docs/parity/fixtures/phase167-rule-expectations.json|22207|4dddecfbb11a93eac11474d23f80bfff08736713a02941f7f21feb32b7a88c24|

All accepted02input hashes remain byte-identical to the amendment. Source pin remains a9aee730466ac67d35a3c03ee24676be5e045878. The17SOURCE records contain those five identities plus all twelve pinned-source digests from accepted02; accepted02 also compares each actual source file against the pinned Git object. No hash renewal or accepted-input rewrite occurred.

Actual output bounds are101,893bytes against1MiB,404records against1024,372bytes per longest line against16KiB and2transactions per largest body against32. It contains357HEADER records,6TX,7IN,7OUT,3COIN,2CASE,2END_CASE and1END, plus magic/PIN/17SOURCE. All six real transactions have empty witness arrays, so no WITNESS rows are appropriate; count0 is explicit on each input. Original128MiB/4096walk budgets remain untouched. Both source files are below629lines.

## Executed Commands and Actual Outcomes

All Bun commands used PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH. Full receipts are under .local/open-bitcoin-dev/phase167/plan26/. No Cargo/Bazel was needed or run.

| Stage/command | Actual outcome | Receipt |
| --- | --- | --- |
|Setup/context|gsd-tools init succeeded; project skill directories absent. Oversized displayed reads were repeated in smaller chunks. These are setup observations, not behavioral evidence.|setup-attempts.log;start.json|
|Initial bun test ./scripts/phase167-bip30-fixtures.test.ts|0pass/1fail/1error: new projector module absent. This proves source absence only, not a prior runtime regression.|source-absence.log|
|First implementation focused tests|3pass/1fail: my independent91812transaction-count expectation was3; accepted decoder proves2. Corrected the test expectation without changing corpus or projector behavior.|first-implementation.log|
|Grammar TDD RED|48pass/2fail: new validator accepted unknown tags and noncanonical integers when supplied as their own expected bytes. This exposed an incomplete new validation boundary.|grammar-red.log|
|Grammar GREEN|50pass/0fail after independent closed tag/arity/field grammar was added before equality.|grammar-green.log|
|Expanded suite first run|86pass/2fail: my expected rendered TX/IN/OUT count21was actually20; a byte-XOR data mutation produced invalid JSON before the intended digest boundary. Corrected count and used valid whitespace mutation to target digest refusal.|full-first.log|
|Expanded GREEN|88pass/0fail,2529assertions.|full-green.log|
|Final bun test ./scripts/phase167-bip30-fixtures.test.ts|93discovered/93pass/0fail,2538assertions,8.99s. Added fixed body/hash literals and explicit scalar/duplicate-role/external-Coin/row-swap controls.|final-tests.log|
|bun run scripts/phase167-bip30-fixtures.ts --write, twice|Both exit0,identical101893bytes and identical SHA256 shown above.|write-first.log;write-second.log|
|bun run scripts/phase167-bip30-fixtures.ts --check|Exit0;exact same bytes/digest/case count. Shell tests also retain missing/stale-output refusal and unchanged mtime on successful check.|final-check.log|
|bun build ./scripts/phase167-bip30-fixtures.ts --target=bun --outdir .local/open-bitcoin-dev/phase167/plan26/build|Exit0;two standard-library-only modules bundled.|build.log|
|bun build ./scripts/phase167-bip30-fixtures.test.ts --target=bun --outdir .local/open-bitcoin-dev/phase167/plan26/build|Exit0;three modules bundled.|test-build.log|
|gsd-tools verify key-links current26PLAN|all_verified:true,verified2,total2. Both producer links are actual; no07consumer acceptance is claimed.|key-links.json|
|Final input hashes, bounds, own new-file diffs and git diff --check|Exit0;exact frozen identities and three-owner scope retained. New-file git diff exit1 denotes expected additions and is handled explicitly; actual tool errors propagate.|final-stats.json;projector-diff.log;tests-diff.log;fixture-diff.log;diff-check.log|

Final tests reconstruct every rendered80-byte HEADER and recheck identity through accepted decodeHeader, then compare every rendered TX/IN/OUT/WITNESS field with accepted decodeBlock. They also compare full typed transaction arrays including coinbase/txid/wtxid. Both fixed pairs, all155-row clocks, complete old Coins, known external spend and real detached anchor have independent literal expectations. Corruption controls cover source paths/digests/pin/evidence, swapped pairs/bodies, raw fields, version/network/height, links/windows/prefix/anchor, missing/duplicate external Coins, all Coin fields, order/ordinal/counts, noncanonical/out-of-range integers, hex/encoding and file/line/record bounds. Explicit offline writes/checks use throwing fetch and temporary symlinks; input refusals preserve existing output and source bytes.

The first retained source-absence receipt was born at2026-10-21T20:55:44.728Z. Final source/data statistics at21:03:41.384Z measure477seconds; finalization through21:06:10.195Z measures625seconds, rounded10minutes. Initial context reading is excluded. completion.json records the final measured interval and three-owner count. Timing normalization is recorded in setup-attempts.log; no guessed start time is used for these metrics.

## Decisions, Simplification and Deviations

The implementation follows the checked plan. The simplification pass retains one accepted Bitcoin decoder and one canonical record renderer, with grammar plus fresh-byte comparison instead of another mutable parser or transaction encoder. The finite204-header carrier avoids repeated80MiB test copies. Intentional duplicate coinbase txids remain scoped by case/role/ordinal.

The initial new validator's grammar gap was corrected inline under Rule2 as a required validation mitigation; the real RED/GREEN is retained above. Incorrect positive test counts and the JSON-targeting mutation were test setup corrections, not historical product bugs or corpus corrections. No source contract or ownership blocker arose. No authentication gate, dependency, production Rust/export/error change, additional owner or guard/hash renewal was needed.

AGENTS.md, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification and Rust/TypeScript guidance informed the pure/shell boundary, meaningful tests and scoped receipt retention. Both canonically distinct active lesson files were completely loaded within8856bytes/2953estimated tokens; the existing audit baseline required no new audit. No project skills existed. Shared todo/STATE/ROADMAP/requirements/config and all other source owners remain root-owned and untouched by this executor.

## Threat and Evidence Boundaries

T-167-26-01 is addressed by full accepted02check, immutable source identities, literal pairs and actual decoder reconstruction. T-167-26-02 is addressed by bounded canonical grammar and complete ordered expected-record equality, with all mutation controls. T-167-26-03 is addressed by separate raw prefixes,155stored-row data and explicitly detached independent seed; no flags, work or canonical markers are serialized. T-167-26-04 is addressed by early explicit-mode refusal, fixed output path, validation before write, read-only exact checks, preserved output on input refusal and throwing-fetch evidence.

No unresolved stub or additional security surface was introduced beyond the declared fixture file and explicit local CLI. Empty witness arrays and91842's empty external Coin set are genuine decoded facts. Evidence remains independently-seeded-historical data adaptation. Plan07 still owns Rust typed construction and actual seeded stage/absorption/disconnect/reconnect evidence; Plans08/16 retain durable historical evidence. This producer establishes no canonical ancestry, HeaderStore earned clock/work authority, ordinary product acceptance, actual-daemon qualification or whole-phase completion. Root independently accepts26 and controls07release; full native/coverage/Bazel and phase reviews remain later gates. commits:false; shared state updates and final saving remain root obligations.

## Self-Check: PASSED

- All three declared source/data files exist with the exact hashes and bounds recorded above.
- Preexisting baseline commit resolves in Git; no task commit is claimed.
- Final93focused tests pass; explicit offline check and both builds pass.
- Both current26producer links verify2/2; accepted02frozen input hashes remain exact.
- gsd-tools verify-summary --check-count3 returns passed:true,three files checked/found and errors:[]; receipt verify-summary.json.

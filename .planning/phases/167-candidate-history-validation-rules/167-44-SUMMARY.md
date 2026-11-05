---
phase: 167-candidate-history-validation-rules
plan: "44"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage-tests
tags: [rust, fjall, basic-filter, metadata-family]
requires:
  - phase: 167-28
    provides: Bounded segmented metadata row-family representation
  - phase: 167-29
    provides: Existing metadata carrier and absent versus unknown count semantics
provides:
  - Genuine metadata-family absence in the unchanged 401-position startup scenario
  - Full nine-map post-tamper refusal conservation
  - Genuine legacy writer and non-family prefix-collision coverage
affects: [167-35, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Exact root-or-colon family ownership, post-tamper refusal baseline, existing typed loader]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
key-decisions:
  - Repair the fixture precondition and preserve MissingMetadata and production startup order.
  - Remove the whole exact family in one raw operation and preserve every unrelated byte.
  - Compare all nine maps after tamper versus after refusal.
  - Keep Byteview as one separate native supplement beyond original201.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-24T00:28:47.173Z"
completed: 2026-10-24
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 44: Complete Metadata-Family Fixture Summary

**The missing-metadata startup test now removes the actual segmented family, earns its original refusal category and proves all nine native maps remain as deliberately tampered.**

ROOT accepted this exact one-owner source/runtime CORE for the Summary stage and separately released this canonical Summary and scoped QA. Independent SummaryQA and final counted source acceptance remain pending; this Summary-stage release is not an accepted-source receipt for a later executor. The generated_at records actual canonical creation. The requirements array copies all five plan IDs as a bounded contribution; CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain globally OPEN. All three planned implementation/verification tasks have actual evidence. No nested task/metadata commits were made under commits:false.

## Exact source and native scope

One existing writable source only: packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs, 10111bytes/272normal lines/SHA2560ca2a3b550f94c6b6b9f6b25fd90555337d9d83049d1014ba3ae62820865f1b1. It is EXACT checked0ca2proposal; the original is5687bytes/SHA5b2273f8445aa056f96a3a37ab05b820e37134fa86352e4472905cb7fa25e54a. No second source, new child, production behavior, dependency or visibility/test API changes. The two local helpers keep normal272-line fit below300/628 without packing or sharing across another owner.

The actual ROOT release {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual44-release-v1/release.json","bytes":14193,"sha256":"c316262de5e985a64e7fe746fbf8a36b2d07bf9885e9ffb538f3bb04ae56e4cd"} binds source106present:1existing owner/105protected. All85 inherited43-frame bodies remain exact, including the six accepted43owners; the additional21read-only/owner inputs are fresh44scope, never claimed historically inside85or82. The accepted43receipt is bound in the packet. Current35all11remain protected/unaccepted observations; original35exclusive5b0eight-body/three-absence authority is untouched.

Native inputs remain ORIGINAL201 exact and SEPARATE Byteview1, derived202total. Pinned Fjall keyspace and three lsm-tree Slice/lib files were already inside201; Byteview was outside it and now has its own explicit fresh ROOT capture. Every job and final/inverse preserve both lists separately. No historical201coverage is invented, new dependency downloaded or failed fixture accessed. The supplemental accepted-body provenance must remain separate in a later ROOT composite35authority after actual44acceptance.

## Actual behavior and source conservation

Production startup already loads saved metadata and builds the verified fence BEFORE unsafe-prune checking. The original401-position fixture, intent20and checkpoint1 remain unchanged. At that size the legacy root is absent and actual segmented manifest/rows exist, so deleting only chain_meta did nothing; intact valid metadata correctly reached UnsafePruneIntent. The expected MissingMetadata category was retained.

The passing repaired case uses the real production loader to assert all401original positions and pre-tamper None transaction counts, observes actual manifest/segment rows, absent legacy root and unrelated undo, and drops every store before raw mutation. Private is_metadata_key owns ONLY key==chain_meta OR starts_with chain_meta:. In ONE existing raw_namespace operation, the helper requires a nonempty real family, removes exactly those keys, proves complete family absence and compares every unrelated chainstate key/value byte. Existing SyncAll persistence remains.

The real loader then asserts empty positions with Some(empty counts), never None/unknown. The test takes all nine FULL ordered native maps after deliberate tamper, drops the store, invokes real startup, checks the original exact missing-metadata detail, reopens and requires all nine maps equal the post-tamper baseline. All original index/body/undo/retained-intent assertions remain. These are assertions reached by the actual passing named test, not a separately invented trace or an inspection of a failed fixture.

A separate real legacy writer creates one ChainPosition in fresh owned scratch through save_chain_meta, proves its legacy root/no manifest, and inserts the non-family chain_metadata prefix collision. The SAME eraser leaves that exact value intact, removes all owned keys and earns ordinary-loader empty/Some(empty). It never shortens or reseeds the original401-position case. The three unrelated original recovery case bodies and the complete parent unsafe-prune control are preserved as independently checked in the source proposal; the applied final body is exact to that reviewed proposal. All105protected inputs remain byte-exact.

The simplification pass retained two short local private helpers and existing APIs. No generic family-mutation framework, namespace clearing, partial manifest deletion, decoder change, category swap or production precedence change was introduced.

## Honest RED, control, GREEN and final outcomes

The exact original RED compiled/executed one real named missing-durable-metadata test and exited101:0PASS/1FAIL/0ignored in1.46test seconds. Its raw panic at unchanged filter_index.rs339 is the actual unsafe-prune detail against the original missing-metadata assertion. This is the diagnosed behavioral failure, not compile/setup error. Its failed fixture is never reopened/reused/cleaned up. The separate unchanged intact-metadata required-history control passes1/0/0 in1.70seconds BEFORE any source edit.

Only after both jobs closed, the exact checked10111-byte proposal replaced the original sole owner. Source-application evidence retains original/final owner copies and read-only protected/native hash observations (those observations are not labeled copied bodies). The one-owner formatter exits0and retains the same0ca2body. Family GREEN actually compiles the changed node and executes all5recovery cases with0failed/ignored in8.28seconds, including genuine legacy prefix collision and the repaired original case. No original RED is rerun for selected-pass credit.

| Fresh final | Named passes | Test seconds | Command wall seconds | Exit |
| ----------- | -----------: | -----------: | -------------------: | ---: |
| final-01    |            5 |         6.48 |                7.053 |    0 |
| final-02    |           41 |        67.40 |               67.764 |    0 |
| final-03    |           30 |        79.61 |               80.076 |    0 |
| final-04    |   check only |          n/a |                2.259 |    0 |
| final-05    |   check only |          n/a |               10.142 |    0 |

These 76passing executions overlap; they are not unique-case totals. Each selected recovery/startup/bounded-row group has nonzero named discovery and all outcomes are ok; no ignored/UNPARSED result is hidden. Both final recovery and full startup include every one of the5GREEN concerns. Empty auxiliary harnesses provide no discovery. Global fmt is --check; Node Clippy retains all-targets/all-features/-Dwarnings.

Exact five final argv:

```sh
bun run scripts/command-timings.ts run --key phase167-44-final-recovery -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::filter_index::startup::recovery -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-44-final-startup -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::filter_index::startup -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-44-final-rows -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::bounded_rows -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-44-final-format -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase167-44-final-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
```

All9actual app jobs retain complete command/argv/raw stdout+stderr/started+result/names+totals/ignored reason, actual runner/helper refs and106source/original201/separate1 before+after copies. This gives 1908actual app source copies, 3618ORIGINAL-native copies and 18SEPARATE Byteview copies; source-application/initial/final/inverse metadata copies are excluded from those totals. Original RED's expected nonzero stays nonzero. No additional unexpected app failure, source expansion or pass-only retry occurred.

## Frozen frame, environment and inverse

Final frame {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution44-v1/final/capture.json","bytes":125233,"sha256":"7b9f9fb147d6c7c92098154ffad56d8e8081bf05c7996599cd8adc8abeeb9794"}. Complete packet {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution44-v1/source-first-packet.json","bytes":1680909,"sha256":"52d40cfec2cf1275643c804ed9a852046136901d7c2ce841c39afd388d343500"} authenticates6587references. All per-job body maps are checked against original106for the two initial commands or final106for later jobs; native201and separate1always match their ROOT baselines. Current final actual source/native paths match the frozen copied frame. No source writes occur while runtime jobs are queued/running; the sole authorized formatter has its precise one-owner write window.

Pinned Bun1.3.9and Rust1.94.1, original canonical physical TMPDIR identity and default target/cache remain. The exact approved1474-byte8da9body helper is bound on every job and preserves strict presence/hash/schema/ENOENT-only behavior. Every job's actual asynchronous nine-key environment proof and application use the SAME explicit snapshot; selected values and raw proof output agree. No full environment dump or target/state/flag/toolchain override is used. The original host-global guard naturally waits for external ESP/Bazel work; guard waits are separate from app outcomes. No signal, waived guard, silence timeout, held host-global lock, atomic snapshot or observation-to-spawn race theorem is claimed.

Whole-file scratch inverse {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution44-v1/inverse/result.json","bytes":3362,"sha256":"8fc523c6ca53dd3e0e5424272778068fc3b3166355d7b5fab77cf087d984cf8d"} restores EXACT original5687-byte5b2273body and leaves all105protected source files byte-exact; forward restores final0ca2body/all106present. BSD patch --batch -E -F0 -p1 has no offset/fuzz/reject. ORIGINAL201and separateByteview1remain exact at both stages, indexed inside scratch rather than writing their original absolute paths. Original authority171913Band supplemental672Bare copied raw/losslessly; body-only projections do not pretend to preserve full numeric historical records through JavaScript Number. No live source rollback or original35before-authority substitution occurs.

## Additional Provenance and Outcome Accounting

The nine actual commands contain82successful test executions and ONE EXPECTED original RED failure; they are not all passing commands. Current final groups5/41/30give76overlapping executions, with0failed/ignored. Original RED101/one real named failure in1.46seconds and unchanged control1PASS in1.70seconds both precede the source edit; GREEN5PASS in8.28seconds follows it. No old full35output or source-only proposal supplies current TDD/final credit.

All one-owner sources and actual native inputs are bound by the explicit current ROOT frame, not an inferred historical start capture. The original three unrelated case bodies, private existing APIs and complete401-position/intent20/checkpoint1 scenario remain. The passing case distinguishes None counts before tamper from Some(empty) after real absence; all nine maps compare post-tamper versus post-refusal rather than the untouched original database. The legacy writer/prefix-collision case is fresh and separate. No production change or source expansion occurred.

Root fresh original authority: {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-before44-v1/capture.json","bytes":171913,"sha256":"de33d5e4ba7ef4b79ff86c02bff714acb207378645b2947137d4c40852fdf99e"}. Separate Byteview authority: {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-before44-v1/supplemental-native.json","bytes":672,"sha256":"5809b8b380a2441594e7d0a0b76722b6a2c7aef997022a477956d5b3b079621d"}. Root application authentication: {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual44-application-authentication-v1.json","bytes":2062,"sha256":"eaf90b1b455d7c6fe3d373a12a69eed6c464c446ef0b0e4aa2d05daa0da52647"}. Root distinct final original-body F0: {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual44-final-inverse-v1/result.json","bytes":2488,"sha256":"a151705eabb58fd44704317041fad972869230bfee68b12c238128f6f4a9db82"}. Root complete packet/raw-name/source/environment authentication: {"path":".local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual44-core-authentication-v1/result.json","bytes":5390,"sha256":"ab22e552177fcbafba568e5167e6fa3a62f1744365b58158fe6f19cbdc91a89a"}. ROOT whole-read the11142-byte author report and reconciled all6587packet references,1908source/3618original-native/18separate-native app copies, all9chronology and82positive outcomes plus the expected RED. These author/ROOT authentication receipts do not substitute for sealed independent CORE.

## Independent CORE and Document QA

The WHOLE sealed independent CORE report was read/authenticated: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent44-source-core-review-v1/actual-core-v1/REVIEW.md`, 16647 bytes, SHA256 `b25b5a8b35bfe6d10f64ba4d3040352f309aaff0c4d8e3cdddb1cca1b6fabd1b`. Its index is `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent44-source-core-review-v1/actual-core-v1/index.json`, 197052 bytes, SHA256 `05e3ead303bf8654089b88229d19897cd243f87ed7f363193434ebc4ed289080`. It is CLEAN with0source/evidence findings,16706assertions,15859hash authentications across6593distinct referenced paths, including every6587packet binding. It independently reconciles all9chronology, raw outcomes, source/native copies, actual environment and a distinct FINAL106-state original-owner F0inverse/forward. It confirms the76overlapping final passes/71distinct selected names,82positive executions across all9jobs plus exactly ONE expected original RED. No reviewer Cargo/Bazel/compiler/native-engine/failed-fixture/live-source invocation or canonical/Git write supplies that review.

ROOT whole-read/authenticated the sealed report/index and all current106source plus ORIGINAL201/SEPARATE Byteview1, then released ONLY this Summary/scoped QA: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root44-core-acceptance-summary-release-v1/release.json`,3605bytes/SHA256 `d69c23e90f4cdb4c13111bce6c42863df7288f94d9fb35f69f1c3b2089268dcf`. Its sourceCoreAccepted=true and sourceAccepted=false remain distinct; final counted acceptance waits for independent SummaryQA and ROOT. All prior inert/readiness/prefix evidence remains unchanged.

CURRENT canonical scoped document QA is under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary44-execution-v1/document-QA/`: initial/final copies, actual mdformat GFM/frontmatter check-before-format-write and final-check receipts, exact fields, actual one-owner hash and all current106/original201/separate1 input-hash checks, then GSD verify-summary --check-count1. The earlier ignored draft's formatting-only initial check/write/final pass stays separately under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary44-preparation-v1/draft-QA/`. These paths identify current versus predecessor evidence truthfully. No GSD verifier ran against an absent canonical Summary. Independent SummaryQA must bind this actual current Summary hash and immutable CORE before final ROOT acceptance. CORE has no Summary dependency.

Repo AGENTS, Bright Builds sidecar/overrides and relevant architecture/code-shape/testing/verification/Rust guidance informed execution. Both active lesson files were fully read,9838bytes/3280conservative tokens; archive inputs were excluded. No known source stub or unmodeled production trust surface was introduced. No external user setup is needed.

## Remaining Gates

ROOT has accepted exact44source/runtime CORE for this Summary stage and released this document; independent SummaryQA and final counted ROOT acceptance remain pending. All five global CVLDrequirements stay OPEN. Accepted43is unchanged. Full35remains FAILED with its original4ignored reasons and original exclusive5b0eight-body/three-absence authority. ROOT must derive complete accepted43/44/46/45source/native supplemental provenance, independently verify the expanded composite/inverse, then separately release35and rerun all six focused+three original final commands. No old pass supplies that future credit.

Peer46/45and later167observer predecessor,13/14, formal15ordinary301/reopen150/301/next302,16separate historical kernel, compactC/D/E(Dfewer than5tasks,E→17), source review BEFORE17including123,18security/native/mutants,19parity/breadcrumbs/README/architecture,25truthful PROJECT/AGENTS,20entire default native/fresh pure-core coverage/Bazel/independent source/security/goal/lifecycle and normal-hook main commit/push remain ROOT-owned. No actual source/app/sharedSTATE/ROADMAP/TODO/counts/requirements/Git/actual35orpeer/later action follows from this Summary. Phase168stays unstarted.

## Self-Check: PASSED

The one final owner, original authority and listed immutable receipts are present and bound by recorded hashes. The current selected test outcomes and format/strict checks are retained, with original chronological evidence distinct from later final credit. The whole sealed independent CORE is CLEAN0 and ROOT has accepted its exact source/runtime contribution for the Summary stage. The actual one owner and all106source/original201/separate1 inputs were checked against the final frame before canonical creation. No commits are expected under commits:false. Independent SummaryQA and final ROOT acceptance remain pending; all global CVLD, whole167, Git and168gates stay open.

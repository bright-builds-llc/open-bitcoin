---
phase: 167-candidate-history-validation-rules
plan: "46"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-network
tags: [rust, peer-lifetime, basic-filter, ownership]
requires:
  - phase: 167-34
    provides: Existing typed node authority and bounded storage foundations
provides:
  - Exact original-peer lifetime authority for admission, dispatch and scoped requests
  - One shared complete BASIC preparation and owned continuation/disconnect settlement
affects: [167-45, 167-35, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Original-lifetime ownership, typed continuation, exact whole-source evidence]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/disconnect.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/basic_tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/registry.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs
key-decisions:
  - Preserve the original lifetime and typed production path before physical effects.
  - Keep current recovery separate from original whole-file inverse authority.
  - Require fresh combined strict evidence and dependency-ordered Summary QA.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-25T00:18:20.623Z"
completed: "2026-10-25"
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 46: Peer-Lifetime Authority and BASIC Continuation Summary

**The seven-owner authority API binds effects to the original admitted peer lifetime, and current producer/consumer composition has fresh strict joint evidence.**

ROOT accepted the exact joint source/runtime CORE for the Plan46 Summary stage and separately released this canonical Summary/scoped document QA under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root46-r8-2-core-acceptance-summary-release-v1/release.json` (4261 bytes; SHA256 `6d34468aa3cfc43016b3c752fb857ec4989166f52a3ffd5ead50665b14882859`). Its sourceCoreAccepted=true and sourceAccepted=false are distinct: independent Summary46QA and final counted ROOT source acceptance remain PENDING. The actual canonical generated_at records this publication. The five requirements IDs copy the bounded plan contribution; all five global CVLD requirements remain OPEN. No nested commits were made under commits:false.

## Exact source and native scope

The canonical plan has three tasks:

- Task 1: Implement actual admission/lifetime/request authority and private controls
- Task 2: Share complete BASIC preparation and guard every continuation effect
- Task 3: Earn only reviewed dependency readiness, then await mandatory joint strict closure

Seven structural owners remain byte-identical to actual R7 and legacyv6; Plan45 never rewrote them. Their current whole bodies are:

| Whole source owner                                                                            | Bytes | Normal lines | SHA256                                                             |
| --------------------------------------------------------------------------------------------- | ----: | -----------: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/network.rs`                                                   | 24951 |          619 | `3200f6bd745cfa549c10a95c7a820bfbd48d8bc2ece43bc06b64f93f9702ea6f` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs`                                 | 21916 |          595 | `bcb6d516bfbc2bb6a062ecdab9e76aed6b9ec8f8db24958b3a0e927953e3f423` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs`            | 20037 |          525 | `e3e0aaccae533321e1ecfd4e9da71302d2b6dad1fc7d09c5bb069a6286386202` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/disconnect.rs` |  6902 |          182 | `2a479038ebf3a886961439843ad5ce83da0d3e8d7eb23fd168ea2035d13ca315` |
| `packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions.rs`                   | 17735 |          537 | `f0e4489e3ed76c1c975c40bec4eb23ecf8af690bd2175d50acd2ddd2b295516b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/basic_tests.rs`       | 20503 |          611 | `349c17e7ccbaa2fd856995b4e8ae56c28b964ffe4641ff944286ac2353775cb9` |
| `packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/registry.rs`          | 19505 |          507 | `f2f368851ace335e92c293cd2430b1ca50b3d8d8dd690e1088011b84a307d5a0` |

Original Plan46 direct provenance remains `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-before46-v1/capture.json` (258744 bytes; SHA256 `de637947f0cdd61130a7fe0e478925ea6c61f9087c43b1c153a82920675a0d82`): three whole bodies/four TRUE absences, within expanded original scope `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-expanded46-poison-scope-v1/expanded-original-before.json` (257871 bytes; SHA256 `498082a8ec40d32f298c6fcd59b125ab402a63ad77aa88d939d1bf6ca3cc4391`). The legacy verification frame is273states, including the four unrelated future45 child absences; original7 inverse protects266 other states and native201/separate1. Historical original271 initial capture is separate and is never relabeled as273 or284. Current joint284 is a later combined frame; no current-present copy replaces the original direct3/4 inverse authority.

## Actual authority and BASIC behavior

`runtime_authority.rs` constructs/clones the shared registry; `network.rs` retains legitimate crate-private concrete exports and the exact existing raw outbound relocation. The private registry distinguishes the original nonce/lifetime and live cleanup-Weak identity, so a same-ID replacement cannot receive admission, request, queue or cleanup credit. The scoped producer checks the original token before effects and preserves actual membership and genuine component errors.

The peer-session module carries typed admission/request/continuation outcomes. A known denial retains already earned raw counters/observations and genuine errors without permitting old cleanup against a replacement. Owned PeerState snapshots and boxed private denial/optional fields retain concrete typed ownership and existing lock/drop behavior; they are not an erased authority escape.

Serving uses one shared complete locked BASIC preparation. Every invalid semantic reason is checked under the same original lifetime before reservation/read/bind and disconnect. Pending disconnect releases the real prune/wake path before guard-free completion; physical handed-frame costs, bounded I/O, leases, message budgets, generic paths and wake ordering remain. Genuine public-genesis BASIC/lease/error controls and actual raw snapshot poison checks remain source-bound. The early copied packet alone supplied no compiler/privacy/borrow/runtime proof; those concerns are now covered by actual source/task/preconsumer and fresh joint runs.

## Legacy staged evidence and honest warning boundary

Actual v6 performed12 fresh commands with255overlapping passes/180unique passing names and no ignored outcome. Its Clippy command was **without `-D`**, retaining308raw diagnostic occurrences. The26operative unused-API groups were independently mapped to genuine future45 production consumers;255passes and dependency readiness were not strict/final acceptance. Whole legacy outcomes: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution46-v6/preconsumer-CORE-v1/outcomes.json` (127589 bytes; SHA256 `558c0c24eade41ddd3586e5e818c510facb44ad9b444df6a141a8fd0f5a5f384`); diagnostics: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution46-v6/preconsumer-CORE-v1/diagnostics.json` (621164 bytes; SHA256 `1b35b9785dac59efb228ebd43ed86837b176a379c0d09a9c5ddbe6d20c55b2b4`); dispositions: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent46-v6-preconsumer-core-review-v1/diagnostic-dispositions.json` (1522814 bytes; SHA256 `3fe26128c308a354ba473101449ea3598790ad6eb25e1a4691a02d02ba4cf4f3`).

| Exact command key                | Named PASS | Ignored | Reported test s | Recorded job s | Native exit |
| -------------------------------- | ---------: | ------: | --------------: | -------------: | ----------: |
| `phase167-46-task1`              |         19 |       0 |           20.45 |         21.470 |           0 |
| `phase167-46-task2-basic`        |         19 |       0 |           17.36 |         18.255 |           0 |
| `phase167-46-pre-format`         | check only |       0 |            0.00 |          2.558 |           0 |
| `phase167-46-pre-lifetime`       |         19 |       0 |           26.52 |         27.152 |           0 |
| `phase167-46-pre-authority`      |        132 |       0 |         4065.27 |       4066.071 |           0 |
| `phase167-46-pre-readiness`      |         21 |       0 |           40.47 |         41.294 |           0 |
| `phase167-46-pre-serving`        |          8 |       0 |          171.93 |        172.631 |           0 |
| `phase167-46-pre-branch`         |          4 |       0 |            2.74 |          3.352 |           0 |
| `phase167-46-pre-transport`      |         14 |       0 |            5.88 |          6.395 |           0 |
| `phase167-46-pre-peer-filters`   |         19 |       0 |           35.81 |         36.143 |           0 |
| `phase167-46-pre-build`          | check only |       0 |            0.00 |          0.378 |           0 |
| `phase167-46-clippy-diagnostics` | check only |       0 |            0.00 |         10.814 |           0 |

Exact historical argv, retained only as staged history:

```sh
bun run scripts/command-timings.ts run --key phase167-46-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_peer_session -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-task2-basic -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_peer_session -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-format -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase167-46-pre-lifetime -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase167_peer_session -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-authority -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-readiness -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::filter_index::catch_up::phase159_readiness_tests -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-serving -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::filter_index::peer_fixtures::phase160_serving_tests -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-branch -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority::filter_index::branch -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-transport -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority::filter_index::transport -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-peer-filters -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::peer_filter_transport -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-pre-build -- cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features
bun run scripts/command-timings.ts run --key phase167-46-clippy-diagnostics -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features
```

Original7 whole-file scratch inverse: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution46-v6/inverse/result.json` (2069 bytes; SHA256 `129f0dc2978a9989fe6116a225e879e5218c1cfb17aaaeba7d3b8986655dafa7`); it restored3whole/four true absences, protected266, preserved native201/separate1 and forwarded to the same actual R7 body. It is not the later Plan45 twelve-owner inverse. The genuine `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root46-v6-dependency-ready-v1/dependency-ready.json` (8035 bytes; SHA256 `051fb3266aa0e80b0c378d9534e17066f7660de7cb57044d6bc4d21a7b11de67`) remains `ROOT46_DEPENDENCY_READY_NOT_FINAL`, with all final flags false and permission only for a separately released45 source stage. It is never retroactively called final acceptance or strict success.

## Fresh joint final evidence on current source

The nine original46 joint rows below have234overlapping passing test observations, nonzero discovery in every test selector and zero warnings/errors/FAILED/UNPARSED/ignored. The separate Plan45 strict-D row also closed0 on this SAME joint frame; that actual strict result is `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/jobs/phase167-45-final-clippy/result.json` / SHA256 `282073e3fec8f3aa8dfe81922016c97f2fc690bbcc0151d356692ded01001272`. Current all20 totals are1148overlapping passes, not unique cases. No legacyv6 or earlier failed/guarded run supplies current final credit.

| Exact command key                | Named PASS | Ignored | Reported test s | Recorded job s | Native exit |
| -------------------------------- | ---------: | ------: | --------------: | -------------: | ----------: |
| `phase167-46-joint-format`       | check only |       0 |            0.00 |          2.003 |           0 |
| `phase167-46-joint-build`        | check only |       0 |            0.00 |          0.408 |           0 |
| `phase167-46-joint-lifetime`     |         36 |       0 |          124.57 |        125.820 |           0 |
| `phase167-46-joint-authority`    |        132 |       0 |         4015.89 |       4016.531 |           0 |
| `phase167-46-joint-readiness`    |         21 |       0 |           23.92 |         24.485 |           0 |
| `phase167-46-joint-serving`      |          8 |       0 |          174.22 |        174.496 |           0 |
| `phase167-46-joint-branch`       |          4 |       0 |            2.50 |          3.037 |           0 |
| `phase167-46-joint-transport`    |         14 |       0 |            6.52 |          7.056 |           0 |
| `phase167-46-joint-peer-filters` |         19 |       0 |           38.18 |         38.821 |           0 |

Exact current nine joint argv:

```sh
bun run scripts/command-timings.ts run --key phase167-46-joint-format -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase167-46-joint-build -- cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features
bun run scripts/command-timings.ts run --key phase167-46-joint-lifetime -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features phase167_peer_session -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-authority -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-readiness -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::filter_index::catch_up::phase159_readiness_tests -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-serving -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::filter_index::peer_fixtures::phase160_serving_tests -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-branch -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority::filter_index::branch -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-transport -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features network::runtime_authority::filter_index::transport -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-46-joint-peer-filters -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features sync::tests::peer_filter_transport -- --test-threads=1
```

The actual readiness selector is `sync::tests::filter_index::catch_up::phase159_readiness_tests`; actual serving is `sync::tests::filter_index::peer_fixtures::phase160_serving_tests`, both explicitly all-features. Both produced genuine nonzero passing discovery. The two lifetime rows each include the real emission/outbox handshake case names asPASS. Current source/API19 binding and strict consumer composition close the staged warning boundary; ROOT/independent acceptance remains the separate unbound stage below.

## Evidence, environment and remaining gates

The shared facts are `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary45-46-r8-2-preparation-v1/facts.json`: exact source-copy refs, all source hashes, original/current provenance, actual argv, raw-result links, measured timings, named outcome mappings, ignored reasons and stage ordering. Full raw names are retained in the actual CORE outcome artifact, rather than duplicated here. Primary joint CORE: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/joint-CORE-v1/CORE.json` (7739630 bytes; SHA256 `c20eb61cfc0d7623e05dc890e9ad784c4defbae60a4c7a323734426e4079b722`). CORE handoff: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/CORE-HANDOFF.json` (1682 bytes; SHA256 `f623c1e86845a6658e386e4dee1fc6c992d54f5fd69887d5a19e39c3909e1a4d`). Full delivery: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-delivery-v1/index.json` (7656797 bytes; SHA256 `830ac689a2cef49f4913a8ac931ba7ba9103902f83feaa3e831b9fc786a55958`). These are actual evidence, not an independent/ROOT final acceptance.

ROOT all20 authentication: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-all20-review-v1/authentication.json` (6646 bytes; SHA256 `dbb3061f42393f778fb9a9707983d33572ab1956a98ed5d702f9b7b1b5fdfdf3`). ROOT inverse authentication: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-inverse-review-v1/authentication.json` (1295 bytes; SHA256 `cd4fee3534b97dd9547c720647ce4d3151b354e60cb14674d8137c158a697d6e`). Actual19-source/API prerequisites: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-input-review-v1/api-prerequisites.json` (14967 bytes; SHA256 `de08f2740106ecb2484c11aab669227dec72d1d7cf7ca396f5072da2700bb065`). Exact zero-diagnostic decision: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/diagnostic-decision.json` (9774 bytes; SHA256 `cfc52815ec88eb29f23cfafad62fe04b5fb7604940a8cef0c236479af230c6fe`). The historical26 operative unused-API warning groups (286 operative occurrences plus22 compiler-summary occurrences,308raw total) are retained in their original records. Actual production consumers/obsolete lookup and re-export retirement plus current strict-D and all20 zero-warning runs close those residuals; no lint whitelist, suppression, widened public API or artificial caller supplies the closure. The existing `Some(*peer)` unbox is part of the real Plan45 consumer body.

Actual two CORE-only metadata guard stops remain separate: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-sealing-HOLD-v1/index.json` (3612 bytes; SHA256 `17b8d2e1b3e46c336149f43b6c99a3994825f3d65a3fa9d1ae3c9513dbe1355f`) records relative-versus-absolute sealer-path rejection; `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-sealing-HOLD-v2/index.json` (3605 bytes; SHA256 `1333fb81d7d19cd354382fda4f8bd5956ee7471209abb0db8b08d484d3e3d0dd`) records missing exact Root diagnostic-decision fields. Both closed outer1 before CORE output and supply no runtime/source pass credit. Successful v3 uses `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/permission.json` (5349 bytes; SHA256 `327e024f55a3f0ac24bf720a94c7720945d4f2f793b69ad220b62bb0315283a4`) and `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/binding.json` (9222 bytes; SHA256 `0e198ab1f67af8e65fb175aa0771020cc1c86db3426a7a20cfbb57908bb13ec7`); it closed0 and is not counted as a failure. Every old receipt and HOLD remains unchanged.

Pinned Bun1.3.9/Rust1.94.1, default target/cache and original physical TMPDIR were retained. Actual asynchronous nine-key child probes and product commands used the same explicit snapshot; the six target/state/flag/toolchain overrides remained unset. Natural host-global/cooperative timing guards serialized work. No signal, latency deadline, held host-global-lock theorem, atomic snapshot or observation-to-spawn race elimination is claimed. Native ORIGINAL201 and separate Byteview1 remain separate lists,202derived total; no old201coverage is invented. No failed fixture was accessed or reused.

The current joint284 frame is `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/joint-final/capture.json` (187416 bytes; SHA256 `d3c5ea565300f4e7396a6f323801de546835ef6f9ca73282662f0e1354d79861`). All20 fresh jobs were closed before the original12 scratch inverse. Source writes remained closed while runtime jobs ran. All18 finals started after the maximum source/native/supplemental capture end. Ignored outcomes are never pass credit: exactly four outer final-sync name/reason pairs are retained, multiplicity1, with no added outer `--ignored`/`--include-ignored` or live opt-in. Existing ordinary bounded subprocess supervision remains unchanged.

Independent actual CORE review: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent-r8-2-actual-joint-core-review-v1/index.json` (6674 bytes; SHA256 `8cc74e5c20a8a583aa0533ce88baeb0407973783bda80db3308316123cd73848`), with whole report `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent-r8-2-actual-joint-core-review-v1/REPORT.md` (16542 bytes; SHA256 `01353860a64f7e0e66e25d8aeed5f60cc739a968c2291b23de71dab200ab53d2`). The sealed review has0findings/18094semantic checks, authenticates all28925CORE files, binds all19source/API owners and all26historical warning groups to actual production links, and reconciles1148overlapping passes/four exact ignored outcomes. It grants no Root adjudication, Summary release or counted source acceptance. ROOT source-core/Summary46 publication grant: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root46-r8-2-core-acceptance-summary-release-v1/release.json` (4261 bytes; SHA256 `6d34468aa3cfc43016b3c752fb857ec4989166f52a3ffd5ead50665b14882859`). Independent Summary46QA/final46 counted acceptance remain PENDING. Only then may ROOT separately release Summary45 and its scoped independent QA/final counted acceptance, all PENDING. Current scoped document QA is under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary46-r8-2-execution-v1/document-QA/`; the earlier inert QA proposal remains separately in the unchanged preparation folder. No GSD verifier was run against an absent canonical Summary. The canonical45 Summary remains absent.

No global CVLD closure, full35 rerun, original35exclusive5b0 replacement, shared STATE/count/requirements/ROADMAP mutation, commit/push or phase168 start follows. Full35 remains pending its own ROOT-released repair/composite/focused and final checks. Whole167/security/parity/breadcrumbs/README/default native/coverage/Bazel/normal-hook main finalization remain ROOT-owned. STOP168.

## Self-Check: PASSED

The seven current whole source hashes and immutable joint/preconsumer/original inverse evidence are bound by the recorded facts. All284source/original201/separate1 current input hashes were checked for publication. Sealed independent actual CORE PASS8cc74e5c and ROOT Summary-stage grant6d34468a are bound; sourceCoreAccepted=true and sourceAccepted=false remain distinct. Actual canonical creation time is recorded. No commits are expected under commits:false. Independent Summary46QA and final counted ROOT acceptance remain PENDING. All five global CVLD,35,whole167,Git and168 gates remain open.

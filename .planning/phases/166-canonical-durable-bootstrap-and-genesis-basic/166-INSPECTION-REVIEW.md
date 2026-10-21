---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
reviewed: "2026-10-20T03:40:52Z"
depth: standard
scope: "166-04 and 166-15 storage inspection checkpoint"
diff_base: 1273b4fb
files_reviewed: 20
files_reviewed_list:
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/open_lease/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests/copy.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-cli/src/lib.rs
  - packages/open-bitcoin-cli/src/bin/open-bitcoin.rs
  - packages/open-bitcoin-cli/BUILD.bazel
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/BUILD.bazel
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
commits: false
---

# Phase 166: Storage Inspection Checkpoint Review

**Reviewed:** 2026-10-20T03:40:52Z
**Depth:** standard
**Files Reviewed:** 20
**Status:** clean within the explicit inspection checkpoint scope

## Summary

No actionable correctness, security or maintainability issues were found in the reviewed source. The review covered the explicit current files, including untracked modules that a tracked Git diff omits. It used the diff against `1273b4fb` to identify moved constructors and entrypoint changes, read the Rust/build source, parsed the complete breadcrumb manifest and inspected the changed registrations. Planning artifacts supplied intent and were not themselves reviewed for defects.

The subsequent protocol-only test addition was reread before report closure: every job truncation, version/trailing bytes, combined 4096 namespace-plus-row boundary and one-over decoder refusal have concrete assertions. No actionable issue was found in that delta; the protocol hash below includes it. Its fresh six-test execution and final combined rerun are executor-owned and were pending when this review closed.

The fresh daemon Bazel packaging delta was also reread: its existing module tree is now included through `srcs`, an explicit crate root prevents multi-source inference ambiguity, and direct existing mempool/network dependencies match the daemon module imports. No new dependency or runtime behavior was introduced by this build declaration change. The recorded RPC build-file hash includes that delta; packaging execution remains separately owned.

During that reread, the missing direct existing `getrandom` dependency was reported to the orchestrator/executor: production `retry.rs:160` and `coins_flush.rs:136` invoke `getrandom::fill`. The executor added `@crate_index//:getrandom` to the binary target before this report closed. The corrected declaration was reread and the hash updated; no unresolved finding remains. Its fresh Bazel execution is still required independently of source review.

This report is scoped to the 166-04/15 defensive local-copy inspection foundation and controller. It does not accept the entire phase, establish canonical original-store admission, renew historical contracts, or replace the final 166-14 review and native verification. Fresh Cargo/Bazel packaging checks remain the implementation executor's responsibility. No build, test, Cargo, Bazel, network, source edit or Git mutation was performed by this reviewer.

## Source Assessment

- The application lease is retained by every guarded constructor and store clone. Engine handles precede the final lease field in declaration order; the coins adapter likewise drops its extracted handles before its lease-owning store. Existing source inspection acquires the pre-existing engine lock without creating or truncating it.
- `inspect_owned_then` retains both original leases through child completion, source identity/content recheck, recovered-tree checks, exact-root cleanup and proof callback. Reaped failures still attempt source recheck and checked cleanup. Uncertain completion refuses without chmod/removal. Cleanup failure prevents successful evidence from reaching the callback.
- The production controller selects only `current_exe()` with the exact reserved argv. The serialized job contains the owned scratch identity and nonce, never the original source path or lease. CLI dispatch precedes Clap; synchronous daemon dispatch precedes its Tokio application helper and configuration startup. Each required existing test root explicitly registers the reserved helper.
- Raw scratch recovery is reached through the worker route. The separate direct raw-pin call is confined to the controlled test characterization. The child installs its fixed non-payload panic-exit hook before raw work. Its keyspace/database/scratch-lease scopes end before success serialization; the parent requires successful exit in addition to complete records. No parent panic-hook change or parent raw-recovery fallback was found.
- Job, physical inventory and detached records have finite byte/count/depth/path bounds. The codec checks sizes before allocation, rejects truncation, duplicate names/keys, unknown framing/version, trailing bytes and mismatched job identity/nonce, and preserves absent namespaces, empty namespaces, missing rows and empty values. The stdin writer runs under parent monitoring; its completion and the child are observed before accepting output. The production deadline includes input delivery and live-child waiting.
- Root and result checks enforce canonical scratch placement, recorded device/inode, private modes, regular result type and single-link identity. Copies use fresh files rather than links. Source inventory checks identity, mode, mtime and content; atime is intentionally excluded. Recovered-tree type/link/cap checks precede permission changes on each visited entry, and root identity is checked before that traversal and before cleanup.
- Test-only faults are compiled under `cfg(test)` and use the exact worker route. Reviewed assertions cover actual detached values, malformed numeric engine names, source preservation, callback ordering, retained locks, timeout/input delivery, complete-but-live/nonzero output, protocol/result failures, changed source/root, cleanup failure, uncertain completion and concurrent distinct roots. Assertions were inspected rather than inferred from test names. No-op and no-job entrypoint cases establish dispatch only.

## Evidence Examined

Existing executor logs were read as supporting evidence; their outcomes were not rerun by this reviewer:

| Receipt under `.local/open-bitcoin-dev/phase166/plan15/` | Observed outcome and limit |
| --- | --- |
| `protocol-green.log` | Four protocol tests passed. This early protocol-only receipt contains staged unused-code warnings; later scoped lint evidence is separate. |
| `worker-final.log` | Eleven controller tests passed, zero failed; 31.21 seconds. |
| `harness-first.log` | Thirty-seven node inspection-filter tests passed, plus one entrypoint test in each CLI library, CLI binary and daemon binary harness. Unrelated filtered zero-test executables were not counted as behavioral evidence. |
| `clippy-first.log` | Scoped compilation/lint completed its dev profile; source and test assertions were reviewed independently of that receipt. |

The 166-CONTEXT decisions, process-isolation RESEARCH amendment, 166-04 foundation summary and 166-15 plan informed the boundary. The historical malformed-engine abort and APFS invalid-name setup limitation remain distinct from successful containment. This review makes no Linux/non-UTF8 runtime claim, arbitrary filesystem/direct-engine writer guarantee, or parent/system-crash, forced-termination, OOM or OS-crash-dump guarantee. Supported cooperative application openers and the actual engine lock are the accepted concurrency scope.

Repository guidance applied: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` (no concrete override), `standards/index.md`, architecture, code-shape, testing, verification and Rust standards. Both active lesson inputs were loaded completely: 7977 bytes and 2660 conservative estimated tokens. No project skill directories or `.claudeignore` were present. Ignored runtime logs were consulted only as explicitly requested evidence, not reviewed as source.

## Reviewed Source Identity

SHA-256 values captured from the actual working files at review time:

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs` | `4910583c507ef7ffd3b13d82b44f8cc57f9ef9222ad320f55bb094ee23f281c1` |
| `packages/open-bitcoin-node/src/storage/coins_view.rs` | `29c498d6950a3f07bd55012fd4d5bb9a362dd6efab5161c5091c09664b121280` |
| `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs` | `38bd884fdff0727e70028dffdfbc9348e69eb64dc47375a68f45cb5890821ea8` |
| `packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs` | `59d9836fa861b163e040f355bb68eb6c2e0491c9ef24cba6763184dadace787a` |
| `packages/open-bitcoin-node/src/storage/fjall_store/open_lease/tests.rs` | `c87f8c576713fe14d1e4225ed00aa8b78e94a7c6165c68f0fc2b60047f8fa5ec` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs` | `3e5d7429882f30b4367ee4bc8fb8689f228bbb937afe5820286a517dc3775305` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs` | `cf8974625ec90de4b5f1796cbe9d4e25c7c55b16ad7d0aefd100e1dcfa6c1c2c` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs` | `099865c9507a9fbcf48179b8157109bf50d1bc84b3d67fc1b69389893e004fef` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs` | `0ba8288008cd87dd2e4caaec804d7d170f59a5b9413fcc3871a0e070a22094ae` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs` | `259c5cd0e07fa800a3feb29d036f25d89d5976c5267eb2289399293937b2f455` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker/tests.rs` | `8d59aeede8ab8e2f912e577e4df7dda8dac74e17b896a7be46741823d29f5b43` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs` | `9d4f361aa23d4e231f634abc69bc4c1c4599a0a58255344b2eee5bf0f79ecb53` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests/copy.rs` | `3e7df9ce9cefe4aebcca25956d48e64df104a46264fa5af16432bf984c5fe1b8` |
| `packages/open-bitcoin-node/src/lib.rs` | `967917b034d829b65f0a21b3370ac05bd6d59ccb3a1bd19c3eb645a2bd9f3bc8` |
| `packages/open-bitcoin-cli/src/lib.rs` | `5c2bc79bba482d4a2cffd36bcca7f14a1c6aa10c1e8a4595e399c0c3bf1e27c5` |
| `packages/open-bitcoin-cli/src/bin/open-bitcoin.rs` | `6089fc7271838d24bf9637c85475b5002fb229d62143c8bd079f23ec1e084284` |
| `packages/open-bitcoin-cli/BUILD.bazel` | `9cca70e4483be7090fb8989676e855dbfaa8ea111f5e233da03771df489a3c16` |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | `04f44a484f78909abe88ef34995a0df07a63e8a816be9f6c3edb1fdb18c33ac5` |
| `packages/open-bitcoin-rpc/BUILD.bazel` | `2b5018114b91f1b34018d149c95f91383fa2e36ab059108864cd65db4db497b1` |
| `docs/parity/source-breadcrumbs.json` | `ed54aa6f8eac4bb009bd663be7b3c138fb41426482c814becdb2a24b10b8298a` |

Source changes after these identities require review of the changed paths. Final phase acceptance and all normal-hook Git saving remain with the orchestrator.

***

_Reviewer: gsd-code-reviewer_
_Depth: standard_

## Bootstrap boundary follow-up | 2026-10-20

The same independent reviewer returned a clean, read-only scoped review of completed 166-05/06 with no actionable findings. It checked the actual strict record inventory and genesis bindings, single prevalidated SyncAll publication, scratch cleanup before original opening, immediate live reproof, sealed ownership, and capability checks against instance identity, write sequence and full proof. History-owner initialization follows proof; generic opener contracts remain intact.

The reviewer inspected retained evidence showing four atomic, sixteen reopen and 562 storage tests passing, plus scoped Clippy. It made no edits and ran no builds/tests. This follow-up does not renew the earlier table's dated source hashes or establish full-phase acceptance; later runtime/index/adapters and final 166-14 review remain pending.

## Sealed RPC context follow-up | 2026-10-20

The independent reviewer returned a clean scoped review of166-16 with no actionable findings. It traced sealed by-value capability consumption, proof-derived parameters and datadir, unchanged ordinary/fixture wallet construction, unavailable-wallet/rescan refusal before effects, preserved descriptor-batch versus HTTP-scope protocols, RPC-library worker registration and current test ownership. The final three iterator-loop lint corrections preserve selector, preservation and retained-handle assertions.

The reviewer read the retained21/21 focused pass and original20/1 observer-lock failure, and made no claim that the original lock cause was proved. It made no edits or test/build calls. Final affected reruns, lint/static checks, native verification and whole-phase approval remain executor/root-owned; this does not renew earlier source-hash snapshots or replace the final166-REVIEW.md.

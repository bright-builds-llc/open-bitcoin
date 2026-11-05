---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T05:14:00Z"
commits: false
---

# Phase 167 Plan 13 context registrar amendment

This planning-only proposal adds one conventional RPC context registrar to original Plan13. The resulting nine-owner scope retains both original tasks, every action/acceptance/verification command, the complete29-plan/50-task/24-wave schedule and all lifecycle/requirement fields. It proposes a configuration-only specialized factory that internally acquires and consumes the genuine seal required by the original action. This interpretation needs independent review, actual accepted12 API verification and root release before any13 source work. It grants no approval or source/build/product acceptance.

## One necessary registrar

`packages/open-bitcoin-rpc/src/context.rs` declares `mod canonical_genesis;` but omits the already-planned sibling context/canonical_history.rs. The sibling is unreachable until registered. The entire source allowance is:

```diff
 mod canonical_genesis;
+mod canonical_history;
 mod filter_index;
```

The baseline registrar is501lines/16168bytes, SHA256 `f0fc6351d63d61c87ea17ebaa1080db7143918fafcddb77e3353c242dce6eb15`. The proposal adds one23-byte declaration, expected502lines/16191bytes, with no unrelated formatting, refactor, inline-path module, export or visibility change. An ignored isolated proposal must format below629lines without extra changes; its full inverse removes that declaration and restores the entire original hash. No checkout source is edited or formatted by this planner.

Only context.rs is added to frontmatter/Task1 files. Task1 becomes five files; Task2 retains its original five, sharing context/canonical_history/tests.rs with Task1. The union is nine distinct RPC owners. There is no new task, wave, dependency, requirement or phase: Plan13 remains wave16 after12. Other28plan files and all original13 action/behavior/acceptance/read-first/done/threat/interface/verification bytes remain exact. Any actual need for another owner or overlong formatted file stops for a checked amendment; no silent node export, source split, minification or file-limit waiver is authorized.

## Stable RPC seams and proposed proof acquisition

Source inspection establishes `context/network.rs::assemble_context` is pub(super) and `context/wallet_state.rs::WalletState` is pub(super) with Unavailable. A conventional sibling can use them without changing those files. Existing context/canonical_genesis.rs demonstrates consuming a genuine OpenedCanonicalGenesis through DurableSyncRuntime::open_canonical_genesis_configured, applying actual runtime configuration and assembling Unavailable wallet state. These are existing RPC seams; they do not prove the unfinished12 history API is accepted.

Public DurableSyncRuntime also has fixture constructors. `network.maybe_canonical_chain` reads an envelope selection, so a public runtime plus that label cannot establish earned history. The proposed specialized public factory is:

```text
ManagedRpcContext<FjallChainstateStore,FjallCoinsView>::
from_canonical_history_runtime_config(config: &RuntimeConfig)
  -> Result<(Self, DurableSyncRuntime), CanonicalContextError>
```

It takes CONFIGURATION ONLY. The factory must internally acquire a genuine fresh/reopened sealed producer result, immediately consume it through the actual safe configured node consumer using type inference, assemble the context with actual returned network/store/selected rules and WalletState::Unavailable, then return that actual runtime. Intermediate helpers/fields stay private. Caller-provided runtime/store/params/chain positions, booleans, numeric work, fixture owners, BASIC status and network labels are excluded as proof inputs. Do not add a node export or visibility owner merely to name a public seal whose containing module is crate-private.

This preserves the original action's security condition “accept only fresh/reopened sealed12 runtime” by internally obtaining and consuming the seal rather than exposing a public arbitrary-runtime parameter. The original action remains byte-exact. Independent review must assess this interpretation; it is not an authority waiver. Keep generic new/from_runtime_config/from_fixture paths strict, all existing mismatch/injection/activation refusals pre-effect, and every original no-wallet/rescan/file-creation check.

Current canonical_chain::try_from_runtime_config_with_store relies on require_ordinary_scope refusing before it builds arbitrary fixture/store network. Future14 exposes Regtest implementation capability, so that current global Err cannot serve as a permanent authority guard. Within already-owned Task1 canonical_chain.rs, retain existing selector/prerequisite diagnostics and independently return CanonicalContextError::UnprovedOwner for unsealed ordinary generic construction and its aliases before wallet/network/store effects, even when require_ordinary_scope succeeds. Explicit fixture constructors stay synthetic and cannot be promoted by network labels. Meaningful tests in the already-owned context/canonical_history/tests.rs must guard this distinction without implementing14, flipping support, accepting a caller boolean or adding an owner. The proposed sealed configuration-only factory is the separate route that genuinely acquires/consumes authority.

Fresh initialization uses the existing sealed canonical-genesis producer/consumer. Existing ordinary history must pass actual12's same-child full replay and fixed deadline, source/lease/cleanup gates, immediate complete original equality/reproof and consuming live-owner assembly. All four selector genesis paths remain strict and supported; unqualified ordinary selectors still refuse. Dispatch between fresh/genesis/history paths must follow checked producer contracts and immutable configuration, not a data label, permissive envelope test or broad fallback after proof refusal. If safe dispatch or a genuine configuration-only acquisition route cannot be demonstrated, stop before13 source work.

## In-progress12 API is a release dependency

Observed while Plan12 source was active, candidate names include `FjallNodeStore::reopen_canonical_history_existing(path, selected)` and `DurableSyncRuntime::open_canonical_history_configured(opened, config, mode)`. Their producer may return a public seal in node's pub(crate) canonical_genesis module. These names are observations, not accepted provider signatures, immutable hashes, compilation evidence or a guarantee that external type inference works. No active12 source was captured in this amendment's immutable source baseline.

Before ANY13 source release, root must first accept12. Then check its actual public producer/consumer signatures and external accessibility by inference, genuine selected/owner/network binding, full copied-child timing/source gates and immediate original proof/effect ordering. Use those accepted methods and actual owners. If unavailable, stop for checked planning rather than alter node exports, accept arbitrary runtime based on scope or use raw storage. Root's12 planning acceptance does not equal12 source/API acceptance. Final13 receipts must bind the accepted12 summary/API/source/evidence hashes actually used.

## Preserve the pinned genesis binary boundary

The existing test packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs:182–188 assigns open_authoritative_network_runtime to:

```text
fn(&RuntimeConfig, Option<open_bitcoin_node::storage::OpenedCanonicalGenesis>)
  -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError>
```

That source/test/signature remains strict and unchanged. Add a distinct history route inside already-owned canonical_startup.rs/open-bitcoind.rs which calls the configuration-only specialized factory, retaining the original genesis helper. Do not widen its argument, turn None into raw authority, replace its test or make a caller's arbitrary runtime acceptable. Keep all four-selector genesis startup/reopen commands and original refusal assertions.

The existing actual binary main dispatches maybe_run_inspection_worker before run_application/config work, and the existing worker macro handles unit-test executable routing. Preserve both, without another worker/binary or macro owner. The real process path is exactly `Command::new(env!("CARGO_BIN_EXE_open-bitcoind"))`, with the hyphen. Retain all original5automated commands, including the Cargo auto-discovered integration process and both genesis selections. No stale target/debug path or unit-test harness is daemon evidence.

The test-local serialized default-regtest history remains independent of future15 corpus. Keep isolated config/datadir, cleared inherited environment, loopback authentication, default-off external networking, shell-owned clock, bounded redacted diagnostics, valid submission/invalid refusal, unavailable wallet/no wallet files, graceful stop/reap/drop of every handle, no concurrent Fjall owner, fully stopped direct durable observations and same-executable/datadir restart. Compare actual accepted tip/hash/full work/coins/BASIC/provenance; preserve all source/nonmutation and required-context negatives. Local product evidence does not claim actual Knots differential success or later-phase qualification.

## Baseline and exact inverses

New ignored evidence is confined to `.local/open-bitcoin-dev/phase167/amend13registrar/`. Its before-manifest captures all29 ORIGINAL plan bytes/metadata, the stable RPC registrar and safe RPC provider/test/source-existence states,40protected phase documents including accepted11/28/29 summaries and all12R1 documents, and1735historical receipts including accepted11/28/29 root receipts and all12R1 planning/checker/acceptance artifacts. The canonical checker prefix is217153bytes, SHA256 `f734e6f31fe5f2eb2593358eeb75ef42d4fbfdabeb43d53107a10cb765f75ced`.

The immutable baseline explicitly EXCLUDES live12node sources, plan12 execution receipts/SUMMARY and root live STATE/ROADMAP/todo/config. Authorized12 changes are not corruption. Frozen historical copies inside planning receipts remain historical; no observation of the ongoing12 implementation is promoted to accepted API evidence. This planner never edits any12R1 document, receipt or report.

Original Plan13 is157lines/18071bytes, SHA256 `ee5bbd4be54a0d4f25b5ec2d545f3cf47730720b8775a4b99000c1f3b10c3db1`. The full plan inverse removes only the ninth frontmatter owner, Task1 files addition and new tagged context. Every original task action/acceptance/command and all other original bytes must return exactly; other28plan inverses are identity. Source inverse evidence uses an ignored proposed registrar only. Final execution must retain actual before/after source bytes, single declaration delta and whole-file inverse after scoped formatting.

Existing Plan13 already fails mdformat at baseline. Preserve its body; new amendment alone may be formatted with existing GFM/frontmatter-aware mdformat. Verify the normalized amended full inverse reconstructs the normalized original residual so no new formatting defect is hidden. Validate all29 frontmatters/task structures,29/50/24 inventory, lifecycle/dependencies/no same-wave overlap, complete contracts/inverses, protected bytes and source states. These are metadata checks only. The independent checker owns a separate append-only canonical result; no self-approval or report mutation is granted.

## Decision coverage and final obligations

Full means preserved planning coverage with original owners, not executed acceptance.

| Decisions | Plan/task                                                   | Coverage | Obligation                                                                                             |
| --------- | ----------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| D-01,D-07 | 13Task1/Task2                                               | Full     | Genuine internally acquired/consumed seal, selected qualification and strict genesis/fixture boundary. |
| D-08      | 13Task1/Task2                                               | Full     | Exact pre-effect refusal and actual authenticated binary stop/reopen/next state evidence.              |
| D-02–D-06 | Accepted prerequisites and original13 runtime/process paths | Full     | Actual selected candidate/script/work/maturity/count/undo rules reach the ordinary owner.              |
| D-09      | Both tasks and original17                                   | Full     | Attributable original/source/receipt inverses and independent evidence classes.                        |
| D-10      | Original13 commands and17/19/20                             | Full     | Source-before-guards, parity/docs and final native/fresh coverage/Bazel/security/goal/lifecycle.       |

The simplification pass chooses one conventional module declaration, existing RPC sibling-private assembly and internal seal acquisition; no public runtime-authority wrapper or node visibility expansion is introduced. All original13 requirements/commands/refusal controls remain.17/20compact-fallback adjudication and every final source/security/goal/native/fresh coverage/Bazel/lifecycle gate remain mandatory; no phase or whole-requirement acceptance follows.

AGENTS local guidance, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards inform the sibling layout, typed consuming boundary and evidence checks. Existing source-grounded internal discovery adds no dependency or research choice. GSD revision scope and root-only saving apply. No source/test/build edit, Cargo/Bazel run, shared metadata/summary/guard/report mutation, Git saving or13 release occurs in this planning task.

## Planner observations awaiting independent check

The isolated registrar proposal is502formatted lines/16191bytes, SHA256 `f15169d2d40a55245c6d5f0d50aa5ce07efc19ef461e59a793fb560ebf0dccc9`. Formatting makes no additional changes, and the complete single-declaration inverse restores the501-line original. The static inventory retains29plans/50tasks/24waves, all original13task contracts and all5commands, all29complete inverses, other28plans,14stable RPC source/existence states,40protected documents and1735historical receipts. Shared owners remain dependency-ordered with zero same-wave collisions; the sole inherited GSD advisory is Plan02's independent producer at a wave greater than1.

The initial validator used lowercase “before” when checking the existing uppercase “Before ANY13 source work” contract and reported that metadata-only mismatch. Its full receipt/script remain as planner-validation-first-case-check.json and validate-first-case-check.ts; the check now uses the actual text without changing the plan contract. The first Markdown check requested formatting only this new amendment; the original Plan13 body stays unchanged. Final metadata checks do not establish accepted12 API validity, source execution or independent approval. Source release remains conditional on independent review, actual12 acceptance/API verification and root adjudication.

The first normalized-inverse formatter check also exposed formatting of newly added unquoted generic signatures and a changed Markdown list-closure boundary. Its full metadata/log/draft copies remain under first-format/. Only new context was corrected: signatures use inline code, and the new block follows the entire unchanged original context instead of interrupting its list. No original plan body was normalized; the complete raw inverse remains the original root baseline.

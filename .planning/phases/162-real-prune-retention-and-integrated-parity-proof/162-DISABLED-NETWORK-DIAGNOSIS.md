---
status: resolved
artifact_role: historical_diagnostic_snapshot
snapshot_of: .planning/debug/resolved/phase162-disabled-sync-network.md
snapshot_at: 2026-10-11T19:31:01Z
trigger: "phase162-disabled-sync-network: parsed -regtest -blockfilterindex=basic with networking/sync disabled unexpectedly uses Mainnet runtime consensus/prune network"
created: 2026-10-11T17:40:28Z
updated: 2026-10-11T18:04:18Z
---

## Current Focus

hypothesis: Resolved by the checked Plan13 Disabled-arm correction; selected network and exact DNS defaults now survive disabled sync resolution without changing activation guards.
test: Read Plan13 executor summary and actual four-file source diff; executor records direct frozen-fixture RED/GREEN and passing loader, guard, config, daemon, format and lint checks.
expecting: Focused diagnostic is closed; this does not establish completion of the broader Phase162 integrated proof or native verification contract.
next_action: Parent resumes Phase162 execution and completes outstanding integrated/native/lifecycle gates; no further diagnostic-agent source or Cargo work is authorized.

## Symptoms

expected: Actual parsed -regtest -blockfilterindex=basic with networking/sync disabled retains Regtest runtime consensus and prune network.
actual: Strong Plan02 fixture assertion finds config.sync.runtime.network Mainnet and params pow_limit_compact=0x1d00ffff, allow_min_difficulty_blocks=false, no_retargeting=false; actual Regtest recipe expects 0x207fffff,true,true (both maturity100).
errors: phase162_daemon_fixture in daemon tests/filter_index/phase162/fixtures.rs around172 failed; parent reports last run 1 failed, 0 passed,13.24s.
reproduction: Parse Phase162 fixture daemon args with -regtest -blockfilterindex=basic and disabled networking/sync; compare parsed authoritative runtime against independent Regtest recipe.
started: Discovered by stronger Phase162 Plan02 actual-regtest assertion during lifecycle162-2026-10-11T16-25-37; earlier interim worker passes do not establish network correctness.

## Eliminated

- hypothesis: CLI -regtest is misparsed or determine_chain chooses Mainnet.
  evidence: loader.rs parse_cli_args maps regtest through parse_chain_key; determine_chain returns Some CLI chain unchanged; loader passes that exact chain into resolve_daemon_sync_config and RuntimeConfig.chain.
  timestamp: unrecorded
- hypothesis: The fixture uses a custom incorrect Regtest recipe or the authoritative consumer independently resets a correct runtime config.
  evidence: Fixture seed uses SyncNetwork::Regtest.consensus_params; before opening store, parsed config consensus equality fails. Authoritative runtime consumes runtime.sync.runtime.clone unchanged and separately sets prune network from that same field.
  timestamp: unrecorded

## Evidence

- timestamp: unrecorded
  checked: Parent handoff and authorization.
  found: Diagnosis-only; own this debug file only. No production edits, Cargo, staging, commits, or changes to phase registry/STATE/ROADMAP/requirements. Existing reported failure is the reproduction signal.
  implication: Investigate source path and test coverage, then propose prerequisite execution plan rather than apply fix.
- timestamp: unrecorded
  checked: Active lessons and debug knowledge base.
  found: Global lessons5230bytes and repo lessons1958bytes,total7188bytes,estimated2397tokens; both read completely. No knowledge-base match with at least two symptom keyword overlaps. Local guidance, sidecar, overrides, architecture/testing/verification/Rust standards loaded.
  implication: Preserve exact environmental evidence; use repo-native verification and timing wrapper when root later authorizes execution. No lesson edits are in scope.
- timestamp: unrecorded
  checked: config/loader.rs::load_runtime_config_for_args and parse_cli_args; config/loader/chain.rs::determine_chain.
  found: CLI flag aliases or -chain= names select Mainnet/Testnet/Signet/Regtest, CLI chain wins over global bitcoin.conf chain flags; no selection defaults to Mainnet. Loader passes selected chain to resolve_daemon_sync_config and independently retains it in RuntimeConfig.chain and RPC ports.
  implication: This is a divergence between two network-bearing configuration fields, not a parser failure.
- timestamp: unrecorded
  checked: config/loader/open_bitcoin_runtime.rs::resolve_daemon_sync_config, daemon_sync_config_for_mode, resolve_config_sync_mode, apply_sync_overrides.
  found: Missing JSONC, default JSONC, explicit JSONC disabled/off, explicit CLI disabled/off, and CLI-disabled override of enabled JSONC converge on DaemonSyncConfig::disabled. CLI mode takes precedence and skips JSONC mode-consistency resolution. Overrides only apply when mode.is_enabled. MainnetIbd explicitly rejects every selected non-mainnet network.
  implication: Fix disabled construction only; preserve CLI precedence, disabled override behavior, no-peer-override application in disabled mode, partial-config errors, and mainnet-only activation.
- timestamp: unrecorded
  checked: config.rs::DaemonSyncConfig::disabled/default; node sync/types.rs::SyncRuntimeConfig::default and SyncNetwork methods.
  found: No-argument defaults intentionally mean Mainnet; runtime default also includes Mainnet DNS seeds. All four supported SyncNetwork variants already provide consensus params, magic, default port, prune-after height, and default_dns_seeds. Regtest default_dns_seeds is empty.
  implication: Assigning runtime.network alone leaves an incoherent Mainnet seed list. The existing producer must also replace dns_seeds with selected-network defaults, without public constructor/API or global default changes.
- timestamp: unrecorded
  checked: bin/open-bitcoind.rs::open_authoritative_network_runtime; node sync/open_runtime.rs::open_with_configured_runtime_activation; sync/progress.rs::local_peer_config.
  found: Durable open takes runtime.sync.runtime unchanged, derives local wire magic and consensus_params from its network, and daemon sets prune network from it. Transient open derives wire magic and default port from the same field. Durable ownership can be triggered by inbound, prune, or explicit index settings even when outbound sync is disabled.
  implication: User-visible effects extend beyond filters to non-mainnet durable validation/pruning and transient wire identity; one producer fix restores their shared selected-network contract.
- timestamp: unrecorded
  checked: bin/open_bitcoind/sync_seed.rs::preflight_daemon_sync and bin/open-bitcoind.rs::start_daemon_sync_worker.
  found: Both return early when sync is disabled. Inbound, relay, block-serving, compact-relay, BASIC indexing, and peer-filter serving have separate resolved flags; network identity does not activate them.
  implication: Preserving the selected network and seeds does not activate syncing, sockets, relay, serving, or DNS resolution. DNS strings are candidate configuration only.
- timestamp: unrecorded
  checked: config/tests/sync_configuration.rs, config/tests/baseline_and_auth.rs, bin/open_bitcoind/tests/daemon_sync.rs, Phase162 fixtures.rs.
  found: Existing tests assert MainnetIbd network, bounds/peers, disabled flags, partial JSONC refusal, regtest enabled refusal, and disabled preflight. They omit non-mainnet disabled runtime.network/seeds. Strong Phase162 fixture compares the parsed runtime consensus against actual seeded Regtest params before authoritative open and exposes the omission.
  implication: Add loader regression coverage for all supported networks and disabled entry paths; retain fixture equality and independent recipe unchanged.
- timestamp: unrecorded
  checked: CLI operator/config.rs::resolve_operator_config, select_network, load_runtime_from_evidence; context/prune.rs::prune_after_height.
  found: Operator CLI/env/onboarding-network evidence eventually becomes -chain=<name> passed to the same RPC loader. No separate daemon bootstrap sync producer was found; configless startup uses the same disabled resolver. RPC manual prune already maps RuntimeConfig.chain correctly while daemon automatic prune uses runtime.sync.runtime.network.
  implication: No operator/bootstrap producer changes are needed. The loader fix also makes their returned runtime configuration coherent; it closes manual-versus-automatic prune network disagreement.
- timestamp: unrecorded
  checked: Checked Plan13 162-13-SUMMARY.md and actual source diff, read during closure after measured clock 2026-10-11T18:04:18Z.
  found: This diagnostic agent directly inspected the diff: only production change is the Disabled arm mapping selected chain and replacing both runtime.network and runtime.dns_seeds. Existing tests add four-network loader/guard and actual durable/transient consumer coverage. Checked legacy blockfilter helper independently derives expected network/default DNS from config.chain while retaining full sync equality and activation assertions.
  implication: Implementation matches the diagnosed producer defect without expanding public API, network authority, enabled-sync support, or production consumer scope. Source inspection is direct; the following test outcomes are executor-recorded, not personally rerun by this diagnosing agent.
- timestamp: unrecorded
  checked: Plan13 executor-recorded serial timing receipts summarized in 162-13-SUMMARY.md.
  found: Executor directly ran frozen fixture RED at 17:54:28.078Z (0 passed, 1 failed, 13.39s), then GREEN at 17:55:34.920Z (1 passed, 15.00s). Loader went from 1 passed/4 failed to 5/5; mode guards 2/2; final config suite 91/91; daemon sync 14/14 including one test containing all eight four-network durable/transient consumer cases. Scoped all-target/all-feature RPC Clippy with -D warnings, rustfmt check, parity breadcrumbs, managed standards checks, and diff check passed.
  implication: Executor measured reproduction and regression evidence resolves this focused defect. The earlier parent-reported 13.24s failure is separate evidence. No Cargo/tests were run by the diagnostic agent, and full Phase162/native/integrated proof remains pending.

## Resolution

root_cause: daemon_sync_config_for_mode receives the correctly selected AddressNetwork but Disabled returns DaemonSyncConfig::disabled(), whose no-argument Mainnet default overwrites the selected runtime network. Disabled means no outbound sync activation, but runtime.sync.runtime remains authoritative for durable consensus/prune and transient wire identity.
fix: Applied by checked Plan13 executor, directly reviewed here. The existing Disabled producer arm maps AddressNetwork to SyncNetwork and assigns runtime.network plus exact selected-network default DNS seeds; public defaults, MainnetIbd refusal, override guards and activation policies remain unchanged. Regression tests cover loaders/guards and actual consumers. Checked legacy blockfilter helper now derives expected disabled network/default DNS independently from config.chain instead of assuming Mainnet while retaining complete sync equality and activation assertions.
verification: Focused fix verified by Plan13 executor-recorded direct frozen-fixture RED (0/1) then GREEN (1/1), loader 5/5, mode guards 2/2, config 91/91, daemon sync 14/14 including actual eight-case consumer matrix, strict scoped Clippy/rustfmt/parity/managed/diff checks. Diagnostic agent read summary and actual source diff but did not run Cargo/tests or edit production. Parent authorized focused closure. Full Phase162 native verifier, integrated parity/prune proof and lifecycle gates remain pending; no broader completion claim.
files_changed: [packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs, packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs, packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs, packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs]

## Entry Mode Matrix

| Entry | Effective mode | Pre-fix behavior | Required contract |
| --- | --- | --- | --- |
| No JSONC / configless bootstrap | Disabled | Mainnet runtime for every selected chain | Preserve selected chain plus network DNS defaults |
| JSONC default, mode disabled/off and network_enabled false | Disabled | Same defect | Same preservation |
| CLI -openbitcoinsync=disabled/off | Disabled | Same defect; CLI wins | Same preservation, override precedence unchanged |
| CLI disabled/off over enabled MainnetIbd JSONC on non-mainnet chain | Disabled | Allowed but loses chain | Preserve chain; remain allowed and ignore enabled-only peers/bounds |
| JSONC network_enabled true, mode mainnet-ibd; or CLI mainnet-ibd | MainnetIbd | Mainnet supported; every non-mainnet rejected | Unchanged |
| Partial JSONC activation without CLI override | Error | Requires mode and network_enabled together | Unchanged |
| RuntimeConfig/DaemonSyncConfig/SyncRuntimeConfig defaults with no selected chain | Disabled/Mainnet | Consistent Mainnet | Unchanged |

## Recommended Regression Matrix

1. In existing config/tests/sync_configuration.rs: table-driven four networks (Mainnet,Testnet,Signet,Regtest) across missing JSONC, default/explicit disabled JSONC (cover off alias), explicit CLI disabled/off, and CLI-disabled override of enabled JSONC. For each assert RuntimeConfig.chain, sync Disabled, sync.runtime.network, exact selected default DNS seeds (Regtest empty), derived consensus/magic/default port/prune height, and unchanged activation flags. Include CLI chain flags, -chain= names, bitcoin.conf global chain flags, and CLI-over-conf precedence without making every combination a separate slow fixture.
2. Keep a configless Mainnet bootstrap case equal to existing defaults. Verify disabled-mode config peer/bound overrides remain ignored; enabled Mainnet peer/bound tests remain authoritative.
3. Expand existing non-mainnet MainnetIbd refusal to Testnet/Signet/Regtest; cover CLI and JSONC activation. Preserve partial JSONC rejection and CLI precedence.
4. Existing daemon test module: parse actual disabled configs and inspect durable runtime.config().network plus network_info().network_magic; inspect transient network_info magic where no durable trigger exists. Assert preflight None and worker None without starting a socket. Use existing observable prune behavior/Phase162 proof instead of introducing public test getters or authority APIs.
5. Re-run the original Phase162 fixture with its strong recipe consensus assertion unchanged; execute the integrated prune-retention paths that distinguish Mainnet100000 versus non-mainnet1000 guard. Treat earlier interim passes as superseded.
6. Later authorized verification must use command-timings wrapper for focused Cargo, then the repo-native default bash scripts/verify.sh contract before completion/commit; no overlapping Cargo target-directory work.

## Scope and Simplification Review

The change is exclusively selected-network and derived-default preservation for Disabled mode, across all already supported networks. It restores broader consumers' existing runtime contracts without expanding enabled-sync support, altering policy, redesigning authority, changing independent recipe params, adding dependencies, or changing public constructors. One local enum match and existing default_dns_seeds method is simpler and safer than patching durable/transient consumers separately or introducing a new public constructor. Suggested production scope is one existing function in config/loader/open_bitcoin_runtime.rs; regression scope is existing config/tests/sync_configuration.rs and optionally existing bin/open_bitcoind/tests/daemon_sync.rs. New source files and parity registry entries are unnecessary for that scope.

## Metadata Correction | 2026-10-11T17:46:28Z

- The original 16:52/16:56 times were unsupported and have been removed. Source-reading and hypothesis-elimination event times were not separately measured; their timestamp fields now explicitly say unrecorded. The previous 17:42:09 clock reading was not an event measurement for those findings and has also been removed from them.
- created is grounded in macOS stat filesystem birthtime: birth_unix 1791740428, rendered as 2026-10-11T12:40:28-0500, equivalent to 2026-10-11T17:40:28Z.
- updated records the clock tool reading 2026-10-11T17:46:28Z obtained immediately before this metadata correction. Filesystem mtime before correction was 2026-10-11T12:43:16-0500; it does not establish individual source-investigation times.
- Test provenance remains unchanged: the failing Phase162 assertion and run counts were parent-reported. This diagnosing agent performed read-only source inspection and did not run Cargo or tests, apply a fix, or verify a fix.

## Focused Closure | 2026-10-11T18:04:18Z

- Closure timestamp is the actual clock-tool reading at the start of this closure review; individual source-read event times were not separately measured.
- Parent explicitly authorized closing and moving only this diagnostic after checked Plan13 completion. Executor-owned fix/test outcomes are attributed above; diagnostic-agent work was limited to reading summary/diff and updating/moving this ignored debug artifact.
- No source, STATE, ROADMAP, requirements, registry, knowledge-base, staging, commit, or push changes were performed by this diagnostic agent. No fix commit exists under the parent-deferred Git policy.
- Full Phase162, default bash scripts/verify.sh and integrated proof remain pending and parent-owned.

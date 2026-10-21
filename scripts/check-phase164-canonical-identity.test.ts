import { expect, test } from "bun:test";
import { checkPhase164, loadInputs } from "./check-phase164-canonical-identity.ts";
import { EDGES, FIXTURE_ATTRIBUTION_MUTATIONS, FIXTURE_INVERSE_CASES, maybeRestoreReviewedSource, type ExactSourceInverse } from "./check-phase164-canonical-identity/contracts.ts";
import { fixtureSourceAttribution, historicalFixtureView, MIGRATED_FIXTURE } from "./check-phase164-canonical-identity/fixture-source.ts";
import { createHash } from "node:crypto";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import "./check-phase164-canonical-identity/historical.test.ts";

import { CLI_MODULES, expectCopiedAdapterCase, COPIED_ADAPTER_MUTATIONS } from "./check-phase165-shared-exact-work/test-support.ts";
import { checkPhase165, loadInputs as load165Inputs } from "./check-phase165-shared-exact-work.ts";
import { checkPhase166, loadInputs as load166Inputs, DAEMON as CANONICAL_STARTUP, OBSERVER as CANONICAL_OBSERVER } from "./check-phase166-canonical-bootstrap.ts";
const real = new Map([...load166Inputs(), ...loadInputs()]);

test("phase165 C fixture attribution exact reviewed current source", () => {
  // Arrange / Act: actual edited fixture, never handcrafted current source.
  const source = real.get(MIGRATED_FIXTURE)!;
  const result = fixtureSourceAttribution(source);
  const sha = (value: string) => createHash("sha256").update(value).digest("hex");
  // Assert: current, earlier explicit fixture and original claims stay distinct.
  expect(result.evidenceClass).toBe("current-explicit-fixture");
  expect(result.currentHash).toBe(sha(source));
  expect(result.historicalHash).toBe("faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615");
  expect(sha(result.historicalSource)).toBe(result.historicalHash);
  expect(fixtureSourceAttribution(result.historicalSource).evidenceClass).toBe("original-historical");
  const older = result.historicalSource.replace("use crate::{AuthoritativeNetworkRuntime, open_authoritative_network_runtime, open_runtime_store};", "use crate::AuthoritativeNetworkRuntime;\nuse crate::tests::fixture_runtime::{\n    fixture_open_authoritative_network_runtime as open_authoritative_network_runtime,\n    fixture_open_runtime_store as open_runtime_store,\n};").replace("ManagedRpcContext::from_runtime_config_with_network_handle(", "ManagedRpcContext::from_fixture_runtime_config_with_network_handle(");
  expect(sha(older)).toBe("933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18");
  expect(fixtureSourceAttribution(older).evidenceClass).toBe("current-explicit-fixture");
  const view = historicalFixtureView(real);
  expect(view).not.toBe(real);
  expect(view.get(MIGRATED_FIXTURE)).toBe(result.historicalSource);
  expect(real.get(MIGRATED_FIXTURE)).toBe(source);
});
for (const [before, after] of FIXTURE_ATTRIBUTION_MUTATIONS) {
  test(`phase165 C fixture attribution refuses ${after}`, () => {
    // Arrange: every control starts from authenticated actual current wiring.
    const source = real.get(MIGRATED_FIXTURE)!;
    expect(fixtureSourceAttribution(source).evidenceClass).toBe("current-explicit-fixture");
    expect(source.split(before).length).toBe(2);
    const changed = source.replace(before, after);
    // Act / Assert: unproved returns the complete raw input and its own hash.
    const result = fixtureSourceAttribution(changed);
    expect(result.evidenceClass).toBe("unproved");
    expect(result.historicalSource).toBe(changed);
    expect(result.historicalHash).toBe(result.currentHash);
    expect(result.currentHash).toBe(createHash("sha256").update(changed).digest("hex"));
  });
}

test("phase165 downstream fixture guard unchanged actual source", () => {
  expect(checkPhase164(real)).toEqual([]);
});
for (const [file, symbol, before, after] of [
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "empty_runtime", "Chainstate::new()", "Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None)"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "empty_runtime", "DurableSyncRuntime::from_live_fixture_source(", "DurableSyncRuntime::open("],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "empty_runtime", "BasicFilterStartupMode::Disabled", "BasicFilterStartupMode::Enabled"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "empty_runtime", ".try_admit_fixture_headers(headers, 1_777_225_155, params)", ".seed_header_store(headers)"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_headers_sync_case", "headers_script(2, vec![grandchild]),", "headers_script(0, vec![genesis]),"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_headers_sync_case", "summary.headers_received != 1", "false"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_headers_sync_case", "summary.messages_processed != 3", "false"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_block_connect_case", ".maybe_connected_block_hash\n        .is_some()", "false"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_block_connect_case", "summary.blocks_received != 1", "false"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_block_connect_case", "snapshot.tip().is_none_or(|tip| {", "false && snapshot.tip().is_none_or(|tip| {"],
  ["packages/open-bitcoin-bench/src/cases/sync_runtime.rs", "run_block_connect_case", ".load_block(genesis_hash)", ".load_block(BlockHash::default())"],
  ["packages/open-bitcoin-bench/src/cases/storage_recovery.rs", "run_restart_recovery_case", "header_store.entries().count() != header_entries.len()", "false"],
  ["packages/open-bitcoin-bench/src/cases/storage_recovery.rs", "run_restart_recovery_case", "header_store.entry(&entry.block_hash) != Some(entry)", "false"],
  ["packages/open-bitcoin-bench/src/cases/storage_recovery.rs", "run_restart_recovery_case", "header_store.best_tip().is_some()", "false"],
  ["packages/open-bitcoin-bench/src/cases/storage_recovery.rs", "run_restart_recovery_case", "header_store.best_height() != -1", "header_store.best_height() != 1"],
  ["packages/open-bitcoin-bench/src/cases/storage_recovery.rs", "run_restart_recovery_case", "!header_store.best_chain_entries().is_empty()", "false"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "basic_params", "coinbase_maturity: 1", "coinbase_maturity: 100"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "open_basic_source", ".connect_block(", ".seed_coins("],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "open_basic_source", "            &[],", "            &future_headers,"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "open_basic_source", "DurableSyncRuntime::from_live_fixture_source(", "DurableSyncRuntime::open("],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "durable_basic_runtime_with_policy", "admit_basic_headers(&runtime, &blocks[1..], maybe_replacement.as_ref());", "network.try_admit_fixture_headers(headers, 2_000_000_000, basic_params()).expect(\"repeat bind\");"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "WireNetworkMessage::Version(VersionMessage {", "WireNetworkMessage::Headers(VersionMessage {"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "WireNetworkMessage::Verack,", "WireNetworkMessage::Headers(HeadersMessage { headers: vec![root.header.clone()] }),"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "maybe_replacement.map_or(&[][..], std::slice::from_ref)", "&[][..]"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "headers: batch.iter().map(|block| block.header.clone()).collect(),", "headers: future.iter().chain(maybe_replacement).map(|block| block.header.clone()).collect(),"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "ScriptVerifyFlags::P2SH,\n                basic_params(),", "ScriptVerifyFlags::P2SH,\n                Default::default(),"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "admit_basic_headers", "let entries = network.header_entries().expect(\"accepted exact headers\");", "runtime.store().save_header_entries(&future_headers, PersistMode::Sync).expect(\"raw future headers\"); let entries = future_headers;"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "durable_basic_runtime_with_policy", "network.connect_stored_block(replacement, fixture_work(),", "network.connect_local_block(replacement,"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "durable_basic_runtime_with_policy", "BlockConnectDisposition::NonExtending {", "BlockConnectDisposition::Duplicate {"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "durable_basic_runtime_with_policy", "for block in blocks.iter().skip(1) {", "for block in blocks.iter().skip(2) {"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "durable_basic_runtime_with_policy", "admit_basic_headers(&runtime, &blocks[1..], maybe_replacement.as_ref());", "network.connect_stored_block(maybe_replacement.as_ref().expect(\"B\"), fixture_work(), 2_000_000_000, ScriptVerifyFlags::P2SH, basic_params()).expect(\"body before header admission\");"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "fixture_work", "ProofWork::ZERO", "ChainPosition::untrusted_fixture_work(999999)"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "replace_basic_tip", ".load_block(block_hash(&basic_replacement(blocks).header))", ".load_block(displaced)"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "replace_basic_tip", "chain_work: fixture_work(),", "chain_work: ChainPosition::untrusted_fixture_work(999999),"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "replace_basic_tip", "        before_tip.chain_work\n", "        fixture_work()\n"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "assert_raw_reopen_refuses_work", "ChainstateError::UntrustedWorkLineage", "ChainstateError::BlockNotFound"],
  ["packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs", "assert_raw_reopen_refuses_work", ".expect(\"unchanged physical state\"),", ".expect(\"unchanged physical state\").without_coins(),"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_genuine_catchup_current_lag_ahead_reopen", "assert_consumers(&sandbox, &runtime, \"catching\");", "drop(runtime); let runtime = DurableSyncRuntime::open(store, config).expect(\"raw reopen before later mutation\");"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_genuine_catchup_current_lag_ahead_reopen", "assert_raw_reopen_refuses_work(&runtime);", "// raw refusal omitted"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", "assert!(!runtime.store().has_block(pruned).expect(\"body absence\"));", "assert!(true);"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", "assert!(!runtime.store().has_undo(pruned).expect(\"undo absence\"));", "assert!(true);"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", "assert_wallet_refusal(&runtime);", "// wallet refusal omitted"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", "WireNetworkMessage::NotFound(", "WireNetworkMessage::Block("],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", ".basic_filter_query(displaced)", ".basic_filter_query(BlockHash::default())"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives", "assert_raw_reopen_refuses_work(&runtime);", "// final raw refusal omitted"],
] as const) {
  test(`phase165 downstream fixture guard ${file}:${symbol}:${before}`, () => {
    // Arrange: select the actual owner before replacing its confirmed executable anchor.
    expect(checkPhase164(real)).toEqual([]);
    const source = real.get(file)!;
    const owner = regions(source).find(fn => fn.symbol === symbol)!;
    expect(owner).toBeDefined();
    expect(owner.raw.includes(before)).toBe(true);
    const changed = owner.raw.replace(before, after);
    expect(changed).not.toBe(owner.raw);
    const inputs = new Map(real).set(file, source.slice(0, owner.start) + changed + source.slice(owner.end));
    // Act / Assert: source controls complement the executed Rust behavior.
    expect(checkPhase164(inputs).some(row => row.file === file && row.rule === "boundary")).toBe(true);
  });
}

const N = "packages/open-bitcoin-node/src/";
const R = "packages/open-bitcoin-rpc/src/";
const C = "packages/open-bitcoin-cli/src/operator/";

test("phase165 live fixture guard actual current source", () => {
  // Arrange / Act / Assert
  expect(checkPhase164(real)).toEqual([]);
});

for (const [file, symbol, before, after] of [
  [N + "sync/live_fixture.rs", "pure_source_gate", "core.validate_live_origin().map_err(invalid)?;", "// origin omitted"],
  [N + "sync/live_fixture.rs", "pure_source_gate", "core.bind_live_work_source(params)", "core.bind_live_work_source(ConsensusParams::default())"],
  [N + "sync/live_fixture.rs", "canonical_root", "CanonicalChain::Testnet3,", "CanonicalChain::Regtest,"],
  [N + "sync/live_fixture.rs", "capture_live_fixture_source", "let binding = pure_source_gate(source, params)?;", "let binding = source.chainstate().bind_live_work_source(params).map_err(invalid)?;"],
  [N + "sync/live_fixture.rs", "capture_live_fixture_source", "let expected = replay.snapshot();", "let expected = source.chainstate().overlay_snapshot();"],
  [N + "sync/live_fixture.rs", "capture_live_fixture_source", "ScriptVerifyFlags::P2SH,", "ScriptVerifyFlags::NONE,"],
  [N + "sync/live_fixture.rs", "read_validation", "StoredValidationStatus::ScriptsValid", "StoredValidationStatus::KnownOnly"],
  [N + "sync/live_fixture.rs", "check_content", "physical != captured.coins", "physical != core.coins().complete_snapshot().map_err(invalid)?"],
  [N + "sync/live_fixture.rs", "check_content", "read_validation(store, &captured.expected)? != captured.validation", "false"],
  [N + "sync/live_fixture.rs", "check_content", "store.load_all_undo_records()? != captured.expected.undo_by_block", "false"],
  [N + "sync/live_fixture.rs", "check_captured_live_fixture_source", "check_content(source, captured)", "Ok(())"],
  [N + "sync/live_fixture.rs", "initialize_live_fixture_source_graph", "network.try_admit_fixture_headers(headers, current_time, captured.params)?;", "// headers omitted"],
  [N + "sync/live_fixture.rs", "initialize_live_fixture_source_graph", "network.retain_checked_candidate(root, captured.params, current_time)?;", "// root linkage omitted"],
  [N + "sync/live_fixture.rs", "initialize_live_fixture_source_graph", "network.retain_cached_candidate(block, captured.params, current_time)?;", "network.retain_checked_candidate(block, captured.params, current_time)?;"],
  [N + "sync/live_fixture.rs", "from_live_fixture_source", "check_captured_live_fixture_source(network.chainstate(), &captured)?;", "// final immutable comparison omitted"],
  [N + "sync/live_fixture.rs", "from_live_fixture_source", "initialize_live_fixture_source_graph(&mut network, &captured)?;\n        check_captured_live_fixture_source(network.chainstate(), &captured)?;", "check_captured_live_fixture_source(network.chainstate(), &captured)?;\n        initialize_live_fixture_source_graph(&mut network, &captured)?;"],
  [N + "sync/open_runtime.rs", "prepare_initialized_network", "network.record_basic_indexing_configuration(maybe_configured_basic_indexing);", "// configured BASIC state omitted"],
  [N + "sync/open_runtime.rs", "finish_initialized_runtime", "network.initialize_basic_filter_index_owner()?;", "// normal BASIC owner omitted"],
] as const) {
  test(`phase165 live fixture guard ${file} ${symbol} ${before}`, () => {
    // Arrange: mutate the real loaded owning function, after its unchanged positive baseline.
    expect(checkPhase164(real)).toEqual([]);
    const source = real.get(file)!;
    expect(source).toBeDefined();
    const maybeOwner = regions(source).find(fn => fn.symbol === symbol);
    expect(maybeOwner).toBeDefined();
    const owner = maybeOwner!;
    expect(owner.raw.includes(before)).toBe(true);
    expect(before).not.toBe(after);
    const changed = owner.raw.replace(before, after);
    const inputs = new Map(real).set(file, source.slice(0, owner.start) + changed + source.slice(owner.end));
    // Act / Assert: source boundary proof, separate from the executed Rust controls.
    expect(checkPhase164(inputs).some(row => row.file === file && row.rule === "boundary"
      && row.detail.includes(symbol))).toBe(true);
  });
}

for (const file of [
  R + "bin/open_bitcoind/tests/phase165_work.rs",
  "packages/open-bitcoin-rpc/tests/phase165_work_startup.rs",
  C + "runtime/chain_compatibility/tests/phase165_work.rs",
  C + "wallet/tests.rs",
  C + "soak/tests/recovery_resume_bounds.rs",
]) {
  test(`adapter evidence file cannot be disabled with an inner cfg: ${file}`, () => {
    // Arrange
    const inputs = new Map(real).set(file, "#![cfg(any())]\n" + real.get(file)!);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "ownership")).toBe(true);
  });
  test(`inner-attribute text in comments does not disable adapter evidence: ${file}`, () => {
    // Arrange
    const inputs = new Map(real).set(file, "// #![cfg(any())]\n" + real.get(file)!);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings).toEqual([]);
  });
}

for (const [file, symbol] of EDGES.filter(row => row[1].startsWith("phase165_") && (
  row[0] === R + "bin/open_bitcoind/tests/phase165_work.rs"
  || row[0] === "packages/open-bitcoin-rpc/tests/phase165_work_startup.rs"
  || row[0] === C + "runtime/chain_compatibility/tests/phase165_work.rs"
  || row[0] === C + "wallet/tests.rs"
  || row[0] === C + "soak/tests/recovery_resume_bounds.rs"
))) {
  for (const attributes of ["", "#[test]\n#[ignore]\n", "#[test]\n#[cfg(any())]\n"]) {
    test(`adapter acceptance test cannot be removed, ignored or disabled: ${file}:${symbol}:${attributes}`, () => {
      // Arrange
      const before = `#[test]\nfn ${symbol}(`;
      expect(real.get(file)?.includes(before)).toBe(true);
      const inputs = mutation(file, before, `${attributes}fn ${symbol}(`);
      // Act
      const findings = checkPhase164(inputs);
      // Assert
      expect(findings.some(row => row.file === file && row.rule === "ownership")).toBe(true);
    });
  }
}

for (const [file, before, after] of [
  [R + "bin/open_bitcoind/tests.rs", 'mod phase165_work;', '/* mod phase165_work; */'],
  [R + "bin/open_bitcoind/tests.rs", 'mod phase165_work;', '#[cfg(any())]\nmod phase165_work;'],
  [R + "bin/open_bitcoind/tests.rs", 'tests/phase165_work.rs', 'other/phase165_work.rs'],
  [C + "runtime/chain_compatibility/tests.rs", 'pub(crate) mod phase165_work;', ''],
  [C + "runtime/chain_compatibility/tests.rs", 'pub(crate) mod phase165_work;', '/* pub(crate) mod phase165_work; */'],
  [C + "runtime/chain_compatibility/tests.rs", 'pub(crate) mod phase165_work;', '#[cfg(any())]\npub(crate) mod phase165_work;'],
]) {
  test(`adapter work registration rejects deletion, redirection and disabled decoys: ${file}: ${after}`, () => {
    // Arrange
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(real.get(file)?.includes(before)).toBe(true);
    expect(findings.some(row => row.file === file && row.rule === "ownership")).toBe(true);
  });
}

for (const [file, before, after] of [
  [CANONICAL_STARTUP, "FjallNodeStore::open_canonical_genesis(data_dir, chain)", "FjallNodeStore::open(data_dir)"],
  [R + "bin/open_bitcoind/tests/phase165_work.rs", "assert_eq!(manifest(&path), before);", "assert!(true);"],
  [R + "bin/open_bitcoind/tests/phase165_work.rs", "crate::open_runtime_store(&runtime)", "Ok(None)"],
  [R + "bin/open_bitcoind/tests/phase165_work.rs", '"work_decision={decision}"', '"observed_work=unavailable"'],
  [C + "runtime/chain_compatibility/tests/phase165_work.rs", "execute_operator_cli_with_default_data_dir(cli, fixture.root.clone())", "OperatorCommandOutcome::success(\"\")"],
  [C + "runtime/chain_compatibility/tests/phase165_work.rs", "assert_eq!(fixture.snapshot(), before,", "assert_ne!(fixture.snapshot(), before,"],
]) {
  test(`actual adapter work evidence rejects changed route or assertion: ${file}: ${before}`, async () => {
    // Arrange
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "boundary")).toBe(true);
    if (file === CANONICAL_STARTUP) expect((await checkPhase166(inputs)).some(row => row.file === file && row.rule === "daemon-sealed-opener")).toBe(true);
  });
}

for (const [file, before, after] of [
  [N + "storage/chain_compatibility.rs", "mod phase165_work;", ""],
  [N + "storage/chain_compatibility.rs", "mod phase165_work;", "/* mod phase165_work; */"],
  [N + "storage/chain_compatibility.rs", 'chain_compatibility/tests/phase165_work.rs', 'other/phase165_work.rs'],
  [N + "storage/chain_compatibility.rs", '#[cfg(test)]\n#[path = "chain_compatibility/tests/phase165_work.rs"]', '#[cfg(any())]\n#[path = "chain_compatibility/tests/phase165_work.rs"]'],
  [N + "storage/chain_compatibility/tests/phase165_work.rs", 'include!("preservation.rs");', ""],
  [N + "storage/chain_compatibility/tests/phase165_work.rs", 'include!("preservation.rs");', '/* include!("preservation.rs"); */'],
  [N + "storage/chain_compatibility/tests/phase165_work.rs", 'include!("preservation.rs");', 'include!("other.rs"); // include!("preservation.rs");'],
]) {
  test(`work preservation source and registration cannot be spoofed: ${file}: ${after}`, () => {
    // Arrange
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "ownership")).toBe(true);
  });
}

for (const [before, after] of [
  ["WorkCompatibilityDecision::ExistingUnproved", "WorkCompatibilityDecision::BootstrapUnavailable"],
  ["; {}; work_decision={}", "; {}; observed_work=exact"],
  ["Self::BootstrapUnavailable => \"bootstrap_unavailable\",", "Self::BootstrapUnavailable => \"legacy_proved\","],
]) {
  test(`work policy cannot become evidence or disappear: ${before}`, () => {
    // Arrange
    const inputs = mutation(N + "storage/chain_compatibility.rs", before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "boundary")).toBe(true);
  });
}

function mutation(file: string, before: string, after: string): Map<string, string> {
  const source = real.get(file)!;
  expect(source.includes(before)).toBe(true);
  const changed = source.replace(before, after);
  if (file === CANONICAL_STARTUP || file === CANONICAL_OBSERVER) expect([source.split(before).length, before === after, changed === source]).toEqual([2, false, false]);
  return new Map(real).set(file, changed);
}
test("current ordinary boundary and explicit fixture corpus pass", () => {
  // Arrange
  const inputs = loadInputs();
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings).toEqual([]);
});

for (const [file, symbol, ordinal] of EDGES) {
  test(`missing reviewed edge: ${file}:${symbol}[${ordinal}]`, () => {
    // Arrange
    const source = real.get(file)!;
    const fn = regions(source).filter(fn => fn.symbol === symbol)[ordinal];
    const inputs = new Map(real).set(file, source.slice(0, fn.start) + source.slice(fn.end));
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "boundary")).toBe(true);
  });
}

const mutations = [
  ["packages/open-bitcoin-network/src/header_store.rs", "self.maybe_consensus_params.as_ref() != Some(params)", "false"],
  ["packages/open-bitcoin-network/src/header_store.rs", "self.maybe_proof_facts(entry.block_hash).is_some()", "true"],
  [N + "sync/block_reconcile.rs", ".requestable_header_entries(runtime.consensus_params)?", ".header_entries()?"],
  [N + "network/candidate_eligibility.rs", ".requestable_header_entries(&params)", ".entries().cloned().collect()"],
  [N + "sync/tests/support_runtime.rs", "revalidate_retained_fixture_owner(runtime);", "// omit strict full matching source replay"],
  [N + "sync/tests/support_runtime.rs", "assert!(\n        !path.exists(),", "assert!(\n        true,"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "self.check_body_eligibility_inner(block, params, current_time, true)", "self.check_body_eligibility_inner(block, params, current_time, false)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", ".filter(|_| self.has_known_body_order(hash))", ".filter(|_| true)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "if state.order_unknown {", "if false {"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "order_unknown,", "order_unknown: false,"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "left.maybe_sequence\n                    .unwrap_or(u64::MAX)", "left.maybe_sequence\n                    .unwrap_or(0)"],
  [N + "network/work_authority.rs", ".check_cached_body_eligibility(block, params, i64::from(block.header.time))", ".check_body_eligibility(block, params, i64::from(block.header.time))"],
  [N + "network/candidate_eligibility.rs", ".check_cached_body_eligibility_detailed(block, params, current_time)", ".check_body_eligibility(block, params, current_time)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", ".map_err(BodyEligibilityError::Validation)?", ".map_err(|error| BodyEligibilityError::Network(invalid_header(error.reject_reason)))?"],
  [N + "sync/block_reconcile.rs", "Err(ActivationFailure::Fatal(error)) => return Err(error),", "Err(ActivationFailure::Fatal(error)) => { maybe_error = Some(error); },"],
  [N + "sync/block_reconcile.rs", "Self::Fatal(error.into())", "Self::Invalid(error.into())"],
  [N + "sync/block_reconcile.rs", "attempted.insert(candidate.block_hash);", "// permit repeated candidate attempts"],
  [N + "sync/block_reconcile.rs", "!attempted.contains(&facts.block_hash)", "true"],
  [N + "sync/block_reconcile.rs", "if connected_count == 0 {", "if true {"],
  [N + "sync/block_reconcile.rs", "if consensus_cause(source)", "if true"],
  [N + "sync/block_reconcile.rs", "ChainstateError::MissingCoin { .. }", "ChainstateError::MissingUndo { .. }"],
  [N + "sync/block_reconcile.rs", "let mut maybe_error = readmit_cached_bodies(runtime, timestamp)?;", "let mut maybe_error = None;"],
  [N + "sync/block_reconcile.rs", "runtime.network.eligible_body_candidates()?", "Vec::new()"],
  [N + "sync/block_reconcile.rs", "if maybe_facts.is_some() {", "if true {"],
  [N + "sync/block_response.rs", "if was_requested && let Some(block) = maybe_block {", "if false && let Some(block) = maybe_block {"],
  [N + "sync/block_response.rs", "maybe_active_tip.is_some_and(|tip| tip.block_hash == block_hash)", "true"],
  [N + "chainstate/live_fixture.rs", "self.chainstate.bind_live_work_source(params)?;", "// skip actual source validation"],
  [N + "chainstate/live_fixture.rs", "|| self.maybe_validated_lineage.is_some()", "|| false"],
  [N + "chainstate/live_fixture.rs", "|| self.maybe_basic_index_owner.is_some()", "|| false"],
  [N + "chainstate/live_fixture.rs", "counts != current.maybe_confirmed_txid_counts", "false"],
  [N + "chainstate/live_fixture.rs", "store.load_all_undo_records().map_err(map_persist)? != current.undo_by_block", "false"],
  [N + "chainstate/live_fixture.rs", "self.chainstate.coins().complete_snapshot()? != physical", "false"],
  [N + "chainstate/live_fixture.rs", "!physical.head_blocks.is_empty()", "false"],
  [N + "chainstate/live_fixture.rs", "physical.maybe_best_block != self.chainstate.tip().map(|tip| tip.block_hash)", "false"],
  [N + "chainstate/live_fixture.rs", "let control = store.filter_publication_guard().map_err(map_persist)?;", "let control = unrelated_store.filter_publication_guard().map_err(map_persist)?;"],
  [N + "chainstate/fjall_store.rs", "store.check_fixture_append_proof_guarded(proof, control)?;", "// skip actual guarded proof validation"],
  [N + "chainstate/fjall_store.rs", "proof.durable_tip() != (tip.height, tip.block_hash)", "false"],
  [N + "storage/fjall_store/filters/ownership.rs", "self.check_basic_filter_append_proof_guarded_counted(proof, control, &mut work).map(|_| ())", "Ok(())"],
  ["packages/open-bitcoin-network/src/lib.rs", "self.epoch.strong_count() == 1", "true"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "epoch: Arc::new(()),", "epoch: self.epoch.clone(),"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "self.epoch = Arc::new(());", "// retain stale current eligibility"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "state.failed = true;\n            self.body_eligibility.epoch = Arc::new(());", "state.failed = true;"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "state.available = false;\n            self.body_eligibility.epoch = Arc::new(());", "state.available = false;"],
  [N + "network/work_authority.rs", "fn check_preference(&self) -> Result<(), ChainstateError> {\n        if !self.epoch.is_current()", "fn check_preference(&self) -> Result<(), ChainstateError> {\n        if false"],
  [N + "network/work_authority.rs", "authorization.check_preference()?;", "// refresh after unrelated stale facts"],
  [N + "network/work_authority.rs", ".maybe_checked_headers\n            .take()", ".maybe_checked_headers\n            .clone()"],
  [N + "sync/types/error.rs", "Self::from(ManagedNetworkError::Chainstate(*source))", "Self::Network { message: source.to_string() }"],
  [N + "chainstate/work_authority.rs", "self.chainstate.bind_live_work_source(params)?;", "// skip actual private source binding"],
  [N + "chainstate/work_authority.rs", "Err(ChainstateError::PreferenceAuthorizationRequired)", "Ok(unqualified_stage)"],
  [N + "chainstate/work_authority.rs", "authorization.check_before_prepare(", "authorization.skip_before_prepare("],
  [N + "chainstate/work_authority.rs", "authorization.check_stage(&self.chainstate, &staged)?;", "// accept genuine stage without preference"],
  [N + "chainstate.rs", ".check_stage(&self.chainstate, &prepared.staged)?;", ".skip_preference(&self.chainstate, &prepared.staged)?;"],
  [N + "chainstate/validation_history.rs", ".check_stage(&self.chainstate, &prepared.staged)?;", ".skip_preference(&self.chainstate, &prepared.staged)?;"],
  [N + "chainstate.rs", "maybe_body_preference: self.maybe_body_preference.clone(),", "maybe_body_preference: None,"],
  [N + "network/work_authority.rs", "owner.check_live_work_source(&self.source)?;", "// skip source owner and generation"],
  [N + "network/work_authority.rs", "owner.check_reorg_work_source(&self.source, staged)?;", "// trust any genuine stage"],
  [N + "network/work_authority.rs", "self.disconnect != disconnect", "false"],
  [N + "network/work_authority.rs", "self.params != params", "false"],
  [N + "network/work_authority.rs", "self.candidate.chain_work > self.active.chain_work", "true"],
  [N + "network/work_authority.rs", "candidate < active", "candidate > active"],
  [N + "network/work_authority.rs", "accepted.same_linkage(&current)", "true"],
  [N + "network/work_authority.rs", "self.blocks_by_hash.get(&position.block_hash) != Some(block)", "false"],
  [N + "network/mempool_lifecycle.rs", ".authorize_direct_connect(block, consensus_params)?;", ".skip_direct_parent_check(block, consensus_params)?;"],
  [N + "network/mempool_lifecycle.rs", "self.retain_authorized_reorg(&mut authorization)?;", "// retain unrelated body history"],
  [N + "network/runtime_authority/filter_index/catch_up.rs", "outcome.maybe_accepted_target != actual_target", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "!Arc::ptr_eq(&self.work_authority.owner, &source.owner)", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "!Arc::ptr_eq(&self.work_authority.generation, &source.generation)", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/stage.rs", "!staged.binding.matches_source(source)", "false"],
  ["packages/open-bitcoin-network/src/lib.rs", "std::sync::Arc::ptr_eq(&self.seal, &other.seal)", "true"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "state.maybe_linkage_seal = Some(std::sync::Arc::new(()));", "// keep copied numeric history only"],
  ["packages/open-bitcoin-chainstate/src/coins.rs", "Err(ChainstateError::CompleteCoinsSnapshotUnavailable)", "Ok(CompleteCoinsSnapshot { coins: HashMap::new(), maybe_best_block: None, head_blocks: Vec::new() })"],
  ["packages/open-bitcoin-chainstate/src/coins/cache.rs", "self.parent.complete_snapshot()?", "self.parent.complete_snapshot().unwrap_or_else(|_| super::CompleteCoinsSnapshot { coins: HashMap::new(), maybe_best_block: None, head_blocks: Vec::new() })"],
  ["packages/open-bitcoin-chainstate/src/coins/memory.rs", "coins: self.coins.clone()", "coins: HashMap::new()"],
  ["packages/open-bitcoin-chainstate/src/engine.rs", "self.work_authority.invalidate_coins_authority();", "self.work_authority.invalidate_stages();"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "source != destination", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "!source.head_blocks.is_empty()", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.validate_live_origin()?;", "// raw import accepted"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "target.work_authority = WorkAuthority::new(WorkOrigin::Live);", "target.work_authority.origin = WorkOrigin::Live;"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "target.work_authority.maybe_params = self.work_authority.maybe_params;", "// discard selected parameters"],
  [N + "storage/coins_view.rs", "snapshot.prefix(&self.coins, [DB_COIN])", "self.coins.prefix([DB_COIN])"],
  [N + "chainstate/live_fixture.rs", "!self.chainstate.coins().parent().belongs_to(&store)", "false"],
  [N + "chainstate/live_fixture.rs", "positions != current.active_chain", "false"],
  [N + "chainstate/live_fixture.rs", "live_coins != physical_coins", "false"],
  [N + "chainstate/live_fixture.rs", "return if saved == *counts", "return if true"],
  [N + "network/runtime_authority/live_fixture.rs", "!matches!(network.admission_scope, AdmissionScope::Fixture)", "false"],
  [N + "network/runtime_authority/live_fixture.rs", "network.install_authority_incarnation();", "// retain preference authorization"],
  [N + "chainstate.rs", "#[cfg(test)]\nmod live_fixture;", "mod live_fixture;"],
  [N + "network/runtime_authority.rs", "#[cfg(test)]\nmod live_fixture;", "mod live_fixture;"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "parent_work.wrapping_add(proof)", "parent_work"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "WorkAuthority::new(WorkOrigin::MechanicalFixture)", "WorkAuthority::new(WorkOrigin::Live)"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.origin == WorkOrigin::Live", "true"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.work_authority.origin != WorkOrigin::Live", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "!Arc::ptr_eq(&self.owner, &binding.owner)", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "!Arc::ptr_eq(&self.generation, &binding.generation)", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "maybe_tip != binding.maybe_tip.as_ref()", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.origin == WorkOrigin::Untrusted", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "selected != params", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.generation = Arc::new(());", "// retain the old generation"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "self.origin = WorkOrigin::Untrusted;", "self.origin = WorkOrigin::Live;"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "independent.work_authority = WorkAuthority::new(origin);", "independent.work_authority = WorkAuthority::new(WorkOrigin::Live);"],
  ["packages/open-bitcoin-chainstate/src/engine/work_authority.rs", "position.block_hash != block_hash(&position.header)", "false"],
  ["packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs", "super::work_authority::derive_child_work(parent_work, block.header.bits)?", "chain_work"],
  ["packages/open-bitcoin-chainstate/src/engine/stage.rs", "self.check_staged_reorg_preview(staged)?;", "// skip the preview binding"],
  [N + "chainstate.rs", ".check_staged_reorg_preview(&prepared.staged)?;", ".check_staged_reorg(&prepared.staged)?;"],
  [N + "chainstate/validation_history.rs", ".validate_staged_connect(prepared.staged)?", ".absorb_staged_connect(prepared.staged)?"],
  [N + "chainstate/validation_history.rs", "self.chainstate.check_staged_reorg(&prepared.staged)?;", "// skip the pre-effect binding"],
  [N + "network/candidate_eligibility.rs", ".filter(|facts| self.has_retained_candidate_suffix(facts.block_hash, &active))", ".filter(|_| true)"],
  [N + "network/candidate_eligibility.rs", "!self.blocks_by_hash.contains_key(&current)", "false"],
  [N + "network/candidate_eligibility.rs", "self.guard_body(block)?;", "let _ignored = self.guard_body(block);"],
  [N + "network/candidate_eligibility.rs", ".check_body_eligibility(block, &params, current_time)?;", ".note_local_block_hash(block_hash(&block.header));"],
  [N + "network/candidate_eligibility.rs", ".maybe_proof_facts(block.header.previous_block_hash)", ".entry(&block.header.previous_block_hash)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "check_block_contextual(block, &context)", "check_block(block)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "selected != params", "false"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "queue.pop_front()", "queue.pop_back()"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", ".checked_add(release.len() as u64)", ".wrapping_add(release.len() as u64)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "left.maybe_sequence\n                    .unwrap_or(u64::MAX)\n                    .cmp(&right.maybe_sequence.unwrap_or(u64::MAX))", "left.block_hash.cmp(&right.block_hash)"],
  ["packages/open-bitcoin-network/src/header_store/eligibility.rs", "state.available = false;", "state.available = true;"],
  [N + "network/runtime_authority/prune_flush.rs", "network.peer_manager.mark_body_unavailable(*hash);", "network.note_local_block_hash(*hash);"],
  [N + "network/mempool_lifecycle.rs", "chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork", "chain_work: u128"],
  [N + "network/runtime_authority.rs", "chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork", "chain_work: u128"],
  [N + "storage/chain_compatibility.rs", "Err(CompatibilityRefusal::from_observation(", "Ok(Self::open(path)?) // marker authorizes\n"],
  [N + "storage/chain_compatibility.rs", "match entries.next()", "match entries.last()"],
  [N + "storage/chain_compatibility.rs", "fs::symlink_metadata(path)", "fs::metadata(path)"],
  [N + "storage/chain_compatibility.rs", "inspect_datadir_without_backend(path.as_ref())", "{ let _store = Self::open(path.as_ref()); inspect_datadir_without_backend(path.as_ref()) }"],
  [N + "network/validation_history.rs", "self.guard_header_batch(&message.headers)?;", "let _ = self.guard_header_batch(&message.headers);"],
  [N + "network/validation_history.rs", "let inserted = headers.insert_header_with_params(header.clone(), &params)?;", "let inserted = headers.insert_header_with_params(header.clone(), &params)?; admission_scope.check_header(header)?;"],
  [N + "network/canonical_admission.rs", "self.guard_header_batch(&candidates)?;", "let _ignored = self.guard_header_batch(&candidates);"],
  [N + "network/canonical_admission.rs", "self.guard_header_batch(&candidates)?;", "self.peer_manager.seed_header_store(headers.clone()); self.guard_header_batch(&candidates)?;"],
  [N + "network/header_sync.rs", "check_block_header_with_params(", "check_block_header("],
  [N + "network/validation_history.rs", ".maybe_proof_facts(first.previous_block_hash)", ".maybe_proof_facts(block_hash(first))"],
  [N + "network/inventory.rs", "block_proof(header.bits)", "block_proof(0x207f_ffff)"],
  [N + "network.rs", ".handle_validated_headers_with_policy(", ".peer_manager.handle_message("],
  [N + "network/action_translation.rs", ".handle_validated_headers_with_policy(", ".peer_manager.handle_message("],
  ["packages/open-bitcoin-network/src/peer/message_dispatch.rs", "tip.chain_work > work", "tip.height > 0"],
  [N + "network/mempool_lifecycle.rs", "self.guard_body(block)?;", "self.blocks_by_hash.insert(block_hash, block.clone()); self.guard_body(block)?;"],
  [N + "network/mempool_lifecycle.rs", ".require_history()", ".maybe_canonical_chain()"],
  [N + "network/canonical_admission.rs", "Self::Fixture => Ok(())", "Self::Canonical(_) => Ok(())"],
  [N + "network/canonical_admission.rs", "self.admission_scope = AdmissionScope::Canonical(chain)", "self.admission_scope = AdmissionScope::Fixture"],
  [R + "context/network.rs", "require_ordinary_scope(chain)?;", "network.set_prune_mode(config.prune_mode)?; require_ordinary_scope(chain)?;"],
  [R + "context/network.rs", "network.maybe_canonical_chain()?", "Some(chain)"],
  [R + "context/network.rs", "return Err(CanonicalContextError::UnprovedOwner);", "return Self::from_fixture_runtime_config_with_network_handle(config, network, maybe_store);"],
  [R + "context/canonical_chain.rs", "Self::try_from_runtime_config(&RuntimeConfig", "Ok(Self::for_fixture_local_operator(network)); Self::try_from_runtime_config(&RuntimeConfig"],
  [CANONICAL_STARTUP, "FjallNodeStore::open_canonical_genesis(data_dir, chain)", "FjallNodeStore::open(data_dir)"],
  [CANONICAL_STARTUP, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_fixture_runtime_config(runtime)"],
  [R + "bin/open_bitcoind/inbound_metrics.rs", ".require_ordinary_history()", ".ordinary_history()"],
  [CANONICAL_OBSERVER, "FjallNodeStore::reopen_canonical_genesis_existing(data_dir, selected_chain)", "FjallNodeStore::open(data_dir)"],
  [C + "wallet/name_resolution.rs", "FjallNodeStore::open_ordinary(data_dir, selected_chain)", "FjallNodeStore::open(data_dir)"],
  [C + "wallet.rs", "FjallNodeStore::open_ordinary(data_dir, selected_chain)", "FjallNodeStore::open(data_dir)"],
  [C + "soak/runtime/chain_compatibility.rs", "FjallNodeStore::open_ordinary(datadir, chain)", "FjallNodeStore::open(datadir)"],
  ["packages/open-bitcoin-consensus/src/chain_spec.rs", "        false\n", "        true\n"],
  ["packages/open-bitcoin-consensus/src/chain_spec.rs", "&MISSING_SIGNET_HISTORY", "&MISSING_HISTORY"],
  ["packages/open-bitcoin-consensus/src/chain_spec.rs", "    chain: CanonicalChain,", "    pub chain: CanonicalChain,"],
  ["packages/open-bitcoin-consensus/src/chain_spec/genesis.rs", "0x52, 0xae", "0x51, 0xae"],
  ["packages/open-bitcoin-consensus/src/chain_spec/genesis.rs", "2_083_236_893", "2_083_236_894"],
  ["packages/open-bitcoin-consensus/src/chain_spec/genesis.rs", "The Times 03/Jan/2009", "The  Times 03/Jan/2009"],
] as const;
for (const [file, before, after] of mutations) {
  test(`actual early-boundary mutation: ${file}: ${before}`, async () => {
    // Arrange
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && (!(file === CANONICAL_STARTUP || file === CANONICAL_OBSERVER) || row.rule === "boundary"))).toBe(true);
    if (file === CANONICAL_STARTUP || file === CANONICAL_OBSERVER) expect((await checkPhase166(inputs)).some(row => row.file === file && row.rule === (file === CANONICAL_OBSERVER ? "existing-observation" : before.startsWith("FjallNodeStore") ? "daemon-sealed-opener" : "daemon-consumption"))).toBe(true);
  });
}

for (const file of [R + "bin/open_bitcoind/inbound_startup.rs", R + "dispatch/filter_index.rs", R + "http/filter_index.rs"]) {
  for (const name of ["ordinary", "fixture_named", "tests_looking_like_fixture"]) {
    test(`new product function remains checked in mixed file ${file}:${name}`, () => {
      // Arrange
      const inputs = new Map(real).set(file, real.get(file)! + `\nfn ${name}() { ManagedRpcContext::for_fixture_local_operator(network); FjallNodeStore::open_unrecovered(path); }`);
      // Act
      const findings = checkPhase164(inputs);
      // Assert
      expect(findings.some(row => row.file === file && row.rule === "opener")).toBe(true);
    });
  }
}

test("string and comment spellings cannot invent product calls", () => {
  // Arrange
  const file = R + "http/filter_index.rs";
  const inputs = new Map(real).set(file, real.get(file)! + '\n// FjallNodeStore::open(path)\nconst TEXT: &str = "ManagedRpcContext::for_fixture_local_operator(network)";');
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings).toEqual([]);
});

test("removing actual mixed-file cfg(test) ownership refuses", () => {
  // Arrange
  const inputs = mutation(R + "http/filter_index.rs", "#[cfg(test)]", "#[cfg(any())]");
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings.some(row => row.rule === "ownership" || row.rule === "boundary")).toBe(true);
});

test("missing named input refuses without discovery", () => {
  // Arrange
  const inputs = new Map(real);
  inputs.delete(N + "storage/chain_compatibility.rs");
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings.some(row => row.rule === "input")).toBe(true);
});

for (const [before, after] of [
  ['#[path = "chain_compatibility/tests/preservation.rs"]', '#[path = "other/preservation.rs"]'],
  ['#[cfg(test)]\n#[path = "chain_compatibility/tests/preservation.rs"]', '#[cfg(any())]\n#[path = "chain_compatibility/tests/preservation.rs"]\n// #[cfg(test)] #[path = "chain_compatibility/tests/preservation.rs"]'],
  ['mod preservation;', '/* mod preservation; */'],
] as const) {
  test(`preservation registration cannot use an unknown path or comment decoy: ${after}`, () => {
    // Arrange
    const file = "packages/open-bitcoin-node/src/storage/chain_compatibility.rs";
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "ownership")).toBe(true);
  });
}

for (const body of [
  "let raw = FjallNodeStore::open; raw(path)",
  "let factory = ManagedRpcContext::for_fixture_local_operator; factory(network)",
  "self.admission_scope = AdmissionScope::Fixture;",
]) {
  test(`product function aliases cannot bypass the opener inventory: ${body}`, () => {
    // Arrange
    const file = R + "http/filter_index.rs";
    const inputs = new Map(real).set(file, real.get(file)! + `\nfn fixture_like_product() { ${body} }`);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file && row.rule === "opener")).toBe(true);
  });
}

test("production guard with a disabling cfg attribute refuses", () => {
  // Arrange
  const inputs = mutation(R + "context/network.rs", "    pub fn from_runtime_config_with_network_handle(", "    #[cfg(any())]\n    pub fn from_runtime_config_with_network_handle(");
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings.some(row => row.rule === "boundary")).toBe(true);
});

test("missing immutable prerequisite cannot borrow a spelling elsewhere", () => {
  // Arrange
  const file = "packages/open-bitcoin-consensus/src/chain_spec.rs";
  const inputs = mutation(file, "    MissingValidationPrerequisite::ExactWork,", "    // MissingValidationPrerequisite::ExactWork,");
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings.some(row => row.file === file && row.rule === "scope")).toBe(true);
});

test("ordinary header closure guard moved after insertion refuses", () => {
  // Arrange
  const file = N + "network/validation_history.rs";
  const guard = "                admission_scope\n                    .check_header(header)\n                    .map_err(|error| error.network_error())?;\n";
  const inputs = mutation(file, guard, "");
  inputs.set(file, inputs.get(file)!.replace("let inserted = headers.insert_header(header.clone())?;", "let inserted = headers.insert_header(header.clone())?;\n" + guard));
  // Act
  const findings = checkPhase164(inputs);
  // Assert
  expect(findings.some(row => row.file === file && row.rule === "boundary")).toBe(true);
});

for (const statement of ["Canonical bootstrap is proven.", "Canonical reopen is supported.", "Production whole-chain parity is qualified."]) {
  test(`unearned current qualification refuses: ${statement}`, () => {
    // Arrange
    const file = "docs/parity/v2-6-canonical-chain-identity.md";
    const inputs = new Map(real).set(file, real.get(file)! + "\n\n" + statement);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "claim")).toBe(true);
  });
}

for (const replacement of ["", "bun run scripts/check-phase164-canonical-identity.ts || true", "bun run scripts/check-phase164-canonical-identity.ts --fixture"]) {
  test(`default native registration refuses substitution: ${replacement}`, () => {
    // Arrange
    const inputs = mutation("scripts/verify.sh", "run_step \"check Phase 164 canonical identity boundary\" bun run scripts/check-phase164-canonical-identity.ts", replacement);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "registration")).toBe(true);
  });
}

for (const [file, inverse] of FIXTURE_INVERSE_CASES) {
  test(`phase165 canonical inverse rejects malformed spans ${file}`, () => {
    // Arrange: complete reviewed current bytes and explicit malformed descriptor shapes.
    const source = real.get(file)!;
    const first = inverse.hunks[0];
    const sparse: [string, string] = [source, first[1]];
    expect(Reflect.deleteProperty(sparse, 1)).toBe(true);
    const malformed: ExactSourceInverse["hunks"][] = [
      [], inverse.hunks.slice(1), [first, ...inverse.hunks], inverse.hunks.toReversed(),
      [first, [first[0].slice(1), first[1]]], [["", first[1]]], [[first[0], first[0]]], [sparse],
    ];
    // Act / Assert: ordered, unique, complete nonempty spans are required before replacement.
    expect(maybeRestoreReviewedSource(source, inverse)).toBeDefined();
    expect(maybeRestoreReviewedSource(source, { ...inverse, restoredRawSha: "eb045d78d273107348b0300c01d29b7552d622abbc6faf81b3ec55359aa9950c", hunks: [sparse] })).toBeUndefined();
    for (const hunks of malformed) {
      expect(maybeRestoreReviewedSource(source, { ...inverse, hunks })).toBeUndefined();
    }
  });
}
for (const change of COPIED_ADAPTER_MUTATIONS) {
  test(`phase165 isolated paired adapter CLI ${change[0]}`, () => {
    expectCopiedAdapterCase(change, load165Inputs(), load165Inputs, [[CLI_MODULES[2], checkPhase164], [CLI_MODULES[0], checkPhase165]]);
  });
}

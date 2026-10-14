import { expect, test } from "bun:test";
import { checkPhase164, loadInputs } from "./check-phase164-canonical-identity.ts";
import { EDGES } from "./check-phase164-canonical-identity/contracts.ts";
import { regions } from "./check-phase164-canonical-identity/regions.ts";
import "./check-phase164-canonical-identity/historical.test.ts";

const real = loadInputs();
const N = "packages/open-bitcoin-node/src/";
const R = "packages/open-bitcoin-rpc/src/";
const C = "packages/open-bitcoin-cli/src/operator/";

function mutation(file: string, before: string, after: string): Map<string, string> {
  const source = real.get(file)!;
  expect(source.includes(before)).toBe(true);
  return new Map(real).set(file, source.replace(before, after));
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
  [N + "storage/chain_compatibility.rs", "Err(CompatibilityRefusal::from_observation(", "Ok(Self::open(path)?) // marker authorizes\n"],
  [N + "storage/chain_compatibility.rs", "match entries.next()", "match entries.last()"],
  [N + "storage/chain_compatibility.rs", "fs::symlink_metadata(path)", "fs::metadata(path)"],
  [N + "storage/chain_compatibility.rs", "inspect_datadir_without_backend(path.as_ref())", "{ let _store = Self::open(path.as_ref()); inspect_datadir_without_backend(path.as_ref()) }"],
  [N + "network/validation_history.rs", "self.guard_header_batch(&message.headers)?;", "let _ = self.guard_header_batch(&message.headers);"],
  [N + "network/validation_history.rs", "let inserted = headers.insert_header(header.clone())?;", "let inserted = headers.insert_header(header.clone())?; admission_scope.check_header(header)?;"],
  [N + "network/mempool_lifecycle.rs", "self.guard_body(block)?;", "self.blocks_by_hash.insert(block_hash, block.clone()); self.guard_body(block)?;"],
  [N + "network/mempool_lifecycle.rs", ".require_history()", ".maybe_canonical_chain()"],
  [N + "network/canonical_admission.rs", "Self::Fixture => Ok(())", "Self::Canonical(_) => Ok(())"],
  [N + "network/canonical_admission.rs", "self.admission_scope = AdmissionScope::Canonical(chain)", "self.admission_scope = AdmissionScope::Fixture"],
  [R + "context/network.rs", "require_ordinary_scope(chain)?;", "network.set_prune_mode(config.prune_mode)?; require_ordinary_scope(chain)?;"],
  [R + "context/network.rs", "network.maybe_canonical_chain()?", "Some(chain)"],
  [R + "context/network.rs", "return Err(CanonicalContextError::UnprovedOwner);", "return Self::from_fixture_runtime_config_with_network_handle(config, network, maybe_store);"],
  [R + "context/canonical_chain.rs", "Self::try_from_runtime_config(&RuntimeConfig", "Ok(Self::for_fixture_local_operator(network)); Self::try_from_runtime_config(&RuntimeConfig"],
  [R + "bin/open-bitcoind.rs", "FjallNodeStore::open_ordinary(data_dir, chain)", "FjallNodeStore::open(data_dir)"],
  [R + "bin/open-bitcoind.rs", "DurableSyncRuntime::open_ordinary_configured(", "DurableSyncRuntime::open_configured("],
  [R + "bin/open_bitcoind/inbound_metrics.rs", ".require_ordinary_history()", ".ordinary_history()"],
  [C + "runtime/support.rs", "FjallNodeStore::open_ordinary(data_dir, selected_chain)", "FjallNodeStore::open(data_dir)"],
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
  test(`actual early-boundary mutation: ${file}: ${before}`, () => {
    // Arrange
    const inputs = mutation(file, before, after);
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.file === file)).toBe(true);
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

import { afterEach, expect, test } from "bun:test";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { CHECK_FILES, checkPhase159FilterRpcs } from "./check-phase159-filter-rpcs.ts";
import { TEST_ACTIONS, TESTS } from "./check-phase159-filter-rpcs/contracts.ts";
import { maybeFunction } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { MODULE_EDGES, meaningfulAssertions } from "./check-phase159-filter-rpcs/evidence.ts";
import { spawnSync } from "node:child_process";

const roots: string[] = [];
const repo = resolve(import.meta.dir, "..");
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "phase159-claims-"));
  roots.push(root);
  expect(spawnSync("git", ["init", "--quiet", root]).status).toBe(0);
  for (const file of new Set([...CHECK_FILES, "packages/open-bitcoin-rpc/src/lib.rs", "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs", "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs", "packages/open-bitcoin-node/src/sync.rs", "packages/open-bitcoin-node/src/sync/tests.rs", "packages/open-bitcoin-node/src/sync/tests/filter_index.rs", "packages/open-bitcoin-cli/src/main.rs", "packages/open-bitcoin-cli/src/client.rs", "packages/open-bitcoin-cli/src/client/tests.rs", "packages/open-bitcoin-cli/src/lib.rs", "packages/open-bitcoin-cli/src/args.rs", "packages/open-bitcoin-cli/src/args/tests.rs"])) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    copyFileSync(join(repo, file), join(root, file));
  }
  return root;
}
function edit(root: string, file: string, change: (source: string) => string): void {
  const source = readFileSync(join(root, file), "utf8");
  const changed = change(source);
  expect(changed !== source).toBe(true);
  writeFileSync(join(root, file), changed);
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

test("current scoped evidence runs before final native/requirement activation", () => {
  // Arrange / Act / Assert
  expect(checkPhase159FilterRpcs(fixture())).toEqual([]);
});

const N = "packages/open-bitcoin-node/src/";
const R = "packages/open-bitcoin-rpc/src/";
const mutations: [string, string, string, string][] = [
  ["lost registry name", R + "method.rs", 'Self::GetBlockFilter => "getblockfilter"', 'Self::GetBlockFilter => "removed"'],
  ["lost typed normalizer", R + "method.rs", "normalize_getblockfilter(params).map(MethodCall::GetBlockFilter)", "Err(RpcFailure::method_not_found())"],
  ["detached context authority", R + "context/filter_index.rs", "self.network.clone()", "unrelated_network.clone()"],
  ["lost real filter dispatch", R + "dispatch.rs", "filter_index::prepare_filter(context, request)", "unrelated_filter(context, request)"],
  ["authentication after parse", R + "http.rs", "if !authorized(headers, &state.auth)", "if false"],
  ["lock held across readiness await", R + "http.rs", "drop(context);", "retain_context(context);"],
  ["completion recaptures target", R + "http/filter_index.rs", ".complete_basic_filter_read(request.block_hash, completion)", ".basic_filter_query(request.block_hash)"],
  ["wrong original hash", R + "http/filter_index.rs", ".complete_basic_filter_read(request.block_hash, completion)", ".complete_basic_filter_read(other_hash, completion)"],
  ["lost barrier await", R + "http/filter_index.rs", "barrier.await.map_err(dispatch::query_failure)?", "fabricate_completion()"],
  ["legacy falsely never connected", R + "dispatch/filter_index.rs", "BasicBlockValidationProvenance::UnknownLegacy => internal_failure()", "BasicBlockValidationProvenance::UnknownLegacy => missing_failure(BasicBlockValidationProvenance::NeverConnected, true)"],
  ["raw backend leaked", R + "dispatch/filter_index.rs", "pub(crate) fn query_failure(_error: BasicFilterQueryError) -> RpcFailure {\n    internal_failure()", "pub(crate) fn query_failure(_error: BasicFilterQueryError) -> RpcFailure {\n    RpcFailure::internal_error(_error.to_string())"],
  ["accepted capture after persistence", N + "chainstate/validation_history.rs", "self.observe_absorbed_validation(", "self.persist()?; self.observe_absorbed_validation("],
  ["lost accepted reorg fence", N + "chainstate/validation_history.rs", "self.accept_basic_index_reorg(accepted, prepared.maybe_index)", "unrelated_reorg(accepted, prepared.maybe_index)"],
  ["lost prepare admission", N + "chainstate.rs", "self.admit_validation_acceptance(", "self.unchecked_validation_acceptance("],
  ["raw seed coverage laundering", N + "storage/fjall_store/coins.rs", "self.invalidate_validation_coverage()?;\n            control.invalidate_append()?;", "control.invalidate_append()?;"],
  ["raw flush sink coverage laundering", N + "storage/fjall_store/coins.rs", "self.check_validation_coverage_for_metadata(active_chain)?;", "skip_validation_coverage(active_chain);"],
  ["parent fields substitute integrity", N + "storage/fjall_store/filters/query.rs", "codec::parse_record(&parent_key, &parent_bytes)?", "codec::parse_record_fields(&parent_key, &parent_bytes)?"],
  ["lost read bound", N + "storage/fjall_store/filters/query.rs", "*size <= BASIC_FILTER_QUERY_MAX_READ_BYTES", "true"],
  ["lost original captured provenance", N + "network/runtime_authority/filter_index/readiness/owner.rs", "provenance: completion.request.provenance", "provenance: BasicBlockValidationProvenance::NeverConnected"],
  ["initial latch substitutes processed readiness", N + "network/runtime_authority/filter_index/readiness/owner.rs", "record.height() >= frontier.accepted_height()", "true"],
  ["wake under authority lock", N + "network/runtime_authority/filter_index/readiness/owner.rs", "drop(network);\n        wake_all(wakes);", "wake_all(wakes);\n        drop(network);"],
  ["poison no longer settles", N + "network/runtime_authority/filter_index/readiness/owner.rs", "drop(poison.into_inner());\n            self.basic_filter_authority_unavailable()", "drop(poison.into_inner()); ManagedNetworkAuthorityError::Poisoned"],
  ["waiter limit silently lifted", N + "network/runtime_authority/filter_index/readiness.rs", "MAX_BASIC_FILTER_WAITERS: usize = 64", "MAX_BASIC_FILTER_WAITERS: usize = 64000"],
  ["real deletion replaced with empty plan", R + "bin/open_bitcoind/tests/filter_index/rpc/retention.rs", "heights: vec![20]", "heights: vec![]"],
  ["body-loss assertion dropped", R + "bin/open_bitcoind/tests/filter_index/rpc/retention.rs", "fixture.store.load_block(pruned)", "fixture.store.load_block(unrelated_hash)"],
  ["live-lock refusal removed", R + "bin/open_bitcoind/tests/filter_index/rpc/retention.rs", "FjallNodeStore::open(&fixture.path).is_err()", "true"],
  ["retention never closes all handles", R + "bin/open_bitcoind/tests/filter_index/rpc/retention.rs", "fixture.close()", "fixture.clone_path()"],
  ["reopen stale comparison weakened", R + "bin/open_bitcoind/tests/filter_index/rpc/retention.rs", "assert_eq!(filter(&fixture, stale).await, original_stale);", "assert!(true);"],
  ["fault fixture raw-seeded", N + "sync/tests/filter_index/catch_up/rpc_faults.rs", ".connect_local_block(block, ScriptVerifyFlags::P2SH, params())", ".seed_coins_from_snapshot(block)"],
  ["private query fault no longer injected", N + "sync/tests/filter_index/catch_up/rpc_faults.rs", ".inject_query_record_fault_for_test(hash, fault)", ".unrelated_no_fault(hash, fault)"],
  ["backend read assertion weakened", N + "sync/tests/filter_index/catch_up/rpc_faults.rs", "StorageError::BackendFailure", "StorageError::Corruption"],
  ["lost first publication fault", N + "sync/tests/filter_index/catch_up/rpc_faults.rs", "publication(FilterPublicationFault::BeforeRecords)", "assert!(true)"],
  ["lost direct poison dispatcher", N + "network/runtime_authority/lifecycle.rs", "let mut network = match self.lock_authority()", "let mut network = match self.authority.lock()"],
  ["lost type aggregation precedence", R + "method/filter_index/normalize.rs", "framework_arguments(params, &[\"blockhash\", \"filtertype\"], 1, GETBLOCKFILTER_HELP)?", "unchecked_arguments(params)?"],
  ["wrong hash display byte order", R + "method/filter_index/normalize.rs", "raw.iter_mut().rev()", "raw.iter_mut()"],
  ["wrong header display byte order", R + "method/filter_index.rs", "raw_header.into_iter().rev()", "raw_header.into_iter()"],
  ["foreign provenance accepted", N + "storage/fjall_store/validation_history.rs", "!accepted.belongs_to(self)", "false"],
  ["raw metadata skips authentic identities", N + "storage/fjall_store/validation_history.rs", "record.status() != StoredValidationStatus::ScriptsValid", "false"],
  ["unmeasured response bound", N + "storage/fjall_store/filters/query.rs", "2 * MAX_SIZE as usize + 64", "4 * MAX_SIZE as usize + 64"],
];
for (const [name, file, from, to] of mutations) {
  test(name, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replaceAll(from, to));
    // Act
    const failures = checkPhase159FilterRpcs(root);
    // Assert
    expect(failures.some(failure => failure.includes(file))).toBe(true);
  });
}

for (const [file, symbol] of TESTS) {
  for (const mode of ["no-op", "ignored"]) {
    test(`${mode} cannot stand in for ${symbol}`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => {
        const body = maybeFunction(source, symbol);
        if (!body) throw new Error(`missing fixed test ${symbol}`);
        if (mode === "ignored") {
          const declaration = source.lastIndexOf("\n", body.start) + 1;
          return source.slice(0, declaration) + "#[ignore]\n" + source.slice(declaration);
        }
        const open = source.indexOf("{", body.start);
        return source.slice(0, open + 1) + " assert!(true); " + source.slice(body.end - 1);
      });
      // Act / Assert
      expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes(symbol))).toBe(true);
    });
  }
}

const DOC = "docs/parity/catalog/basic-compact-filters.md";
for (const claim of ["Peer filter serving is supported.", "CFPR-02 is Complete.", "CFNET-01 is Complete.", "CFOP-01 is Complete.",
  "CFGR-01 is Complete.", "v2.5 has shipped.", "Production readiness is established.", "Production-funds use is safe.",
  "All private faults were injected through daemon HTTP.", "The 32 MiB filter is consensus-generated.", "Entire HTTP response memory is capped at 67,108,928 bytes.",
  "Default automatic pruning is proven by this fixture."]) {
  for (const file of [DOC, "README.md", "packages/README.md", "docs/parity/deviations-and-unknowns.md"]) test(`unsupported independent claim in ${file}: ${claim}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source + `\n${claim}\nOther features remain deferred.\n`);
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("unsupported claim"))).toBe(true);
  });
}

for (const kind of ["test", "guard"]) {
  for (const section of ["documented", "executed"]) {
    test(`historical verifier order survives missing159 ${kind} ${section}`, () => {
      // Arrange
      const root = fixture();
      const command = kind === "test" ? "bun test ./scripts/check-phase159-filter-rpcs.test.ts" : "bun run scripts/check-phase159-filter-rpcs.ts";
      edit(root, "scripts/verify.sh", source => source.split("\n").map(line =>
        line.endsWith(command) && (section === "executed" ? line.startsWith("run_step ") : line === command) ? "# removed" : line).join("\n"));
      // Act / Assert
      expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("verifier"))).toBe(true);
    });
  }
}

test("new untracked Rust path requires an exact manifest registration", () => {
  // Arrange
  const root = fixture();
  const file = N + "network/runtime_authority/filter_index/unregistered.rs";
  writeFileSync(join(root, file), "// Parity breadcrumbs:\n// - packages/bitcoin-knots/src/index/base.cpp\n");
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("breadcrumb") && failure.includes(file))).toBe(true);
});

test("a public production fault factory is rejected but private cfg(test) owners pass", () => {
  // Arrange
  const root = fixture();
  const file = N + "storage/fjall_store/filters/query.rs";
  edit(root, file, source => source.replace("#[cfg(test)]\n    pub(crate) fn inject_query_record_fault_for_test", "pub fn inject_query_record_fault_for_test"));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("production test seam"))).toBe(true);
});

test("missing CFRP parity evidence cannot be replaced by a completed future requirement", () => {
  // Arrange
  const root = fixture();
  edit(root, "docs/parity/index.json", source => {
    const value = JSON.parse(source);
    const row = value.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-authenticated-basic-filter-and-index-rpcs");
    row.requirements = ["CFPR-02"];
    row.evidence = [];
    row.status = "done";
    return JSON.stringify(value);
  });
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("parity"))).toBe(true);
});

test("a raw factory cannot mint accepted provenance beside genuine receipts", () => {
  // Arrange
  const root = fixture();
  const file = N + "chainstate/validation_history.rs";
  edit(root, file, source => source + "\npub(crate) fn from_raw(store: FjallNodeStore, identities: Vec<BlockValidationIdentity>) -> AcceptedValidationReceipt { AcceptedValidationReceipt { store: store, identities } }\n");
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("genuine connect/reorg receipt"))).toBe(true);
});

for (const attribute of ["#[cfg(any())]", "#[cfg(not(test))]", "#[cfg_attr(test, ignore)]", "#[cfg_attr(test, cfg(any()))]"]) {
  test(`WR04 excluding selector attribute ${attribute}`, () => {
    // Arrange
    const root = fixture();
    const file = R + "method/filter_index/tests.rs";
    const symbol = "phase159_filter_rpc_contract_numeric_errors_roundtrip";
    edit(root, file, source => source.replace(`fn ${symbol}`, `${attribute}\nfn ${symbol}`));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes(symbol))).toBe(true);
  });
}
for (const [file, declaration] of [[R + "bin/open_bitcoind/tests/filter_index.rs", "mod rpc;"], [R + "lib.rs", "pub mod method;"]]) {
  for (const attribute of ["#[cfg(any())]", "#[cfg_attr(test, cfg(any()))]"]) {
    test(`WR04 excluding owning module ${file}: ${attribute}`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => source.replace(declaration, `${attribute}\n${declaration}`));
      // Act / Assert
      expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("owning"))).toBe(true);
    });
  }
}

for (const symbol of ["phase159_filter_rpc_contract_numeric_errors_roundtrip", "phase159_validation_history_codec_roundtrip_binds_raw_identity", "phase159_basic_query_authority_memory_is_disabled"]) {
  for (const assertion of ["assert_eq!(1, 1);", "assert_ne!(1, 2);", "assert!(1 == 1);", 'assert_eq!(2 + 2, 4, "constant message");', "let observed = 7; assert_eq!(observed, observed);", "let observed = 7; assert!(observed == observed);"]) {
    test(`WR05 constant behavior ${symbol}: ${assertion}`, () => {
      // Arrange
      const root = fixture();
      const [file] = TESTS.find(([, name]) => name === symbol)!;
      edit(root, file, source => {
        const body = maybeFunction(source, symbol)!;
        const open = source.indexOf("{", body.start);
        return source.slice(0, open + 1) + assertion + source.slice(body.end - 1);
      });
      // Act / Assert
      expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(symbol))).toBe(true);
    });
  }
}

test("WR06 Git-ignored generated Rust is excluded from phase discovery", () => {
  // Arrange
  const root = fixture();
  const generated = N + "target/generated.rs";
  mkdirSync(dirname(join(root, generated)), { recursive: true });
  writeFileSync(join(root, generated), "pub fn generated() {}\n");
  writeFileSync(join(root, ".gitignore"), "target/\n");
  // Act / Assert
  expect(checkPhase159FilterRpcs(root)).toEqual([]);
});

for (const edge of MODULE_EDGES) {
  test(`WR04 actual owning edge stays compiled: ${edge.parent}:${edge.symbol}`, () => {
    // Arrange
    const root = fixture();
    edit(root, edge.parent, source => source.replace(new RegExp(`(?:pub(?:\\s*\\([^)]*\\))?\\s+)?\\bmod\\s+${edge.symbol}\\s*;`), declaration => `#[cfg(any())]\n${declaration}`));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(edge.parent) && failure.includes("owning"))).toBe(true);
  });
}
for (const [file, declaration] of [[R + "dispatch/filter_index.rs", "mod initial_tests {"], [R + "bin/open_bitcoind/coins_flush.rs", "mod phase159_tests {"]]) {
  test(`WR04 inline owning module remains compiled: ${file}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => source.replace(declaration, `#[cfg_attr(test, cfg(any()))]\n${declaration}`));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("executable"))).toBe(true);
  });
}
test("WR04 file-level test exclusion cannot hide a required selector", () => {
  // Arrange
  const root = fixture();
  const file = R + "method/filter_index/tests.rs";
  edit(root, file, source => "#![cfg(any())]\n" + source);
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("owning"))).toBe(true);
});
test("WR04 inner body attributes do not control owning declaration classification", () => {
  // Arrange
  const root = fixture();
  const file = R + "method/filter_index/tests.rs";
  edit(root, file, source => source.replace("fn phase159_filter_rpc_contract_numeric_errors_roundtrip() {", "fn phase159_filter_rpc_contract_numeric_errors_roundtrip() {\n#[cfg_attr(any(), allow(dead_code))]\nlet internal_marker = 1;"));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root)).toEqual([]);
});

for (const assertion of ["assert_eq!(1, 1);", "assert_ne!(1, 2);", "assert!(1 == 1);", 'assert_eq!(2 + 2, 4, "message");', "assert_eq!(observed, observed);", "assert!(observed >= observed);", "assert!(true, \"message\");"]) {
  test(`WR05 assertions stay meaningful even with retained action: ${assertion}`, () => {
    // Arrange
    const source = "fn check() { let observed = actual_query(); " + assertion + " }";
    const body = maybeFunction(source, "check")!.body;
    // Act / Assert
    expect(meaningfulAssertions(source, body)).toBe(false);
  });
}
test("WR05 actual result comparison remains meaningful", () => {
  // Arrange
  const source = "fn check() { let code = actual_query(); assert_eq!(code.as_i32(), -3); }";
  // Act / Assert
  expect(meaningfulAssertions(source, maybeFunction(source, "check")!.body)).toBe(true);
});
for (const [file, symbol, action] of TEST_ACTIONS) {
  test(`WR05 fixed selector retains its actual action: ${symbol}`, () => {
    // Arrange
    const root = fixture();
    edit(root, file, source => {
      const body = maybeFunction(source, symbol)!;
      const original = source.slice(body.start, body.end);
      const call = new RegExp(`\\b${action.replaceAll("::", "\\s*::\\s*")}(?=\\s*(?:::<[^;{}]*?>)?\\s*\\()`, "g");
      const changed = original.replace(call, "removed_action");
      return source.slice(0, body.start) + changed + source.slice(body.end);
    });
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(symbol) && (failure.includes("actual behavior") || failure.includes("actual behavior connection")))).toBe(true);
  });
}

test("WR06 an ignored tracked Rust file remains checked", () => {
  // Arrange
  const root = fixture();
  const file = N + "tracked_generated.rs";
  const anchor = "packages/bitcoin-knots/src/validation.cpp";
  writeFileSync(join(root, file), `// Parity breadcrumbs:\n// - ${anchor}\n\npub fn retained() {}\n`);
  edit(root, "docs/parity/source-breadcrumbs.json", source => {
    const manifest = JSON.parse(source);
    manifest.groups.push({ label: "tracked-generated-control", files: [file], breadcrumbs: [anchor] });
    return JSON.stringify(manifest);
  });
  expect(spawnSync("git", ["add", "--", file], { cwd: root }).status).toBe(0);
  writeFileSync(join(root, ".gitignore"), file + "\n");
  expect(checkPhase159FilterRpcs(root)).toEqual([]);
  writeFileSync(join(root, file), "pub fn retained() {}\n");
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("breadcrumb"))).toBe(true);
});
test("WR06 first-party discovery does not recursively inspect a Git link", () => {
  // Arrange
  const root = fixture();
  const link = N + "foreign-source";
  mkdirSync(join(root, link), { recursive: true });
  writeFileSync(join(root, link, "generated.rs"), "pub fn outside_first_party() {}\n");
  expect(spawnSync("git", ["update-index", "--add", "--cacheinfo", `160000,${"1".repeat(40)},${link}`], { cwd: root }).status).toBe(0);
  // Act / Assert
  expect(checkPhase159FilterRpcs(root)).toEqual([]);
});

const C = "packages/open-bitcoin-cli/src/";
for (const [name, from, to] of [
  ["lost getblockfilter conversion", "MethodCall::GetBlockFilter(request) =>", "MethodCall::OtherFilter(request) =>"],
  ["lost getindexinfo conversion", "MethodCall::GetIndexInfo(request) =>", "MethodCall::OtherIndex(request) =>"],
  ["wrong display hash order", ".rev()", ".skip(0)"],
  ["wrong node HTTP route", ".map(SupportedMethod::scope)", ".map(|_| MethodScope::Wallet)"],
  ["lost shared normalization", "normalize_method_call(method_name, params)", "unrelated_normalizer(method_name, params)"],
  ["wrong filter wire key", '"blockhash": blockhash', '"block_hash": blockhash'],
  ["wrong index wire key", '"index_name": name', '"name": name'],
  ["wrong v0 wire value", 'BlockFilterSelection::V0 => "v0"', 'BlockFilterSelection::V0 => "basic"'],
  ["uppercase display hash", 'format!("{byte:02x}")', 'format!("{byte:02X}")'],
  ["lost authorization header", '.header("Authorization", &self.authorization_header)', '.header("X-Authorization", &self.authorization_header)'],
] ) {
  test(`CLIguard ${name}`, () => {
    // Arrange
    const root = fixture();
    edit(root, C + "client.rs", source => source.replace(from, to));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(C + "client.rs"))).toBe(true);
  });
}
test("CLIguard native execution remains after workspace tests", () => {
  // Arrange
  const root = fixture();
  edit(root, "scripts/verify.sh", source => source.replace('run_step "cargo test" cargo test --manifest-path packages/Cargo.toml --workspace --all-features', "# removed workspace test stage"));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("CLI binary"))).toBe(true);
});

for (const suffix of ["", " --no-run", " -- --list", " nonexistent_phase159_selector"]) {
  test(`CLIguard default verifier executes the whole binary suite: ${suffix || "removed"}`, () => {
    // Arrange
    const root = fixture();
    const command = 'run_step "cargo test CLI RPC client binary" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features';
    edit(root, "scripts/verify.sh", source => source.replace(command, suffix ? command + suffix : "# removed explicit CLI tests"));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("CLI binary"))).toBe(true);
  });
}
test("CLIguard a comment cannot supply the actual JSON field projection", () => {
  // Arrange
  const root = fixture();
  edit(root, C + "client.rs", source => source.replace('Ok(serde_json::json!({"blockhash": blockhash, "filtertype": filtertype}))', '// serde_json::json!({"blockhash": blockhash, "filtertype": filtertype})\n            Ok(serde_json::json!({"wrong": blockhash, "filtertype": filtertype}))'));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes("actual CLI wire"))).toBe(true);
});
test("CLIguard executable entrypoint keeps the actual client route", () => {
  // Arrange
  const root = fixture();
  edit(root, C + "main.rs", source => source.replace("client::run_cli(", "detached_client::run_cli("));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(C + "main.rs"))).toBe(true);
});

test("Metadataguard raw DTO identity parsing cannot precede ledger authentication", () => {
  // Arrange
  const root = fixture();
  const file = N + "storage/fjall_store/validation_history.rs";
  edit(root, file, source => source.replace("for position in positions {", "for position in positions {\nlet _ = BlockValidationIdentity::new(position.block_hash, position.previous_block_hash(), position.height)?;"));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("raw metadata identity parse"))).toBe(true);
});
test("Metadataguard accepted identity construction remains strict", () => {
  // Arrange
  const root = fixture();
  const file = N + "storage/validation_history.rs";
  edit(root, file, source => source.replace("|| hash == parent_hash", "|| false"));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file) && failure.includes("strict accepted identity"))).toBe(true);
});
for (const [name, from, to] of [
  ["lost ScriptsValid", "record.status() != StoredValidationStatus::ScriptsValid", "false"],
  ["lost hash binding", "identity.hash() != position.block_hash", "false"],
  ["lost parent binding", "identity.parent_hash() != position.previous_block_hash()", "false"],
  ["lost height binding", "identity.height() != position.height", "false"],
  ["decode error becomes absence", "maybe_validation_history_record(position.block_hash)?", "maybe_validation_history_record(position.block_hash).ok().flatten()"],
  ["invalidation under history lock", "drop(control);\n        if !authenticated {\n            self.invalidate_validation_coverage()?;", "if !authenticated {\n            self.invalidate_validation_coverage()?;\n            drop(control);"],
]) {
  test(`Metadataguard ${name}`, () => {
    // Arrange
    const root = fixture();
    const file = N + "storage/fjall_store/validation_history.rs";
    edit(root, file, source => source.replace(from, to));
    // Act / Assert
    expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file))).toBe(true);
  });
}
test("Appendguard fixture earns genuine configured recovery", () => {
  // Arrange
  const root = fixture();
  const file = N + "storage/fjall_store/filters/tests/append.rs";
  edit(root, file, source => source.replace(".configure_basic_filter_index_before_prune(", ".install_fake_recovered_proof("));
  // Act / Assert
  expect(checkPhase159FilterRpcs(root).some(failure => failure.includes(file))).toBe(true);
});

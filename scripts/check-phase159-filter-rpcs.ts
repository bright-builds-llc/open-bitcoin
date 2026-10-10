#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { compact, hasOrderedCode, maybeFunction, ordinaryRust, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { CLI, CONTRACTS, DOC, NEW_RUST, NODE, PHASE, RPC, SOURCE_FILES, SURFACE, TEST_ACTIONS, TEST_CONTRACTS, TESTS } from "./check-phase159-filter-rpcs/contracts.ts";
import { OWNER_FILES, checkModuleOwnership, meaningfulAssertions, supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { worktreePaths } from "./check-parity-breadcrumbs.ts";

const UPSTREAM = ["rpc/blockchain.cpp", "rpc/node.cpp", "rpc/server.cpp", "rpc/util.cpp", "index/base.cpp", "index/blockfilterindex.cpp", "validation.cpp", "chain.h"].map(file => "packages/bitcoin-knots/src/" + file);
const GUIDANCE = [DOC, "README.md", "packages/README.md", "docs/parity/deviations-and-unknowns.md"];
const REPORTS = [PHASE + "159-04-QUERY-MEASUREMENTS.md", PHASE + "159-07-DAEMON-PROOF.md"];
export const CHECK_FILES = [...new Set([...SOURCE_FILES, ...OWNER_FILES, NODE + "network/mempool_lifecycle.rs", ...UPSTREAM, ...GUIDANCE, ...REPORTS,
  ...["01", "02", "03", "04", "05", "06", "07"].map(plan => PHASE + `159-${plan}-SUMMARY.md`),
  "packages/open-bitcoin-cli/BUILD.bazel", "packages/open-bitcoin-rpc/BUILD.bazel", "docs/parity/index.json", "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh", "scripts/check-parity-breadcrumbs.ts", "scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase159-filter-rpcs.test.ts", "scripts/check-phase159-filter-rpcs/contracts.ts", "scripts/check-phase159-filter-rpcs/evidence.ts"])];
type Texts = (file: string) => string;

/** Lexical contracts supplement executed Rust evidence; they cannot establish parity or a native pass. */
export function checkPhase159FilterRpcs(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: cannot read required evidence`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  for (const [file, symbol, anchors] of CONTRACTS) {
    const maybeBody = maybeBoundary(text(file), symbol, true);
    if (!maybeBody || !supportedOwnership(text(file), maybeBody.start, false)
      || !hasOrderedCode(maybeBody.body, anchors)) failures.push(`${file}:${symbol}: production boundary`);
  }
  for (const [file, symbol, anchors] of TEST_CONTRACTS) {
    const maybeBody = maybeFunction(text(file), symbol);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, anchors)) failures.push(`${file}:${symbol}: actual behavior evidence`);
  }
  for (const [file, symbol] of TESTS) {
    const maybeTest = maybeFunction(text(file), symbol);
    if (!maybeTest || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeTest.attributes)
      || /#\[\s*ignore\b/.test(maybeTest.attributes) || !supportedOwnership(text(file), maybeTest.start, true)
      || !meaningfulAssertions(text(file), maybeTest.body)) failures.push(`${file}:${symbol}: executable nonempty evidence`);
  }
  for (const [file, symbol, action] of TEST_ACTIONS) {
    const maybeTest = maybeFunction(text(file), symbol);
    if (!maybeTest || !new RegExp(`\\b${action.replaceAll("::", "\\s*::\\s*")}(?:\\s*::\\s*<[^;{}]*?>)?\\s*\\(`).test(maybeTest.body)) failures.push(`${file}:${symbol}: minimum actual behavior connection ${action}`);
  }
  checkSource(text, failures);
  checkCliConversion(text, failures);
  checkRegistrations(text, failures);
  checkModuleOwnership(text, failures);
  checkBreadcrumbs(root, text, failures);
  checkParity(text, failures);
  checkDocuments(text, failures);
  checkVerifier(text("scripts/verify.sh"), failures);
  return [...new Set(failures)];
}

function maybeBoundary(source: string, symbol: string, production: boolean) {
  // The shared legacy selector treats an array type's semicolon as a declaration.
  // Mask only signature array separators, preserving offsets and the actual body.
  const code = production ? ordinaryRust(source) : rustCode(source);
  const matches = [...code.matchAll(new RegExp(`\\bfn\\s+${symbol}\\b`, "g"))];
  if (matches.length !== 1) return undefined;
  const start = matches[0].index;
  const open = code.indexOf("{", start);
  if (open < 0) return undefined;
  const signature = source.slice(start, open).replace(/\[[^\]]*\]/g, array => array.replaceAll(";", " "));
  return maybeFunction(source.slice(0, start) + signature + source.slice(open), symbol, production);
}

function checkSource(text: Texts, failures: string[]): void {
  for (const file of NEW_RUST.filter(file => !/(?:\/tests(?:\/|\.rs)|_tests\.rs)/.test(file))) {
    // Owning #[cfg(test)] modules/functions are masked, including public test helpers.
    const code = ordinaryRust(text(file));
    if (/\bpub(?:\s*\([^)]*\))?\s+(?:async\s+)?fn\s+\w*(?:for_test|inject_\w*fault)\w*\b/.test(code)) failures.push(`${file}: production test seam`);
  }
  const method = text(RPC + "method.rs");
  const supported = method.split("pub enum MethodCall")[0];
  const origin = maybeFunction(supported, "origin", true)?.body ?? "";
  const scope = maybeFunction(supported, "scope", true)?.body ?? "";
  if (!origin.includes("_ => MethodOrigin::BaselineParity") || /GetBlockFilter|GetIndexInfo/.test(origin)
    || !hasOrderedCode(scope, ["MethodScope::Wallet", "Self::GetBlockFilter", "Self::GetIndexInfo", "MethodScope::Node"])) failures.push(`${RPC}method.rs: baseline node-scoped registry`);
  for (const [variant, name] of [["GetBlockFilter", "getblockfilter"], ["GetIndexInfo", "getindexinfo"]]) {
    const all = maybeFunction(method, "all", true);
    if (!all?.body.includes(`Self::${variant},`) || !method.includes(`Self::${variant} => "${name}"`)
      || !method.includes(`#[serde(rename = "${name}")]`)) failures.push(`${RPC}method.rs: registry ${name}`);
  }
  for (const symbol of ["query_failure", "internal_failure"]) {
    const file = RPC + "dispatch/filter_index.rs";
    const maybeBody = maybeFunction(text(file), symbol, true);
    if (!maybeBody || (symbol === "query_failure" && compact(maybeBody.body) !== "internal_failure()")) failures.push(`${file}:${symbol}: fixed redacted projection`);
  }
  for (const symbol of ["commit_prepared_connect", "commit_prepared_reorg"]) {
    const file = NODE + "chainstate/validation_history.rs";
    const body = compact(maybeFunction(text(file), symbol, true)?.body ?? "");
    const capture = body.indexOf("self.observe_absorbed_validation(");
    const persist = body.indexOf("self.persist(");
    if (capture < 0 || persist < capture) failures.push(`${file}:${symbol}: capture before fallible persistence`);
  }
  const mempool = NODE + "network/mempool_lifecycle.rs";
  const maybeReorg = maybeFunction(text(mempool), "reorg_to_branch", true);
  if (!maybeReorg || !hasOrderedCode(maybeReorg.body, ["prepare_reorg(", "install_prepared_reorg_preview(", "apply_reorg_mempool_lifecycle(", "commit_prepared_reorg("])) failures.push(`${mempool}: preflight before preview/mempool`);
  const lifecycle = NODE + "network/runtime_authority/lifecycle.rs";
  for (const symbol of ["apply_lifecycle_command", "dispatch_checkpoint_completion", "dispatch_checkpoint_abort"]) {
    // apply_lifecycle_command also has the pure network dispatcher; select the handle impl.
    const source = text(lifecycle).split("\npub(in crate::network) fn apply_lifecycle_command")[0];
    const maybeBody = maybeFunction(source, symbol, true);
    if (!maybeBody || !compact(maybeBody.body).includes("self.lock_authority()")) failures.push(`${lifecycle}:${symbol}: poison settlement dispatcher`);
  }
  for (const file of [lifecycle, NODE + "network/runtime_authority/effects.rs", NODE + "network/runtime_authority.rs"]) {
    if (/\.authority\s*\.\s*lock\s*\(/.test(ordinaryRust(text(file)))) failures.push(`${file}: direct lock bypasses poison settlement`);
  }
  const readiness = NODE + "network/runtime_authority/filter_index/readiness.rs";
  if (!compact(ordinaryRust(text(readiness))).includes("MAX_BASIC_FILTER_WAITERS:usize=64;")) failures.push(`${readiness}: 64 waiter admission policy`);
  for (const [file, symbol] of [[NODE + "chainstate/validation_history.rs", "AcceptedValidationReceipt"], [NODE + "storage/validation_history.rs", "AcceptedValidationBatch"]]) {
    const fields = ordinaryRust(text(file)).match(new RegExp(`struct ${symbol}\\s*\\{([^}]+)\\}`))?.[1];
    if (!fields || /\bpub\b/.test(fields)) failures.push(`${file}: sealed ${symbol}`);
  }
  const receipt = NODE + "chainstate/validation_history.rs";
  const constructions = [...ordinaryRust(text(receipt)).matchAll(/\bAcceptedValidationReceipt\s*\{\s*store\s*:/g)];
  if (constructions.length !== 3) failures.push(`${receipt}: only genuine connect/reorg receipt minting`);
  const metadata = NODE + "storage/fjall_store/validation_history.rs";
  const maybeMetadata = maybeBoundary(text(metadata), "check_validation_coverage_for_metadata", true);
  if (!maybeMetadata || /\bBlockValidationIdentity\s*::\s*new\s*\(/.test(maybeMetadata.body)) failures.push(`${metadata}: raw metadata identity parse must not precede or replace authentic ledger comparison`);
  const identities = NODE + "storage/validation_history.rs";
  const identitySource = text(identities);
  const identityStart = identitySource.indexOf("impl BlockValidationIdentity {");
  const identityEnd = identitySource.indexOf("\n#[derive", identityStart);
  const maybeConstructor = identityStart < 0 ? undefined : maybeBoundary(identitySource.slice(identityStart, identityEnd < 0 ? undefined : identityEnd), "new", true);
  if (!maybeConstructor || !hasOrderedCode(maybeConstructor.body, ["hash == BlockHash::from_byte_array([0; 32])", "hash == parent_hash", "(height == 0) != (parent_hash == BlockHash::from_byte_array([0; 32]))", "return Err(history_corruption(", "Ok(Self"])) failures.push(`${identities}: strict accepted identity construction`);
  const query = NODE + "storage/fjall_store/filters/query.rs";
  const maybeBody = maybeFunction(text(query), "basic_filter_point_query_guarded", true);
  if (!maybeBody || /\b(?:while|loop)\b|load_basic_filter_record\(|maybe_active_basic_filter_record\(|generate_records\(|load_block\(|load_undo\(/.test(maybeBody.body)) failures.push(`${query}: bounded request path`);
  for (const expression of ["BASIC_FILTER_QUERY_MAX_RECORD_BYTES: usize = MAX_SIZE as usize + codec::RECORD_OVERHEAD", "BASIC_FILTER_QUERY_MAX_READ_BYTES: usize = 2 * BASIC_FILTER_QUERY_MAX_RECORD_BYTES", "BASIC_FILTER_QUERY_MAX_HEX_BYTES: usize = 2 * MAX_SIZE as usize + 64", "MAX_LOGICAL_COPY_BYTES: usize = 3 * MAX_SIZE as usize + 352", "MAX_HASH_PADDED_BYTES: usize = 2 * MAX_SIZE as usize + 832"]) {
    if (!compact(ordinaryRust(text(query))).includes(compact(expression))) failures.push(`${query}: measured query byte policy`);
  }
}

function checkRegistrations(text: Texts, failures: string[]): void {
  const pairs: [string, string[]][] = [
    [NODE + "storage.rs", ["mod validation_history;"]], [NODE + "storage/fjall_store.rs", ["mod validation_history;"]],
    [NODE + "chainstate.rs", ["mod validation_history;"]], [NODE + "network.rs", ["mod validation_history;"]],
    [NODE + "network/runtime_authority.rs", ["mod validation_history;"]],
    [NODE + "network/runtime_authority/filter_index.rs", ["mod query;", "mod readiness;"]],
    [NODE + "network/runtime_authority/filter_index/readiness.rs", ["mod owner;"]],
    [NODE + "storage/fjall_store/filters.rs", ["mod query;"]],
    [NODE + "storage/fjall_store/filters/tests.rs", ['#[path = "query/tests.rs"]', "mod query;"]],
    [NODE + "sync/tests/filter_index/catch_up.rs", ['#[path = "../../../network/runtime_authority/filter_index/query/tests.rs"]', 'mod phase159_query_tests;', 'mod phase159_readiness_tests;', "mod rpc_faults;"]],
    [NODE + "sync/runtime_state.rs", ["mod tests;"]],
    [NODE + "chainstate/validation_history.rs", ["mod tests;"]],
    [NODE + "chainstate/validation_history/tests.rs", ["mod coverage;", "mod fixtures;", "mod network;"]],
    [NODE + "network/runtime_authority/filter_index/readiness/tests.rs", ["mod failures;", "mod interleavings;"]],
    ...["context", "dispatch", "http", "method"].map(module => [RPC + `${module}.rs`, ["mod filter_index;"]] as [string, string[]]),
    [RPC + "method/filter_index.rs", ["mod normalize;", "mod tests;"]], [RPC + "dispatch/filter_index.rs", ["mod tests;"]],
    [RPC + "http/tests.rs", ["mod filter_index;"]], [RPC + "http/tests/filter_index.rs", ["mod fixtures;"]],
    [RPC + "bin/open_bitcoind/tests/filter_index.rs", ['#[path = "filter_index/rpc.rs"]', "mod rpc;"]],
    [RPC + "bin/open_bitcoind/tests/filter_index/rpc.rs", ['#[path = "rpc/retention.rs"]', "mod retention;", 'mod failures;']],
  ];
  for (const [file, anchors] of pairs) {
    for (const anchor of anchors) {
      // Literal paths need the original span, but comments cannot earn registration.
      const code = rustCode(text(file));
      const found = anchor.startsWith("#[path") ? text(file).split("\n").some((line, index) => line.trim() === anchor && code.split("\n")[index].includes("#[path")) : compact(code).includes(compact(anchor));
      if (!found) failures.push(`${file}: module registration ${anchor}`);
    }
  }
}

function checkBreadcrumbs(root: string, text: Texts, failures: string[]): void {
  try {
    const manifest = JSON.parse(text("docs/parity/source-breadcrumbs.json")) as { groups: { files?: string[]; patterns?: string[]; breadcrumbs?: string[] }[] };
    const discovered = worktreePaths(root).filter(file => (file.startsWith(NODE) || file.startsWith(RPC) || file.startsWith(CLI)) && file.endsWith(".rs"));
    for (const file of new Set([...NEW_RUST, ...discovered])) {
      const exact = manifest.groups.filter(group => group.files?.includes(file));
      const matches = exact.length ? exact : manifest.groups.filter(group => group.patterns?.some(pattern => {
        const expression = pattern.replace(/[.+^${}()|[\]\\]/g, "\\$&").replaceAll("**", "DOUBLESTAR").replaceAll("*", "[^/]*").replaceAll("DOUBLESTAR", ".*");
        return new RegExp(`^${expression}$`).test(file);
      }));
      if (matches.length !== 1 || (NEW_RUST.includes(file) && (!exact.length || !matches[0].breadcrumbs?.length))) { failures.push(`${file}: exact breadcrumb registration`); continue; }
      const source = text(file) || readFileSync(resolve(root, file), "utf8");
      const comments = [...source.matchAll(/^\/\/ - (packages\/bitcoin-knots\/[^\n]+)$/gm)].map(match => match[1]);
      if (JSON.stringify(comments) !== JSON.stringify(matches[0].breadcrumbs ?? [])) failures.push(`${file}: exact breadcrumb anchors`);
    }
  } catch { failures.push("docs/parity/source-breadcrumbs.json: breadcrumb inventory invalid"); }
}

function checkParity(text: Texts, failures: string[]): void {
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const rows = index.checklist.surfaces.filter((row: { requirements: string[] }) => row.requirements.some(req => ["CFRP-01", "CFRP-02"].includes(req)));
    const row = rows[0];
    if (index.baseline !== "29.3.knots20260210" || rows.length !== 1 || row.id !== SURFACE
      || JSON.stringify(row.requirements) !== '["CFRP-01","CFRP-02"]' || !["in_progress", "done"].includes(row.status)) {
      failures.push("docs/parity/index.json: unique scoped CFRP parity owner"); return;
    }
    for (const file of [...NEW_RUST, DOC, ...REPORTS, "scripts/check-phase159-filter-rpcs.ts", "scripts/check-phase159-filter-rpcs.test.ts", "scripts/verify.sh"]) {
      if (!row.evidence.includes(file)) failures.push(`docs/parity/index.json: parity evidence ${file}`);
    }
    for (const file of UPSTREAM) if (!row.upstream.sources.includes(file)) failures.push(`docs/parity/index.json: pinned parity source ${file}`);
    for (const term of ["UnknownLegacy", "resource-policy", "64"]) {
      if (!row.intentional_differences.some((item: string) => item.includes(term))) failures.push(`docs/parity/index.json: intentional parity difference ${term}`);
    }
    if (!row.known_gaps.some((item: string) => item.includes("CFPR-02") && item.includes("CFNET") && item.includes("deferred"))) failures.push("docs/parity/index.json: later-phase parity exclusions");
    checkClaims(JSON.stringify(row), "docs/parity/index.json", failures);
  } catch { failures.push("docs/parity/index.json: invalid parity manifest"); }
}

function checkDocuments(text: Texts, failures: string[]): void {
  for (const file of GUIDANCE) checkClaims(text(file), file, failures);
  for (const term of ["UnknownLegacy", "33,554,602", "67,109,204", "67,108,928", "64 pending", "402 original", "11 replacements", "1,126-byte", "451,352", "450,226", "synthetic genesis", "maturity one", "1000", "cfg(test)", "CFRP-01/02", "root-owned gates", "not an entire", "synthetic codec", "not latency guarantees"]) {
    if (!text(DOC).includes(term)) failures.push(`${DOC}: honest scope/resource disclosure ${term}`);
  }
  for (const file of ["README.md", "packages/README.md"]) {
    if (!text(file).includes("#authenticated-basic-queries-phase-159") || !text(file).includes("UnknownLegacy")) failures.push(`${file}: current contributor query contract`);
  }
  for (const command of ["cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --", "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --", "bazel run //packages/open-bitcoin-rpc:open_bitcoind --", "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --", "-regtest -datadir=", "getindexinfo 'basic block filter index'", "getblockfilter '<known-block-hash>' basic"]) {
    if (!text(DOC).includes(command)) failures.push(`${DOC}: repo-local UAT ${command}`);
  }
  for (const [file, target] of [["packages/open-bitcoin-cli/BUILD.bazel", "open_bitcoin_cli"], ["packages/open-bitcoin-rpc/BUILD.bazel", "open_bitcoind"]]) {
    if (!text(file).includes(`name = "${target}"`)) failures.push(`${file}: actual UAT target`);
  }
  for (const file of REPORTS) {
    if (!text(file).includes("phase_lifecycle_id: 159-2026-10-09T16-11-49")) failures.push(`${file}: current lifecycle evidence`);
  }
}

function checkClaims(source: string, file: string, failures: string[]): void {
  const subjects = /peer (?:filter )?serving|CFPR-02|CFNET(?:-\d+)?|CFOP(?:-\d+)?|CFGR(?:-\d+)?|v2\.5|production readiness|production[- ]funds (?:use|safety)/gi;
  const positive = /\b(?:is|are|has|have)\s+(?:(?:now|fully|already)\s+)*(?:enabled|supported|available|ready|shipped|safe|established|complete)|\bpassed\b/i;
  for (const clause of source.split(/(?<=[.!?])\s+|\n|;|\bbut\b|\bhowever\b|\band\b|\bwhile\b/)) {
    const matches = [...clause.matchAll(subjects)];
    for (const [index, match] of matches.entries()) {
      const end = matches[index + 1]?.index ?? clause.length;
      const before = clause.slice(index === 0 ? 0 : matches[index - 1].index + matches[index - 1][0].length, match.index);
      if (positive.test(clause.slice(match.index, end)) && !/\b(?:does not|do not|no claim|not a)\b/i.test(before)) failures.push(`${file}: unsupported claim`);
    }
    if (/All private faults were injected through daemon HTTP|32 MiB filter is consensus-generated|Entire HTTP response memory is capped|Default automatic pruning is proven/i.test(clause)) failures.push(`${file}: unsupported claim`);
  }
}

function checkVerifier(source: string, failures: string[]): void {
  const documented = source.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "";
  const executed = source.split("\n").filter(line => line.startsWith("run_step ")).join("\n");
  const commands = [
    ...["154-basic-filters", "155-filter-index", "156-prune-coordination", "157-index-catch-up", "158-validated-reorg", "159-filter-rpcs"].flatMap(name => [`bun test ./scripts/check-phase${name}.test.ts`, `bun run scripts/check-phase${name}.ts`]),
    "bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts",
  ];
  for (const [label, section] of [["documented", documented], ["executed", executed]]) {
    const actual = section.split("\n").filter(line => commands.some(command => line.endsWith(command)));
    if (actual.length !== commands.length || actual.some((line, i) => !line.endsWith(commands[i]))) failures.push(`scripts/verify.sh: ordered historical ${label} verifier`);
  }
  const workspace = 'run_step "cargo test" cargo test --manifest-path packages/Cargo.toml --workspace --all-features';
  const client = 'run_step "cargo test CLI RPC client binary" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features';
  if (!source.includes(`${workspace}\n${client}\n`)) failures.push("scripts/verify.sh: required CLI binary test execution immediately after workspace tests");
}

function checkCliConversion(text: Texts, failures: string[]): void {
  const entrypoint = CLI + "main.rs";
  const maybeMain = maybeBoundary(text(entrypoint), "main", true);
  if (!maybeMain || !/\bclient\s*::\s*run_cli\s*\(/.test(maybeMain.body)) failures.push(`${entrypoint}: actual CLI client entrypoint`);
  const file = CLI + "client.rs";
  const checks: [string, RegExp, string][] = [
    ["method_call_to_json", /BlockFilterSelection::Basic\s*=>\s*"basic"/g, "BlockFilterSelection::Basic"],
    ["method_call_to_json", /BlockFilterSelection::V0\s*=>\s*"v0"/g, "BlockFilterSelection::V0"],
    ["method_call_to_json", /format!\("\{byte:02x\}"\)/g, "format!"],
    ["method_call_to_json", /serde_json::json!\(\{\s*"blockhash"\s*:\s*blockhash\s*,\s*"filtertype"\s*:\s*filtertype\s*\}\)/g, "serde_json::json!"],
    ["method_call_to_json", /Some\(name\)\s*=>\s*serde_json::json!\(\{\s*"index_name"\s*:\s*name\s*\}\)/g, "Some(name)"],
    ["post_json", /\.header\("Authorization",\s*&self\.authorization_header\)/g, ".header("],
  ];
  for (const [symbol, pattern, prefix] of checks) {
    const source = text(file);
    const maybeBody = maybeBoundary(source, symbol, true);
    const original = maybeBody ? source.slice(maybeBody.start, maybeBody.end) : "";
    const code = rustCode(original);
    if (![...original.matchAll(pattern)].some(match => code.slice(match.index).startsWith(prefix))) failures.push(`${file}:${symbol}: actual CLI wire field/type/auth literal`);
  }
}

if (import.meta.main) {
  const failures = checkPhase159FilterRpcs();
  if (failures.length) {
    console.error("Phase 159 scoped source/evidence/claim check failed:");
    for (const failure of failures.slice(0, 40)) console.error(`- ${failure}`);
    if (failures.length > 40) console.error(`- ${failures.length - 40} additional bounded findings`);
    process.exit(1);
  }
  console.log(`Phase 159 scoped production boundaries, ${TESTS.length} named behavioral selectors, exact breadcrumbs and historical verifier order checked; full native/source/security/lifecycle gates remain independent.`);
}

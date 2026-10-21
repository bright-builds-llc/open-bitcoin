import { afterEach, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import {
  checkPhase127AuthoritativeNetworkStateUnification,
  PHASE127_TARGET_FILES,
} from "../check-phase127-authoritative-network-state-unification.ts";
import { readSourceCorpus, readSourceRoot } from "../source-corpus.ts";

const repoRoot = path.resolve(import.meta.dir, "../..");
const integrationFile = "packages/open-bitcoin-rpc/tests/black_box_parity.rs";
const blockPredicate = "matches!(message, WireNetworkMessage::Block(block) if block_hash(&block.header) == expected_hash)";
const servedAssertion = "assert!(matches_block(&served, expected_hash));";
const mixedAssertion = "assert!(matches_block(&mixed_block_response, expected_hash));";
const proofDiagnostic = "P127 production proof: integration must cover shared mutation, restart serving, frozen schemas, and redaction";
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) rmSync(root, { force: true, recursive: true });
});

function corpus(): Map<string, string> {
  return new Map(PHASE127_TARGET_FILES.map(file => [file,
    file === "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs"
      || file === "scripts/check-phase127-authoritative-network-state-unification.ts"
      || file === "scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts"
      ? readSourceRoot(repoRoot, file) : readSourceCorpus(repoRoot, file),
  ]));
}

function writeCorpus(files: Map<string, string>): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase127-proof-"));
  tempRoots.push(root);
  for (const [file, source] of files) {
    const destination = path.join(root, file);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, source);
  }
  return root;
}

test("phase165 served-block current owning helper passes", () => {
  // Arrange
  const root = writeCorpus(corpus());
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toEqual([]);
});

for (const [label, needle, replacement] of [
  ["wrong wire variant", blockPredicate, blockPredicate.replace("::Block(block)", "::Tx(block)")],
  ["wrong header hash", blockPredicate, blockPredicate.replace("== expected_hash", "!= expected_hash")],
  ["unconditional helper", blockPredicate, "true"],
  ["comment helper decoy", blockPredicate, `/* ${blockPredicate} */ true`],
  ["literal helper decoy", blockPredicate, `let decoy = r#"${blockPredicate}"#; true`],
  ["missing helper", "pub(super) fn matches_block(", "pub(super) fn disconnected_block("],
  ["missing served assertion", servedAssertion, ""],
  ["missing mixed assertion", mixedAssertion, ""],
  ["comment served assertion", servedAssertion, `/* ${servedAssertion} */`],
  ["literal mixed assertion", mixedAssertion, `let decoy = r#"${mixedAssertion}"#;`],
  ["wrong served hash", servedAssertion, servedAssertion.replace("expected_hash", "other_hash")],
  ["disconnected mixed assertion", mixedAssertion, "assert!(true);"],
] as const) {
  test(`phase165 served-block proof refuses ${label}`, () => {
    // Arrange
    const files = corpus();
    expect(checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files))).toEqual([]);
    const source = files.get(integrationFile) ?? "";
    expect(source.split(needle)).toHaveLength(2);
    let changed = source.replace(needle, replacement);
    if (label === "disconnected mixed assertion") {
      changed += `\nfn disconnected_proof() { ${mixedAssertion} }\n`;
    }
    expect(changed).not.toBe(source);
    files.set(integrationFile, changed);
    // Act
    const failures = checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files));
    // Assert
    expect(failures).toContain(proofDiagnostic);
  });
}

const DAEMON = "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs";
const CANONICAL = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs";
const AUTHORITY_FAILURE = "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime";
test.each([
  ["unproved context factory", CANONICAL, "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)", "ManagedRpcContext::from_fixture_runtime_config(runtime, opened)"],
  ["missing-owner refusal removed", CANONICAL, "let Some(opened) = maybe_opened else {\n        // There is no earned transient canonical owner in this phase.\n        return Err(compatibility_refusal(runtime, chain));\n    };", "let Some(opened) = maybe_opened else { return Ok(forged_runtime()); };"],
  ["borrowed owner", CANONICAL, "runtime, opened)", "runtime, &opened)"],
  ["cross-owner assembly", CANONICAL, "assemble_durable_runtime(runtime, sync_runtime)?", "assemble_durable_runtime(runtime, other_runtime)?"],
  ["unearned prebuilt context", CANONICAL, "authoritative.maybe_prebuilt_context = Some(context);", "authoritative.maybe_prebuilt_context = Some(default_context());"],
  ["preflight swallowed", CANONICAL, "let chain = preflight(runtime)?;\n    let Some(opened) = maybe_opened", "let chain = selected_chain_without_preflight(runtime);\n    let Some(opened) = maybe_opened"],
  ["activation guard disabled", CANONICAL, "if activation {", "if false {"],
  ["early preflight return", CANONICAL, "let activation =", "return Ok(chain); let activation ="],
  ["test-only factory", CANONICAL, "pub(super) fn open_authoritative_network_runtime(", "#[cfg(test)]\n#[allow(dead_code)]\npub(super) fn open_authoritative_network_runtime("],
  ["test-only module", DAEMON, '#[path = "open_bitcoind/canonical_startup.rs"]', '#[cfg(test)]\n#[allow(dead_code)]\n#[path = "open_bitcoind/canonical_startup.rs"]'],
  ["disconnected module", DAEMON, '#[path = "open_bitcoind/canonical_startup.rs"]', '#[path = "open_bitcoind/disconnected.rs"]'],
  ["test-only imported factory", DAEMON, "use canonical_startup::{open_authoritative_network_runtime, open_runtime_store};", "#[cfg(test)]\nuse canonical_startup::{open_authoritative_network_runtime, open_runtime_store};"],
  ["forged Some arm", DAEMON, "Some(context) => context,", "Some(_context) => default_context(),"],
  ["borrowed prebuilt context", DAEMON, "match authoritative_runtime.maybe_prebuilt_context.take()", "match authoritative_runtime.maybe_prebuilt_context.as_ref()"],
  ["ordinary None store replaced", DAEMON, "            maybe_runtime_store.clone(),", "            None,"],
  ["shared context replaced", DAEMON, "let shared_context = Arc::new(tokio::sync::Mutex::new(context));", "let shared_context = Arc::new(tokio::sync::Mutex::new(default_context()));"],
  ["disconnected application", DAEMON, "    run_application()\n}", "    disconnected_application()\n}"],
  ["cross-owner opened storage", DAEMON, "let maybe_runtime_store = open_runtime_store(&runtime)?;", "let unused = open_runtime_store(&runtime)?; let maybe_runtime_store = other_storage();"],
  ["cross-owner served runtime", DAEMON, "match opened {", "match replacement_opened {"],
] as const)("rejects actual canonical authority mutation: %s", (_label, file, needle, replacement) => {
  // Arrange
  expect(checkPhase127AuthoritativeNetworkStateUnification(repoRoot)).toEqual([]);
  const root = createFixture({ maybeMutate(files) { replaceUnique(files, file, needle, replacement); } });
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toContain(AUTHORITY_FAILURE);
});

test("retains the direct ordinary context and async entry composition form", () => {
  // Arrange
  const files = corpus();
  files.set(DAEMON, `async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())? {
      OpenedAuthoritativeRuntime::Durable(authoritative_runtime) => {
        serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime).await
      }
      OpenedAuthoritativeRuntime::Transient(authoritative_runtime) => {
        serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime).await
      }
    }
  }
  async fn serve_authoritative_runtime<S, V>() {
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
      &runtime, authoritative_runtime.network.clone(), maybe_runtime_store.clone(),
    )?;
    let shared_context = Arc::new(tokio::sync::Mutex::new(context));
  }
  fn open_authoritative_network_runtime(sync_runtime: DurableSyncRuntime) {
    AuthoritativeNetworkRuntime {
      network: sync_runtime.network_handle(),
      peer_identity_authority: sync_runtime.peer_identity_authority(),
      announcement_outboxes: sync_runtime.announcement_outboxes(),
      maybe_sync_runtime: Some(sync_runtime),
    }
  }`);
  const root = writeCorpus(files);
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [CANONICAL, "pub(super) fn open_authoritative_network_runtime(", "\n#[cfg(test)]\npub(super) fn basic_filter_startup_mode", "Some(context)", "Some(default_context())"],
  [DAEMON, "async fn run_application()", "\nasync fn serve_authoritative_runtime<", "match opened {", "match other_opened {"],
  [DAEMON, "async fn serve_authoritative_runtime<", "\n#[derive(Debug, Clone, PartialEq, Eq)]\nstruct DaemonSyncPreflightError", "Some(context) => context,", "Some(context) => default_context(),"],
].flatMap(([file, marker, boundary, needle, replacement]) =>
  ["test", "dead", "comment", "literal"].map(mode => [`${marker} ${mode}`, file, marker, boundary, needle, replacement, mode] as const),
))("rejects real authority mutation despite expected decoy: %s", (_label, file, marker, boundary, needle, replacement, mode) => {
  // Arrange
  const root = createFixture({ maybeMutate(files) {
    const original = files.get(file as TargetFile) ?? "";
    const start = original.indexOf(marker);
    const end = original.indexOf(boundary, start);
    expect(start).toBeGreaterThan(-1);
    expect(end).toBeGreaterThan(start);
    const expected = original.slice(start, end);
    replaceUnique(files, file as TargetFile, needle, replacement);
    const changed = files.get(file as TargetFile) ?? "";
    const decoy = mode === "test" ? `#[cfg(test)]\n${expected}`
      : mode === "dead" ? `fn disconnected_owner() { ${expected} }`
      : mode === "comment" ? `/* ${expected} */` : `const DECOY: &str = r###"${expected}"###;`;
    files.set(file as TargetFile, changed.slice(0, start) + decoy + "\n" + changed.slice(start));
  } });
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toContain(AUTHORITY_FAILURE);
});

test("fixed helper ownership is invariant under staging in an owned scratch repository", () => {
  // Arrange
  const root = realpathSync(createFixture());
  expect(path.dirname(root)).toBe(realpathSync(tmpdir()));
  expect(path.basename(root).startsWith("open-bitcoin-phase127-")).toBe(true);
  expect(existsSync(path.join(root, ".git"))).toBe(false);
  const helper = readFileSync(path.join(root, CANONICAL), "utf8");
  const decoy = "packages/open-bitcoin-rpc/src/bin/open-bitcoind/canonical_startup.rs";
  mkdirSync(path.dirname(path.join(root, decoy)), { recursive: true });
  writeFileSync(path.join(root, decoy), helper);
  const git = (...args: string[]) => {
    const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
    expect(result.exitCode).toBe(0);
    return Buffer.from(result.stdout).toString();
  };
  git("init", "--quiet");
  expect(git("ls-files", "--", CANONICAL, decoy)).toBe("");
  const untracked = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Act
  git("add", "--", CANONICAL, decoy);
  const tracked = git("ls-files", "--", CANONICAL, decoy).trim().split("\n");
  const staged = checkPhase127AuthoritativeNetworkStateUnification(root);
  writeFileSync(path.join(root, CANONICAL), helper.replace("Some(context)", "Some(default_context())"));
  const changedOwner = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(tracked.sort()).toEqual([CANONICAL, decoy].sort());
  expect(untracked).toEqual([]);
  expect(staged).toEqual(untracked);
  expect(changedOwner).toContain(AUTHORITY_FAILURE);
});

test("a broken declared GEN factory cannot fall back to the ordinary None constructor", () => {
  // Arrange
  const files = corpus();
  replaceUnique(files, CANONICAL, "authoritative.maybe_prebuilt_context = Some(context);", "authoritative.maybe_prebuilt_context = Some(default_context());");
  replaceUnique(files, DAEMON, `let context = match authoritative_runtime.maybe_prebuilt_context.take() {
        Some(context) => context,
        None => ManagedRpcContext::from_runtime_config_with_network_handle(
            &runtime,
            authoritative_runtime.network.clone(),
            maybe_runtime_store.clone(),
        )?,
    };`, "let context = ManagedRpcContext::from_runtime_config_with_network_handle(&runtime, authoritative_runtime.network.clone(), maybe_runtime_store.clone(),)?;");
  const root = writeCorpus(files);
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toContain(AUTHORITY_FAILURE);
});

test("runtime proof effects remain rejected while the test-only scratch Git control stays excluded", () => {
  // Arrange
  const files = corpus();
  const helper = "scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts";
  const original = files.get(helper) ?? "";
  expect(checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files))).toEqual([]);
  files.set(helper, `${original}\nBun.spawnSync([]);\n`);
  expect(files.get(helper)).not.toBe(original);
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files));
  // Assert
  expect(failures).toContain("P127 deterministic scope: checker must remain local and public-network-free");
});

test("missing fixed runtime proof fails closed", () => {
  // Arrange
  const root = writeCorpus(corpus());
  const helper = "scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts";
  rmSync(path.join(root, helper));
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toContain(`P127 missing target: ${helper}`);
});

test.each([
  ["path", '#[path = "open_bitcoind/canonical_startup.rs"]\nmod canonical_startup;', '#[path = "open_bitcoind/disconnected.rs"]\nmod canonical_startup;'],
  ["tokio", "#[tokio::main]\nasync fn run_application(", "async fn run_application("],
].flatMap(([label, expected, replacement]) => ["comment", "literal"].map(mode => [label, mode, expected, replacement] as const)))(
  "rejects real %s owner mutation with expected %s attribute decoy", (_label, mode, expected, replacement) => {
    // Arrange
    const files = corpus();
    replaceUnique(files, DAEMON, expected, replacement);
    const decoy = mode === "comment" ? `/* ${expected} */` : `const DECOY: &str = r###"${expected}"###;`;
    files.set(DAEMON, `${decoy}\n${files.get(DAEMON)}`);
    const root = writeCorpus(files);
    // Act
    const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
    // Assert
    expect(failures).toContain(AUTHORITY_FAILURE);
  },
);

type TargetFile = (typeof PHASE127_TARGET_FILES)[number];
function createFixture(options: { maybeMutate?: (files: Map<string, string>) => void } = {}): string {
  const files = corpus();
  options.maybeMutate?.(files);
  return writeCorpus(files);
}
function replaceUnique(files: Map<string, string>, file: string, needle: string, replacement: string): void {
  const source = files.get(file) ?? "";
  expect(source.split(needle)).toHaveLength(2);
  expect(replacement).not.toBe(needle);
  const changed = source.replace(needle, () => replacement);
  expect(changed).not.toBe(source);
  files.set(file, changed);
}

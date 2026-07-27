#!/usr/bin/env bun

import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import {
  normalizeRust,
  rustAssociatedCallMethods,
  rustCallArguments,
  rustFunction,
  rustLetInitializers,
  rustMatchDiscriminants,
  rustStructLiteralFieldInitializers,
  stripRustNonCode,
} from "./rust-source-invariants";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE127_TEST =
  "bun test scripts/check-phase127-authoritative-network-state-unification.test.ts";
const PHASE127_CHECK =
  "bun run scripts/check-phase127-authoritative-network-state-unification.ts";
const PHASE126_CHECK =
  "bun run scripts/check-phase126-compact-relay-residual-hardening.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

export const PHASE127_DAEMON_HELPER_DIR =
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind";

export const PHASE127_TARGET_FILES = [
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/sync_seed.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/inbound_status.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "scripts/check-phase127-authoritative-network-state-unification.ts",
  "scripts/rust-source-invariants.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof PHASE127_TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;

export function checkPhase127AuthoritativeNetworkStateUnification(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE127_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  checkProductionAuthority(repoRoot, texts, failures);
  checkDurableServing(texts, failures);
  checkOperatorProjection(texts, failures);
  checkIntegrationAndParity(texts, failures);
  checkVerifier(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<TargetFile, string>();
  for (const file of PHASE127_TARGET_FILES) {
    const absolutePath = path.join(repoRoot, file);
    if (!existsSync(absolutePath)) {
      failures.push(`P127 missing target: ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readSourceCorpus(repoRoot, file));
  }
  return texts;
}

function checkProductionAuthority(
  repoRoot: string,
  texts: TextCorpus,
  failures: string[],
): void {
  const daemon =
    texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  const main = rustFunction(
    daemon,
    "async fn main() -> Result<(), Box<dyn std::error::Error>>",
  );
  const authoritativeRuntimeInitializers = rustLetInitializers(
    main,
    "authoritative_runtime",
  );
  const sharedContextStart = main.indexOf("let shared_context");
  const contextInitializers = rustLetInitializers(
    sharedContextStart === -1 ? "" : main.slice(0, sharedContextStart),
    "context",
  );
  const sharedContextInitializers = rustLetInitializers(main, "shared_context");
  const authorityFactory = rustFunction(
    daemon,
    "fn open_authoritative_network_runtime(",
  );
  const daemonCodeWithoutAuthorityFactory = stripRustNonCode(daemon).replace(
    authorityFactory,
    "",
  );
  const productionDaemonCode = [
    daemonCodeWithoutAuthorityFactory,
    ...productionDaemonHelperSources(repoRoot).map(stripRustNonCode),
  ].join("\n");
  const handleConstructors = rustAssociatedCallMethods(
    productionDaemonCode,
    "ManagedNetworkHandle",
  );
  const peerNetworkMethods = rustAssociatedCallMethods(
    productionDaemonCode,
    "ManagedPeerNetwork",
  );
  if (
    authoritativeRuntimeInitializers.length !== 1 ||
    normalizeRust(authoritativeRuntimeInitializers[0] ?? "") !==
      "open_authoritative_network_runtime(&runtime,maybe_runtime_store.clone())?" ||
    countOccurrences(main, "open_authoritative_network_runtime(") !== 1 ||
    contextInitializers.length !== 1 ||
    normalizeRust(contextInitializers[0] ?? "") !==
      "ManagedRpcContext::from_runtime_config_with_network_handle(&runtime,authoritative_runtime.network.clone(),maybe_runtime_store.clone(),)?" ||
    sharedContextInitializers.length !== 1 ||
    normalizeRust(sharedContextInitializers[0] ?? "") !==
      "Arc::new(tokio::sync::Mutex::new(context))" ||
    handleConstructors.some((method) =>
      ["transient_runtime", "from_network_fixture"].includes(method),
    ) ||
    peerNetworkMethods.length !== 0
  ) {
    failures.push(
      "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime",
    );
  }
  if (main.includes("MemoryChainstateStore")) {
    failures.push(
      "P127 fresh RPC chainstate: daemon production composition must not allocate MemoryChainstateStore",
    );
  }

  const context =
    texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  const injected = rustFunction(
    context,
    "pub fn from_runtime_config_with_network_handle(",
  );
  if (
    !injected.includes("network: ManagedNetworkHandle") ||
    !injected.includes("network,") ||
    [
      "MemoryChainstateStore",
      "ManagedPeerNetwork::new",
      "ManagedNetworkHandle::transient_runtime(",
      "ManagedNetworkHandle::from_network_fixture(",
    ].some((forbidden) => injected.includes(forbidden))
  ) {
    failures.push(
      "P127 fresh RPC chainstate: injected production context must retain the supplied network handle",
    );
  }
}

function checkDurableServing(texts: TextCorpus, failures: string[]): void {
  const context = texts.get("packages/open-bitcoin-rpc/src/context.rs") ?? "";
  const contextCode = stripRustNonCode(context);
  const store =
    texts.get("packages/open-bitcoin-node/src/storage/fjall_store.rs") ?? "";
  const storeLoad = rustFunction(store, "pub fn load_block(");
  const resolve = rustFunction(context, "fn resolve_block_intent(");
  const maybeBlockInitializers = rustLetInitializers(resolve, "maybe_block");
  const blockInitializers = rustLetInitializers(resolve, "block");
  const blockMatchDiscriminants = blockInitializers.flatMap(
    rustMatchDiscriminants,
  );
  const blockResponseCalls = rustCallArguments(resolve, "block_serve_response");
  if (
    !contextCode.includes("trait DurableBlockSource: Send + Sync") ||
    !contextCode.includes("impl DurableBlockSource for FjallNodeStore") ||
    !contextCode.includes("FjallNodeStore::load_block(self, block_hash)") ||
    maybeBlockInitializers.length !== 1 ||
    normalizeRust(maybeBlockInitializers[0] ?? "") !==
      "self.maybe_block_source.as_ref().map(|source|source.load_block(intent.block_hash()))" ||
    blockInitializers.length !== 1 ||
    blockMatchDiscriminants.length !== 1 ||
    normalizeRust(blockMatchDiscriminants[0] ?? "") !== "maybe_block" ||
    blockResponseCalls.length !== 1 ||
    normalizeRust(blockResponseCalls[0]?.[0] ?? "") !== "block" ||
    [
      "lookup_block(",
      "blocks_by_hash",
      "block_cache",
      "cached_block",
      "lookup_cached",
    ].some((forbidden) => resolve.includes(forbidden))
  ) {
    failures.push(
      "P127 durable serving: production block resolution must use the request-scoped durable source",
    );
  }
  if (
    !storeLoad.includes("pub fn load_block(&self, block_hash: BlockHash)") ||
    !storeLoad.includes(
      "self.get_bytes(StorageNamespace::BlockIndex, &block_key(block_hash))?",
    )
  ) {
    failures.push(
      "P127 durable serving store: Fjall must remain the persisted block-body authority",
    );
  }
}

function checkOperatorProjection(texts: TextCorpus, failures: string[]): void {
  const authority =
    texts.get("packages/open-bitcoin-node/src/network/runtime_authority.rs") ??
    "";
  const inbound =
    texts.get("packages/open-bitcoin-rpc/src/context/inbound_status.rs") ?? "";
  const dispatch =
    texts.get("packages/open-bitcoin-rpc/src/dispatch/node.rs") ?? "";
  const authoritySnapshot = rustFunction(
    authority,
    "pub fn operator_snapshot(&self)",
  );
  const inboundProjection = rustFunction(
    inbound,
    "pub fn authoritative_operator_snapshot(",
  );
  const networkInfoProjection = rustFunction(
    dispatch,
    "pub(super) fn get_network_info(",
  );
  const networkStatusProjection = rustFunction(
    dispatch,
    "pub(super) fn open_bitcoin_network_status(",
  );
  const statusResultCalls = rustCallArguments(networkStatusProjection, "Ok");
  const returnedStatus =
    statusResultCalls.length === 1 && statusResultCalls[0]?.length === 1
      ? (statusResultCalls[0]?.[0] ?? "")
      : "";
  if (
    !authoritySnapshot.includes(
      "self.read(ManagedPeerNetwork::operator_snapshot)",
    ) ||
    !hasSingleRustLetInitializer(
      inboundProjection,
      "network",
      "self.network.operator_snapshot()?",
    ) ||
    !hasSingleRustLetInitializer(
      inboundProjection,
      "inbound",
      "self.inbound_status_from_snapshot(&network)",
    ) ||
    !inboundProjection.includes(
      "Ok(AuthoritativeOperatorSnapshot { network, inbound })",
    ) ||
    !hasSingleRustLetInitializer(
      networkInfoProjection,
      "snapshot",
      "context.authoritative_operator_snapshot().map_err(network_authority_error_to_failure)?",
    ) ||
    !hasSingleRustLetInitializer(
      networkInfoProjection,
      "network_info",
      "snapshot.network()",
    ) ||
    !hasSingleRustLetInitializer(
      networkInfoProjection,
      "mempool_info",
      "snapshot.mempool()",
    ) ||
    !hasSingleRustLetInitializer(
      networkStatusProjection,
      "snapshot",
      "context.authoritative_operator_snapshot().map_err(network_authority_error_to_failure)?",
    ) ||
    !hasSingleRustStructFieldInitializer(
      returnedStatus,
      "OpenBitcoinNetworkStatusResponse",
      "inbound",
      "snapshot.inbound().clone()",
    ) ||
    !hasSingleRustStructFieldInitializer(
      returnedStatus,
      "OpenBitcoinNetworkStatusResponse",
      "relay",
      "snapshot.relay().clone()",
    ) ||
    !hasSingleRustStructFieldInitializer(
      returnedStatus,
      "OpenBitcoinNetworkStatusResponse",
      "block_relay",
      "snapshot.block_relay().clone()",
    )
  ) {
    failures.push(
      "P127 authoritative projection: RPC and operator status must use one owned network snapshot",
    );
  }
  if (
    [inboundProjection, networkInfoProjection, networkStatusProjection].some(
      (projection) =>
        projection.includes("ManagedNetworkOperatorSnapshot::default()") ||
        projection.includes("block_relay_evidence_status()"),
    )
  ) {
    failures.push(
      "P127 authoritative projection: direct block-relay projection must not bypass the owned snapshot",
    );
  }
}

function checkIntegrationAndParity(
  texts: TextCorpus,
  failures: string[],
): void {
  const integration =
    texts.get("packages/open-bitcoin-rpc/tests/black_box_parity.rs") ?? "";
  const dashboard =
    texts.get(
      "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
    ) ?? "";
  const support =
    texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  const integrationAnchors = [
    "phase127_production_composition_shares_sync_serving_and_operator_authority",
    "let pre_sync_context = ManagedRpcContext::from_runtime_config_with_network_handle(",
    ".sync_once(&mut transport",
    ".load_block(expected_hash)",
    "start_inbound_accept_loop(",
    "WireNetworkMessage::Block(ref served_block)",
    "phase127_mixed_missing_transaction_block_request",
    "let mixed_block_response = peer.receive().await;",
    "let mixed_not_found_response = peer.receive().await;",
    'sorted_result_keys(&status_response),\n        ["block_relay", "inbound", "metrics", "relay"]',
  ];
  if (!integrationAnchors.every((anchor) => integration.includes(anchor))) {
    failures.push(
      "P127 production proof: integration must cover shared mutation, restart serving, frozen schemas, and redaction",
    );
  }
  if (
    !dashboard.includes(
      "dashboard_model_block_relay_rows_surface_shared_status_contract",
    ) ||
    !dashboard.includes(
      "dashboard_model_block_relay_rows_preserve_unavailable_reason_without_sensitive_text",
    ) ||
    !support.includes(
      "support_bundle_renders_block_relay_evidence_from_shared_projection",
    ) ||
    !support.includes(
      "support_bundle_redacts_sensitive_block_relay_reasons_in_json_and_markdown",
    ) ||
    !support.includes(
      "authoritative_rpc_status_support_bundle_redacts_every_forbidden_material_class_in_json_and_markdown",
    ) ||
    !support.includes("let raw_rpc_json =") ||
    !support.includes("execute_support_command(") ||
    !support.includes('output_dir.join("support-evidence.json")') ||
    !support.includes('output_dir.join("support-evidence.md")')
  ) {
    failures.push(
      "P127 operator contracts: dashboard and support schema/redaction regressions must remain direct evidence",
    );
  }

  const catalog = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const index = texts.get("docs/parity/index.json") ?? "";
  const anchors = [
    "packages/bitcoin-knots/src/node/context.h",
    "packages/bitcoin-knots/src/rpc/server_util.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/src/validation.cpp",
    "packages/bitcoin-knots/src/node/blockstorage.cpp",
  ];
  if (
    !catalog.includes("Phase 127 authoritative network state unification") ||
    !index.includes('"id": "v2-1-authoritative-network-state-unification"') ||
    !index.includes(
      '"scripts/check-phase127-authoritative-network-state-unification.ts"',
    ) ||
    !index.includes(
      '"scripts/check-phase127-authoritative-network-state-unification.test.ts"',
    ) ||
    !anchors.every(
      (anchor) => catalog.includes(anchor) && index.includes(anchor),
    ) ||
    !catalog.includes("Phase 128") ||
    !catalog.includes("Phase 129") ||
    !index.includes("Phase 128 retains") ||
    !index.includes("Phase 129 retains")
  ) {
    failures.push(
      "P127 bounded parity: exact Knots anchors and Phase 128/129 exclusions must remain registered",
    );
  }
}

function checkVerifier(texts: TextCorpus, failures: string[]): void {
  const verify = texts.get("scripts/verify.sh") ?? "";
  if (
    !orderedLines(visibleCommandOrder(verify), [
      PHASE126_CHECK,
      PHASE127_TEST,
      PHASE127_CHECK,
      PHASE117_TEST,
    ]) ||
    !orderedLines(verify, [
      `run_step "check Phase 126 compact relay residual hardening" ${PHASE126_CHECK}`,
      `run_step "test Phase 127 authoritative network state unification checker" ${PHASE127_TEST}`,
      `run_step "check Phase 127 authoritative network state unification" ${PHASE127_CHECK}`,
      `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
    ])
  ) {
    failures.push(
      "P127 verifier order: Phase 127 must follow Phase 126 and precede the final Phase 117 gate",
    );
  }

  const checker =
    texts.get(
      "scripts/check-phase127-authoritative-network-state-unification.ts",
    ) ?? "";
  for (const forbidden of [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ]) {
    if (checker.includes(forbidden)) {
      failures.push(
        "P127 deterministic scope: checker must remain local and public-network-free",
      );
      break;
    }
  }
}

function productionDaemonHelperSources(repoRoot: string): string[] {
  const helperRoot = path.join(
    repoRoot,
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind",
  );
  if (!existsSync(helperRoot)) return [];
  return rustSourcePaths(helperRoot)
    .filter((sourcePath) => path.basename(sourcePath) !== "tests.rs")
    .map((sourcePath) => readFileSync(sourcePath, "utf8"));
}

function rustSourcePaths(directory: string): string[] {
  const paths: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const entryPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      paths.push(...rustSourcePaths(entryPath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".rs")) paths.push(entryPath);
  }
  return paths;
}

function hasSingleRustLetInitializer(
  functionText: string,
  binding: string,
  expected: string,
): boolean {
  const initializers = rustLetInitializers(functionText, binding);
  return (
    initializers.length === 1 &&
    normalizeRust(initializers[0] ?? "") === normalizeRust(expected)
  );
}

function hasSingleRustStructFieldInitializer(
  expression: string,
  structType: string,
  field: string,
  expected: string,
): boolean {
  const initializers = rustStructLiteralFieldInitializers(
    expression,
    structType,
    field,
  );
  return (
    initializers.length === 1 &&
    normalizeRust(initializers[0] ?? "") === normalizeRust(expected)
  );
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

function countOccurrences(text: string, needle: string): number {
  return needle.length === 0 ? 0 : text.split(needle).length - 1;
}

if (import.meta.main) {
  const failures = checkPhase127AuthoritativeNetworkStateUnification();
  if (failures.length > 0) {
    console.error(
      "Phase 127 authoritative network state unification check failed:",
    );
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 127 authoritative network state unification validated.");
}

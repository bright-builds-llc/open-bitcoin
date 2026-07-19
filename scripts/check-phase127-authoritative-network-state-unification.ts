#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE127_TEST =
  "bun test scripts/check-phase127-authoritative-network-state-unification.test.ts";
const PHASE127_CHECK =
  "bun run scripts/check-phase127-authoritative-network-state-unification.ts";
const PHASE126_CHECK =
  "bun run scripts/check-phase126-compact-relay-residual-hardening.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

const TARGET_FILES = [
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
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
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
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
  checkProductionAuthority(texts, failures);
  checkDurableServing(texts, failures);
  checkOperatorProjection(texts, failures);
  checkIntegrationAndParity(texts, failures);
  checkVerifier(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    const absolutePath = path.join(repoRoot, file);
    if (!existsSync(absolutePath)) {
      failures.push(`P127 missing target: ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readFileSync(absolutePath, "utf8"));
  }
  return texts;
}

function checkProductionAuthority(texts: TextCorpus, failures: string[]): void {
  const daemon =
    texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  if (
    countOccurrences(
      daemon,
      "open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())?",
    ) !== 1 ||
    !daemon.includes("network: sync_runtime.network_handle(),") ||
    !daemon.includes(
      "ManagedRpcContext::from_runtime_config_with_network_handle(",
    ) ||
    !daemon.includes("authoritative_runtime.network.clone(),")
  ) {
    failures.push(
      "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime",
    );
  }
  if (daemon.includes("MemoryChainstateStore")) {
    failures.push(
      "P127 fresh RPC chainstate: daemon production composition must not allocate MemoryChainstateStore",
    );
  }

  const context =
    texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  const injected = functionSection(
    context,
    "pub fn from_runtime_config_with_network_handle(",
    "pub fn set_inbound_listener_evidence(",
  );
  if (
    !injected.includes("network: ManagedNetworkHandle") ||
    injected.includes("MemoryChainstateStore::default()")
  ) {
    failures.push(
      "P127 fresh RPC chainstate: injected production context must retain the supplied network handle",
    );
  }
}

function checkDurableServing(texts: TextCorpus, failures: string[]): void {
  const context = texts.get("packages/open-bitcoin-rpc/src/context.rs") ?? "";
  const store =
    texts.get("packages/open-bitcoin-node/src/storage/fjall_store.rs") ?? "";
  const resolve = functionSection(
    context,
    "fn resolve_block_intent(",
    "fn push_encoded(",
  );
  if (
    !context.includes("trait DurableBlockSource: Send + Sync") ||
    !context.includes("impl DurableBlockSource for FjallNodeStore") ||
    !context.includes("FjallNodeStore::load_block(self, block_hash)") ||
    !resolve.includes("source.load_block(intent.block_hash())")
  ) {
    failures.push(
      "P127 durable serving: production block resolution must use the request-scoped durable source",
    );
  }
  if (
    !store.includes("pub fn load_block(&self, block_hash: BlockHash)") ||
    !store.includes("self.get_bytes(StorageNamespace::BlockIndex, &block_key(block_hash))?")
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
  if (
    !authority.includes(
      "pub fn operator_snapshot(&self) -> Result<ManagedNetworkOperatorSnapshot",
    ) ||
    !inbound.includes("let network = self.network.operator_snapshot()?;") ||
    countOccurrences(dispatch, ".authoritative_operator_snapshot()") < 2
  ) {
    failures.push(
      "P127 authoritative projection: RPC and operator status must use one owned network snapshot",
    );
  }
  if (
    dispatch.includes("block_relay_evidence_status()\n        .map_err") &&
    !dispatch.includes("Phase 116 compatibility anchor")
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
    'sorted_result_keys(&status_response),\n        ["block_relay", "inbound", "metrics", "relay"]',
    "PHASE127_FORBIDDEN_PERMISSION",
    "PHASE127_FORBIDDEN_TRANSACTION",
    "PHASE127_FORBIDDEN_DYNAMIC_LABEL",
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
    )
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

function functionSection(text: string, startNeedle: string, endNeedle: string): string {
  const start = text.indexOf(startNeedle);
  if (start === -1) return "";
  const end = text.indexOf(endNeedle, start + startNeedle.length);
  return text.slice(start, end === -1 ? text.length : end);
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
  console.log(
    "Phase 127 authoritative network state unification validated.",
  );
}

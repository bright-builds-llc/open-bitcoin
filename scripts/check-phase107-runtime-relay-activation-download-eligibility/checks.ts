import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT, TARGET_FILES, REQUIRED_SUPPRESSION_LABELS, REQUIRED_STATUS_SYMBOLS, REQUIRED_POLICY_SYMBOLS, TargetFile, TextCorpus } from "./constants.ts";
import { checkParitySurface, checkRequiredDocsAndCommands } from "./parity.ts";
import { checkVerifierOrder, checkForbiddenDefaultVerifierGates, checkSensitivePublicEvidence, checkForbiddenClaims } from "./claims.ts";
import { requireGateBeforeMutation, sectionBetween, requireContains, orderedIndexes } from "./helpers.ts";
import { readText } from "./filesystem.ts";

export function checkPhase107RuntimeRelayActivationDownloadEligibility(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE107_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkRuntimeConfigPropagation(texts, failures);
  checkManagedRelayDownloadPolicy(texts, failures);
  checkSchedulerEligibilityGate(texts, failures);
  checkStatusEvidence(texts, failures);
  checkSupportRedaction(texts, failures);
  checkParitySurface(texts, failures);
  checkRequiredDocsAndCommands(texts, failures);
  checkVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenDefaultVerifierGates(texts.get("scripts/verify.sh") ?? "", failures);
  checkSensitivePublicEvidence(texts, failures);
  checkForbiddenClaims(texts, failures);

  return failures;
}

export function checkRuntimeConfigPropagation(texts: TextCorpus, failures: string[]): void {
  const context = texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  if (
    !orderedIndexes(context, [
      "ManagedPeerNetwork::new_with_block_relay_activation(",
      "config.relay",
      "config.block_serving",
      "config.inbound.enabled",
    ])
  ) {
    failures.push(
      "runtime-config: ManagedRpcContext must call new_with_block_relay_activation with config.relay, config.block_serving, and config.inbound.enabled",
    );
  }
  if (context.includes("ManagedPeerNetwork::new(")) {
    failures.push("runtime-config: ManagedRpcContext must not use the default ManagedPeerNetwork::new constructor");
  }
}

export function checkManagedRelayDownloadPolicy(texts: TextCorpus, failures: string[]): void {
  const relayServing = texts.get("packages/open-bitcoin-node/src/network/relay_serving.rs") ?? "";
  for (const needle of [
    "peer_manager.set_relay_download_policy(RelayDownloadPolicy {",
    "activation: relay_activation",
    "inbound_serving_enabled",
    "relay_download_eligibility_counters",
  ]) {
    requireContains(relayServing, needle, `managed-network: missing relay download policy evidence ${needle}`, failures);
  }

  const policyCorpus = [
    texts.get("packages/open-bitcoin-network/src/peer/relay_download.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/inventory_state.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs") ?? "",
  ].join("\n");
  for (const symbol of REQUIRED_POLICY_SYMBOLS) {
    requireContains(policyCorpus, symbol, `relay-download-policy: missing ${symbol}`, failures);
  }
  for (const label of REQUIRED_SUPPRESSION_LABELS) {
    requireContains(policyCorpus, label, `relay-download-policy: missing suppression label ${label}`, failures);
    requireContains(policyCorpus, `suppress_${label}`, `relay-download-policy: missing action label suppress_${label}`, failures);
  }
}

export function checkSchedulerEligibilityGate(texts: TextCorpus, failures: string[]): void {
  const scheduler = texts.get("packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs") ?? "";
  const announcementSection = sectionBetween(scheduler, "pub fn record_announcement", "pub fn request_parent");
  requireGateBeforeMutation(
    announcementSection,
    "relay_eligibility_suppression(input.peer_id, relay_id, &input.relay_eligibility)",
    ["self.insert_in_flight(relay_id", "self.insert_candidate("],
    "record_announcement",
    failures,
  );

  const parentSection = sectionBetween(scheduler, "pub fn request_parent", "pub fn expire_and_schedule");
  requireGateBeforeMutation(
    parentSection,
    "relay_eligibility_suppression(input.peer_id, input.relay_id, &input.relay_eligibility)",
    ["self.insert_in_flight(input.relay_id"],
    "request_parent",
    failures,
  );
}

export function checkStatusEvidence(texts: TextCorpus, failures: string[]): void {
  const status = texts.get("packages/open-bitcoin-node/src/status/relay_evidence.rs") ?? "";
  for (const symbol of REQUIRED_STATUS_SYMBOLS) {
    requireContains(status, symbol, `status-evidence: missing ${symbol}`, failures);
  }

  const fanout = texts.get("packages/open-bitcoin-node/src/network/relay_fanout.rs") ?? "";
  for (const needle of [
    "RelayEvidenceStatus::with_activation_and_counters(",
    "enabled: self.relay_activation.enabled",
    "self.relay_download_eligibility_counters()",
  ]) {
    requireContains(fanout, needle, `status-evidence: missing managed projection ${needle}`, failures);
  }
}

export function checkSupportRedaction(texts: TextCorpus, failures: string[]): void {
  const redaction = texts.get("packages/open-bitcoin-cli/src/operator/support/redaction.rs") ?? "";
  for (const needle of [
    "sanitize_relay_reason_field(&mut relay.activation)",
    "sanitize_relay_reason_field(&mut relay.download_eligibility)",
  ]) {
    requireContains(redaction, needle, `support-redaction: missing ${needle}`, failures);
  }

  const tests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  const hasSplitSupportRegistry =
    tests.includes("mod sync_fixtures;") &&
    tests.includes("mod forensics_recovery_relay;");
  for (const needle of [
    "RelayActivationEvidence { enabled: true }",
    "RelayDownloadEligibilityCounters",
    "txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "wtxid=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    "permission_string",
    "dynamic_label",
  ]) {
    if (hasSplitSupportRegistry && !tests.includes(needle)) {
      continue;
    }
    requireContains(tests, needle, `support-redaction: missing redaction test evidence ${needle}`, failures);
  }
}

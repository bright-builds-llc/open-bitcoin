#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-runtime-relay-activation-download-eligibility";
const PHASE106_TEST_COMMAND = "bun test scripts/check-phase106-parity-uat-release-boundary.test.ts";
const PHASE106_CHECKER_COMMAND = "bun run scripts/check-phase106-parity-uat-release-boundary.ts";
const PHASE107_TEST_COMMAND =
  "bun test scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts";
const PHASE107_CHECKER_COMMAND =
  "bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts";
const REQUIRED_REQUIREMENTS = ["ACT-01", "ACT-02", "INV-02", "INV-03", "DL-01", "DL-02", "REL-03"] as const;
const TARGET_FILES = [
  "README.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-network/src/peer/relay_download.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.ts",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts",
  "scripts/verify.sh",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-RESEARCH.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "README.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-network/src/peer/relay_download.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.ts",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts",
  "scripts/verify.sh",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-RESEARCH.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
  "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
  "packages/bitcoin-knots/test/functional/rpc_rawtransaction.py",
] as const;
const REQUIRED_SUPPRESSION_LABELS = [
  "relay_disabled",
  "not_relay_eligible",
  "inbound_serving_required",
  "permission_required",
  "protected_not_relay",
] as const;
const REQUIRED_STATUS_SYMBOLS = [
  "RelayActivationEvidence",
  "RelayDownloadEligibilityCounters",
  "with_activation_and_counters",
  "activation: RelayEvidenceField<RelayActivationEvidence>",
  "download_eligibility: RelayEvidenceField<RelayDownloadEligibilityCounters>",
  "eligible_peer_count",
  "ineligible_peer_count",
  "relay_disabled_count",
  "inbound_serving_required_count",
  "permission_required_count",
  "protected_not_relay_count",
] as const;
const REQUIRED_POLICY_SYMBOLS = [
  "RelayDownloadPolicy",
  "set_relay_download_policy",
  "relay_download_eligibility",
  "RelayEligibilityDecision",
  "TxParentRequestInput",
  "relay_eligibility: RelayEligibilityDecision",
  "relay_eligibility: relay_eligibility.clone()",
  "relay_eligibility,",
] as const;
const REQUIRED_RUNTIME_GUIDE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
  "-openbitcoinrelay=1",
  "-openbitcoininbound=1",
  "openbitcoinnetworkstatus",
  "support bundle --output-dir=/tmp/open-bitcoin-relay-enabled-support",
  "bash scripts/verify.sh",
] as const;
const REQUIRED_DOC_NEEDLES = [
  "resolved `RuntimeConfig.relay`",
  "resolved `config.inbound.enabled`",
  "transaction download scheduling requires relay eligibility before request-state mutation",
  "aggregate, sanitized, and fixed-label only",
  "`RelayDownloadEligibilityCounters`",
  "Support evidence must not include peer ids, endpoints, permission strings, class names, txids, wtxids, raw transaction hex, credentials, or dynamic",
] as const;
const REQUIRED_GAP_TERMS = [
  "public relay by default",
  "compact block relay",
  "package relay",
  "bloom/filter serving",
  "public-network relay CI",
  "production service operation",
  "production full-node readiness",
  "production-funds wallet safety",
  "production-funds wallet use",
  "durable mempool recovery",
] as const;
const FORBIDDEN_CLAIMS = [
  "public relay by default",
  "compact block relay",
  "compact-block relay",
  "package relay",
  "bloom/filter serving",
  "public-network relay ci",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
  "production-readiness proof",
  "production-funds wallet safety",
  "production-funds wallet use",
  "durable mempool recovery",
  "public propagation",
] as const;
const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "not ",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "future",
  "later",
  "remain",
  "remains",
  "no claim",
  "not claim",
  "not supported",
  "only",
  "bounded",
  "opt-in",
  "separate",
] as const;
const POSITIVE_CLAIM_PATTERNS = [
  /\bsupports?\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bproves?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis available\b/,
  /\bis complete\b/,
  /\bis ready\b/,
] as const;
const FORBIDDEN_DEFAULT_VERIFIER_GATES = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 86400",
  "sleep 259200",
  "public-network",
  "wall-clock",
  "service-manager",
  "production-deployment",
  "production-funds",
] as const;
const SENSITIVE_PUBLIC_EVIDENCE_PATTERNS = [
  /txid=[0-9a-f]{64}/i,
  /wtxid=[0-9a-f]{64}/i,
  /\bpeer_id=\d+/i,
  /020000000001/i,
  /\bpermission_string=/i,
  /\bcredential=/i,
  /\bsecret=/i,
  /\bcookie=/i,
  /\bdynamic_label=/i,
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  known_gaps?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  suspected_unknowns?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};

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

function checkRuntimeConfigPropagation(texts: TextCorpus, failures: string[]): void {
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

function checkManagedRelayDownloadPolicy(texts: TextCorpus, failures: string[]): void {
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

function checkSchedulerEligibilityGate(texts: TextCorpus, failures: string[]): void {
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

function checkStatusEvidence(texts: TextCorpus, failures: string[]): void {
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

function checkSupportRedaction(texts: TextCorpus, failures: string[]): void {
  const redaction = texts.get("packages/open-bitcoin-cli/src/operator/support/redaction.rs") ?? "";
  for (const needle of [
    "sanitize_relay_reason_field(&mut relay.activation)",
    "sanitize_relay_reason_field(&mut relay.download_eligibility)",
  ]) {
    requireContains(redaction, needle, `support-redaction: missing ${needle}`, failures);
  }

  const tests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  for (const needle of [
    "RelayActivationEvidence { enabled: true }",
    "RelayDownloadEligibilityCounters",
    "txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "wtxid=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    "permission_string",
    "dynamic_label",
  ]) {
    requireContains(tests, needle, `support-redaction: missing redaction test evidence ${needle}`, failures);
  }
}

function checkParitySurface(texts: TextCorpus, failures: string[]): void {
  const raw = texts.get("docs/parity/index.json") ?? "";
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`docs/parity/index.json is not valid JSON: ${String(error)}`);
    return;
  }

  const topSurfaces = Array.isArray(parsed.surfaces) ? (parsed.surfaces as ParitySurface[]) : [];
  const topMatches = topSurfaces.filter((surface) => surface.name === SURFACE_ID);
  if (topMatches.length !== 1) {
    failures.push(`expected exactly one top-level Phase 107 surface ${SURFACE_ID}`);
  } else if (topMatches[0]?.status !== "done") {
    failures.push(`${SURFACE_ID}: expected top-level status done`);
  }

  const checklistSurfaces = Array.isArray(parsed.checklist?.surfaces)
    ? (parsed.checklist.surfaces as ParitySurface[])
    : [];
  const matches = checklistSurfaces.filter((surface) => surface.id === SURFACE_ID);
  if (matches.length !== 1) {
    failures.push(`expected exactly one parity checklist surface ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`${SURFACE_ID}: expected checklist status done`);
  }
  if (!sameMembers(asStringArray(surface.requirements), [...REQUIRED_REQUIREMENTS])) {
    failures.push(`${SURFACE_ID}: expected requirements ${REQUIRED_REQUIREMENTS.join(", ")}`);
  }
  const evidence = asStringArray(surface.evidence);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    if (!evidence.includes(root)) {
      failures.push(`${SURFACE_ID}: missing evidence root ${root}`);
    }
  }

  const anchors = [
    ...asStringArray(surface.upstream?.sources),
    ...asStringArray(surface.upstream?.tests),
  ];
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!anchors.includes(anchor)) {
      failures.push(`${SURFACE_ID}: missing Knots anchor ${anchor}`);
    }
  }

  const gapText = [
    ...asStringArray(surface.known_gaps),
    ...asStringArray(surface.suspected_unknowns),
  ]
    .join("\n")
    .toLowerCase();
  for (const term of REQUIRED_GAP_TERMS) {
    if (!gapText.includes(term.toLowerCase())) {
      failures.push(`${SURFACE_ID}: missing explicit deferred/no-claim term ${term}`);
    }
  }
}

function checkRequiredDocsAndCommands(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  requireContains(corpus, SURFACE_ID, `docs: missing Phase 107 surface id ${SURFACE_ID}`, failures);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(corpus, requirement, `docs: missing Phase 107 requirement ${requirement}`, failures);
  }

  const docsCorpus = [
    texts.get("README.md") ?? "",
    texts.get("docs/architecture/config-precedence.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/catalog/mempool-policy.md") ?? "",
    texts.get("docs/parity/catalog/rpc-cli-config.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
  ].join("\n");
  const normalizedDocsCorpus = normalizeWhitespace(docsCorpus);
  for (const needle of REQUIRED_DOC_NEEDLES) {
    requireContains(
      normalizedDocsCorpus,
      normalizeWhitespace(needle),
      `docs: missing aggregate sanitized Phase 107 wording ${needle}`,
      failures,
    );
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  if (!normalizeWhitespace(runtimeGuide).includes("aggregate, sanitized, and fixed-label only")) {
    failures.push("runtime guide missing aggregate sanitized Phase 107 evidence wording");
  }
  for (const command of REQUIRED_RUNTIME_GUIDE_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 107 runtime guide command ${command}`);
    }
  }
}

function checkVerifierOrder(verifyText: string, failures: string[]): void {
  const visibleMarker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const visibleStart = verifyText.indexOf(visibleMarker);
  const visibleBodyStart = visibleStart + visibleMarker.length;
  const visibleEnd = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", visibleBodyStart);
  const visibleText =
    visibleStart === -1 || visibleEnd === -1 ? "" : verifyText.slice(visibleBodyStart, visibleEnd);
  if (
    !orderedIndexes(visibleText, [
      PHASE106_TEST_COMMAND,
      PHASE106_CHECKER_COMMAND,
      PHASE107_TEST_COMMAND,
      PHASE107_CHECKER_COMMAND,
    ])
  ) {
    failures.push("verifier-scope: Phase 107 visible order must immediately follow Phase 106");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 106 parity UAT release boundary checker"',
      'run_step "check Phase 106 parity UAT release boundary"',
      'run_step "test Phase 107 runtime relay activation/download eligibility checker"',
      'run_step "check Phase 107 runtime relay activation/download eligibility"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 107 executable order must follow Phase 106 and precede pure-core checks");
  }
}

function checkForbiddenDefaultVerifierGates(verifyText: string, failures: string[]): void {
  const runStepLines = verifyText
    .split("\n")
    .map((line) => line.trim().toLowerCase())
    .filter((line) => line.startsWith("run_step "));
  for (const line of runStepLines) {
    for (const forbidden of FORBIDDEN_DEFAULT_VERIFIER_GATES) {
      if (line.includes(forbidden)) {
        failures.push(`verifier-scope: default verifier must not run ${forbidden}`);
      }
    }
  }
}

function checkSensitivePublicEvidence(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!isPublicEvidenceFile(file)) {
      continue;
    }
    for (const [lineIndex, line] of text.split("\n").entries()) {
      for (const patternValue of SENSITIVE_PUBLIC_EVIDENCE_PATTERNS) {
        if (patternValue.test(line)) {
          failures.push(`${file}:${lineIndex + 1}: sensitive public evidence must stay aggregate and sanitized`);
        }
      }
    }
  }
}

function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && !file.startsWith(".planning/") && file !== "README.md") {
      continue;
    }
    for (const paragraph of markdownParagraphs(text)) {
      const lowerText = paragraph.text.toLowerCase();
      for (const forbidden of FORBIDDEN_CLAIMS) {
        if (!lowerText.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerText) || !hasPositiveClaim(lowerText)) {
          continue;
        }
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 107 claim: ${forbidden}`);
      }
    }
  }
}

function requireGateBeforeMutation(
  section: string,
  gateNeedle: string,
  mutationNeedles: readonly string[],
  label: string,
  failures: string[],
): void {
  if (section.length === 0) {
    failures.push(`scheduler-gate: missing ${label} section`);
    return;
  }
  const gateIndex = section.indexOf(gateNeedle);
  if (gateIndex === -1) {
    failures.push(`scheduler-gate: ${label} missing relay eligibility gate ${gateNeedle}`);
    return;
  }

  for (const mutationNeedle of mutationNeedles) {
    const mutationIndex = section.indexOf(mutationNeedle);
    if (mutationIndex !== -1 && mutationIndex < gateIndex) {
      failures.push(`scheduler-gate: ${label} eligibility gate must appear before insert_in_flight and insert_candidate`);
      return;
    }
  }
}

function sectionBetween(text: string, startNeedle: string, endNeedle: string): string {
  const start = text.indexOf(startNeedle);
  if (start === -1) {
    return "";
  }
  const end = text.indexOf(endNeedle, start + startNeedle.length);
  return end === -1 ? text.slice(start) : text.slice(start, end);
}

function readText(repoRoot: string, filePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, filePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing target file ${filePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function requireContains(text: string, needle: string, message: string, failures: string[]): void {
  if (!text.includes(needle)) {
    failures.push(message);
  }
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function sameMembers(actual: string[], expected: string[]): boolean {
  return actual.length === expected.length && expected.every((item) => actual.includes(item));
}

function orderedIndexes(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) {
      return false;
    }
    cursor = index;
  }
  return true;
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let startLine = 1;
  let current: string[] = [];
  for (const [index, line] of text.split("\n").entries()) {
    const trimmed = line.trim();
    if (trimmed === "" || (trimmed.startsWith("- ") && current.length > 0)) {
      if (current.length > 0) {
        paragraphs.push({ startLine, text: current.join(" ") });
        current = [];
      }
      startLine = index + 2;
      if (trimmed === "") {
        continue;
      }
    }
    if (current.length === 0) {
      startLine = index + 1;
    }
    current.push(line);
  }
  if (current.length > 0) {
    paragraphs.push({ startLine, text: current.join(" ") });
  }
  return paragraphs;
}

function hasNoClaimMarker(line: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => line.includes(marker));
}

function hasPositiveClaim(line: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((patternValue) => patternValue.test(line));
}

function isPublicEvidenceFile(file: string): boolean {
  return (
    file === "README.md" ||
    file.startsWith("docs/architecture/") ||
    file === "docs/operator/runtime-guide.md" ||
    file.startsWith("docs/parity/catalog/") ||
    file === "docs/parity/checklist.md" ||
    file === "packages/open-bitcoin-node/src/status/relay_evidence.rs"
  );
}

if (import.meta.main) {
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility();
  if (failures.length > 0) {
    console.error("Phase 107 runtime relay activation/download eligibility check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 107 runtime relay activation/download eligibility validated.");
}

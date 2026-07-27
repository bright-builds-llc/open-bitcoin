#!/usr/bin/env bun

import { existsSync } from "node:fs";
import path from "node:path";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/tests.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-rpc/src/method/node.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase116-operator-block-relay-evidence.ts",
  "scripts/check-phase116-operator-block-relay-evidence.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_REQUIREMENTS = ["OBS-01", "OBS-02", "OBS-03", "OBS-04", "OBS-05"] as const;
const REQUIRED_SYMBOLS = [
  "BlockRelayEvidenceStatus",
  "block_relay_evidence_status",
  "MetricKind::BlockServedCount",
  "BLOCK_RELAY_LOG_SOURCE",
  "block_relay_log_record",
  "openbitcoinnetworkstatus",
  "redact_block_relay_evidence",
] as const;
const REQUIRED_FIXED_COUNTERS = [
  "block_served_count",
  "block_serving_suppressed_count",
  "compact_announced_count",
  "compact_reconstructed_count",
  "compact_missing_tx_requested_count",
  "compact_fallback_count",
  "compact_malformed_count",
  "compact_timeout_count",
  "compact_cleanup_count",
  "block_serving_eligible",
  "block_serving_suppressed",
  "compact_announced",
  "compact_reconstruction_failed",
  "compact_download_timeout",
  "compact_download_peer_disconnect",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "open_bitcoin_network_status_includes_block_relay_projection",
  "operator_status_block_relay_maps_shared_contract_and_human_lines",
  "operator_status_block_relay_fallback_uses_default_unavailable_contract",
  "dashboard_model_block_relay_rows_surface_shared_status_contract",
  "dashboard_model_block_relay_rows_preserve_unavailable_reason_without_sensitive_text",
  "block_relay_metric_kinds_are_low_cardinality_counters",
  "block_relay_metric_status_maps_to_each_fixed_metric_kind",
  "block_relay_log_record_uses_fixed_source_labels_and_counts",
  "block_relay_log_record_omits_sensitive_and_dynamic_material",
  "phase116_block_relay_evidence_projects_negotiation_serving_download_and_cleanup",
  "support_bundle_renders_block_relay_evidence_from_shared_projection",
  "support_bundle_redacts_sensitive_block_relay_reasons_in_json_and_markdown",
] as const;
const REQUIRED_REDACTION_NEEDLES = [
  "cmpctblock",
  "blocktxn",
  "getblocktxn",
  "0000000000000000000000000000000000000000000000000000000000000000",
  "127.0.0.1:",
  "198.51.100.116:8333",
  "peer_id=",
  "permission_string",
  "credential=phase116",
  "secret=phase116",
  "cookie=phase116",
  "dynamic_label",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
  "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts",
  "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
  "bash scripts/verify.sh",
] as const;
const REQUIRED_FILE_NEEDLES = [
  {
    file: "packages/open-bitcoin-node/src/status.rs",
    needle: "pub block_relay: BlockRelayEvidenceStatus",
  },
  {
    file: "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
    needle: "pub struct BlockRelayEvidenceStatus",
  },
  {
    file: "packages/open-bitcoin-node/src/metrics.rs",
    needle: "block_relay_metric_samples",
  },
  {
    file: "packages/open-bitcoin-node/src/logging.rs",
    needle: "block_relay_log_record",
  },
  {
    file: "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
    needle: "record_compact_download_actions",
  },
  {
    file: "packages/open-bitcoin-rpc/src/dispatch/node.rs",
    needle:
      "block_relay_evidence_status()\n            .map_err(network_authority_error_to_failure)?",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs",
    needle: "Block relay evidence",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs",
    needle: '"Block relay activation"',
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
    needle: "redact_block_relay_evidence",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs",
    needle: "## Block Relay Evidence",
  },
] as const;
const REQUIRED_BREADCRUMB_FILES_BY_GROUP = [
  {
    label: "cli-operator-onboarding-contracts",
    files: [
      "packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs",
      "packages/open-bitcoin-cli/src/operator/status/tests.rs",
    ],
  },
  {
    label: "cli-operator-support-bundles",
    files: [
      "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
      "packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests.rs",
    ],
  },
  {
    label: "cli-operator-dashboard-contracts",
    files: [
      "packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs",
      "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
    ],
  },
  {
    label: "node-observability-contracts",
    files: [
      "packages/open-bitcoin-node/src/metrics.rs",
      "packages/open-bitcoin-node/src/metrics/tests.rs",
      "packages/open-bitcoin-node/src/logging.rs",
      "packages/open-bitcoin-node/src/logging/tests.rs",
    ],
  },
  {
    label: "node-status-contract",
    files: [
      "packages/open-bitcoin-node/src/status.rs",
      "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
    ],
  },
  {
    label: "rpc-surface",
    files: [
      "packages/open-bitcoin-rpc/src/context/network.rs",
      "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
    ],
  },
] as const;
const FORBIDDEN_CLAIMS = [
  "public block serving by default",
  "production readiness",
  "release validator",
  "public-network ci",
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
  "remain",
  "remains",
  "only",
  "unavailable",
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
  /\bis ready\b/,
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type BreadcrumbGroup = { files?: unknown; label?: unknown };

export function checkPhase116OperatorBlockRelayEvidence(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE116_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkRequiredText(texts, failures);
  checkFileNeedles(texts, failures);
  checkBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenClaims(texts, failures);

  return failures;
}

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!corpus.includes(requirement)) {
      failures.push(`missing Phase 116 requirement ${requirement}`);
    }
  }
  for (const symbol of REQUIRED_SYMBOLS) {
    if (!corpus.includes(symbol)) {
      failures.push(`missing required Phase 116 symbol ${symbol}`);
    }
  }
  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    if (!corpus.includes(testName)) {
      failures.push(`missing required Phase 116 behavior test ${testName}`);
    }
  }

  const contractCorpus = [
    texts.get("packages/open-bitcoin-node/src/status/block_relay_evidence.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/metrics/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/logging/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/status/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
  ].join("\n");
  for (const counter of REQUIRED_FIXED_COUNTERS) {
    if (!contractCorpus.includes(counter)) {
      failures.push(`missing fixed block-relay evidence ${counter}`);
    }
  }

  const supportTests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  for (const needle of REQUIRED_REDACTION_NEEDLES) {
    if (!supportTests.includes(needle)) {
      failures.push(`missing Phase 116 support redaction coverage for ${needle}`);
    }
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 116 runtime guide command ${command}`);
    }
  }
}

function checkFileNeedles(texts: TextCorpus, failures: string[]): void {
  for (const { file, needle } of REQUIRED_FILE_NEEDLES) {
    const text = texts.get(file) ?? "";
    if (!text.includes(needle)) {
      failures.push(`${file}: missing shared Phase 116 contract needle ${needle}`);
    }
  }
}

function checkBreadcrumbs(raw: string, failures: string[]): void {
  let parsed: { groups?: unknown };
  try {
    parsed = JSON.parse(raw) as { groups?: unknown };
  } catch (error) {
    failures.push(`docs/parity/source-breadcrumbs.json is not valid JSON: ${String(error)}`);
    return;
  }

  const groups = Array.isArray(parsed.groups) ? (parsed.groups as BreadcrumbGroup[]) : [];
  for (const expected of REQUIRED_BREADCRUMB_FILES_BY_GROUP) {
    const maybeGroup = groups.find((group) => group.label === expected.label);
    if (!maybeGroup) {
      failures.push(`missing source breadcrumb group ${expected.label}`);
      continue;
    }
    const files = asStringArray(maybeGroup.files);
    for (const file of expected.files) {
      if (!files.includes(file)) {
        failures.push(`source breadcrumb group ${expected.label} missing file ${file}`);
      }
    }
  }
}

function checkVerifierOrder(verifyText: string, failures: string[]): void {
  const visibleMarker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const visibleStart = verifyText.indexOf(visibleMarker);
  const visibleBodyStart = visibleStart + visibleMarker.length;
  const visibleEnd = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", visibleBodyStart);
  const visibleText =
    visibleStart === -1 || visibleEnd === -1
      ? ""
      : verifyText.slice(visibleBodyStart, visibleEnd);

  if (
    !orderedIndexes(visibleText, [
      "bun test scripts/check-phase105-operator-relay-evidence.test.ts",
      "bun run scripts/check-phase105-operator-relay-evidence.ts",
      "bun test scripts/check-phase111-full-block-serving-request-path.test.ts",
      "bun run scripts/check-phase111-full-block-serving-request-path.ts",
      "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts",
      "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
    ])
  ) {
    failures.push("verifier-scope: Phase 116 visible order must follow Phase 111 and stay after Phase 105");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 111 full block-serving request path checker"',
      'run_step "check Phase 111 full block-serving request path"',
      'run_step "test Phase 116 operator block-relay evidence checker"',
      'run_step "check Phase 116 operator block-relay evidence"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 116 executable order must follow Phase 111 and precede pure-core checks");
  }
}

function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && !file.startsWith(".planning/") && file !== "README.md") {
      continue;
    }
    for (const paragraph of markdownParagraphs(text)) {
      const lowerText = paragraph.text.toLowerCase();
      if (
        !lowerText.includes("phase 116") &&
        !lowerText.includes("block-relay") &&
        !lowerText.includes("block relay")
      ) {
        continue;
      }
      for (const forbidden of FORBIDDEN_CLAIMS) {
        if (!lowerText.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerText) || !hasPositiveClaim(lowerText)) {
          continue;
        }
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 116 claim: ${forbidden}`);
      }
    }
  }
}

function readText(repoRoot: string, filePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, filePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing target file ${filePath}`);
    return "";
  }

  return readSourceCorpus(repoRoot, filePath);
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
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

function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let startLine = 1;
  let current: string[] = [];
  for (const [index, line] of text.split("\n").entries()) {
    if (line.trim() === "") {
      if (current.length > 0) {
        paragraphs.push({ startLine, text: current.join(" ") });
        current = [];
      }
      startLine = index + 2;
      continue;
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

if (import.meta.main) {
  const failures = checkPhase116OperatorBlockRelayEvidence();
  if (failures.length > 0) {
    console.error("Phase 116 operator block-relay evidence check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 116 operator block-relay evidence validated.");
}

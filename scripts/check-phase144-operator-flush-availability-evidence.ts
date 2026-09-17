#!/usr/bin/env bun

import { existsSync } from "node:fs";
import path from "node:path";
import { readSourceCorpus, readSourceRoot } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence/tests.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-rpc/src/method/node.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase144-operator-flush-availability-evidence.ts",
  "scripts/check-phase144-operator-flush-availability-evidence.test.ts",
  "scripts/verify.sh",
] as const;
const NEW_FIELD_FILES = [
  "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/metrics/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/logging/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs",
] as const;
const REQUIRED_REQUIREMENTS = ["CSOBS-01", "CSOBS-02"] as const;
const REQUIRED_SYMBOLS = [
  "ChainstateDurabilityEvidence",
  "chainstate_durability",
  "project_chainstate_durability",
  "record_have_bytes",
  "MetricKind::ChainstateDurabilityCacheSizeClass",
  "CHAINSTATE_DURABILITY_LOG_SOURCE",
  "chainstate_durability_log_record",
  "chainstate_durability_metric_samples",
  "openbitcoinnetworkstatus",
  "redact_chainstate_durability",
  "MAX_DASHBOARD_CHARTS",
] as const;
const REQUIRED_FIXED_COUNTERS = [
  "cache_size",
  "last_flush_reason",
  "available_count",
  "unavailable_count",
  "index_known_without_payload_count",
  "ok",
  "large",
  "critical",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "chainstate_durability_default_unavailable_uses_stable_reason",
  "execute_flush_retains_last_write_decision_not_later_none_classification",
  "have_bytes_accumulator_sets_unavailable_when_payload_absent",
  "open_bitcoin_network_status_includes_chainstate_durability_projection",
  "operator_status_chainstate_durability_maps_shared_contract_and_human_lines",
  "dashboard_model_chainstate_durability_rows_surface_shared_status_contract",
  "dashboard_max_charts_remains_eight",
  "chainstate_durability_metric_kinds_are_low_cardinality_gauges_and_counters",
  "chainstate_durability_log_record_omits_sensitive_and_dynamic_material",
  "support_bundle_renders_chainstate_durability_from_shared_projection",
  "support_bundle_redacts_sensitive_chainstate_durability_reasons_in_json_and_markdown",
] as const;
const REQUIRED_REDACTION_NEEDLES = [
  "peer_id=",
  "127.0.0.1:",
  "0000000000000000000000000000000000000000000000000000000000000000",
  "credential=phase144",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support",
  "bash scripts/verify.sh",
] as const;
const REQUIRED_FILE_NEEDLES = [
  {
    file: "packages/open-bitcoin-node/src/status.rs",
    needle: "pub chainstate_durability: FieldAvailability<ChainstateDurabilityEvidence>",
  },
  {
    file: "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
    needle: "pub struct ChainstateDurabilityEvidence",
  },
  {
    file: "packages/open-bitcoin-node/src/metrics.rs",
    needle: "chainstate_durability_metric_samples",
  },
  {
    file: "packages/open-bitcoin-node/src/logging.rs",
    needle: "chainstate_durability_log_record",
  },
  {
    file: "packages/open-bitcoin-rpc/src/dispatch/node.rs",
    needle: "snapshot.chainstate_durability().clone()",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs",
    needle: "Chainstate durability",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs",
    needle: '"Chainstate durability"',
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
    needle: "redact_chainstate_durability",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs",
    needle: "## Chainstate Durability",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
    needle: "MAX_DASHBOARD_CHARTS",
  },
] as const;
const REQUIRED_BREADCRUMB_FILES_BY_GROUP = [
  {
    label: "node-status-contract",
    files: ["packages/open-bitcoin-node/src/status/chainstate_durability.rs"],
  },
  {
    label: "node-chainstate-adapter",
    files: ["packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/durability.rs"],
  },
  {
    label: "node-network-chainstate-durability-evidence",
    files: ["packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs"],
  },
  {
    label: "node-observability-contracts",
    files: [
      "packages/open-bitcoin-node/src/metrics/chainstate_durability.rs",
      "packages/open-bitcoin-node/src/logging/chainstate_durability.rs",
    ],
  },
  {
    label: "cli-operator-onboarding-contracts",
    files: ["packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs"],
  },
  {
    label: "cli-operator-dashboard-contracts",
    files: ["packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs"],
  },
  {
    label: "cli-operator-support-bundles",
    files: ["packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs"],
  },
] as const;
const FORBIDDEN_NEW_FIELD_NEEDLES = [
  "pruned",
  "block_status_pruned",
  "getblock",
  "peer_id=",
  "127.0.0.1:",
  "txid:vout",
  "cmpctblock",
  "archive-node",
  "archive node",
  "prune mode",
  "public default",
  "public-default historical serving",
  "production ready",
  "production readiness",
  "assumeutxo",
  "compact filter",
] as const;
const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "must not include",
  "not ",
  "this is not",
  "without",
  "omits",
  "omit",
  "never",
  "forbid",
  "forbidden",
] as const;
const REQUEST_LOG_SYMBOLS = ["by_hash", "request_log", "HashMap<BlockHash"] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type BreadcrumbGroup = { files?: unknown; label?: unknown };

export const PHASE144_NEW_FIELD_FILES = NEW_FIELD_FILES;

export function checkPhase144OperatorFlushAvailabilityEvidence(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE144_REPO_ROOT ?? DEFAULT_REPO_ROOT,
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
  checkDocsContract(texts, failures);
  checkForbiddenNewFieldClaims(repoRoot, texts, failures);

  return failures;
}

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!corpus.includes(requirement)) {
      failures.push(`missing Phase 144 requirement ${requirement}`);
    }
  }
  for (const symbol of REQUIRED_SYMBOLS) {
    if (!corpus.includes(symbol)) {
      failures.push(`missing required Phase 144 symbol ${symbol}`);
    }
  }
  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    if (!corpus.includes(testName)) {
      failures.push(`missing required Phase 144 behavior test ${testName}`);
    }
  }

  const contractCorpus = [
    texts.get("packages/open-bitcoin-node/src/status/chainstate_durability.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
  ].join("\n");
  for (const counter of REQUIRED_FIXED_COUNTERS) {
    if (!contractCorpus.includes(counter)) {
      failures.push(`missing fixed chainstate durability evidence ${counter}`);
    }
  }

  const supportTests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  for (const needle of REQUIRED_REDACTION_NEEDLES) {
    if (!supportTests.includes(needle)) {
      failures.push(`missing Phase 144 support redaction coverage for ${needle}`);
    }
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 144 runtime guide command ${command}`);
    }
  }

}

function checkFileNeedles(texts: TextCorpus, failures: string[]): void {
  for (const { file, needle } of REQUIRED_FILE_NEEDLES) {
    const text = texts.get(file) ?? "";
    if (!text.includes(needle)) {
      failures.push(`${file}: missing shared Phase 144 contract needle ${needle}`);
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
    visibleStart === -1 || visibleEnd === -1 ? "" : verifyText.slice(visibleBodyStart, visibleEnd);

  if (
    !orderedIndexes(visibleText, [
      "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts",
      "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
      "bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts",
      "bun run scripts/check-phase144-operator-flush-availability-evidence.ts",
    ])
  ) {
    failures.push("verifier-scope: Phase 144 visible order must follow Phase 116");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 116 operator block-relay evidence checker"',
      'run_step "check Phase 116 operator block-relay evidence"',
      'run_step "test Phase 144 operator flush and availability evidence checker"',
      'run_step "check Phase 144 operator flush and availability evidence"',
    ])
  ) {
    failures.push("verifier-scope: Phase 144 executable order must follow Phase 116");
  }
}

function checkDocsContract(texts: TextCorpus, failures: string[]): void {
  const statusSnapshot = texts.get("docs/architecture/status-snapshot.md") ?? "";
  const observability = texts.get("docs/architecture/operator-observability.md") ?? "";
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  const docsCorpus = [statusSnapshot, observability, runtimeGuide].join("\n");

  for (const needle of [
    "chainstate_durability",
    "cache_size",
    "last_flush_reason",
    "have-bytes",
    "Unavailable:",
    "FieldAvailability",
  ]) {
    if (!docsCorpus.includes(needle)) {
      failures.push(`missing Phase 144 docs contract needle ${needle}`);
    }
  }

  if (!statusSnapshot.includes("fail-closed") && !statusSnapshot.includes("interrupted-without-B")) {
    failures.push("status-snapshot.md must say fail-closed / interrupted-without-B leaves coins tip unset");
  }
}

function checkForbiddenNewFieldClaims(
  repoRoot: string,
  texts: TextCorpus,
  failures: string[],
): void {
  for (const file of NEW_FIELD_FILES) {
    const text = readNewFieldText(repoRoot, file, texts);
    const lines = text.split("\n");
    for (const [index, line] of lines.entries()) {
      const lowerLine = line.toLowerCase();
      const maybePrevious = lines[index - 1]?.toLowerCase() ?? "";
      const nearby = `${maybePrevious} ${lowerLine}`;
      for (const forbidden of FORBIDDEN_NEW_FIELD_NEEDLES) {
        if (!lowerLine.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(nearby)) {
          continue;
        }
        failures.push(`${file}: forbidden new-field needle ${forbidden}`);
      }
    }
    for (const symbol of REQUEST_LOG_SYMBOLS) {
      if (text.includes(symbol)) {
        failures.push(`${file}: new-field corpus must not include request-log symbol ${symbol}`);
      }
    }
  }

  for (const file of [
    "docs/architecture/status-snapshot.md",
    "docs/architecture/operator-observability.md",
    "docs/operator/runtime-guide.md",
  ] as const) {
    for (const paragraph of markdownParagraphs(texts.get(file) ?? "")) {
      const lowerText = paragraph.text.toLowerCase();
      if (!lowerText.includes("chainstate_durability") && !lowerText.includes("have-bytes")) {
        continue;
      }
      for (const forbidden of FORBIDDEN_NEW_FIELD_NEEDLES) {
        if (!lowerText.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerText)) {
          continue;
        }
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 144 claim: ${forbidden}`);
      }
    }
  }
}

function readNewFieldText(repoRoot: string, filePath: string, texts: TextCorpus): string {
  const absolutePath = path.join(repoRoot, filePath);
  if (existsSync(absolutePath)) {
    return readSourceRoot(repoRoot, filePath);
  }
  return texts.get(filePath as TargetFile) ?? "";
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

if (import.meta.main) {
  const failures = checkPhase144OperatorFlushAvailabilityEvidence();
  if (failures.length > 0) {
    console.error("Phase 144 operator flush and availability evidence check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 144 operator flush and availability evidence validated.");
}

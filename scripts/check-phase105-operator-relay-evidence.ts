#!/usr/bin/env bun

import path from "node:path";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-operator-rpc-metrics-logs-support-evidence";
const PHASE104_TEST_COMMAND = "bun test scripts/check-phase104-relay-serving-fanout.test.ts";
const PHASE104_CHECKER_COMMAND = "bun run scripts/check-phase104-relay-serving-fanout.ts";
const PHASE105_TEST_COMMAND = "bun test scripts/check-phase105-operator-relay-evidence.test.ts";
const PHASE105_CHECKER_COMMAND = "bun run scripts/check-phase105-operator-relay-evidence.ts";
const REQUIRED_REQUIREMENTS = ["OBS-01", "OBS-02", "OBS-03", "OBS-04"] as const;
const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "README.md",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/tests.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase105-operator-relay-evidence.ts",
  "scripts/check-phase105-operator-relay-evidence.test.ts",
  "scripts/verify.sh",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-01-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-02-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-03-SUMMARY.md",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "README.md",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase105-operator-relay-evidence.ts",
  "scripts/check-phase105-operator-relay-evidence.test.ts",
  "scripts/verify.sh",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-01-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-02-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-03-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_ANCHORS = ["packages/bitcoin-knots/src/rpc/net.cpp", "packages/bitcoin-knots/src/rpc/mempool.cpp", "packages/bitcoin-knots/src/rpc/rawtransaction.cpp", "packages/bitcoin-knots/src/net_processing.cpp", "packages/bitcoin-knots/src/txmempool.cpp", "packages/bitcoin-knots/test/functional/rpc_net.py", "packages/bitcoin-knots/test/functional/rpc_mempool_info.py", "packages/bitcoin-knots/test/functional/rpc_rawtransaction.py", "packages/bitcoin-knots/test/functional/mempool_accept.py", "packages/bitcoin-knots/test/functional/p2p_tx_download.py"] as const;
const REQUIRED_FIXED_COUNTERS = [
  "accepted_count",
  "rejected_count",
  "orphaned_count",
  "requested_count",
  "served_count",
  "announced_count",
  "suppressed_count",
  "evicted_count",
  "expired_count",
  "rebroadcast_deferred_count",
] as const;
const REQUIRED_SYMBOLS = [
  "RelayEvidenceStatus",
  "RelayEvidenceCounters",
  "RelayEvidenceField",
  "RelayEvidenceCapability",
  "MetricKind::RelayAcceptedCount",
  "MetricKind::RelayRebroadcastDeferredCount",
  "RELAY_MEMPOOL_LOG_SOURCE",
  "relay_mempool_log_record",
  "openbitcoinnetworkstatus",
  "redacted_relay_mempool_evidence",
] as const;
const REQUIRED_BEHAVIOR_TESTS = [
  "operator_status_renders_relay_evidence_from_open_bitcoin_network_status",
  "dashboard_sections_surface_relay_evidence_rows",
  "dashboard_charts_render_retained_relay_metric_samples_without_dynamic_labels",
  "relay_metric_kinds_are_low_cardinality_counters",
  "relay_status_maps_to_each_fixed_relay_metric_kind",
  "relay_mempool_log_record_uses_fixed_outcome_counts",
  "relay_mempool_log_record_omits_sensitive_and_dynamic_material",
  "support_bundle_renders_relay_and_mempool_evidence_from_shared_projection",
  "support_bundle_redacts_sensitive_relay_reasons_in_json_and_markdown",
] as const;
const REQUIRED_REDACTION_NEEDLES = [
  "raw tx hex",
  "020000000001",
  "txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "wtxid=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  "127.0.0.1:",
  "198.51.100.105:8333",
  "peer_id=",
  "permission_string",
  "credential=phase105",
  "secret=phase105",
  "cookie=phase105",
  "dynamic_label",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-relay-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-relay-support",
] as const;
const REQUIRED_FILE_NEEDLES = [
  {
    file: "packages/open-bitcoin-node/src/status/relay_evidence.rs",
    needle: "pub struct RelayEvidenceStatus",
  },
  {
    file: "packages/open-bitcoin-node/src/status.rs",
    needle: "pub relay: relay_evidence::RelayEvidenceStatus",
  },
  {
    file: "packages/open-bitcoin-node/src/metrics.rs",
    needle: "relay_metric_samples",
  },
  {
    file: "packages/open-bitcoin-node/src/logging.rs",
    needle: "relay_mempool_log_record",
  },
  {
    file: "packages/open-bitcoin-rpc/src/context/network.rs",
    needle: "relay_evidence_status",
  },
  {
    file: "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
    needle: "open_bitcoin_network_status_returns_available_inbound_evidence",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
    needle: "mempool.relay",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
    needle: "snapshot.mempool.relay",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
    needle: "push_relay_mempool_evidence",
  },
  {
    file: "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
    needle: "redact_relay_mempool_evidence",
  },
] as const;
const REQUIRED_BREADCRUMB_FILES_BY_GROUP = [
  {
    label: "cli-operator-onboarding-contracts",
    files: [
      "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
      "packages/open-bitcoin-cli/src/operator/status/tests.rs",
    ],
  },
  {
    label: "cli-operator-support-bundles",
    files: [
      "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
      "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests.rs",
    ],
  },
  {
    label: "cli-operator-dashboard-contracts",
    files: [
      "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
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
    files: ["packages/open-bitcoin-node/src/status.rs", "packages/open-bitcoin-node/src/status/relay_evidence.rs"],
  },
  {
    label: "rpc-surface",
    files: [
      "packages/open-bitcoin-rpc/src/context/network.rs",
      "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
    ],
  },
] as const;
const REQUIRED_GAP_TERMS = ["public propagation", "compact block relay", "package relay", "bloom/filter serving", "public relay defaults", "public relay by default", "public-network relay CI", "production service operation", "production-service proof", "production full-node readiness", "production full-node readiness proof", "production-funds wallet use", "production-funds wallet safety proof", "release validator"] as const;
const FORBIDDEN_CLAIMS = [
  "public propagation",
  "compact block relay",
  "compact-block relay",
  "package relay",
  "bloom/filter serving",
  "public relay defaults",
  "public relay by default",
  "public-network relay ci",
  "production service operation",
  "production-service proof",
  "production full-node readiness",
  "production-readiness proof",
  "production full-node readiness proof",
  "production-funds wallet use",
  "production-funds wallet safety proof",
  "release validator",
] as const;
const NO_CLAIM_MARKERS = ["does not", "do not", "must not", "not ", "without", "outside", "out of scope", "deferred", "future", "later", "remain", "remains", "no claim", "not claim", "not supported", "only", "intentionally different", "unavailable"] as const;
const POSITIVE_CLAIM_PATTERNS = [/\bsupports?\b/, /\bprovides?\b/, /\benables?\b/, /\badds?\b/, /\bimplements?\b/, /\bships?\b/, /\bproves?\b/, /\bis supported\b/, /\bis enabled\b/, /\bis available\b/, /\bis complete\b/, /\bis ready\b/] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type ParitySurface = {
  evidence?: unknown;
  id?: unknown;
  known_gaps?: unknown;
  requirements?: unknown;
  status?: unknown;
  suspected_unknowns?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown } };
type BreadcrumbGroup = { files?: unknown; label?: unknown };

export function checkPhase105OperatorRelayEvidence(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE105_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkParitySurface(texts, failures);
  checkRequiredText(texts, failures);
  checkFileNeedles(texts, failures);
  checkBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenClaims(texts, failures);

  return failures;
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

  const surfaces = Array.isArray(parsed.checklist?.surfaces)
    ? (parsed.checklist.surfaces as ParitySurface[])
    : [];
  const matches = surfaces.filter((surface) => surface.id === SURFACE_ID);
  if (matches.length !== 1) {
    failures.push(`expected exactly one parity checklist surface ${SURFACE_ID}`);
    return;
  }
  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`${SURFACE_ID}: expected status done`);
  }

  const requirements = asStringArray(surface.requirements);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!requirements.includes(requirement)) {
      failures.push(`${SURFACE_ID}: missing requirement ${requirement}`);
    }
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

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  for (const requirement of REQUIRED_REQUIREMENTS) {
    if (!corpus.includes(requirement)) {
      failures.push(`missing Phase 105 requirement ${requirement}`);
    }
  }
  for (const symbol of REQUIRED_SYMBOLS) {
    if (!corpus.includes(symbol)) {
      failures.push(`missing required Phase 105 symbol ${symbol}`);
    }
  }
  for (const testName of REQUIRED_BEHAVIOR_TESTS) {
    if (!corpus.includes(testName)) {
      failures.push(`missing required Phase 105 behavior test ${testName}`);
    }
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!corpus.includes(anchor)) {
      failures.push(`missing Phase 105 Knots anchor ${anchor}`);
    }
  }

  const contractCorpus = [
    texts.get("packages/open-bitcoin-node/src/status/relay_evidence.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/metrics/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/logging/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/status/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "",
  ].join("\n");
  for (const counter of REQUIRED_FIXED_COUNTERS) {
    if (!contractCorpus.includes(counter)) {
      failures.push(`missing fixed relay counter evidence ${counter}`);
    }
  }

  const supportTests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  for (const needle of REQUIRED_REDACTION_NEEDLES) {
    if (!supportTests.includes(needle)) {
      failures.push(`missing Phase 105 support redaction coverage for ${needle}`);
    }
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 105 runtime guide command ${command}`);
    }
  }
}

function checkFileNeedles(texts: TextCorpus, failures: string[]): void {
  for (const { file, needle } of REQUIRED_FILE_NEEDLES) {
    const text = texts.get(file) ?? "";
    if (!text.includes(needle)) {
      failures.push(`${file}: missing shared Phase 105 contract needle ${needle}`);
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
      PHASE104_TEST_COMMAND,
      PHASE104_CHECKER_COMMAND,
      PHASE105_TEST_COMMAND,
      PHASE105_CHECKER_COMMAND,
    ])
  ) {
    failures.push("verifier-scope: Phase 105 visible order must follow Phase 104");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 104 relay serving/fanout checker"',
      'run_step "check Phase 104 relay serving/fanout"',
      'run_step "test Phase 105 operator relay evidence checker"',
      'run_step "check Phase 105 operator relay evidence"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 105 executable order must follow Phase 104 and precede pure-core checks");
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
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 105 claim: ${forbidden}`);
      }
    }
  }
}

function readText(repoRoot: string, filePath: TargetFile, failures: string[]): string {
  try {
    return readSourceCorpus(repoRoot, filePath);
  } catch {
    failures.push(`missing target file ${filePath}`);
    return "";
  }
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
  const failures = checkPhase105OperatorRelayEvidence();
  if (failures.length > 0) {
    console.error("Phase 105 operator relay evidence check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 105 operator relay evidence validated.");
}

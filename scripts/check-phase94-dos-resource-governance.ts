#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-dos-resource-governance";
const PHASE93_TEST_COMMAND = "bun test scripts/check-phase93-peer-policy.test.ts";
const PHASE93_CHECKER_COMMAND = "bun run scripts/check-phase93-peer-policy.ts";
const PHASE94_TEST_COMMAND =
  "bun test scripts/check-phase94-dos-resource-governance.test.ts";
const PHASE94_CHECKER_COMMAND =
  "bun run scripts/check-phase94-dos-resource-governance.ts";
const PHASE94_REQUIREMENTS = ["DOS-01", "DOS-02", "DOS-03", "DOS-04", "DOS-05"] as const;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = { name?: unknown; status?: unknown };

const TARGET_FILES = [
  "packages/open-bitcoin-network/src/resource.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/resource_governance.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-node/src/status/inbound.rs",
  "packages/open-bitcoin-node/src/network/inbound.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/writer.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "scripts/verify.sh",
] as const;

const REQUIRED_EVIDENCE = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
] as const;

const REQUIRED_LABELS = [
  "wrong_network_magic malformed_header payload_oversized invalid_checksum unsupported_command malformed_payload trailing_payload",
  "slow_handshake idle_peer connection_churn_limited repeated_failure_limited reconnect_suppressed_banned reconnect_suppressed_discouraged",
  "resource_pressure_active read_queue_pressure write_queue_pressure request_cap_reached payload_rejected timeout_disconnect churn_rejected reconnect_suppressed",
].flatMap((group) => group.split(" "));

const REQUIRED_RUNTIME_WIRING = [
  "decide_queue",
  "tokio::time::timeout",
  "decide_churn",
  "decide_repeated_failure",
  "decide_reconnect",
  "record_inbound_resource_event",
  "reconnect_suppression_input_for_remote_addr",
] as const;

const REQUIRED_STRUCTURED_LOG_EMISSION = [
  "append_structured_log_record",
  "maybe_resource_governance_log_dir",
  "LogRetentionPolicy",
  "record_inbound_resource_event_at",
  "record_inbound_resource_event_appends_inbound_resource_governance_log_record",
  "open-bitcoin-runtime-",
  "serde_json::from_str",
] as const;

const REQUIRED_STRUCTURED_LOG_PROJECTION = [
  "INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE",
  "inbound_resource_governance_log_record",
  "inbound_resource_governance",
  "outcome=",
  "reason=",
  "label=",
  "source=",
  "message=",
  "next_action=",
] as const;

const REQUIRED_METRICS = [
  "inbound_resource_pressure_active_count inbound_read_queue_pressure_count inbound_write_queue_pressure_count inbound_request_cap_reached_count",
  "inbound_payload_rejected_count inbound_timeout_disconnect_count inbound_churn_rejected_count inbound_reconnect_suppressed_count",
].flatMap((group) => group.split(" "));

const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/protocol.h packages/bitcoin-knots/src/net.cpp packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/banman.cpp packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/test/functional/p2p_invalid_messages.py packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py packages/bitcoin-knots/test/functional/p2p_timeouts.py",
  "packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py packages/bitcoin-knots/test/functional/p2p_getdata.py",
].flatMap((group) => group.split(" "));

const FORBIDDEN_VERIFY_STRINGS = [
  "openbitcoinlisten=0.0.0.0",
  "public-network",
  "mainnet listener",
  "systemctl",
  "launchctl",
  "service-manager",
] as const;

const FORBIDDEN_POSITIVE_CLAIMS = [
  "transaction relay support",
  "compact block relay support",
  "mempool propagation support",
  "broad address relay support",
  "public inbound default",
  "public-network ci",
  "production service operation",
  "production full-node readiness",
  "bip37 support",
  "compact filter support",
] as const;

const ALLOWED_SCOPE_TERMS = [
  "does not",
  "do not",
  "not ",
  "no ",
  "without",
  "outside",
  "remain outside",
  "remains outside",
  "deferred",
  "future",
  "not claim",
  "not claiming",
  "no-claim",
  "non-claim",
] as const;

const POSITIVE_CLAIM_MARKERS = [
  " provides ",
  " supports ",
  " adds ",
  " enables ",
  " includes ",
  " ships ",
  " support is enabled",
  " support is supported",
  " is supported",
  " is enabled",
  " is complete",
  " is achieved",
  " readiness is achieved",
] as const;

const RAW_RESOURCE_EVIDENCE_STRINGS = [
  "peer_id",
  "raw_endpoint",
  "payload_bytes",
  "permission_string",
  "credential",
  "secret",
] as const;

const RAW_RESOURCE_EVIDENCE_FILES = [
  "packages/open-bitcoin-node/src/status/inbound.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/writer.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
] as const;

const RPC_STRUCTURED_LOG_FILES = [
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/resource_governance.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];

export type CheckPhase94Options = { rootDir?: string };

export function checkPhase94DosResourceGovernance(
  options: CheckPhase94Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyPhase94Labels(texts, failures);
  verifyRuntimeWiring(texts, failures);
  verifyStructuredLogEmission(texts, failures);
  verifyStructuredLogProjection(texts, failures);
  verifyMetricNames(texts, failures);
  verifyPhase94Docs(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifyRawResourceEvidenceBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required Phase 94 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizedLower(text).includes(normalizedLower(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} missing required value: ${required}`);
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 94 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 94 parity index surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`Phase 94 parity index missing done surface: ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("Phase 94 checklist surfaces must be an array");
    return;
  }

  const checklistSurface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (checklistSurface?.status !== "done") {
    failures.push(`Phase 94 checklist missing done ${SURFACE_ID}`);
  }
  const actual = JSON.stringify(checklistSurface?.requirements);
  const expected = JSON.stringify(PHASE94_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`Phase 94 requirements mismatch: expected ${expected}, got ${actual}`);
  }
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(checklistSurface?.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
}

function verifyPhase94Labels(texts: Map<TargetFile, string>, failures: string[]): void {
  const corpus = [
    texts.get("packages/open-bitcoin-network/src/resource.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/context/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/status/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/status/render/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/render/inbound.rs") ?? "",
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
  ].join("\n");

  for (const label of REQUIRED_LABELS) {
    requireNormalizedContains(corpus, label, "Phase 94 label coverage", failures);
  }
}

function verifyRuntimeWiring(texts: Map<TargetFile, string>, failures: string[]): void {
  const runtimeText = [
    texts.get("packages/open-bitcoin-network/src/resource.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "",
  ].join("\n");

  for (const required of REQUIRED_RUNTIME_WIRING) {
    requireContains(runtimeText, required, "Phase 94 runtime wiring", failures);
  }
}

function verifyStructuredLogEmission(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const rpcContextText = RPC_STRUCTURED_LOG_FILES.map((file) => texts.get(file) ?? "").join("\n");
  for (const required of REQUIRED_STRUCTURED_LOG_EMISSION) {
    requireContains(
      rpcContextText,
      required,
      "ManagedRpcContext structured log emission",
      failures,
    );
  }

  const appendPath = texts.get("packages/open-bitcoin-rpc/src/context/resource_governance.rs") ?? "";
  requireContains(
    appendPath,
    "append_structured_log_record(log_dir, &record, self.resource_governance_log_retention)?",
    "ManagedRpcContext structured log emission append path",
    failures,
  );
}

function verifyStructuredLogProjection(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const logProjectionText = [
    texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/logging/writer.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/context/resource_governance.rs") ?? "",
    texts.get("packages/open-bitcoin-rpc/src/context/tests.rs") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
  ].join("\n");

  for (const required of REQUIRED_STRUCTURED_LOG_PROJECTION) {
    requireContains(logProjectionText, required, "Phase 94 structured log projection", failures);
  }
}

function verifyMetricNames(texts: Map<TargetFile, string>, failures: string[]): void {
  const metricText = [
    texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "",
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
  ].join("\n");

  for (const metric of REQUIRED_METRICS) {
    requireContains(metricText, metric, "Phase 94 metric coverage", failures);
  }
}

function verifyPhase94Docs(texts: Map<TargetFile, string>, failures: string[]): void {
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  requireContains(runtimeGuide, "Phase 94 Resource Governance Review", "Phase 94 docs", failures);
  requireContains(
    runtimeGuide,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
    "Phase 94 UAT command",
    failures,
  );
  requireContains(
    runtimeGuide,
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
    "Phase 94 UAT command",
    failures,
  );
  requireContains(
    runtimeGuide,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "Phase 94 UAT command",
    failures,
  );
  requireContains(
    runtimeGuide,
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "Phase 94 UAT command",
    failures,
  );

  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "Phase 94 parity catalog", failures);
  requireContains(checklistText, SURFACE_ID, "Phase 94 parity checklist", failures);
  for (const requirement of PHASE94_REQUIREMENTS) {
    requireContains(p2pText, requirement, "Phase 94 parity catalog", failures);
    requireContains(checklistText, requirement, "Phase 94 parity checklist", failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireContains(p2pText, anchor, "Phase 94 parity catalog", failures);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("Phase 94 verifier-order missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [
        PHASE93_TEST_COMMAND,
        PHASE93_CHECKER_COMMAND,
        PHASE94_TEST_COMMAND,
        PHASE94_CHECKER_COMMAND,
      ],
      "Phase 94 verifier-order printed commands must follow Phase 93",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "Phase 94 DoS/resource governance checker tests" ${PHASE94_TEST_COMMAND}`,
    "Phase 94 verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "Phase 94 DoS/resource governance checker" ${PHASE94_CHECKER_COMMAND}`,
    "Phase 94 verifier-order",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE93_TEST_COMMAND,
      PHASE93_CHECKER_COMMAND,
      PHASE94_TEST_COMMAND,
      PHASE94_CHECKER_COMMAND,
      "bash scripts/check-pure-core-deps.sh",
    ],
    "Phase 94 verifier-order executed commands must follow Phase 93 and precede pure-core checks",
    failures,
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (containsForbiddenVerifyFragment(executableText, forbidden)) {
      failures.push(`Phase 94 default verifier boundary contains forbidden text: ${forbidden}`);
    }
  }
}

function verifyOrderedCommands(
  text: string,
  commands: readonly string[],
  failure: string,
  failures: string[],
): void {
  let previousIndex = -1;
  for (const command of commands) {
    const currentIndex = text.indexOf(command);
    if (currentIndex === -1 || currentIndex <= previousIndex) {
      failures.push(failure);
      return;
    }
    previousIndex = currentIndex;
  }
}

function containsForbiddenVerifyFragment(text: string, fragment: string): boolean {
  if (/^[a-z-]+ $/.test(fragment)) {
    const command = escapeRegExp(fragment.trim());
    return new RegExp(`(^|[\\s;&|()])${command}(?=\\s)`).test(text);
  }

  return text.includes(fragment);
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  const lower = normalizedLower(unit);
  for (const claim of FORBIDDEN_POSITIVE_CLAIMS) {
    if (lower.includes(claim) && isPositiveClaim(lower)) {
      failures.push(`Phase 94 no-claim boundary forbidden claim in ${file}: ${unit}`);
    }
  }
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term));
}

function isPositiveClaim(lowerUnit: string): boolean {
  return POSITIVE_CLAIM_MARKERS.some((marker) => lowerUnit.includes(marker));
}

function verifyRawResourceEvidenceBoundary(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  for (const file of RAW_RESOURCE_EVIDENCE_FILES) {
    const text = texts.get(file) ?? "";
    for (const rawDetail of RAW_RESOURCE_EVIDENCE_STRINGS) {
      if (text.includes(rawDetail)) {
        failures.push(`Phase 94 raw resource evidence boundary raw detail in ${file}: ${rawDetail}`);
      }
    }
  }
}

function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }

    const tableRows = lines.filter((line) => line.startsWith("|") && !/^\|\s*-/.test(line));
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      units.push(...sentenceUnits(lines.filter((line) => !line.startsWith("|")).join(" ")));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }
  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  return normalized.length === 0 ? [] : normalized.split(/(?<=[.!?])\s+/);
}

if (import.meta.main) {
  const failures = checkPhase94DosResourceGovernance();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 94 DoS and resource-governance evidence");
  }
}

#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-peer-policy-runtime-bridge";
const PHASE95_TEST_COMMAND =
  "bun test scripts/check-phase95-network-participation-release-boundary.test.ts";
const PHASE95_CHECKER_COMMAND =
  "bun run scripts/check-phase95-network-participation-release-boundary.ts";
const PHASE96_TEST_COMMAND =
  "bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts";
const PHASE96_CHECKER_COMMAND =
  "bun run scripts/check-phase96-peer-policy-runtime-bridge.ts";
const TRACEABILITY_REQUIREMENTS = ["EVICT-03", "EVICT-04", "DOS-03"] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/banman.h",
  "packages/bitcoin-knots/src/banman.cpp",
  "packages/bitcoin-knots/src/net_permissions.cpp",
] as const;
const TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer_policy.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/peer_policy.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;
const RAW_PEER_POLICY_MARKERS = [
  "peer_id=",
  "raw_endpoint",
  "permission_string",
  "payload_bytes",
  "credential",
  "secret",
  "cookie=",
] as const;
const RAW_OUTPUT_SCAN_FILES = [
  "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
] as const;
const FORBIDDEN_PHASE96_VERIFY_GATES = [
  "public-network",
  "dnsseed",
  "seednode",
  "service-manager",
  "systemd",
  "launchd",
  "multi-day",
  "soak",
  "wildcard listener",
] as const;
const FORBIDDEN_CLAIM_PHRASES = [
  "public banlist",
  "transaction relay",
  "mempool propagation",
  "compact block relay",
  "public inbound default",
  "production service",
  "production readiness",
  "production full-node readiness",
] as const;
const POSITIVE_CLAIM_MARKERS = [
  " provides ",
  " supports ",
  " adds ",
  " enables ",
  " includes ",
  " ships ",
  " is enabled",
  " is supported",
  " is ready",
] as const;
const ALLOWED_NEGATION_MARKERS = [
  "not ",
  "no ",
  "without",
  "outside",
  "deferred",
  "future",
  "not a ",
  "not claim",
  "does not claim",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckPhase96Options = { rootDir?: string };
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = { name?: unknown; status?: unknown };

export function checkPhase96PeerPolicyRuntimeBridge(
  options: CheckPhase96Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyPureRuntimeState(texts, failures);
  verifyManagedProjection(texts, failures);
  verifyScopedReconnectSuppression(texts, failures);
  verifyStructuredLogEvidence(texts, failures);
  verifyRawPeerPolicyBoundary(texts, failures);
  verifyDocsAndParity(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P96 missing required corpus file: ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyPureRuntimeState(texts: Map<TargetFile, string>, failures: string[]): void {
  const peerPolicy = texts.get("packages/open-bitcoin-network/src/peer_policy.rs") ?? "";
  for (const needle of [
    "pub struct PeerPolicyRuntimeState",
    "pub fn matches_ip",
    "reconnect_suppression_input_for_ip",
    "MAX_PEER_POLICY_RUNTIME_DECISIONS",
    "pub fn misbehavior_decisions",
    "pub fn ban_decisions",
    "pub fn unban_decisions",
  ]) {
    requireContains(peerPolicy, needle, "P96 pure runtime state", failures);
  }
}

function verifyManagedProjection(texts: Map<TargetFile, string>, failures: string[]): void {
  const network = `${texts.get("packages/open-bitcoin-node/src/network.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-node/src/network/peer_policy.rs") ?? ""
  }`;
  for (const needle of [
    "record_peer_policy_ban",
    "record_peer_policy_unban",
    "record_peer_policy_misbehavior",
    "peer_policy_runtime_state.misbehavior_decisions()",
    "peer_policy_runtime_state.ban_decisions()",
    "peer_policy_runtime_state.unban_decisions()",
  ]) {
    requireContains(network, needle, "P96 managed projection", failures);
  }
  if (/from_policy_decisions\([\s\S]*?&\[\][\s\S]*?&\[\][\s\S]*?&\[\]/.test(network)) {
    failures.push("P96 managed projection still passes empty decision slices");
  }
}

function verifyScopedReconnectSuppression(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const context = texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  requireContains(context, "remote_addr.ip()", "P96 reconnect suppression", failures);
  requireContains(context, "now_unix_seconds", "P96 reconnect suppression", failures);
  for (const forbidden of [
    "let _ = (remote_addr, now_unix_seconds)",
    "active_bans > 0",
    "discouraged_peers > 0",
  ]) {
    requireAbsent(context, forbidden, "P96 reconnect suppression", failures);
  }
}

function verifyStructuredLogEvidence(texts: Map<TargetFile, string>, failures: string[]): void {
  const logging = texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "";
  const context = texts.get("packages/open-bitcoin-rpc/src/context/peer_policy.rs") ?? "";
  for (const needle of [
    "INBOUND_PEER_POLICY_LOG_SOURCE",
    "inbound_peer_policy_log_record",
    "redacted_peer_policy_field",
  ]) {
    requireContains(logging, needle, "P96 structured log projection", failures);
  }
  for (const needle of [
    "Parity breadcrumbs:",
    "packages/bitcoin-knots/src/banman.cpp",
    "record_inbound_peer_policy_event_at",
    "append_structured_log_record",
  ]) {
    requireContains(context, needle, "P96 RPC peer-policy log context", failures);
  }
}

function verifyRawPeerPolicyBoundary(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  for (const file of RAW_OUTPUT_SCAN_FILES) {
    const text = texts.get(file) ?? "";
    for (const marker of RAW_PEER_POLICY_MARKERS) {
      if (text.includes(marker)) {
        failures.push(`P96 raw peer-policy marker leaked through output surface ${file}: ${marker}`);
      }
    }
  }
  const logging = texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "";
  const redaction = texts.get("packages/open-bitcoin-cli/src/operator/support/redaction.rs") ?? "";
  for (const marker of RAW_PEER_POLICY_MARKERS) {
    requireContains(`${logging}\n${redaction}`, marker, "P96 sanitizer raw marker coverage", failures);
  }
}

function verifyDocsAndParity(texts: Map<TargetFile, string>, failures: string[]): void {
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  const p2p = texts.get("docs/parity/catalog/p2p.md") ?? "";
  for (const text of [runtimeGuide, p2p]) {
    requireContains(text, "scoped runtime peer-policy bridge evidence", "P96 docs", failures);
    requireContains(text, "bounded reconnect suppression", "P96 docs", failures);
    requireContains(text, "not a public banlist", "P96 docs", failures);
  }
  for (const command of [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  ]) {
    requireContains(runtimeGuide, command, "P96 runtime guide UAT commands", failures);
  }
  for (const requirement of TRACEABILITY_REQUIREMENTS) {
    requireContains(p2p, requirement, "P96 catalog requirement traceability", failures);
  }
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  const sourceBreadcrumbs = texts.get("docs/parity/source-breadcrumbs.json") ?? "";
  for (const sourceFile of [
    "packages/open-bitcoin-node/src/network/peer_policy.rs",
    "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  ]) {
    requireContains(sourceBreadcrumbs, sourceFile, "P96 source breadcrumbs", failures);
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`P96 parity index JSON parse failed: ${String(error)}`);
    return;
  }
  const topSurface = Array.isArray(parsed.surfaces)
    ? (parsed.surfaces.find((entry) => (entry as ParitySurface).name === SURFACE_ID) as
        | ParitySurface
        | undefined)
    : undefined;
  if (topSurface?.status !== "done") {
    failures.push(`P96 parity index missing done top-level surface: ${SURFACE_ID}`);
  }
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("P96 parity checklist surfaces must be an array");
    return;
  }
  const surface = surfaces.find((entry) => (entry as ChecklistSurface).id === SURFACE_ID) as
    | ChecklistSurface
    | undefined;
  if (surface?.status !== "done") {
    failures.push(`P96 parity checklist missing done surface: ${SURFACE_ID}`);
  }
  requireExactArray(
    surface?.requirements,
    [],
    "P96 checklist requirements must not duplicate canonical v1.9 ownership",
    failures,
  );
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireArrayIncludes(surface?.upstream?.sources, "P96 upstream anchors", anchor, failures);
  }
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [
    PHASE95_TEST_COMMAND,
    PHASE95_CHECKER_COMMAND,
    PHASE96_TEST_COMMAND,
    PHASE96_CHECKER_COMMAND,
    "Phase 95 is followed by Phase 96",
  ]) {
    requireContains(text, command, "P96 verifier wiring", failures);
  }
  requireOrdered(text, PHASE95_CHECKER_COMMAND, PHASE96_TEST_COMMAND, "P96 visible verifier order", failures);
  requireOrdered(text, PHASE96_TEST_COMMAND, PHASE96_CHECKER_COMMAND, "P96 visible verifier order", failures);
  requireOrdered(
    text,
    'run_step "Phase 95 network participation release boundary checker"',
    'run_step "Phase 96 peer-policy runtime bridge checker tests"',
    "P96 executable verifier order",
    failures,
  );
  requireOrdered(
    text,
    'run_step "Phase 96 peer-policy runtime bridge checker tests"',
    'run_step "Phase 96 peer-policy runtime bridge checker"',
    "P96 executable verifier order",
    failures,
  );

  for (const line of text.split(/\r?\n/)) {
    const lower = line.toLowerCase();
    if (!lower.includes("phase 96") && !lower.includes("check-phase96-peer-policy-runtime-bridge")) {
      continue;
    }
    for (const forbidden of FORBIDDEN_PHASE96_VERIFY_GATES) {
      if (lower.includes(forbidden)) {
        failures.push(`P96 default verifier introduces forbidden gate '${forbidden}': ${line.trim()}`);
      }
    }
  }
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of [
    "docs/operator/runtime-guide.md",
    "docs/parity/catalog/p2p.md",
    "docs/parity/index.json",
  ] as const) {
    const lines = (texts.get(file) ?? "").split(/\r?\n/);
    lines.forEach((line, index) => {
      if (isForbiddenPositiveClaim(line)) {
        failures.push(`P96 forbidden positive claim in ${file}:${index + 1}: ${line.trim()}`);
      }
    });
  }
}

function isForbiddenPositiveClaim(line: string): boolean {
  const lower = ` ${line.toLowerCase()} `;
  const hasForbiddenPhrase = FORBIDDEN_CLAIM_PHRASES.some((phrase) => lower.includes(phrase));
  if (!hasForbiddenPhrase) {
    return false;
  }
  const hasPositiveMarker = POSITIVE_CLAIM_MARKERS.some((marker) => lower.includes(marker));
  if (!hasPositiveMarker) {
    return false;
  }
  return !ALLOWED_NEGATION_MARKERS.some((marker) => lower.includes(marker));
}

function requireContains(text: string, needle: string, label: string, failures: string[]): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireAbsent(text: string, needle: string, label: string, failures: string[]): void {
  if (text.includes(needle)) {
    failures.push(`${label} contains forbidden text: ${needle}`);
  }
}

function requireOrdered(
  text: string,
  before: string,
  after: string,
  label: string,
  failures: string[],
): void {
  const beforeIndex = text.indexOf(before);
  const afterIndex = text.indexOf(after);
  if (beforeIndex === -1 || afterIndex === -1 || beforeIndex >= afterIndex) {
    failures.push(`${label} must order '${before}' before '${after}'`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value) || !value.includes(required)) {
    failures.push(`${label} missing ${required}`);
  }
}

function requireExactArray(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} must be an array`);
    return;
  }
  const actual = [...value].map(String).sort();
  const sortedExpected = [...expected].sort();
  if (JSON.stringify(actual) !== JSON.stringify(sortedExpected)) {
    failures.push(`${label} expected ${sortedExpected.join(", ")} but found ${actual.join(", ")}`);
  }
}

if (import.meta.main) {
  const failures = checkPhase96PeerPolicyRuntimeBridge();
  if (failures.length > 0) {
    console.error("Phase 96 peer-policy runtime bridge check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log("Phase 96 peer-policy runtime bridge checks passed.");
}

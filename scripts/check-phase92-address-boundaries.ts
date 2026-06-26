#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-address-advertisement-discovery-boundaries";
const PHASE91_TEST_COMMAND = "bun test scripts/check-phase91-peer-permissions.test.ts";
const PHASE91_CHECKER_COMMAND = "bun run scripts/check-phase91-peer-permissions.ts";
const PHASE92_TEST_COMMAND = "bun test scripts/check-phase92-address-boundaries.test.ts";
const PHASE92_CHECKER_COMMAND = "bun run scripts/check-phase92-address-boundaries.ts";
const PHASE92_REQUIREMENTS = ["ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04"] as const;
const ADDRESS_PERMISSION_FLAG =
  "-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=in,noban,forceinbound,download,addr";
type RequiredCommand = { label: string; required: readonly [string, ...string[]] };

function requiredCommand(
  label: string,
  first: string,
  ...required: string[]
): RequiredCommand {
  return { label, required: [first, ...required] };
}

const TARGET_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
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
  "local_advertisement_candidates", "suppressed_advertisements",
  "not_publicly_routable", "bounded getaddr", "learned_address_entries",
  "learned_address_rejections", "latest_address_decision", "full_relay_deferred",
] as const;
const REQUIRED_UAT_COMMANDS = [
  requiredCommand(
    "Cargo address-boundary daemon startup",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
    "-openbitcoininbound=1",
    "-openbitcoinlisten=127.0.0.1:18444",
    ADDRESS_PERMISSION_FLAG,
  ),
  requiredCommand(
    "Bazel address-boundary daemon startup",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
    "-openbitcoininbound=1",
    "-openbitcoinlisten=127.0.0.1:18444",
    ADDRESS_PERMISSION_FLAG,
  ),
  requiredCommand(
    "Cargo address-boundary network status",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
    "openbitcoinnetworkstatus",
  ),
  requiredCommand(
    "Bazel address-boundary network status",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
    "openbitcoinnetworkstatus",
  ),
  requiredCommand(
    "Cargo address-boundary operator status",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "status --format json",
  ),
  requiredCommand(
    "Bazel address-boundary operator status",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "status --format json",
  ),
  requiredCommand(
    "Cargo address-boundary support bundle",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "support bundle --output-dir=/tmp/open-bitcoin-address-support",
  ),
  requiredCommand(
    "Bazel address-boundary support bundle",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "support bundle --output-dir=/tmp/open-bitcoin-address-support",
  ),
] as const;
const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/protocol.h", "packages/bitcoin-knots/src/netaddress.h",
  "packages/bitcoin-knots/src/netaddress.cpp", "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp", "packages/bitcoin-knots/src/addrman.h",
  "packages/bitcoin-knots/src/addrman.cpp", "packages/bitcoin-knots/src/addrdb.h",
  "packages/bitcoin-knots/src/addrdb.cpp",
] as const;
const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-network/src/address.rs",
  "packages/open-bitcoin-network/src/address/advertisement.rs",
  "packages/open-bitcoin-network/src/address/book.rs", "packages/open-bitcoin-network/src/address/response.rs",
  "packages/open-bitcoin-network/src/address/tests.rs",
] as const;
const REQUIRED_BREADCRUMB_ANCHORS = REQUIRED_CATALOG_ANCHORS;
const FORBIDDEN_VERIFY_STRINGS = [
  "curl ", "nc ", "systemctl", "launchctl",
  "dig ", "nslookup", "--public-network", "multi-day",
] as const;
const FORBIDDEN_POSITIVE_CLAIMS = [
  "supports full address relay", "full address relay support",
  "includes peer discovery support", "peer discovery support",
  "public inbound by default is enabled", "public inbound by default",
  "public-network readiness is achieved", "public-network readiness",
  "production full-node readiness is achieved", "production full-node readiness",
  "unsolicited address relay is supported", "unsolicited address relay support",
  "addr gossip relay is supported", "addr gossip relay support",
  "dns seed discovery support is enabled", "dns seed discovery support",
  "upnp/nat-pmp discovery support is enabled", "upnp/nat-pmp discovery support",
  "-discover parity is supported", "-discover parity",
  "-externalip parity is supported", "-externalip parity",
] as const;
const ALLOWED_SCOPE_TERMS = [
  "does not", "do not", "not ", "no ", "without", "outside", "remain outside",
  "remains outside", "deferred", "future", "not claim", "not claiming", "no-claim",
  "non-claim",
] as const;
const RAW_EVIDENCE_STRINGS = [
  "127.0.0.1:", "0.0.0.0:", "::1", "address_bytes",
  "peer_id=", "raw_permission", "operator_loopback", "inbound.allow_public=true",
] as const;
const RAW_EVIDENCE_FILES = [
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
] as const;
const COMMAND_PREFIXES = Array.from(
  new Set(REQUIRED_UAT_COMMANDS.map((command) => command.required[0])),
);

export type CheckPhase92Options = { rootDir?: string };

type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };

type BreadcrumbIndex = { groups?: unknown };

type ChecklistSurface = {
  evidence?: unknown; id?: unknown; requirements?: unknown; status?: unknown;
};

type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };

type ParitySurface = { name?: unknown; status?: unknown };

type TargetFile = (typeof TARGET_FILES)[number];

export function checkPhase92AddressBoundaries(
  options: CheckPhase92Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanEvidence(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifyRawEvidenceBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required Phase 92 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizeShellCommand(text: string): string {
  return normalizeWhitespace(text.replace(/\\\s*/g, " "));
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

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`Phase 92 requirement coverage ${label} must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE92_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(
      `Phase 92 requirement coverage ${label} mismatch: expected ${expected}, got ${actual}`,
    );
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 92 requirement coverage parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 92 requirement coverage parity index surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`Phase 92 requirement coverage missing done surface: ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("Phase 92 requirement coverage checklist.surfaces must be an array");
    return;
  }

  const surface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`Phase 92 requirement coverage checklist missing done ${SURFACE_ID}`);
  }
  requireExactRequirements(surface?.requirements, `${SURFACE_ID}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(
      surface?.evidence,
      `Phase 92 requirement coverage ${SURFACE_ID}.evidence`,
      evidence,
      failures,
    );
  }
}

function verifyHumanEvidence(texts: Map<TargetFile, string>, failures: string[]): void {
  verifyRuntimeGuideCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifyEvidenceLabels(texts, failures);
  verifyParityDocs(texts, failures);
}

function verifyRuntimeGuideCommands(text: string, failures: string[]): void {
  const commandUnits = shellCommandUnits(text);
  for (const command of REQUIRED_UAT_COMMANDS) {
    const commandFound = commandUnits.some((unit) =>
      command.required.every((required) => unit.includes(normalizeShellCommand(required))),
    );
    if (!commandFound) {
      failures.push(`Phase 92 UAT command missing ${command.label}: ${command.required.join(" ")}`);
    }
  }
}

function shellCommandUnits(text: string): string[] {
  const units: string[] = [];
  let currentLines: string[] = [];

  for (const rawLine of text.replaceAll("\r\n", "\n").split("\n")) {
    const line = rawLine.trim();
    const lineStartsCommand = COMMAND_PREFIXES.some((prefix) => line.startsWith(prefix));

    if (lineStartsCommand) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [line];
      continue;
    }

    if (currentLines.length === 0) {
      continue;
    }

    if (line.length === 0 || line.startsWith("```")) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [];
      continue;
    }

    currentLines.push(line);
  }

  pushCurrentShellCommandUnit(currentLines, units);
  return units;
}

function pushCurrentShellCommandUnit(currentLines: string[], units: string[]): void {
  if (currentLines.length === 0) {
    return;
  }

  units.push(normalizeShellCommand(currentLines.join("\n")));
}

function verifyEvidenceLabels(texts: Map<TargetFile, string>, failures: string[]): void {
  const corpus = [
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/status/render/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/render/inbound.rs") ?? "",
  ].join("\n");

  for (const label of REQUIRED_LABELS) {
    requireNormalizedContains(corpus, label, "Phase 92 evidence label", failures);
  }
}

function verifyParityDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "Phase 92 parity catalog", failures);
  requireContains(checklistText, SURFACE_ID, "Phase 92 parity checklist", failures);

  for (const requirement of PHASE92_REQUIREMENTS) {
    requireContains(p2pText, requirement, "Phase 92 parity catalog", failures);
    requireContains(checklistText, requirement, "Phase 92 parity checklist", failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireContains(p2pText, anchor, "Phase 92 parity catalog", failures);
  }
}

function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  let parsed: BreadcrumbIndex;
  try {
    parsed = JSON.parse(text) as BreadcrumbIndex;
  } catch (error) {
    failures.push(`Phase 92 source breadcrumb coverage JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("Phase 92 source breadcrumb coverage groups must be an array");
    return;
  }

  const maybeGroup = parsed.groups.find((entry) => {
    const group = entry as BreadcrumbGroup;
    return group.label === "network-address-boundaries";
  }) as BreadcrumbGroup | undefined;

  if (maybeGroup === undefined) {
    failures.push("Phase 92 source breadcrumb coverage missing network-address-boundaries group");
    return;
  }

  for (const file of REQUIRED_BREADCRUMB_FILES) {
    requireArrayIncludes(
      maybeGroup.files,
      "Phase 92 source breadcrumb coverage files",
      file,
      failures,
    );
  }
  for (const anchor of REQUIRED_BREADCRUMB_ANCHORS) {
    requireArrayIncludes(
      maybeGroup.breadcrumbs,
      "Phase 92 source breadcrumb coverage breadcrumbs",
      anchor,
      failures,
    );
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  verifyPrintedVerifierOrder(text, failures);

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 92 address boundaries checker" ${PHASE92_TEST_COMMAND}`,
    "verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 92 address boundaries" ${PHASE92_CHECKER_COMMAND}`,
    "verifier-order",
    failures,
  );
  verifyExecutableVerifierOrder(executableText, failures);
  verifyDefaultVerifierBoundary(executableText, failures);
}

function verifyPrintedVerifierOrder(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("verifier-order missing VERIFY_COMMAND_ORDER block");
    return;
  }

  verifyOrderedCommands(
    maybeOrderBlock[1],
    [
      PHASE91_TEST_COMMAND,
      PHASE91_CHECKER_COMMAND,
      PHASE92_TEST_COMMAND,
      PHASE92_CHECKER_COMMAND,
    ],
    "verifier-order requires printed Phase 92 commands after Phase 91 commands",
    failures,
  );
}

function verifyExecutableVerifierOrder(executableText: string, failures: string[]): void {
  verifyOrderedCommands(
    executableText,
    [
      PHASE91_TEST_COMMAND,
      PHASE91_CHECKER_COMMAND,
      PHASE92_TEST_COMMAND,
      PHASE92_CHECKER_COMMAND,
      "bash scripts/check-pure-core-deps.sh",
    ],
    "verifier-order requires executed Phase 92 test and checker after Phase 91 and before pure-core checks",
    failures,
  );
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

function verifyDefaultVerifierBoundary(executableText: string, failures: string[]): void {
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (containsForbiddenVerifyFragment(executableText, forbidden)) {
      failures.push(
        `Phase 92 default verifier boundary must not add forbidden command text: ${forbidden}`,
      );
    }
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
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
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
    if (lower.includes(claim)) {
      failures.push(`Phase 92 no-claim boundary forbidden claim in ${file}: ${unit}`);
    }
  }
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term));
}

function verifyRawEvidenceBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of RAW_EVIDENCE_FILES) {
    const text = texts.get(file) ?? "";
    for (const rawDetail of RAW_EVIDENCE_STRINGS) {
      if (text.includes(rawDetail)) {
        failures.push(`Phase 92 raw evidence boundary raw detail in ${file}: ${rawDetail}`);
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

    const tableRows = lines.filter(
      (line) => line.startsWith("|") && !/^\|\s*-/.test(line),
    );
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      const prose = lines.filter((line) => !line.startsWith("|")).join(" ");
      units.push(...sentenceUnits(prose));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }

  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  if (normalized.length === 0) {
    return [];
  }

  return normalized.split(/(?<=[.!?])\s+/);
}

if (import.meta.main) {
  const failures = checkPhase92AddressBoundaries();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 92 address advertisement and discovery boundary evidence");
  }
}

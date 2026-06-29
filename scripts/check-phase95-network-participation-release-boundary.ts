#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-network-participation-release-boundary";
const PHASE94_TEST_COMMAND = "bun test scripts/check-phase94-dos-resource-governance.test.ts";
const PHASE94_CHECKER_COMMAND = "bun run scripts/check-phase94-dos-resource-governance.ts";
const PHASE95_TEST_COMMAND = "bun test scripts/check-phase95-network-participation-release-boundary.test.ts";
const PHASE95_CHECKER_COMMAND = "bun run scripts/check-phase95-network-participation-release-boundary.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_PHASE95_REQUIREMENTS = [
  "BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05", "BOUND-06",
] as const;
const PHASE_REQUIREMENTS = {
  "v1-9-inbound-listener-admission-policy": [
    "INB-01", "INB-02", "INB-03", "INB-04", "INB-05",
  ],
  "v1-9-peer-permissions-connection-classes": [
    "PERM-01", "PERM-02", "PERM-03", "PERM-04",
  ],
  "v1-9-address-advertisement-discovery-boundaries": [
    "ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04",
  ],
  "v1-9-eviction-ban-misbehavior-policy": [
    "EVICT-01", "EVICT-02", "EVICT-03", "EVICT-04",
  ],
  "v1-9-dos-resource-governance": [
    "DOS-01", "DOS-02", "DOS-03", "DOS-04", "DOS-05",
  ],
  [SURFACE_ID]: REQUIRED_PHASE95_REQUIREMENTS,
} as const;
const REQUIRED_V1_9_REQUIREMENTS = Object.values(PHASE_REQUIREMENTS).flat();
const REQUIREMENT_PHASE_ASSIGNMENTS = {
  "INB-01": 98,
  "INB-02": 98,
  "INB-03": 98,
  "INB-04": 98,
  "INB-05": 97,
  "PERM-01": 91,
  "PERM-02": 91,
  "PERM-03": 91,
  "PERM-04": 91,
  "ADDR-01": 92,
  "ADDR-02": 92,
  "ADDR-03": 92,
  "ADDR-04": 92,
  "EVICT-01": 93,
  "EVICT-02": 93,
  "EVICT-03": 96,
  "EVICT-04": 96,
  "DOS-01": 94,
  "DOS-02": 94,
  "DOS-03": 96,
  "DOS-04": 97,
  "DOS-05": 94,
  "BOUND-01": 95,
  "BOUND-02": 95,
  "BOUND-03": 95,
  "BOUND-04": 95,
  "BOUND-05": 95,
  "BOUND-06": 98,
} as const;
const ROADMAP_TRACEABILITY_ROWS = [
  { phase: 90, requirements: [] },
  { phase: 91, requirements: ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] },
  { phase: 92, requirements: ["ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04"] },
  { phase: 93, requirements: ["EVICT-01", "EVICT-02"] },
  { phase: 94, requirements: ["DOS-01", "DOS-02", "DOS-05"] },
  { phase: 95, requirements: ["BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05"] },
  { phase: 96, requirements: ["EVICT-03", "EVICT-04", "DOS-03"] },
  { phase: 97, requirements: ["INB-05", "DOS-04"] },
  { phase: 98, requirements: ["INB-01", "INB-02", "INB-03", "INB-04", "BOUND-06"] },
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp", "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/addrman.cpp", "packages/bitcoin-knots/src/banman.cpp",
  "packages/bitcoin-knots/src/net_permissions.cpp",
] as const;
const REQUIRED_PHASE95_EVIDENCE = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase95-network-participation-release-boundary.ts",
  "scripts/check-phase95-network-participation-release-boundary.test.ts",
  "scripts/verify.sh",
  ".planning/milestones/v1.9-REQUIREMENTS.md",
  ".planning/milestones/v1.9-ROADMAP.md",
] as const;
const REQUIRED_UAT_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin",
  "openbitcoinnetworkstatus",
  "status --format json",
  "support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
] as const;
const REQUIRED_SUPPORT_REDACTION_ROOTS = [
  "INBOUND_ENDPOINT_REDACTION_SAFEGUARD",
  "INBOUND_PERMISSION_REDACTION_SAFEGUARD",
  "INBOUND_ADDRESS_REDACTION_SAFEGUARD",
  "INBOUND_PEER_POLICY_REDACTION_SAFEGUARD",
  "INBOUND_RESOURCE_GOVERNANCE_REDACTION_SAFEGUARD",
  "inbound resource-governance evidence bounded/redacted",
  "redact_inbound_resource_governance_evidence",
  "redacted_resource_governance_evidence",
  "sanitized_resource_governance_text",
  "inbound_support_redacts_raw_phase94_resource_governance_material",
  "peer_id=",
  "raw_endpoint",
  "payload_bytes",
  "permission_string",
  "credential",
  "secret",
  "cookie=",
  "config=",
] as const;
const FORBIDDEN_POSITIVE_CLAIMS = [
  "transaction relay support",
  "compact block relay support",
  "mempool propagation support",
  "full address relay support",
  "full address relay",
  "public inbound default",
  "public inbound defaults",
  "public inbound by default",
  "public-network ci",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
] as const;
const POSITIVE_CLAIM_VERBS = [
  "provides", "provide", "supports", "support", "adds", "add", "enables", "enable",
  "includes", "include", "ships", "ship", "has", "have",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "openbitcoinlisten=0.0.0.0",
  "public-network CI",
  "production full-node readiness",
  "production service operation",
  "production-service operation",
  "systemctl",
  "launchctl",
  "service-manager",
  "sleep 259200",
  "sleep 86400",
  "--restart-after-progress",
  "run-live-mainnet-smoke",
] as const;
const CLAIM_SCAN_FILES = [
  "README.md",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
] as const;
const TARGET_FILES = [
  ".planning/milestones/v1.9-REQUIREMENTS.md",
  ".planning/milestones/v1.9-ROADMAP.md",
  "README.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = { name?: unknown; status?: unknown };
type CheckPhase95Options = { rootDir?: string };

export function checkPhase95NetworkParticipationReleaseBoundary(
  options: CheckPhase95Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyKnotsAnchors(texts, failures);
  verifyNoClaimBoundary(texts, failures);
  verifyUatCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifySupportRedactionRoots(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyRequirementTraceability(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`BOUND-03 missing required Phase 95 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`BOUND-06 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyPhaseRequirementSurfaces(parsed, failures);
  verifyPhase95ChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("BOUND-06 parity index surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`BOUND-06 parity index missing done surface: ${SURFACE_ID}`);
  }
}

function verifyPhaseRequirementSurfaces(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("BOUND-06 parity checklist surfaces must be an array");
    return;
  }

  for (const [surfaceId, expectedRequirements] of Object.entries(PHASE_REQUIREMENTS)) {
    const surface = surfaces.find((entry) => {
      const maybeSurface = entry as ChecklistSurface;
      return maybeSurface.id === surfaceId;
    }) as ChecklistSurface | undefined;
    if (surface?.status !== "done") {
      failures.push(`BOUND-06 parity checklist missing done v1.9 surface: ${surfaceId}`);
    }
    requireExactRequirements(
      surface?.requirements,
      expectedRequirements,
      `BOUND-06 parity checklist ${surfaceId}`,
      failures,
    );
  }
  verifyRequirementCountsFromArrays(
    surfaces
      .map((entry) => (entry as ChecklistSurface).requirements)
      .filter(Array.isArray)
      .flat() as string[],
    "BOUND-06 parity index v1.9 checklist surfaces",
    failures,
  );
}

function verifyPhase95ChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    return;
  }

  const surface = surfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  for (const evidence of REQUIRED_PHASE95_EVIDENCE) {
    requireArrayIncludes(surface?.evidence, `BOUND-06 ${SURFACE_ID}.evidence`, evidence, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireArrayIncludes(
      surface?.upstream?.sources,
      `BOUND-02 ${SURFACE_ID}.upstream.sources`,
      anchor,
      failures,
    );
  }
}

function requireExactRequirements(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const wanted = JSON.stringify(expected);
  if (actual !== wanted) {
    failures.push(`${label} requirements mismatch: expected ${wanted}, got ${actual}`);
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

function verifyKnotsAnchors(texts: Map<TargetFile, string>, failures: string[]): void {
  const catalogText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(catalogText, anchor, "BOUND-02 P2P catalog Knots anchors", failures);
    requireContains(
      releaseReadiness,
      anchor.replace("packages/bitcoin-knots/src/", ""),
      "BOUND-02 release-readiness Knots anchor rollup",
      failures,
    );
  }
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of CLAIM_SCAN_FILES) {
    const text = texts.get(file) ?? "";
    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  if (isExplicitDeferredMatrixRow(unit)) {
    return;
  }

  const lower = normalizedLower(unit);
  for (const claim of FORBIDDEN_POSITIVE_CLAIMS) {
    if (lower.includes(claim) && isPositiveClaim(lower, claim)) {
      failures.push(`BOUND-01 forbidden v1.9 network participation claim in ${file}: ${unit}`);
    }
  }
}

function isPositiveClaim(lowerUnit: string, claim: string): boolean {
  return (
    POSITIVE_CLAIM_VERBS.some((verb) => containsUnnegatedVerbClaim(lowerUnit, verb, claim))
    || [
      `${claim} support is enabled`,
      `${claim} support is supported`,
      `${claim} is available`,
      `${claim} is supported`,
      `${claim} is enabled`,
      `${claim} is complete`,
      `${claim} is achieved`,
      `${claim} readiness is achieved`,
    ].some((phrase) => lowerUnit.includes(phrase))
  );
}

function containsUnnegatedVerbClaim(lowerUnit: string, verb: string, claim: string): boolean {
  const phrase = `${verb} ${claim}`;
  let searchFrom = 0;
  while (searchFrom < lowerUnit.length) {
    const phraseIndex = lowerUnit.indexOf(phrase, searchFrom);
    if (phraseIndex === -1) {
      return false;
    }
    const prefix = lowerUnit.slice(Math.max(0, phraseIndex - 16), phraseIndex);
    if (!/(does not |do not |doesn't |don't |not )$/.test(prefix)) {
      return true;
    }
    searchFrom = phraseIndex + phrase.length;
  }
  return false;
}

function isExplicitDeferredMatrixRow(unit: string): boolean {
  const lower = normalizedLower(unit);
  return (
    lower.startsWith("|")
    && lower.includes("| `deferred` |")
    && lower.includes("| not allowed yet |")
  );
}

function verifyUatCommands(text: string, failures: string[]): void {
  for (const command of REQUIRED_UAT_COMMANDS) {
    requireContains(text, command, "BOUND-04 Phase 95 UAT command family", failures);
  }
}

function verifySupportRedactionRoots(texts: Map<TargetFile, string>, failures: string[]): void {
  const supportText = [
    texts.get("packages/open-bitcoin-cli/src/operator/support/redaction.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "",
  ].join("\n");

  for (const root of REQUIRED_SUPPORT_REDACTION_ROOTS) {
    requireContains(supportText, root, "BOUND-05 support redaction roots", failures);
  }
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("BOUND-03 verifier-order missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [
        PHASE94_TEST_COMMAND,
        PHASE94_CHECKER_COMMAND,
        PHASE95_TEST_COMMAND,
        PHASE95_CHECKER_COMMAND,
      ],
      "BOUND-03 verifier-order printed commands must place Phase 95 immediately after Phase 94",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "Phase 95 network participation release boundary checker tests" ${PHASE95_TEST_COMMAND}`,
    "BOUND-03 executable verifier Phase 95 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "Phase 95 network participation release boundary checker" ${PHASE95_CHECKER_COMMAND}`,
    "BOUND-03 executable verifier Phase 95 checker",
    failures,
  );
  requireContains(
    text,
    "Phase 94 is followed by Phase 95",
    "BOUND-03 verifier ordering comment",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE94_TEST_COMMAND,
      PHASE94_CHECKER_COMMAND,
      PHASE95_TEST_COMMAND,
      PHASE95_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "BOUND-03 executable verifier commands must run Phase 95 after Phase 94 and before pure-core checks",
    failures,
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`BOUND-03 default verifier boundary contains forbidden text: ${forbidden}`);
    }
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
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

function verifyRequirementTraceability(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  verifyChecklistMarkdown(texts.get("docs/parity/checklist.md") ?? "", failures);
  verifyRequirementsTable(texts.get(".planning/milestones/v1.9-REQUIREMENTS.md") ?? "", failures);
  verifyRoadmapTraceability(texts.get(".planning/milestones/v1.9-ROADMAP.md") ?? "", failures);
}

function verifyChecklistMarkdown(text: string, failures: string[]): void {
  const ids = extractRequirementIdsFromSurfaceRows(text);
  verifyRequirementCountsFromArrays(ids, "BOUND-06 parity checklist markdown v1.9 rows", failures);
}

function extractRequirementIdsFromSurfaceRows(text: string): string[] {
  const ids: string[] = [];
  for (const surfaceId of Object.keys(PHASE_REQUIREMENTS)) {
    const row = text
      .split("\n")
      .find((line) => line.startsWith("|") && line.includes(surfaceId));
    if (row === undefined) {
      continue;
    }
    ids.push(...requirementIds(row));
  }
  return ids;
}

function verifyRequirementsTable(text: string, failures: string[]): void {
  requireContains(text, "v1.9 requirements: 28 total", "BOUND-06 requirements coverage", failures);
  requireContains(text, "Mapped to phases: 28", "BOUND-06 requirements coverage", failures);
  requireContains(text, "Unmapped: 0", "BOUND-06 requirements coverage", failures);
  verifyRequirementCountsFromArrays(
    Object.keys(REQUIREMENT_PHASE_ASSIGNMENTS),
    "BOUND-06 requirements traceability assignment map",
    failures,
  );
  for (const [requirement, phase] of Object.entries(REQUIREMENT_PHASE_ASSIGNMENTS)) {
    const rowPattern = new RegExp(`\\|\\s*${escapeRegExp(requirement)}\\s*\\|\\s*Phase ${phase}\\s*\\|`);
    if (!rowPattern.test(text)) {
      failures.push(`BOUND-06 requirements traceability missing ${requirement} -> Phase ${phase}`);
    }
  }
}

function verifyRoadmapTraceability(text: string, failures: string[]): void {
  requireContains(
    text,
    "Coverage:** 28/28 v1.9 requirements mapped, 0 unmapped",
    "BOUND-06 roadmap coverage",
    failures,
  );
  verifyRequirementCountsFromArrays(
    ROADMAP_TRACEABILITY_ROWS.flatMap(({ requirements }) => requirements),
    "BOUND-06 roadmap traceability rows",
    failures,
  );
  for (const { phase, requirements } of ROADMAP_TRACEABILITY_ROWS) {
    const requirementText = requirements.length === 0 ? "—" : requirements.join(", ");
    const expected = `| Phase ${phase} | ${requirementText} | ${requirements.length} |`;
    requireContains(text, expected, "BOUND-06 roadmap phase traceability", failures);
  }
}

function verifyRequirementCountsFromArrays(
  ids: readonly string[],
  label: string,
  failures: string[],
): void {
  const counts = new Map<string, number>();
  for (const id of ids) {
    if (!REQUIRED_V1_9_REQUIREMENTS.includes(id)) {
      continue;
    }
    counts.set(id, (counts.get(id) ?? 0) + 1);
  }
  for (const id of REQUIRED_V1_9_REQUIREMENTS) {
    const count = counts.get(id) ?? 0;
    if (count !== 1) {
      failures.push(`${label} BOUND-06 expected ${id} exactly once, found ${count}`);
    }
  }
}

function requirementIds(text: string): string[] {
  return text.match(/\b(?:INB|PERM|ADDR|EVICT|DOS|BOUND)-\d{2}\b/g) ?? [];
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

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

if (import.meta.main) {
  const failures = checkPhase95NetworkParticipationReleaseBoundary();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 95 network participation release boundary");
  }
}

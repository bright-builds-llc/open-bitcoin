#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-parity-uat-release-boundary";
const PHASE105_TEST_COMMAND = "bun test scripts/check-phase105-operator-relay-evidence.test.ts";
const PHASE105_CHECKER_COMMAND = "bun run scripts/check-phase105-operator-relay-evidence.ts";
const PHASE106_TEST_COMMAND = "bun test scripts/check-phase106-parity-uat-release-boundary.test.ts";
const PHASE106_CHECKER_COMMAND = "bun run scripts/check-phase106-parity-uat-release-boundary.ts";
const V2_REQUIREMENTS_BY_SURFACE = {
  "v2-0-relay-activation-boundary": ["ACT-01", "ACT-02", "ACT-03", "ACT-04"],
  "v2-0-transaction-inventory-download-scheduling": [
    "INV-01",
    "INV-02",
    "INV-03",
    "INV-04",
    "DL-01",
    "DL-02",
  ],
  "v2-0-orphan-handling-admission-outcome-bridge": [
    "DL-03",
    "DL-04",
    "DL-05",
    "MEM-01",
    "MEM-02",
  ],
  "v2-0-mempool-chainstate-lifecycle-durable-recovery": [
    "MEM-03",
    "MEM-04",
    "MEM-05",
    "MEM-06",
  ],
  "v2-0-relay-serving-fanout-rebroadcast-policy": ["REL-01", "REL-02", "REL-03", "REL-04"],
  "v2-0-operator-rpc-metrics-logs-support-evidence": ["OBS-01", "OBS-02", "OBS-03", "OBS-04"],
  [SURFACE_ID]: ["BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05"],
} as const;
const REQUIREMENT_PHASE_ASSIGNMENTS = {
  "ACT-01": "107",
  "ACT-02": "107",
  "ACT-03": "100",
  "ACT-04": "100",
  "INV-01": "101",
  "INV-02": "107",
  "INV-03": "107",
  "INV-04": "101",
  "DL-01": "107",
  "DL-02": "107",
  "DL-03": "102",
  "DL-04": "102",
  "DL-05": "102",
  "MEM-01": "102",
  "MEM-02": "102",
  "MEM-03": "103",
  "MEM-04": "108",
  "MEM-05": "108",
  "MEM-06": "108",
  "REL-01": "108",
  "REL-02": "108",
  "REL-03": "107",
  "REL-04": "104",
  "OBS-01": "105",
  "OBS-02": "105",
  "OBS-03": "105",
  "OBS-04": "105",
  "BOUND-01": "106",
  "BOUND-02": "106",
  "BOUND-03": "106",
  "BOUND-04": "106",
  "BOUND-05": "106",
} as const;
const TARGET_FILES = [
  "README.md",
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/release-readiness.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase106-parity-uat-release-boundary.ts",
  "scripts/check-phase106-parity-uat-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE_ROOTS = [
  "README.md",
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase106-parity-uat-release-boundary.ts",
  "scripts/check-phase106-parity-uat-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/protocol.h",
  "packages/bitcoin-knots/src/node/txdownloadman.h",
  "packages/bitcoin-knots/src/node/txdownloadman_impl.cpp",
  "packages/bitcoin-knots/src/txorphanage.cpp",
  "packages/bitcoin-knots/src/txmempool.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/policy/policy.cpp",
  "packages/bitcoin-knots/src/rpc/net.cpp",
  "packages/bitcoin-knots/src/rpc/mempool.cpp",
  "packages/bitcoin-knots/src/rpc/rawtransaction.cpp",
  "packages/bitcoin-knots/test/functional/p2p_tx_download.py",
  "packages/bitcoin-knots/test/functional/p2p_orphan_handling.py",
  "packages/bitcoin-knots/test/functional/mempool_accept.py",
  "packages/bitcoin-knots/test/functional/rpc_rawtransaction.py",
] as const;
const REQUIRED_SOURCE_BREADCRUMBS = [
  "packages/open-bitcoin-network/src/relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -regtest openbitcoinnetworkstatus",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -regtest openbitcoinnetworkstatus",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-relay-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-relay-support",
  PHASE106_TEST_COMMAND,
  PHASE106_CHECKER_COMMAND,
  "bash scripts/verify.sh",
] as const;
const REQUIRED_GAP_TERMS = [
  "compact block relay",
  "package relay",
  "bloom/filter serving",
  "public relay defaults",
  "public relay by default",
  "public-network relay CI",
  "production service operation",
  "production-service proof",
  "production full-node readiness",
  "production full-node readiness proof",
  "production-funds wallet use",
  "production-funds wallet safety proof",
] as const;
const FORBIDDEN_CLAIMS = [
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
  "service-manager",
  "production-deployment",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type V2SurfaceId = keyof typeof V2_REQUIREMENTS_BY_SURFACE;
type RequirementId = keyof typeof REQUIREMENT_PHASE_ASSIGNMENTS;
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
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };

export function checkPhase106ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE106_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkParityIndex(texts, failures);
  checkRequirementTraceability(texts, failures);
  checkRequiredText(texts, failures);
  checkSourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkVerifierOrder(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenDefaultVerifierGates(texts.get("scripts/verify.sh") ?? "", failures);
  checkForbiddenClaims(texts, failures);

  return failures;
}

function checkParityIndex(texts: TextCorpus, failures: string[]): void {
  const raw = texts.get("docs/parity/index.json") ?? "";
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`docs/parity/index.json is not valid JSON: ${String(error)}`);
    return;
  }

  const topSurfaces = Array.isArray(parsed.surfaces) ? (parsed.surfaces as ParitySurface[]) : [];
  for (const surfaceId of Object.keys(V2_REQUIREMENTS_BY_SURFACE) as V2SurfaceId[]) {
    const matches = topSurfaces.filter((surface) => surface.name === surfaceId);
    if (matches.length !== 1) {
      failures.push(`expected exactly one top-level v2.0 surface ${surfaceId}`);
      continue;
    }
    if (matches[0].status !== "done") {
      failures.push(`${surfaceId}: expected top-level status done`);
    }
  }

  const checklistSurfaces = Array.isArray(parsed.checklist?.surfaces)
    ? (parsed.checklist.surfaces as ParitySurface[])
    : [];
  const observedRequirements = new Map<string, string[]>();
  for (const surfaceId of Object.keys(V2_REQUIREMENTS_BY_SURFACE) as V2SurfaceId[]) {
    const matches = checklistSurfaces.filter((surface) => surface.id === surfaceId);
    if (matches.length !== 1) {
      failures.push(`expected exactly one parity checklist surface ${surfaceId}`);
      continue;
    }
    const surface = matches[0];
    if (surface.status !== "done") {
      failures.push(`${surfaceId}: expected checklist status done`);
    }
    const requirements = asStringArray(surface.requirements);
    const expected = [...V2_REQUIREMENTS_BY_SURFACE[surfaceId]];
    if (!sameMembers(requirements, expected)) {
      failures.push(`${surfaceId}: expected requirements ${expected.join(", ")}`);
    }
    for (const requirement of requirements) {
      const owners = observedRequirements.get(requirement) ?? [];
      owners.push(surfaceId);
      observedRequirements.set(requirement, owners);
    }
  }

  const expectedRequirements = Object.keys(REQUIREMENT_PHASE_ASSIGNMENTS);
  if (expectedRequirements.length !== 32) {
    failures.push(`internal checker error: expected 32 v2.0 requirements, found ${expectedRequirements.length}`);
  }
  for (const requirement of expectedRequirements) {
    const owners = observedRequirements.get(requirement) ?? [];
    if (owners.length !== 1) {
      failures.push(`v2.0 requirement ${requirement} must have exactly one parity surface owner; found ${owners.length}`);
    }
  }

  checkPhase106Surface(checklistSurfaces, failures);
}

function checkPhase106Surface(surfaces: ParitySurface[], failures: string[]): void {
  const surface = surfaces.find((candidate) => candidate.id === SURFACE_ID);
  if (!surface) {
    failures.push(`missing parity checklist surface ${SURFACE_ID}`);
    return;
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

function checkRequirementTraceability(texts: TextCorpus, failures: string[]): void {
  const requirementsText = texts.get(".planning/REQUIREMENTS.md") ?? "";
  for (const [requirement, phase] of Object.entries(REQUIREMENT_PHASE_ASSIGNMENTS) as Array<
    [RequirementId, string]
  >) {
    const rowNeedle = `| ${requirement} | Phase ${phase} |`;
    const count = countOccurrences(requirementsText, rowNeedle);
    if (count !== 1) {
      failures.push(`requirement traceability: ${requirement} must map to Phase ${phase} exactly once`);
    }
  }

  const roadmapText = texts.get(".planning/ROADMAP.md") ?? "";
  const phase106Row = roadmapText
    .split("\n")
    .find((line) => line.startsWith("| 106 |") && line.includes("Parity Traceability"));
  if (!phase106Row) {
    failures.push("roadmap traceability: missing Phase 106 roadmap row");
    return;
  }
  for (const requirement of V2_REQUIREMENTS_BY_SURFACE[SURFACE_ID]) {
    if (!phase106Row.includes(requirement)) {
      failures.push(`roadmap traceability: Phase 106 row missing ${requirement}`);
    }
  }
}

function checkRequiredText(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  for (const requirement of V2_REQUIREMENTS_BY_SURFACE[SURFACE_ID]) {
    if (!corpus.includes(requirement)) {
      failures.push(`missing Phase 106 requirement ${requirement}`);
    }
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!corpus.includes(anchor)) {
      failures.push(`missing Phase 106 Knots anchor ${anchor}`);
    }
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 106 runtime guide command ${command}`);
    }
  }
}

function checkSourceBreadcrumbs(raw: string, failures: string[]): void {
  for (const file of REQUIRED_SOURCE_BREADCRUMBS) {
    if (!raw.includes(file)) {
      failures.push(`source breadcrumbs missing v2.0 evidence file ${file}`);
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
      PHASE105_TEST_COMMAND,
      PHASE105_CHECKER_COMMAND,
      PHASE106_TEST_COMMAND,
      PHASE106_CHECKER_COMMAND,
    ])
  ) {
    failures.push("verifier-scope: Phase 106 visible order must follow Phase 105");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 105 operator relay evidence checker"',
      'run_step "check Phase 105 operator relay evidence"',
      'run_step "test Phase 106 parity UAT release boundary checker"',
      'run_step "check Phase 106 parity UAT release boundary"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 106 executable order must follow Phase 105 and precede pure-core checks");
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

function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && file !== "README.md") {
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
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 106 claim: ${forbidden}`);
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

  return readFileSync(absolutePath, "utf8");
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

function countOccurrences(text: string, needle: string): number {
  if (needle.length === 0) {
    return 0;
  }
  return text.split(needle).length - 1;
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

if (import.meta.main) {
  const failures = checkPhase106ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 106 parity UAT release boundary check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 106 parity UAT release boundary validated.");
}

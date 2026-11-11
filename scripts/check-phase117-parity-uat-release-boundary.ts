#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE117_TEST = "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK = "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
const ARCHIVED_V21_REQUIREMENTS =
  ".planning/milestones/v2.1-REQUIREMENTS.md";

const REQUIREMENTS_BY_SURFACE = {
  "v2-1-block-serving-activation-eligibility-boundary": [
    "BSRV-01",
    "BSRV-02",
    "BSRV-03",
    "BSRV-05",
    "BSRV-06",
  ],
  "v2-1-full-block-serving-request-path": ["BSRV-04", "GOV-01", "GOV-05"],
  "v2-1-bip152-wire-codec-message-semantics": ["CMP-01", "CMP-02", "CMP-03", "RCN-01"],
  "v2-1-compact-relay-negotiation-announcement-policy": ["CMP-04", "CMP-05", "CMP-06"],
  "v2-1-compact-block-reconstruction": ["RCN-02", "RCN-03", "GOV-04"],
  "v2-1-missing-transaction-fallback-validation-handoff": [
    "RCN-04",
    "RCN-05",
    "RCN-06",
    "RCN-07",
    "GOV-02",
    "GOV-03",
  ],
  "v2-1-operator-block-relay-evidence": ["OBS-01", "OBS-02", "OBS-03", "OBS-04", "OBS-05"],
  "v2-1-parity-uat-release-boundary": [
    "BOUND-01",
    "BOUND-02",
    "BOUND-03",
    "BOUND-04",
    "BOUND-05",
  ],
} as const;

const TARGET_FILES = [
  "README.md",
  ".planning/REQUIREMENTS.md",
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/support-matrix.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase117-parity-uat-release-boundary.ts",
  "scripts/check-phase117-parity-uat-release-boundary.test.ts",
] as const;

const CLAIM_FILES = new Set<TargetFile>([
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/support-matrix.md",
]);

const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/protocol.h",
  "packages/bitcoin-knots/src/blockencodings.h",
  "packages/bitcoin-knots/src/blockencodings.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/net_processing.h",
  "packages/bitcoin-knots/src/net.h",
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/node/blockstorage.cpp",
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
  "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
] as const;

const REQUIRED_BREADCRUMB_GROUPS = [
  "network-block-serving-activation-boundary",
  "node-network-block-serving-adapter",
  "codec-bip152-compact-block",
  "network-compact-relay-peer-state",
  "network-compact-block-reconstruction",
  "network-compact-block-download",
  "node-network-block-relay-evidence-adapter",
  "node-status-contract",
  "node-observability-contracts",
  "rpc-surface",
  "cli-operator-onboarding-contracts",
  "cli-operator-dashboard-contracts",
  "cli-operator-support-bundles",
] as const;

const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -regtest openbitcoinnetworkstatus",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -regtest openbitcoinnetworkstatus",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
  PHASE117_TEST,
  PHASE117_CHECK,
  "bash scripts/verify.sh",
] as const;

const DANGEROUS_CLAIMS = [
  "package relay",
  "bip37",
  "bloom-filter serving",
  "bloom filter serving",
  "compact-filter serving",
  "compact filter serving",
  "public block serving by default",
  "public serving by default",
  "public relay defaults",
  "archive-node",
  "archive node",
  "production-scale historical serving",
  "public-network ci",
  "production service operation",
  "production deployment",
  "production full-node readiness",
  "production-funds wallet",
] as const;

const SCOPED_CAPABILITIES = ["block serving", "block-serving", "compact block relay", "compact-block relay"] as const;
const POSITIVE_PATTERNS = [
  /\bsupports\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bproves?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis ready\b/,
] as const;
const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "is not",
  "are not",
  "not a ",
  "must not",
  "not required",
  "not supported",
  "without claiming",
  "without making",
  "without turning",
  "outside",
  "out of scope",
  "deferred",
  "future",
  "remain deferred",
  "remains deferred",
  "no claim",
  "optional uat",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type TextCorpus = Map<TargetFile, string>;
type SurfaceId = keyof typeof REQUIREMENTS_BY_SURFACE;
type ParitySurface = {
  id?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { surfaces?: unknown; checklist?: { surfaces?: unknown } };
type TraceabilityStage = "implementation" | "gap-closure" | "post-audit-gap-planning";

export function checkPhase117ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE117_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  const maybeIndex = parseParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  if (maybeIndex) checkSurfaceOwnership(maybeIndex, failures);
  checkRequirementTraceability(texts.get(".planning/REQUIREMENTS.md") ?? "", failures);
  if (maybeIndex) checkRequiredEvidence(maybeIndex, texts, failures);
  checkClaims(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): TextCorpus {
  const texts = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    const sourceFile =
      file === ".planning/REQUIREMENTS.md" ? v21RequirementsSource(repoRoot) : file;
    const absolutePath = path.join(repoRoot, sourceFile);
    if (!existsSync(absolutePath)) {
      failures.push(`missing target file ${file}`);
      texts.set(file, "");
      continue;
    }
    texts.set(file, readFileSync(absolutePath, "utf8"));
  }
  return texts;
}

function v21RequirementsSource(repoRoot: string): string {
  const livePath = path.join(repoRoot, ".planning/REQUIREMENTS.md");
  if (existsSync(livePath)) {
    const liveRequirements = readFileSync(livePath, "utf8");
    if (liveRequirements.includes("**Milestone:** v2.1 ")) {
      return ".planning/REQUIREMENTS.md";
    }
  }

  return existsSync(path.join(repoRoot, ARCHIVED_V21_REQUIREMENTS))
    ? ARCHIVED_V21_REQUIREMENTS
    : ".planning/REQUIREMENTS.md";
}

function parseParityIndex(raw: string, failures: string[]): ParityIndex | null {
  try {
    return JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`invalid parity index JSON: ${String(error)}`);
    return null;
  }
}

function checkSurfaceOwnership(index: ParityIndex, failures: string[]): void {
  const topSurfaces = asSurfaceArray(index.surfaces);
  const checklistSurfaces = asSurfaceArray(index.checklist?.surfaces);
  const v21Checklist = checklistSurfaces.filter((surface) =>
    typeof surface.id === "string" ? surface.id.startsWith("v2-1-") : false,
  );

  for (const [surfaceId, expectedRequirements] of Object.entries(REQUIREMENTS_BY_SURFACE) as Array<
    [SurfaceId, readonly string[]]
  >) {
    const topMatches = topSurfaces.filter((surface) => surface.name === surfaceId);
    const checklistMatches = v21Checklist.filter((surface) => surface.id === surfaceId);
    const top = topMatches[0];
    const checklist = checklistMatches[0];
    if (topMatches.length !== 1 || checklistMatches.length !== 1) {
      failures.push(`v2.1 surface ${surfaceId} must have exactly one top-level and checklist entry`);
    }
    if (!top || !checklist) failures.push(`missing v2.1 surface ${surfaceId}`);
    if (top?.status !== "done") failures.push(`top-level v2.1 surface ${surfaceId} must be done`);
    if (checklist?.status !== "done") failures.push(`checklist v2.1 surface ${surfaceId} must be done`);
    if (checklist && !sameMembers(asStringArray(checklist.requirements), expectedRequirements)) {
      failures.push(`v2.1 surface ${surfaceId} has incorrect requirement ownership`);
    }
  }

  for (const requirement of allRequirements()) {
    const owners = v21Checklist.filter((surface) =>
      asStringArray(surface.requirements).includes(requirement),
    );
    if (owners.length !== 1) {
      failures.push(`${requirement} must have exactly one parity surface owner`);
    }
  }
}

function checkRequirementTraceability(raw: string, failures: string[]): void {
  const stage = traceabilityStage(raw);
  for (const requirement of allRequirements()) {
    const phase = expectedPhase(requirement, stage);
    const needle = `| ${requirement} | Phase ${phase} |`;
    if (countOccurrences(raw, needle) !== 1) {
      failures.push(`requirement traceability: ${requirement} must map to Phase ${phase} exactly once`);
    }
  }
}

function checkRequiredEvidence(index: ParityIndex, texts: TextCorpus, failures: string[]): void {
  const v21Surfaces = asSurfaceArray(index.checklist?.surfaces).filter((surface) =>
    typeof surface.id === "string" ? surface.id.startsWith("v2-1-") : false,
  );
  const indexAnchors = new Set(
    v21Surfaces.flatMap((surface) => [
      ...asStringArray(surface.upstream?.sources),
      ...asStringArray(surface.upstream?.tests),
    ]),
  );
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!indexAnchors.has(anchor)) failures.push(`missing Phase 117 parity-index Knots anchor ${anchor}`);
  }
  const breadcrumbs = texts.get("docs/parity/source-breadcrumbs.json") ?? "";
  const breadcrumbAnchors = checkBreadcrumbGroups(breadcrumbs, failures);
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!breadcrumbAnchors.has(anchor)) failures.push(`missing Phase 117 breadcrumb Knots anchor ${anchor}`);
  }
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 117 runtime guide command ${command}`);
    }
  }
}

function checkBreadcrumbGroups(raw: string, failures: string[]): Set<string> {
  try {
    const parsed = JSON.parse(raw) as {
      groups?: Array<{ breadcrumbs?: unknown; files?: unknown; label?: unknown }>;
    };
    const groups = parsed.groups ?? [];
    const labels = new Set(groups.map((group) => group.label).filter((label): label is string => typeof label === "string"));
    const anchors = new Set<string>();
    for (const group of REQUIRED_BREADCRUMB_GROUPS) {
      if (!labels.has(group)) failures.push(`missing breadcrumb group ${group}`);
      const matches = groups.filter((candidate) => candidate.label === group);
      if (matches.length !== 1) failures.push(`breadcrumb group ${group} must appear exactly once`);
      for (const match of matches) {
        if (asStringArray(match.files).length === 0) failures.push(`breadcrumb group ${group} must name source files`);
        for (const anchor of asStringArray(match.breadcrumbs)) anchors.add(anchor);
      }
    }
    return anchors;
  } catch (error) {
    failures.push(`invalid source breadcrumbs JSON: ${String(error)}`);
    return new Set();
  }
}

function checkClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts) {
    if (!CLAIM_FILES.has(file)) continue;
    for (const paragraph of markdownParagraphs(text)) {
      const tableNoClaim = tableRowHasNoClaimStatus(paragraph.text);
      for (const clause of claimClauses(paragraph.text)) {
        const lower = clause.toLowerCase();
        for (const segment of dangerousClaimSegments(lower)) {
          if (!hasPositiveClaim(segment) || hasNoClaimMarker(segment) || tableNoClaim) continue;
          for (const topic of DANGEROUS_CLAIMS) {
            if (segment.includes(topic)) {
              failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 117 claim: ${topic}`);
            }
          }
        }
        if (!hasPositiveClaim(lower) || hasNoClaimMarker(lower)) continue;
        const maybeCapability = SCOPED_CAPABILITIES.find((topic) => lower.includes(topic));
        if (maybeCapability && !isScopedCapabilityClaim(lower)) {
          failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 117 claim: ${maybeCapability}`);
        }
      }
    }
  }
}

function allRequirements(): string[] {
  return Object.values(REQUIREMENTS_BY_SURFACE).flatMap((requirements) => [...requirements]);
}

function traceabilityStage(raw: string): TraceabilityStage {
  if (/\|\s*[A-Z]+-\d+\s*\|\s*Phase\s+12[789]\s*\|/.test(raw)) {
    return "post-audit-gap-planning";
  }
  if (/\|\s*[A-Z]+-\d+\s*\|\s*Phase\s+12[56]\s*\|/.test(raw)) {
    return "gap-closure";
  }
  return "implementation";
}

function expectedPhase(requirement: string, stage: TraceabilityStage): string {
  if (stage === "post-audit-gap-planning") {
    if (["BSRV-03", "BSRV-04", "OBS-02", "OBS-04"].includes(requirement)) return "127";
    if (["CMP-04", "CMP-05", "OBS-03"].includes(requirement)) return "128";
    if (["OBS-01", "BOUND-02"].includes(requirement)) return "129";
  }
  if (stage !== "implementation") {
    if (["RCN-04", "RCN-05", "RCN-06"].includes(requirement)) return "125";
    if (["RCN-02", "RCN-03", "GOV-04", "BOUND-01"].includes(requirement)) {
      return "126";
    }
    if (stage === "gap-closure" && requirement === "CMP-05") return "126";
  }
  // Parity surfaces retain the original implementation roots while traceability
  // moves to the exact active gap-closure owner only during that lifecycle stage.
  if (requirement === "CMP-05") return "118";
  if (["RCN-02", "RCN-03", "GOV-04"].includes(requirement)) return "119";
  if (["RCN-07", "GOV-02", "GOV-03"].includes(requirement)) return "120";
  if (requirement === "OBS-03") return "121";
  if (requirement.startsWith("BSRV")) return requirement === "BSRV-04" ? "111" : "110";
  if (["CMP-01", "CMP-02", "CMP-03", "RCN-01"].includes(requirement)) return "112";
  if (requirement.startsWith("CMP")) return "113";
  if (requirement.startsWith("RCN")) return "115";
  if (requirement.startsWith("GOV")) return "111";
  if (requirement.startsWith("OBS")) return "116";
  return "117";
}

function asSurfaceArray(value: unknown): ParitySurface[] {
  return Array.isArray(value) ? value.filter((item): item is ParitySurface => typeof item === "object" && item !== null) : [];
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function sameMembers(actual: string[], expected: readonly string[]): boolean {
  return actual.length === expected.length && expected.every((value) => actual.includes(value));
}

function countOccurrences(text: string, needle: string): number {
  return needle.length === 0 ? 0 : text.split(needle).length - 1;
}

function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let current: string[] = [];
  let startLine = 1;
  for (const [index, line] of text.split("\n").entries()) {
    if (line.trim().startsWith("|") && line.trim().endsWith("|")) {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      paragraphs.push({ startLine: index + 1, text: line.trim() });
      startLine = index + 2;
      continue;
    }
    if (line.trim() === "") {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      startLine = index + 2;
      continue;
    }
    if (current.length === 0) startLine = index + 1;
    current.push(line);
  }
  if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
  return paragraphs;
}

function claimClauses(paragraph: string): string[] {
  if (paragraph.startsWith("|") && paragraph.endsWith("|")) {
    return paragraph
      .slice(1, -1)
      .split("|")
      .map((value) => value.trim())
      .filter((value) => value !== "");
  }
  return paragraph.split(/(?<=[.!?])\s+|\s+\|\s+/).filter((value) => value.trim() !== "");
}

function dangerousClaimSegments(clause: string): string[] {
  return clause
    .split(/\s*;\s*|,?\s+while\s+|,?\s+but\s+|\s+whereas\s+|\s+and\s+(?=(?:open bitcoin\s+)?(?:package relay|bip37|bloom-filter serving|bloom filter serving|compact-filter serving|compact filter serving|public|archive|production))/)
    .map((value) => value.trim())
    .filter((value) => value !== "");
}

function tableRowHasNoClaimStatus(paragraph: string): boolean {
  if (!paragraph.startsWith("|") || !paragraph.endsWith("|")) return false;
  return paragraph
    .slice(1, -1)
    .split("|")
    .map((value) => value.trim().toLowerCase().replaceAll("`", ""))
    .some((value) => ["deferred", "unsupported", "not allowed", "not allowed yet"].includes(value));
}

function hasPositiveClaim(text: string): boolean {
  return POSITIVE_PATTERNS.some((pattern) => pattern.test(text));
}

function hasNoClaimMarker(text: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => text.includes(marker));
}

function isScopedCapabilityClaim(text: string): boolean {
  return text.includes("bounded") && (text.includes("default-off") || text.includes("explicit") || text.includes("opt-in"));
}

if (import.meta.main) {
  const failures = checkPhase117ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 117 parity UAT release boundary check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 117 parity UAT release boundary validated.");
}

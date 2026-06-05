#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";

const SURFACE_ID = "v1-4-operator-evidence-release-boundaries";
const REQUIRED_REQUIREMENTS = [
  "OBS-01",
  "OBS-02",
  "OBS-03",
  "SEC-01",
  "SEC-02",
  "SEC-03",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/threat-model-v1.4.md",
  "docs/parity/release-readiness.md",
  "docs/operator/runtime-guide.md",
  "scripts/run-live-mainnet-smoke.ts",
  "scripts/test-run-live-mainnet-smoke.sh",
  "scripts/verify.sh",
] as const;
const DEFERRED_SURFACE_TEXT = [
  "inbound serving",
  "transaction relay",
  "production-funds wallet",
  "migration apply mode",
  "packaging",
  "hosted dashboard",
  "GUI",
  "Windows service",
  "unattended production-node",
] as const;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: unknown;
  checklist?: {
    surfaces?: unknown;
  };
};

function repoRoot(): string {
  return execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function readText(repoRootPath: string, relativePath: string): string {
  return readFileSync(path.join(repoRootPath, relativePath), "utf8");
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain: ${needle}`);
  }
}

function requireArrayIncludes(value: unknown, label: string, required: string): void {
  if (!Array.isArray(value)) {
    throw new Error(`${label} must be an array`);
  }
  if (!value.includes(required)) {
    throw new Error(`${label} missing required value: ${required}`);
  }
}

function parseParityIndex(repoRootPath: string): ParityIndex {
  return JSON.parse(readText(repoRootPath, "docs/parity/index.json")) as ParityIndex;
}

function checklistSurfaces(index: ParityIndex): ChecklistSurface[] {
  const maybeSurfaces = index.checklist?.surfaces;
  if (!Array.isArray(maybeSurfaces)) {
    throw new Error("docs/parity/index.json checklist.surfaces must be an array");
  }

  return maybeSurfaces as ChecklistSurface[];
}

function requireSurface(index: ParityIndex): ChecklistSurface {
  const matchingSurfaces = checklistSurfaces(index).filter(
    (surface) => surface.id === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    throw new Error(
      `expected exactly one checklist surface with id ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
  }

  const [surface] = matchingSurfaces;
  if (surface.status !== "done") {
    throw new Error(`${SURFACE_ID} status must be done`);
  }

  return surface;
}

function verifyParityIndex(index: ParityIndex): void {
  const surface = requireSurface(index);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireArrayIncludes(surface.requirements, `${SURFACE_ID}.requirements`, requirement);
  }
  for (const evidencePath of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `${SURFACE_ID}.evidence`, evidencePath);
  }

  const auditText = JSON.stringify(index.audit ?? {});
  for (const requiredText of [
    "v1_4_threat_model",
    "v1_4_release_boundaries",
    "threat-model-v1.4.md",
    "release-readiness.md",
    ...REQUIRED_REQUIREMENTS,
  ]) {
    requireContains(auditText, requiredText, "docs/parity/index.json audit");
  }
}

function verifyReadme(repoRootPath: string): void {
  const readme = readText(repoRootPath, "docs/parity/README.md");

  requireContains(readme, "threat-model-v1.4.md", "docs/parity/README.md");
  requireContains(readme, "threat-model-v1.3.md", "docs/parity/README.md");
}

function verifyRuntimeGuide(repoRootPath: string): void {
  const runtimeGuide = readText(repoRootPath, "docs/operator/runtime-guide.md");

  for (const requiredText of [
    "bash scripts/verify.sh",
    "bash scripts/test-run-live-mainnet-smoke.sh",
    "bun run scripts/run-live-mainnet-smoke.ts",
    "--manual-peer=HOST:8333",
    "--restart-after-progress",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "result.firstHeaderProgress",
    "result.firstBlockProgress",
    "result.restartResumeEvidence",
    "support-evidence.json",
    "support-evidence.md",
  ]) {
    requireContains(runtimeGuide, requiredText, "docs/operator/runtime-guide.md");
  }
}

function verifyThreatModel(repoRootPath: string): void {
  const threatModel = readText(repoRootPath, "docs/parity/threat-model-v1.4.md");

  for (const requiredText of [
    "STRIDE Threat Register",
    "ASVS L1 Mapping",
    "OWASP ASVS v5.0.0",
    "V14-TM-01",
    "V14-TM-02",
    "V14-TM-03",
    "V14-TM-04",
    "V14-TM-05",
    "V14-TM-06",
    "V14-TM-07",
    "V14-TM-08",
  ]) {
    requireContains(threatModel, requiredText, "docs/parity/threat-model-v1.4.md");
  }
}

function verifyDeferredSurfaceDocs(repoRootPath: string): void {
  const releaseReadiness = readText(repoRootPath, "docs/parity/release-readiness.md");
  requireContains(
    releaseReadiness,
    "scripts/check-v1.4-release-boundaries.ts",
    "docs/parity/release-readiness.md",
  );

  for (const relativePath of [
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/catalog/p2p.md",
  ]) {
    const text = readText(repoRootPath, relativePath);
    for (const requiredText of DEFERRED_SURFACE_TEXT) {
      requireContains(text, requiredText, relativePath);
    }
  }
}

function verifyChecklist(repoRootPath: string): void {
  const checklist = readText(repoRootPath, "docs/parity/checklist.md");

  requireContains(checklist, SURFACE_ID, "docs/parity/checklist.md");
}

function verifyVerifyScript(repoRootPath: string): void {
  const verifyScript = readText(repoRootPath, "scripts/verify.sh");

  requireContains(
    verifyScript,
    "bun run scripts/check-v1.3-release-boundaries.ts",
    "scripts/verify.sh",
  );
  requireContains(
    verifyScript,
    "bun run scripts/check-v1.4-release-boundaries.ts",
    "scripts/verify.sh",
  );
  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh");
}

function main(): void {
  const repoRootPath = repoRoot();
  const index = parseParityIndex(repoRootPath);

  verifyParityIndex(index);
  verifyChecklist(repoRootPath);
  verifyReadme(repoRootPath);
  verifyRuntimeGuide(repoRootPath);
  verifyThreatModel(repoRootPath);
  verifyDeferredSurfaceDocs(repoRootPath);
  verifyVerifyScript(repoRootPath);

  console.log("validated v1.4 release boundary parity roots");
}

main();

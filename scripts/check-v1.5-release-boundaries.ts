#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-5-unattended-operation-release-boundaries";
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03", "REL-04"] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/threat-model-v1.5.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/p2p.md",
  "docs/operator/runtime-guide.md",
  "scripts/check-v1.5-release-boundaries.ts",
  "scripts/verify.sh",
] as const;
const DEFERRED_SURFACE_TEXT = [
  "production-node",
  "inbound serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet",
  "migration apply mode",
  "packaging",
  "hosted dashboard",
  "GUI",
  "Windows service",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl --user",
  "launchctl",
] as const;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: unknown;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
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

function requireAllContains(text: string, needles: readonly string[], label: string): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function parseParityIndex(): ParityIndex {
  return JSON.parse(readText("docs/parity/index.json")) as ParityIndex;
}

function checklistSurfaces(index: ParityIndex): ChecklistSurface[] {
  const maybeSurfaces = index.checklist?.surfaces;
  if (!Array.isArray(maybeSurfaces)) {
    throw new Error("docs/parity/index.json checklist.surfaces must be an array");
  }

  return maybeSurfaces as ChecklistSurface[];
}

function requireChecklistSurface(index: ParityIndex): ChecklistSurface {
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

function requireTopLevelSurface(index: ParityIndex): void {
  if (!Array.isArray(index.surfaces)) {
    throw new Error("docs/parity/index.json surfaces must be an array");
  }

  const matchingSurfaces = (index.surfaces as ParitySurface[]).filter(
    (surface) => surface.name === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    throw new Error(
      `expected exactly one top-level surface with name ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
  }

  const [surface] = matchingSurfaces;
  if (surface.status !== "done") {
    throw new Error(`${SURFACE_ID} top-level status must be done`);
  }
}

function verifyParityIndex(index: ParityIndex): void {
  requireTopLevelSurface(index);

  const surface = requireChecklistSurface(index);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireArrayIncludes(surface.requirements, `${SURFACE_ID}.requirements`, requirement);
  }
  for (const evidencePath of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `${SURFACE_ID}.evidence`, evidencePath);
  }

  const auditText = JSON.stringify(index.audit ?? {});
  requireAllContains(
    auditText,
    [
      "v1_5_threat_model",
      "v1_5_release_boundaries",
      "threat-model-v1.5.md",
      "release-readiness.md",
      ...REQUIRED_REQUIREMENTS,
    ],
    "docs/parity/index.json audit",
  );
}

function verifyChecklist(): void {
  const checklist = readText("docs/parity/checklist.md");

  requireAllContains(
    checklist,
    [SURFACE_ID, ...REQUIRED_REQUIREMENTS, "threat-model-v1.5.md"],
    "docs/parity/checklist.md",
  );
}

function verifyReadme(): void {
  const readme = readText("docs/parity/README.md");

  requireAllContains(
    readme,
    [
      "current v1.5 closeout evidence",
      "threat-model-v1.5.md",
      SURFACE_ID,
      "v1.4",
      "v1.3 threat models remain historical evidence",
    ],
    "docs/parity/README.md",
  );
}

function verifyThreatModel(): void {
  const threatModel = readText("docs/parity/threat-model-v1.5.md");

  requireAllContains(
    threatModel,
    [
      "STRIDE Threat Register",
      "ASVS L1 Mapping",
      "OWASP ASVS v5.0.0",
      "Release Boundary Matrix",
      "Requirements Traceability",
      "V15-TM-01",
      "V15-TM-02",
      "V15-TM-03",
      "V15-TM-04",
      "V15-TM-05",
      "V15-TM-06",
      "V15-TM-07",
      "V15-TM-08",
      ...REQUIRED_REQUIREMENTS,
    ],
    "docs/parity/threat-model-v1.5.md",
  );
}

function verifyReleaseReadiness(): void {
  const releaseReadiness = readText("docs/parity/release-readiness.md");

  requireAllContains(
    releaseReadiness,
    [
      "v1.5 Unattended Operation Claim Boundary Matrix",
      "scripts/check-v1.5-release-boundaries.ts",
      "Operator UAT commands",
      "outside deterministic default",
      "public-network CI",
      ...DEFERRED_SURFACE_TEXT,
    ],
    "docs/parity/release-readiness.md",
  );
}

function verifyDeferredSurfaceDocs(): void {
  for (const relativePath of [
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/catalog/p2p.md",
  ]) {
    const text = readText(relativePath);
    requireAllContains(text, DEFERRED_SURFACE_TEXT, relativePath);
    requireContains(text, "v1.5", relativePath);
    requireContains(text, "bash scripts/verify.sh", relativePath);
  }
}

function verifyRuntimeGuide(): void {
  const runtimeGuide = readText("docs/operator/runtime-guide.md");

  requireAllContains(
    runtimeGuide,
    [
      "v1.5 operator review",
      "bun run scripts/check-v1.5-release-boundaries.ts",
      "bash scripts/verify.sh",
      "support-evidence.json",
      "support-evidence.md",
      "compatibility-harness-report.json",
      "compatibility-harness-report.md",
      "Public-network long-run review",
      "real launchd/systemd actions remain opt-in UAT",
    ],
    "docs/operator/runtime-guide.md",
  );
}

function verifyVerifyScript(): void {
  const verifyScript = readText("scripts/verify.sh");

  requireContains(
    verifyScript,
    "bun run scripts/check-v1.5-release-boundaries.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  const index = parseParityIndex();

  verifyParityIndex(index);
  verifyChecklist();
  verifyReadme();
  verifyThreatModel();
  verifyReleaseReadiness();
  verifyDeferredSurfaceDocs();
  verifyRuntimeGuide();
  verifyVerifyScript();

  console.log("validated v1.5 release boundary parity roots");
}

main();

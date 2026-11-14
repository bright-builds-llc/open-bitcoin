import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { SURFACE_ID, REQUIRED_REQUIREMENTS, REQUIRED_EVIDENCE_ROOTS, REQUIRED_KNOTS_ANCHORS, REQUIRED_OUTCOME_LABELS, REQUIRED_ORPHAN_LABELS, REQUIRED_CONSTANTS, REQUIRED_BRIDGE_SYMBOLS, REQUIRED_BREADCRUMB_GROUPS, TextCorpus, ParityIndex, ParitySurface, BreadcrumbGroup } from "./constants.ts";
import { requireExactRequirements, requireArrayIncludes, requireContains } from "./helpers.ts";

export function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 102 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

export function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 102 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 102 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`Phase 102 parity index surface must be done: ${SURFACE_ID}`);
  }
}

export function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("Phase 102 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`Phase 102 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0]!;
  if (surface.status !== "done") {
    failures.push(`Phase 102 checklist surface must be done: ${SURFACE_ID}`);
  }
  requireExactRequirements(surface.requirements, REQUIRED_REQUIREMENTS, "Phase 102 checklist", failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireArrayIncludes(surface.evidence, root, `Phase 102 evidence root missing: ${root}`, failures);
  }
  for (const source of REQUIRED_KNOTS_ANCHORS.slice(0, 9)) {
    requireArrayIncludes(surface.upstream?.sources, source, `Phase 102 Knots source missing: ${source}`, failures);
  }
  for (const test of REQUIRED_KNOTS_ANCHORS.slice(9)) {
    requireArrayIncludes(surface.upstream?.tests, test, `Phase 102 Knots test missing: ${test}`, failures);
  }
}

export function verifyParityDocs(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(docs, requirement, `Phase 102 requirement missing: ${requirement}`, failures);
  }
  requireContains(docs, SURFACE_ID, `Phase 102 surface id missing: ${SURFACE_ID}`, failures);
  for (const root of REQUIRED_EVIDENCE_ROOTS) {
    requireContains(docs, root, `Phase 102 evidence root missing: ${root}`, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(docs, anchor, `Phase 102 Knots anchor missing: ${anchor}`, failures);
  }
  for (const label of [
    ...REQUIRED_OUTCOME_LABELS,
    ...REQUIRED_ORPHAN_LABELS,
    ...REQUIRED_CONSTANTS,
    ...REQUIRED_BRIDGE_SYMBOLS,
  ]) {
    requireContains(docs, label, `Phase 102 docs label missing: ${label}`, failures);
  }
  requireContains(
    docs,
    "Phase 102 does not claim durable mempool persistence",
    "Phase 102 no-claim boundary sentence missing",
    failures,
  );
}

export function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  let parsed: { groups?: unknown };
  try {
    parsed = JSON.parse(text) as { groups?: unknown };
  } catch (error) {
    failures.push(`Phase 102 source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("Phase 102 source breadcrumb groups must be an array");
    return;
  }

  for (const expectedGroup of REQUIRED_BREADCRUMB_GROUPS) {
    const matches = parsed.groups.filter((entry) => {
      const maybeGroup = entry as BreadcrumbGroup;
      return maybeGroup.label === expectedGroup.label;
    }) as BreadcrumbGroup[];
    if (matches.length !== 1) {
      failures.push(`source breadcrumb missing group: ${expectedGroup.label}`);
      continue;
    }

    const group = matches[0]!;
    for (const file of expectedGroup.files) {
      requireArrayIncludes(group.files, file, `source breadcrumb missing file: ${file}`, failures);
    }
    for (const anchor of expectedGroup.anchors) {
      requireArrayIncludes(group.breadcrumbs, anchor, `source breadcrumb missing Knots anchor: ${anchor}`, failures);
    }
  }
}

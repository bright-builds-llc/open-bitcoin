import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { SURFACE_ID, REQUIRED_REQUIREMENTS, REQUIRED_EVIDENCE_ROOTS, REQUIRED_KNOTS_ANCHORS, REQUIRED_RUNTIME_GUIDE_COMMANDS, REQUIRED_DOC_NEEDLES, REQUIRED_GAP_TERMS, TextCorpus, ParityIndex, ParitySurface } from "./constants.ts";
import { requireContains, asStringArray, sameMembers, normalizeWhitespace } from "./helpers.ts";

export function checkParitySurface(texts: TextCorpus, failures: string[]): void {
  const raw = texts.get("docs/parity/index.json") ?? "";
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`docs/parity/index.json is not valid JSON: ${String(error)}`);
    return;
  }

  const topSurfaces = Array.isArray(parsed.surfaces) ? (parsed.surfaces as ParitySurface[]) : [];
  const topMatches = topSurfaces.filter((surface) => surface.name === SURFACE_ID);
  if (topMatches.length !== 1) {
    failures.push(`expected exactly one top-level Phase 107 surface ${SURFACE_ID}`);
  } else if (topMatches[0]?.status !== "done") {
    failures.push(`${SURFACE_ID}: expected top-level status done`);
  }

  const checklistSurfaces = Array.isArray(parsed.checklist?.surfaces)
    ? (parsed.checklist.surfaces as ParitySurface[])
    : [];
  const matches = checklistSurfaces.filter((surface) => surface.id === SURFACE_ID);
  if (matches.length !== 1) {
    failures.push(`expected exactly one parity checklist surface ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`${SURFACE_ID}: expected checklist status done`);
  }
  if (!sameMembers(asStringArray(surface.requirements), [...REQUIRED_REQUIREMENTS])) {
    failures.push(`${SURFACE_ID}: expected requirements ${REQUIRED_REQUIREMENTS.join(", ")}`);
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

export function checkRequiredDocsAndCommands(texts: TextCorpus, failures: string[]): void {
  const corpus = [...texts.values()].join("\n");
  requireContains(corpus, SURFACE_ID, `docs: missing Phase 107 surface id ${SURFACE_ID}`, failures);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(corpus, requirement, `docs: missing Phase 107 requirement ${requirement}`, failures);
  }

  const docsCorpus = [
    texts.get("README.md") ?? "",
    texts.get("docs/architecture/config-precedence.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/catalog/mempool-policy.md") ?? "",
    texts.get("docs/parity/catalog/rpc-cli-config.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
  ].join("\n");
  const normalizedDocsCorpus = normalizeWhitespace(docsCorpus);
  for (const needle of REQUIRED_DOC_NEEDLES) {
    requireContains(
      normalizedDocsCorpus,
      normalizeWhitespace(needle),
      `docs: missing aggregate sanitized Phase 107 wording ${needle}`,
      failures,
    );
  }

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  if (!normalizeWhitespace(runtimeGuide).includes("aggregate, sanitized, and fixed-label only")) {
    failures.push("runtime guide missing aggregate sanitized Phase 107 evidence wording");
  }
  for (const command of REQUIRED_RUNTIME_GUIDE_COMMANDS) {
    if (!runtimeGuide.includes(command)) {
      failures.push(`missing Phase 107 runtime guide command ${command}`);
    }
  }
}

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { SURFACE_ID, REQUIREMENTS, PHASE122_CHECK, PHASE123_TEST, PHASE123_CHECK, PHASE117_TEST, TargetFile, ParityIndex, NamedSurface, ChecklistSurface, BreadcrumbManifest } from "./constants.ts";
import { normalizeWhitespace, requireContains, requireExactCount, requireRepeatedOrder } from "./helpers.ts";

export function verifyPhase121Compatibility(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const checker =
    texts.get("scripts/check-phase121-block-relay-metrics-log-runtime.ts") ?? "";
  for (const needle of [
    "P121 authoritative snapshot",
    "P121 activation omission",
    "P121 same snapshot reuse",
    "P121 obsolete provider wiring",
    "P121 no-claim boundary",
  ]) requireContains(checker, needle, "P123 migrated Phase 121 guarantees", failures);
  const docs = normalizeWhitespace(
    texts.get("docs/architecture/operator-observability.md") ?? "",
  );
  for (const needle of ["runtime-only", "non-serialized", "not the sync projection source"]) {
    requireContains(docs, needle, "P123 operator evidence provenance", failures);
  }
}

export function verifyParity(texts: Map<TargetFile, string>, failures: string[]): void {
  const indexText = texts.get("docs/parity/index.json") ?? "";
  let index: ParityIndex;
  try {
    index = JSON.parse(indexText) as ParityIndex;
  } catch (error) {
    failures.push(`P123 parity index JSON parse failed: ${String(error)}`);
    return;
  }
  const named = (index.surfaces ?? []).filter(
    (entry) => (entry as NamedSurface).name === SURFACE_ID,
  ) as NamedSurface[];
  if (named.length !== 1 || named[0]?.status !== "done") {
    failures.push(`P123 parity index must contain one done surface: ${SURFACE_ID}`);
  }
  const matches = (index.checklist?.surfaces ?? []).filter(
    (entry) => (entry as ChecklistSurface).id === SURFACE_ID,
  ) as ChecklistSurface[];
  if (matches.length !== 1 || matches[0]?.status !== "done") {
    failures.push(`P123 parity checklist must contain one done surface: ${SURFACE_ID}`);
    return;
  }
  const surface = matches[0];
  if (JSON.stringify(surface?.requirements) !== JSON.stringify(REQUIREMENTS)) {
    failures.push("P123 parity requirements must be exactly HARD-02,HARD-03,HARD-04");
  }
  const parityCorpus = [
    JSON.stringify(surface),
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
  ].join("\n");
  for (const needle of [
    "receive-independent idle expiration",
    "successful typed Block writes",
    "runtime-only served evidence",
    "unchanged public status",
    "one authoritative sync-network snapshot",
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
  ]) requireContains(parityCorpus, needle, "P123 exact parity evidence", failures);
  for (const needle of [
    "default-off",
    "package relay",
    "filter serving",
    "public-network CI",
    "production service",
    "production full-node readiness",
    "production-funds",
    "blocking runtime",
    "existing timeout constant",
  ]) requireContains(parityCorpus, needle, "P123 parity no-claim boundary", failures);
}

export function verifyBreadcrumbs(text: string, failures: string[]): void {
  let manifest: BreadcrumbManifest;
  try {
    manifest = JSON.parse(text) as BreadcrumbManifest;
  } catch (error) {
    failures.push(`P123 breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }
  for (const file of [
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  ]) {
    const matches = (manifest.groups ?? []).filter((group) => group.files?.includes(file));
    if (matches.length !== 1) {
      failures.push(`P123 breadcrumb must contain exactly one group for ${file}`);
      continue;
    }
    const breadcrumbs = matches[0]?.breadcrumbs ?? [];
    if (!breadcrumbs.includes("packages/bitcoin-knots/src/net.cpp") ||
        !breadcrumbs.includes("packages/bitcoin-knots/src/net_processing.cpp")) {
      failures.push(`P123 breadcrumb missing Knots runtime anchors for ${file}`);
    }
  }
}

export function verifyVerifierWiring(text: string, failures: string[]): void {
  requireExactCount(text, PHASE123_TEST, 2, "P123 verifier mutation command", failures);
  requireExactCount(text, PHASE123_CHECK, 2, "P123 verifier live checker command", failures);
  requireExactCount(
    text,
    'run_step "test Phase 123 runtime timing and evidence integrity checker"',
    1,
    "P123 verifier test label",
    failures,
  );
  requireExactCount(
    text,
    'run_step "check Phase 123 runtime timing and evidence integrity"',
    1,
    "P123 verifier checker label",
    failures,
  );
  requireRepeatedOrder(
    text,
    [PHASE122_CHECK, PHASE123_TEST, PHASE123_CHECK, PHASE117_TEST],
    2,
    "P123 verifier Phase 122/123/117 order",
    failures,
  );
}

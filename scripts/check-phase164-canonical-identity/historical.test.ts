import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import manifest from "../../packages/testdata/phase164/historical-tooling/manifest.json";
import { historicalToolingView, MANIFEST_FILE, toolingAttribution } from "./historical-tooling.ts";
import { loadSources } from "../check-phase162-integrated-filter-proof.ts";
import { PROVENANCE_DIGEST_FILES, PHASE163, provenanceMetadata } from "../check-phase163-provenance.ts";
import { checkPhase164, loadInputs } from "../check-phase164-canonical-identity.ts";

test("original Phase163 receipt digest qualifies only the preserved historical tooling view", () => {
  // Arrange
  const sources = loadSources();
  const originalReceipt = sources.get(PHASE163 + "163-VERIFICATION.md")!;
  // Act
  const attribution = toolingAttribution(sources, PROVENANCE_DIGEST_FILES);
  // Assert
  expect(attribution.evidence_scope).toBe("historical/source-derived-regression");
  expect(attribution.currentOrdinaryQualification).toBe(false);
  expect(attribution.rawCurrentDigest).not.toBe(attribution.historicalDigest);
  expect(attribution.historicalDigest).toBe(provenanceMetadata(originalReceipt).source_digest);
  expect(sources.get(PHASE163 + "163-VERIFICATION.md")).toBe(originalReceipt);
});

for (const row of manifest.bindings) {
  test(`missing current tooling input refuses without checkout fallback: ${row.file}`, () => {
    // Arrange
    const sources = loadSources();
    sources.delete(row.file);
    // Act / Assert
    expect(() => historicalToolingView(sources)).toThrow("unreviewed current tooling fingerprint");
  });
  for (const file of [row.file, row.snapshot]) {
    test(`unknown tooling or original snapshot bytes refuse: ${file}`, () => {
      // Arrange
      const sources = loadSources();
      sources.set(file, sources.get(file)! + "\n");
      // Act / Assert
      expect(() => historicalToolingView(sources)).toThrow();
      expect(checkPhase164(sources).some(row => row.rule === "history")).toBe(true);
    });
  }
  test(`missing original snapshot refuses: ${row.snapshot}`, () => {
    // Arrange
    const sources = loadSources();
    sources.delete(row.snapshot);
    // Act / Assert
    expect(() => historicalToolingView(sources)).toThrow("snapshot changed or missing");
  });
}

test("original unmodified tooling is explicitly historical and portable without git", () => {
  // Arrange
  const sources = loadSources();
  for (const row of manifest.bindings) sources.set(row.file, sources.get(row.snapshot)!);
  // Act
  const attribution = toolingAttribution(sources, PROVENANCE_DIGEST_FILES);
  // Assert
  expect(attribution.rawCurrentDigest).toBe(attribution.historicalDigest);
  expect(attribution.currentOrdinaryQualification).toBe(false);
});

for (const file of [MANIFEST_FILE, "scripts/check-phase164-canonical-identity/historical-tooling.ts", "scripts/check-phase164-canonical-identity/fixture-source.ts"]) {
  test(`manifest or adapter cannot silently relabel original evidence: ${file}`, () => {
    // Arrange
    const inputs = loadInputs();
    inputs.set(file, inputs.get(file)! + "\n");
    // Act
    const findings = checkPhase164(inputs);
    // Assert
    expect(findings.some(row => row.rule === "history")).toBe(true);
  });
}

test("specified corpus root cannot borrow missing historical snapshots from the checkout", () => {
  // Arrange
  const root = mkdtempSync(join(tmpdir(), "open-bitcoin-phase164-corpus-"));
  const inputs = loadInputs();
  try {
    for (const [file, source] of inputs) {
      if (file === manifest.bindings[0].snapshot) continue;
      const destination = join(root, file);
      mkdirSync(dirname(destination), { recursive: true });
      writeFileSync(destination, source);
    }
    // Act / Assert
    expect(() => loadInputs(root)).toThrow("ENOENT");
  } finally { rmSync(root, { recursive: true }); }
});

import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildTrace, checkCurrentTrace, type TraceIndex } from "./check-v2-6-traceability.ts";

const root = resolve(import.meta.dir, "..");
const read = (file: string) => readFileSync(resolve(root, file));
const index: TraceIndex = JSON.parse(read("docs/parity/index.json").toString());
const requirements = read("docs/parity/requirements.md").toString();

test("current trace binds claims, actual results and the preserved archive", () => {
  // Act
  const trace = checkCurrentTrace(root);
  // Assert
  expect(trace.claims.filter(claim => claim.id !== "CPAR-02" && claim.status === "done")).toHaveLength(29);
  expect(trace.differential).toHaveLength(7);
  expect(trace.reference_ambiguities[0].receipt).toBe("docs/parity/receipts/crec-01-regtest.json");
  expect(trace.v2_5_archive.files).toHaveLength(10);
});

test("a current difference cannot point to an unfinished owner", () => {
  // Arrange
  const changed = structuredClone(index);
  const row = changed.deviations.find(deviation => deviation.id === "v2-6-bip30-lossless-overwrite-undo");
  if (!row) throw new Error("expected BIP30 difference");
  const closure = changed.checklist.surfaces.find(owner => owner.id === "v2-6-cpar-02");
  if (!closure) throw new Error("expected CPAR-02 owner");
  closure.status = "planned";
  row.surfaces = ["v2-6-cpar-02"];
  // Act and assert
  expect(() => buildTrace(changed, requirements, read)).toThrow("Difference needs completed owners");
});

test("the trace requires each current intentional difference", () => {
  // Arrange
  const changed = structuredClone(index);
  changed.deviations = changed.deviations.filter(deviation => deviation.id !== "v2-6-manual-prune-keep-window-refusal");
  // Act and assert
  expect(() => buildTrace(changed, requirements, read)).toThrow("Missing v2.6 differences");
});

test("a changed archived receipt loses its original attribution", () => {
  // Arrange
  const readChanged = (file: string) => file.endsWith("162-NATIVE-EVIDENCE.md") ? Buffer.from("changed receipt") : read(file);
  // Act and assert
  expect(() => buildTrace(index, requirements, readChanged)).toThrow("v2.5 archive bytes changed");
});

test("a receipt with a different binary loses its observed-result link", () => {
  // Arrange
  const readChanged = (file: string) => {
    if (file !== "docs/parity/receipts/cdif-01-regtest.json") return read(file);
    const original = JSON.parse(read(file).toString());
    return Buffer.from(JSON.stringify({ ...original, binary_sha256: "0".repeat(64) }));
  };
  // Act and assert
  expect(() => buildTrace(index, requirements, readChanged)).toThrow("Invalid observed result: v2-6-cdif-01");
});

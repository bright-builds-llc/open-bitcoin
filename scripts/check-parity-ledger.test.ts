import { expect, test } from "bun:test";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { checkCurrentParityLedger, checkParityLedger, type LedgerInputs } from "./check-parity-ledger.ts";

const ROOT = resolve(import.meta.dir, "..");
const COMMIT = "a9aee730466ac67d35a3c03ee24676be5e045878";
const requirementsText = readFileSync(join(ROOT, "docs/parity/requirements.md"), "utf8");
const currentIndex = JSON.parse(readFileSync(join(ROOT, "docs/parity/index.json"), "utf8"));

type Surface = {
  id: string;
  status: string;
  requirements: string[];
  evidence: string[];
  evidence_classes: string[];
  upstream?: { sources: string[] };
  differential_receipt?: string;
  knots_binary_sha256?: string;
};
type Index = { baseline: string; checklist: { surfaces: Surface[] } };

function fixture(): LedgerInputs {
  return {
    requirementsText,
    index: structuredClone(currentIndex),
    baseline: { gitlinkCommit: COMMIT, checkoutCommit: COMMIT },
    isRegularFile: (relativePath) => {
      const full = join(ROOT, relativePath);
      return existsSync(full) && lstatSync(full).isFile();
    },
    readTextFile: (relativePath) => readFileSync(join(ROOT, relativePath), "utf8"),
  };
}

function surface(inputs: LedgerInputs, id: string): Surface {
  const maybeSurface = (inputs.index as Index).checklist.surfaces.find((row) => row.id === id);
  if (!maybeSurface) throw new Error(`missing fixture surface ${id}`);
  return maybeSurface;
}

test("current checkout has one v2.6 parity surface per approved requirement", () => {
  // Arrange
  const expected = [];
  // Act
  const failures = checkCurrentParityLedger(ROOT);
  // Assert
  expect(failures).toEqual(expected);
});

test("missing requirement owner is rejected", () => {
  // Arrange
  const inputs = fixture();
  (inputs.index as Index).checklist.surfaces = (inputs.index as Index).checklist.surfaces
    .filter((row) => row.id !== "v2-6-cdif-01");
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("requirements: CDIF-01 must have exactly one v2.6 surface; found 0");
});

test("duplicate requirement owner is rejected", () => {
  // Arrange
  const inputs = fixture();
  const duplicate = structuredClone(surface(inputs, "v2-6-cdif-01"));
  duplicate.id = "v2-6-second-cdif-01";
  (inputs.index as Index).checklist.surfaces.push(duplicate);
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("requirements: CDIF-01 must have exactly one v2.6 surface; found 2");
});

test("missing evidence file rejects a completed surface", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-candidate-history-validation").evidence[0] = "docs/parity/absent-phase167-evidence.md";
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-candidate-history-validation evidence: missing or non-regular file docs/parity/absent-phase167-evidence.md");
});

test("wrong baseline tag is rejected", () => {
  // Arrange
  const inputs = fixture();
  (inputs.index as Index).baseline = "29.3.knots-unpinned";
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("ledger: baseline must be 29.3.knots20260210");
});

test("wrong Knots gitlink commit is rejected", () => {
  // Arrange
  const inputs = fixture();
  inputs.baseline.gitlinkCommit = "0".repeat(40);
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("Knots gitlink differs from the approved commit");
});

test("wrong checked-out Knots commit is rejected", () => {
  // Arrange
  const inputs = fixture();
  inputs.baseline.checkoutCommit = "0".repeat(40);
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("Knots checkout differs from the approved commit");
});

test("upstream anchor outside pinned Knots is rejected", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-candidate-history-validation").upstream!.sources[0] = "docs/parity/requirements.md";
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-candidate-history-validation: upstream source is outside pinned Knots: docs/parity/requirements.md");
});

test("completed surface without evidence is rejected", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-candidate-history-validation").evidence = [];
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-candidate-history-validation: done surface needs evidence");
});

test("seeded historical evidence cannot be relabeled as daemon differential", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-candidate-history-validation").evidence_classes.push("knots_daemon_differential");
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-candidate-history-validation: actual Knots differential requires a referenced receipt");
});

test("daemon receipt must bind the pinned source commit", () => {
  // Arrange
  const inputs = fixture();
  const candidate = surface(inputs, "v2-6-candidate-history-validation");
  const receipt = "docs/parity/receipts/test-differential.json";
  candidate.evidence_classes.push("knots_daemon_differential");
  candidate.evidence.push(receipt);
  candidate.differential_receipt = receipt;
  candidate.knots_binary_sha256 = "a".repeat(64);
  const regularFile = inputs.isRegularFile;
  const readTextFile = inputs.readTextFile;
  inputs.isRegularFile = (relativePath) => relativePath === receipt || regularFile(relativePath);
  inputs.readTextFile = (relativePath) => relativePath === receipt
    ? JSON.stringify({ source_commit: "0".repeat(40), binary_sha256: "a".repeat(64), status: "passed" })
    : readTextFile(relativePath);
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-candidate-history-validation: actual Knots differential receipt lacks pinned source, binary and passed outcome");
});

test("source-bound daemon receipt can qualify its evidence class", () => {
  // Arrange
  const inputs = fixture();
  const candidate = surface(inputs, "v2-6-candidate-history-validation");
  const receipt = "docs/parity/receipts/test-differential.json";
  const regularFile = inputs.isRegularFile;
  const readTextFile = inputs.readTextFile;
  candidate.evidence_classes.push("knots_daemon_differential");
  candidate.evidence.push(receipt);
  candidate.differential_receipt = receipt;
  candidate.knots_binary_sha256 = "a".repeat(64);
  inputs.isRegularFile = (relativePath) => relativePath === receipt || regularFile(relativePath);
  inputs.readTextFile = (relativePath) => relativePath === receipt
    ? JSON.stringify({ source_commit: COMMIT, binary_sha256: "a".repeat(64), status: "passed" })
    : readTextFile(relativePath);
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toEqual([]);
});

test("planned surface cannot claim evidence prematurely", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-cdif-01").evidence.push("docs/parity/requirements.md");
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-cdif-01: planned surface cannot claim evidence");
});

test("evidence path traversal is rejected before filesystem lookup", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-candidate-history-validation").evidence.push("../outside.md");
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain('v2-6-candidate-history-validation evidence: unsafe or non-normalized path "../outside.md"');
});

const verifySource = () => readFileSync(join(ROOT, "scripts/verify.sh"), "utf8");

test("current verifier no longer executes active GSD traceability", () => {
  // Arrange / Act
  const source = verifySource();
  // Assert
  expect(source).not.toContain("bun test scripts/check-active-milestone-verification-traceability.test.ts");
  expect(source).not.toContain("bun run scripts/check-active-milestone-verification-traceability.ts");
});

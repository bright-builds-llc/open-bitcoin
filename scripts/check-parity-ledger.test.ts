import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { REGTEST_CORPUS } from "./knots-regtest/regtest-case.ts";
import { readRegtestHistory } from "./knots-regtest/regtest-history.ts";
import { normalizeRejection, type observeRejections } from "./knots-regtest/rejections.ts";
import { checkCurrentParityLedger, checkParityLedger, checkQualificationRecord, type LedgerInputs } from "./check-parity-ledger.ts";
import { summarizeKnotsCoins } from "./knots-regtest/state.ts";
import { identifyKnotsOracle } from "./knots-regtest/build.ts";
import { loggedCommand } from "./knots-regtest/process.ts";
import { qualify, QUALIFICATION_FORMAT } from "./knots-regtest/receipt.ts";

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
    baseline: { gitlinkCommit: COMMIT, checkoutCommit: COMMIT, checkoutTree: "643d6c87370510b5193461e37647526369dfe0b3" },
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
  const expected: string[] = [];
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

test.each([
  ["source tree", { source_tree: "0".repeat(40) }, "receipt lacks the pinned tree or daemon build inputs"],
  ["submitted bytes", { block_sha256: "0".repeat(64) }, "receipt does not match the canonical height-one Regtest input"],
  ["Knots process result", { knots: { exit_status: 1 } }, "receipt lacks the expected isolated knots process outcome"],
])("CDIF-01 rejects a passing receipt with a wrong %s", (_field, replacement, expected) => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-01").differential_receipt;
  const readTextFile = inputs.readTextFile;
  inputs.readTextFile = (relativePath) => {
    const text = readTextFile(relativePath);
    return relativePath === receipt ? JSON.stringify({ ...JSON.parse(text), ...replacement }) : text;
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual([`v2-6-cdif-01: ${expected}`]);
});

test("CDIF-02 rejects a tampered halving block with the original header", () => {
  // Arrange
  const inputs = fixture();
  const history = JSON.parse(inputs.readTextFile(REGTEST_CORPUS));
  const row = history.rows[149];
  row.block_hex = row.block_hex.slice(0, -2) + "01";

  // Act / Assert
  expect(() => readRegtestHistory(() => JSON.stringify(history))).toThrow("merkle mismatch");
});

test.each([
  ["accepted count", { accepted_count: 300 }],
  ["accepted tips digest", { accepted_tips_sha256: "0".repeat(64) }],
  ["checkpoint list", { checkpoints: [] }],
])("CDIF-02 rejects a mismatched %s in either process result", (_field, replacement) => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-02").differential_receipt;
  const readTextFile = inputs.readTextFile;
  inputs.readTextFile = (relativePath) => {
    const text = readTextFile(relativePath);
    if (relativePath !== receipt) return text;
    const document = JSON.parse(text);
    return JSON.stringify({ ...document, knots: { ...document.knots, ...replacement },
      open_bitcoin: { ...document.open_bitcoin, ...replacement } });
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual([
    "v2-6-cdif-02: receipt lacks the expected isolated knots process outcome",
    "v2-6-cdif-02: receipt lacks the expected isolated open_bitcoin process outcome",
  ]);
});

test("CDIF-02 rejects monetary evidence missing the maturity spend", () => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-02").differential_receipt;
  const readTextFile = inputs.readTextFile;
  inputs.readTextFile = (relativePath) => {
    const text = readTextFile(relativePath);
    return relativePath === receipt ? JSON.stringify({ ...JSON.parse(text), maturity: null }) : text;
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual(["v2-6-cdif-02: receipt does not match the canonical Regtest monetary evidence"]);
});

test.each([null, "inconclusive", "duplicate", "rejected", { code: -22 }])(
  "CDIF-03 requires a specific consensus reason, rejecting %j", (result) => {
    // Arrange / Act
    const normalize = () => normalizeRejection(result);

    // Assert
    expect(normalize).toThrow("Unexpected submitblock result");
  },
);

type RejectionReceiptFixture = { knots: { runs: Awaited<ReturnType<typeof observeRejections>>[] } };

test.each([
  ["missing case", (result: RejectionReceiptFixture) => result.knots.runs[0].cases.pop()],
  ["different rejection", (result: RejectionReceiptFixture) => {
    result.knots.runs[0].cases[0].submitblock = "bad-cb-amount";
  }],
  ["state mutation", (result: RejectionReceiptFixture) => {
    result.knots.runs[0].cases[0].after.mempool_count = 1;
  }],
])("CDIF-03 refuses a receipt with %s", (_field, mutate) => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-03").differential_receipt;
  const readText = inputs.readTextFile;
  inputs.readTextFile = (path) => {
    const text = readText(path);
    if (path !== receipt) return text;
    const result = JSON.parse(text);
    mutate(result);
    return JSON.stringify(result);
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual(["v2-6-cdif-03: invalid knots rejection evidence for through-201"]);
});

test("CDIF-04 rejects a duplicate outpoint in a Knots ASCII projection", () => {
  // Arrange
  const row = `${"1".repeat(64)},0,1250000000,1,301,51`;

  // Act / Assert
  expect(() => summarizeKnotsCoins(`${row}\n${row}\n`, 301)).toThrow("Duplicate projected outpoint");
});

test("CDIF-04 rejects a UTXO mismatch at the same height and best block", () => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-04").differential_receipt;
  const readText = inputs.readTextFile;
  inputs.readTextFile = (path) => {
    const text = readText(path);
    if (path !== receipt) return text;
    const result = JSON.parse(text);
    result.open_bitcoin.utxos.sha256 = "0".repeat(64);
    return JSON.stringify(result);
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual(["v2-6-cdif-04: receipt lacks the expected isolated open_bitcoin process outcome"]);
});

test.each([
  ["missing oracle", "error", async (dir: string) => {
    identifyKnotsOracle(join(dir, "absent-knots"), dir);
  }],
  ["wrong oracle", "error", async (dir: string) => {
    const executable = join(dir, "wrong-knots");
    writeFileSync(executable, "#!/bin/sh\nprintf '%s\\n' 'not the pinned Knots daemon'\n", { mode: 0o700 });
    identifyKnotsOracle(executable, dir);
  }],
  ["process timeout", "timeout", async (dir: string) => {
    await loggedCommand(dir, ["/bin/sh", "-c", "sleep 10 & wait"], join(dir, "timeout.log"), process.env, 25);
  }],
])("CDIF-05 writes failure evidence for %s", async (_case, outcome, action) => {
  // Arrange
  const dir = mkdtempSync(join(tmpdir(), "cdif05-failure-"));
  try {
    // Act
    const attempt = qualify(dir, async (stage) => {
      stage("negative_control");
      await action(dir);
      return { format: QUALIFICATION_FORMAT, status: "passed" };
    }, () => []);
    await expect(attempt).rejects.toThrow();

    // Assert
    expect(existsSync(join(dir, "receipt.json"))).toBe(false);
    const failure = JSON.parse(readFileSync(join(dir, "failure.json"), "utf8"));
    expect(failure).toMatchObject({ status: "failed", stage: "negative_control", outcome });
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

type QualificationFixture = {
  knots_build: { version: string };
  knots: { runs: { accepted: string[]; exit_status: number }[] };
  open_bitcoin: { runs: { chainwork: string }[] };
  corpus_sha256: string;
};

test.each([
  ["missing accepted case", (record: QualificationFixture) => record.knots.runs[1].accepted.pop()],
  ["wrong oracle", (record: QualificationFixture) => { record.knots_build.version = "not Knots"; }],
  ["wrong corpus digest", (record: QualificationFixture) => { record.corpus_sha256 = "0".repeat(64); }],
  ["failed process", (record: QualificationFixture) => { record.knots.runs[1].exit_status = 137; }],
  ["mismatched state", (record: QualificationFixture) => { record.open_bitcoin.runs[1].chainwork = "0".repeat(64); }],
])("CDIF-05 refuses to publish a passing receipt with %s", async (_case, mutate) => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-cdif-05").differential_receipt;
  if (!receipt) throw new Error("CDIF-05 fixture has no passing receipt");
  const record = JSON.parse(inputs.readTextFile(receipt));
  mutate(record);
  const dir = mkdtempSync(join(tmpdir(), "cdif05-invalid-receipt-"));
  try {
    // Act
    await expect(qualify(dir, async () => record, (result) =>
      checkQualificationRecord(result, inputs))).rejects.toThrow("v2-6-cdif-05");

    // Assert
    expect(existsSync(join(dir, "receipt.json"))).toBe(false);
    const failure = JSON.parse(readFileSync(join(dir, "failure.json"), "utf8"));
    expect(failure).toMatchObject({ status: "failed", stage: "validation", outcome: "error" });
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

type BranchReceiptFixture = {
  open_bitcoin: { runs: { steps: { active: { hash: string } }[] }[] };
  knots: { restart: { ambiguous: { selected: string } } };
};

test.each([
  ["wrong live tie winner", (record: BranchReceiptFixture) => {
    record.open_bitcoin.runs[0].steps[2].active.hash = "0".repeat(64);
  }, "v2-6-crec-01: invalid open_bitcoin branch observations for a-first"],
  ["winner outside the restart tie", (record: BranchReceiptFixture) => {
    record.knots.restart.ambiguous.selected = "0".repeat(64);
  }, "v2-6-crec-01: invalid Knots restart fallback classification"],
])("CREC-01 rejects a receipt with %s", (_case, mutate, diagnostic) => {
  // Arrange
  const inputs = fixture();
  const receipt = surface(inputs, "v2-6-crec-01").differential_receipt;
  const readText = inputs.readTextFile;
  inputs.readTextFile = (path) => {
    const text = readText(path);
    if (path !== receipt) return text;
    const record = JSON.parse(text);
    mutate(record);
    return JSON.stringify(record);
  };

  // Act
  const failures = checkParityLedger(inputs);

  // Assert
  expect(failures).toEqual([diagnostic]);
});

test("planned surface cannot claim evidence prematurely", () => {
  // Arrange
  const inputs = fixture();
  surface(inputs, "v2-6-crec-02").evidence.push("docs/parity/requirements.md");
  // Act
  const failures = checkParityLedger(inputs);
  // Assert
  expect(failures).toContain("v2-6-crec-02: planned surface cannot claim evidence");
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

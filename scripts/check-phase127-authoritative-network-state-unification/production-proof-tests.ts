import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import {
  checkPhase127AuthoritativeNetworkStateUnification,
  PHASE127_TARGET_FILES,
} from "../check-phase127-authoritative-network-state-unification.ts";
import { readSourceCorpus } from "../source-corpus.ts";

const repoRoot = path.resolve(import.meta.dir, "../..");
const integrationFile = "packages/open-bitcoin-rpc/tests/black_box_parity.rs";
const blockPredicate = "matches!(message, WireNetworkMessage::Block(block) if block_hash(&block.header) == expected_hash)";
const servedAssertion = "assert!(matches_block(&served, expected_hash));";
const mixedAssertion = "assert!(matches_block(&mixed_block_response, expected_hash));";
const proofDiagnostic = "P127 production proof: integration must cover shared mutation, restart serving, frozen schemas, and redaction";
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) rmSync(root, { force: true, recursive: true });
});

function corpus(): Map<string, string> {
  return new Map(PHASE127_TARGET_FILES.map(file => [file, readSourceCorpus(repoRoot, file)]));
}

function writeCorpus(files: Map<string, string>): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase127-proof-"));
  tempRoots.push(root);
  for (const [file, source] of files) {
    const destination = path.join(root, file);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, source);
  }
  return root;
}

test("phase165 served-block current owning helper passes", () => {
  // Arrange
  const root = writeCorpus(corpus());
  // Act
  const failures = checkPhase127AuthoritativeNetworkStateUnification(root);
  // Assert
  expect(failures).toEqual([]);
});

for (const [label, needle, replacement] of [
  ["wrong wire variant", blockPredicate, blockPredicate.replace("::Block(block)", "::Tx(block)")],
  ["wrong header hash", blockPredicate, blockPredicate.replace("== expected_hash", "!= expected_hash")],
  ["unconditional helper", blockPredicate, "true"],
  ["comment helper decoy", blockPredicate, `/* ${blockPredicate} */ true`],
  ["literal helper decoy", blockPredicate, `let decoy = r#"${blockPredicate}"#; true`],
  ["missing helper", "pub(super) fn matches_block(", "pub(super) fn disconnected_block("],
  ["missing served assertion", servedAssertion, ""],
  ["missing mixed assertion", mixedAssertion, ""],
  ["comment served assertion", servedAssertion, `/* ${servedAssertion} */`],
  ["literal mixed assertion", mixedAssertion, `let decoy = r#"${mixedAssertion}"#;`],
  ["wrong served hash", servedAssertion, servedAssertion.replace("expected_hash", "other_hash")],
  ["disconnected mixed assertion", mixedAssertion, "assert!(true);"],
] as const) {
  test(`phase165 served-block proof refuses ${label}`, () => {
    // Arrange
    const files = corpus();
    expect(checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files))).toEqual([]);
    const source = files.get(integrationFile) ?? "";
    expect(source.split(needle)).toHaveLength(2);
    let changed = source.replace(needle, replacement);
    if (label === "disconnected mixed assertion") {
      changed += `\nfn disconnected_proof() { ${mixedAssertion} }\n`;
    }
    expect(changed).not.toBe(source);
    files.set(integrationFile, changed);
    // Act
    const failures = checkPhase127AuthoritativeNetworkStateUnification(writeCorpus(files));
    // Assert
    expect(failures).toContain(proofDiagnostic);
  });
}

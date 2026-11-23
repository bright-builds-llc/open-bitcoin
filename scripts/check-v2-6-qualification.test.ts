import { expect, test } from "bun:test";
import { BOUNDED_SCOPE, boundedQualificationScope, qualificationIssue } from "./check-v2-6-qualification.ts";
import { digestQualificationInputs, QUALIFICATION_INPUT_FORMAT } from "./knots-regtest/qualification-inputs.ts";

const files = [
  "packages/open-bitcoin-consensus/src/lib.rs",
  "scripts/run-knots-regtest-differential.ts",
  "packages/testdata/phase167/regtest-history.json",
  "scripts/knots-regtest/build.ts",
] as const;
const entries = files.map(file => [file, "100644", "1".repeat(64)] as const);
const sha256 = digestQualificationInputs(entries);
const source_commit = "a".repeat(40);
const receipt = {
  status: "passed", network: "regtest", open_bitcoin_build: { source_commit },
  qualification_inputs: { format: QUALIFICATION_INPUT_FORMAT, source_commit, sha256 },
};

for (const [index, file] of files.entries()) test(`qualification becomes stale when ${file} changes`, () => {
  // Arrange
  const changed = entries.map((entry, offset) => offset === index ? [entry[0], entry[1], "2".repeat(64)] as const : entry);
  // Act and assert
  expect(qualificationIssue(receipt, sha256)).toBeUndefined();
  expect(qualificationIssue(receipt, digestQualificationInputs(changed))).toBe("qualification inputs changed");
});

test("a launch binding must name the actual product build commit", () => {
  // Arrange
  const differentBuild = { ...receipt, open_bitcoin_build: { source_commit: "b".repeat(40) } };
  // Act and assert
  expect(qualificationIssue(differentBuild, sha256)).toBe("invalid run binding");
});

test("legacy receipts remain recorded but need renewed qualification", () => {
  const legacy = { status: "passed", network: "regtest", open_bitcoin_build: { source_commit } };
  expect(qualificationIssue(legacy, sha256)).toBe("missing launch input binding");
});

for (const claim of ["whole_chain", "public_default", "production_ready"] as const) test(`rejects an unsupported ${claim} claim`, () => {
  expect(boundedQualificationScope({ ...BOUNDED_SCOPE, [claim]: true })).toBe(false);
  expect(boundedQualificationScope(BOUNDED_SCOPE)).toBe(true);
});

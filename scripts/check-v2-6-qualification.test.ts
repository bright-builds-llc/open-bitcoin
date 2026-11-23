import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { BOUNDED_SCOPE, boundedQualificationScope, qualificationIssue } from "./check-v2-6-qualification.ts";
import { beginQualification, currentQualificationInputs, digestQualificationInputs, QUALIFICATION_INPUT_FORMAT } from "./knots-regtest/qualification-inputs.ts";

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

test("checkout binding sees real input edits and ignores documentation and receipt publication", () => {
  // Arrange
  const root = mkdtempSync(join(tmpdir(), "open-bitcoin-qualification-fixture-"));
  const git = (cwd: string, args: string[]) => execFileSync("git", ["-C", cwd,
    "-c", "user.name=Qualification Fixture", "-c", "user.email=fixture@example.invalid",
    "-c", "commit.gpgsign=false", ...args], { encoding: "utf8" }).trim();
  const write = (file: string, contents: string) => {
    const path = join(root, file);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, contents);
  };
  try {
    git(root, ["init", "--quiet"]);
    write("packages/bitcoin-knots/reference", "pinned oracle\n");
    const knots = join(root, "packages/bitcoin-knots");
    git(knots, ["init", "--quiet"]);
    git(knots, ["add", "reference"]);
    git(knots, ["commit", "--quiet", "-m", "Pin oracle fixture"]);
    git(root, ["update-index", "--add", "--cacheinfo", `160000,${git(knots, ["rev-parse", "HEAD"])},packages/bitcoin-knots`]);
    for (const file of files) write(file, "original input\n");
    git(root, ["add", "--", ...files]);
    git(root, ["commit", "--quiet", "-m", "Commit qualification inputs"]);
    const commit = git(root, ["rev-parse", "HEAD"]);
    const seal = beginQualification(root);
    const record = { open_bitcoin_build: { source_commit: commit } };
    const binding = seal(record).qualification_inputs;

    // Act and assert
    write("docs/parity/receipts/cdif-01-regtest.json", "new receipt\n");
    write("README.md", "new prose\n");
    expect(seal(record).qualification_inputs).toEqual(binding);
    for (const file of files) {
      write(file, "changed input\n");
      expect(currentQualificationInputs(root)).not.toBe(binding.sha256);
      expect(() => seal(record)).toThrow("qualification inputs");
      write(file, "original input\n");
    }
    write("packages/open-bitcoin-consensus/src/new_rule.rs", "new input\n");
    expect(currentQualificationInputs(root)).not.toBe(binding.sha256);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

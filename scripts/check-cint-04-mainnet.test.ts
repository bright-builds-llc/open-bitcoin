import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildMainnetAudit, checkMainnetReceipt, RECEIPT } from "./check-cint-04-mainnet.ts";

const root = resolve(import.meta.dir, "..");
const audit = buildMainnetAudit(root);

test("tracked audit matches all six finite Mainnet contexts", () => {
  // Arrange
  const receipt = readFileSync(resolve(root, RECEIPT), "utf8");
  // Act and assert
  checkMainnetReceipt(receipt, audit);
  expect(audit.cases.map(row => row.height)).toEqual([91722, 91812, 91842, 91880, 170060, 692261]);
  expect(audit.cases.filter(row => row.overwrite_originals.length).map(row => row.bip30_scan_expected)).toEqual([false, false]);
  expect(audit.cases.at(-1)?.activation.script_flags).not.toContain("TAPROOT");
});

test("an altered activation claim disagrees with the checked receipt", () => {
  // Arrange
  const changed = structuredClone(audit);
  changed.cases.at(-1)?.activation.script_flags.push("TAPROOT");
  // Act and assert
  expect(() => checkMainnetReceipt(JSON.stringify(changed, null, 2) + "\n", audit)).toThrow("verified case prerequisites");
});

test("an altered creating-output commitment invalidates the receipt", () => {
  // Arrange
  const changed = structuredClone(audit);
  changed.cases[1].external_inputs.outpoints_and_creation_proofs_sha256 = "0".repeat(64);
  // Act and assert
  expect(() => checkMainnetReceipt(JSON.stringify(changed, null, 2) + "\n", audit)).toThrow("verified case prerequisites");
});

test("the receipt cannot claim whole-chain validation", () => {
  // Arrange
  const changed = { ...audit, scope: "whole-chain Mainnet validation" };
  // Act and assert
  expect(() => checkMainnetReceipt(JSON.stringify(changed, null, 2) + "\n", audit)).toThrow("verified case prerequisites");
});

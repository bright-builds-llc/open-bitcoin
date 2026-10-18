import { expect, test } from "bun:test";
import { readSourceCorpus } from "./source-corpus";
import { checkCurrentWorkProjection } from "./check-phase68-active-chain-persistence";
import { resolve } from "node:path";

const source = readSourceCorpus(resolve(import.meta.dir, ".."),
  "packages/open-bitcoin-node/src/sync/runtime_state.rs");

test("current exact carrier and lossless projection pass", () => {
  // Arrange / Act
  const findings = checkCurrentWorkProjection(source);
  // Assert
  expect(findings).toEqual([]);
});

for (const [before, after] of [
  ["open_bitcoin_core::consensus::chain_proof::ProofWork", "u128"],
  ["block.chain_work.to_string()", "block.height.to_string()"],
  ["block.chain_work.to_string()", "(block.chain_work as u128).to_string()"],
  ["block.chain_work.to_string()", "block.height.to_string() /* block.chain_work.to_string() */"],
  ["open_bitcoin_core::consensus::chain_proof::ProofWork", "u128 /* chain_work: ProofWork */"],
] as const) {
  test(`current work regression refuses: ${after}`, () => {
    // Arrange
    expect(source.includes(before)).toBe(true);
    const mutated = source.replaceAll(before, after);
    // Act
    const findings = checkCurrentWorkProjection(mutated);
    // Assert
    expect(findings.length).toBeGreaterThan(0);
  });
}

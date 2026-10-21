import { expect, test } from "bun:test";
import { buildVectors, checkVectors } from "./phase166-genesis-vectors";

test("four independently reproduced complete genesis bodies match frozen expectations", async () => {
  // Arrange
  const frozen = await Bun.file("docs/parity/fixtures/phase166-genesis-vectors.json").json();
  // Act
  const actual = await buildVectors();
  // Assert
  expect(actual).toEqual(frozen);
  expect(actual.genesis.map((row) => row.chain)).toEqual(["mainnet", "testnet3", "signet", "regtest"]);
  for (const row of actual.genesis) {
    expect(row.block_hex.length).toBe(570);
    expect(row.header_hex.length).toBe(160);
    expect(row.output_script_hex.length).toBe(134);
    expect(row.basic.predecessor_raw).toBe("00".repeat(32));
    expect(row.proof_hex.length).toBe(64);
    expect(row.proof_le.length).toBe(64);
  }
});

for (const mutation of ["body", "filter-key", "predecessor"] as const) {
  test(`independent checker rejects changed ${mutation}`, async () => {
    // Arrange
    const altered = await buildVectors();
    const row = altered.genesis[0];
    if (mutation === "body") row.block_hex = `${row.block_hex.slice(0, -2)}01`;
    if (mutation === "filter-key") row.basic.k0 = "0";
    if (mutation === "predecessor") row.basic.predecessor_raw = "01".repeat(32);
    // Act / Assert
    await expect(checkVectors(altered)).rejects.toThrow("genesis vector mismatch");
  });
}

test("wide carry, wrap and encoding are separate arithmetic-only evidence", async () => {
  // Arrange
  const fixture = await buildVectors();
  // Act
  const arithmetic = fixture.arithmetic_only;
  // Assert
  expect(arithmetic.evidence_class).toBe("arithmetic-only-no-chain-authority");
  expect(arithmetic.addition.map((row) => row.name)).toEqual([
    "carry-bit128", "carry-above-bit128", "wrap-bit256", "two-unit-proofs-wrap",
  ]);
  expect(arithmetic.encoding.some((row) => BigInt(`0x${row.hex}`) > ((1n << 128n) - 1n))).toBe(true);
  expect(fixture.genesis.every((row) => BigInt(`0x${row.proof_hex}`) < (1n << 128n))).toBe(true);
});

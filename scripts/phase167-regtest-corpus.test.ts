import { expect, test } from "bun:test";
import { buildCorpus, checkCorpus } from "./phase167-regtest-corpus";
import { assertFrozenJsonBytes } from "./phase167-regtest-serialization";

const generated = buildCorpus();

test("phase167 ordinary corpus freezes canonical genesis through301 and distinct next302", async () => {
  // Arrange
  const frozenHistory = await Bun.file(
    "packages/testdata/phase167/regtest-history.json",
  ).json();
  const frozenExpected = await Bun.file(
    "docs/parity/fixtures/phase167-regtest-expectations.json",
  ).json();
  // Act
  const actual = await generated;
  // Assert
  expect(actual.history).toEqual(frozenHistory);
  expect(actual.expectations).toEqual(frozenExpected);
  expect(actual.history.rows.length).toBe(302);
  expect(actual.history.rows.map((row) => row.height)).toEqual(
    Array.from({ length: 302 }, (_, height) => height),
  );
  expect(actual.history.rows[0].block_hash_raw).toBe(
    "06226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f",
  );
  expect(actual.history.next_302.height).toBe(302);
  expect(actual.history.next_302.parent_hash_raw).toBe(
    actual.history.rows[301].block_hash_raw,
  );
  expect(
    new Set(
      [...actual.history.rows, actual.history.next_302].map(
        (row) => row.block_hash_raw,
      ),
    ).size,
  ).toBe(303);
});

test("phase167 default monetary rules preserve halving fees and exclude genesis coins", async () => {
  // Arrange
  const { history, expectations } = await generated;
  // Act
  const boundaries = expectations.boundaries.map((row) => [
    row.height,
    row.subsidy_sats,
  ]);
  // Assert
  expect(history.constants.coinbase_maturity).toBe(100);
  expect(history.constants.subsidy_halving_interval).toBe(150);
  expect(boundaries).toEqual([
    [149, "5000000000"],
    [150, "2500000000"],
    [299, "2500000000"],
    [300, "1250000000"],
  ]);
  expect(expectations.genesis_utxo_count).toBe(0);
  expect(expectations.rows[0].utxos).toEqual([]);
  expect(expectations.rows[101].fees_sats).toBe("1000");
  expect(expectations.rows[151].fees_sats).toBe("3000");
  expect(expectations.rows[201].fees_sats).toBe("2000");
  expect(expectations.rows[301].utxo_count).toBe(303);
  expect(expectations.rows[301].utxo_total_sats).toBe("1122500000000");
});

test("phase167 distinct height1 prevouts fund maturity101 witness151 and creating201", async () => {
  // Arrange
  const { expectations } = await generated;
  const rows = expectations.rows;
  // Act
  const spent = [101, 151, 201].map((height) => rows[height].spent[0]);
  // Assert
  expect(spent.map((coin) => coin.vout)).toEqual([0, 2, 1]);
  expect(new Set(spent.map((coin) => `${coin.txid}:${coin.vout}`)).size).toBe(
    3,
  );
  expect(spent.every((coin) => coin.height === 1 && coin.coinbase)).toBe(true);
  expect(rows[101].height - spent[0].height).toBe(100);
  expect(rows[151].transactions[1].inputs[0].witness).toEqual(["51"]);
  expect(rows[151].transactions[0].inputs[0].witness).toEqual([
    "00".repeat(32),
  ]);
});

test("phase167 creating201 uses independent parent zero MTP and real512second units", async () => {
  // Arrange
  const { history, expectations } = await generated;
  const fact = expectations.creating_live_201;
  // Act
  const lastInvalid = fact.creating_parent_mtp + fact.sequence_units * 512 - 1;
  // Assert
  expect(fact.version).toBe(2);
  expect(fact.creating_parent_height).toBe(0);
  expect(fact.creating_parent_mtp).toBe(1296688602);
  expect(fact.sequence).toBe((1 << 22) | fact.sequence_units);
  expect(fact.last_invalid_time).toBe(lastInvalid);
  expect(lastInvalid).toBeLessThan(fact.candidate_parent_mtp);
  expect(lastInvalid + 512).toBeGreaterThanOrEqual(fact.candidate_parent_mtp);
  expect(history.branch_mtp_scenario.parent_mtp).not.toBe(
    fact.candidate_parent_mtp,
  );
  expect(history.branch_mtp_scenario.parent_mtp).toBeLessThanOrEqual(
    lastInvalid,
  );
  expect(fact.context_suffix_for_plan16).toEqual({
    first_height: 46,
    last_height: 200,
    count: 155,
    lacks_creating_parent_zero: true,
  });
});

test("phase167 exact full work and BASIC records bind every independently serialized body", async () => {
  // Arrange
  const { expectations } = await generated;
  // Act / Assert
  for (const row of [...expectations.rows, expectations.next_302]) {
    expect(row.proof_hex).toBe("2".padStart(64, "0"));
    expect(row.chain_work_hex).toBe(
      (BigInt(row.height + 1) * 2n).toString(16).padStart(64, "0"),
    );
    expect(row.basic.block_hash_raw).toBe(row.block_hash_raw);
    expect(row.basic.predecessor_raw).toBe(
      row.height
        ? expectations.rows[row.height - 1].basic.header_raw
        : "00".repeat(32),
    );
    expect(typeof row.basic.encoded).toBe("string");
    expect((row.basic.encoded as string).length).toBeGreaterThan(0);
    expect((row.basic.header_raw as string).length).toBe(64);
  }
});

test("phase167 independently named invalid blocks retain exact source-derived boundary matrix", async () => {
  // Arrange
  const { history } = await generated;
  // Act
  const names = history.negatives.map((row) => row.name);
  // Assert
  expect(names).toEqual([
    "overpay-one-satoshi-149",
    "overpay-one-satoshi-150",
    "overpay-one-satoshi-299",
    "overpay-one-satoshi-300",
    "depth99-at100",
    "wrong-version",
    "wrong-difficulty",
    "time-equals-parent-mtp",
    "wrong-bip34-height",
    "absolute-nonfinal",
    "bip68-height-nonfinal",
    "bip68-time-nonfinal",
    "witness-without-commitment",
    "witness-reserve-31",
    "witness-reserve-33",
    "witness-commitment-mismatch",
    "missing-input",
    "aggregate-sigops-80004",
    "invalid-pow",
  ]);
  expect(
    history.negatives
      .filter((row) => !row.pow_satisfied)
      .map((row) => row.name),
  ).toEqual(["invalid-pow"]);
  expect(
    history.negatives.every((row) => row.isolated_known_identity_bound <= 306),
  ).toBe(true);
  expect(
    history.branch_mtp_scenario.independent_known_identities,
  ).toBeLessThanOrEqual(306);
});

for (const changed of [
  "monetary",
  "synthetic-root",
  "maturity1",
  "caller-score",
  "invalid-pow",
  "missing-input",
  "basic",
] as const) {
  test(`phase167 independent frozen check rejects altered ${changed}`, async () => {
    // Arrange
    const original = await generated;
    const history = structuredClone(original.history);
    const expected = structuredClone(original.expectations);
    if (changed === "monetary")
      history.constants.subsidy_halving_interval = 149;
    if (changed === "synthetic-root")
      history.rows[0].block_hash_raw = "11".repeat(32);
    if (changed === "maturity1") history.constants.coinbase_maturity = 1;
    if (changed === "caller-score")
      expected.rows[150].chain_work_hex = "ff".repeat(32);
    if (changed === "invalid-pow") {
      const maybeNegative = history.negatives.find(
        (row) => row.name === "invalid-pow",
      );
      if (!maybeNegative) throw Error("missing invalid-PoW source case");
      history.rows[149].block_hex = maybeNegative.block_hex;
    }
    if (changed === "missing-input")
      expected.rows[101].transactions[1].inputs[0].txid = "11".repeat(32);
    if (changed === "basic")
      expected.rows[150].basic.header_raw = "00".repeat(32);
    // Act / Assert
    await expect(checkCorpus(history, expected)).rejects.toThrow(
      "regtest corpus/source expectations mismatch",
    );
  });
}

test("phase167 exact frozen-byte checker accepts complete canonical files", async () => {
  // Arrange
  const { history, expectations } = await generated;
  const historyBytes = await Bun.file(
    "packages/testdata/phase167/regtest-history.json",
  ).bytes();
  const expectationBytes = await Bun.file(
    "docs/parity/fixtures/phase167-regtest-expectations.json",
  ).bytes();
  // Act / Assert
  expect(() => assertFrozenJsonBytes(historyBytes, history)).not.toThrow();
  expect(() =>
    assertFrozenJsonBytes(expectationBytes, expectations),
  ).not.toThrow();
});

test("phase167 exact frozen-byte checker rejects equivalent whitespace changes", async () => {
  // Arrange
  const { history } = await generated;
  const canonical = JSON.stringify(history, null, 2) + "\n";
  const changed = [
    canonical + "\n",
    JSON.stringify(history) + "\n",
    canonical.replace('  "format":', '    "format":'),
  ];
  for (const text of changed) {
    expect(JSON.parse(text)).toEqual(history);
    // Act / Assert
    expect(() =>
      assertFrozenJsonBytes(Buffer.from(text, "utf8"), history),
    ).toThrow("regtest frozen JSON bytes mismatch");
  }
});

test("phase167 exact frozen-byte checker rejects equivalent object key order", async () => {
  // Arrange
  const { history } = await generated;
  const reordered = Object.fromEntries(Object.entries(history).reverse());
  const changed = Buffer.from(
    JSON.stringify(reordered, null, 2) + "\n",
    "utf8",
  );
  expect(JSON.parse(changed.toString("utf8"))).toEqual(history);
  // Act / Assert
  expect(() => assertFrozenJsonBytes(changed, history)).toThrow(
    "regtest frozen JSON bytes mismatch",
  );
});

// Registered raw-byte matrix for both frozen file kinds. Each mutation reaches
// the same assertion used by the generator's --check path.
const frozenMutations = {
  compact: (_frozen: Buffer, expected: unknown) =>
    Buffer.from(JSON.stringify(expected) + "\n"),
  changedIndentation: (_frozen: Buffer, expected: unknown) =>
    Buffer.from(JSON.stringify(expected, null, 4) + "\n"),
  extraFinalLf: (frozen: Buffer) => Buffer.concat([frozen, Buffer.from("\n")]),
  missingFinalLf: (frozen: Buffer) => frozen.subarray(0, frozen.length - 1),
  leadingBom: (frozen: Buffer) =>
    Buffer.concat([Buffer.from([0xef, 0xbb, 0xbf]), frozen]),
  invalidUtf8: (frozen: Buffer) => Buffer.concat([Buffer.from([0xff]), frozen]),
  reorderedKeys: (_frozen: Buffer, expected: unknown) =>
    Buffer.from(
      JSON.stringify(
        Object.fromEntries(Object.entries(expected as object).reverse()),
        null,
        2,
      ) + "\n",
    ),
};

for (const frozenCase of [
  {
    kind: "history",
    path: "packages/testdata/phase167/regtest-history.json",
    expected: async () => (await generated).history,
  },
  {
    kind: "expectations",
    path: "docs/parity/fixtures/phase167-regtest-expectations.json",
    expected: async () => (await generated).expectations,
  },
]) {
  test(`phase167 exact frozen-byte matrix accepts canonical ${frozenCase.kind}`, async () => {
    // Arrange
    const expected = await frozenCase.expected();
    const frozen = await Bun.file(frozenCase.path).bytes();
    // Act / Assert
    expect(() => assertFrozenJsonBytes(frozen, expected)).not.toThrow();
  });

  for (const [mutationName, mutate] of Object.entries(frozenMutations)) {
    test(`phase167 exact frozen-byte matrix rejects ${mutationName} ${frozenCase.kind}`, async () => {
      // Arrange
      const expected = await frozenCase.expected();
      const frozen = Buffer.from(await Bun.file(frozenCase.path).bytes());
      expect(frozen.at(-1)).toBe(10);
      const changed = mutate(frozen, expected);
      expect(changed.equals(frozen)).toBe(false);
      // Act / Assert
      expect(() => assertFrozenJsonBytes(changed, expected)).toThrow(
        "regtest frozen JSON bytes mismatch",
      );
    });
  }
}

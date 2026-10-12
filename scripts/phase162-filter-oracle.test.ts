import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { EDGE_INPUTS } from "./generate-basic-filter-vectors";
import { LIMITS, PIN, runBounded } from "./generate-peer-filter-fixtures";
import { deriveRecipe, validateRecipe, RECIPE_ID, type RawBlock } from "./phase162-filter-oracle";

function smallRecipe() {
  const names = ["actual-genesis", "actual-funding", "actual-spending"];
  return { recipe_id: RECIPE_ID, network_magic: "fabfb5da", blocks: names.map(name => {
    const row = EDGE_INPUTS.find(row => row.name === name);
    if (!row) throw new Error("missing public raw recipe");
    return { name, previous_name: row.previous_name, raw_block: row.raw_block };
  }) };
}

test("resolves historical and same-block inputs from pinned raw decoding", () => {
  // Arrange / Act
  const result = deriveRecipe(smallRecipe());
  // Assert
  expect(result.cases[2].spent).toEqual(["51", "52"]);
  expect(result.wire.some(row => row.command === "cfilter")).toBe(true);
  expect(result.recipe_id).toBe(RECIPE_ID);
});

test("rejects supplied filter expectations before invoking a child", () => {
  // Arrange
  const recipe = smallRecipe();
  Object.assign(recipe.blocks[0], { encoded: "00" });
  // Act / Assert
  expect(() => validateRecipe(recipe)).toThrow("schema");
});

test("wrong Knots pin fails before raw oracle invocation", () => {
  expect(() => deriveRecipe(smallRecipe(), undefined, () => "0".repeat(40))).toThrow("pin mismatch");
});

const hash = (bytes: Buffer) => createHash("sha256").update(createHash("sha256").update(bytes).digest()).digest();
function le32(value: number): Buffer { const bytes = Buffer.alloc(4); bytes.writeUInt32LE(value); return bytes; }
function le64(value: bigint): Buffer { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(value); return bytes; }
const concat = (...parts: (Buffer | string)[]) => Buffer.concat(parts.map(part => typeof part === "string" ? Buffer.from(part, "hex") : part));
type RawFixture = { row: RawBlock; hash: Buffer; finalTx: Buffer };

/** Independent tiny raw encoder: oracle inputs, never Rust or filter expectations. */
function rawChild(maybeParent: RawFixture | undefined, height: number, tag = 81,
  maybeFunding?: Buffer, fundingScript = "51"): RawFixture {
  const number = le32(height);
  let length = 4;
  while (length && number[length - 1] === 0) length--;
  const script = concat(Buffer.from([length]), number.subarray(0, length), Buffer.from([1, tag]));
  const coinbase = concat("0100000001", Buffer.alloc(32), "ffffffff", Buffer.from([script.length]), script,
    "ffffffff01", le64(5_000_000_000n), Buffer.from([fundingScript.length / 2]), fundingScript, "00000000");
  const transactions = [coinbase];
  if (maybeFunding) {
    for (const output of ["52", "53"]) {
      const previous = transactions.length === 1 ? maybeFunding : transactions[1];
      transactions.push(concat("0200000001", hash(previous), "0000000000ffffffff01",
        le64(4_900_000_000n - BigInt(height * 2000 + transactions.length * 1000)), "01", output, "00000000"));
    }
  }
  let level = transactions.map(hash);
  while (level.length > 1) {
    if (level.length % 2) level.push(level[level.length - 1]);
    level = Array.from({ length: level.length / 2 }, (_, i) => hash(concat(level[i * 2], level[i * 2 + 1])));
  }
  const header = concat("01000000", maybeParent?.hash ?? Buffer.alloc(32), level[0],
    le32(1000 + height * 100 + tag), "ffff7f20", le32(0));
  const name = `h${height}-t${tag}`;
  return { row: { name, previous_name: maybeParent?.row.name ?? "",
    raw_block: concat(header, Buffer.from([transactions.length]), ...transactions).toString("hex") },
  hash: hash(header), finalTx: transactions[transactions.length - 1] };
}

function independentRecipe(count = 1001, script = "51") {
  const chain: RawFixture[] = [];
  for (let height = 0; height < count; height++) {
    const maybeFunding = height === 101 ? chain[1].finalTx : height > 101 ? chain[height - 1].finalTx : undefined;
    chain.push(rawChild(chain.at(-1), height, 81, maybeFunding, height === 1 ? script : "51"));
  }
  return { chain, recipe: { recipe_id: RECIPE_ID, network_magic: "fabfb5da", blocks: chain.map(row => row.row) } };
}

test("raw referenced output mutation changes independently derived spend and commitment", () => {
  // Arrange
  const original = independentRecipe(102);
  const mutated = independentRecipe(102, "55");
  // Act
  const before = deriveRecipe(original.recipe);
  const after = deriveRecipe(mutated.recipe);
  // Assert
  expect(before.cases[101].spent).toEqual(["51", "52"]);
  expect(after.cases[101].spent).toEqual(["55", "52"]);
  expect(after.cases[101].encoded).not.toBe(before.cases[101].encoded);
  expect(after.cases[101].header_raw).not.toBe(before.cases[101].header_raw);
});

test("finite batches preserve branch-local raw outputs and distinct positive h1000 checkpoints", () => {
  // Arrange
  const { chain, recipe } = independentRecipe();
  let parent = chain[998];
  for (let height = 999; height <= 1002; height++) {
    parent = rawChild(parent, height, 98, parent.finalTx);
    recipe.blocks.push(parent.row);
  }
  // Act
  const result = deriveRecipe(recipe);
  const checkpoints = result.wire.filter(row => row.command === "cfcheckpt");
  // Assert
  expect(result.cases).toHaveLength(1005);
  expect(result.cases[1001].spent).toEqual(["53", "52"]);
  expect(checkpoints).toHaveLength(2);
  expect(checkpoints[0].input.headers_raw).toEqual([result.cases[1000].header_raw]);
  expect(checkpoints[1].input.headers_raw).toEqual([result.cases[1002].header_raw]);
  expect(checkpoints[1].input.headers_raw).not.toEqual(checkpoints[0].input.headers_raw);
  expect(checkpoints.every(row => row.expected?.frame_hex.startsWith(recipe.network_magic))).toBe(true);
  expect(result.wire.filter(row => row.command === "cfheaders").every(row =>
    Array.isArray(row.input.hashes_raw) && row.input.hashes_raw.length === 4)).toBe(true);
});

test("accepts 1100 active blocks plus a two-block fork through h1100", () => {
  // Arrange
  const { chain, recipe } = independentRecipe(1100);
  let parent = chain[1098];
  for (let height = 1099; height <= 1100; height++) {
    parent = rawChild(parent, height, 98, parent.finalTx);
    recipe.blocks.push(parent.row);
  }
  // Act
  const result = deriveRecipe(recipe);
  // Assert
  expect(result.cases).toHaveLength(1102);
  expect(result.cases.at(-1)?.name).toBe("h1100-t98");
  expect(result.cases.at(-1)?.spent).toEqual(["53", "52"]);
});

test("rejects 1101 canonical records independently of branch budget", () => {
  // Arrange
  const { recipe } = independentRecipe(1101);
  // Act / Assert
  expect(() => validateRecipe(recipe)).toThrow("active block budget exceeded");
});

test("rejects a ninth branch record independently of total budget", () => {
  // Arrange
  const { chain, recipe } = independentRecipe();
  let parent = chain[998];
  for (let height = 999; height <= 1007; height++) {
    parent = rawChild(parent, height, 98, parent.finalTx);
    recipe.blocks.push(parent.row);
  }
  // Act / Assert
  expect(() => validateRecipe(recipe)).toThrow("branch block budget exceeded");
});

test.each(["spent", "outputs", "hash_raw", "header_raw", "block_hash_raw"])("rejects caller-supplied %s expectations", field => {
  const recipe = smallRecipe();
  Object.assign(recipe.blocks[0], { [field]: [] });
  expect(() => validateRecipe(recipe)).toThrow("schema");
});

test.each([
  ["duplicate identity", (recipe: ReturnType<typeof smallRecipe>) => { recipe.blocks[1].name = recipe.blocks[0].name; }],
  ["future ancestry", (recipe: ReturnType<typeof smallRecipe>) => { recipe.blocks[1].previous_name = recipe.blocks[2].name; }],
  ["raw hex", (recipe: ReturnType<typeof smallRecipe>) => { recipe.blocks[0].raw_block = "zz"; }],
  ["case budget", (recipe: ReturnType<typeof smallRecipe>) => { recipe.blocks = Array(1109).fill(recipe.blocks[0]); }],
  ["input budget", (recipe: ReturnType<typeof smallRecipe>) => { recipe.blocks[0].raw_block = "00".repeat(LIMITS.input); }],
])("refuses %s violation", (_name, mutate) => {
  const recipe = smallRecipe();
  mutate(recipe);
  expect(() => validateRecipe(recipe)).toThrow();
});

test("raw parent mismatch fails even when named ancestry is valid", () => {
  const recipe = smallRecipe();
  recipe.blocks[1].raw_block = recipe.blocks[1].raw_block.slice(0, 8) + "00".repeat(32) + recipe.blocks[1].raw_block.slice(72);
  expect(() => deriveRecipe(recipe)).toThrow("parent identity mismatch");
});

test("duplicate raw identity fails independently of unique case names", () => {
  const recipe = smallRecipe();
  recipe.blocks[1].raw_block = recipe.blocks[0].raw_block;
  expect(() => deriveRecipe(recipe)).toThrow("duplicate raw block identity");
});

test("raw unresolved and double-spent outpoints fail visibly", () => {
  const recipe = smallRecipe();
  recipe.blocks[2].raw_block = recipe.blocks[2].raw_block.replace(
    "e0b2d11ebadcf85195908dd88a99ed125d05689be1d083f8d7f54cf26fd05bf3", "ff".repeat(32));
  expect(() => deriveRecipe(recipe)).toThrow("unresolved, future or spent raw prevout");
});

test("branch cannot spend a sibling-only raw output", () => {
  const { chain, recipe } = independentRecipe(103);
  const branch = rawChild(chain[101], 102, 98, chain[102].finalTx);
  recipe.blocks.push(branch.row);
  expect(() => deriveRecipe(recipe)).toThrow("unresolved, future or spent raw prevout");
});

test("oversized child output is refused before parsing", () => {
  expect(() => deriveRecipe(smallRecipe(), undefined, (command, args) => command === "git" ?
    (args.includes("rev-parse") ? PIN : "") : " ".repeat(LIMITS.stdout + 1))).toThrow("output budget");
});

test("child failure propagates", () => {
  expect(() => deriveRecipe(smallRecipe(), undefined, (command, args) => {
    if (command === "git") return args.includes("rev-parse") ? PIN : "";
    throw new Error("deliberate child failure");
  })).toThrow("child failure");
});

test("strict response schema refuses missing cases", () => {
  expect(() => deriveRecipe(smallRecipe(), undefined, (command, args, root, maybeInput) => {
    if (command !== "python3") return runBounded(command, args, root, maybeInput);
    const result = JSON.parse(runBounded(command, args, root, maybeInput));
    result.cases.pop();
    return JSON.stringify(result);
  })).toThrow("case count");
});

test("self-inconsistent child commitments fail response validation", () => {
  expect(() => deriveRecipe(smallRecipe(), undefined, (command, args, root, maybeInput) => {
    const output = runBounded(command, args, root, maybeInput);
    if (command !== "python3") return output;
    const result = JSON.parse(output);
    result.cases[0].hash_raw = "00".repeat(32);
    return JSON.stringify(result);
  })).toThrow("commitment mismatch");
});

test("ordered input digest is independent of JSON object key insertion order", () => {
  const recipe = smallRecipe();
  const reordered = { ...recipe, blocks: recipe.blocks.map(row => ({
    raw_block: row.raw_block, previous_name: row.previous_name, name: row.name,
  })) };
  expect(deriveRecipe(reordered).ordered_input_digest).toBe(deriveRecipe(recipe).ordered_input_digest);
});

test("stdin entrypoint emits the same bounded raw-input digest and commitments", () => {
  // Arrange / Act
  const recipe = smallRecipe();
  const child = spawnSync(process.execPath, ["scripts/phase162-filter-oracle.ts"], {
    cwd: resolve(import.meta.dir, ".."), input: JSON.stringify(recipe), encoding: "utf8",
    timeout: LIMITS.timeout, maxBuffer: LIMITS.stdout,
  });
  // Assert
  expect(child.error).toBeUndefined();
  expect(child.status).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual(deriveRecipe(recipe));
});

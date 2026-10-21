/** Bounded source-derived genesis evidence, never runtime acceptance authority. */
import { createHash } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";

const ROOT = resolve(import.meta.dir, "..");
const FIXTURE = "docs/parity/fixtures/phase166-genesis-vectors.json";
const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
const MASK = (1n << 256n) - 1n;
const ZERO = "00".repeat(32);
const SOURCES = [
  "src/kernel/chainparams.cpp", "src/chain.cpp", "src/arith_uint256.h",
  "src/blockfilter.cpp", "src/index/blockfilterindex.cpp",
  "test/functional/test_framework/crypto/siphash.py",
  "test/functional/test_framework/messages.py",
];

function hash256(bytes: Uint8Array): Buffer {
  const digest = createHash("sha256").update(bytes).digest();
  return createHash("sha256").update(digest).digest();
}

function unsigned(value: number | bigint, width: number): Buffer {
  const bytes = Buffer.alloc(width);
  let remaining = BigInt(value);
  if (remaining < 0n || remaining >= (1n << BigInt(width * 8))) throw new Error("integer bounds");
  for (let index = 0; index < width; index++) {
    bytes[index] = Number(remaining & 255n);
    remaining >>= 8n;
  }
  return bytes;
}

function fixedHex(value: bigint): string { return value.toString(16).padStart(64, "0"); }

function proof(bits: number): bigint {
  const size = bits >>> 24;
  const word = BigInt(bits & 0x007fffff);
  const target = size <= 3 ? word >> BigInt(8 * (3 - size)) : word << BigInt(8 * (size - 3));
  if ((bits & 0x00800000) || target <= 0n || target > MASK) throw new Error("invalid target");
  // GetBlockProof's (~target / (target + 1)) + 1, using unbounded independent integers.
  return ((MASK - target) / (target + 1n)) + 1n;
}

async function pinnedSources() {
  const revision = execFileSync("git", ["-C", "packages/bitcoin-knots", "rev-parse", "HEAD"],
    { cwd: ROOT, encoding: "utf8", timeout: 10_000, maxBuffer: 1024 }).trim();
  if (revision !== PIN) throw new Error("Knots pin mismatch");
  const sources = [];
  let chainparams = "";
  for (const relative of SOURCES) {
    const file = Bun.file(resolve(ROOT, "packages/bitcoin-knots", relative));
    if (file.size > 1_000_000) throw new Error("source size bound");
    const bytes = await file.bytes();
    const pinned = execFileSync("git", ["-C", "packages/bitcoin-knots", "show", `${PIN}:${relative}`],
      { cwd: ROOT, timeout: 10_000, maxBuffer: 1_000_000 });
    if (!Buffer.from(bytes).equals(pinned)) throw new Error("dirty Knots source refused");
    sources.push({ path: relative, sha256: createHash("sha256").update(bytes).digest("hex") });
    if (relative === SOURCES[0]) chainparams = Buffer.from(bytes).toString("utf8");
  }
  return { sources, chainparams };
}

function transactionFromSource(source: string) {
  const maybeTimestamp = source.match(/const char\* pszTimestamp = "([^"]+)";/);
  const maybePubkey = source.match(/genesisOutputScript = CScript\(\) << "([a-f0-9]+)"_hex << OP_CHECKSIG/);
  if (!maybeTimestamp || !maybePubkey) throw new Error("genesis transaction source anchor");
  const scriptSig = Buffer.concat([Buffer.from("04ffff001d010445", "hex"), Buffer.from(maybeTimestamp[1])]);
  const outputScript = Buffer.concat([Buffer.from([65]), Buffer.from(maybePubkey[1], "hex"), Buffer.from([0xac])]);
  if (scriptSig.length !== 77 || outputScript.length !== 67) throw new Error("genesis script bounds");
  const transaction = Buffer.concat([
    unsigned(1, 4), Buffer.from([1]), Buffer.alloc(32), unsigned(0xffffffff, 4),
    Buffer.from([scriptSig.length]), scriptSig, unsigned(0xffffffff, 4), Buffer.from([1]),
    unsigned(5_000_000_000n, 8), Buffer.from([outputScript.length]), outputScript, unsigned(0, 4),
  ]);
  return { transaction, outputScript };
}

function genesisFromSource(source: string) {
  const { transaction, outputScript } = transactionFromSource(source);
  const roots = [...source.matchAll(/genesis = CreateGenesisBlock\((\d+), (\d+), (0x[a-f0-9]+), 1, 50 \* COIN\);[\s\S]*?assert\(consensus.hashGenesisBlock == uint256\{"([a-f0-9]{64})"\}\);/g)];
  if (roots.length !== 4) throw new Error("four genesis source anchors required");
  const chains = ["mainnet", "testnet3", "signet", "regtest"];
  return roots.map((match, index) => {
    const bits = Number(match[3]);
    const header = Buffer.concat([
      unsigned(1, 4), Buffer.alloc(32), hash256(transaction), unsigned(Number(match[1]), 4),
      unsigned(bits, 4), unsigned(Number(match[2]), 4),
    ]);
    const hash = hash256(header);
    if (Buffer.from(hash).reverse().toString("hex") !== match[4]) throw new Error("pinned genesis hash mismatch");
    const work = proof(bits);
    const txid = hash256(transaction);
    return {
      chain: chains[index], header_hex: header.toString("hex"),
      block_hex: Buffer.concat([header, Buffer.from([1]), transaction]).toString("hex"),
      block_hash_raw: hash.toString("hex"), block_hash_display: match[4],
      transaction_hex: transaction.toString("hex"), txid_raw: txid.toString("hex"),
      txid_display: Buffer.from(txid).reverse().toString("hex"),
      outpoint_hex: Buffer.concat([txid, unsigned(0, 4)]).toString("hex"),
      output_script_hex: outputScript.toString("hex"), output_sats: "5000000000",
      bits_hex: bits.toString(16).padStart(8, "0"), proof_hex: fixedHex(work),
      proof_le: unsigned(work, 32).toString("hex"),
    };
  });
}

type BasicOracleRow = {
  block_hash_raw: string; outputs: string[]; count: number;
  k0: string; k1: string; encoded: string; hash_raw: string; hash_display: string;
  header_raw: string; header_display: string; predecessor_raw: string;
};

function basicFromOracle(roots: ReturnType<typeof genesisFromSource>): BasicOracleRow[] {
  const cases = roots.map((row) => ({
    name: row.chain, raw_block: row.block_hex, block_hash_raw: row.block_hash_raw,
    outputs: [], spent: [], predecessor_raw: ZERO, previous_name: "",
  }));
  const result = spawnSync("python3", ["scripts/basic-filter-oracle.py"], {
    cwd: ROOT, input: JSON.stringify({ cases }), encoding: "utf8",
    timeout: 30_000, maxBuffer: 128_000,
  });
  if (result.error || result.status !== 0) throw new Error("pinned BASIC oracle failed", { cause: result.error });
  const output = JSON.parse(result.stdout) as { cases: BasicOracleRow[] };
  if (output.cases.length !== 4) throw new Error("BASIC oracle case bound");
  return output.cases;
}

async function arithmeticOnly() {
  const source = await Bun.file(resolve(ROOT, "docs/parity/fixtures/phase165-work-vectors.json")).json();
  if (source.source_pin.commit !== PIN) throw new Error("work vector pin mismatch");
  const names = ["carry-bit128", "carry-above-bit128", "wrap-bit256", "two-unit-proofs-wrap"];
  const addition = names.map((name) => {
    const maybeRow = source.addition_vectors.find((row: { name: string }) => row.name === name);
    if (!maybeRow) throw new Error("wide arithmetic source missing");
    const sum = (BigInt(`0x${maybeRow.left_hex}`) + BigInt(`0x${maybeRow.right_hex}`)) & MASK;
    if (fixedHex(sum) !== maybeRow.sum_hex) throw new Error("wide arithmetic source mismatch");
    return { name, left_hex: maybeRow.left_hex, right_hex: maybeRow.right_hex,
      sum_hex: fixedHex(sum), sum_le: unsigned(sum, 32).toString("hex") };
  });
  const encoding = [1n << 128n, (5n << 128n), 1n << 255n, MASK].map((value) => ({
    hex: fixedHex(value), little_endian: unsigned(value, 32).toString("hex"), decimal: value.toString(),
  }));
  return { evidence_class: "arithmetic-only-no-chain-authority", addition, encoding };
}

/** Reproduce from pinned source and the existing narrow oracle; never invoke production Rust. */
export async function buildVectors() {
  const { chainparams, sources } = await pinnedSources();
  const roots = genesisFromSource(chainparams);
  const basic = basicFromOracle(roots);
  const genesis = roots.map((row, index) => {
    const filter = basic[index];
    if (filter.block_hash_raw !== row.block_hash_raw || filter.outputs.length !== 1 ||
        filter.outputs[0] !== row.output_script_hex || filter.count !== 1 || filter.predecessor_raw !== ZERO) {
      throw new Error("genesis BASIC complete-body binding mismatch");
    }
    return { ...row, basic: {
      k0: filter.k0, k1: filter.k1, encoded: filter.encoded, hash_raw: filter.hash_raw,
      hash_display: filter.hash_display, header_raw: filter.header_raw,
      header_display: filter.header_display, predecessor_raw: filter.predecessor_raw, count: filter.count,
    } };
  });
  return {
    schema_version: 1, source_pin: { version: "29.3.knots20260210", commit: PIN, sources },
    evidence_class: "source-derived-complete-genesis-proof-basic",
    scope: "Independent source-derived bytes; no native daemon differential or chain authority. Genesis outputs remain BASIC inputs despite their UTXO exclusion.",
    formulas: {
      genesis: "chainparams.cpp CreateGenesisBlock; SHA256d header/transaction; raw hashes in wire order",
      proof: "(~target / (target + 1)) + 1; exact unsigned 256-bit LE/hex",
      basic: "blockfilter.cpp: all nonempty non-OP_RETURN outputs, SipHash keyed by genesis hash, high-half mapping M=784931, Rice P=19",
      filter_header: "SHA256d(SHA256d(encoded) || zero predecessor); blockfilterindex.cpp CustomAppend height zero has empty undo",
    }, genesis, arithmetic_only: await arithmeticOnly(),
  };
}

/** Reject every changed field against independent recomputation, including body/key/predecessor. */
export async function checkVectors(value: unknown): Promise<void> {
  if (!isDeepStrictEqual(value, await buildVectors())) throw new Error("genesis vector mismatch");
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  if (args.length !== 1 || !["--check", "--write"].includes(args[0])) throw new Error("expected --check or --write");
  if (args[0] === "--write") {
    await Bun.write(resolve(ROOT, FIXTURE), `${JSON.stringify(await buildVectors(), null, 2)}\n`);
    console.log("Wrote four source-derived genesis vectors and separate arithmetic-only evidence.");
  } else {
    await checkVectors(await Bun.file(resolve(ROOT, FIXTURE)).json());
    console.log("Checked four complete genesis/proof/BASIC vectors; arithmetic is not chain authority.");
  }
}

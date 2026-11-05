/** Independent pinned-source serialized facts; no storage, network or authority. */
import { createHash } from "node:crypto";

export const ZERO = "00".repeat(32);

export const BITS = 0x207fffff;

const MAX256 = (1n << 256n) - 1n;

type Input = {
  txid: string;
  vout: number;
  script: string;
  sequence: number;
  witness: string[];
};

export type Output = { sats: string; script: string };

export type Tx = {
  version: number;
  inputs: Input[];
  outputs: Output[];
  lock_time: number;
};

type Header = {
  version: number;
  parent: string;
  merkle: string;
  time: number;
  bits: number;
  nonce: number;
};

export type Body = { header: Header; transactions: Tx[] };

export const hash = (bytes: Uint8Array) =>
  createHash("sha256").update(bytes).digest();

const hash256 = (bytes: Uint8Array) => hash(hash(bytes));

export const sha = (bytes: Uint8Array) => hash(bytes).toString("hex");

export const bytes = (hex: string) => Buffer.from(hex, "hex");

const concat = (...parts: Uint8Array[]) => Buffer.concat(parts);

export const hex256 = (value: bigint) => value.toString(16).padStart(64, "0");

export const display = (raw: string) => bytes(raw).reverse().toString("hex");

function unsigned(value: number | bigint, width: number): Buffer {
  const n = BigInt(value);
  if (n < 0n || n >= 1n << BigInt(width * 8))
    throw Error("unsigned integer bounds");
  const result = Buffer.alloc(width);
  for (let index = 0; index < width; index++)
    result[index] = Number((n >> BigInt(index * 8)) & 255n);
  return result;
}

function compact(value: number): Buffer {
  if (value < 0 || !Number.isSafeInteger(value))
    throw Error("CompactSize bounds");
  if (value < 253) return unsigned(value, 1);
  if (value <= 65535) return concat(bytes("fd"), unsigned(value, 2));
  return concat(bytes("fe"), unsigned(value, 4));
}

const vector = (value: string) =>
  concat(compact(value.length / 2), bytes(value));

export function encodeTransaction(tx: Tx, witness = true): Buffer {
  const hasWitness =
    witness && tx.inputs.some((input) => input.witness.length > 0);
  const inputs = tx.inputs.map((input) =>
    concat(
      bytes(input.txid),
      unsigned(input.vout, 4),
      vector(input.script),
      unsigned(input.sequence, 4),
    ),
  );
  const outputs = tx.outputs.map((output) =>
    concat(unsigned(BigInt(output.sats), 8), vector(output.script)),
  );
  const stacks = hasWitness
    ? tx.inputs.map((input) =>
        concat(compact(input.witness.length), ...input.witness.map(vector)),
      )
    : [];
  return concat(
    unsigned(tx.version, 4),
    hasWitness ? bytes("0001") : Buffer.alloc(0),
    compact(inputs.length),
    ...inputs,
    compact(outputs.length),
    ...outputs,
    ...stacks,
    unsigned(tx.lock_time, 4),
  );
}

export const txid = (tx: Tx) =>
  hash256(encodeTransaction(tx, false)).toString("hex");

export const wtxid = (tx: Tx) => hash256(encodeTransaction(tx)).toString("hex");

function merkle(leaves: Buffer[]): string {
  if (!leaves.length) return ZERO;
  let level = leaves;
  while (level.length > 1) {
    if (level.length % 2) level = [...level, level[level.length - 1]];
    const next: Buffer[] = [];
    for (let index = 0; index < level.length; index += 2)
      next.push(hash256(concat(level[index], level[index + 1])));
    level = next;
  }
  return level[0].toString("hex");
}

export function encodeHeader(header: Header): Buffer {
  return concat(
    unsigned(header.version, 4),
    bytes(header.parent),
    bytes(header.merkle),
    unsigned(header.time, 4),
    unsigned(header.bits, 4),
    unsigned(header.nonce, 4),
  );
}

export const blockHash = (body: Body) =>
  hash256(encodeHeader(body.header)).toString("hex");

export const encodeBlock = (body: Body) =>
  concat(
    encodeHeader(body.header),
    compact(body.transactions.length),
    ...body.transactions.map((tx) => encodeTransaction(tx)),
  );

const target = (bits: number) =>
  BigInt(bits & 0x007fffff) << BigInt(8 * ((bits >>> 24) - 3));

export const proof = (bits: number) =>
  (MAX256 - target(bits)) / (target(bits) + 1n) + 1n;

export const validPow = (body: Body) =>
  BigInt(`0x${display(blockHash(body))}`) <= target(body.header.bits);

export function seal(body: Body): Body {
  body.header.merkle = merkle(body.transactions.map((tx) => bytes(txid(tx))));
  body.header.nonce = 0;
  while (!validPow(body)) {
    if (++body.header.nonce > 10_000_000)
      throw Error("deterministic fixture mining bound");
  }
  return body;
}

export function heightScript(height: number): string {
  if (height <= 16) return Buffer.from([0x50 + height, 0]).toString("hex");
  const number: number[] = [];
  for (let n = height; n; n >>>= 8) number.push(n & 255);
  if (number[number.length - 1] & 128) number.push(0);
  return Buffer.from([number.length, ...number, 0]).toString("hex");
}

export function commitWitness(body: Body): void {
  body.transactions[0].inputs[0].witness = [ZERO];
  const root = merkle([
    Buffer.alloc(32),
    ...body.transactions.slice(1).map((tx) => bytes(wtxid(tx))),
  ]);
  const commitment = hash256(concat(bytes(root), Buffer.alloc(32))).toString(
    "hex",
  );
  body.transactions[0].outputs.push({
    sats: "0",
    script: `6a24aa21a9ed${commitment}`,
  });
}

export function median(times: number[]): number {
  if (!times.length) throw Error("missing independent MTP");
  const sorted = [...times].sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)];
}

export function genesis(source: string): Body {
  const maybeTimestamp = source.match(/const char\* pszTimestamp = "([^"]+)";/);
  const maybeKey = source.match(
    /genesisOutputScript = CScript\(\) << "([a-f0-9]+)"_hex << OP_CHECKSIG/,
  );
  if (!maybeTimestamp || !maybeKey)
    throw Error("genesis source anchors missing");
  const tx: Tx = {
    version: 1,
    lock_time: 0,
    inputs: [
      {
        txid: ZERO,
        vout: 0xffffffff,
        script: `04ffff001d010445${Buffer.from(maybeTimestamp[1]).toString("hex")}`,
        sequence: 0xffffffff,
        witness: [],
      },
    ],
    outputs: [{ sats: "5000000000", script: `41${maybeKey[1]}ac` }],
  };
  const body = {
    header: {
      version: 1,
      parent: ZERO,
      merkle: txid(tx),
      time: 1296688602,
      bits: BITS,
      nonce: 2,
    },
    transactions: [tx],
  };
  if (
    display(blockHash(body)) !==
      "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206" ||
    !validPow(body)
  )
    throw Error("canonical source genesis mismatch");
  return body;
}

/** Require the generator's exact pretty JSON UTF-8 bytes, including its final LF. */
export function assertFrozenJsonBytes(
  frozen: Uint8Array,
  expected: unknown,
): void {
  const canonical = Buffer.from(
    JSON.stringify(expected, null, 2) + "\n",
    "utf8",
  );
  if (!Buffer.from(frozen).equals(canonical)) {
    throw Error("regtest frozen JSON bytes mismatch");
  }
}

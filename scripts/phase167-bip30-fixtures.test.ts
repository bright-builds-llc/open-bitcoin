import { beforeAll, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, symlinkSync, rmSync, statSync, unlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { decodeBlock, decodeHeader } from "./phase167-historical-corpus";
import { loadProjectionInputs, validateProjectionInputs, renderBip30Fixtures, validateProjectionText,
  runProjection, OUTPUT, FIXED_INPUTS, BOUNDS, type ProjectionInputs } from "./phase167-bip30-fixtures";

let inputs: ProjectionInputs;
beforeAll(() => { inputs = loadProjectionInputs(); }, 120_000);

test("actual complete bodies retain both literal byte-bound repeat/original pairs", () => {
  // Arrange / Act
  const projection = validateProjectionInputs(inputs);
  // Assert
  expect(projection.cases.map(row => [row.height, row.originalHeight, row.repeatBytes, row.originalBytes]))
    .toEqual([[91842, 91812, 214, 616], [91880, 91722, 472, 214]]);
  expect(projection.cases.map(row => [row.repeat.length, row.original.length])).toEqual([[1, 2], [2, 1]]);
  expect(projection.cases[0].repeat[0].txid).toBe("d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599");
  expect(projection.cases[1].repeat[0].txid).toBe("e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468");
});

test("both case headers and raw-body SHA256 values match independently frozen literals", () => {
  // Arrange / Act
  const cases = validateProjectionInputs(inputs).cases;
  // Assert
  expect(cases.map(row => [row.hash, row.originalHash, row.repeatDigest, row.originalDigest])).toEqual([
    ["00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec",
      "00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f",
      "95434eb656dd8feb2611648dc54f4d85d5fad64eaa6a892c89b09888c60fcd93",
      "19e5a91add4003e8a102b9392fdf3e53c422ed9deae39b519dd353a70555695e"],
    ["00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721",
      "00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e",
      "f93f123ca5a9fb01cef7cf5b9caec38619b3c9a6d13eaa8dc65c1753c253f4a2",
      "c7acdfd2acb2da89dfbdb01f3c6405cf7100108aaf09b4f1031592e79d244e8e"],
  ]);
});

test("actual prefix, original-parent and expiry clocks use eleven independent samples", () => {
  // Arrange / Act
  const projection = validateProjectionInputs(inputs);
  // Assert
  expect(projection.cases.map(row => [row.parentMtp, row.creationMtp, row.expiryMtp]))
    .toEqual([[1289767733, 1289756038, 1289713700], [1289778307, 1289721830, 1289728088]]);
  expect(projection.cases.map(row => [row.history[0].height, row.history.at(-1)?.height,
    row.mtpPrefix[0].height, row.mtpPrefix.at(-1)?.height, row.originalMtp[0].height, row.originalMtp.at(-1)?.height]))
    .toEqual([[91841, 91687, 91686, 91677, 91811, 91801], [91879, 91725, 91724, 91715, 91721, 91711]]);
  for (const row of projection.cases) {
    expect(row.history).toHaveLength(155); expect(row.mtpPrefix).toHaveLength(10); expect(row.originalMtp).toHaveLength(11);
    const support = [...row.history, ...row.mtpPrefix];
    for (let i = 0; i < 155; i++) {
      const window = support.slice(i, i + 11);
      expect(window).toHaveLength(11);
      const sourceTimes = window.map(h => {
        const source = inputs.headers.find(raw => raw.hash === h.hash);
        if (!source) throw new Error("missing raw source");
        return decodeHeader(Buffer.from(source.raw, "hex"), source.hash).time;
      }).sort((a, b) => a - b);
      expect(window.map(h => h.time).sort((a, b) => a - b)[5]).toBe(sourceTimes[5]);
      if (i < 154) expect(row.history[i].previousHash).toBe(row.history[i + 1].hash);
    }
  }
});

test("overwritten Coins preserve actual original amounts scripts height and parent MTP", () => {
  // Arrange / Act
  const cases = validateProjectionInputs(inputs).cases;
  // Assert
  expect(cases.map(row => row.overwritten.map(coin => [coin.vout, coin.value, coin.coinbase, coin.created_height, coin.created_mtp])))
    .toEqual([[[0, "5000000000", true, 91812, 1289756038]], [[0, "5000000000", true, 91722, 1289721830]]]);
  expect(cases[0].overwritten[0].script).toBe("41046896ecfc449cb8560594eb7f413f199deb9b4e5d947a142e7dc7d2de0b811b8e204833ea2a2fd9d4c7b153a8ca7661d0a0b7fc981df1f42f55d64b26b3da1e9cac");
  expect(cases[1].overwritten[0].script).toBe("4104124b212f5416598a92ccec88819105179dcb2550d571842601492718273fe0f2179a9695096bff94cd99dcccdea7cd9bd943bfca8fea649cac963411979a33e9ac");
});

test("91880 retains its actual noncoinbase transaction and complete independently fixed external Coin", () => {
  // Arrange / Act
  const cases = validateProjectionInputs(inputs).cases;
  // Assert
  expect(cases[0].external).toEqual([]);
  expect(cases[1].external).toEqual([{ txid: "cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3", vout: 1,
    value: "4480000000", script: "76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac", coinbase: false, created_height: 90644, created_mtp: 1289277562 }]);
  expect(cases[1].repeat[1].txid).toBe("bc9423e0a8b3571181d8545c0091600d8fc421c380457c45c1ca8c0698f765b8");
  expect(cases[1].repeat[1].outputs.map(out => out.value)).toEqual(["1767000000", "2713000000"]);
});

test("the real detached BIP34 commitment remains separately labeled independent seed", () => {
  // Arrange / Act
  const anchor = validateProjectionInputs(inputs).anchor;
  // Assert
  expect([anchor.height, anchor.hash, anchor.version, anchor.time, anchor.bits, anchor.nonce])
    .toEqual([227931, "000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8", 2, 1364196609, 436371822, 2483358230]);
  expect(renderBip30Fixtures(inputs).split("\n").filter(line => line.startsWith("HEADER\tdetached_bip34"))).toHaveLength(1);
});

type Mutation = [string, (input: ProjectionInputs) => void, string];
const mutations: Mutation[] = [
  ["pin", input => { input.pin = "0".repeat(40); }, "pin/evidence"],
  ["evidence", input => { input.evidence = "canonical"; }, "pin/evidence"],
  ["source digest", input => { input.sources[0].digest = "0".repeat(64); }, "source digest/path"],
  ["source path capability", input => { input.sources[0].path = "/etc/passwd"; }, "source digest/path"],
  ["missing source", input => { input.sources.pop(); }, "source digest/path"],
  ["duplicate source", input => { input.sources.push(input.sources[0]); }, "source digest/path"],
  ["wrong body network", input => { input.bodies[0].network = "testnet3"; }, "body network/identity"],
  ["swapped original bodies", input => { [input.bodies[2], input.bodies[3]] = [input.bodies[3], input.bodies[2]]; }, "body network/identity"],
  ["missing original body", input => { input.bodies.pop(); }, "body/envelope count"],
  ["truncated body", input => { input.bodies[1].raw = input.bodies[1].raw.slice(0, -2); }, "body bytes/digest"],
  ["renewed body digest", input => { input.bodies[0].sha256 = "0".repeat(64); }, "body bytes/digest"],
  ["modified transaction", input => { input.bodies[1].raw = input.bodies[1].raw.slice(0, -2) + "01"; }, "body bytes/digest"],
  ["body clock", input => { input.bodies[0].acquired_at = "2026-10-21T18:00:00.000Z"; }, "frozen observation"],
  ["reversed first pair", input => { input.envelopes[0].overwrite_original = [input.bodies[3].hash]; }, "overwrite pair"],
  ["reversed second pair", input => { input.envelopes[1].overwrite_original = [input.bodies[2].hash]; }, "overwrite pair"],
  ["wrong envelope", input => { input.envelopes[0].height++; }, "envelope identity"],
  ["wrong envelope parent", input => { input.envelopes[0].parent = input.bodies[0].hash; }, "envelope parent"],
  ["reversed history", input => { input.envelopes[0].candidate_history.reverse(); }, "window header"],
  ["missing history", input => { input.envelopes[0].candidate_history.pop(); }, "window count"],
  ["duplicate history", input => { input.envelopes[0].candidate_history[1] = input.envelopes[0].candidate_history[0]; }, "window header"],
  ["wrong parent window", input => { input.envelopes[0].parent_window.reverse(); }, "window header"],
  ["missing original MTP", input => { input.envelopes[2].parent_window.pop(); }, "window count"],
  ["modified expiry MTP", input => { input.envelopes[1].expiry_mtp++; }, "MTP field"],
  ["wrong expiry window", input => { input.envelopes[1].expiry_window.reverse(); }, "window header"],
  ["duplicate raw header", input => { input.headers.push(input.headers[1]); }, "duplicate header"],
  ["raw header network", input => { input.headers[1].network = "testnet3"; }, "header network"],
  ["out of range height", input => { input.headers[1].height = 4294967296; }, "height range"],
  ["modified raw header", input => { input.headers[1].raw = "02" + input.headers[1].raw.slice(2); }, "header hash"],
  ["truncated raw header", input => { input.headers[1].raw = input.headers[1].raw.slice(2); }, "bounded lowercase hex"],
  ["missing anchor", input => { input.headers = input.headers.filter(row => row.height !== 227931); }, "detached BIP34"],
  ["wrong anchor height", input => { const row = input.headers.find(row => row.height === 227931); if (row) row.height--; }, "detached BIP34"],
  ["missing prefix", input => { input.headers = input.headers.filter(row => row.height !== 91677); }, "MTP prefix"],
  ["prefix height-only substitution", input => { const row = input.headers.find(row => row.height === 91677); if (row) row.height++; }, "MTP prefix"],
  ["missing external Coin", input => { input.envelopes[1].external_utxos = []; }, "external Coin"],
  ["duplicate external Coin", input => { input.envelopes[1].external_utxos.push(input.envelopes[1].external_utxos[0]); }, "external Coin"],
  ["wrong external amount", input => { input.envelopes[1].external_utxos[0].value = "4480000001"; }, "external Coin"],
  ["wrong external script", input => { input.envelopes[1].external_utxos[0].script = "51"; }, "external Coin"],
  ["wrong external MTP", input => { input.envelopes[1].external_utxos[0].created_mtp++; }, "external Coin"],
  ["wrong external height", input => { input.envelopes[1].external_utxos[0].created_height++; }, "external Coin"],
  ["wrong external coinbase", input => { input.envelopes[1].external_utxos[0].coinbase = true; }, "external Coin"],
];
for (const [label, mutate, reason] of mutations) test(`actual selected corpus refuses ${label}`, () => {
  // Arrange
  const changed = structuredClone(inputs); mutate(changed);
  // Act / Assert
  expect(() => validateProjectionInputs(changed)).toThrow(reason);
});

test("closed grammar rejects unknown tags even if offered as its own expected bytes", () => {
  const text = renderBip30Fixtures(inputs).replace("END\t2\n", "EVALUATE\t2\n");
  expect(() => validateProjectionText(text, text)).toThrow("tag");
});

test("wire integer grammar refuses noncanonical values independently of comparison", () => {
  const text = renderBip30Fixtures(inputs).replace("TX\trepeat\t0\t1\t0", "TX\trepeat\t0\t01\t0");
  expect(() => validateProjectionText(text, text)).toThrow("integer");
});

test("all projected header scalars reconstruct exact accepted eighty-byte headers", () => {
  // Arrange
  const projection = validateProjectionInputs(inputs);
  const headers = [projection.anchor, ...projection.cases.flatMap(row =>
    [row.candidate, row.originalHeader, ...row.history, ...row.mtpPrefix, ...row.originalMtp])];
  // Act / Assert
  for (const header of headers) {
    const raw = Buffer.alloc(80);
    raw.writeInt32LE(header.version); Buffer.from(header.previousHash, "hex").reverse().copy(raw, 4);
    Buffer.from(header.merkleRoot, "hex").reverse().copy(raw, 36);
    raw.writeUInt32LE(header.time, 68); raw.writeUInt32LE(header.bits, 72); raw.writeUInt32LE(header.nonce, 76);
    const source = inputs.headers.find(row => row.hash === header.hash) ?? inputs.bodies.find(row => row.hash === header.hash);
    expect(raw.toString("hex")).toBe(source?.raw.slice(0, 160));
    expect(decodeHeader(raw, header.hash)).toEqual({ hash: header.hash, parent: header.previousHash,
      time: header.time, bits: header.bits, version: header.version });
  }
});

test("full typed transaction fields match accepted decoder for both complete bodies", () => {
  // Arrange
  const projection = validateProjectionInputs(inputs);
  // Act / Assert
  for (const row of projection.cases) for (const role of ["repeat", "original"] as const) {
    const source = inputs.bodies.find(body => body.height === (role === "repeat" ? row.height : row.originalHeight));
    if (!source) throw new Error("missing test source body");
    const decoded = decodeBlock(Buffer.from(source.raw, "hex"), source.hash);
    expect(row[role]).toEqual(decoded.transactions.map(tx => {
      const bytes = Buffer.from(tx.raw, "hex");
      return { txid: tx.txid, wtxid: tx.wtxid, coinbase: tx.coinbase, version: bytes.readInt32LE(),
        lockTime: bytes.readUInt32LE(bytes.length - 4), inputs: tx.inputs, outputs: tx.outputs, witness: tx.witness };
    }));
  }
});

test("deterministic closed text has no executable payload or trailing records", () => {
  // Arrange / Act
  const text = renderBip30Fixtures(inputs);
  // Assert
  expect(renderBip30Fixtures(inputs)).toBe(text);
  expect(() => validateProjectionText(text, text)).not.toThrow();
  expect(() => validateProjectionText(text + "END\t2\n", text)).toThrow();
  expect(text).toContain("OPEN_BITCOIN_PHASE167_BIP30\t1\n");
});

test("rendered HEADER records independently reconstruct all accepted raw headers", () => {
  // Arrange
  const records = renderBip30Fixtures(inputs).split("\n").map(line => line.split("\t")).filter(row => row[0] === "HEADER");
  // Act / Assert
  expect(records).toHaveLength(357);
  for (const row of records) {
    const raw = Buffer.alloc(80);
    raw.writeInt32LE(Number(row[6])); Buffer.from(row[7], "hex").reverse().copy(raw, 4);
    Buffer.from(row[8], "hex").reverse().copy(raw, 36); raw.writeUInt32LE(Number(row[9]), 68);
    raw.writeUInt32LE(Number(row[10]), 72); raw.writeUInt32LE(Number(row[11]), 76);
    expect(decodeHeader(raw, row[5]).hash).toBe(row[5]);
    const source = [...inputs.headers, ...inputs.bodies].find(source => source.hash === row[5]);
    expect(raw.toString("hex")).toBe(source?.raw.slice(0, 160));
  }
});

test("every rendered TX IN OUT and WITNESS field matches complete accepted body decoding", () => {
  // Arrange
  const records = renderBip30Fixtures(inputs).split("\n").map(line => line.split("\t"));
  let caseHeight = 0; let count = 0;
  // Act / Assert
  for (const row of records) {
    if (row[0] === "CASE") caseHeight = Number(row[2]);
    if (!["TX", "IN", "OUT", "WITNESS"].includes(row[0])) continue;
    const height = row[1] === "repeat" ? caseHeight : caseHeight === 91842 ? 91812 : 91722;
    const source = inputs.bodies.find(body => body.height === height);
    if (!source) throw new Error("missing test body");
    const tx = decodeBlock(Buffer.from(source.raw, "hex"), source.hash).transactions[Number(row[2])];
    const bytes = Buffer.from(tx.raw, "hex");
    if (row[0] === "TX") expect(row.slice(3)).toEqual([String(bytes.readInt32LE()), String(bytes.readUInt32LE(bytes.length - 4)),
      tx.txid, tx.wtxid, String(tx.inputs.length), String(tx.outputs.length)]);
    if (row[0] === "IN") {
      const input = tx.inputs[Number(row[3])]; const witness = tx.witness[Number(row[3])] ?? [];
      expect(row.slice(4)).toEqual([input.txid, String(input.vout), input.script || "-", String(input.sequence), String(witness.length)]);
      expect(tx.coinbase).toBe(input.txid === "0".repeat(64) && input.vout === 4294967295);
    }
    if (row[0] === "OUT") {
      const output = tx.outputs[Number(row[3])]; expect(row.slice(4)).toEqual([output.value, output.script || "-"]);
    }
    if (row[0] === "WITNESS") expect(row[5]).toBe(tx.witness[Number(row[3])][Number(row[4])] || "-");
    count++;
  }
  expect(count).toBe(20);
});

function editRecord(text: string, tag: string, edit: (fields: string[]) => string[]) {
  const rows = text.trimEnd().split("\n"); const index = rows.findIndex(line => line.startsWith(tag + "\t"));
  if (index < 0) throw new Error("missing actual record: " + tag);
  rows[index] = edit(rows[index].split("\t")).join("\t"); return rows.join("\n") + "\n";
}
const textMutations: [string, (text: string) => string, string][] = [
  ["magic version", text => text.replace("BIP30\t1\n", "BIP30\t2\n"), "magic/version"],
  ["source digest", text => editRecord(text, "SOURCE", fields => { fields[2] = "0".repeat(64); return fields; }), "source digest/path"],
  ["unknown tag", text => text.replace("END\t2\n", "EVALUATE\t2\n"), "tag"],
  ["wrong arity", text => text.replace("END\t2\n", "END\t2\t0\n"), "arity"],
  ["wrong network", text => editRecord(text, "CASE", fields => { fields[1] = "testnet3"; return fields; }), "literal/role"],
  ["reversed pair", text => editRecord(text, "CASE", fields => { fields[4] = "91722"; return fields; }), "record mismatch"],
  ["signed i32 overflow", text => editRecord(text, "TX", fields => { fields[3] = "2147483648"; return fields; }), "integer field range"],
  ["negative u32", text => editRecord(text, "IN", fields => { fields[5] = "-1"; return fields; }), "integer field range"],
  ["i64 overflow", text => editRecord(text, "CASE", fields => { fields[10] = "9223372036854775808"; return fields; }), "integer field range"],
  ["Amount overflow", text => editRecord(text, "OUT", fields => { fields[4] = "2100000000000001"; return fields; }), "integer field range"],
  ["noncanonical number", text => editRecord(text, "TX", fields => { fields[2] = "00"; return fields; }), "integer"],
  ["negative zero", text => editRecord(text, "TX", fields => { fields[3] = "-0"; return fields; }), "integer"],
  ["uppercase hash", text => editRecord(text, "TX", fields => { fields[5] = fields[5].toUpperCase(); return fields; }), "lowercase hex"],
  ["odd script", text => editRecord(text, "IN", fields => { fields[6] = "1"; return fields; }), "lowercase hex"],
  ["empty script without dash", text => editRecord(text, "IN", fields => { fields[6] = ""; return fields; }), "empty bytes"],
  ["script escape", text => editRecord(text, "OUT", fields => { fields[5] = "\\x51"; return fields; }), "lowercase hex"],
  ["wrong ordinal", text => editRecord(text, "HEADER", fields => { fields[2] = "1"; return fields; }), "record mismatch"],
  ["modified header scalar", text => editRecord(text, "HEADER", fields => { fields[11] = String(Number(fields[11]) + 1); return fields; }), "record mismatch"],
  ["duplicate original role transaction", text => text.replace("TX\toriginal\t1\t", "TX\toriginal\t0\t"), "record mismatch"],
  ["missing external Coin", text => text.split("\n").filter(line => !line.startsWith("COIN\texternal\t")).join("\n"), "record count"],
  ["swapped header rows", text => {
    const rows = text.split("\n"); const i = rows.findIndex(row => row.startsWith("HEADER\thistory\t0\t"));
    [rows[i], rows[i + 1]] = [rows[i + 1], rows[i]]; return rows.join("\n");
  }, "record mismatch"],
  ["omitted transaction", text => text.split("\n").filter(line => !line.startsWith("TX\trepeat\t1\t")).join("\n"), "record count"],
  ["duplicate record", text => text.replace("END\t2\n", "END\t2\nEND\t2\n"), "record count"],
  ["truncated record", text => text.slice(0, -1), "ASCII/LF"],
  ["modified old Coin", text => editRecord(text, "COIN", fields => { fields[8] = "1"; return fields; }), "record mismatch"],
  ["comment after records", text => text.replace("END\t2\n", "# extra\nEND\t2\n"), "documentation"],
  ["CRLF", text => text.replaceAll("\n", "\r\n"), "ASCII/LF"],
  ["non-ASCII", text => text + "é\n", "ASCII/LF"],
  ["line overflow", text => text.replace("END\t2\n", "#".repeat(BOUNDS.line + 1) + "\n"), "line bound"],
  ["record overflow", text => text + "END\t2\n".repeat(BOUNDS.records), "record bound"],
  ["byte overflow", text => text + "x".repeat(BOUNDS.bytes), "byte bound"],
];
for (const [label, mutate, reason] of textMutations) test(`actual rendered corpus refuses ${label}`, () => {
  // Arrange
  const expected = renderBip30Fixtures(inputs); const changed = mutate(expected);
  // Act / Assert
  expect(changed).not.toBe(expected);
  expect(() => validateProjectionText(changed, expected)).toThrow(reason);
});

function temporaryRoot(): string {
  const root = mkdtempSync(resolve(tmpdir(), "phase167-projection-"));
  for (const directory of ["scripts", "docs/parity/fixtures", "packages/testdata/phase167"])
    mkdirSync(resolve(root, directory), { recursive: true });
  symlinkSync(resolve(import.meta.dir, "../packages/bitcoin-knots"), resolve(root, "packages/bitcoin-knots"));
  for (const [file] of FIXED_INPUTS) symlinkSync(resolve(import.meta.dir, "..", file), resolve(root, file));
  return root;
}
test("explicit writes are deterministic and actual checks stay offline without repairing output", () => {
  // Arrange
  const root = temporaryRoot(); const originalFetch = globalThis.fetch;
  globalThis.fetch = Object.assign(() => { throw new Error("network attempted by offline projection"); }, { preconnect: originalFetch.preconnect });
  try {
    // Act
    const first = runProjection(["--write"], root); const expected = readFileSync(resolve(root, OUTPUT), "utf8");
    const second = runProjection(["--write"], root); const beforeCheck = statSync(resolve(root, OUTPUT)).mtimeMs;
    // Assert
    expect(second).toEqual(first); expect(readFileSync(resolve(root, OUTPUT), "utf8")).toBe(expected);
    expect(runProjection(["--check"], root)).toEqual(first);
    expect(statSync(resolve(root, OUTPUT)).mtimeMs).toBe(beforeCheck);
    const corrupt = expected.replace("END\t2\n", "END\t1\n"); writeFileSync(resolve(root, OUTPUT), corrupt);
    expect(() => runProjection(["--check"], root)).toThrow("record mismatch");
    expect(readFileSync(resolve(root, OUTPUT), "utf8")).toBe(corrupt);
    unlinkSync(resolve(root, OUTPUT));
    expect(() => runProjection(["--check"], root)).toThrow("ENOENT");
    expect(() => statSync(resolve(root, OUTPUT))).toThrow("ENOENT");
  } finally { globalThis.fetch = originalFetch; rmSync(root, { recursive: true }); }
}, 120_000);

test("source-digest refusal on explicit write preserves the existing output exactly", () => {
  // Arrange
  const root = temporaryRoot(); const file = resolve(root, "scripts/phase167-historical-corpus.ts");
  const raw = readFileSync(file); raw[raw.length - 1] = 32;
  unlinkSync(file); writeFileSync(file, raw); writeFileSync(resolve(root, OUTPUT), "retained output\n");
  try {
    // Act / Assert
    expect(() => runProjection(["--write"], root)).toThrow("fixed source digest");
    expect(readFileSync(resolve(root, OUTPUT), "utf8")).toBe("retained output\n");
  } finally { rmSync(root, { recursive: true }); }
}, 120_000);

test("accepted data-digest refusal on explicit write preserves output without repairing inputs", () => {
  // Arrange
  const root = temporaryRoot(); const file = resolve(root, "packages/testdata/phase167/blocks.json");
  const raw = readFileSync(file); raw[raw.length - 1] = 32;
  unlinkSync(file); writeFileSync(file, raw); writeFileSync(resolve(root, OUTPUT), "previous output\n");
  try {
    // Act / Assert
    expect(() => runProjection(["--write"], root)).toThrow("corpus digest");
    expect(readFileSync(resolve(root, OUTPUT), "utf8")).toBe("previous output\n");
    expect(readFileSync(file).equals(raw)).toBe(true);
  } finally { rmSync(root, { recursive: true }); }
}, 120_000);

for (const args of [[], ["--acquire"], ["--write", "--check"], ["--check", "--acquire"], ["--help"], ["--output", "/tmp/elsewhere"]])
  test(`mode refusal precedes all I/O: ${args.join(" ") || "absent"}`, () => {
    expect(() => runProjection(args, "/nonexistent-phase167-root")).toThrow("exactly --check or --write");
  });

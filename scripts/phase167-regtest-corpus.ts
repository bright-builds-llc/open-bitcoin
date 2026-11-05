/** Pinned-source corpus orchestration and ledger; never admission authority. */
import {
  ZERO,
  hash,
  sha,
  bytes,
  hex256,
  display,
  encodeTransaction,
  txid,
  wtxid,
  blockHash,
  encodeHeader,
  encodeBlock,
  proof,
  validPow,
  seal,
  heightScript,
  commitWitness,
  median,
  genesis,
  BITS,
  assertFrozenJsonBytes,
  type Tx,
  type Output,
  type Body,
} from "./phase167-regtest-serialization";

import { execFileSync, spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";

const ROOT = resolve(import.meta.dir, "..");
const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
const HISTORY = "packages/testdata/phase167/regtest-history.json";
const EXPECTATIONS = "docs/parity/fixtures/phase167-regtest-expectations.json";
const MATURITY = 100;
const HALVING = 150;
const TIME_FLAG = 1 << 22;
type Coin = Output & {
  txid: string;
  vout: number;
  coinbase: boolean;
  height: number;
  creating_parent_mtp: number;
};
const clone = <T>(value: T): T => structuredClone(value);
const subsidy = (height: number) =>
  5_000_000_000n >> BigInt(Math.floor(height / HALVING));

function coinbase(height: number, fee: bigint): Tx {
  const reward = subsidy(height) + fee;
  const outputs =
    height === 1
      ? [
          { sats: "2000000000", script: "51" },
          { sats: "2000000000", script: "51" },
          {
            sats: "1000000000",
            script: `0020${hash(bytes("51")).toString("hex")}`,
          },
        ]
      : [{ sats: reward.toString(), script: "51" }];
  return {
    version: 1,
    lock_time: height,
    inputs: [
      {
        txid: ZERO,
        vout: 0xffffffff,
        script: heightScript(height),
        sequence: 0xffffffff,
        witness: [],
      },
    ],
    outputs,
  };
}

function spend(
  first: Tx,
  vout: number,
  fee: bigint,
  sequence = 0xffffffff,
  witness: string[] = [],
): Tx {
  return {
    version: 2,
    lock_time: 0,
    inputs: [{ txid: txid(first), vout, script: "", sequence, witness }],
    outputs: [
      {
        sats: (BigInt(first.outputs[vout].sats) - fee).toString(),
        script: "51",
      },
    ],
  };
}

async function sourceInputs(root: string) {
  const upstream = resolve(root, "packages/bitcoin-knots");
  if (
    execFileSync("git", ["-C", upstream, "rev-parse", "HEAD"], {
      encoding: "utf8",
    }).trim() !== PIN
  )
    throw Error("Knots pin mismatch");
  const paths = [
    "src/kernel/chainparams.cpp",
    "src/validation.cpp",
    "src/consensus/consensus.h",
    "src/consensus/tx_verify.cpp",
    "src/coins.cpp",
    "src/pow.cpp",
    "src/chain.cpp",
    "src/blockfilter.cpp",
    "src/util/golombrice.h",
    "test/functional/test_framework/crypto/siphash.py",
    "test/functional/test_framework/messages.py",
  ];
  const sources = paths.map((relative) => {
    const live = execFileSync(
      "git",
      ["-C", upstream, "show", `${PIN}:${relative}`],
      { maxBuffer: 2_000_000 },
    );
    return {
      path: `packages/bitcoin-knots/${relative}`,
      sha256: sha(live),
      pinned: live,
    };
  });
  for (const row of sources)
    if (
      !Buffer.from(await Bun.file(resolve(root, row.path)).bytes()).equals(
        row.pinned,
      )
    )
      throw Error("dirty pinned source refused");
  const chainparams = sources[0].pinned.toString();
  const regtest = chainparams.slice(
    chainparams.indexOf("class CRegTestParams"),
  );
  for (const anchor of [
    "consensus.nSubsidyHalvingInterval = 150",
    "consensus.BIP34Height = 1",
    "consensus.CSVHeight = 1",
    "consensus.SegwitHeight = 0",
    "consensus.fPowNoRetargeting = true",
    "CreateGenesisBlock(1296688602, 2, 0x207fffff, 1, 50 * COIN)",
  ])
    if (!regtest.includes(anchor))
      throw Error("default regtest constants changed");
  if (!sources[2].pinned.toString().includes("COINBASE_MATURITY = 100"))
    throw Error("default maturity changed");
  return {
    chainparams,
    sources: sources.map(({ path, sha256 }) => ({ path, sha256 })),
    oracle: {
      path: "scripts/basic-filter-oracle.py",
      sha256: sha(
        await Bun.file(resolve(root, "scripts/basic-filter-oracle.py")).bytes(),
      ),
    },
  };
}

function negatives(chain: Body[], timeUnits: number) {
  const cases: {
    name: string;
    height: number;
    expected_reason: string;
    body: Body;
    evidence_class: string;
  }[] = [];
  const add = (
    name: string,
    height: number,
    reason: string,
    mutate: (body: Body) => void,
  ) => {
    const body = clone(chain[height]);
    mutate(body);
    seal(body);
    cases.push({
      name,
      height,
      expected_reason: reason,
      body,
      evidence_class: "pinned-source-expected-not-executed",
    });
  };
  for (const height of [149, 150, 299, 300])
    add(`overpay-one-satoshi-${height}`, height, "bad-cb-amount", (body) => {
      body.transactions[0].outputs[0].sats = (
        BigInt(body.transactions[0].outputs[0].sats) + 1n
      ).toString();
    });
  add("depth99-at100", 100, "bad-txns-premature-spend-of-coinbase", (body) => {
    body.transactions.push(spend(chain[1].transactions[0], 0, 1000n));
  });
  add("wrong-version", 149, "bad-version(0x00000003)", (body) => {
    body.header.version = 3;
  });
  add("wrong-difficulty", 149, "bad-diffbits", (body) => {
    body.header.bits = 0x207ffffe;
  });
  add("time-equals-parent-mtp", 149, "time-too-old", (body) => {
    body.header.time = median(
      chain.slice(138, 149).map((block) => block.header.time),
    );
  });
  add("wrong-bip34-height", 150, "bad-cb-height", (body) => {
    body.transactions[0].inputs[0].script = heightScript(151);
  });
  add("absolute-nonfinal", 101, "bad-txns-nonfinal", (body) => {
    body.transactions[1].lock_time = 101;
    body.transactions[1].inputs[0].sequence = 0xfffffffe;
  });
  add("bip68-height-nonfinal", 101, "bad-txns-nonfinal", (body) => {
    body.transactions[1].inputs[0].sequence = 101;
  });
  add("bip68-time-nonfinal", 201, "bad-txns-nonfinal", (body) => {
    body.transactions[1].inputs[0].sequence = TIME_FLAG | (timeUnits + 1);
  });
  add("witness-without-commitment", 101, "unexpected-witness", (body) => {
    body.transactions[1].inputs[0].witness = ["51"];
  });
  add("witness-reserve-31", 151, "bad-witness-nonce-size", (body) => {
    body.transactions[0].inputs[0].witness = ["00".repeat(31)];
  });
  add("witness-reserve-33", 151, "bad-witness-nonce-size", (body) => {
    body.transactions[0].inputs[0].witness = ["00".repeat(33)];
  });
  add(
    "witness-commitment-mismatch",
    151,
    "bad-witness-merkle-match",
    (body) => {
      body.transactions[1].inputs[0].witness = ["5151"];
    },
  );
  add("missing-input", 101, "bad-txns-inputs-missingorspent", (body) => {
    body.transactions[1].inputs[0].txid = "11".repeat(32);
  });
  add("aggregate-sigops-80004", 101, "bad-blk-sigops", (body) => {
    body.transactions[0].outputs[0].script = "ac".repeat(10000);
    body.transactions[1].outputs[0].script = "ac".repeat(10000);
    const second = spend(chain[1].transactions[0], 1, 2000n);
    second.outputs[0].script = "ac";
    body.transactions.push(second);
    body.transactions[0].outputs[0].sats = (subsidy(101) + 3000n).toString();
  });
  const invalidPow = clone(chain[149]);
  while (validPow(invalidPow)) invalidPow.header.nonce++;
  cases.push({
    name: "invalid-pow",
    height: 149,
    expected_reason: "high-hash",
    body: invalidPow,
    evidence_class: "pinned-source-expected-not-executed",
  });
  return cases.map(({ body, ...row }) => ({
    ...row,
    parent_height: row.height - 1,
    isolated_known_identity_bound: row.height + 1,
    block_hex: encodeBlock(body).toString("hex"),
    block_hash_raw: blockHash(body),
    pow_satisfied: validPow(body),
  }));
}

function ledgerFacts(chain: Body[]) {
  const utxos = new Map<string, Coin>();
  const oracleCases: unknown[] = [];
  const facts = chain.map((body, height) => {
    const parentMtp = height
      ? median(
          chain
            .slice(Math.max(0, height - 11), height)
            .map((block) => block.header.time),
        )
      : body.header.time;
    const spent: Coin[] = [];
    let fee = 0n;
    const transactionFacts = body.transactions.map((tx, index) => {
      let incoming = 0n;
      if (index)
        for (const input of tx.inputs) {
          const key = `${input.txid}:${input.vout}`;
          const maybeCoin = utxos.get(key);
          if (
            !maybeCoin ||
            (maybeCoin.coinbase && height - maybeCoin.height < MATURITY)
          )
            throw Error("independent missing/immature input");
          incoming += BigInt(maybeCoin.sats);
          spent.push(maybeCoin);
          utxos.delete(key);
        }
      const outgoing = tx.outputs.reduce(
        (sum, output) => sum + BigInt(output.sats),
        0n,
      );
      if (index) {
        if (outgoing > incoming) throw Error("negative fee");
        fee += incoming - outgoing;
      }
      const identity = txid(tx);
      if (height)
        tx.outputs.forEach((output, vout) => {
          if (
            !output.script.startsWith("6a") &&
            output.script.length / 2 <= 10000
          ) {
            const key = `${identity}:${vout}`;
            if (utxos.has(key))
              throw Error("independent live outpoint collision");
            utxos.set(key, {
              ...output,
              txid: identity,
              vout,
              coinbase: index === 0,
              height,
              creating_parent_mtp: parentMtp,
            });
          }
        });
      return {
        txid_raw: identity,
        txid_display: display(identity),
        wtxid_raw: wtxid(tx),
        transaction_hex: encodeTransaction(tx).toString("hex"),
        stripped_hex: encodeTransaction(tx, false).toString("hex"),
        coinbase: index === 0,
        inputs: tx.inputs,
        outputs: tx.outputs,
        version: tx.version,
        lock_time: tx.lock_time,
      };
    });
    const reward = body.transactions[0].outputs.reduce(
      (sum, output) => sum + BigInt(output.sats),
      0n,
    );
    if (reward !== subsidy(height) + fee)
      throw Error("independent reward mismatch");
    const raw = blockHash(body);
    oracleCases.push({
      name: `height-${height}`,
      previous_name: height ? `height-${height - 1}` : "",
      raw_block: encodeBlock(body).toString("hex"),
      block_hash_raw: raw,
      outputs: [],
      spent: spent.map((coin) => coin.script),
      predecessor_raw: ZERO,
    });
    return {
      height,
      header_hex: encodeHeader(body.header).toString("hex"),
      block_hash_raw: raw,
      block_hash_display: display(raw),
      parent_hash_raw: body.header.parent,
      time: body.header.time,
      parent_mtp: parentMtp,
      median_time_past: median(
        chain
          .slice(Math.max(0, height - 10), height + 1)
          .map((block) => block.header.time),
      ),
      bits: BITS,
      nonce: body.header.nonce,
      subsidy_sats: subsidy(height).toString(),
      fees_sats: fee.toString(),
      coinbase_sats: reward.toString(),
      proof_hex: hex256(proof(BITS)),
      chain_work_hex: hex256(BigInt(height + 1) * proof(BITS)),
      transactions: transactionFacts,
      spent,
      utxo_count: utxos.size,
      utxo_total_sats: [...utxos.values()]
        .reduce((sum, coin) => sum + BigInt(coin.sats), 0n)
        .toString(),
      utxos: [...utxos.values()].sort(
        (a, b) => a.txid.localeCompare(b.txid) || a.vout - b.vout,
      ),
    };
  });
  return { facts, oracleCases };
}

function basicFacts(
  root: string,
  oracleCases: unknown[],
  blockHashes: string[],
) {
  const oracle = spawnSync("python3", ["scripts/basic-filter-oracle.py"], {
    cwd: root,
    input: JSON.stringify({ cases: oracleCases }),
    encoding: "utf8",
    timeout: 30_000,
    maxBuffer: 4_000_000,
  });
  if (oracle.error || oracle.status !== 0)
    throw Error("source-audited BASIC oracle failure", {
      cause: oracle.error ?? oracle.stderr,
    });
  const basic = (
    JSON.parse(oracle.stdout) as {
      cases: { block_hash_raw: string; [key: string]: unknown }[];
    }
  ).cases;
  if (
    basic.length !== 303 ||
    basic.some((row, height) => row.block_hash_raw !== blockHashes[height])
  )
    throw Error("BASIC full body correspondence");
  return basic;
}

/** Pure deterministic derivation plus the existing source-audited BASIC oracle. */
export async function buildCorpus(root = ROOT) {
  const source = await sourceInputs(root);
  const chain = [genesis(source.chainparams)];
  const creatingMtp = chain[0].header.time;
  let timeUnits = 0;
  for (let height = 1; height <= 302; height++) {
    const transactions: Tx[] = [];
    let fee = 0n;
    if (height === 101) {
      fee = 1000n;
      transactions.push(spend(chain[1].transactions[0], 0, fee));
    }
    if (height === 151) {
      fee = 3000n;
      transactions.push(
        spend(chain[1].transactions[0], 2, fee, 0xffffffff, ["51"]),
      );
    }
    if (height === 201) {
      const parentMtp = median(
        chain.slice(-11).map((block) => block.header.time),
      );
      timeUnits = Math.floor((parentMtp - creatingMtp) / 512);
      fee = 2000n;
      transactions.push(
        spend(chain[1].transactions[0], 1, fee, TIME_FLAG | timeUnits),
      );
    }
    const parent = chain[height - 1];
    const body = {
      header: {
        version: 4,
        parent: blockHash(parent),
        merkle: ZERO,
        time: parent.header.time + 600,
        bits: BITS,
        nonce: 0,
      },
      transactions: [coinbase(height, fee), ...transactions],
    };
    if (height === 151) commitWitness(body);
    chain.push(seal(body));
  }
  const { facts, oracleCases } = ledgerFacts(chain);
  const basic = basicFacts(
    root,
    oracleCases,
    facts.map((row) => row.block_hash_raw),
  );
  const fork = chain.slice(0, 194).map(clone);
  for (let height = 194; height <= 201; height++) {
    const body = clone(chain[height]);
    body.header.parent = blockHash(fork[height - 1]);
    body.header.time = fork[height - 1].header.time + 1;
    fork.push(seal(body));
  }
  const forkMtp = median(fork.slice(190, 201).map((body) => body.header.time));
  const minTime = creatingMtp + timeUnits * 512 - 1;
  if (!(
    facts[201].parent_mtp > minTime &&
    forkMtp <= minTime &&
    forkMtp !== facts[201].parent_mtp
  ))
    throw Error("independent genuine branch disagreement missing");
  const serial = chain.map((body, height) => ({
    height,
    block_hex: encodeBlock(body).toString("hex"),
    block_hash_raw: facts[height].block_hash_raw,
    parent_hash_raw: body.header.parent,
  }));
  const history = {
    format: "phase167-regtest-history-v1",
    evidence_class: "independent-source-derived-bytes-not-authority",
    source_pin: {
      commit: PIN,
      sources: source.sources,
      basic_oracle: source.oracle,
    },
    constants: {
      coinbase_maturity: MATURITY,
      subsidy_halving_interval: HALVING,
      pow_bits: BITS,
      no_retargeting: true,
      bip34_height: 1,
      csv_height: 1,
      segwit_height: 0,
    },
    rows: serial.slice(0, 302),
    next_302: serial[302],
    negatives: negatives(chain, timeUnits),
    branch_mtp_scenario: {
      isolated_prefix_last_height: 193,
      independent_known_identities: 202,
      rows: fork.slice(194).map((body, index) => ({
        height: index + 194,
        block_hex: encodeBlock(body).toString("hex"),
        block_hash_raw: blockHash(body),
      })),
      candidate_height: 201,
      expected_reason: "bad-txns-nonfinal",
      same_transaction_hex_as_main_201: encodeTransaction(
        chain[201].transactions[1],
      ).toString("hex"),
      parent_mtp: forkMtp,
    },
  };
  const expectations = {
    format: "phase167-regtest-expectations-v1",
    evidence_class:
      "source-derived-formulas-and-existing-pinned-BASIC-oracle-not-daemon",
    source_pin: history.source_pin,
    constants: history.constants,
    rows: facts
      .slice(0, 302)
      .map((row, height) => ({ ...row, basic: basic[height] })),
    next_302: { ...facts[302], basic: basic[302] },
    creating_live_201: {
      reserved_outpoint: `${txid(chain[1].transactions[0])}:1`,
      creating_height: 1,
      creating_parent_height: 0,
      creating_parent_mtp: creatingMtp,
      candidate_parent_mtp: facts[201].parent_mtp,
      version: 2,
      sequence: TIME_FLAG | timeUnits,
      sequence_units: timeUnits,
      granularity_seconds: 512,
      last_invalid_time: minTime,
      valid: minTime < facts[201].parent_mtp,
      context_suffix_for_plan16: {
        first_height: 46,
        last_height: 200,
        count: 155,
        lacks_creating_parent_zero: true,
      },
      kernel_probe_execution_owner: "Plan16Task1-not-executed",
    },
    boundaries: [149, 150, 299, 300].map((height) => ({
      height,
      subsidy_sats: subsidy(height).toString(),
      fees_sats: facts[height].fees_sats,
    })),
    genesis_utxo_count: 0,
    final_utxos_301: facts[301].utxos,
    unique_transaction_count_301: facts
      .slice(0, 302)
      .reduce((sum, row) => sum + row.transactions.length, 0),
    no_adapter_or_daemon_execution: true,
  };
  return { history, expectations };
}

/** Frozen comparison reproduces bytes/facts independently of product Rust. */
export async function checkCorpus(
  history: unknown,
  expectations: unknown,
  root = ROOT,
): Promise<void> {
  const actual = await buildCorpus(root);
  if (
    !isDeepStrictEqual(history, actual.history) ||
    !isDeepStrictEqual(expectations, actual.expectations)
  )
    throw Error("regtest corpus/source expectations mismatch");
}

if (import.meta.main) {
  const mode = process.argv.slice(2);
  if (mode.length !== 1 || !["--write", "--check"].includes(mode[0]))
    throw Error("expected --write or --check");
  if (mode[0] === "--check") {
    const actual = await buildCorpus();
    assertFrozenJsonBytes(
      await Bun.file(resolve(ROOT, HISTORY)).bytes(),
      actual.history,
    );
    assertFrozenJsonBytes(
      await Bun.file(resolve(ROOT, EXPECTATIONS)).bytes(),
      actual.expectations,
    );
    console.log(
      "source-derived regtest corpus matches frozen bytes; no runtime authority implied",
    );
  } else {
    const generated = await buildCorpus();
    await Bun.write(
      resolve(ROOT, HISTORY),
      JSON.stringify(generated.history, null, 2) + "\n",
    );
    await Bun.write(
      resolve(ROOT, EXPECTATIONS),
      JSON.stringify(generated.expectations, null, 2) + "\n",
    );
  }
}

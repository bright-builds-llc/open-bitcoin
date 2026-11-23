import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { buildOpenBitcoinCli, sha256File } from "./build.ts";
import { CHECKPOINT_FORMAT, checkCheckpointReceipt, readCheckpointCorpus } from "./checkpoint-recovery.ts";
import { loggedCommand } from "./process.ts";
import { qualify } from "./receipt.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";

export async function runCheckpointQualification(root: string, run: string) {
  const readText = (path: string) => readFileSync(join(root, path), "utf8");
  return qualify(run, async (stage) => {
    const corpus = readCheckpointCorpus(readText);
    stage("operator_build");
    const cli = await buildOpenBitcoinCli(root, run, "CREC-04");
    stage("native_matrix");
    const report = join(run, "checkpoint-cases.json");
    const log = join(run, "checkpoint-matrix.log");
    await loggedCommand(root, [process.execPath, "run", "scripts/command-timings.ts", "run", "--key", "crec04-checkpoint-qualification", "--",
      "cargo", "test", "--locked", "--manifest-path", "packages/Cargo.toml", "-p", "open-bitcoin-node", "--lib",
      "crec04_reopens_proved_prefix_or_preserves_refused_source", "--", "--nocapture"], log, {
      ...process.env, OPEN_BITCOIN_CREC04_REPORT: report, OPEN_BITCOIN_CREC04_CLI: cli.executable,
    });
    if (statSync(report).size > 1_048_576 || await sha256File(cli.executable) !== cli.build.binary_sha256) {
      throw new Error("Checkpoint report bounds or operator executable changed");
    }
    return {
      format: CHECKPOINT_FORMAT, status: "passed", recorded_at: new Date().toISOString(), bun: Bun.version,
      network: "regtest", corpus: REGTEST_CORPUS, corpus_sha256: corpus.corpusSha256,
      fixture_sha256: corpus.fixtureSha256, native_log_sha256: await sha256File(log),
      cli_build: cli.build, cases: JSON.parse(readFileSync(report, "utf8")),
    };
  }, (record) => checkCheckpointReceipt(record, readText), CHECKPOINT_FORMAT);
}

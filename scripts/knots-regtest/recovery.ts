import { isDeepStrictEqual } from "node:util";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import { readRecoveryCorpus } from "./replacement-spends.ts";
import type { RegtestTip } from "./regtest-history.ts";

export const RECOVERY_FORMAT = "open-bitcoin-crec-03-v1";
export const RECOVERY_ID = "v2-6-crec-03";

type RecordValue = Record<string, unknown>;

function object(value: unknown): RecordValue | null {
  return value && typeof value === "object" && !Array.isArray(value) ? value as RecordValue : null;
}

function hex256(value: unknown): value is string {
  return typeof value === "string" && /^[a-f0-9]{64}$/.test(value);
}

function matchesBasic(value: unknown, tip: RegtestTip): boolean {
  return Array.isArray(value) && value.length === 3 && value[0] === tip.hash
    && typeof value[1] === "string" && value[1].length > 0 && value[1].length <= 65_536
    && value[1].length % 2 === 0 && /^[a-f0-9]+$/.test(value[1]) && hex256(value[2]);
}

function sameTip(value: unknown, tip: RegtestTip) {
  return isDeepStrictEqual(value, { height: tip.height, hash: tip.hash });
}

function checkStoppedState(value: unknown, tip: RegtestTip, chainwork: string, utxos: unknown) {
  const observation = object(value);
  const cli = object(observation?.cli);
  return observation && cli?.exit_status === 0 && sameTip(cli.tip, tip)
    && cli.chainwork === chainwork && cli.basic_height === tip.height
    && observation.chainwork === chainwork && isDeepStrictEqual(observation.utxos, utxos);
}

function buildMatches(value: RecordValue | null, crate: string, executable: string) {
  return value && hex256(value.binary_sha256) && hex256(value.build_log_sha256)
    && isDeepStrictEqual(value.cargo_options, [
      "--locked", "--manifest-path", "packages/Cargo.toml", "-p", crate, "--bin", executable,
    ]);
}

function daemonMatches(value: unknown, initial: RegtestTip, final: RegtestTip): value is RecordValue {
  const live = object(value);
  return !!live && Array.isArray(live.flags) && ["-chain=regtest", "-rpcbind=127.0.0.1", "-blockfilterindex=1"]
    .every((flag) => (live.flags as unknown[]).includes(flag))
    && live.peer_count === 0 && live.exit_status === 0 && sameTip(live.initial_tip, initial)
    && sameTip(live.final_tip, final) && matchesBasic(live.basic, final);
}

export function checkRecoveryReceipt(
  value: unknown, readText: (path: string) => string,
): string[] {
  const record = object(value);
  const corpus = readRecoveryCorpus(readText);
  const { replacement: held, resumed, spend } = corpus;
  const daemon = object(record?.daemon_build);
  const cli = object(record?.cli_build);
  if (!record || record.format !== RECOVERY_FORMAT || record.status !== "passed"
    || record.network !== "regtest" || record.corpus !== REGTEST_CORPUS || record.corpus_sha256 !== spend.corpusSha256
    || record.spend_sha256 !== spend.spendSha256 || record.resume_sha256 !== corpus.resumeSha256
    || record.resumed_outpoint !== corpus.funding || record.open_bitcoin_observer_sha256 !== spend.observerSha256
    || typeof daemon?.source_commit !== "string" || !/^[a-f0-9]{40}$/.test(daemon.source_commit)
    || daemon.source_commit !== cli?.source_commit
    || !buildMatches(daemon, "open-bitcoin-rpc", "open-bitcoind")
    || !buildMatches(cli, "open-bitcoin-cli", "open-bitcoin")
    || daemon.binary_sha256 === cli.binary_sha256) {
    return [`${RECOVERY_ID}: recovery lacks bound runtime builds, inputs or observer`];
  }
  const failures: string[] = [];
  const start = { height: 0, hash: spend.genesisHash };
  const { initial, restart, continuation } = record;
  const winning = { height: held.end.height, hash: held.end.hash };
  if (!daemonMatches(initial, start, held.end) || initial.name !== "replacement"
    || !isDeepStrictEqual(initial.steps, [{ hash: held.end.hash, submitblock: null, active: winning }])) {
    failures.push(`${RECOVERY_ID}: initial daemon did not stop at the earned replacement tip`);
  }
  const refused = object(object(initial)?.live_cli);
  if (typeof refused?.exit_status !== "number" || !Number.isInteger(refused.exit_status)
    || refused.exit_status <= 0 || refused.error !== "chain_compatibility_refused") {
    failures.push(`${RECOVERY_ID}: live owner refusal is missing`);
  }
  if (!daemonMatches(restart, held.end, held.end) || restart.duplicate !== "duplicate"
    || !isDeepStrictEqual(restart.basic, object(initial)?.basic)) {
    failures.push(`${RECOVERY_ID}: restart lost tip, duplicate recognition or BASIC state`);
  }
  if (!daemonMatches(continuation, held.end, resumed) || continuation.submitblock !== null) {
    failures.push(`${RECOVERY_ID}: resumed daemon did not accept the recovered-output spend`);
  }
  for (const key of ["stopped", "reopened"] as const) {
    if (!checkStoppedState(record[key], held.end, held.chainwork, held.utxos)) {
      failures.push(`${RECOVERY_ID}: invalid ${key} CLI or durable coins observation`);
    }
  }
  if (!checkStoppedState(record.continued, resumed, corpus.resumedWork, corpus.resumedCoins)) {
    failures.push(`${RECOVERY_ID}: invalid continued CLI or durable coins observation`);
  }
  return failures;
}

import { readFile } from "node:fs/promises";
import {
  executeTimedCommand,
  readTimingRecords,
  recordTimingBatch,
  renderTimingReport,
  resolveStateRoot,
  summarizeRecords,
} from "./command-timings";

function takeOption(args: string[], name: string): string | undefined {
  const index = args.indexOf(name);
  if (index === -1) {
    return undefined;
  }
  const value = args[index + 1];
  if (value === undefined) {
    throw new Error(`${name} requires a value`);
  }
  args.splice(index, 2);
  return value;
}

export async function runCommandTimingCli(rawArguments: readonly string[]): Promise<number> {
  const [subcommand, ...rawArgs] = rawArguments;
  const args = [...rawArgs];
  if (subcommand === "run") {
    const separator = args.indexOf("--");
    if (separator === -1) {
      throw new Error("run requires -- before the command");
    }
    const optionArgs = args.slice(0, separator);
    const command = args.slice(separator + 1);
    const key = takeOption(optionArgs, "--key");
    const source = takeOption(optionArgs, "--source") ?? "ad-hoc";
    const verifyMode = takeOption(optionArgs, "--verify-mode") ?? null;
    if (key === undefined || optionArgs.length > 0) {
      throw new Error("run requires --key and accepts only --source/--verify-mode");
    }
    const record = await executeTimedCommand(command, { key, source, verifyMode });
    return record.exitStatus ?? 1;
  }

  if (subcommand === "report") {
    const maybeKey = takeOption(args, "--key");
    if (args.length > 0) {
      throw new Error("report accepts only --key");
    }
    const records = await readTimingRecords(resolveStateRoot(), maybeKey);
    console.log(renderTimingReport(summarizeRecords(records)));
    return 0;
  }

  if (subcommand === "record-batch") {
    const file = takeOption(args, "--file");
    const source = takeOption(args, "--source") ?? "verify";
    const verifyMode = takeOption(args, "--verify-mode") ?? null;
    if (file === undefined || args.length > 0) {
      throw new Error("record-batch requires --file and accepts --source/--verify-mode");
    }
    const entries = (await readFile(file, "utf8"))
      .split("\n")
      .filter((line) => line.length > 0)
      .map((line) => {
        const [key, startedAt, duration, status] = line.split("\t");
        if (
          key === undefined ||
          startedAt === undefined ||
          duration === undefined ||
          status === undefined
        ) {
          throw new Error("invalid timing batch row");
        }
        return {
          key,
          startedAtMs: Number(startedAt),
          durationMs: Number(duration),
          exitStatus: Number(status),
        };
      });
    await recordTimingBatch(entries, { key: "verify-step", source, verifyMode });
    return 0;
  }

  console.error(
    "usage: command-timings.ts run --key KEY [--source SOURCE] -- COMMAND [ARGS...]\n" +
      "       command-timings.ts report [--key KEY]",
  );
  return 2;
}

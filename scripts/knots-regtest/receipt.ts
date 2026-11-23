import { renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export const QUALIFICATION_FORMAT = "open-bitcoin-cdif-05-v1";

export async function qualify(
  run: string,
  exercise: (stage: (value: string) => void) => Promise<Record<string, unknown>>,
  validate: (record: Record<string, unknown>) => string[],
) {
  let stage = "inputs";
  try {
    const record = await exercise((value) => { stage = value; });
    stage = "validation";
    if (record.format !== QUALIFICATION_FORMAT || record.status !== "passed") {
      throw new Error("Qualification did not yield a complete passing record");
    }
    const failures = validate(record);
    if (failures.length) throw new Error(failures.join("; "));
    stage = "publish";
    const pending = join(run, "receipt.tmp");
    const receipt = join(run, "receipt.json");
    writeFileSync(pending, `${JSON.stringify(record, null, 2)}\n`, { flag: "wx", mode: 0o600 });
    renameSync(pending, receipt);
    return receipt;
  } catch (error) {
    let cause: unknown = error;
    while (cause instanceof Error && cause.name !== "TimeoutError" && cause.cause instanceof Error) {
      cause = cause.cause;
    }
    const timeout = cause instanceof Error && cause.name === "TimeoutError";
    const failure = { format: QUALIFICATION_FORMAT, status: "failed", stage,
      outcome: timeout ? "timeout" : "error" };
    writeFileSync(join(run, "failure.json"), `${JSON.stringify(failure, null, 2)}\n`, { flag: "wx", mode: 0o600 });
    throw error;
  }
}

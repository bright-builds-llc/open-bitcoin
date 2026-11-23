#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { checkCurrentParityLedger } from "./check-parity-ledger.ts";
import { currentQualificationInputs, QUALIFICATION_INPUT_FORMAT } from "./knots-regtest/qualification-inputs.ts";

export const DIFFERENTIALS = ["cdif-01", "cdif-02", "cdif-03", "cdif-04", "cdif-05", "crec-01", "crec-02"] as const;
export const BOUNDED_SCOPE = { network: "regtest", whole_chain: false, public_default: false, production_ready: false } as const;
const ROOT = resolve(import.meta.dir, "..");
const COMMIT = /^[0-9a-f]{40}$/;

type Receipt = {
  status?: unknown;
  network?: unknown;
  open_bitcoin_build?: { source_commit?: unknown };
  qualification_inputs?: { format?: unknown; source_commit?: unknown; sha256?: unknown };
};
type Surface = { id: string; status: string; differential_receipt?: string; qualification_scope?: unknown };

export function qualificationIssue(receipt: Receipt, expected: string): string | undefined {
  const binding = receipt.qualification_inputs;
  if (!binding) return "missing launch input binding";
  if (receipt.status !== "passed" || receipt.network !== "regtest"
    || binding.format !== QUALIFICATION_INPUT_FORMAT
    || typeof binding.source_commit !== "string" || !COMMIT.test(binding.source_commit)
    || binding.source_commit !== receipt.open_bitcoin_build?.source_commit) return "invalid run binding";
  if (binding.sha256 !== expected) return "qualification inputs changed";
  return undefined;
}

export function boundedQualificationScope(scope: unknown): boolean {
  if (!scope || typeof scope !== "object" || Array.isArray(scope)) return false;
  const fields = scope as Record<string, unknown>;
  return Object.keys(fields).length === Object.keys(BOUNDED_SCOPE).length
    && Object.entries(BOUNDED_SCOPE).every(([key, value]) => fields[key] === value);
}

function qualificationStatus(root: string) {
  const expected = currentQualificationInputs(root);
  const index: { checklist: { surfaces: Surface[] } } = JSON.parse(readFileSync(resolve(root, "docs/parity/index.json"), "utf8"));
  const owners = new Map(index.checklist.surfaces.map(surface => [surface.id, surface]));
  const closure = owners.get("v2-6-cpar-02");
  if (closure?.status === "done" && !boundedQualificationScope(closure.qualification_scope)) {
    throw new Error("CPAR-02 qualification requires the bounded Regtest claim scope");
  }
  const stale = DIFFERENTIALS.flatMap(id => {
    const file = `docs/parity/receipts/${id}-regtest.json`;
    const owner = owners.get(`v2-6-${id}`);
    if (owner?.status !== "done" || owner.differential_receipt !== file) return [{ id, reason: "missing qualifying owner or receipt" }];
    const receipt: Receipt = JSON.parse(readFileSync(resolve(root, file), "utf8"));
    const reason = qualificationIssue(receipt, expected);
    return reason ? [{ id, reason }] : [];
  });
  return { input_sha256: expected, current: DIFFERENTIALS.length - stale.length, stale };
}

if (import.meta.main) {
  try {
    const mode = Bun.argv[2];
    if (Bun.argv.length !== 3 || (mode !== "--status" && mode !== "--check")) {
      throw new Error("Usage: bun run scripts/check-v2-6-qualification.ts --status|--check");
    }
    const status = qualificationStatus(ROOT);
    if (mode === "--status") console.log(JSON.stringify(status, null, 2));
    else {
      const failures = [...checkCurrentParityLedger(ROOT), ...status.stale.map(row => `${row.id}: ${row.reason}`)];
      if (failures.length) throw new Error(failures.join("\n"));
      console.log(`CPAR-02 verified ${status.current} current actual Knots receipts for ${status.input_sha256}`);
    }
  } catch (error) {
    console.error(String(error));
    process.exitCode = 1;
  }
}

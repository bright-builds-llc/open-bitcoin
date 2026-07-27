import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { PHASE101_TEST_COMMAND, PHASE101_CHECKER_COMMAND, PHASE102_TEST_COMMAND, PHASE102_CHECKER_COMMAND, PURE_CORE_COMMAND, FORBIDDEN_CLAIMS, POSITIVE_CLAIM_PATTERNS, FORBIDDEN_VERIFIER_SCOPE, TextCorpus } from "./constants.ts";
import { requireContains, verifyOrderedCommands, executableVerifyText, hasNoClaimMarker } from "./helpers.ts";

export function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("verifier-scope missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [PHASE101_TEST_COMMAND, PHASE101_CHECKER_COMMAND, PHASE102_TEST_COMMAND, PHASE102_CHECKER_COMMAND],
      "verifier-scope visible order must place Phase 102 immediately after Phase 101",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    text,
    "Phase 101 is followed by Phase 102",
    "verifier-scope ordering comment missing Phase 102",
    failures,
  );
  requireContains(
    executableText,
    `run_step "test Phase 102 orphan admission bridge checker" ${PHASE102_TEST_COMMAND}`,
    "verifier-scope executable Phase 102 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 102 orphan admission bridge" ${PHASE102_CHECKER_COMMAND}`,
    "verifier-scope executable Phase 102 checker",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE101_TEST_COMMAND,
      PHASE101_CHECKER_COMMAND,
      PHASE102_TEST_COMMAND,
      PHASE102_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "verifier-scope executable order must run Phase 102 after Phase 101 and before pure-core checks",
    failures,
  );

  for (const line of executableText.split(/\r?\n/)) {
    const lower = line.toLowerCase();
    if (!lower.includes("phase 102") && !lower.includes("check-phase102")) {
      continue;
    }
    for (const forbidden of FORBIDDEN_VERIFIER_SCOPE) {
      if (lower.includes(forbidden)) {
        failures.push(`verifier-scope forbidden Phase 102 gate '${forbidden}': ${line.trim()}`);
      }
    }
  }
}

export function verifyNoClaimBoundary(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  const units = docs.replace(/\s+/g, " ").split(/(?<=[.!?])\s+/);
  for (const unit of units) {
    const lower = unit.toLowerCase();
    for (const claim of FORBIDDEN_CLAIMS) {
      if (!lower.includes(claim)) {
        continue;
      }
      if (hasNoClaimMarker(lower)) {
        continue;
      }
      if (POSITIVE_CLAIM_PATTERNS.some((pattern) => pattern.test(lower))) {
        failures.push(`no-claim boundary violation for ${claim}: ${unit.trim()}`);
      }
    }
  }
}

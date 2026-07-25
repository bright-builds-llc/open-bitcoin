#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-2-resource-time-fee-primitives";
const FEEP_REQUIREMENTS = [
  "FEEP-01",
  "FEEP-02",
  "FEEP-03",
  "FEEP-04",
  "FEEP-05",
] as const;

const PHASE129_CHECK =
  "bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts";
const PHASE130_TEST =
  "bun test scripts/check-phase130-resource-time-fee-primitives.test.ts";
const PHASE130_CHECK =
  "bun run scripts/check-phase130-resource-time-fee-primitives.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";

export const PHASE130_TARGET_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  ".planning/phases/130-resource-time-and-fee-primitives/130-09-SUMMARY.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/resource.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/types.rs",
  "packages/open-bitcoin-mempool/src/context.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "scripts/check-phase130-resource-time-fee-primitives.ts",
  "scripts/verify.sh",
] as const;

const PURE_POLICY_FILES = [
  "packages/open-bitcoin-mempool/src/resource.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/context.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
] as const;

const HIDDEN_EFFECT_TOKENS = [
  "SystemTime",
  "UNIX_EPOCH",
  "thread_rng",
  "OsRng",
  "rand::",
] as const;

const CONTEXT_TYPES = [
  "pub struct AdmissionContext {",
  "pub struct PressureDecisionContext {",
  "pub struct BlockLifecycleContext {",
  "pub struct ReorgLifecycleContext {",
] as const;

const FEE_ROLE_TYPES = [
  "pub struct StaticRelayFeeRate(FeeRate);",
  "pub struct IncrementalRelayFeeRate(FeeRate);",
  "pub struct RollingMempoolFeeRate(FeeRate);",
  "pub struct EffectiveAdmissionFeeRate(FeeRate);",
] as const;

const RESOURCE_TYPES = [
  "pub struct TransactionVirtualSize(usize);",
  "pub struct AccountedMempoolMemory(usize);",
  "pub struct MempoolCapacity(usize);",
] as const;

const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-mempool/src/resource.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs",
  "packages/open-bitcoin-mempool/src/context.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
] as const;

const FORBIDDEN_BROAD_CLAIMS = [
  "public relay by default",
  "public or default relay",
  "guaranteed propagation",
  "production full-node readiness",
  "production-funds wallet use",
] as const;

const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "not ",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "future",
  "later",
  "remain",
  "remains",
  "no claim",
  "not claim",
  "only",
  "bounded",
] as const;

export function checkPhase130ResourceTimeFeePrimitives(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE130_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  checkFeep01(repoRoot, failures);
  checkFeep02(repoRoot, failures);
  checkFeep03(repoRoot, failures);
  checkFeep04(repoRoot, failures);
  checkFeep05(repoRoot, failures);
  checkOverflow(repoRoot, failures);
  checkIncrementalExclusion(repoRoot, failures);
  checkOrigins(repoRoot, failures);
  checkHiddenEffects(repoRoot, failures);
  checkCauseRole(repoRoot, failures);
  checkLegacyPartialMetadata(repoRoot, failures);
  checkRetryJitter(repoRoot, failures);
  checkRpcMappings(repoRoot, failures);
  checkDeferredBoundaries(repoRoot, failures);
  checkParityOwnership(repoRoot, failures);
  checkBreadcrumbs(repoRoot, failures);
  checkIdentityPrivacy(repoRoot, failures);
  checkForbiddenClaims(repoRoot, failures);
  checkReadmeFreshness(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  checkDeterministicScope(repoRoot, failures);
  checkLegacyEnforcementSeam(repoRoot, failures);
  return failures;
}

function checkFeep01(repoRoot: string, failures: string[]): void {
  const resource = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/resource.rs");
  const missing = RESOURCE_TYPES.some((needle) => !resource.includes(needle));
  if (
    missing ||
    !resource.includes("pub struct MempoolResourceLedger {") ||
    !resource.includes("pub fn recompute_resource_ledger") ||
    !resource.includes("MEMPOOL_RESOURCE_ACCOUNTING_VERSION")
  ) {
    failures.push(
      "P130 FEEP-01: TransactionVirtualSize, AccountedMempoolMemory, and MempoolCapacity must remain distinct",
    );
  }
}

function checkFeep02(repoRoot: string, failures: string[]): void {
  const fee = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/fee.rs");
  const missing = FEE_ROLE_TYPES.some((needle) => !fee.includes(needle));
  if (
    missing ||
    !fee.includes("pub fn effective_admission_fee_rate") ||
    !fee.includes("pub fn evaluate_package_fee_floors") ||
    !fee.includes("member_meets_static_floor")
  ) {
    failures.push(
      "P130 FEEP-02: StaticRelayFeeRate, IncrementalRelayFeeRate, RollingMempoolFeeRate, and EffectiveAdmissionFeeRate must remain distinct",
    );
  }
}

function checkFeep03(repoRoot: string, failures: string[]): void {
  const context = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/context.rs");
  if (
    !context.includes("pub struct MempoolEntryMetadata {") ||
    !context.includes("pub accepted_at: MempoolAcceptanceTime") ||
    !context.includes("pub origin: MempoolOrigin") ||
    !context.includes("pub relay_intent: RelayIntent") ||
    !context.includes("LegacyUnknown")
  ) {
    failures.push(
      "P130 FEEP-03: MempoolEntryMetadata must retain acceptance time, origin, and relay intent",
    );
  }
}

function checkFeep04(repoRoot: string, failures: string[]): void {
  const context = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/context.rs");
  const retry = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
  );
  const missingContext = CONTEXT_TYPES.some((needle) => !context.includes(needle));
  if (missingContext || !retry.includes("pub struct RetryDecisionContext {")) {
    failures.push(
      "P130 FEEP-04: operation-specific immutable contexts must remain present",
    );
  }
}

function checkFeep05(repoRoot: string, failures: string[]): void {
  const lifecycle = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  );
  if (
    !lifecycle.includes("pub struct MempoolLifecycleDelta {") ||
    !lifecycle.includes("pub final_membership:") ||
    !lifecycle.includes("pub enum MempoolRemovalCause") ||
    !lifecycle.includes("pub const fn as_str(self)")
  ) {
    failures.push(
      "P130 FEEP-05: MempoolLifecycleDelta must remain the committed-fact vocabulary",
    );
  }
}

function checkOverflow(repoRoot: string, failures: string[]): void {
  const resource = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/resource.rs");
  if (
    !resource.includes("Overflow { component: &'static str }") ||
    !resource.includes("checked_add") ||
    !resource.includes("checked_product")
  ) {
    failures.push(
      "P130 overflow: resource accounting must retain checked overflow failure",
    );
  }
}

function checkIncrementalExclusion(repoRoot: string, failures: string[]): void {
  const feeCases = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs",
  );
  if (!feeCases.includes("incremental_relay_fee_is_not_an_admission_floor")) {
    failures.push(
      "P130 incremental exclusion: incremental relay fee must not act as an ordinary admission floor",
    );
  }
}

function checkOrigins(repoRoot: string, failures: string[]): void {
  const context = readTarget(repoRoot, "packages/open-bitcoin-mempool/src/context.rs");
  if (
    !context.includes("    Local,") ||
    !context.includes("    Peer,") ||
    !context.includes("MempoolOrigin::Local") ||
    !context.includes("MempoolOrigin::Peer")
  ) {
    failures.push(
      "P130 origin: Peer and Local mempool origins must remain distinct",
    );
  }
}

function checkHiddenEffects(repoRoot: string, failures: string[]): void {
  for (const file of PURE_POLICY_FILES) {
    const text = readTarget(repoRoot, file);
    if (HIDDEN_EFFECT_TOKENS.some((token) => text.includes(token))) {
      failures.push(
        "P130 hidden effects: pure mempool policy must not read wall-clock or randomness",
      );
      return;
    }
  }
}

function checkCauseRole(repoRoot: string, failures: string[]): void {
  const lifecycle = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  );
  if (
    !lifecycle.includes("pub enum MempoolRemovalCause {") ||
    !lifecycle.includes("pub enum MempoolRemovalRole {") ||
    !lifecycle.includes("pub cause: MempoolRemovalCause") ||
    !lifecycle.includes("pub role: MempoolRemovalRole")
  ) {
    failures.push(
      "P130 cause-role: MempoolRemovalCause and MempoolRemovalRole must remain independent",
    );
  }
}

function checkLegacyPartialMetadata(repoRoot: string, failures: string[]): void {
  const codec = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
  );
  if (
    !codec.includes("partial mempool entry metadata is corrupt") ||
    !codec.includes("maybe_accepted_at_unix_seconds") ||
    !codec.includes("MempoolEntryMetadata::legacy_unknown()")
  ) {
    failures.push(
      "P130 legacy compatibility: partial mempool entry metadata must fail closed as corruption",
    );
  }
}

function checkRetryJitter(repoRoot: string, failures: string[]): void {
  const retry = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
  );
  if (
    !retry.includes("const MAX_RETRY_JITTER_SECONDS: u64 = 300;") ||
    !retry.includes("pub struct RetryJitterSeconds") ||
    !retry.includes("pub struct RetryDecisionContext {")
  ) {
    failures.push(
      "P130 retry jitter: RetryJitterSeconds must enforce the inclusive 0..=300 bound",
    );
  }
}

function checkRpcMappings(repoRoot: string, failures: string[]): void {
  const rpc = readTarget(repoRoot, "packages/open-bitcoin-rpc/src/dispatch/node.rs");
  if (
    !rpc.includes("bytes: info.total_virtual_size,") ||
    !rpc.includes("usage: info.accounted_memory,") ||
    !rpc.includes("maxmempool: info.mempool_capacity,") ||
    !rpc.includes("mempoolminfee: info.effective_admission_fee_rate_sats_per_kvb,") ||
    !rpc.includes("incrementalrelayfee: info.incremental_relay_fee_rate_sats_per_kvb,") ||
    !rpc.includes("rollingmempoolfee: info.rolling_mempool_fee_rate_sats_per_kvb,") ||
    !rpc.includes("effectiveadmissionfee: info.effective_admission_fee_rate_sats_per_kvb,") ||
    !rpc.includes("capacityenforcement: info.capacity_enforcement.as_str()")
  ) {
    failures.push(
      "P130 RPC mapping: getmempoolinfo.usage must project accounted memory",
    );
  }
}

function checkDeferredBoundaries(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  if (
    !catalog.includes("**Phase 131** owns accounted-memory enforcement") ||
    !catalog.includes("**Phase 134** owns complete cross-cache projection")
  ) {
    failures.push(
      "P130 deferred boundary: Phase 131 and Phase 134 ownership must remain explicit",
    );
  }
}

function checkParityOwnership(repoRoot: string, failures: string[]): void {
  const index = readTarget(repoRoot, "docs/parity/index.json");
  const checklist = readTarget(repoRoot, "docs/parity/checklist.md");
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  for (const requirement of FEEP_REQUIREMENTS) {
    if (!index.includes(`"${requirement}"`) || !checklist.includes(requirement)) {
      failures.push(`P130 parity ownership: ${requirement} must remain registered`);
    }
  }
  if (
    !index.includes(SURFACE_ID) ||
    !checklist.includes(SURFACE_ID) ||
    !catalog.includes("`FEEP-01` through") ||
    !catalog.includes("`FEEP-05`")
  ) {
    failures.push(`P130 parity ownership: ${SURFACE_ID} surface must remain registered`);
  }
}

function checkBreadcrumbs(repoRoot: string, failures: string[]): void {
  const breadcrumbs = readTarget(repoRoot, "docs/parity/source-breadcrumbs.json");
  for (const file of REQUIRED_BREADCRUMB_FILES) {
    if (!breadcrumbs.includes(`"${file}"`)) {
      failures.push(
        "P130 breadcrumbs: production and test resource files must both be registered",
      );
      return;
    }
  }
}

function checkIdentityPrivacy(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  if (
    !catalog.includes(
      "transaction identities stay on authenticated direct responses.",
    )
  ) {
    failures.push(
      "P130 privacy: shared evidence must keep transaction identities on authenticated responses",
    );
  }
}

function checkForbiddenClaims(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  for (const paragraph of catalog.split(/\r?\n\s*\r?\n/)) {
    const lower = paragraph.toLowerCase();
    if (!lower.includes("phase 130") && !lower.includes("v2-2-resource-time-fee")) {
      continue;
    }
    for (const claim of FORBIDDEN_BROAD_CLAIMS) {
      if (!lower.includes(claim)) continue;
      if (NO_CLAIM_MARKERS.some((marker) => lower.includes(marker))) continue;
      failures.push(
        "P130 no-claim: Phase 130 must not assert public or default relay",
      );
      return;
    }
  }
}

function checkReadmeFreshness(repoRoot: string, failures: string[]): void {
  const rootReadme = readTarget(repoRoot, "README.md");
  if (rootReadme.includes("> Status: Open Bitcoin v2.1")) {
    failures.push(
      "P130 README root freshness: README.md still advertises v2.1 active status",
    );
  }

  const packagesReadme = readTarget(repoRoot, "packages/README.md");
  if (packagesReadme.includes("current v2.1 milestone")) {
    failures.push(
      "P130 README packages freshness: packages/README.md still describes current v2.1 milestone",
    );
  }

  const parityReadme = readTarget(repoRoot, "docs/parity/README.md");
  if (parityReadme.includes("The current v2.1 claim is intentionally narrow:")) {
    failures.push(
      "P130 README parity freshness: docs/parity/README.md still presents v2.1 as the current claim",
    );
  }
}

function checkVerifierWiring(repoRoot: string, failures: string[]): void {
  const verify = readTarget(repoRoot, "scripts/verify.sh");
  const heredoc = visibleCommandOrder(verify);
  const requiredVisible = [
    PHASE129_CHECK,
    PHASE130_TEST,
    PHASE130_CHECK,
    PHASE117_TEST,
  ];
  const requiredSteps = [
    `run_step "check Phase 129 integration guardrails and milestone reconciliation" ${PHASE129_CHECK}`,
    `run_step "test Phase 130 resource time and fee primitives checker" ${PHASE130_TEST}`,
    `run_step "check Phase 130 resource time and fee primitives" ${PHASE130_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  ];
  if (!orderedLines(heredoc, requiredVisible)) {
    failures.push(
      "P130 verifier heredoc: Phase 130 pair must run between Phase 129 and the Phase 117 gate",
    );
  }
  if (!orderedLines(verify, requiredSteps)) {
    failures.push(
      "P130 verifier run_step: Phase 130 pair must run between Phase 129 and the Phase 117 gate",
    );
  }

  requireFinalPhaseChecker(heredoc, "P130 final gate heredoc order", failures);
  requireFinalPhaseChecker(
    runStepLines(verify),
    "P130 final gate run_step order",
    failures,
  );
}

function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase130-resource-time-fee-primitives.ts",
  );
  const forbiddenTokens = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbiddenTokens.some((token) => checker.includes(token))) {
    failures.push(
      "P130 deterministic scope: checker must remain local and public-network-free",
    );
  }
}

function checkLegacyEnforcementSeam(repoRoot: string, failures: string[]): void {
  // Phase 131 deleted the live PolicyConfig.legacy_vsize_trim_limit seam and flipped
  // capacity-enforcement evidence to accounted_memory. Phase 130 history remains
  // auditable through SUMMARY/catalog archive wording without resurrecting the live field.
  const summary = readTarget(
    repoRoot,
    ".planning/phases/130-resource-time-and-fee-primitives/130-09-SUMMARY.md",
  );
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  if (
    !summary.includes("MempoolCapacityEnforcement::LegacyVsize") ||
    !summary.includes("legacy_vsize") ||
    !catalog.includes("fixed `legacy_vsize` during Phase 130")
  ) {
    failures.push(
      "P130 legacy enforcement: Phase 130 must retain historical legacy_vsize capacity enforcement documentation",
    );
  }
}

function requireFinalPhaseChecker(
  text: string,
  label: string,
  failures: string[],
): void {
  const phaseCommands = text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => /\bbun (?:test|run) scripts\/check-phase\d+/.test(line));
  if (!phaseCommands.at(-1)?.includes(PHASE117_CHECK)) {
    failures.push(`${label} must end with ${PHASE117_CHECK}`);
  }
}

function runStepLines(text: string): string {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("run_step "))
    .join("\n");
}

function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  return readFileSync(absolutePath, "utf8");
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

if (import.meta.main) {
  const failures = checkPhase130ResourceTimeFeePrimitives();
  if (failures.length > 0) {
    console.error("Phase 130 resource time and fee primitives check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 130 resource time and fee primitives validated.");
}

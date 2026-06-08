#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");

const SUPPORT_SOURCE_STRINGS = [
  "restartResumeEvidence",
  "finalStatus",
  "resourcePressure",
  "Restart/resume evidence",
  "Recovery diagnosis",
  "Service restart/resume",
  "Metrics availability",
  "assert_absent",
] as const;

const RUNTIME_GUIDE_STRINGS = [
  "v1.5 operator review",
  "bash scripts/verify.sh",
  "bash scripts/test-run-live-mainnet-smoke.sh",
  "support bundle --output-dir=/tmp/open-bitcoin-support",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "live_smoke.summary.finalStatus",
  "restartResumeEvidence",
  "status.service.restart_resume",
  "status.metrics",
  "status.logs",
  "opt-in UAT",
] as const;

const ARCHITECTURE_STRINGS = [
  "Support bundles embed this same snapshot",
  "Unavailable",
  "bounded support evidence",
  "raw daemon log",
  "raw peer table",
] as const;

const PARITY_STRINGS = [
  "v1.5 support bundle/operator review evidence",
  "support-evidence.json",
  "support-evidence.md",
  "opt-in UAT outside default",
  "production-node service guarantee",
] as const;

const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl --user",
  "launchctl",
] as const;

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain: ${needle}`);
  }
}

function requireAllContains(text: string, needles: readonly string[], label: string): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function verifySupportSource(
  supportLiveSmoke: string,
  supportRender: string,
  operatorBinaryTests: string,
): void {
  const supportSource = [supportLiveSmoke, supportRender, operatorBinaryTests].join("\n");
  requireAllContains(supportSource, SUPPORT_SOURCE_STRINGS, "support bundle source/tests");
}

function verifyRuntimeGuide(runtimeGuide: string): void {
  requireAllContains(runtimeGuide, RUNTIME_GUIDE_STRINGS, "docs/operator/runtime-guide.md");
}

function verifyArchitecture(statusSnapshot: string, operatorObservability: string): void {
  const architectureDocs = [statusSnapshot, operatorObservability].join("\n");
  requireAllContains(architectureDocs, ARCHITECTURE_STRINGS, "architecture docs");
}

function verifyParity(parityP2p: string): void {
  requireAllContains(parityP2p, PARITY_STRINGS, "docs/parity/catalog/p2p.md");
}

function verifyVerifyScript(verifyScript: string): void {
  requireContains(
    verifyScript,
    "bun run scripts/check-phase65-support-review.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  const supportLiveSmoke = readText(
    "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs",
  );
  const supportRender = readText("packages/open-bitcoin-cli/src/operator/support/render.rs");
  const operatorBinaryTests = readText("packages/open-bitcoin-cli/tests/operator_binary.rs");
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const statusSnapshot = readText("docs/architecture/status-snapshot.md");
  const operatorObservability = readText("docs/architecture/operator-observability.md");
  const parityP2p = readText("docs/parity/catalog/p2p.md");
  const verifyScript = readText("scripts/verify.sh");

  verifySupportSource(supportLiveSmoke, supportRender, operatorBinaryTests);
  verifyRuntimeGuide(runtimeGuide);
  verifyArchitecture(statusSnapshot, operatorObservability);
  verifyParity(parityP2p);
  verifyVerifyScript(verifyScript);

  console.log("Phase 65 support review checks passed.");
}

main();

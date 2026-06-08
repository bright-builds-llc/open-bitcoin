#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");

const DIAGNOSIS_STRINGS = [
  "compatible",
  "version_rejected",
  "network_mismatch",
  "service_bit_mismatch",
  "unsupported_message_order",
  "timeout",
  "peer_disconnect",
  "malformed_payload",
  "local_configuration_failure",
] as const;

const SOURCE_STRINGS = [
  "CompatibilityHarnessReport",
  "compatibility-harness-report.json",
  "compatibility-harness-report.md",
  "evaluate_transcript",
  "TranscriptEvent::OutboundConnect",
  "CompatibilityCommand::Harness",
  "redaction_boundaries",
] as const;

const OPERATOR_CONTRACT_STRINGS = [
  "Compatibility(CompatibilityArgs)",
  "CompatibilityScenario",
  "CompatibilityCommand",
] as const;

const TEST_STRINGS = [
  "open_bitcoin_compatibility_harness_writes_json_and_markdown_reports",
  "open_bitcoin_compatibility_harness_covers_required_diagnosis_scenarios",
  "assert_absent",
] as const;

const RUNTIME_GUIDE_STRINGS = [
  "Compatibility harness operator wrapper",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "compatibility harness",
  "compatibility-harness-report.json",
  "compatibility-harness-report.md",
  "raw wire payloads",
  "opt-in local compatibility evidence",
] as const;

const PARITY_STRINGS = [
  "v1.5 compatibility harness operator wrapper",
  "compatibility-harness-report.json",
  "compatibility-harness-report.md",
  "open-bitcoin-network::evaluate_transcript",
  "production-node service guarantee",
] as const;

const FORBIDDEN_VERIFY_STRINGS = [
  "compatibility harness --peer-endpoint",
  "--scenario=service-bit-mismatch",
  "run-live-mainnet-smoke",
  "--manual-peer",
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

function verifySource(compatibilitySource: string, operatorContract: string): void {
  requireAllContains(compatibilitySource, SOURCE_STRINGS, "operator compatibility source");
  requireAllContains(compatibilitySource, DIAGNOSIS_STRINGS, "operator compatibility source");
  requireAllContains(operatorContract, OPERATOR_CONTRACT_STRINGS, "operator CLI contract");
}

function verifyTests(operatorBinaryTests: string): void {
  requireAllContains(operatorBinaryTests, TEST_STRINGS, "operator binary tests");
  requireAllContains(operatorBinaryTests, DIAGNOSIS_STRINGS, "operator binary tests");
}

function verifyRuntimeGuide(runtimeGuide: string): void {
  requireAllContains(runtimeGuide, RUNTIME_GUIDE_STRINGS, "docs/operator/runtime-guide.md");
  requireAllContains(runtimeGuide, DIAGNOSIS_STRINGS, "docs/operator/runtime-guide.md");
}

function verifyParity(parityP2p: string): void {
  requireAllContains(parityP2p, PARITY_STRINGS, "docs/parity/catalog/p2p.md");
}

function verifyVerifyScript(verifyScript: string): void {
  requireContains(
    verifyScript,
    "bun run scripts/check-phase66-compatibility-wrapper.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  const compatibilitySource = readText("packages/open-bitcoin-cli/src/operator/compatibility.rs");
  const operatorContract = readText("packages/open-bitcoin-cli/src/operator.rs");
  const operatorBinaryTests = readText("packages/open-bitcoin-cli/tests/operator_binary.rs");
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const parityP2p = readText("docs/parity/catalog/p2p.md");
  const verifyScript = readText("scripts/verify.sh");

  verifySource(compatibilitySource, operatorContract);
  verifyTests(operatorBinaryTests);
  verifyRuntimeGuide(runtimeGuide);
  verifyParity(parityP2p);
  verifyVerifyScript(verifyScript);

  console.log("Phase 66 compatibility wrapper checks passed.");
}

main();

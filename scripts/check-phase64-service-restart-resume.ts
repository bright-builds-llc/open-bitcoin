#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");

const STATUS_CONTRACT_STRINGS = [
  "ServiceRestartResumeStatus",
  "ServicePriorShutdownStatus",
  "ServiceResumeProgressStatus",
  "ServiceStaleInflightStatus",
  "restart_resume",
  "service restart/resume evidence unavailable",
] as const;

const RESTART_RESUME_FIELDS = [
  "same_datadir",
  "prior_shutdown",
  "durable_progress",
  "stale_inflight",
  "recovery_category",
  "next_action",
] as const;

const RENDER_STRINGS = [
  "restart_resume=",
  "same_datadir=",
  "prior_shutdown=",
  "stale_inflight=",
  "recovery_category=",
  "next_action=",
] as const;

const DASHBOARD_ROWS = [
  "Restart/resume",
  "Prior shutdown",
  "Resume progress",
  "Stale in-flight",
  "Resume action",
] as const;

const DOC_STRINGS = [
  "Service-supervised restart/resume evidence",
  "service.restart_resume",
  "prior_shutdown",
  "same_datadir",
  "durable_progress",
  "stale_inflight",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "optional public-network restart smoke",
] as const;

const PARITY_STRINGS = [
  "service-supervised restart/resume evidence",
  "service.restart_resume",
  "default verification",
  "production-node service guarantee",
] as const;

const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--restart-after-progress",
  "systemctl --user restart",
  "launchctl kickstart",
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

function requireAllContains(
  text: string,
  needles: readonly string[],
  label: string,
): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function verifyStatusContract(statusRs: string, serviceStatusRs: string): void {
  requireAllContains(
    statusRs,
    STATUS_CONTRACT_STRINGS,
    "packages/open-bitcoin-node/src/status.rs",
  );
  requireAllContains(
    serviceStatusRs,
    RESTART_RESUME_FIELDS,
    "packages/open-bitcoin-cli/src/operator/status/service_status.rs",
  );
  requireContains(
    serviceStatusRs,
    "durable_runtime_metadata",
    "packages/open-bitcoin-cli/src/operator/status/service_status.rs",
  );
  requireContains(
    serviceStatusRs,
    "ServiceStaleInflightStatus::StaleRequestsRecorded",
    "packages/open-bitcoin-cli/src/operator/status/service_status.rs",
  );
}

function verifyRenderers(statusRenderRs: string, dashboardModelRs: string): void {
  requireAllContains(
    statusRenderRs,
    RENDER_STRINGS,
    "packages/open-bitcoin-cli/src/operator/status/render.rs",
  );
  requireAllContains(
    dashboardModelRs,
    DASHBOARD_ROWS,
    "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  );
}

function verifyDocs(runtimeGuide: string, parityP2p: string): void {
  requireAllContains(runtimeGuide, DOC_STRINGS, "docs/operator/runtime-guide.md");
  requireAllContains(parityP2p, PARITY_STRINGS, "docs/parity/catalog/p2p.md");
}

function verifyDefaultVerificationBoundaries(verifySh: string): void {
  requireContains(
    verifySh,
    "check-phase64-service-restart-resume",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifySh, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  const statusRs = readText("packages/open-bitcoin-node/src/status.rs");
  const serviceStatusRs = readText(
    "packages/open-bitcoin-cli/src/operator/status/service_status.rs",
  );
  const statusRenderRs = readText(
    "packages/open-bitcoin-cli/src/operator/status/render.rs",
  );
  const dashboardModelRs = readText(
    "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  );
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const parityP2p = readText("docs/parity/catalog/p2p.md");
  const verifySh = readText("scripts/verify.sh");

  verifyStatusContract(statusRs, serviceStatusRs);
  verifyRenderers(statusRenderRs, dashboardModelRs);
  verifyDocs(runtimeGuide, parityP2p);
  verifyDefaultVerificationBoundaries(verifySh);

  console.log("Phase 64 service restart/resume checks passed.");
}

main();

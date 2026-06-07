#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const SERVICE_LIFECYCLE_LABELS = [
  "unmanaged",
  "installed-stopped",
  "running",
  "failed",
  "disabled",
  "unavailable-manager",
] as const;
const SERVICE_COMMAND_STRINGS = [
  "ServiceCommand::Preview",
  "ServiceCommand::Start",
  "ServiceCommand::Stop",
  "ServiceCommand::Restart",
] as const;
const OPERATOR_SERVICE_COMMAND_VARIANTS = [
  "Preview",
  "Start",
  "Stop",
  "Restart",
] as const;
const SYSTEMD_USER_COMMANDS = [
  "systemctl --user start open-bitcoin-node.service",
  "systemctl --user stop open-bitcoin-node.service",
  "systemctl --user restart open-bitcoin-node.service",
] as const;
const LAUNCHD_USER_COMMANDS = [
  "launchctl bootstrap",
  "launchctl bootout",
  "launchctl kickstart -k",
] as const;
const DASHBOARD_ACTION_LABELS = [
  "t start service",
  "o stop service",
  "x restart service",
] as const;
const DASHBOARD_ACTION_SOURCE_PAIRS = [
  'action("t", "start service"',
  'action("o", "stop service"',
  'action("x", "restart service"',
] as const;
const REPO_LOCAL_SERVICE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
] as const;
const PHASE_62_SYNC_TRUTH_STRINGS = [
  "Sync configured targets",
  "Sync attempts",
  "Sync latest stop reason",
  "Sync recovery category",
  "Sync pressure",
  "Peers",
  "downloaded_block_height",
  "connected_block_height",
] as const;
const FORBIDDEN_DOC_PHRASES = [
  "production service",
  "production full node",
  "packaged service guarantee",
  "unattended production-node replacement",
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

function verifyLifecycleContract(statusRs: string, runtimeGuide: string): void {
  requireContains(
    statusRs,
    "ServiceLifecycleStatus",
    "packages/open-bitcoin-node/src/status.rs",
  );
  requireAllContains(
    statusRs,
    SERVICE_LIFECYCLE_LABELS,
    "packages/open-bitcoin-node/src/status.rs",
  );
  requireContains(
    runtimeGuide,
    "Service lifecycle labels",
    "docs/operator/runtime-guide.md",
  );
  requireAllContains(
    runtimeGuide,
    SERVICE_LIFECYCLE_LABELS,
    "docs/operator/runtime-guide.md",
  );
}

function verifyServiceCommands(operatorRs: string, serviceRs: string): void {
  requireAllContains(
    operatorRs,
    OPERATOR_SERVICE_COMMAND_VARIANTS,
    "packages/open-bitcoin-cli/src/operator.rs",
  );
  requireAllContains(
    serviceRs,
    SERVICE_COMMAND_STRINGS,
    "packages/open-bitcoin-cli/src/operator/service.rs",
  );
}

function verifyDaemonTargeting(runtimeRs: string, runtimeGuide: string): void {
  requireContains(
    runtimeRs,
    "resolve_service_daemon_binary",
    "packages/open-bitcoin-cli/src/operator/runtime.rs",
  );
  requireContains(
    runtimeRs,
    "open-bitcoind",
    "packages/open-bitcoin-cli/src/operator/runtime.rs",
  );
  requireContains(runtimeGuide, "open-bitcoind", "docs/operator/runtime-guide.md");
}

function verifyPlatformCommands(launchdRs: string, systemdRs: string): void {
  requireAllContains(
    launchdRs,
    LAUNCHD_USER_COMMANDS,
    "packages/open-bitcoin-cli/src/operator/service/launchd.rs",
  );
  requireAllContains(
    systemdRs,
    SYSTEMD_USER_COMMANDS,
    "packages/open-bitcoin-cli/src/operator/service/systemd.rs",
  );
}

function verifyDaemonServiceArguments(launchdRs: string, systemdRs: string): void {
  requireAllContains(
    launchdRs,
    ["-datadir=", "-openbitcoinconf="],
    "packages/open-bitcoin-cli/src/operator/service/launchd.rs",
  );
  requireAllContains(
    systemdRs,
    ["-datadir=", "-openbitcoinconf="],
    "packages/open-bitcoin-cli/src/operator/service/systemd.rs",
  );
  requireNotContains(
    launchdRs,
    "<string>--datadir</string>",
    "packages/open-bitcoin-cli/src/operator/service/launchd.rs",
  );
  requireNotContains(
    launchdRs,
    "<string>--config</string>",
    "packages/open-bitcoin-cli/src/operator/service/launchd.rs",
  );
  requireNotContains(
    systemdRs,
    "--datadir",
    "packages/open-bitcoin-cli/src/operator/service/systemd.rs",
  );
  requireNotContains(
    systemdRs,
    "--config",
    "packages/open-bitcoin-cli/src/operator/service/systemd.rs",
  );
}

function verifyDashboardActions(actionRs: string, modelRs: string, runtimeGuide: string): void {
  requireAllContains(
    actionRs,
    [
      "DashboardAction::StartService",
      "DashboardAction::StopService",
      "DashboardAction::RestartService",
      "ServiceCommand::Start",
      "ServiceCommand::Stop",
      "ServiceCommand::Restart",
    ],
    "packages/open-bitcoin-cli/src/operator/dashboard/action.rs",
  );
  requireAllContains(
    modelRs,
    DASHBOARD_ACTION_SOURCE_PAIRS,
    "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  );
  requireAllContains(
    runtimeGuide,
    DASHBOARD_ACTION_LABELS,
    "docs/operator/runtime-guide.md",
  );
}

function verifyStatusSurfaces(
  statusRenderRs: string,
  dashboardModelRs: string,
  runtimeGuide: string,
): void {
  requireAllContains(
    statusRenderRs,
    ["ServiceLifecycleStatus", ...PHASE_62_SYNC_TRUTH_STRINGS],
    "packages/open-bitcoin-cli/src/operator/status/render.rs",
  );
  requireAllContains(
    dashboardModelRs,
    ["ServiceLifecycleStatus", "Configured targets", "Latest stop reason", "Pressure"],
    "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  );
  requireAllContains(
    runtimeGuide,
    [
      "status/dashboard JSON",
      "and human output preserve Phase 62 sync lifecycle",
      "configured targets",
      "attempt counters",
      "latest stop reason",
      "recovery category/action",
      "resource pressure",
      "peer health",
      "downloaded/connected block evidence",
    ],
    "docs/operator/runtime-guide.md",
  );
}

function verifyDocs(runtimeGuide: string): void {
  requireAllContains(
    runtimeGuide,
    [
      ...REPO_LOCAL_SERVICE_COMMANDS,
      "`service preview` is always side-effect-free",
      "service install` and\n`service uninstall` are previews unless `--apply` is supplied",
      "~/Library/LaunchAgents/org.open-bitcoin.node.plist",
      "~/.config/systemd/user/open-bitcoin-node.service",
      "<log_dir>/open-bitcoin.log",
    ],
    "docs/operator/runtime-guide.md",
  );
  for (const phrase of FORBIDDEN_DOC_PHRASES) {
    requireNotContains(runtimeGuide, phrase, "docs/operator/runtime-guide.md");
  }
}

function verifyVerifyScript(verifyScript: string): void {
  requireContains(
    verifyScript,
    "bun run scripts/check-phase63-service-lifecycle.ts",
    "scripts/verify.sh",
  );
  requireNotContains(verifyScript, "systemctl --user start", "scripts/verify.sh");
  requireNotContains(verifyScript, "systemctl --user stop", "scripts/verify.sh");
  requireNotContains(verifyScript, "systemctl --user restart", "scripts/verify.sh");
  requireNotContains(verifyScript, "launchctl bootstrap", "scripts/verify.sh");
  requireNotContains(verifyScript, "launchctl bootout", "scripts/verify.sh");
  requireNotContains(verifyScript, "launchctl kickstart", "scripts/verify.sh");
  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
  requireNotContains(verifyScript, "--manual-peer", "scripts/verify.sh");
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh");
}

function main(): void {
  const statusRs = readText("packages/open-bitcoin-node/src/status.rs");
  const operatorRs = readText("packages/open-bitcoin-cli/src/operator.rs");
  const runtimeRs = readText("packages/open-bitcoin-cli/src/operator/runtime.rs");
  const serviceRs = readText("packages/open-bitcoin-cli/src/operator/service.rs");
  const launchdRs = readText("packages/open-bitcoin-cli/src/operator/service/launchd.rs");
  const systemdRs = readText("packages/open-bitcoin-cli/src/operator/service/systemd.rs");
  const actionRs = readText("packages/open-bitcoin-cli/src/operator/dashboard/action.rs");
  const dashboardModelRs = readText("packages/open-bitcoin-cli/src/operator/dashboard/model.rs");
  const statusRenderRs = readText("packages/open-bitcoin-cli/src/operator/status/render.rs");
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const verifyScript = readText("scripts/verify.sh");

  verifyLifecycleContract(statusRs, runtimeGuide);
  verifyServiceCommands(operatorRs, serviceRs);
  verifyDaemonTargeting(runtimeRs, runtimeGuide);
  verifyPlatformCommands(launchdRs, systemdRs);
  verifyDaemonServiceArguments(launchdRs, systemdRs);
  verifyDashboardActions(actionRs, dashboardModelRs, runtimeGuide);
  verifyStatusSurfaces(statusRenderRs, dashboardModelRs, runtimeGuide);
  verifyDocs(runtimeGuide);
  verifyVerifyScript(verifyScript);

  console.log("validated Phase 63 service lifecycle");
}

main();

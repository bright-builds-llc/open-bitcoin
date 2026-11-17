import {
  PUBLIC_NETWORK_NOT_RUN_LINE,
  REQUIRED_UAT_COMMANDS,
  UAT_PACKAGE,
} from "./constants.ts";

export function checkUatCommands(texts: Map<string, string>, failures: string[]): void {
  const uatPackage = texts.get(UAT_PACKAGE) ?? "";
  for (const command of REQUIRED_UAT_COMMANDS) {
    if (!uatPackage.includes(command)) {
      failures.push(`missing Phase 151 UAT command ${command}`);
    }
  }
  const hasNotRunLine = uatPackage
    .split("\n")
    .some((line) => line.trim() === PUBLIC_NETWORK_NOT_RUN_LINE);
  if (!hasNotRunLine) {
    failures.push("151-UAT.md must record public-network review as the exact line not run");
  }
}

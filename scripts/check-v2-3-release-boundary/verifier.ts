import {
  REQUIRED_UAT_COMMANDS,
  UAT_PACKAGE,
} from "./constants.ts";

export function checkUatCommands(texts: Map<string, string>, failures: string[]): void {
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  const uatPackage = texts.get(UAT_PACKAGE) ?? "";
  const documented = `${runtimeGuide}\n${uatPackage}`;
  for (const command of REQUIRED_UAT_COMMANDS) {
    if (!documented.includes(command)) {
      failures.push(`missing Phase 145 UAT command ${command}`);
    }
  }
}

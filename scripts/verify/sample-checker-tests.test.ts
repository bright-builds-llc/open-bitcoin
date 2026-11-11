import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const PRELOAD = path.join(import.meta.dir, "sample-checker-tests.ts");
// One looped site of 100 rows, one helper reached from two places, one single case.
const LOOPED_SUITE = `import { expect, test } from "bun:test";
for (let row = 0; row < 100; row++) test(\`looped \${row}\`, () => expect(row).toBeGreaterThanOrEqual(0));
function refuses(name: string) { test(name, () => expect(name).toBeTruthy()); }
for (let row = 0; row < 10; row++) refuses(\`first caller \${row}\`);
for (let row = 0; row < 10; row++) refuses(\`second caller \${row}\`);
test("single", () => expect(1).toBe(1));
`;

function runSuite(relativePath: string, environment: Record<string, string>): { status: number; output: string } {
  const root = mkdtempSync(path.join(tmpdir(), "sample-checker-tests-"));
  try {
    const file = path.join(root, relativePath);
    mkdirSync(path.dirname(file), { recursive: true });
    writeFileSync(file, LOOPED_SUITE);
    const environmentWithoutAgentMarkers = { ...process.env, ...environment };
    // Bun hides pass lines when it detects an agent shell.
    for (const name of ["CLAUDECODE", "CLAUDE_CODE_ENTRYPOINT", "AGENT"]) delete environmentWithoutAgentMarkers[name];
    const result = Bun.spawnSync(["bun", "test", `./${relativePath}`, "--preload", PRELOAD], {
      cwd: root,
      env: environmentWithoutAgentMarkers,
      stdout: "pipe",
      stderr: "pipe",
    });
    return { status: result.exitCode, output: result.stdout.toString("utf8") + result.stderr.toString("utf8") };
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

test("phase_checker_suite_keeps_a_spread_of_rows_per_call_site", () => {
  // Arrange
  const suite = "scripts/check-phase999-example.test.ts";

  // Act
  const result = runSuite(suite, { OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS: "" });

  // Assert
  expect(result.status).toBe(0);
  // 100 rows keep 0-4, 8, 13, 21, 34, 55, 89; each 10-row caller keeps 0-4 and 8.
  expect(result.output).toContain("sampled checker cases: ran 24 of 121");
  expect(result.output).toContain("(pass) looped 89");
  expect(result.output).toContain("(pass) second caller 8");
  expect(result.output).not.toContain("looped 90");
});

test("exhaustive_switch_runs_every_case", () => {
  // Arrange
  const suite = "scripts/check-phase999-example.test.ts";

  // Act
  const result = runSuite(suite, { OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS: "1" });

  // Assert
  expect(result.status).toBe(0);
  expect(result.output).toContain(" 121 pass");
  expect(result.output).not.toContain("sampled checker cases");
});

test("suites_outside_the_phase_checkers_are_not_sampled", () => {
  // Arrange
  const suite = "scripts/other-tool.test.ts";

  // Act
  const result = runSuite(suite, { OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS: "" });

  // Assert
  expect(result.status).toBe(0);
  expect(result.output).toContain(" 121 pass");
});

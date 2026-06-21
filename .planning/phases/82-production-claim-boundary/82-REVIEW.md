---
phase: 82-production-claim-boundary
reviewed: 2026-06-21T13:32:07Z
depth: standard
files_reviewed: 15
files_reviewed_list:
  - README.md
  - docs/operator/runtime-guide.md
  - docs/parity/production-claim-boundary.md
  - docs/parity/release-readiness.md
  - docs/parity/deviations-and-unknowns.md
  - docs/parity/index.json
  - docs/parity/checklist.md
  - docs/parity/README.md
  - docs/parity/catalog/operator-runtime-release-hardening.md
  - docs/parity/catalog/p2p.md
  - docs/parity/catalog/chainstate.md
  - scripts/check-phase82-production-claim-boundary.ts
  - scripts/check-phase82-production-claim-boundary.test.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 82: Code Review Report

**Reviewed:** 2026-06-21T13:32:07Z
**Depth:** standard
**Files Reviewed:** 15
**Status:** issues_found

## Summary

Reviewed the Phase 82 production-claim-boundary docs, parity roots, checker, checker tests, and verifier wiring. Repo-local guidance materially used for this review: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/code-shape.md`, `standards/core/architecture.md`, and `standards/languages/typescript-javascript.md`.

The current docs do not appear to overclaim production readiness; they consistently keep v1.8 scoped to defining future production-readiness gates and keep production-adjacent surfaces deferred. JSON syntax is valid, targeted Phase 82 tests pass, the targeted checker passes, and `scripts/verify.sh` has valid Bash syntax. The issues found are false-negative risks in the Phase 82 checker.

Verification performed:

- `bun test scripts/check-phase82-production-claim-boundary.test.ts` passed
- `bun run scripts/check-phase82-production-claim-boundary.ts` passed
- `bash -n scripts/verify.sh` passed
- `python3 -m json.tool docs/parity/index.json >/dev/null` passed

## Warnings

### WR-01: Verifier Wiring Check Can Pass On Non-Executed Command Text

**File:** `scripts/check-phase82-production-claim-boundary.ts:524`
**Issue:** `verifyVerifierWiring` checks command presence and ordering against the entire `scripts/verify.sh` text. That file also contains a non-executed `VERIFY_COMMAND_ORDER` heredoc at `scripts/verify.sh:242` with the Phase 82 commands. If the executable `run_step` calls are later removed or reordered while the heredoc remains correct, the Phase 82 checker can still pass even though the default verifier no longer actually runs the Phase 82 test/checker pair. That is a false negative for the PROD-04 deterministic verification boundary.
**Fix:**
```ts
function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const executableText = executableVerifyText(text);

  for (const command of [PHASE82_TEST_COMMAND, PHASE82_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase80Index = executableText.indexOf(PHASE80_CHECKER_COMMAND);
  const phase82TestIndex = executableText.indexOf(PHASE82_TEST_COMMAND);
  const phase82CheckerIndex = executableText.indexOf(PHASE82_CHECKER_COMMAND);
  if (!(phase80Index !== -1 && phase82TestIndex > phase80Index && phase82CheckerIndex > phase82TestIndex)) {
    failures.push("verifier-order requires executed Phase 82 test and checker after Phase 80 checker");
  }
}
```
Also add a regression test where `scripts/verify.sh` contains only the legacy heredoc command block and no executable Phase 82 `run_step` calls; it should fail with `verifier-order`.

### WR-02: Claim Matrix Validation Does Not Bind Forbidden Statements To Deferred Status

**File:** `scripts/check-phase82-production-claim-boundary.ts:270`
**Issue:** `verifyCanonicalBoundary` checks that the matrix header exists, that the allowed statement exists, that `not allowed yet` appears somewhere, and that each forbidden statement appears somewhere. It does not parse the claim-to-evidence matrix rows or assert that each forbidden production-adjacent statement is paired with support term `deferred`, current status `not allowed yet`, and a no-default-verifier proof boundary. A future edit could accidentally mark `Open Bitcoin has production full-node readiness.` as `supported` or `allowed` while another row still contains `not allowed yet`, and the checker would pass. That false negative weakens PROD-01 through PROD-04 traceability.
**Fix:**
```ts
type ClaimMatrixRow = {
  statement: string;
  supportTerm: string;
  currentStatus: string;
  verificationCommand: string;
};

function requireForbiddenClaimRows(rows: ClaimMatrixRow[], failures: string[]): void {
  for (const statement of NOT_ALLOWED_STATEMENTS) {
    const row = rows.find((candidate) => candidate.statement.includes(statement));
    if (row === undefined) {
      failures.push(`claim-to-evidence matrix missing row for ${statement}`);
      continue;
    }
    if (
      row.supportTerm !== "`deferred`" ||
      row.currentStatus !== "not allowed yet" ||
      !row.verificationCommand.includes("No default verifier may prove this in v1.8")
    ) {
      failures.push(`claim-to-evidence matrix row for ${statement} must remain deferred and not allowed`);
    }
  }
}
```
Add a regression test that changes one forbidden row to `supported` or `allowed` while leaving another `not allowed yet` row intact; the checker should fail.

---

_Reviewed: 2026-06-21T13:32:07Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_

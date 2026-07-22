---
quick_id: 260722-ctq
description: Reconcile post-v2.1 documentation state
phase: quick-260722-ctq-reconcile-post-v2-1-documentation-state
plan: "01"
type: execute
mode: quick-full
wave: 1
depends_on: []
autonomous: true
generated_by: gsd-plan-phase
lifecycle_mode: direct-fallback
phase_lifecycle_id: quick-260722-ctq
generated_at: 2026-07-22T09:17:07-05:00
files_modified:
  - README.md
  - .planning/ARCHITECTURE.md
  - .planning/CONVENTIONS.md
  - docs/parity/release-readiness.md
  - docs/parity/support-matrix.md
  - docs/parity/production-claim-boundary.md
  - docs/parity/deviations-and-unknowns.md
  - docs/parity/catalog/rpc-cli-config.md
  - scripts/check-current-documentation-reconciliation.ts
  - scripts/check-current-documentation-reconciliation.test.ts
  - scripts/verify.sh
  - docs/metrics/lines-of-code.md
must_haves:
  truths:
    - "Live project documentation says v2.1 shipped and was archived on 2026-07-22, reports the final 39/39 requirements, 20/20 phases, 13/13 integration links, and 11/11 flows, and routes future work to /gsd-new-milestone."
    - "Current support documentation classifies bounded, explicit, default-off v2.0 transaction relay as preview while keeping only public/default/production relay beyond that boundary deferred."
    - "The RPC/CLI catalog lists exactly the SupportedMethod serde names and documents sendtoaddress plus -rpcwallet routing for the implemented wallet subset without claiming richer send or multiwallet lifecycle parity."
    - "A deterministic local checker and focused mutation suite reject future regressions in the reconciled facts, and the checker pair runs immediately after the final Phase 117 gate in both verifier command lists."
    - "Historical milestone narratives and the protected planning/parity roots remain unchanged, the full verification contract passes, and the authorized change is committed locally once without a push."
  artifacts:
    - path: "docs/parity/release-readiness.md"
      provides: "Canonical shipped-and-archived v2.1 release handoff with final audit scores"
      contains: "39/39"
    - path: "docs/parity/support-matrix.md"
      provides: "Canonical preview classification for bounded transaction relay"
      contains: "| transaction relay | `preview` |"
    - path: "docs/parity/catalog/rpc-cli-config.md"
      provides: "RPC method and implemented wallet-routing catalog"
      contains: "sendtoaddress"
    - path: "scripts/check-current-documentation-reconciliation.ts"
      provides: "Injectable, local-filesystem-only documentation consistency checker"
      exports:
        - checkCurrentDocumentationReconciliation
    - path: "scripts/check-current-documentation-reconciliation.test.ts"
      provides: "Arrange/Act/Assert mutation coverage for every required reconciliation boundary"
    - path: "scripts/verify.sh"
      provides: "Default verifier integration after the final Phase 117 gate"
    - path: "docs/metrics/lines-of-code.md"
      provides: "Fresh tracked LOC report including the new TypeScript checker and tests"
  key_links:
    - from: "scripts/check-current-documentation-reconciliation.ts"
      to: "packages/open-bitcoin-rpc/src/method.rs"
      via: "parse SupportedMethod serde rename values and compare them as an exact set with the catalog's two supported-method lists"
      pattern: "SupportedMethod|serde\\(rename"
    - from: "scripts/check-current-documentation-reconciliation.ts"
      to: "docs/parity/support-matrix.md"
      via: "section-aware Markdown table parsing of the current transaction-relay row"
      pattern: "transaction relay.*preview"
    - from: "scripts/verify.sh"
      to: "scripts/check-current-documentation-reconciliation.test.ts"
      via: "visible command-order entry and executed run_step immediately after Phase 117"
      pattern: "check-current-documentation-reconciliation"
---

# Quick Task 260722-ctq: Post-v2.1 Documentation Reconciliation

<objective>
Reconcile the live documentation with the shipped and archived v2.1 state, guard those current-state facts deterministically, refresh generated LOC evidence, and land the authorized result as one local implementation commit.

Purpose: Remove stale pre-v1.2 and pre-archive guidance without rewriting historical milestone truth or broadening public/default/production support claims.

Output: Reconciled current-state docs, a tested Bun/TypeScript consistency checker wired into the repo verifier, a fresh LOC report, and local commit `docs: reconcile post-v2.1 project state` with no push.
</objective>

<execution-context>
@/Users/peterryszkiewicz/.codex/get-shit-done/workflows/execute-plan.md
@/Users/peterryszkiewicz/.codex/get-shit-done/templates/summary.md
</execution-context>

<context>
@AGENTS.md
@AGENTS.bright-builds.md
@standards/core/verification.md
@standards/core/testing.md
@standards/languages/typescript-javascript.md
@.planning/STATE.md
@.planning/milestones/v2.1-MILESTONE-AUDIT.md
@packages/open-bitcoin-rpc/src/method.rs
@scripts/check-phase117-parity-uat-release-boundary.ts

The repo-local guidance makes Bun/TypeScript the canonical substantial automation surface, `bash scripts/verify.sh` the full verification contract, and `docs/metrics/lines-of-code.md` a required tracked generated artifact. Bright Builds testing rules require focused Arrange/Act/Assert unit tests; the checker should therefore keep parsing and comparison logic data-in/data-out and filesystem access in a thin shell.

Protected exclusions: do not modify `.planning/PROJECT.md`, `.planning/MILESTONES.md`, `.planning/STATE.md`, archived milestone artifacts, `docs/parity/index.json`, or `docs/parity/checklist.md`. Preserve historical v1.x sections and milestone-specific statements as historical truth.

<interfaces>
Create this internal checker interface:

```typescript
export function checkCurrentDocumentationReconciliation(
  maybeRepoRoot?: string,
): string[];
```

The optional root is the test injection seam. The CLI calls the same function with the repository default, prints one stable failure bullet per violation and exits `1` on failure, or prints one success line and exits `0` on success.

The authoritative Rust method set is the 20 `#[serde(rename = "...")]` values inside `SupportedMethod`: 14 baseline-backed methods (`getblockchaininfo`, `getmempoolinfo`, `getnetworkinfo`, `sendrawtransaction`, `deriveaddresses`, `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`, `listdescriptors`, `getwalletinfo`, `getbalances`, `listunspent`, `importdescriptors`, `rescanblockchain`) and six Open Bitcoin extensions (`openbitcoinnetworkstatus`, `openbitcoinsyncstatus`, `openbitcoinsyncpause`, `openbitcoinsyncresume`, `buildtransaction`, `buildandsigntransaction`).
</interfaces>
</context>

<tasks>

<task type="auto">
  <name>Task 1: Reconcile live post-v2.1 documentation</name>
  <files>README.md, .planning/ARCHITECTURE.md, .planning/CONVENTIONS.md, docs/parity/release-readiness.md, docs/parity/support-matrix.md, docs/parity/production-claim-boundary.md, docs/parity/deviations-and-unknowns.md, docs/parity/catalog/rpc-cli-config.md</files>
  <action>Make narrow current-state edits only. In README, architecture, and conventions, state that v2.1 shipped and was archived on 2026-07-22; replace the obsolete “later v1.2” sync boundary with the present explicit opt-in full-sync, bounded inbound, transaction-relay, block-serving, and compact-relay boundaries, while retaining the no-public-default, no-production-service/deployment, no-production-readiness, and no-production-funds claims. In release readiness, replace active/archive-ready language and `/gsd-complete-milestone v2.1` routing with shipped/archive wording, a link to `.planning/milestones/v2.1-MILESTONE-AUDIT.md`, final scores of 39/39 requirements, 20/20 phases, 13/13 integration links, and 11/11 flows, and `/gsd-new-milestone` as the only next route. Change the canonical current support-matrix transaction-relay row to `preview` for bounded explicit default-off v2.0 evidence. In the production claim boundary and deviations register, add or revise the current transaction-relay statement/row so the shipped bounded v2.0 path is preview and only public/default/production relay beyond it is deferred; leave historical v1.x tables and prose untouched. In the RPC/CLI catalog, make the two supported-method lists equal the 20-name interface above, document `sendtoaddress` and `-rpcwallet`/`/wallet/&lt;name&gt;` routing as supported only for the implemented wallet subset, and narrow deferrals to richer `send` semantics plus `loadwallet`, `unloadwallet`, and `listwallets` lifecycle parity. Do not change runtime behavior or any protected exclusion.</action>
  <verify>
    <automated>rg -n "shipped|archived|2026-07-22|39/39|20/20|13/13|11/11|gsd-new-milestone|transaction relay.*preview|sendtoaddress|rpcwallet" README.md .planning/ARCHITECTURE.md .planning/CONVENTIONS.md docs/parity/release-readiness.md docs/parity/support-matrix.md docs/parity/production-claim-boundary.md docs/parity/deviations-and-unknowns.md docs/parity/catalog/rpc-cli-config.md</automated>
  </verify>
  <done>All eight live docs agree with the archived v2.1 evidence and implemented RPC/relay scope; current statements no longer send readers through v2.1 completion or future-v1.2 routes; historical v1.x narratives and all protected files are unchanged.</done>
</task>

<task type="auto" tdd="true">
  <name>Task 2: Add and wire the deterministic reconciliation checker</name>
  <files>scripts/check-current-documentation-reconciliation.ts, scripts/check-current-documentation-reconciliation.test.ts, scripts/verify.sh</files>
  <behavior>
    - A fixture copied from the reconciled live corpus returns no failures.
    - Reintroducing `/gsd-complete-milestone v2.1` or archive-ready/pending-completion language in README or the current v2.1 release-readiness section fails.
    - Reintroducing later/pre-v1.2 sync language in architecture or conventions fails.
    - Changing the current support-matrix transaction-relay row from `preview` to generic `deferred` fails.
    - Promoting public/default/production transaction relay from `deferred`, or describing bounded v2.0 relay itself as deferred in either current boundary table, fails.
    - Removing any SupportedMethod serde name from the catalog or adding a catalog-only supported method fails exact-set comparison.
    - Blanket deferral of `sendtoaddress` or `-rpcwallet` fails while the narrowly named richer-send and wallet-lifecycle deferrals pass.
    - Removing or reordering either verifier entry fails the wiring assertion.
  </behavior>
  <action>Write the Bun tests first with temporary repository fixtures, one concern per test and explicit Arrange/Act/Assert comments. Implement `checkCurrentDocumentationReconciliation(maybeRepoRoot?)` using only `node:fs` reads and `node:path`; do not use network access, subprocesses, external packages, wall-clock state, or mutable repository state. Parse named Markdown sections/tables instead of broad whole-file keyword bans so preserved historical v1.x prose remains legal. Extract only serde rename strings from the `SupportedMethod` enum body, parse both catalog supported-method lists, normalize to sets, and report missing/extra names deterministically. Require the exact final audit counts, archive date/state, audit link, and next-milestone route in current v2.1 content. Parse the canonical support-matrix transaction-relay row as `preview`, and parse the current production-boundary/deviation rows so only relay beyond the bounded v2.0 path is `deferred`. Reject blanket deferral phrases for `sendtoaddress` or `-rpcwallet`. Add self-wiring checks for both `VERIFY_COMMAND_ORDER` and executed `run_step` lists. Insert the test then checker commands immediately after the final Phase 117 checker in both lists without moving or weakening any existing phase gate; update the nearby ordering comment accordingly.</action>
  <verify>
    <automated>bun test scripts/check-current-documentation-reconciliation.test.ts &amp;&amp; bun run scripts/check-current-documentation-reconciliation.ts</automated>
  </verify>
  <done>The exported checker passes the reconciled repository, every approved mutation produces a stable targeted failure, its CLI is deterministic, and both verifier lists execute the test/check pair immediately after Phase 117.</done>
</task>

<task type="auto">
  <name>Task 3: Refresh evidence, verify the repository, and create the single local commit</name>
  <files>docs/metrics/lines-of-code.md</files>
  <action>Treat Tasks 1-3 as one implementation commit unit; do not create intermediate commits. Regenerate the LOC report from the worktree after the new checker and tests exist, then run its worktree freshness check. Run both the test and live command for existing Phase 83, 88, 106, 117, and 124 checkers to prove the historical support and release guardrails still accept the narrow reconciliation; do not edit those existing checker files to make them pass. Run the new checker pair again, then run the full repo contract with the timing wrapper exactly as specified below, polling rather than terminating a quiet resumable session. Run `git diff --check`, inspect the full diff, and assert that no protected exclusion or unrelated user file changed. Stage only the 12 implementation/documentation targets in this plan, including the refreshed LOC report; leave GSD-owned PLAN/SUMMARY/STATE handling to the quick-task orchestrator. Create exactly one user-requested implementation commit named `docs: reconcile post-v2.1 project state`. Do not amend unrelated commits and do not push.</action>
  <verify>
    <automated>bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md &amp;&amp; bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check &amp;&amp; bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts &amp;&amp; bun run scripts/check-phase83-support-matrix-issue-evidence.ts &amp;&amp; bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts &amp;&amp; bun run scripts/check-phase88-deterministic-claim-guardrails.ts &amp;&amp; bun test scripts/check-phase106-parity-uat-release-boundary.test.ts &amp;&amp; bun run scripts/check-phase106-parity-uat-release-boundary.ts &amp;&amp; bun test scripts/check-phase117-parity-uat-release-boundary.test.ts &amp;&amp; bun run scripts/check-phase117-parity-uat-release-boundary.ts &amp;&amp; bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts &amp;&amp; bun run scripts/check-phase124-milestone-closeout-reconciliation.ts &amp;&amp; bun test scripts/check-current-documentation-reconciliation.test.ts &amp;&amp; bun run scripts/check-current-documentation-reconciliation.ts &amp;&amp; bun run scripts/command-timings.ts run --key verify-full -- bash scripts/verify.sh &amp;&amp; git diff --check</automated>
  </verify>
  <done>The LOC report is fresh, all focused and full checks pass, the reviewed diff contains only authorized live-doc/checker/verifier/LOC changes plus orchestration-owned quick artifacts, commit `docs: reconcile post-v2.1 project state` exists locally, and no push occurred.</done>
</task>

</tasks>

<threat-model>
## Trust Boundaries

| Boundary | Description |
| --- | --- |
| Repository files to checker parser | Locally tracked Markdown and Rust source are parsed as potentially drifted input; they must produce bounded deterministic failures rather than trigger execution. |
| Checker result to verification shell | The checker communicates only by stable stdout/stderr and exit status; it must not invoke processes or networks itself. |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
| --- | --- | --- | --- | --- |
| T-DOCREC-01 | Tampering | Current support and release claims | mitigate | Parse named current sections/tables and exact method sets; mutation tests prove each protected fact fails closed. |
| T-DOCREC-02 | Information disclosure | Checker diagnostics | mitigate | Read only fixed repo-relative targets and print bounded file/fact failure labels, never file contents or environment values. |
| T-DOCREC-03 | Denial of service | Checker execution | mitigate | Use synchronous bounded local reads over the fixed corpus; prohibit network, subprocess, retry, and wall-clock behavior. |
| T-DOCREC-04 | Elevation of privilege | Injected repository root | accept | The root only selects a test fixture; all joined paths are fixed constants and the checker performs no writes or process execution. |
</threat-model>

<verification>
Completion requires the new focused test/check pair, both forms of the Phase 83/88/106/117/124 guards, a fresh worktree LOC report, the timed full `bash scripts/verify.sh` contract, `git diff --check`, full diff review, protected-file review, and the exact local commit with no push. The aggregate verifier supplies the repository-required Rust formatting check, Clippy, all-target build, all-feature tests, coverage, and Bazel smoke build.
</verification>

<success-criteria>
- Current public and architectural docs describe the shipped-and-archived v2.1 state and retain explicit non-claims.
- Bounded v2.0 transaction relay is preview; only its public/default/production expansion is deferred.
- The RPC catalog exactly matches the Rust method enum and accurately scopes wallet routing and remaining deferrals.
- The local deterministic guard fails every approved mutation and is wired after Phase 117 in both verifier lists.
- Historical/protected artifacts are unchanged, all verification passes, LOC is fresh, and one user-requested implementation commit is created locally without pushing.
</success-criteria>

<output>
After completion, create `.planning/quick/260722-ctq-reconcile-post-v2-1-documentation-state-/260722-ctq-SUMMARY.md` with the changed-doc facts, checker/test coverage, verification evidence, commit hash, and residual risks.
</output>

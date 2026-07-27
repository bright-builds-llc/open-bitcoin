import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const PHASE131_CHECK =
  "bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts";
export const PHASE132_TEST =
  "bun test scripts/check-phase132-typed-package-staged-admission.test.ts";
export const PHASE132_CHECK =
  "bun run scripts/check-phase132-typed-package-staged-admission.ts";
export const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

export const REQUIRED_BREADCRUMB_PATHS = [
  "packages/open-bitcoin-mempool/src/package.rs",
  "packages/open-bitcoin-mempool/src/package/shape.rs",
  "packages/open-bitcoin-mempool/src/package/report.rs",
  "packages/open-bitcoin-mempool/src/package/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/diagram.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/diagram/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/truc.rs",
  "packages/open-bitcoin-mempool/src/policy/truc/tests.rs",
  "packages/open-bitcoin-mempool/src/pool/candidate.rs",
  "packages/open-bitcoin-mempool/src/pool/oracle.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs",
  "packages/open-bitcoin-mempool/src/pool/patch.rs",
  "packages/open-bitcoin-mempool/src/pool/patch/graph.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective/limits.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/revision_cases.rs",
] as const;

export const PHASE132_TARGET_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-consensus/src/classify.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/package.rs",
  "packages/open-bitcoin-mempool/src/package/shape.rs",
  "packages/open-bitcoin-mempool/src/package/report.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  "packages/open-bitcoin-mempool/src/policy/output.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  "packages/open-bitcoin-mempool/src/policy/truc.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective.rs",
  "packages/open-bitcoin-mempool/src/pool/tests.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  "packages/open-bitcoin-mempool/src/types.rs",
  "scripts/check-phase132-typed-package-staged-admission.ts",
  "scripts/verify.sh",
] as const;

export const NEGATED_BOUNDARY_VERB =
  /\b(?:does|do)\s+not\s+(?:add|claim|enable|expose|implement|include|provide|ship|support)\b/;
export const WITHOUT_BOUNDARY_VERB =
  /\bwithout\s+(?:adding|claiming|enabling|exposing|implementing|including|providing|shipping|supporting)\b/;
export const DEFERRED_BOUNDARY_PREDICATE =
  /\b(?:is|are|remain|remains)\s+(?:currently\s+)?(?:deferred|unsupported)\b/;
export const NEGATED_SUPPORT_BOUNDARY_PREDICATE =
  /\b(?:is|are)\s+not\s+(?:available|enabled|implemented|included|provided|supported)\b/;
export const OUTSIDE_SCOPE_BOUNDARY_PREDICATE =
  /\b(?:is|are|remain|remains)\s+outside\b[^.!?;—|]*\bscope\b/;
export const PREFIXED_DEFERRED_BOUNDARY =
  /\b(?:currently\s+)?(?:deferred|unsupported)\s+(?:the\s+)?$/;

export interface BreadcrumbManifest {
  groups: Array<{
    files: string[];
    breadcrumbs: string[];
  }>;
}

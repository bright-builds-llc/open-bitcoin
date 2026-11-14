import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REQUIRED_BREADCRUMB_PATHS, BreadcrumbManifest } from "./constants.ts";
import { hasExplicitClaimBoundary, claimClauses } from "./claims.ts";
import { readTarget } from "./filesystem.ts";
import { requireAll } from "./helpers.ts";

export function checkParityClosure(repoRoot: string, failures: string[]): void {
  const registry = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests.rs",
  );
  const breadcrumbsText = readTarget(
    repoRoot,
    "docs/parity/source-breadcrumbs.json",
  );
  if (
    !registry.includes("mod package_parity_cases;") ||
    !breadcrumbsText.includes(
      '"packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs"',
    )
  ) {
    failures.push(
      "P132 parity: package_parity_cases must remain registered and breadcrumbed",
    );
  }
  let manifest: BreadcrumbManifest | undefined;
  try {
    manifest = JSON.parse(breadcrumbsText) as BreadcrumbManifest;
  } catch {
    failures.push("P132 breadcrumbs: source-breadcrumbs.json must remain valid JSON");
    return;
  }
  for (const requiredPath of REQUIRED_BREADCRUMB_PATHS) {
    const maybeGroup = manifest.groups.find((group) =>
      group.files.includes(requiredPath),
    );
    if (
      !maybeGroup ||
      maybeGroup.breadcrumbs.length === 0 ||
      maybeGroup.breadcrumbs.includes("none")
    ) {
      failures.push(
        `P132 breadcrumbs: missing non-none Knots anchors for ${requiredPath}`,
      );
    }
  }
  const parityCases = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  );
  const requiredParityEvidence = [
    "MAX_PACKAGE_COUNT",
    "MAX_PACKAGE_WEIGHT",
    "stale_revision_sparse_patch_rejects_before_apply_without_mutation",
    "overlap_over_100_conservative_limited_rbf_count_precedes_union",
    "max_bound_generated_sparse_overlay_recompute_has_zero_clone_and_one_trim",
  ];
  const hasSplitParityRegistry =
    parityCases.includes(
      "mod dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecyc;",
    ) &&
    parityCases.includes(
      "mod max_bound_shape_fingerprint_order_and_try_from_package_refinement_are_pi;",
    );
  if (
    !requiredParityEvidence.every((needle) => parityCases.includes(needle)) &&
    !hasSplitParityRegistry
  ) {
    failures.push(
      "P132 parity: integrated package-policy matrix must retain bounds stale overlap and oracle cases",
    );
  }
}

export function checkDocumentation(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(
    repoRoot,
    "docs/parity/catalog/mempool-policy.md",
  );
  const readme = readTarget(repoRoot, "README.md");
  const packagesReadme = readTarget(repoRoot, "packages/README.md");
  requireAll(
    catalog,
    [
      "## Typed Package Vocabulary and Staged Admission",
      "PACK-01",
      "PACK-02",
      "PACK-03",
      "PACK-04",
      "PACK-05",
      "PACK-06",
      "PACK-07",
      "anchor=true",
      "send=false",
      "dust=false",
      "sparse overlay",
      "ordered",
      "one final trim",
      "general package wire",
      "guaranteed propagation",
    ],
    "P132 docs: catalog must map PACK-01 through PACK-07 and bounded intentional/deferred boundaries",
    failures,
  );
  for (const anchor of [
    "doc/policy/packages.md",
    "src/policy/packages.cpp",
    "src/validation.cpp",
    "src/test/txpackage_tests.cpp",
    "test/functional/mempool_package_rbf.py",
    "test/functional/mempool_truc.py",
    "test/functional/mempool_ephemeral_dust.py",
  ]) {
    if (!catalog.includes(anchor)) {
      failures.push(`P132 docs: catalog is missing pinned Knots anchor ${anchor}`);
    }
  }
  requireAll(
    `${readme}\n${packagesReadme}`.toLowerCase(),
    [
      "bounded local",
      "package admission",
      "peer package assembly",
      "general package wire",
      "guaranteed propagation",
    ],
    "P132 docs: READMEs must claim bounded local package admission and retain relay exclusions",
    failures,
  );
}

export function checkNarrowClaims(repoRoot: string, failures: string[]): void {
  const claimFiles = [
    "README.md",
    "packages/README.md",
    "docs/parity/catalog/mempool-policy.md",
  ];
  const forbiddenClaims = [
    "general package wire",
    "peer package assembly",
    "rpc package adapter",
    "public package relay",
    "default package relay",
    "guaranteed propagation",
    "public-network package",
    "production readiness",
  ];
  for (const relativePath of claimFiles) {
    const text = readTarget(repoRoot, relativePath);
    for (const paragraph of text.split(/\r?\n\s*\r?\n/)) {
      for (const clause of claimClauses(paragraph)) {
        const lower = clause.toLowerCase();
        for (const claim of forbiddenClaims) {
          if (!lower.includes(claim)) continue;
          if (hasExplicitClaimBoundary(lower, claim)) {
            continue;
          }
          failures.push(
            "P132 claims: bounded local admission must not become a general package-wire or production claim",
          );
          return;
        }
      }
    }
  }
  for (const relativePath of REQUIRED_BREADCRUMB_PATHS) {
    const source = readTarget(repoRoot, relativePath);
    if (
      source.includes("open_bitcoin_network") ||
      source.includes("open_bitcoin_node") ||
      source.includes("open_bitcoin_rpc")
    ) {
      failures.push(
        `P132 architecture: pure package core must not import an adapter in ${relativePath}`,
      );
    }
  }
}

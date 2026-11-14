import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { readTarget } from "./filesystem.ts";
import { requireAll, requireOrdered, orderedOffsets, countMatches, sectionBetween } from "./helpers.ts";

export function checkPack07Policy(repoRoot: string, failures: string[]): void {
  const admission = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  );
  const replacement = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  );
  const truc = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/truc.rs",
  );
  const ephemeral = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  );
  const output = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/output.rs",
  );
  const types = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/types.rs",
  );
  const ephemeralDefaults = sectionBetween(
    types,
    "impl Default for EphemeralPolicy",
    "pub struct PolicyConfig",
  );
  const policyDefaults = sectionBetween(
    types,
    "impl Default for PolicyConfig",
    "pub struct AggregateStats",
  );
  const fee = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/fee.rs",
  );
  const classify = readTarget(
    repoRoot,
    "packages/open-bitcoin-consensus/src/classify.rs",
  );
  const singletonEvaluation = sectionBetween(
    admission,
    "fn evaluate_singleton",
    "fn remove_individual_groups",
  );

  requireOrdered(
    singletonEvaluation,
    [
      "RollingMempoolFeeRate::ZERO,",
      "evaluate_truc_package(",
      "prospective.rolling_mempool_fee_rate(),",
      "validate_candidate_limits(identity.txid)",
      "validate_ephemeral_spends(",
      "run_late_script_checks(",
    ],
    "P132 PACK-07: policy order must keep static/TRUC/rolling/limits/replacement/ephemeral/late scripts",
    failures,
  );
  if (
    !orderedOffsets(singletonEvaluation, [
      "validate_ephemeral_spends(",
      "run_late_script_checks(",
    ])
  ) {
    failures.push(
      "P132 PACK-07: policy order must keep ephemeral checks before late scripts",
    );
  }
  requireAll(
    replacement,
    [
      "MAX_REPLACEMENT_CANDIDATES: usize = 100",
      "enforce_conservative_candidate_bound(view, &direct_conflicts)?;",
      "potential_count",
      ".checked_add(descendant_count)",
      "collect_removal_union(view, &direct_conflicts)",
    ],
    "P132 PACK-07: limited replacement must retain conservative pre-union 100-candidate counting",
    failures,
  );
  requireAll(
    truc,
    [
      "direct_conflicts: &BTreeSet<Txid>",
      "maybe_sibling_eviction: Option<EligibleSiblingEviction>",
      "find_sibling_eviction(view, members, direct_conflicts)",
      "validate_parent_children(",
      "!direct_conflicts.contains(txid)",
      "intent.sibling",
    ],
    "P132 PACK-07: TRUC must evaluate direct conflicts and sibling intent against pre-replacement facts",
    failures,
  );
  requireAll(
    ephemeral,
    [
      "member.fees.base.to_sats() != 0 || member.fees.modified.to_sats() != 0",
      "missing.insert(OutPoint",
      "missing.remove(&input.previous_output);",
      "if missing.is_empty()",
    ],
    "P132 PACK-07: ephemeral policy must retain zero-fee and complete-spend predicates",
    failures,
  );
  requireAll(
    ephemeralDefaults,
    ["anchor: true", "send: false", "dust: false"],
    "P132 PACK-07: ephemeral defaults must remain anchor=true send=false dust=false",
    failures,
  );
  requireAll(
    policyDefaults,
    ["permit_bare_anchor: true"],
    "P132 PACK-07: bare-anchor transaction default must remain permit_bare_anchor=true",
    failures,
  );
  requireAll(
    output,
    [
      "ScriptPubKeyType::PayToAnchor",
      "permissions.anchor",
      "permissions.send",
      "output.value.to_sats() == 0 || permissions.dust",
    ],
    "P132 PACK-07: P2A/send/dust permission predicates must remain cumulative",
    failures,
  );
  requireAll(
    classify,
    [
      "const PAY_TO_ANCHOR_PROGRAM: [u8; 2] = [0x4e, 0x73];",
      "ScriptPubKeyType::PayToAnchor",
    ],
    "P132 PACK-07: exact pay-to-anchor witness-program bytes must remain classified",
    failures,
  );
  if (!fee.includes("FeeRate::from_sats_per_kvb(3_000)")) {
    failures.push(
      "P132 PACK-07: ephemeral dust threshold must retain the 3000 sat/kvB default",
    );
  }
  if (
    countMatches(admission, /trim_prospective_to_capacity\(/g) !== 1 ||
    !admission.includes("rewrite_final_membership(&prospective, &mut results);")
  ) {
    failures.push(
      "P132 PACK-07: package execution must retain one trim and final-membership rewrite",
    );
  }
}

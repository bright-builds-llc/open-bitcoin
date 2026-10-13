# v2.5 archive-safe parity provenance

v2.5 archive readiness is established by the renewed passing milestone audit; v2.5 remains unarchived and unshipped.

This policy was introduced2026-10-12; corrected-source readiness is updated2026-10-13. The [original integrated proof](v2-5-integrated-filter-proof.md), its six source hashes and original Phase 162 certificates retain their original delivery boundary. The [original gaps audit](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md) and [integration audit](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md) preserve the later archive defect. CFGR-01 has one active closure owner, Phase 163; the other 21 requirements remain Complete.

## Evidence selection and publication

The [provenance policy](../../scripts/check-phase163-provenance.ts) selects fixed v2.5 requirements, roadmap and audit paths, independently of a later milestone's active controls. Equivalent parsed active/archive copies are allowed during the standard safety-copy overlap; conflicting identities, requirement descriptions, owners, statuses or baselines refuse. A missing or malformed candidate cannot borrow authority from an archive filename or SHIPPED header. Supplied maps are the complete evidence universe: omitted keys never fall back to valid checkout files.

Historical delivery, current requirement closure and milestone publication are separate states. Original source/function/assertion, baseline, unique parity ownership, lifecycle, frozen checker and excluded-capability guards remain required. Source hashes supplement independent execution and review; they do not establish those results by themselves. The existing [integrated-proof inventory owner](index.json) keeps its historical done status and exact three-requirement inventory; this page adds evidence to that owner rather than creating an overlapping surface.

The exported `READINESS_CLAIM` is earned by the actual corrected-source renewed audit at2026-10-13T06:00:48Z. The prior renewed passing audit, canonical functional closure, 16/16 security acceptance, full native pass and final five-criterion assessment remain dated `34443772` evidence in the [preserved reports](../../.planning/reports/phase163-before-fixture-revalidation/README.md). The corrected21-input source now has an actual full default pass, independent acceptance and canonical functional closure. The new source-bound audit now passes22/22requirements,31/31seams and18/18flows with zero blocking gaps; readiness is now published. FinalSC1–SC5phaseverification nowpasses. At a later actual archive, when standard archived controls become authoritative, replace current unarchived closure/readiness wording with `ARCHIVED_CLAIM`; stale current unarchived claims must refuse. Preserve dated Phase 162 proof/certificates and all historical phase directories consumed by native guards. Actual archive, tag and shipment require their separate workflow.

The [transition evidence](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-TRANSITION-EVIDENCE.md) distinguishes observed pre-extension suites from the later source-binding rerun. The archive, completion and passing-audit layouts are synthetic fixture data. Tests use finite owned regular-file corpora and real CLI/filesystem/map entrypoints, then remove their temporary roots. They reproduce the safety copy, audit move, active requirements deletion and later active milestone without archiving this checkout. Their archive transition also reconciles current claim prose; filesystem movement alone is insufficient. Those focused results alone do not establish live closure. The corrected21-input [native receipt](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-NATIVE-EVIDENCE.md) and [canonical functional verification](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-VERIFICATION.md) now earn CFGR-01 closure on `e0c16fb2`. The earlier `34443772` full/audit/final receipts are preserved as dated history. The renewed actual audit now earns readiness; readiness-state native checks and finalSC1–SC5phase reconciliation nowpass. Normal-hook source saving remains separate.

## Reproduction from the repository root

Use the pinned Bun 1.3.9 installation on this host. These are reproduction instructions; listing a command is not an execution receipt. The full default verifier remains the pre-commit/release contract; fast iteration cannot substitute for it.

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH
bun --version
bun test ./scripts/check-phase163-provenance.test.ts
bun test ./scripts/check-phase163-provenance/native.test.ts
bun run scripts/check-phase158-validated-reorg.ts
bun run scripts/check-phase162-integrated-filter-proof.ts
bun run scripts/check-active-milestone-verification-traceability.ts
bash scripts/verify.sh
```

Run Cargo and Bazel commands serially through the timing runner. These operator help commands expose existing repo-local entrypoints; they create no archive or retained-client fixture:

```bash
bun run scripts/command-timings.ts run --key phase163-cargo-help -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --help
bun run scripts/command-timings.ts run --key phase163-bazel-help -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --help
```

The [dated Phase 162 UAT](../../.planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-UAT.md) supplies the genuine lifecycle and fault selectors, nonzero discovery requirements, exact operator forms and resource observations. The [current handoff](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md) attributes historical native/CI evidence and remaining gates.

## Preserved product and evidence limits

Scope remains BASIC/type 0 on one active Fjall chainstate, explicit/default-off indexing/networking and missing-history refusal. Filters restore neither deleted bodies nor wallet eligibility and add no wallet acceleration. The 550 MiB logical body/undo target remains soft and unattained in the measured pressure scenario; allocated-file samples are points, with no peak or reclamation guarantee. Synthetic-root, caller-selected work and Open Bitcoin's 210000 versus Knots regtest's 150 halving interval exclude whole-chain Knots consensus differential claims.

The inherited v2.4 stale-metadata finish-or-Repair refusal, generic paired-unlink sink default and crash-window support-counter undercount advisories remain unfixed. BIP37/V0, assumeutxo/dual-chainstate, archive-scale serving, public/default networking, public-network CI, implicit repair/import/download, production service/readiness and production-funds safety remain excluded. Provenance archival adds no runtime capability.

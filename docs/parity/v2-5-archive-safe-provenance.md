# v2.5 archive-safe parity provenance

v2.5 controls are archived under .planning/milestones/; this provenance does not establish new runtime capability or production/funds safety.

This policy was introduced on 2026-10-12; milestone control archival completed on 2026-10-13. The [original integrated proof](v2-5-integrated-filter-proof.md), its six source hashes and original Phase 162 certificates retain their original delivery boundary. The [original gaps audit](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md) and [integration audit](../../.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md) preserve the later archive defect. Archived CFGR-01 closure belongs uniquely to Phase 163; the other 21 requirements remain Complete.

## Evidence selection and publication

The [provenance policy](../../scripts/check-phase163-provenance.ts) selects fixed v2.5 requirements, roadmap and audit paths, independently of a later milestone's active controls. Equivalent parsed active/archive copies are allowed during the standard safety-copy overlap; conflicting identities, requirement descriptions, owners, statuses or baselines refuse. A missing or malformed candidate cannot borrow authority from an archive filename or SHIPPED header. Supplied maps are the complete evidence universe: omitted keys never fall back to valid checkout files.

Historical delivery, current requirement closure and milestone publication are separate states. Original source/function/assertion, baseline, unique parity ownership, lifecycle, frozen checker and excluded-capability guards remain required. Source hashes supplement independent execution and review; they do not establish those results by themselves. The existing [integrated-proof inventory owner](index.json) keeps its historical done status and exact three-requirement inventory; this page adds evidence to that owner rather than creating an overlapping surface.

Current source acceptance for the 21-input digest `14fa9cc5` includes all 176 default-verifier stages, four layout suites of 5,032 tests each, clean independent review, 16 verified security mitigations, functional activation, the renewed audit, readiness execution and final SC1–SC5 verification. All 22 requirements have unique Complete owners; all five phase plans, 90 milestone plans and ten phases are complete. Earlier e0 acceptance and failed or interrupted receipts retain their original sources, dates and outcomes. Normal-hook source saving and actual archival remain separate. Runtime, resource, networking and funds-safety limits are unchanged.

## Reproduction from the repository root

Use the pinned Bun 1.3.9 installation on this host. These are reproduction instructions; listing a command is not an execution receipt. The full default verifier remains the pre-commit/release contract; fast iteration cannot substitute for it.

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
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

## Archived milestone controls

The versioned [roadmap](../../.planning/milestones/v2.5-ROADMAP.md), [requirements](../../.planning/milestones/v2.5-REQUIREMENTS.md) and [audit](../../.planning/milestones/v2.5-MILESTONE-AUDIT.md) are authoritative after the committed safety checkpoint and retirement of active requirements. Historical phase folders and source receipts remain in place. The current exact archived claim above records this location boundary. Actual saving/tag outcomes are derived from Git and command receipts.

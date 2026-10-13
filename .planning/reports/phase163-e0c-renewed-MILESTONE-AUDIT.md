---
milestone: v2.5
milestone_name: Prune-Aware Compact-Filter Serving
audited: "2026-10-13T06:00:48Z"
generated_by: gsd-audit-milestone
phase_lifecycle_id: 163-2026-10-12T21-09-41
source_commit: d47f6e09b4621808974cc083fcc27ee9d2c1b4c7
source_worktree_dirty: true
source_digest: e0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d
source_inventory_count: 21
status: passed
scores:
  requirements: 22/22
  phases: 10/10
  integration: 31/31
  flows: 18/18
scope:
  phases: 10
  plans: 90
  summaries: 90
  completed_plans_at_audit: 88
  completed_phases_at_audit: 9
  final_phase_certificates_at_audit: 9
  functional_phase_certificates_at_audit: 1
  historical_verification_bytes_fully_read: 437074
  current_functional_verification_bytes_fully_read: 23119
  historical_summary_frontmatters_parsed: 85
  current_summary_frontmatters_parsed: 5
integration_report_sha256: b72d68f391a6bc46b2c1aa0c34705dc5c967418590ee06eb698abb318cd20f22
functional_verification_sha256_at_audit: dbee7fe9f8a146273f9829b3618d8e01741361b997facde3660903975892c55d
native_run_id: 38c0607f-9a5a-4180-9aa8-10dd17163f26
activation_run_id: 2ea6c840-d47d-4073-a20e-cec1b178af96
readiness_publication_at_audit: not_asserted
phase163_status_at_audit: in_progress
gaps:
  requirements: []
  integration: []
  flows: []
tech_debt:
  - id: HUMAN-156
    phase: 156
    items:
      - Original debugger human-confirmation condition remains unperformed; its session remains unarchived. This audit does not claim that separate condition was fulfilled.
  - id: INHERITED-V24
    phase: v2.4
    items:
      - Retained stale-metadata recovery refusal, generic sink no-op default and crash-window support-counter undercount advisories remain accepted and unfixed; unrelated cleanup is outside v2.5 scope.
---

# v2.5 corrected-source renewed milestone audit

**PASS:22/22requirements,10/10scoped certificates,31/31connected seams,18/18flows; zero blocking gaps/orphans.** This is a genuinely new assessment of corrected21-input `e0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d`, not a redate of source344 or7ed receipts. At this actual audit boundary there are nine final certificates plus one fresh functional Phase163certificate; canonical04/05 and Phase163remainin_progress,3/5phaseplans,88/90milestoneplans,9/10phases. The phases score means scoped verification coverage, not ten completed final phases. Readiness publication, final SC1–SC5certificate and completion counters follow this passing audit as explicitly required by approved Plan04/05.

Actual archival, tag and shipment remain separate and unperformed. Audit readiness does not add runtime capability or production/funds safety.

## Scope and method

Active ROADMAP scopes Phases154–163 and90plans. Historical directories remain tracked for native consumers; a global directory count/file-presence completion estimate is not used as active completion evidence. Two independent readers fully read allnine historical canonical verification bodies (437,074bytes), parsed all85historical SUMMARYfrontmatters and mapped21olderactiveIDs. Root fully read current163functional verification (23,119bytes), parsed currentfive SUMMARYfrontmatters and independently cross-referenced all22checkboxes, owners, SATISFIED rows and completion claims. Every active owner is unique; historical162CFGR-01 remains separate delivery evidence. Ten strict lifecycles independently returnvalid/reasonsnone. Nyquist discovery is skipped because workflow.nyquist_validation isfalse.

Actual [renewed integration report](v2.5-INTEGRATION-AUDIT.md), generated05:53:19Z, independently reassesses31named seams,18flows and22wired requirements. Original30runtime seams/17runtime flows carry forward through unchanged Rust/production adapter evidence and new complete native execution; the extra archive/next-milestone seam/flow has actual CLI/filesystem/map, helper-layout and refusal evidence. Original f14/f60audits, six-root historical proof and prior344completed reports remain unchanged in [preserved history](reports/phase163-before-fixture-revalidation/README.md). Intermediate7edsecurity and initial05:18e0functional acceptance retain their actual dates.

## Actual current-source acceptance

- Full/default run38c0607f-9a5a-4180-9aa8-10dd17163f26,04:24:45.370Z–05:08:05.389Z,2,600,019ms; all176unique stages success/exit0/full. Whole logSHA52d1c1553adf16cc61281407950f7aefc71e38d6f34517e96d7f836858dd9770.
- Current [native report](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-NATIVE-EVIDENCE.md) records124pure+160native/0fail,1,794assertions,160owned roots removed,147files/3,585,615bytes/corpus51f8c503…,4354workspace/0fail/6existingignored,7CLI/0fail,strictfmt/clippy/build,benchmarks,sixBazel targets andpure-corecoverage. All21pre/postsource hashes match; all20original inputs remain plus explicitly bound pure fixture helper.
- Current [source review](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-REVIEW.md) is clean21/zero findings, SHA11fb37b9e534cc2e400e9100672162f4a3d1c22cb0637a7fb21e419787a198be; [security](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-SECURITY.md)16/16closed/zeroopen, SHA1762177298de89abce4588a2d718cd9c8407812515560928e074eb1ffb8b3e05.
- Actual functional activation2ea6c840-d47d-4073-a20e-cec1b178af96,05:23:52.456Z–05:28:00.356Z,247,900ms,284pass/0fail/1,794assertions;160roots,147files/3,528,801bytes/corpus10c7253a…. Live witnesses unchanged.
- [Current canonical functional verification](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-VERIFICATION.md) genuinely refreshed05:36:20Z after current04/05summary generation,4/4SC1–SC4,sourcee0/21,strictlifecyclevalid; SHA at this auditdbee7fe9f8a146273f9829b3618d8e01741361b997facde3660903975892c55d. SC5/readiness/final phase is conditional future work at this boundary.

## Three-source requirement coverage

Every row is checked/Complete in current REQUIREMENTS, SATISFIED in its owning canonical verification and listed in an owning SUMMARY's requirements-completed. No requirement is partial, unsatisfied, missing or orphaned. The canonical04carrier may remain executionin_progress while actual functional completion is activated; its final plan state waits for Plan05 by approved design.

| Requirement | Active owner | Completing SUMMARY                                                                                  | Owning verification                                                                        | Status                 |
| ----------- | ------------ | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ---------------------- |
| CFIL-01     | 154          | [154-03-SUMMARY](phases/154-basic-generation-and-commitment-parity/154-03-SUMMARY.md)               | [154](phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md)               | SATISFIED              |
| CFIL-02     | 154          | [154-03-SUMMARY](phases/154-basic-generation-and-commitment-parity/154-03-SUMMARY.md)               | [154](phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md)               | SATISFIED              |
| CFIX-02     | 155          | [155-02-SUMMARY](phases/155-recoverable-index-and-pre-prune-startup-protection/155-02-SUMMARY.md)   | [155](phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md)   | SATISFIED              |
| CFIX-04     | 155          | [155-02-SUMMARY](phases/155-recoverable-index-and-pre-prune-startup-protection/155-02-SUMMARY.md)   | [155](phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md)   | SATISFIED              |
| CFPR-03     | 155          | [155-03-SUMMARY](phases/155-recoverable-index-and-pre-prune-startup-protection/155-03-SUMMARY.md)   | [155](phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md)   | SATISFIED              |
| CFPR-01     | 156          | [156-08-SUMMARY](phases/156-index-owned-manual-and-automatic-prune-coordination/156-08-SUMMARY.md)  | [156](phases/156-index-owned-manual-and-automatic-prune-coordination/156-VERIFICATION.md)  | SATISFIED              |
| CFAC-01     | 157          | [157-10-SUMMARY](phases/157-safe-activation-and-scheduled-index-catch-up/157-10-SUMMARY.md)         | [157](phases/157-safe-activation-and-scheduled-index-catch-up/157-VERIFICATION.md)         | SATISFIED              |
| CFAC-02     | 157          | [157-10-SUMMARY](phases/157-safe-activation-and-scheduled-index-catch-up/157-10-SUMMARY.md)         | [157](phases/157-safe-activation-and-scheduled-index-catch-up/157-VERIFICATION.md)         | SATISFIED              |
| CFIX-01     | 157          | [157-10-SUMMARY](phases/157-safe-activation-and-scheduled-index-catch-up/157-10-SUMMARY.md)         | [157](phases/157-safe-activation-and-scheduled-index-catch-up/157-VERIFICATION.md)         | SATISFIED              |
| CFIX-03     | 158          | [158-07-SUMMARY](phases/158-validated-reorg-and-retained-branch-identity/158-07-SUMMARY.md)         | [158](phases/158-validated-reorg-and-retained-branch-identity/158-VERIFICATION.md)         | SATISFIED              |
| CFRP-01     | 159          | [159-08-SUMMARY](phases/159-authenticated-basic-filter-and-index-rpcs/159-08-SUMMARY.md)            | [159](phases/159-authenticated-basic-filter-and-index-rpcs/159-VERIFICATION.md)            | SATISFIED              |
| CFRP-02     | 159          | [159-08-SUMMARY](phases/159-authenticated-basic-filter-and-index-rpcs/159-08-SUMMARY.md)            | [159](phases/159-authenticated-basic-filter-and-index-rpcs/159-VERIFICATION.md)            | SATISFIED              |
| CFNET-01    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFNET-02    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFNET-03    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFNET-04    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFNET-05    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFNET-06    | 160          | [160-12-SUMMARY](phases/160-explicit-bounded-peer-filter-serving/160-12-SUMMARY.md)                 | [160](phases/160-explicit-bounded-peer-filter-serving/160-VERIFICATION.md)                 | SATISFIED              |
| CFOP-01     | 161          | [161-09-SUMMARY](phases/161-shared-operator-index-and-retention-evidence/161-09-SUMMARY.md)         | [161](phases/161-shared-operator-index-and-retention-evidence/161-VERIFICATION.md)         | SATISFIED              |
| CFGR-02     | 162          | [162-12-SUMMARY](phases/162-real-prune-retention-and-integrated-parity-proof/162-12-SUMMARY.md)     | [162](phases/162-real-prune-retention-and-integrated-parity-proof/162-VERIFICATION.md)     | SATISFIED              |
| CFPR-02     | 162          | [162-12-SUMMARY](phases/162-real-prune-retention-and-integrated-parity-proof/162-12-SUMMARY.md)     | [162](phases/162-real-prune-retention-and-integrated-parity-proof/162-VERIFICATION.md)     | SATISFIED              |
| CFGR-01     | 163          | [163-04-SUMMARY](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-04-SUMMARY.md) | [163](phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-VERIFICATION.md) | SATISFIED (functional) |

## Phase evidence and integration

| Phase | Current scoped certificate | Active requirement count | State at audit                                    |
| ----- | -------------------------- | -----------------------: | ------------------------------------------------- |
| 154   | passed,final               |                        2 | complete                                          |
| 155   | passed,final               |                        3 | complete                                          |
| 156   | passed,final               |                        1 | complete;separate human debug condition preserved |
| 157   | passed,final               |                        3 | complete                                          |
| 158   | passed,final               |                        1 | complete                                          |
| 159   | passed,final               |                        2 | complete                                          |
| 160   | passed,final               |                        6 | complete                                          |
| 161   | passed,final               |                        1 | complete                                          |
| 162   | passed,final               |                        2 | complete;CFGR-01historicalonly                    |
| 163   | passed,functionalSC1–SC4   |                        1 | 04/05/phasein_progress;SC5pending                 |

Integration report contains each named seam/flow and per-requirement completing SUMMARY/canonical-verification links. No missing consumer, orphan export, broken flow, unprotected flow or source blocker remains. Actual original8archived fixture failures now pass on corrected source; all15postfix outputs (live control+six production boundaries+eight former failures) exit0 without error/signal, alongside allthree-layout helper/18boundary and nested native/witness regression evidence. Raw production invalid maps remain strict and are not normalized by the test-only helper.

## Reconciled records and preserved limits

Stale Pending/21Complete current prose and progress checkboxes were reconciled after their actual frozen execution boundaries. TD01/TD02now distinguish priorCI/native results from currentlocalfull6; only debugCurrentFocus/update times changed, preserving original failures/fixes and the Phase156unperformed human condition. Currentlocalmanagedchecks scan1494sources with0findings. No new remoteCI success is claimed before saving.

BASIC/type0, singleactiveFjall, explicit/default-off indexing/networking, exact-tip/bounded-work/resource contracts and missing-history/body/wallet restrictions remain. The measured550MiBlogicaltarget remains soft/unattained; physical allocation is point evidence. Synthetic-root, caller-selectedwork and210000-versus150halving limits exclude whole-chain consensus parity. HTTP-handler versus physicalserver, privatefault/replay composition and UnknownLegacy distinctions retain their declared boundaries. Inheritedv2.4advisories remain unfixed. BIP37/V0,assumeutxo/dualchainstate,archive-scale,publicdefaults/public-networkCI,implicitrepair/import/download andproduction/service/funds claims remain excluded.

## Read evidence and next gate

Independent raw read artifacts are local corrected-audit-coverage-a.json SHA29a1f67436d353186ba33983fa3f93126a185104d89a4e34e31291b1c41478d0 and-b.json SHAf029526386ab9ae447ee619501e43d3da327946da8882d0c9e5ecdc9fcc591dc. Root22-rowcrossref SHAae57011d2353a414abf9c735fe49dbdfd88a3bcc063f125308f635d52d4fd973; strictlifecycle results corrected-audit-lifecycles.json. Old canonicalreports were fully read; SUMMARYfrontmatters were parsed as required, with fullhistoricalSUMMARYbody review belonging to the separate archival preparation. No unavailable or unread canonical verification is hidden.

This actual PASS now permits earned readiness publication and its current-state guard/native checks. Final phase verification and04/05/global counter reconciliation must follow before the normal-hook source commit/push and the separate real archival handoff. This report does not infer those later outcomes from its own existence.

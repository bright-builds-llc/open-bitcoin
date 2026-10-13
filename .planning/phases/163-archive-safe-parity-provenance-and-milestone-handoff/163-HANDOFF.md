---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-12T22:35:50Z"
status: phase-complete
requirements-completed: []
---

# Phase 163 current milestone handoff

v2.5 archive readiness is established by the renewed passing milestone audit; v2.5 remains unarchived and unshipped.

## Historical delivery and actual receipts

Phases 154–162 completed the original 85-plan delivery. [Phase 162 formal verification](../162-real-prune-retention-and-integrated-parity-proof/162-VERIFICATION.md) records 46/46 obligations, clean independent source review and 61 closed mitigations. The original [proof](../../../docs/parity/v2-5-integrated-filter-proof.md), certificates and ordered six-source digest `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575` remain unchanged. CFPR-02/CFGR-02 remain Complete; the later [original milestone](163-ORIGINAL-MILESTONE-AUDIT.md) and [integration](163-ORIGINAL-INTEGRATION-AUDIT.md) audits reopened only CFGR-01's archive handoff. Their SHA256 values remain `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8` and `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709`.

Read-only inspection on 2026-10-12 verified these actual ignored local `verify-full` JSON receipts:

| Receipt                                | UTC interval              | Result and boundary                                                            |
| -------------------------------------- | ------------------------- | ------------------------------------------------------------------------------ |
| `6c7c6001-7fbd-42ec-875d-840c0c380707` | 02:51:15.991–03:26:53.544 | success/exit0/full; 2,137,553ms wrapper; standalone Phase 162 default contract |
| `b427fa6a-122a-4c28-9dcb-e58342ec3eac` | 04:32:30.640–05:06:55.177 | success/exit0/full; 2,064,537ms wrapper; Phase 162 normal full-hook evidence   |

Their filenames under `.local/open-bitcoin-dev/command-timings/verify-full/` are `2026-10-12T02-51-15.991Z-6c7c6001-7fbd-42ec-875d-840c0c380707.json` and `2026-10-12T04-32-30.640Z-b427fa6a-122a-4c28-9dcb-e58342ec3eac.json`. Both record baseline `a77edab276a9daa086055aa61e6143ce86bc1218`, `dirty: true`, Darwin arm64 and Rust 1.94.1. They are historical worktree executions, not exact-d47 runs or current Phase 163 acceptance. [162-NATIVE-EVIDENCE.md](../162-real-prune-retention-and-integrated-parity-proof/162-NATIVE-EVIDENCE.md) preserves the inner 35m36.960s duration, failed attempts, actual scope and later publication refresh boundaries.

Read-only GitHub queries independently returned completed/success for [CI 38206596321](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596321) and [Bright Builds 38206596267](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596267), both at exact `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`. These remote statuses certify that historical head only. Both [Phase 156 writers](../../debug/phase156-automatic-writers.md) and [Bright Builds lengths](../../debug/bright-builds-file-lengths-ci.md) now retain dated automated dispositions. Human confirmation was not performed; Phase 156's original human-confirmation archival condition remains unsatisfied.

## New focused evidence and remaining gates

[Plan 01](163-01-SUMMARY.md) implemented fixed milestone controls and honest Pending claims; [scoped checker review](163-CHECKER-REVIEW.md) records its exact accepted byte manifest, not whole-phase acceptance. [Plan 02](163-02-SUMMARY.md) and [transition evidence](163-TRANSITION-EVIDENCE.md) separately attribute the original 171/0 pure/native, 3,203/0 historical158/162 and 1,528/0 caller159–161 suites. The later 20-input source-binding extension has a fresh-import 201/0 pure/native receipt in 133.07s; it does not relabel earlier source snapshots. A further coordinated page-scanning follow-up and the combined documentation/control freeze require their own final checks.

Completed, passing-audit and archived fixture certificates are synthetic data. Actual CLI/filesystem/map tests reproduce the safety copy, audit move, active requirements removal and later milestone in owned finite temporary layouts, including the required current claim reconciliation at the archive boundary. They preserve the original dated proof and remove their roots; they do not archive this checkout or activate live requirements.

The corrected21-input `e0c16fb2` [native pass](163-NATIVE-EVIDENCE.md), clean source review,16/16 security acceptance and [canonical functional verification](163-VERIFICATION.md) now earn CFGR-01 closure. Actual full6 is38c0607f,2,600,019ms/all176stages0; source21 remains unchanged. All22 requirements are Complete with unique owners. Canonical [Plan04](163-04-SUMMARY.md) and distinct [Plan05](163-05-SUMMARY.md) are complete, with5/5phaseplans,90/90milestoneplans,10/10phases. Actual activation2ea6passed284/0 and the actual corrected-source renewed audit06:00:48Z passes22/22requirements,31/31seams,18/18flows. Readiness is published, actualcb2bbcae284-case readiness checks passed, and fresh06:23:00ZfinalSC1–SC5verification/source21passes. Prior344receipts are preserved in [historical reports](../../reports/phase163-before-fixture-revalidation/README.md). Normal-hook commit/push remains the original wrapper gate; actual archive/tag/shipment remains separate. Current handoff updated2026-10-13T05:21:19Z.

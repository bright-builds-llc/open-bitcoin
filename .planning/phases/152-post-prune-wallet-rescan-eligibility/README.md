# Phase 152: Post-Prune Wallet Rescan Eligibility

Status: Pending planning. Depends on Phase 151. Requirement: SNAP-01.

Closes INT-02 from [the v2.4 audit](../../v2.4-MILESTONE-AUDIT.md). Node and durable RPC scans must enforce creating-payload eligibility for every admitted entry, including midrange/chunk-resume scans after real paired pruning. Refusal preserves prior wallet state; leftover snapshots remain non-authoritative.

The [roadmap](../../ROADMAP.md) defines the four planning tasks and success criteria. Run `/gsd-plan-phase 152`. This directory is a phase entry; it contains no execution plan, gathered context, or completion evidence yet.

Verification must use real-storage regressions on both adapters and the default `bash scripts/verify.sh` contract, including reopen/resume and a retained-payload control.

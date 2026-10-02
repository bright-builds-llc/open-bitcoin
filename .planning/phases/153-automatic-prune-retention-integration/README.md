# Phase 153: Automatic Prune Retention Integration

Status: Pending planning. Depends on Phase 152. Requirements: PRUN-01, PRUN-02.

Closes INT-01 from [the v2.4 audit](../../v2.4-MILESTONE-AUDIT.md). Wire measured retained payload usage and resolved mode into the existing automatic planner and production durable lifecycle. Preserve keep-window, prune-after, durable locks, paired deletion, error-path cache/undo cleanup, honest status, and finish-or-refuse recovery.

The [roadmap](../../ROADMAP.md) defines the four planning tasks and success criteria. Run `/gsd-plan-phase 153` after Phase 152. This directory is a phase entry; it contains no execution plan, gathered context, or completion evidence yet.

Verification must trigger real deletion through the automatic production lifecycle, exercise no-op/protected cases and reopen, and run default `bash scripts/verify.sh`. The temporary IBD target remains deferred. Re-audit v2.4 after closure.

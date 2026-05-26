---
phase: 46
phase_name: Durable Recovery and Invalid Data Handling
phase_lifecycle_id: 46-2026-05-26T17-16-33
lifecycle_mode: yolo
generated_at: 2026-05-26T17:16:33Z
generated_by: gsd-yolo-discuss-plan-execute-commit-and-push
requirements:
  - NODE-02
  - NODE-03
  - NODE-05
---

# Phase 46 Discussion Log

## Yolo Recommendation

Proceed with the additive durable recovery surface now. The runtime already persists validated headers, block bodies, chainstate snapshots, peer outcomes, health signals, and storage recovery metadata. The missing phase work is to make the recovery boundaries explicit in status and to close deterministic tests around invalid block data.

## Questions Resolved

- Should `block_height` change meaning?
  - No. Preserve it as connected height and add explicit fields for downloaded and connected heights.
- Should invalid peer data fail the entire sync run?
  - No. Keep the current peer-rotation behavior. Persist the latest peer failure and recovery guidance while allowing a replacement peer to succeed.
- Should this phase add a new durable download queue?
  - No. The existing durable block store plus header best-chain reconciliation is enough for the stated recovery behavior.
- Should storage recovery guidance be overridden by peer guidance?
  - No. Storage metadata takes precedence because incompatible or corrupt stores require operator action before retry.
- Should cancellation be implemented as a new runtime API?
  - No. This phase documents intentional cancellation in the operator surface and keeps code changes to existing status/runtime behavior.

## Risk Notes

- Additive status fields require updating all test fixtures that construct `SyncProgress`.
- Classifying all chainstate block validation failures as invalid peer data is appropriate for sync block messages, but local chainstate tests should remain unchanged.
- Recovery guidance should be specific without implying automated store mutation.

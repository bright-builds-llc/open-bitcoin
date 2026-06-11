# Feature Research: v1.6 Mainnet Full-Sync Completion

**Domain:** Explicit opt-in `open-bitcoind` mainnet sync-to-tip and stay-current operation  
**Researched:** 2026-06-11  
**Confidence:** HIGH for requirement shape; MEDIUM for implementation sequencing until phase plans inspect the current sync and chainstate internals.

## Research Frame

v1.6 should make a narrow but meaningful claim: a source-built, explicit opt-in
`open-bitcoind` workflow can sync the active mainnet chain to the current tip,
survive restarts, and stay current with new blocks through deterministic,
auditable behavior. That claim is only credible if it is based on validated
headers, downloaded blocks, connected active-chain state, durable UTXO or
chainstate persistence where needed, reorg-safe tip tracking, and operator
evidence that can be reproduced without putting public-network checks into
default verification.

This research was materially informed by repo-local guidance in `AGENTS.md`,
Bright Builds guidance in `AGENTS.bright-builds.md`, the v1.6 project goal in
`.planning/PROJECT.md`, the shipped v1.5 milestone archive, and the repo
constraint that public-network checks remain opt-in.

## Table Stakes

These are the user-facing and operator-facing capabilities that should be
treated as required for a credible v1.6 milestone.

| Capability | Why It Is Table Stakes | Requirement Implication |
| --- | --- | --- |
| Full active-chain sync to tip | A sync-to-tip claim is not satisfied by bounded header or block progress. Operators need evidence that the daemon reaches the current best known mainnet tip. | Track validated headers, downloaded blocks, connected height/hash, best-chain work, and tip freshness as one coherent state. |
| Mainnet-scale block validation and connect | The project cannot claim synced state if blocks are merely fetched. Validation and connect behavior must be truthful at mainnet scale. | Persist and recover the chainstate/UTXO data needed to reconnect, resume, and prove active-chain connection without replaying unsafe partial state. |
| Stay-current operation after IBD | Sync-to-tip is incomplete if the daemon reaches tip once and then stalls. | Continue headers/block polling, peer rotation, block announcement handling, and tip freshness checks after initial catch-up. |
| Restart-safe progress through long IBD | Full IBD can outlive a shell session, service cycle, machine sleep, or crash. | Reopen the same datadir, report clean versus unclean shutdown, clear stale in-flight work, and resume without duplicate connects or lost progress counters. |
| Reorg-aware active-chain handling | Tip tracking must tolerate normal mainnet reorgs and invalid competing branches without corrupting durable state. | Detect competing branches, disconnect or reconnect bounded state as needed, preserve best-chain selection, and expose reorg evidence. |
| Peer health and no-progress recovery | A full sync depends on many hours of public peer behavior; one bad peer, stall, timeout, or incompatible transcript cannot wedge the run. | Rotate peers, preserve contribution attribution, apply bounded retry/backoff, and report typed no-progress causes with next actions. |
| Bounded resources for full-chain operation | A successful sync-to-tip claim must not rely on unbounded memory, logs, peer queues, or support artifacts. | Document and test bounds for peers, in-flight blocks, request queues, cache sizes, storage writes, metrics, logs, and generated reports. |
| Truth-aligned operator surfaces | Operators should not have to reconcile conflicting status, dashboard, RPC, logs, metrics, support bundle, and live-smoke output. | Reuse a shared sync truth contract that clearly distinguishes headers-only, downloaded, connected, validated, current, stalled, and recovering states. |
| Opt-in full-sync UAT evidence | The milestone needs real public-network evidence, but default verification must stay deterministic. | Provide copy-pasteable Cargo and Bazel commands for opt-in full-sync review, tip freshness review, restart/resume review, and support bundle collection. |
| Release-boundary and threat-model updates | v1.6 expands the sync claim, not the product claim. | Refresh parity, threat model, readiness, and docs so v1.6 does not imply production-node, inbound serving, relay, wallet, packaging, migration apply, GUI, or hosted-dashboard scope. |

## Differentiators and Optional Follow-Ups

These capabilities would make v1.6 more operator-friendly or easier to audit,
but they should not displace the table-stakes sync-to-tip work.

| Capability | Value | Suggested Treatment |
| --- | --- | --- |
| Full-sync evidence timeline | A compact timeline of height/hash/work, peer changes, stalls, restarts, reorgs, and final tip freshness would make support review much easier. | Strong differentiator; include if it reuses existing metrics/log/support infrastructure. |
| Tip freshness SLA language | Operators benefit from knowing whether "current" means equal to peer tip, within N blocks, or within a time window. | Consider table stakes if the requirement text needs precise pass/fail criteria; otherwise add as release-readiness polish. |
| Peer quality scoring | Ranking peers by useful progress, timeout rate, bad data, and disconnect behavior can improve long runs. | Optional unless current peer rotation cannot sustain full sync. Keep scoring deterministic and bounded. |
| Disk and cache preflight estimate | Long IBD fails late if disk space, file handles, or cache settings are inadequate. | Useful operator differentiator; table stakes only for preventing known destructive or misleading failures. |
| Knots comparison transcript for tip evidence | A side-by-side report against the pinned Knots baseline would strengthen parity confidence. | Optional follow-up unless parity docs require fresh external comparison for the v1.6 claim. |
| Prune-aware or partial-storage modes | Some operators may ask for reduced disk usage. | Defer unless already designed; v1.6 should first prove the straightforward sync-to-tip path. |
| Alerting hooks | Local notifications for stalled sync, low disk, or fresh tip reached would be useful. | Optional later operator polish; avoid broad service or hosted-dashboard expansion. |
| Performance tuning dashboard | Throughput charts, cache hit ratios, and ETA can help long reviews. | Optional if the shared truth contract already exposes safe bounded metrics. |

## Anti-Features

These are likely requests or shortcuts that should be explicitly excluded from
v1.6 to keep the milestone honest.

| Anti-Feature | Why It Is Tempting | Why It Should Stay Out | Safer Alternative |
| --- | --- | --- | --- |
| Claiming full sync from headers-only or downloaded-only evidence | It is easier to reach high heights without full connect semantics. | It would overstate node correctness and break the project parity posture. | Require connected active-chain height/hash/work and durable state evidence. |
| Skipping validation or UTXO persistence to reach tip faster | It makes live demos easier. | It invalidates the sync-to-tip claim and creates unsafe restart behavior. | Keep validation/connect truth explicit, and scope any fast-path as diagnostic-only if needed. |
| Adding public-network checks to `bash scripts/verify.sh` | It looks like stronger CI coverage. | It makes default verification nondeterministic and contradicts shipped constraints. | Keep deterministic fixtures in default verify and public-mainnet UAT opt-in. |
| Broad production full-node marketing | Syncing to tip is a major milestone. | Inbound serving, relay, compact blocks, packaging, support policy, and production readiness remain deferred. | Use "explicit opt-in mainnet full-sync review" language. |
| Inbound serving, address advertisement, transaction relay, or compact block relay | These are natural next full-node features. | They materially expand peer responsibilities, resource governance, and parity scope. | Defer to a dedicated production-node or relay milestone. |
| Production-funds wallet claims | A current chain improves wallet usefulness. | Wallet safety and funds handling need their own threat model and parity evidence. | Preserve existing practical wallet boundaries and defer production-funds language. |
| Migration apply mode or source datadir mutation | Operators with existing nodes want easy migration. | Existing Core/Knots datadirs and wallets are high-value data. | Keep dry-run-only migration posture until a dedicated safety design. |
| Packaging, hosted dashboard, GUI, or Windows service polish | These improve adoption. | They do not prove mainnet sync-to-tip correctness and would dilute the milestone. | Keep terminal-first source-built workflows and document deferred scope. |
| Hard-coded trusted peers or centralized tip oracles | They can make UAT less flaky. | They weaken the public-network parity claim and may hide peer compatibility issues. | Allow explicit manual peers for diagnosis, but do not make them hidden defaults. |
| Unbounded logs, metrics, support bundles, or peer queues | Full sync produces a lot of evidence. | It creates disk and memory risks during exactly the long run v1.6 is trying to support. | Preserve bounded retention with summarized timelines and redaction. |

## Dependencies on Shipped v1.5

v1.6 should build directly on v1.5 rather than re-solving operator readiness.

| Shipped v1.5 Capability | How v1.6 Should Use It |
| --- | --- |
| Explicit unattended daemon sync loop | Extend the loop from bounded operator-review targets to full active-chain completion and stay-current operation. |
| Documented stop conditions, pause/resume, clean shutdown, retry/backoff | Add sync-to-tip and stay-current stop/recovery states without weakening existing operator controls. |
| Resource and recovery taxonomy | Reuse the typed recovery categories and add full-chain-specific causes such as reorg recovery, low disk, chainstate replay, and stale tip. |
| Long-run sync truth contract | Extend shared status fields to distinguish validated header, downloaded block, connected active chain, current tip, and stale-tip states. |
| Service supervision and restart/resume evidence | Use launchd/systemd review flows to prove service-supervised full sync can resume through same-datadir restarts. |
| Redacted support bundles | Add v1.6 full-sync timelines, tip freshness, chainstate/reorg evidence, peer contribution summaries, and final pass/fail fields. |
| Compatibility harness operator wrapper | Reuse peer transcript diagnosis for peers that block full sync, and align daemon peer replacement with compatibility report categories. |
| Deterministic release-boundary checks | Refresh the checker so v1.6 docs make the full-sync claim explicit while preserving all deferred scopes. |
| Repo-local Cargo and Bazel UAT command pattern | Provide copy-pasteable deterministic and opt-in public-mainnet commands for operators and reviewers. |

## Suggested Requirement Categories

These categories are suitable for a future `REQUIREMENTS.md`. The exact IDs can
change during milestone planning, but each category should map to one or more
roadmap phases and UAT evidence paths.

### Full Active-Chain Validation and Persistence

- **SYNC-01:** `open-bitcoind` can sync the active mainnet chain to the current best known tip through validated headers, downloaded blocks, connected block state, best-chain work, and durable height/hash evidence.
- **SYNC-02:** Mainnet-scale connect behavior persists the chainstate or UTXO data needed for truthful restart, reorg handling, and final tip evidence.
- **SYNC-03:** Status and evidence distinguish headers-only, downloaded-only, connected, validated, current, stale, and recovering states.

### Tip Tracking and Stay-Current Operation

- **TIP-01:** After initial catch-up, the daemon stays current by continuing peer/header/block progress without requiring a new interactive command.
- **TIP-02:** Operator-facing output defines and reports tip freshness using stable height/hash/work/time fields and pass/fail interpretation.
- **TIP-03:** The daemon reports typed stale-tip, peer-tip-disagreement, and no-new-block states without over-crediting progress.

### Reorg, Peer Rotation, and No-Progress Recovery

- **REC-01:** Reorg handling preserves durable active-chain correctness and exposes bounded disconnect/reconnect evidence.
- **REC-02:** Peer rotation, retry, timeout, and contribution attribution continue to work across full-chain sync attempts.
- **REC-03:** No-progress diagnosis remains typed and actionable for incompatible peers, public-network unreachability, storage pressure, invalid data, stale tips, and operator cancellation.

### Resource Bounds and Durable Restart/Resume

- **RR-01:** Full-sync operation enforces documented bounds for peers, in-flight requests, caches, storage writes, metrics, logs, and support evidence.
- **RR-02:** Same-datadir restart and service-supervised restart resume full-sync or stay-current work without duplicate block connects, stale in-flight requests, or lost final-tip evidence.
- **RR-03:** Recovery behavior distinguishes clean shutdown, unclean shutdown, schema mismatch, store corruption, lock contention, low disk, resource exhaustion, and chainstate replay needs.

### Operator Observability and Support Evidence

- **OBS-01:** Status, dashboard, RPC sync status, metrics, structured logs, live-smoke snapshots, and support bundles share the same full-sync truth contract.
- **OBS-02:** Operator can generate redacted v1.6 support evidence summarizing the full-sync timeline, final tip, stay-current state, restarts, reorgs, peer outcomes, resources, and latest recovery guidance.
- **OBS-03:** Human and JSON outputs remain compact, stable, and suitable for both manual review and scripted UAT checks.

### Opt-In UAT and Deterministic Verification

- **VER-01:** Default `bash scripts/verify.sh` remains deterministic and does not require public peers, internet access, long-running services, or a full mainnet datadir.
- **VER-02:** Deterministic tests cover simulated full-sync, reorg, restart/resume, resource-bound, and no-progress behavior with hermetic fixtures.
- **VER-03:** Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for opt-in public-mainnet full-sync review, stay-current review, service restart review, compatibility diagnosis, and support bundle collection.

### Release Boundaries, Parity, and Documentation

- **REL-01:** v1.6 parity and release-readiness docs describe the sync-to-tip claim, evidence fields, threat model changes, and known deferred scopes.
- **REL-02:** Deterministic release-boundary checks fail when docs imply inbound serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, packaging polish, hosted dashboard, GUI, or broad production-node readiness.
- **REL-03:** README and operator docs are refreshed where contributor-facing status, UAT workflow, or release boundaries change.

## Suggested MVP Definition

### Launch With v1.6

- Explicit opt-in `open-bitcoind` mainnet full-sync workflow that reaches and reports the active tip through connected, validated, durable state.
- Stay-current loop after initial catch-up with typed stale-tip and no-progress diagnosis.
- Reorg-safe and restart-safe durable recovery for same-datadir daemon and service-supervised runs.
- Shared operator truth across status, dashboard, RPC, metrics, logs, live-smoke reports, and support bundles.
- Deterministic fixture coverage plus opt-in public-mainnet UAT commands and pass/fail report fields.
- Refreshed threat model, parity docs, release-readiness matrix, and deterministic boundary checker.

### Defer Beyond v1.6

- Inbound serving, address advertisement, transaction relay, compact block relay, and production full-node claims.
- Production-funds wallet use.
- Migration apply mode or source datadir mutation.
- Signed packaging, Windows service support, hosted dashboard, Qt or desktop GUI work, and broad distribution polish.
- Default-verification public-network checks.

## Sources

- `.planning/PROJECT.md`
- `.planning/MILESTONES.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/milestones/v1.5-ROADMAP.md`
- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`
- User v1.6 milestone scope, 2026-06-11.

---
*Feature research for: Open Bitcoin v1.6 Mainnet Full-Sync Completion*  
*Researched: 2026-06-11*

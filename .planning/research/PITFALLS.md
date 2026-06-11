# Pitfalls Research

**Domain:** Open Bitcoin v1.6 mainnet full-sync completion
**Researched:** 2026-06-11
**Confidence:** HIGH for risk classes and scope boundaries, MEDIUM for phase numbering until the v1.6 roadmap is created.

## Context

v1.5 shipped a source-built, explicit opt-in unattended mainnet operator-review
loop with bounded progress, durable restart/resume evidence, service lifecycle
evidence, compatibility reports, support bundles, and deterministic release
boundaries. v1.6 raises the claim from bounded review to explicit opt-in sync to
the active mainnet tip and staying current.

The main mistake is treating this as "run the loop longer." Full sync-to-tip is
a different claim: consensus validation, active chainstate, durable UTXO state,
reorg handling, peer scheduling, restart recovery, and operator evidence must
all agree before the release can truthfully say the node reached tip.

## Proposed v1.6 Phase Map

The active roadmap has not yet assigned v1.6 phases. These proposed slots
continue after shipped Phase 67 and should be adjusted when the roadmap is
created.

| Proposed phase | Responsibility |
| --- | --- |
| Phase 68: Full-Sync Requirements and Claim Boundary | Define sync-to-tip acceptance criteria, explicit non-claims, parity roots, and deterministic-versus-opt-in verification split. |
| Phase 69: Consensus and Active Chainstate at Mainnet Scale | Ensure every connected block path is consensus-gated and chainwork-driven before progress can count as connected. |
| Phase 70: Durable UTXO Storage and Corruption Recovery | Persist chainstate and UTXO data with atomicity, schema, crash recovery, size bounds, and repair guidance. |
| Phase 71: Sustained Peer Scheduling and Anti-Stall Sync | Drive long-running headers and block download across peer rotation, in-flight limits, replacement, backoff, and no-progress diagnosis. |
| Phase 72: Reorg and Restart/Resume Correctness | Prove disconnect/reconnect, stale in-flight cleanup, same-datadir resume, and multi-block reorg behavior. |
| Phase 73: Operator Tip Truth and Opt-In Full-Sync UAT | Expose truthful progress, tip, lag, stay-current, support, and UAT evidence without overstating production readiness. |
| Phase 74: Release Boundaries and Deterministic Verification | Lock the v1.6 parity docs, threat model, default verification exclusions, and out-of-scope surfaces. |

## Critical Pitfalls

### 1. Counting Downloaded Blocks As Consensus-Connected Blocks

**What goes wrong:** The node reports full sync because it has headers or block
bodies near tip, but active chainstate has not connected those blocks through
the full consensus path. This can hide invalid block acceptance, missing UTXO
updates, or a chainstate stuck far behind header progress.

**Warning signs:**
- Status language uses "synced" when only `header_height` or
  `downloaded_block_height` advanced.
- Tests assert height counters without checking active tip hash, chainwork,
  UTXO deltas, and undo data.
- Live reports treat support bundle existence or block-body receipt as success.

**Prevention strategy:** Keep `header_height`, `downloaded_block_height`, and
`connected_block_height` separate. Only connected progress should count toward a
sync-to-tip claim. Require deterministic fixtures and parity checks for invalid
headers, invalid blocks, coinbase maturity, subsidy-plus-fees limits, BIP30
overwrite behavior, merkle roots, contextual header checks, and script/spend
validation before live UAT can be accepted.

**Phase to address:** Phase 68 defines the claim; Phase 69 enforces the
consensus and chainstate gate; Phase 73 prevents operator-surface drift.

### 2. Treating Full Chainstate As A Height Counter

**What goes wrong:** The durable store preserves the last connected height but
not a complete, reloadable, internally consistent UTXO view. The node can appear
to resume at tip but fail to validate the next block, survive reorgs, or answer
truthful status about active chainstate.

**Warning signs:**
- Restart tests reopen a tip hash but do not validate a subsequent spend against
  persisted coins.
- UTXO, undo, block index, and active chain pointers are written independently
  without a recovery invariant.
- Reindex or corruption recovery guidance exists only as generic "delete the
  datadir" text.

**Prevention strategy:** Define a durable chainstate contract: block index,
chainwork, active tip, UTXO entries, undo records, schema version, flush
ordering, and crash recovery must move through explicit states. Test reload after
partial writes, incompatible schema, lock contention, corruption markers, and
unclean shutdown. Do not promote a sync-to-tip claim until a fresh process can
connect more blocks from the reopened state.

**Phase to address:** Phase 69 for chainstate invariants; Phase 70 for durable
storage, recovery, and growth controls; Phase 72 for restart/resume proof.

### 3. Linear-Only IBD With No Serious Reorg Path

**What goes wrong:** The IBD loop handles the best chain only as a straight line.
At mainnet scale, competing headers, late-arriving blocks, peer disagreement, and
short reorgs are normal. A linear-only implementation can corrupt active
chainstate, double-connect blocks, or refuse the most-work chain.

**Warning signs:**
- Reorg tests cover one toy disconnect but not multi-block disconnect/reconnect
  with persisted undo.
- Chain selection uses height, latest peer, or arrival order rather than
  cumulative work.
- In-flight block requests survive a reorg without being invalidated or
  reclassified.

**Prevention strategy:** Model chain selection around cumulative work and make
reorg application explicit: disconnect old active blocks with undo, reconnect
the better branch, reconcile in-flight downloads, and persist the transition
atomically enough to recover after a crash. Include deterministic reorg
fixtures, restart-after-reorg tests, and support evidence that distinguishes
headers, downloaded bodies, and connected active tip.

**Phase to address:** Phase 69 establishes chainwork and active-chain rules;
Phase 72 owns reorg and restart/resume correctness.

### 4. Peer Scheduling That Works Only With One Friendly Peer

**What goes wrong:** A bounded review loop can show progress with one useful
peer, but full IBD needs sustained scheduling across churn, idle peers, partial
responses, malformed data, `notfound`, disconnects, and rate differences. Weak
scheduling either stalls forever or overreacts by churning good peers.

**Warning signs:**
- One peer owns all useful progress and replacement peers never catch up.
- In-flight limits are global but not peer-attributed, or peer failures clear
  useful work from other peers.
- `notfound`, duplicate blocks, malformed blocks, and disconnects are logged but
  do not affect scheduling decisions.

**Prevention strategy:** Keep peer contribution accounting typed and durable
enough for operator evidence. Separate header sync, block body scheduling, block
connect, and peer health. Cap in-flight work, rotate stalled peers, preserve
useful peers, retry missing blocks without duplicate connects, and keep failure
credit local to the responsible peer.

**Phase to address:** Phase 71 owns sustained scheduling, peer rotation,
anti-stall behavior, and no-progress taxonomy; Phase 72 verifies stale in-flight
cleanup across reorgs and restarts.

### 5. Misdetecting "At Tip" And "Staying Current"

**What goes wrong:** The node calls itself current based on local headers,
wall-clock freshness, or a single peer's claim. It can be behind the network,
stuck after reaching a historical height, or unable to process new blocks while
still showing reassuring status.

**Warning signs:**
- "Synced" is derived from elapsed time, last peer message time, or local best
  header alone.
- Lag is rendered as a precise ETA without a clear source and confidence.
- No-progress states disappear after restart or are overwritten by generic
  `steady` status.

**Prevention strategy:** Define tip truth as an evidence bundle, not one number:
validated headers, connected active chainstate, peer-observed best height or
headers, last useful progress timestamp, no-progress category, and stay-current
window. Use cautious wording such as "connected to known validated tip" when the
global public tip cannot be proven. Require opt-in UAT to demonstrate reaching a
mainnet tip candidate and then connecting newly announced blocks or diagnosing
why that did not happen.

**Phase to address:** Phase 68 defines the acceptance language; Phase 71
provides no-progress evidence; Phase 73 owns operator truth and UAT.

### 6. Storage Growth, Compaction, And Corruption Becoming An Afterthought

**What goes wrong:** Full mainnet sync changes the storage profile from bounded
evidence to long-running, high-volume writes. Without explicit bounds and
recovery, block bodies, UTXO data, indexes, metrics, logs, and support evidence
can grow without control or leave the store unrecoverable after a crash.

**Warning signs:**
- Tests use tiny fixture stores only and never inspect large-store behavior.
- Metrics/log retention is bounded but chainstate or block-body retention is not.
- Corruption handling exists but cannot identify which store family failed.

**Prevention strategy:** Budget disk usage by store family, document which data
is retained, define pruning as out of scope unless deliberately added, and add
store-health evidence that points at the failing component. Cover schema
versions, atomic write batches, flush ordering, recovery precedence, compaction
expectations, lock contention, and operator-safe repair guidance.

**Phase to address:** Phase 70 owns durable storage growth and corruption
recovery; Phase 73 surfaces store health in status and support evidence.

### 7. Restart/Resume Evidence That Does Not Prove Resume Safety

**What goes wrong:** The node reopens the same datadir and reports previous
progress, but stale in-flight requests, half-applied chainstate, duplicate block
connects, or peer retry state can corrupt the next run. v1.5 proved bounded
restart/resume evidence; v1.6 needs restart safety at full-chain scale.

**Warning signs:**
- Resume tests stop after reading durable status and do not continue syncing.
- In-flight headers or blocks are replayed blindly after restart.
- Clean and unclean shutdown share the same recovery category and next action.

**Prevention strategy:** On startup, reconcile durable active tip, downloaded
block bodies, pending requests, peer retry state, and latest stop reason before
opening sockets. Treat in-flight work as suspect unless it can be revalidated.
Test same-datadir resume after clean shutdown, unclean shutdown, mid-batch block
download, mid-connect crash, reorg, and storage pressure.

**Phase to address:** Phase 72 owns restart/resume correctness; Phase 70 owns
storage recovery categories; Phase 73 owns operator-facing evidence.

### 8. Nondeterministic Verification Sneaking Into Default Checks

**What goes wrong:** Full-sync ambition pressures the project to put public
mainnet, service-manager, timing, or disk-size-sensitive checks into default
verification. That makes `bash scripts/verify.sh` flaky and pushes contributors
toward skipping the very guardrails that protect consensus work.

**Warning signs:**
- Default tests require internet, public peers, `launchctl`, `systemctl`, or a
  long wall-clock sync.
- Test pass/fail depends on current tip height, mempool contents, peer behavior,
  DNS seed availability, or elapsed time thresholds.
- Live reports are checked into git as golden fixtures.

**Prevention strategy:** Keep deterministic checks hermetic: synthetic peers,
fixed fixtures, local stores, fake clocks, and controlled service adapters.
Reserve public-network full-sync attempts for explicit opt-in UAT commands and
local generated reports. Default verification should check the boundaries,
schemas, redaction, docs roots, parity breadcrumbs, and deterministic model
behavior, not public network availability.

**Phase to address:** Every phase should carry deterministic tests for its
domain. Phase 74 owns the final verification contract and release-boundary
guards.

### 9. Operator Claims That Outrun Evidence

**What goes wrong:** The project truthfully ships sync-to-tip evidence but copy,
status, docs, or support artifacts imply production full-node support, network
service readiness, or wallet safety. This is especially risky because v1.6 is
closer to a normal node than prior bounded review milestones.

**Warning signs:**
- Docs say "full node ready" without naming source-built, explicit opt-in, and
  non-claim boundaries.
- `synced=true` lacks fields explaining known tip source, connected height, and
  latest progress.
- Support bundles blur successful full sync, diagnosed blocker, and partial
  progress into one "healthy" label.

**Prevention strategy:** Keep release language evidence-based. Use separate
states for partial progress, reached known tip, stayed current, diagnosed
blocker, storage recovery, and operator cancellation. Require support and
readiness docs to state explicit non-claims: no inbound serving, relay,
production-funds wallet use, migration apply mode, packaging, hosted dashboard,
GUI, Windows service, public-network CI, or broad production-node guarantee.

**Phase to address:** Phase 73 owns operator truth and support evidence; Phase
74 owns final release-readiness and parity roots.

### 10. Scope Creep Into Production Node Surfaces

**What goes wrong:** Full sync-to-tip sounds like a production-node milestone, so
inbound serving, transaction relay, compact blocks, wallet production use,
migration apply, packaging, pruning, and hosted monitoring get pulled in before
the outbound full-sync claim is correct and auditable.

**Warning signs:**
- A full-sync phase starts adding relay, inbound peer serving, address
  advertisement, or wallet-funds language to "make it feel complete."
- Requirements include packaging, signed installers, hosted dashboards, or
  migration apply as acceptance criteria for sync-to-tip.
- Mempool repair during reorg becomes a blocker even though transaction relay is
  still deferred.

**Prevention strategy:** Treat v1.6 as one expansion: explicit opt-in outbound
mainnet sync to active tip and staying current. Document every production-adjacent
surface as deferred unless a separate requirement explicitly scopes it with
parity evidence, threat model, and verification plan.

**Phase to address:** Phase 68 sets the requirement boundary; Phase 74 enforces
it in docs, parity roots, and release checks.

## Risk-To-Phase Matrix

| Risk class | Primary warning sign | Prevention owner |
| --- | --- | --- |
| Consensus safety | Downloaded or header-only progress is reported as connected active-chain progress. | Phase 69, with Phase 73 truth-surface checks. |
| Chainstate correctness | Persisted height/hash cannot validate the next block after restart. | Phase 69 and Phase 70. |
| Reorg behavior | Chain selection follows height or arrival order instead of cumulative work. | Phase 72, with chainwork rules in Phase 69. |
| Peer scheduling | One friendly peer masks stalls, duplicate requests, or bad-peer attribution. | Phase 71. |
| Storage growth/corruption | Store size, schema, compaction, and corruption recovery are undocumented or untested. | Phase 70. |
| Restart/resume | Same-datadir reopen shows previous status but cannot safely continue work. | Phase 72. |
| Operator claims | Docs or status say "synced" without connected-tip evidence and non-claims. | Phase 73 and Phase 74. |
| Nondeterministic tests | Default verification touches public peers, real services, or timing thresholds. | Phase 74, with per-phase deterministic fixtures. |
| Scope creep | Inbound serving, relay, wallet, migration, or packaging become hidden acceptance criteria. | Phase 68 and Phase 74. |

## Explicit Out-Of-Scope Pitfalls

These are real Bitcoin-node pitfalls, but they should not become v1.6 blockers
unless the roadmap deliberately expands scope.

- **Inbound serving and address advertisement:** Important for production node
  usefulness, but v1.6 should not claim inbound reachability or peer-serving
  parity.
- **Transaction relay, mempool propagation, and package relay:** Reorgs must keep
  active chainstate correct, but full relay policy and mempool repair can remain
  deferred while transaction relay is out of scope.
- **Compact block relay and block serving:** Sync-to-tip can use existing block
  download behavior without claiming compact-block serving or relay parity.
- **Production-funds wallet use:** Wallet status may depend on chain progress,
  but spending real funds safely needs a separate threat model and parity gate.
- **Migration apply mode or source datadir mutation:** Full sync must not imply
  automatic Core/Knots cutover, service disable, wallet import, or source-datadir
  writes.
- **Packaging, signed installers, Windows service support, and hosted
  dashboards:** These are operator-distribution or hosted-product milestones, not
  prerequisites for the source-built opt-in sync claim.
- **Public-network CI and checked-in live reports:** Live full-sync evidence
  should remain opt-in and locally generated unless a future release-policy phase
  explicitly changes the verification contract.
- **Timing-threshold release gates:** Benchmarks can provide trend evidence, but
  a release should not pass or fail only because one public-network run met a
  wall-clock threshold.
- **Pruning, assumeutxo, assumevalid, and snapshot bootstrap:** These can be
  valuable later, but adding them while first claiming full validation to tip
  risks weakening the evidence story.

## "Looks Done But Isn't" Checklist

- [ ] Connected active chainstate, not just headers or downloaded bodies, reaches
  the known mainnet tip candidate.
- [ ] A fresh process reopens the same datadir and validates additional blocks
  from persisted UTXO state.
- [ ] Multi-block reorg fixtures cover disconnect, reconnect, undo persistence,
  stale in-flight cleanup, and restart after reorg.
- [ ] Peer scheduling handles idle, slow, malformed, disconnecting, and
  `notfound` peers without losing useful progress from healthy peers.
- [ ] Store-health evidence distinguishes schema mismatch, corruption, lock
  contention, backend failure, resource exhaustion, and operator cancellation.
- [ ] Status, dashboard, RPC, logs, metrics, support bundles, and live UAT reports
  use the same progress vocabulary and do not call partial progress "synced."
- [ ] `bash scripts/verify.sh` remains deterministic and public-network-free.
- [ ] Release-readiness docs explicitly preserve non-claims for inbound serving,
  relay, production wallet use, migration apply, packaging, hosted dashboard,
  GUI, Windows service support, public-network CI, and broad production-node
  guarantees.

## Sources

- `.planning/PROJECT.md`
- `.planning/MILESTONES.md`
- `.planning/STATE.md`
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md`
- `docs/parity/release-readiness.md`
- `docs/parity/catalog/chainstate.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/deviations-and-unknowns.md`
- `docs/operator/runtime-guide.md`
- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`

The checked-in Bright Builds sidecar and repo-local guidance materially informed
the functional-core, deterministic-verification, and opt-in public-network
boundaries. The pinned canonical standards pages referenced by the sidecar were
not available through the fetch tool in this environment, so this research uses
the local sidecar summary plus repo-owned milestone and parity artifacts.

---
*Pitfalls research for: Open Bitcoin v1.6*
*Researched: 2026-06-11*

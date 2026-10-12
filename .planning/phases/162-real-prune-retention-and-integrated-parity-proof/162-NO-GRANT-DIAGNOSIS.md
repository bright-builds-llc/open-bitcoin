---
artifact_role: historical_diagnostic_snapshot
snapshot_of: .planning/debug/phase162-no-grant-filter-disposition.md
snapshot_at: 2026-10-11T20:17:33Z
status: diagnosed
trigger: "Phase162 Plan15 frozen no-grant filter request resets before expected Pong sentinel"
created: 2026-10-11T20:15:27Z
updated: 2026-10-11T20:15:27Z
---

## Current Focus

hypothesis: Confirmed: frozen client incorrectly applies silent-ignore plus Pong to an unadvertised BASIC capability; production disconnect and fixture no-grant configuration match pinned Knots.
test: Completed read-only comparison of exact pinned semantics, functional reference, production ordering, actual parsed fixture and existing integrated failure receipt.
expecting: A checked test-only amendment expects no bytes then bounded disconnect, using a fresh handshake for each denied request. Granted exact frames/sentinels and all-only absent-backend ignore controls remain unchanged.
next_action: Parent amends Plan04/15 ownership and negative expectation, assigns client implementation/focused verification, then requires a fresh successful full584-row lifecycle; no implementation or Cargo/Bazel run by this diagnostician.

## Symptoms

expected: Frozen no-grant session expected no compact-filter replies followed by a Pong sentinel for each of ten exchanges.
actual: Full584codec-row integrated lifecycle passed real reorg, manual/automatic pruning, cold reopen, RPC9hashes×2selectors/4auth, and three granted sessions each10exchanges/20frames/1890bytes with staleh1000 replies; no-grant advertised noCF, then first getcfilters19..21 reset before sentinel.
errors: ConnectionReset os54 at inbound.rs188→427; admitted1/rejected0/resource0/timeouts0/latestgetcfilters. Test0passed/1failed exit101,241.57s.
reproduction: Unchanged phase162_integrated_lifecycle full pressure corpus after Plan14 selected trusted-header checkpoint correction; receipt .local/open-bitcoin-dev/command-timings/phase162-stale-after-normal-header-checkpoint/2026-10-11T20-07-12.418Z-0be048b2-5288-4f79-8a7a-171ce081cbc1.json.
started: First Plan15 complete-corpus rerun after Plan14 passed all granted sessions and reached final no-grant case.

## Eliminated

- hypothesis: Production should silently ignore valid BASIC requests when the session has no grant.
  evidence: Exact pinned net_processing.cpp3162–3169 explicitly sets fDisconnect for BASIC without peer.m_our_services NODE_COMPACT_FILTERS. p2p_blockfilters.py195–216 independently expects disconnect for each family on a fresh peer without capability.
  timestamp: 2026-10-11T20:15:27Z
- hypothesis: The fixture accidentally expresses all-only disabled-backend silent-ignore policy.
  evidence: fixtures.rs186 always enables BASIC; inbound.rs454 selects general=false,permissions=[in]. fixtures.rs195–225 adds no permission class for [in] and asserts the default registry; observed handshake has noCF and all=false. No all permission or absent backend is present in the failed case.
  timestamp: 2026-10-11T20:15:27Z
- hypothesis: Stale proof or retained-record loss remains the cause of this first no-grant failure.
  evidence: Actual log has all3granted original10exchange sessions passing exact20frames/1890bytes, including active/stale positive checkpoints. Failed request is active19..21; capability gate precedes all stop/provenance/backend reads at branch.rs255–263.
  timestamp: 2026-10-11T20:15:27Z
- hypothesis: Resource rejection, failed admission or timeout explains the reset.
  evidence: Actual bounded handshake succeeds with current inbound=1; failure evidence admitted1/rejected0/resource_rejections0/timeout_disconnects0/latestgetcfilters matches synchronous capability rejection.
  timestamp: 2026-10-11T20:15:27Z

## Evidence

- timestamp: 2026-10-11T20:15:27Z
  checked: Startup instructions, active lessons, debug knowledge base and baseline revision.
  found: Both active lesson files total7188bytes/2397estimatedtokens and were fully read; knowledge base has no matching filter/capability pattern. Knots HEAD is a9aee730466ac67d35a3c03ee24676be5e045878. Parent owns shared-state/plan/task/Git changes; source is frozen.
  implication: This diagnosis is limited to one new debug document and read-only evidence; existing client failure is the reproduction, without costly rerun.

- timestamp: 2026-10-11T20:15:27Z
  checked: Pinned Knots net_processing.cpp3156–3206 and family callers3221,3250,3291; p2p_blockfilters.py195–216.
  found: All3families share PrepareBlockFilterRequest. BASIC+own advertised CF is the first gate; its absence disconnects immediately. Functional reference reconnects before each denied request and waits for disconnect.
  implication: A serial Pong fence after an ungranted request contradicts the authoritative baseline, regardless of enabled backend or body pruning.

- timestamp: 2026-10-11T20:15:27Z
  checked: Pinned net_permissions.h48–55; net_permissions.cpp53,59; init.cpp1097–1103,2358–2361; net_processing.cpp1551–1553,3200–3204.
  found: Explicit blockfilters carries a distinct prerequisite bit, while all expands effective BlockFilters without explicit bit. General serving/explicit blockfilters require BASIC enabled; all alone may start without BASIC, overlays peer NODE_COMPACT_FILTERS, then valid semantics reach absent-index return without disconnect.
  implication: No-grant/no-advertised-CF and all-only/advertised-CF/no-backend are different branches. Existing absent-backend Pong controls must remain.

- timestamp: 2026-10-11T20:15:27Z
  checked: Frozen inbound.rs279–442,449–468; fixtures.rs153–245; retention.rs SettledIdentity and paired deletion/reopen helpers.
  found: Same reopened retained fixture always enables BASIC, parses actual flags/default permission registry, and asserts settled identities. physical_session computes zero expected frames for granted=false but still unconditionally sends sentinel at370, expects Pong via exact frame reader188 and reuses one doomed socket for all10requests. Plan04 Task2 explicitly repeats this wrong no-grant ping expectation.
  implication: Correct the negative client disposition and plan wording only; fixture configuration correctly exercises intended no-grant policy. Retention assertions, all granted exchanges and independent expected frames are unrelated and remain frozen.

- timestamp: 2026-10-11T20:15:27Z
  checked: Production capability.rs25–54, branch.rs249–263, serving.rs59–64,174–194; inbound connection_runtime.rs214–231.
  found: Session own services reflect general/effective permission. validate_network returns UnsupportedTypeOrCapability before provenance/backend; prepare maps InvalidDisconnect before reservation, resolver disconnects and returns error, inbound loop terminates on no resolved responses. Valid advertised requests can instead return NoBackend later.
  implication: Production implements pinned ordering and disposition. Relaxing policy or replacing semantic invalid with local unavailable would introduce a parity bug.

- timestamp: 2026-10-11T20:15:27Z
  checked: Existing peer_serving/evidence.rs344–415 and capability/tests.rs12–54; serving/tests/families.rs125–188.
  found: Existing configured all-only/no-backend control proves all3valid families return only Pong, and invalid type disconnects with EOF/ConnectionReset/ConnectionAborted accepted; capability prerequisite matrix distinguishes all versus explicit; aggregate pressure does not mask capability InvalidDisconnect.
  implication: Reuse existing portable terminal-disconnect assertion pattern in test-only client and retain absent-backend sentinel regression; do not merge the two cases.

- timestamp: 2026-10-11T20:15:27Z
  checked: /tmp/phase162-plan15-client.log and exact command timing JSON.
  found: Actual integrated failure is0/1 exit101,241.57s test/254697ms wrapper, runId0be048b2-5288-4f79-8a7a-171ce081cbc1. All granted cases passed before final no-grant exchange0 reset. Parent/executor independently report13frozen hashes unchanged, unique PID78486 datadir absent and Cargo idle.
  implication: Existing failure is sufficient reproduction evidence for diagnosis; no new test or cleanup run claimed here. Full integrated success remains outstanding.

## Resolution

root_cause: High confidence. The frozen test and Plan04 negative-control wording conflate absent serving capability (pinned semantic disconnect) with advertised all-only capability plus absent BASIC backend (valid request silently ignored). physical_session's unconditional sentinel at inbound.rs370 expects Pong on the correctly disconnected no-grant socket.
fix: Not applied (diagnose-only). Smallest parity-preserving correction is test-only no-grant disconnect observation and narrowly checked Plan04/15 wording/ownership amendment; no production or fixture policy correction is needed.
verification: Read-only authoritative-source and actual prior failure-log/receipt analysis. No Cargo/Bazel/focused/full test run by this agent; no corrected run claimed. Parent must grant and run focused behavior, then full original584-row same-datadir corpus.
files_changed: [.planning/debug/phase162-no-grant-filter-disposition.md]

## Suggested Minimal Amendment

1. Preserve all granted original10exchanges, exact20frames/1890bytes, independent active/bodyless/stale predecessor/checkpoint assertions, Pong fences, configured service bits, settled identity and cleanup unchanged.
2. For no-grant, handshake and pre-request Pong drain still prove a live admitted connection and noCF. Send the exact original request, require zero response bytes followed by bounded EOF, ConnectionReset or ConnectionAborted. Any byte, timeout/WouldBlock after the deadline, unrelated I/O error or partial response fails. Do not send a post-request ping or claim sentinel_fenced=true for denial.
3. Because disconnect terminates the session, open and fully handshake a fresh socket before each denied exchange. Retaining all10original denied requests gives strongest minimal coverage; at least all3families require independent sessions, as the pinned functional reference does. Keep server-peer disappearance, zero transport charges, bounded client/listener joins and unchanged same-datadir identity after every session.
4. Do not add grants, disable the fixture backend, alter own service bits, relax InvalidDisconnect, waive stale replies or replace the original pressure lifecycle. Amend only contradictory no-grant Pong/freeze text after explicit parent planning review; preserve all other frozen source/recipe/oracle expectations.

## Focused Behavioral Strategy for Parent Handoff

- A short genuinely accepted/configured loopback fixture can test only disposition cheaply: BASIC enabled, peerblockfilters=false, permissions=[in]; fresh complete handshake and noCF for each getcfilters/getcfheaders/getcfcheckpt, exact bounded no-byte disconnect, zero live charges and joined cleanup. Prefer using the corrected Phase162 transport helper itself so focused green validates its client logic rather than a parallel helper. No storage-pressure, prune threshold or stale-retention completion claim follows from that control.
- Keep or rerun the existing `phase160_peer_evidence_configured_all_only_no_backend_all_families` control separately: all-only plus BASIC disabled advertises CF, valid requests yield Pong with no filters; invalid type still disconnects.
- A failed implementation that ignores ungranted requests must fail the focused disconnect test by deadline. One that serves data or leaves bytes must fail immediately. General/effective permission successful cases must retain exact received frames, and all-only valid absent-backend must retain the connection.
- After focused/source/lint checks, the fresh mandatory `phase162_integrated_lifecycle` remains full584codecrows, actual manual0..20/automatic21..714 paired deletion, genuine reorg/cold reopen,9RPC hashes×2selectors/4auth, all3granted original10exchanges and corrected no-grant controls on the same retained authority. Root owns timing lock, reviews, native/formal verification and D-14 final Git.

## Material Guidance and Scope

Loaded repo AGENTS, Bright Builds sidecar/overrides/index, architecture/code-shape/testing/verification/Rust standards, both bounded active lesson inputs, STATE, Phase162 CONTEXT/04PLAN/15PLAN/14SUMMARY, frozen inbound/fixture/retention interfaces and pinned request/permission/reference tests. GSD debugger hypothesis discipline, root source freeze and no-rerun instruction govern this report. No source/client/plan/shared-state/Git mutation occurred.

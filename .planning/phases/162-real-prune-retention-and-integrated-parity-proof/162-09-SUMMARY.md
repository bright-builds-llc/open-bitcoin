---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "09"
subsystem: testing
tags: [physical-socket-prefix, affine-receipts, fjall, cancellation, normal-maturity]
requires:
  - phase: 162-01
    provides: Shared mature recipe, accepted adapter and independent raw-block oracle
  - phase: 162-06
    provides: Complete long retained-filter lifecycle and successful public outbound client
  - phase: 162-08
    provides: Concrete publication and corruption fault evidence
provides:
  - Actual outbound first/second physical failures and exact private completed-frame receipts
  - Actual inbound CF executor partial, stale-session and successful receipt controls
  - Real inbound socket deadline, queue, shutdown and disconnect cleanup with separate future-drop evidence
affects: [162-10, 162-11, 162-12]
tech-stack:
  added: []
  patterns: [Guard-before-owner cleanup, independent exact wire prefixes, returned versus dropped cancellation]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs
key-decisions:
  - The actual inbound CF seam is execute_inbound_filter with InboundFilterWriter and SocketFilterWriter; announcement execution is separate.
  - Explicit blockfilters permission survives general-serving toggles; disconnect and genuine re-admission invalidate the handed session generation.
  - Socket priming bytes and completed old physical frames never become valid current-range completion.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T22:50:31Z
duration: 20 min recorded receipt window
completed: 2026-10-11
---

# Phase 162 Plan 09: Physical Partial Writes and Private Receipts Summary

**Real TCP prefixes on genuine mature Fjall histories earn only complete-frame receipts; stale sessions, cancellation, deadlines and queue failures cannot complete the current range or retain settled resources.**

## Performance and Ownership

- Tasks: 2/2. Exactly four assigned Rust paths changed, including two new test children. No production source, public API, visibility, dependency, shared fixture, script or output schema changed.
- First recorded Cargo receipt starts 2026-10-11T22:28:30.451Z; initial mandatory context/source loading preceded it. Final strict lint ended 22:47:40.078Z. Final summary clock is 22:50:31Z.
- Material guidance: AGENTS, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, active lessons, GSD executor/execute-plan reasoning/template, STATE, checked Plan09 and its independently checked interface amendment, phase CONTEXT/RESEARCH/UI contract and preceding 04/06/07/08/14/15/16 handoffs.
- Both active lesson inputs were read completely: 7,188 bytes, 2,397 conservative estimated tokens. No archives or new audit trigger; no project skill directories exist. Parent supplied synchronized main baseline a77edab276a9daa086055aa61e6143ce86bc1218 and materialized pinned Knots.
- All Cargo jobs used Bun1.3.9 on PATH, Rust1.94.1, the cooperative timing wrapper and exclusive serial ownership. Every session exited; Cargo was reported idle to root before summary creation.
- Root retains STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, independent source/security reviews, full native/formal checks, requirement activation and all Git. No staging, commit, branch, push or hook bypass occurred. Requirements remain pending.

## Genuine Authority and Independent Expected Bytes

Every focused branch accepts exactly102 shared recipe blocks, h0..101, with runtime Regtest parameters and normal coinbase maturity100. Bodies are ordinarily saved; connect_local_block supplies real ordered accepted receipts. Fresh PreserveSaved accepts genuine genesis and an ordinary Always fence before complete owner closure and configured Enabled reopen. Bounded ordinary index turns, normal coins/undo/metadata settlement and a real fence promotion earn safe h101 before response preparation.

The h101 block contains its actual mature historical spend and same-block successor. Inbound setup reconstructs spent scripts with BasicFilterInputs from the actual accepted position and persisted undo, comparing them to raw recipe outpoints. It also checks unchanged shared wallet/fork recipe facts without claiming additional accepted fork history. No MemoryOnly backend, seeded UTXO, injected snapshot authority, maturity override or manufactured readiness is used.

The outbound test composes the existing AcceptedRecipe adapter methods with a supplied guarded path, preserving the same normal constructor sequence. The inbound library includes packages/testdata/phase162/recipe.rs with the required core_types alias and uses public runtime/storage methods; it imports no daemon-binary helper or node-private progress method. The executed public catch-up turn supplies its actual maybe_progress.

Both directions request h99..101. Independent expectations receive only encoded raw blocks with predecessor names and network magic. The existing bounded process supervisor runs phase162-filter-oracle.ts and the pinned Knots Python oracle/serializer. No Rust filter query, response or computed filter commitment enters expected data. Exact frame lengths are62,62,65bytes, totaling189. Comparison covers every physical byte, including magic, header/checksum, complete payloads and partial next-frame bytes.

These pressure-free branches use the same recipe/version as the long lifecycle, with their own102-block prefix. They are not the full584-row store or its complete ordered input digest. Plan06 retains the separate successful public-client20frames/1,890bytes observation, and Plan04 retains60frames/5,670bytes. This plan supplies direct private receipt evidence rather than reinterpreting those public totals.

## Task 1: Actual Outbound TcpPeerSession Writer

The existing control_writes hook surrounds the actual TcpPeerSession socket writer. The actual receive_sync_message caller prepares the accepted range after an explicit serving grant, own capability offer and real handshake state; production send_response alone acknowledges writer outcomes. No test calls acknowledge_write or complete_basic_filter_frame manually.

| Final outbound case                              | Physical bytes | Private completed frames / bytes | Current-range outcome                                                       |
| ------------------------------------------------ | -------------: | -------------------------------: | --------------------------------------------------------------------------- |
| Partial first frame                              |             31 |                            0 / 0 | Explicit Io error; Aborted                                                  |
| Partial second frame                             |             93 |                           1 / 62 | Explicit Io error; incomplete second and suffix excluded                    |
| Controlled failed write deadline                 |             31 |                            0 / 0 | Actual31-byte prefix, then injected TimedOut at real writer                 |
| Cancellation after handed full write             |             62 |                           1 / 62 | Callback cancellation returns Ok, but disposition PhysicalPrefixInvalidated |
| Serving epoch revoke/restore during handed write |             62 |                           1 / 62 | Explicit invalidation error, PhysicalPrefixInvalidated                      |
| Actual local write-side disconnect               |              0 |                            0 / 0 | Real socket shutdown/write failure                                          |
| Complete control                                 |            189 |                          3 / 189 | CurrentRangeComplete, success                                               |

The controlled timeout is an injected failed writer outcome after actual bytes, not a claim of an OS timer expiring. Cancellation's successful executor return is explicitly distinct from successful current-range authority. Complete old physical frames remain factual; no unwritten frame or range is credited.

Every write observes the live active-frame charge, retained suffix, free authority access and one real independent generic32-byte owner. Socket write timeout is finite and refreshed before effects; the remote reaches finite EOF after session closure. Filter charges settle to zero while that generic owner remains exactly1descriptor/32bytes; its actual drop and peer disconnect then produce the full default snapshot. The complete control also includes the genuine h101 spend-derived frame.

## Task 2: Actual Inbound CF Executor and Real Socket Policies

The original plan named a nonexistent InboundWireTransport and the separate transaction-announcement executor. Before inbound edits, root amended the plan against actual execute_inbound_filter, InboundFilterWriter, SocketFilterWriter and InboundFilterFailure; an independent checker passed with zero blockers/warnings. No production seam or visibility expansion was needed.

Inbound setup uses general serving off and actual parsed loopback in/blockfilters permission. Admission asserts requested/effective capability; own-Version/Verack updates actual peer state. The writer receives the actual post-handshake peer-record active/inactive effects. Re-admission in the epoch control uses the actual socket remote address and real handshake preparation, establishing a new session generation while the old frame is handed.

| Final inbound case                           |        Physical CF bytes |       Private completed frames / bytes | Actual terminal observation                                      |
| -------------------------------------------- | -----------------------: | -------------------------------------: | ---------------------------------------------------------------- |
| First13-byte header prefix                   |                       13 |                                  0 / 0 | ConnectionReset; no current-range Ok                             |
| Second13-byte header prefix                  |                       75 |                                 1 / 62 | Same; current second frame excluded                              |
| First header plus partial payload            |                       31 |                                  0 / 0 | ConnectionReset; exact independent31-byte prefix                 |
| Second header plus partial payload           |                       93 |                                 1 / 62 | Same; exact complete first plus31-byte second prefix             |
| Disconnect/re-admit during full handed write |                       62 |                                 1 / 62 | Transport Invalidated; new valid session cannot revive old range |
| Expired real socket deadline                 |                        0 |                                  0 / 0 | Real SocketFilterWriter Rejected(idle_peer), timeout0            |
| Real aggregate queue refusal                 |                        0 |                                  0 / 0 | Real SocketFilterWriter Rejected; external baseline preserved    |
| Returned real Notify cancellation            |                        0 |                                  0 / 0 | Real saturated SocketFilterWriter Write(Interrupted)             |
| Actual remote disconnect                     | No successful CF receipt |                                  0 / 0 | Real saturated socket close produces Write error                 |
| Real complete control                        |                      189 |                                3 / 189 | Actual SocketFilterWriter success                                |
| Separate pending executor future drop        |                       31 | No Written outcome or receipt returned | Handed charge survives invalidation until actual future drop     |

Custom partial writers physically write their prefixes before returning errors. Other writes return Written only after complete physical write. They prove context and authority remain independently available across the production writer await. Private failure.progress contains exact0/1frame and0/62byte progress; both cleanup-error fields remain absent. Epoch invalidation uses actual disconnect and re-admission: general-serving toggles alone would preserve an explicit grant and cannot substitute for session invalidation.

Every returned SocketFilterWriter control uses the actual ResourceGovernancePolicy, Established handshake, real permission effects, RuntimeQueuePressureState and shutdown Notify. Writer-owned pending-write byte and queued-message increments return exactly to captured queue_pressure_input; all captured queue fields match. In the queue case, externally supplied max_aggregate_queued_messages remains present exactly as before the attempted write; no test clears it to manufacture zero pressure. CF and generic transport charges separately settle to zero.

The real production InboundResponseBatch owns two normally encoded generic suffix frames,64bytes total. Partial writers observe both descriptors during their physical await; actual batch drop retires them after the executor settles. Final FilterTransportSnapshot is default for every case. No test substitutes transaction-announcement completion for CF evidence.

Shutdown and disconnect first saturate the real socket with bounded zero padding. Final frozen observations are **5,380,220 shutdown priming bytes** and **3,037,152 disconnect priming bytes**. The shutdown receiver reads exactly that padding, all zero, with no CF bytes; padding never enters the independent-response comparison or completed receipt. Disconnect deliberately closes the receiver, so primed bytes are a send observation only. Earlier successful samples2,971,096/3,085,888 and1,816,700/1,661,052 were intermediate OS-buffer observations, not final values or fixed guarantees.

The separate future-drop test physically writes31bytes, then remains pending without returning Written. Actual disconnect/re-admission retires queued suffix but keeps exactly the handed active-frame charge. Dropping the actual executor releases that charge; physical EOF then proves the31-byte prefix. This is RAII lifetime evidence, not a returned SocketFilterWriter cancellation or its queue-cleanup proof.

## Verification, Initial Failures and Timing Receipts

All Cargo commands use pinned PATH and `bun run scripts/command-timings.ts run --key KEY -- cargo ...`, with --manifest-path packages/Cargo.toml and --all-features. Tests use --lib, serial libtest threads and --nocapture; every new selector has nonzero discovery.

| Timing key                                | Actual result                                              | Wrapper ms |
| ----------------------------------------- | ---------------------------------------------------------- | ---------: |
| phase162-outbound-write-compile           | Owned private path-helper call refused; compile failure    |      6,918 |
| phase162-outbound-write-compile-corrected | No-run compile passed                                      |     11,933 |
| phase162-outbound-write-faults            | 0/6; fixture omitted explicit serving grant before request |     13,782 |
| phase162-outbound-write-faults-granted    | 6/6;13.41s                                                 |     19,822 |
| phase162-outbound-write-faults-final      | Added complete/generic control;7/7;14.71s                  |     22,909 |
| phase162-inbound-write-compile            | Owned guessed private-progress/undo-field APIs refused     |      3,795 |
| phase162-inbound-write-compile-corrected  | Correct public turn/real undo no-run compile passed        |      6,470 |
| phase162-inbound-write-compile-sized      | Same-file helper placement no-run compile passed           |      6,517 |
| phase162-inbound-write-faults             | 0/11; overstrong initial-admission active-effect assertion |     28,440 |
| phase162-inbound-write-faults-permissions | Actual post-handshake effects/session epoch;11/11;24.11s   |     28,658 |
| phase162-inbound-write-faults-final       | Sized source11/11;23.92s                                   |     29,833 |
| phase162-outbound-write-faults-frozen     | Final cleanup guard ordering;7/7;15.45s                    |     22,650 |
| phase162-inbound-write-faults-frozen      | Final whole-priming deadline;11/11;24.60s                  |     29,191 |
| phase162-inbound-cf-executor-regression   | Required existing selector10/10;8.15s                      |      8,378 |
| phase162-inbound-cf-socket-regression     | Required existing selector6/6;8.65s                        |      8,820 |
| phase162-write-faults-clippy              | Node+RPC all-targets/all-features -D warnings;7.39s        |      7,522 |

Initial failures were harness/API/setup mistakes, not production defects or invented semantic RED for missing functionality. The first outbound permission failure and first inbound admission-effect failure both occurred before writer effects; their6/11datadirs were actually removed by guards. The corrected tests exercise already-shipped behavior. Two transient extraction-helper Python quoting attempts also failed before any file mutation; correcting the inspected unmatched delimiter allowed the narrow helper move. No blind production retry, ownership widening or guard waiver occurred.

Exact final receipt paths under .local/open-bitcoin-dev/command-timings/:

- phase162-outbound-write-faults-frozen/2026-10-11T22-46-03.092Z-0c78dcf6-be05-42c8-a2f2-f30062b974da.json
- phase162-inbound-write-faults-frozen/2026-10-11T22-46-46.106Z-1eda3bf7-24d9-400d-a121-497d54b3cbbe.json
- phase162-inbound-cf-executor-regression/2026-10-11T22-47-15.316Z-75d2ba72-19ac-4856-a128-b7b574419232.json
- phase162-inbound-cf-socket-regression/2026-10-11T22-47-23.717Z-5738fc1a-cd30-4184-b50f-4b3caf147c88.json
- phase162-write-faults-clippy/2026-10-11T22-47-32.556Z-ecc097f0-57e3-4459-922b-570dfaad56dd.json

The required executor regression selector incidentally discovers two existing announcement tests alongside eight CF tests; those announcement passes are not CF proof. No unaffected long Phase158 or584-row pressure corpus was rerun. Scoped rustfmt, owned diff whitespace, stub/manual-completion scans, Phase160/161 source/claim guards, registered breadcrumbs1141Rust files and managed all checks with zero findings pass. Strict lint supplies compiled diagnostics. Quiet yields were polled normally and never interpreted as command timeouts.

## Simplification, Cleanup and Source Registration

The explicit simplification pass keeps small named socket/oracle/acceptance helpers and separate returned-versus-dropped cancellation cases. Generic cleanup, prefix assertions and bounded socket helpers sit in the already-owned inbound parent; no additional path, suppression, generic framework or dependency was introduced. Existing unrelated parent test bodies remain unchanged. Final file lengths are node parent595, node child354, inbound parent626 and inbound child627. The longest new helper is the164-line outbound run, close to the approximate161-line refactor trigger; the pass keeps its single linear Arrange/Act/Assert case lifecycle together, with acceptance, request preparation and independent oracle already extracted. Inbound fixture and socket helpers are below161lines.

Cleanup guards precede database, response, socket and context owners. Oracle processes use the shared bounded supervisor, which stops/reaps children and joins pipe workers before outcomes propagate. All local test execution stays on the task thread; there are no detached socket tasks. Each successful or unwound fixture closes every owner, proves a raw Fjall reopen, removes only its fresh owned path and asserts absence. Final post-exit inventory under actual TMPDIR and /tmp finds **zero** focused write-fault directories, including initial failed fixtures.

| Final owned source                                                            | SHA256                                                           |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs            | c1b2a60097ef7af50290f4511073d6bed2182e4ab61e1f13c11fe8532fffbd7a |
| packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs   | 3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020 |
| packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs          | 5f173ac212b0b6845ffe64c919d8c8f68c735e283e4c23d64eacd9b96aa6b667 |
| packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs | 6134ec09df47b94a485915294cf46599b762ac5b99c16cd67cc079b822ae47a7 |

Both actual new paths were reported immediately and root registered them. Their exact ordered breadcrumb anchors are packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/test/functional/p2p_blockfilters.py.

Shared recipe SHA256 remains82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497; accepted adapter remains623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580. Final reads also match the published Plan07 missing_history/recovery, Plan08 publication/corruption, Plan06 outbound and Plan15 inbound plus shared daemon fixture/restrictions hashes. No prior source was rewritten.

Parent parity proposal: add this summary, the two new test roots, phase162_outbound_write and phase162_inbound_write selectors and frozen receipts alongside existing retained-filter transport evidence. Preserve current-range success versus factual old physical prefix, exact failure-prefix credit, external-pressure baseline and padding exclusions. No production behavior difference or requirement-owner change is created.

## Declared Mitigations, Deviations and Remaining Gates

| Threat     | Concrete scoped executed evidence                                                                                                                                                    |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T-162-09-1 | Both actual writer adapters; exact independent first/second header and payload prefixes; zero or earlier full-frame-only receipts; positive3/189controls.                            |
| T-162-09-2 | Actual outbound cancellation/epoch and inbound disconnect/re-admission; old physical full frame cannot complete current range; separate pending-future lifetime.                     |
| T-162-09-3 | Real SocketFilterWriter deadline/queue/Notify/socket-close results; exact queue baseline, generic batch suffix drop and default settled transport snapshot; bounded padding and EOF. |

The independently checked interface amendment corrects stale plan names without changing architecture or ownership. Owned API/permission harness corrections and same-file size simplification are documented above. No outside-owned production bug, authentication gate or unresolved implementation blocker was found. Task1, Task2 and summary commits are explicitly deferred by D-14; no shared state or Git was mutated.

No blocking stub, unwired source, new production endpoint/auth/schema/dependency or unmodeled production trust surface was introduced. Existing temporary-datadir and loopback boundaries are covered by the declared threats. Independent source/security review and unresolved-High disposition remain parent gates; no ASVS certification is claimed.

These focused branches do not replace the successful long retention lifecycle. The synthetic root, caller-supplied work, Open Bitcoin210000 versus pinned Knots Regtest150 halving, codec-only pressure and logical-soft-target versus physical-cap distinctions remain explicit. No whole-chain Knots consensus differential, most-work selection, archive scale, BIP37/V0, assumeutxo/dual-chainstate, public defaults/network CI, implicit repair/import/download, wallet acceleration, production services/readiness/funds or milestone audit/archive/shipment is claimed. Phase161 presentation/schema/redaction and v2.4 advisories remain unchanged.

**Next10:** Consume these direct private physical-receipt roots alongside the long actual client lifecycle when transitioning only current scoped claims and parity documentation. Preserve Plan15 CHECKPOINT_CONTRACTS, delegated lineage/IfNeeded/no-snapshot/order/error guards and positive/negative mutations. Plans11/12, independent reviews, full default bash scripts/verify.sh, formal lifecycle checks, evidence-gated requirement activation, generated LOC and strict Git finalization remain root responsibilities.

## Self-Check: PASSED

All four owned Rust paths and this summary exist. Final source hashes match the frozen matrices and strict lint. Frozen receipts and both affected regression receipts exist with exit0; initial setup failures retain exit101. Baseline is unchanged. Actual new paths are registered; scoped formatting, source/claim guards, managed/parity and whitespace/stub scans pass. Both focused-directory inventories are empty and all owned Cargo sessions exited. Frontmatter has the matching lifecycle identity, generated_by, empty requirements-completed and exactly two top delimiters. No nonexistent task commits are claimed under D-14; state/requirements/parity/LOC/Git remain parent-owned.

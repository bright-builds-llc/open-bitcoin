---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-11T02:29:06Z"
---

# Phase160 Decision and Requirement Coverage

Every locked decision is delivered in full. No deferred Phase161/162 work, frontend contract, V0/BIP37 product, public/default activation, new dependency/crate or independent durable owner is included.

| Decision | Plan/task owners | Coverage | Evidence boundary |
| -- | -- | -- | -- |
| D-01 | 02/T1,03/T1–2,11/T2 | Full | Explicit general/per-peer prerequisites before effects; index/listeners remain independent. |
| D-02 | 02/T2,03/T2,09/T2,10/T2,11/T2 | Full | Actual own versions before catch-up; prune classification retained. |
| D-03 | 02/T1,03/T1,08/T1–3,11/T2,12/T1 | Full | requested/all/implicit matrix and all-only no-backend corner. |
| D-04 | 02/T1–2,03/T2,15/T2,08/T2,09/T1,10/T1–2 | Full | One typed capability; existing outbound grammar stays explicitly scoped; general serving works both directions. |
| D-05 | 01/T2,05/T2,15/T1–2,08/T1–3 | Full | Pinned gate order,including saturated reservation invalid-request controls. |
| D-06 | 01/T1–2,05/T2,16/T2,11/T1 | Full | Exact limits,height/genesis/combined-invalid and pinned short/trailing handling. |
| D-07 | 04/T1–2,05/T1–3,16/T2,11/T1–2 | Full | Exact fixed256 proof/header facts,genuine ScriptsValid and stop ancestry. |
| D-08 | 01/T1,08/T1,16/T1–2,11/T1–2 | Full | Ordered persisted bytes/hash/predecessor/raw wire digest order. |
| D-09 | 01/T1,05/T2,06/T1–3,08/T1,16/T2,11/T1 | Full | Positive1000 checkpoint intervals; work refusal is not a stop-height protocol cap. |
| D-10 | 05/T3,06/T2,08/T1–3 | Full | Shared guarded persisted authority; no body regeneration,repair or RPC readiness wait. |
| D-11 | 06/T2,08/T1–3,11/T1–2 | Full | Whole response prevalidated; no shortened or fabricated success. |
| D-12 | 06/T1–3,07/T1,14/T1–3,09/T1,10/T1–3,11/T1–2,12/T1 | Full | Explicit numeric work/read/copy/byte/request/peer/aggregate/queue budgets with actual measurements. |
| D-13 | 05/T3,13/T1–2,07/T1–2,08/T1,14/T1–3,09/T1–2,10/T1–3,17/T1–2 | Full | Captured branch/lifecycle/session identity and Plan13 monotonic shared read-integrity epoch; transport outside shared locks; historical guards follow actual governed owners. |
| D-14 | 15/T1–2,08/T2,09/T1–2,10/T1–3,11/T1–2 | Full | All three families on actual inbound and outbound callers. |
| D-15 | 07/T1–2,09/T1–2,10/T1–3,11/T1–2,17/T1–2 | Full | Full framed write only; exact physical prefix and one-use sequence receipts; late cancelled writes are physical-only; moved write/ack edges retain mutation-sensitive historical guards. |
| D-16 | 13/T1–2,07/T1–2,14/T1–3,09/T1–2,10/T1–3,11/T1–2,17/T1–2 | Full | Cancel drops actual queued Vec suffix; handed frame bytes and peer/request slot remain charged until completion/drop; shared epoch ABA/overflow fail closed; actual delegated suffix cleanup remains guarded. |
| D-17 | 01/T1–2,04/T1–2,05/T1–3,06/T3,08/T3,16/T1–2,11/T1–2 | Full | Independent pin plus genuine concrete accepted histories; no helper-only parity claims. |
| D-18 | 09/T1–2,10/T1–3,11/T1–2 | Full | Both production transports have loopback success and forced partial/failure evidence. |
| D-19 | 16/T1–2,17/T1–2,12/T1–3 | Full | Breadcrumbs,docs/native guard,UAT,simplification,preserved historical guard compatibility,full native and independent source/security review. |
| D-20 | All context sections;17/T2;12/T1–3 | Full | Headless only; no browser/UI-SPEC or broad Phase161/162 product projection; explicit README production non-claim retained. |

| Requirement | Implementing plans | Closing proof |
| -- | -- | -- |
| CFNET-01 | 02,03,08,09,10 | 11/T2;12/T2–3 |
| CFNET-02 | 01,05,06,15,08,09,10 | 16/T1–2;11/T1–2;12/T2–3 |
| CFNET-03 | 01,05,06,15,08,09,10 | 16/T1–2;11/T1–2;12/T2–3 |
| CFNET-04 | 01,05,06,15,08,09,10 | 16/T1–2;11/T1–2;12/T2–3 |
| CFNET-05 | 01,02,04,05,06,15,08,09,10 | 16/T1–2;11/T1–2;12/T2–3 |
| CFNET-06 | 06,13,07,15,08,14,09,10 | 16/T1–2;11/T1–2;17/T1–2;12/T2–3 |

## Dependency and Ownership Graph

Each task needs canonical/context/interface reads plus its plan dependencies. Creates are the exact files and contracts in its files/action/done fields; every task is autonomous without checkpoint. Shared module/registry changes are sequential. Review/breadcrumb root edits are consolidated in12.

| Wave | Plans | Creates | Needs |
| -- | -- | -- | -- |
| 1 | 160-01,160-04 | Six wire families;exact pure proof API | Existing codec/target/source pins |
| 2 | 160-02 | Typed requested/effective capability;own services composition | 01 |
| 3 | 160-03,160-05 | Startup configuration;exact header facts/branch policy | 02;04 for05 |
| 4 | 160-06 | Finite measured resource and guarded read contracts | 05 |
| 5 | 160-13 | Monotonic shared publication epoch and raw/recovery/disable invalidation | 06 |
| 6 | 160-07 | Request lease/affine receipt/cleanup | 13 |
| 7 | 160-15 | Pure request actions,intent/outcome contracts and inert relay bridge | 07 |
| 8 | 160-08 | Complete persisted plans and durable handle intent resolution | 03,05,06,07,15 |
| 9 | 160-14 | Same-registry generic/scratch+CFactive pending governance | 08 |
| 10 | 160-09,160-10,160-16 | Actual inbound/outbound adapters plus disjoint independent fixture generation | 09/10need08,14;16needs14 |
| 11 | 160-11 | Concrete Rust/cross-transport and mixed-race proof using16goldens | 09,10,16 |
| 12 | 160-17 | Eight-path historical native guard compatibility and explicit README non-claim repair | 11 |
| 13 | 160-12 | Source registration,native guard,docs and parent-owned final review/native/git gate | 17 |

No same-wave file ownership overlaps. Plan15owns seven pure dispatch/outcome/relay/type-migration files,Plan08nine durable preparation/reader/wrapper/test files,Plan14ten shared-governance files. Plans09and10own disjoint RPC and node files in wave10. Plan16owns exactly the original five oracle/generator/golden paths moved from11and runs disjoint from09/10in wave10. Plan11retains its original nine Rust proof/test paths plus five narrow fixture/test children (fourteen total), including independent static wire comparisons and cross-direction race extraction; its full concrete mixed-traffic assertions remain;12now follows17,which follows11. T-160-31moves uniquely to16without adding a capability or changing source-bounded size facts. Plan15serving.rs overlap with08is sequential and limited to canonical-type removal/import plus witnessed pattern compilation fix;08owns all preparation behavior. Plan14new helpers use existing pinned net.cpp/net.h/net_processing.cpp/protocol.cpp breadcrumb anchors and are registered serially before adapter integration. Plan13owns seven epoch/store paths after extracting publication/epoch.rs;Plan07retains eleven receipt/lease/lifecycle paths with two tasks,and additional readiness/barrel files are thin ownership/visibility wiring. Plan13's two tasks touch at most four files each. The one pure arithmetic feature04 is dedicated TDD; other code tasks declare task-level TDD and concrete behavior. All staging/commits/pushes stay deferred to the strict outer wrapper after clean final verification.

## Evidence-Triggered Native Compatibility Replan

The plan set now contains17plans. The first full default native receipt,packages/target/phase160-native-final-1.log,exited1 after7m55.375s at Phase123 (39pass/16fail),before Rust verification. Historical source contracts expected Vec<u8>,session.send and inline acknowledgement,while completed Plans09/10/14 use private governed buffers,exact encoded physical writes and delegated Written-only helpers. Parent's later read-only sweep isolated two moved Phase128 write/ack edges,Phase129's inherited Phase128 errors,Phase134's moved per-prefix completion and Phase132's unsupported 'remain excluded' non-claim predicate. No production defect follows from these literal mismatches; existing production ownership and ordering stay intact.

Plan17owns exactly eight paths: Phase123 checks.ts/evidence.ts/test.ts,Phase128 checker/test,Phase134 checker/test/mutations.ts,and README.md. Its two disjoint tasks each have at most five paths and together target45% context. Existing exported target inventories automatically update Phase129/Phase134 fixture copies; no constants or fixture-helper edit is required. The only README change replaces the new production non-claim's 'remain excluded' with 'remain deferred',leaving Phase132 guard/tests intact. Plan12 keeps twelve paths; its README overlap is sequential in wave13. No production Rust,new dependency,capability,threat ID,locked decision or deferred feature is added. T-160-35 moves uniquely from12to17; all39threat IDs remain registered exactly once.

Plan17 executors run only focused Bun/source/docs checks and preserve every historical behavioral protection with actual-corpus and effective mutation evidence. Parent owns the Cargo target/ongoing Rust preflight,unchanged full default native rerun,independent source/security reviews,lifecycle verification,shared state/requirements/todo and all git finalization. Passing preflight/focused suites cannot substitute for the full native receipt. Requirements remain pending until that gate.

## Serial Per-Wave Breadcrumb Registration

Executors create comments and report exact new paths. After all same-wave executors finish,the parent or designated last executor adds exact files arrays to docs/parity/source-breadcrumbs.json using the existing version1 groups format. Keep upstream anchor paths in the same order as source comments. This manifest is parent-serialized coordination metadata,not parallel plan ownership; Plan12 performs a complete final audit.

| New path responsibility | Required defensible pinned anchors |
| -- | -- |
| network/message/filter.rs and its tests | packages/bitcoin-knots/src/protocol.h;packages/bitcoin-knots/src/protocol.cpp;packages/bitcoin-knots/test/functional/test_framework/messages.py |
| network/filter_serving capability policy and tests | packages/bitcoin-knots/src/net_permissions.h;packages/bitcoin-knots/src/net_permissions.cpp;packages/bitcoin-knots/src/init.cpp |
| consensus/chain_proof.rs,header_store/proof.rs and tests | packages/bitcoin-knots/src/chain.cpp;packages/bitcoin-knots/src/arith_uint256.h;packages/bitcoin-knots/src/net_processing.cpp |
| network/filter_serving policy/resources and tests | packages/bitcoin-knots/src/net_processing.cpp;packages/bitcoin-knots/src/net.h;packages/bitcoin-knots/test/functional/p2p_blockfilters.py |
| storage/fjall_store/filters/query/peer.rs and tests | packages/bitcoin-knots/src/index/blockfilterindex.cpp;packages/bitcoin-knots/src/blockfilter.cpp |
| node/network/filter_transport and runtime_authority/filter_index branch/transport/serving modules/tests | packages/bitcoin-knots/src/net_processing.cpp;packages/bitcoin-knots/src/index/blockfilterindex.cpp |
| node/sync/session/filter_serving and peer fixture/measurement/evidence/transport tests | packages/bitcoin-knots/src/net.cpp;packages/bitcoin-knots/src/net_processing.cpp;packages/bitcoin-knots/test/functional/p2p_blockfilters.py |
| RPC listener filter_serving and peer/daemon startup evidence tests | packages/bitcoin-knots/src/init.cpp;packages/bitcoin-knots/src/net.cpp;packages/bitcoin-knots/src/net_processing.cpp;packages/bitcoin-knots/test/functional/p2p_blockfilters.py |

Choose the narrow applicable subset for each exact path and document none only when no source anchor is defensible. Existing modified files retain their existing defensible groups. Check each serialized wave registration with bun scripts/check-parity-breadcrumbs.ts --check; fix only targeted new-file comments/mapping when it reports stale coverage. Never edit managed standards or suppress the checker. Native final verification repeats this gate.

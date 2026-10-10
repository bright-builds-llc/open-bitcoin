# Phase 159: Authenticated BASIC Filter and Index RPCs - Discussion Log

Audit trail only; downstream consumers use CONTEXT.md.

Date: 2026-10-09 CDT. Mode: Yolo, one recommendation pass. User authorized autonomous recommended choices through the strict-push wrapper. No manual approval is recorded.

| Area | Recommended choice selected | Alternative considered | Rationale |
| --- | --- | --- | --- |
| RPC contract | Exact pinned parsing, results and ordered errors | Generic serde validation | Generic -32602 loses pinned -3/-5/-8 and precedence. |
| Read authority | Narrow typed queries on current managed owner | Separate index/read owner | Shared authority preserves activation/progress/branch coherence. |
| Read cost | Bounded measured point reads with integrity checks | Full ancestry scan per request; prepared outside-lock reads | Recovery scans are unsuitable request cost; outside-lock protocol only if measured need. |
| Connection history | Genuine accepted provenance surviving stale/prune/reopen | Current membership or payload availability | Those facts do not establish historical script validity. |
| Summary | Initial-sync latch and processed height | Tip equality or safe checkpoint | Pinned BaseIndex summary separates these facts. |
| Readiness | Preserve available records and bounded notification ordering | Wait for initial completion under context lock | Initial sync must return promptly and later lag needs accurate absence classification. |
| Authentication | Existing auth-before-JSON transport | New route or separate auth | Reuse preserves node scope and avoids disclosure. |
| Evidence | Actual configured authority, validated chain, paired prune/reopen and HTTP | Synthetic dispatch-only fixture | Product connection and accepted history must be proven. |
| UI gate | No frontend; JSON RPC contract only | New dashboard/design scope | Broad operator projections are Phase 161. |

Two advisor agents examined pinned RPC/framework sources and shared durable authority; recommendations were synthesized into D-01–D-13. Exact implementation and measured limits remain research/planner discretion. Deferred scope is recorded in context. Workflow metadata commits are consolidated behind clean final verification, as in prior phase strict runs.

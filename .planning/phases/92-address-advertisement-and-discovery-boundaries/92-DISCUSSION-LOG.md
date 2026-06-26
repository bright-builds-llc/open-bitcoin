# Phase 92: Address Advertisement and Discovery Boundaries - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-26T03:55:28.679Z
**Phase:** 92-address-advertisement-and-discovery-boundaries
**Mode:** Yolo
**Areas discussed:** Local listener advertisement, Bounded getaddr response policy, Learned address management, Operator evidence and release boundaries, Verification and UAT

---

## Local Listener Advertisement

| Option | Description | Selected |
|--------|-------------|----------|
| Configured listener candidates only | Derive advertisements from Open Bitcoin-owned inbound listener config and runtime-bound listener evidence. | yes |
| Interface/DNS/external-IP discovery | Infer addresses from host interfaces, DNS, UPnP, or external probes. | no |
| Full Knots local-address compatibility | Accept broad baseline local-address knobs immediately. | no |

**User's choice:** Auto-selected configured listener candidates only.
**Notes:** Preserves Phase 90 opt-in listener boundaries and avoids accidental public-network or Knots `-externalip`/`-discover` compatibility claims.

---

## Bounded getaddr Response Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic capped responses | Add scoped `getaddr`/`addr` request-response behavior with count, age, cache, source, and permission caps. | yes |
| Full address relay | Implement gossip relay, rebroadcast scheduling, unsolicited address fanout, and broad peer selection. | no |
| No getaddr handling | Defer all address request handling to a future milestone. | no |

**User's choice:** Auto-selected deterministic capped responses.
**Notes:** Uses Phase 91 `addr` permission evidence as a policy input without enabling full relay.

---

## Learned Address Management

| Option | Description | Selected |
|--------|-------------|----------|
| Typed contract first | Introduce typed learned-address records with routability, source, freshness, and persistence evidence. | yes |
| Direct durable addrman clone | Attempt full Knots `addrman` persistence and selection parity immediately. | no |
| Parser-only placeholder | Decode `addr` messages without retaining typed address-management evidence. | no |

**User's choice:** Auto-selected typed contract first.
**Notes:** Keeps the implementation auditable and deterministic while creating a future-compatible address-management seam.

---

## Operator Evidence and Release Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Separate evidence labels | Distinguish local advertisements, suppressed advertisements, bounded responses, learned entries, and full-relay deferral. | yes |
| Renderer-local summaries | Add ad hoc CLI/support text without shared status fields. | no |
| Broad discovery wording | Document Phase 92 as peer discovery or full address relay. | no |

**User's choice:** Auto-selected separate evidence labels.
**Notes:** Follows Phase 90/91 status/support patterns and protects release language from overclaiming.

---

## Verification and UAT

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic pure/synthetic tests | Use pure policy tests, synthetic P2P messages, loopback fixtures, docs checks, and Bun checker fixtures. | yes |
| Public-network discovery tests | Verify with public peers, DNS seeds, real external reachability, or long-running network behavior. | no |
| Manual-only evidence | Rely on UAT text without deterministic checks. | no |

**User's choice:** Auto-selected deterministic pure/synthetic tests.
**Notes:** Preserves the repo-native `bash scripts/verify.sh` contract and public-network-free release boundary.

---

## the agent's Discretion

- Exact Rust module names and splits for address policy/address-manager types.
- Whether the initial learned-address store is in-memory or snapshot-backed, as long as persistence evidence is deterministic.
- Exact response cap constants, provided tests and docs prove the cap and no-relay boundary.
- Exact low-cardinality status/support field names, provided they are shared status fields rather than renderer-local text.

## Deferred Ideas

- Full address relay and address gossip fanout.
- `addrv2` relay parity.
- DNS seed governance and public peer discovery.
- Public inbound defaults, public-network CI, and production full-node readiness claims.

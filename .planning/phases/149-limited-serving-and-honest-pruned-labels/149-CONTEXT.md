---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-27T19:50:32.170Z
---

# Phase 149: Limited Serving and Honest Pruned Labels - Context

**Gathered:** 2026-09-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Peers and status see limited-network serving and honest `Pruned` versus
`Unavailable` labels only after real deletes. In prune mode the node
advertises `NODE_NETWORK_LIMITED`, does not advertise full `NODE_NETWORK`,
refuses block requests older than the limited serve window, and does not
serve a block whose payload prune removed. Status and RPC report `Pruned`
only when have-pruned is set and the payload is gone. A missing payload
without prune stays `Unavailable`.

This phase does not add operator prune commands, prune-height fields, prune
locks on RPC/CLI/dashboard, or support-evidence copy. Phase 150 owns those.
It does not write the v2.4 parity-doc pass or no-claim checkers. Phase 151
owns those. It does not delete payloads or set have-pruned. Phase 148
already did that.

</domain>

<decisions>
## Implementation Decisions

### Service advertisement

- **D-01:** Prune mode means `PruneMode` is manual-only or automatic. Disabled
  stays the current full-history advertisement. Advertisement follows the
  configured mode at startup, not the first successful delete. An empty
  prune plan still advertises limited service.
- **D-02:** In prune mode, local services are `NETWORK_LIMITED | WITNESS`
  and must not include `NETWORK`. `NETWORK_LIMITED` is service bit 10
  (`1 << 10`), added on the existing `ServiceFlags` newtype. Do not add
  the `bitflags` crate, bloom, compact-filter, or other new service bits.
- **D-03:** When prune is disabled, keep today's `NETWORK | WITNESS`
  default. Do not add `NETWORK_LIMITED` to the non-prune default in this
  phase. Knots full nodes also set the limited bit; that co-advertisement
  is a Phase 151 parity note, not a Phase 149 behavior change.
- **D-04:** Only our advertised services change. Do not build a new
  historical-download or peer-selection policy for remote limited peers.

### Limited serve window

- **D-05:** The limited window reuses `MIN_BLOCKS_TO_KEEP` (288). A block
  request is outside the window when its tip distance is greater than
  `288 + 2`. The `+2` is the Knots `net_processing` race buffer from
  milestone research, not a second keep window for deletion. Phase 147's
  prune keep window stays exactly 288.
- **D-06:** The window gate applies only while prune mode is on. Disabled
  prune still serves any block whose payload is present, including blocks
  older than 288.
- **D-07:** Outside the window, do not send block bytes even when the
  payload is still on disk. That is the BIP 159 fingerprinting rule:
  peers must not learn the real prune depth. Inside the window, serve
  only when the existing payload-present fact is true.
- **D-08:** A request whose payload prune already removed is not served,
  whether or not it also falls outside the window. Do not reconstruct
  the block from undo, coins, or leftover snapshot bytes.

### Honest Pruned versus Unavailable

- **D-09:** Emit `Pruned` / `BlockServingDataAvailability::Pruned` only
  when the durable have-pruned flag is set and the payload is absent.
  Have-pruned is the Phase 148 flag, never inferred from prune config or
  from "bytes missing".
- **D-10:** A missing payload with have-pruned false stays `Unavailable`,
  including when prune mode is configured but no delete has committed.
- **D-11:** Payload present stays `Available`, even if have-pruned is
  already true for older heights. `Pruned` is not a chain-wide status.
- **D-12:** `Pruned` applies to index-known or active-chain blocks that
  would otherwise be a missing-payload result. An unknown hash stays the
  existing not-found path. Do not relabel `Unknown`, `Stale`, `SideChain`,
  or `Unvalidated` as `Pruned`.
- **D-13:** Update the reserved-label comments on
  `BlockServingDataAvailability::Pruned` and
  `BlockServingStatusLabel::Pruned` so they describe this earned label.
  Classifier tests may keep injecting `Pruned` directly. Production code
  may inject it only under D-09.

### Wire refusal

- **D-14:** Out-of-window and pruned-payload requests do not get a new
  wire message. Reuse the Phase 143 missing-inventory / `NotFound` refusal
  for "not served".
- **D-15:** An ordinary peer that requests a block body outside the limited
  window is disconnected, matching Knots historical-block serving. A peer
  that already has the existing download-serving permission is refused
  without that disconnect. Do not invent a new permission flag. Map the
  exemption onto `PermissionEffectLabel::DownloadServingPolicyInput` if
  that is the live download permission; otherwise the closest existing
  download permission, recorded in research.
- **D-16:** In-window missing payload does not disconnect by itself. It
  stays the Phase 143 unavailable or, when D-09 holds, pruned refusal.

### What the window covers

- **D-17:** The window and pruned-payload checks cover block-body
  `getdata` and compact-block / block-txn body serving. Those paths
  already share `managed_block_serve_input`; extend that input rather
  than adding a second probe.
- **D-18:** Header serving and header sync are unchanged. This phase does
  not apply the block-body window to `getheaders`.
- **D-19:** In prune mode, do not offer historical block-body inventory
  outside the limited window. Recent-window inventory stays. Do not add
  BIP37 merkle-block serving or compact-filter serving.

### Boundaries carried forward

- **D-20:** No prune RPC, CLI, or dashboard fields, no manual prune
  command, no prune-lock operator surface, and no support-bundle prune
  counts. Phase 150 owns those.
- **D-21:** New first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`. Cite
  pinned Knots service-bit and limited-serve anchors where a defensible
  source line exists. The v2.4 parity-doc pass and no-claim checkers stay
  in Phase 151.
- **D-22:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. No new
  production crate or third-party library.

### Claude's Discretion

- Module placement for the pure window predicate versus the node-shell
  service-flag and disconnect wiring, as long as the window math stays
  I/O-free and Fjall stays in the node shell.
- The exact tip-distance comparison that implements "greater than 288 + 2",
  as long as it matches the Knots limited-serve check the researcher cites
  and does not widen Phase 147's delete keep window.
- How status and RPC project the existing `Pruned` label, as long as the
  D-09 conditions are the only production source of that label.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract

- `.planning/ROADMAP.md` — Phase 149 goal, SERV-01, SERV-02, SERV-03,
  LABL-01, success criteria, and the research flag for inventory and
  block-serve fact wiring including Knots getdata edge cases
- `.planning/REQUIREMENTS.md` — SERV-01..03 and LABL-01 wording; operator
  surfaces stay OPER-01..03 and LOCK-02 in Phase 150
- `.planning/PROJECT.md` — functional core stays I/O-free; `Pruned` only
  after a real delete; `NODE_NETWORK_LIMITED` is in the v2.4 scope
- `.planning/research/STACK.md` — `NETWORK_LIMITED = 1 << 10`, serve
  refusal at 288 + 2, `Pruned` only when `have_pruned && !payload_present`
- `.planning/research/FEATURES.md` — prune mode never adds `NODE_NETWORK`;
  ordinary peers are disconnected for historical block requests past the
  limited window
- `.planning/research/SUMMARY.md` — label-lie and full-`NETWORK` pitfalls
- `.planning/phases/148-fjall-payload-unlink-and-have-pruned/148-CONTEXT.md`
  — have-pruned only after a durable paired delete; this phase must not
  set that flag
- `.planning/phases/147-pure-prune-policy-and-lock-windows/147-CONTEXT.md`
  — `PruneMode`, `MIN_BLOCKS_TO_KEEP` = 288, delete window stays exact
- `.planning/phases/143-honest-stored-block-availability/143-CONTEXT.md`
  — missing payload is `Unavailable` until prune earns `Pruned`; refuse
  with the existing not-found path

### External contract

- BIP 159 (`NODE_NETWORK_LIMITED`, bit 10) — must serve at least the last
  288 blocks, must not also claim full historical serving, and should not
  serve deeper than that threshold so prune depth is not fingerprinted.
  https://bitcoin.org/bip/159/

### Pinned Knots anchors

- `packages/bitcoin-knots/src/protocol.h` — `NODE_NETWORK_LIMITED` bit
- `packages/bitcoin-knots/src/net_processing.cpp` —
  `NODE_NETWORK_LIMITED_MIN_BLOCKS` (288), the +2 historical-request
  buffer, and the ordinary-peer disconnect on too-old block requests
- `packages/bitcoin-knots/src/init.cpp` — prune mode does not add
  `NODE_NETWORK`; limited service is the prune advertisement
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `IsBlockPruned`
  requires `m_have_pruned` and missing block data
- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` (288)

### Existing Open Bitcoin seams

- `packages/open-bitcoin-network/src/message.rs` — `ServiceFlags` and
  `LocalPeerConfig::default` (`NETWORK | WITNESS`)
- `packages/open-bitcoin-network/src/block_serving.rs` —
  `classify_block_serving_status`, reserved `Pruned` variants
- `packages/open-bitcoin-node/src/network/inventory.rs` —
  `managed_block_serve_input`; today missing payload is always
  `Unavailable`
- `packages/open-bitcoin-chainstate/src/prune/range.rs` —
  `MIN_BLOCKS_TO_KEEP`
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` —
  durable `have_pruned` load path
- `docs/parity/source-breadcrumbs.json` — required breadcrumb registry

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `ServiceFlags` is a `u64` newtype with `NETWORK` (`1 << 0`) and
  `WITNESS` (`1 << 3`). `NETWORK_LIMITED` belongs beside those constants.
- `classify_block_serving_status` already maps injected
  `BlockServingDataAvailability::Pruned` to `BlockServingStatusLabel::Pruned`
  and refuses to serve anything except `Available`. The shell is what
  must start injecting `Pruned` honestly.
- `managed_block_serve_input` already computes `payload_present` from
  cache or durable bytes and feeds the classifier. The window check and
  have-pruned fact belong on this input, not a second probe.
- `FjallNodeStore::load_have_pruned` is the durable flag. Phase 148 tests
  prove it stays false until a paired delete commits.
- `MIN_BLOCKS_TO_KEEP` already exists in chainstate prune policy. The
  serve buffer is additional policy, not a change to that constant.

### Established Patterns

- Pure network and chainstate code stays I/O-free. Service bits and the
  window predicate can live in the network or chainstate crate. Reading
  have-pruned and choosing disconnect stay in the node shell.
- Phase 143 refusal is `NotFound` / missing inventory, not a new message.
  This phase adds the limited-window disconnect on top of that refusal
  for ordinary peers only.
- Compact announcement and compact-txn paths already reuse
  `managed_block_serve_input`.

### Integration Points

- `LocalPeerConfig` construction wherever the node builds its version
  message from prune mode.
- `managed_block_serve_input` in
  `packages/open-bitcoin-node/src/network/inventory.rs`.
- Status and RPC projections that already render
  `BlockServingStatusLabel`, including `pruned_count` in block-relay
  evidence. Those counters may move only when the earned label is emitted.

</code_context>

<specifics>
## Specific Ideas

BIP 159 is the external contract: bit 10 means "I can serve at least the
last 288 blocks" and must not be combined with a full-history
`NODE_NETWORK` claim on a pruned node. Milestone research is more specific
than the phase success text: Knots refuses historical block requests at
tip distance greater than 288 + 2, and disconnects ordinary peers who ask.
The success criteria's "refused" and "not served" stay the byte outcome;
the disconnect is the Knots peer outcome for the out-of-window case.

Have-pruned remains stricter than Knots startup order. Phase 148 records
it only after the delete commits. This phase reads that flag and does not
move it earlier.

</specifics>

<deferred>
## Deferred Ideas

- Operator prune status, manual prune, prune locks, and sanitized support
  evidence stay in Phase 150.
- Parity-doc citations and no-claim checkers stay in Phase 151, including
  whether a non-prune node should also advertise `NETWORK_LIMITED` beside
  `NETWORK` the way Knots full nodes do.
- BIP37 merkle blocks, compact-filter serving, and archive historical
  serving stay out of v2.4.
- A new downloader preference for remote `NODE_NETWORK_LIMITED` peers is
  not part of this serving phase.

None of the discussion widened the phase into those surfaces.

</deferred>

---

*Phase: 149-limited-serving-and-honest-pruned-labels*
*Context gathered: 2026-09-27*

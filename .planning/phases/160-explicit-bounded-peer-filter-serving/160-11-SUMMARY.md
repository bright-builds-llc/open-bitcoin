---
phase: 160-explicit-bounded-peer-filter-serving
plan: "11"
subsystem: testing
tags: [bip157, tcp, fjall, provenance, knots, bounded-memory]
requires:
  - phase: "160-09"
    provides: Configured inbound executor and actual socket failure seams
  - phase: "160-10"
    provides: Guarded production outbound TCP executor
  - phase: "160-14"
    provides: Shared mixed pending ledger and source-derived encoder peak accounting
  - phase: "160-16"
    provides: Immutable independent wire/proof corpus and bounded pinned helper driver
provides:
  - Genuine durable-chain outbound limits, requested stale branches, missing-row and paired-body-absence evidence
  - Configured daemon versions and all-family exact inbound pinned-serializer comparisons
  - Concrete shared-authority mixed transport admission in both orders and eight barrier races
  - Independent 42-case Rust wire and 27-case exact proof comparisons
affects: ["160-12", "phase160-final-verification"]
tech-stack:
  added: []
  patterns: [Real TCP delegated encoded writes, Shared-handle resource observation, Bounded pinned helper reuse]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed/race.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs
    - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/tests/families.rs
key-decisions:
  - Distinguish synthetic protocol goldens from genuine accepted continuous spend history; dynamically serialize genuine public record fields with the unchanged pinned helper.
  - Delegate helper pin checking, input/capture bounds and ten-second subprocess deadline to the existing exported Bun runBounded driver.
  - Use the same configured handle and the public outbound sync_once loop, overriding send_encoded only to pause and then delegate unchanged bytes to concrete TcpPeerTransport.
  - Keep raw-body and UnknownLegacy controls at immediate managed dispatch because ordinary sync maintenance may legitimately validate a retained body before a later request.
requirements-addressed: [CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: 2026-10-11T02:14:37Z
duration: approximately 27min
completed: "2026-10-11"
---

# Phase 160 Plan 11: Concrete Peer Filter Evidence Summary

**Genuine durable spending histories serve exact CF bytes on both TCP directions, while one shared mixed ledger enforces both admission orders, exact boundaries and eight physical-winner races.**

## Tasks and Acceptance

Both evidence tasks are complete. All changes await the strict outer consolidated commit.

- [x] Genuine positive fixtures use continuous consensus acceptance, historical and same-block spends, concrete durable coins and Fjall; no MemoryCoinsView-only or raw snapshot replaces positive evidence.
- [x] Actual outbound bytes and internal shared reservations are asserted, including whole legal limits and no shortened success.
- [x] Both actual directions have successful writes, forced first/second partial writes, lifecycle/cancellation/queue/timeout/disconnect cleanup and complementary mixed traffic evidence.
- [x] Production failure observations retain sanitized fixed kinds and explicit local dispositions; raw private error text is not a peer CF response.

## Concrete Evidence

The node fixture cold-reopens 3,002 genuinely accepted continuous blocks. Real outbound frames cover genesis, inclusive 1,000-filter and 2,000-header limits, corresponding one-over disconnects, and checkpoints at 0/999/1,000/1,001/2,000/3,001. An actually missing required checkpoint at 2,000 yields no response to stop 3,001. An actual legal paired deletion removes height20 body and undo while its indexed filter still serves. This is a narrow eligibility/body-independence check, not the full Phase162 prune/reopen product lifecycle.

Genuine competing branches are accepted and indexed through ordinary managed reorgs. The displaced requested branch serves its exact own hashes/filter bytes rather than active-height substitutes. Actual outbound preparation/execution also covers complete lagged prefixes, missing middle/final rows, missing predecessor, corruption and backend failure, with no query-time catch-up or partial normal response.

The genuine raw-block test sends the first three accepted bodies and actual historical/same-block spent scripts to the unchanged independent BASIC helper. It checks block identity, encoded filter and chained header, then uses the pinned six-family serializers to compare all three response-family TCP frames. Configured daemon tests independently frame genuinely persisted records and observe ordinary configured inbound startup, a lagged complete prefix, later missing ranges, retained stale ancestry and actual paired body/undo absence.

Static Plan16 protocol cases are explicitly synthetic serializer controls. The separate wire test checks all42 typed-input payload/frame/decode identities. Root contributed the std-only consensus fixture reader: all27 proof rows and all23 defined signed equivalent-time expectations match; four invalid-target rows have undefined time and are excluded from division.

Header-only refusal is observed through the real outbound loop. Raw-body and UnknownLegacy controls assert immediate managed dispatch and strict UnknownOrDisallowedStop. Running full background sync after saving a valid raw body can legitimately accept it, so those controls deliberately do not falsely claim unchanged provenance through that loop. Old timestamp and old proof tests retain a genuinely accepted stale stop but explicitly seed extreme best-header facts only as negative controls; they never claim the seeded headers were consensus accepted.

## Shared Mixed Transport Proof

One configured FilterListenerFixture exposes the same handle to an actual inbound executor and the public outbound sync_once loop. ControlledSession overrides send_encoded and delegates the unchanged borrowed slice to concrete TcpPeerTransport; it never uses the scripted default.

- Generic-first: the actual outbound own-version buffer remains retained while inbound CF admission is attempted. Internal generic plus active CF charges equal exactly8MiB on acceptance; one byte over refuses with zero completed CF messages and no CF write.
- CF-first: an actual handed CF and queued suffix remain live while the public outbound encoder attempts its independently source-sized peak. Exact available peak headroom admits; one byte less refuses before normal output even though the eventual two final frames would fit.
- Eight barrier races gate a physically received Ping immediately before production Pong admission against inbound CF admission. Each race holds whichever actual writer wins, asserts exactly one physical winner and bounded internal totals, checks the observed Pong/CF bytes, and reaches zero only after real buffers settle.
- Encoding scratch is gone when an encoded write blocks. Tests preserve its separate admission peak and do not falsely report scratch still live after the encoder returns. Queued suffixes and handed cancellation charges remain independently tested by the actual executor regressions.

The real ingress control sends valid and invalid requests from all three families before VERSION and before VERACK, then completes handshake and observes a Pong sentinel. It also sends all three valid unsolicited response families after handshake; no CF effect or disconnect occurs. These socket checks verified the root's independently reviewed WR-01 handshake and WR-02 bounded allowlist repairs.

## TDD and Verification

Pinned Bun1.3.9 and scripts/command-timings.ts serialized every Cargo invocation. Commands were polled within60 seconds; quiet 3,002-block tests were allowed to finish.

| Check | Nonzero final result |
| -- | -- |
| Required node filter phase160_peer_evidence | **8 passed**, 0 failed, 81.59s |
| Required daemon bin filter phase160_peer_evidence | **4 passed**, 0 failed, 18.46s; exact required command after final matrix strengthening |
| Additional RPC library filter phase160_peer_evidence | **4 passed**, 0 failed, 20.61s |
| Consensus filter phase160_peer_evidence_ | **2 passed**, 0 failed;27 proof/23 defined time cases |
| Existing node phase160_outbound_ regressions | **18 passed**, 0 failed, 11.31s |
| All inbound_listener::tests::peer_filters regressions | **18 passed**, 0 failed, 26.79s |
| Existing configured phase160_startup_ regressions | **6 passed**, 0 failed, 3.64s |
| Node and RPC strict Clippy, all targets/features, -D warnings | Passed |
| Scoped rustfmt check and git diff --check | Passed |
| Managed starter checks | Zero findings |

The exact required commands are:

```bash
bun run scripts/command-timings.ts run --key phase160-peer-evidence-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160_peer_evidence -- --nocapture
bun run scripts/command-timings.ts run --key phase160-peer-evidence-daemon -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind phase160_peer_evidence -- --nocapture
```

The final configured matrix crosses general off/on with ordinary, explicit blockfilters, all and all+explicit while enabled, plus disabled ordinary/all. Existing six startup regressions cover disabled explicit/all+explicit/general prerequisite refusal and unchanged retained sources. The all-only disabled-backend socket test proves valid requests remain silent and a subsequent unsupported-type request disconnects without response.

Task1 RED executed one test and failed because the initial test expected a response plan from a semantic one-over request; actual dispatch correctly returned a disconnect error. Intermediate fixture reconnect expectations and visibility were corrected. Task2 RED failed compilation on the new concrete test harness, and its actual ingress test then exposed the reviewed unsolicited-response reset until root's WR-02 fix. Final GREEN includes both root review repairs. Existing static proof tests were added as cross-checks; no fabricated failing production proof baseline is claimed.

## Scope, Deviations and Simplification

The parent explicitly expanded the original nine-path budget to fourteen: a narrow configurable test-fixture constructor for the public outbound loop, the existing manual inbound family handshake update, and three small transport test children. The five new Rust paths have pinned parity comments and were serially registered by the parent. No generator, oracle, dependency, production crate, storage schema, shared planning state or manifest was changed by this executor.

Root supplied the two proof tests and42-case wire comparison. Plan11 supplied the concrete evidence and extended that already-owned wire child with negative controls. The public own-version fixture default keeps listener activation off; only the explicit serving fixture starts a local listener.

The simplification pass retained one existing pressure helper and the production affine owners. It extracted the race from a716-line test file into a276-line child, used a small test facade rather than exposing private history types, reused the existing Bun bounded driver, and replaced redundant tail conditions with one is_multiple_of check. New evidence files remain below628 lines. No new production abstraction or second accounting pool was introduced.

No authentication gate, unresolved stub or new production trust-boundary surface was introduced. T-160-32 is covered by the genuine/negative provenance separation. T-160-33 is covered by concrete observed bytes, exact earned prefixes and settled shared ownership under forced failures.

## Outer Finalization

The parent owns the full native verifier, independent review/security receipts, lifecycle verification, STATE/ROADMAP/requirements and strict commit/push. This executor performed no staging, commit, push, branch/worktree change or hook bypass. Requirements-completed stays empty. No Phase161 operator/UI, Phase162 integrated product, public/default listener or full production synchronization claim is made.

## Self-Check: PASSED

The five new Rust files, all fourteen scoped paths and this summary exist. Required node and daemon filters executed8 and4 nonzero passing tests; consensus2, actual mixed/ingress library4, outbound18, inbound18 and startup6 also passed. Scoped formatting, strict affected-crate Clippy, managed rules and whitespace checks passed. Root independently reviewed the fourteen paths and owns final metadata/verification. Commit-existence checks are intentionally inapplicable under the explicit no-intermediate-commit lifecycle; no commit or requirement-completion receipt is fabricated.


## Workspace preflight fixture correction | 2026-10-11

Root's first broad Rust preflight found three older manually constructed peer setups that depended on pre-handshake dispatch. Root completed their Version/Verack exchange using the existing helper, without changing assertions or production policy. The two Plan11-owned files remain in the fourteen-path scope; the third is original Plan08 serving/tests.rs. All three focused reruns passed1/1, including genuine reorg, four held reservations/semantic precedence and complete/missing catch-up. The remaining previously unreached primitives/RPC/test-harness/wallet preflight also passed. This is not a claim that the final full native contract has already passed.

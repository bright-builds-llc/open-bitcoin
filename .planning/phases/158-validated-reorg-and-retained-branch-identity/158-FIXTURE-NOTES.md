# Phase 158 Fixture Reuse Notes

**Date:** 2026-10-08 CDT
**Evidence:** Focused read-only source scout; these notes claim no new executed test.

- Reuse `sync/tests/filter_index/catch_up/fixtures.rs::TurnHistory::new` and `seed` for compact continuous historical/same-block spends. Open with `DurableSyncRuntime::open_configured(..., BasicFilterStartupMode::Enabled)` so the genuine recovered owner is installed.
- Follow `chainstate/fjall_store/tests/fixture.rs::ReorgFixture::stage_from` for a separately validated oracle: fork from the ancestor, reverse displaced blocks, use genuine stage positions/undo and the ancestor header to generate expected records. Drop the oracle stage before production `network.reorg_to_branch`.
- Drive `network.drive_basic_filter_index_turn`; preserve its actual accepted target/lag and progress. Configured startup performs one eight-block turn, so choose observed post-open prefixes for lag cases instead of injecting progress.
- A later connect uses saved actual body plus `network.connect_stored_block`, then the same owner. `network.flush_coins(FlushMode::Always, ...)` earns durability; index turns alone do not.
- Real reopen drops runtime, store clones and proofs, then reopens Fjall and configured Enabled runtime. Compare active-height records with displaced hash lookup, retaining exact bytes/hash/header/parent.
- Keep sync reconciliation assertions in the existing sync test visibility rather than widening production APIs only for fixtures.

## Default-maturity case

The configured regtest runtime derives maturity 100; pinned Knots also uses 100. Existing compact helpers explicitly use maturity one. For the added standard case, generate heights 1–100 as coinbase-only, spend height-1 reward at 101, then chain non-coinbase outputs with same-block children. Use the actual runtime parameters and flags without mutation. Mine PoW and valid coinbase heights, fork from 100 and remain below 150. The runtime's current halving interval differs from Knots regtest at 150, so this test does not imply full parameter parity.

Suggested modules: fixtures for bootstrap/reopen/finish/flush; branches for valid bodies and staged oracle; retention for active/hash equality. Plan 06 consumes these helpers for loss/fault/protection/measurement gates. Bodies and expected records are test data, never production authority or a history cache.

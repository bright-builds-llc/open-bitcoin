---
phase: 167-candidate-history-validation-rules
plan: "15"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: ordinary-regtest-corpus-and-managed-history
tags: [rust, bun, regtest, corpus, chainstate, rpc, evidence]
requires:
  - phase: 167-14
    provides: Accepted bounded default-Regtest support and parsed CLI same-datadir evidence
provides:
  - Independently expected genesis-through-301 serialized ordinary corpus and next302
  - Actual selected default-Regtest managed connect, rejection, checkpoint and reopen evidence
  - Source-bound BIP68 block reason and public missing-input RPC rejection controls
affects: [167-16, 167-49, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [whole-byte frozen JSON, selected canonical rules, classified internal and public refusals]
key-files:
  created:
    - scripts/phase167-regtest-corpus.ts
    - scripts/phase167-regtest-serialization.ts
    - scripts/phase167-regtest-corpus.test.ts
    - packages/testdata/phase167/regtest-history.json
    - docs/parity/fixtures/phase167-regtest-expectations.json
    - packages/open-bitcoin-node/src/sync/tests/phase167_ordinary.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_support.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
key-decisions:
  - Preserve pinned public BIP68 and missing-input reasons while distinguishing direct typed errors.
  - Keep seeded missing-retarget and recovery controls in Plan 16.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
generated_at: "2026-10-27T21:22:35Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final15
---

# Phase 167 Plan 15: Serialized ordinary Regtest history

The nine-owner execution and independent evidence review passed; canonical Summary QA and Root FINAL15 remain pending. The [evidence index](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/index.json), [complete 467-source frame](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/final-frame.json), and [nine-owner scratch inverse](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/nine-owner-scratch-inverse.json) pin the final source and logs. No Git staging, commit or push occurred. Phase 168 remains held.

## Evidence references

- Accepted [FINAL14](../../../.local/open-bitcoin-dev/phase167/plan14/root-final14-v1/final-source-acceptance.json), SHA-256 `4024761b6439e5ded5dfcec23f09853080646416b11e1fdbf196f6d71e3afe34`.
- [Final execution index](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/index.json), SHA-256 `771684be1b2042191c8d8b16ae17ca1d8f992bcfb857c3d113b115f141e6476b`; [unified source/native frame](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/final-frame.json), SHA-256 `c0e2053ca189bcedcab24c02fd2da85157ca12cf4f23a972307eaa7c24069ebf`; [scratch inverse](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/nine-owner-scratch-inverse.json), SHA-256 `36428e01d904fc32519d8a2624e09f560980e4ba7a8b777777f2c7a9cc474811`.
- Root's checked [BIP68 block-rejection amendment](167-15-BIP68-BLOCK-REJECTION-AMENDMENT.md) and [repair closure](../../../.local/open-bitcoin-dev/phase167/plan15/root-bip68-repair-v1/root-fix-closure.json), SHA-256 `5792443f6ea2e6951ad2583b9da2aa22cd9f9c97595cf8dad7f4d6fc8692ea71`.
- Checked [missing-input evidence-class amendment](167-15-MISSING-INPUT-EVIDENCE-CLASS-AMENDMENT.md) and [public RPC amendment](167-15-MISSING-INPUT-RPC-EVIDENCE-AMENDMENT.md); final [RPC closure](../../../.local/open-bitcoin-dev/phase167/plan15/root-missing-input-rpc-v1/root-rpc-closure.json), SHA-256 `6784d8936d1b45f558d8333aa02cce238887a082f86b9d50b1e492af168ac30f`.
- Independent [Task 1](../../../.local/open-bitcoin-dev/phase167/plan15/independent-task1-actual-review-2026-10-27.json), SHA-256 `b11494d65f732de34e1d1b2ca9212512c1afb00426dd603fe7f59b7f35eab954`; [Task 2](../../../.local/open-bitcoin-dev/phase167/plan15/independent-task2-actual-review-2026-10-27.json), SHA-256 `e91499ca5529e07d324a60cfaec16306209f058293ea3aca75eb82fade11f778`; and [final evidence review](../../../.local/open-bitcoin-dev/phase167/plan15/independent-final-evidence-review-2026-10-27.json), SHA-256 `df240b241d721f1ac64be52d21f8dcad23cb4b424500ba9d83984f4b3b98ba40`.

## Changed source and evidence class

Task 1 added a deterministic source-derived default-Regtest corpus generator, pure serialization/mining and shared raw-byte comparator, 32 registered Bun cases, 302 serialized rows (genesis through 301), a distinct next 302, 19 named isolated negative candidates, and two pretty JSON frozen artifacts. Source-derived independently expected subsidy, fees, UTXO, work, BASIC and creating-parent MTP remain data, not authority. The two JSON artifacts were regenerated to reflect the current protected `scripts/basic-filter-oracle.py` SHA256; their data and facts match the inert prepared originals in every other field.

Task 2 registered ordinary node tests through actual selected default-Regtest canonical genesis, header ingress, managed durable connect, complete-handle reopen at 150 and 301, next 302 connect, full UTXO/work/BASIC/provenance checks, live height 201 distant creating-history extension, genuine alternate-branch MTP disagreement, isolated 19 negative names, structural unseeded missing-MTP precedence, and physical durable corruption refusal. The direct missing-input path checks exact typed `MissingCoin` and serialized absent outpoint; it does not claim to execute the public submission mapping. The frozen Knots public reason stays `bad-txns-inputs-missingorspent`.

Root separately owns the checked block-context BIP68 repair in `apply.rs` and `phase167_scripts.rs`, which maps block BIP68 failure to Knots `bad-txns-nonfinal` while retaining the transaction/mempool reason. Focused 1/1 and 17/17 chainstate tests passed after actual behavioral RED. Root also owns the separate RPC canonical-history regression using the same frozen missing-input bytes; its strengthened final run passed 1/1 with parse→re-encode equality and full live/durable provenance preservation. Those three files are external protected owners, not Plan 15 edits. Plan 14's accepted same-physical-datadir adapter evidence compares RPC blocks/hash with parsed CLI height/hash, typed tip work with CLI work, and CLI BASIC safe height; Plan 15's node source uses a separate datadir and claims no RPC work/BASIC fields.

## Actual test-first and final commands

- Test-only Bun import failure was setup, zero named discovery. First runnable exact Bun test had 32 discovered, 28 pass/4 fail, limited to stale BASIC oracle source-pin bytes in both frozen files. This is genuine frozen-byte RED; refreshed files yielded 32/32 green. Final `bun test ./scripts/phase167-regtest-corpus.test.ts` passed 32/32. Actual `bun scripts/phase167-regtest-corpus.ts --check` passed complete raw bytes of both files. Registered cases include canonical positives plus compact, indentation, extra/missing LF, BOM, invalid UTF-8 and reordered-key refusals for both file kinds.
- First literal timing-wrapped Cargo selection stopped at two owned test compile mismatches; setup only. The compiled seven-test run then had four passes/three failures on actual BIP68 block reason, a genuine product parity RED. After Root's BIP68 fix it had six passes/one failure due the direct/internal missing-input evidence-class assertion; no product RED credit for that assertion. After the checked evidence-class clarification, the final literal command below, with pinned Bun on the PATH and isolated `CARGO_TARGET_DIR`, passed seven named ordinary tests with zero failures in 112.50 seconds. A separate zero-discovery integration binary receives no credit. Failed attempts and complete logs/exits are in the index.

```bash
bun run scripts/command-timings.ts run --key phase167-15-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_ordinary -- --test-threads=1
```

## Supplemental focused commands

These commands used the pinned Bun PATH and the isolated `CARGO_TARGET_DIR=.local/open-bitcoin-dev/phase167/plan14/cargo-target`; their actual outcomes are bound by the Root closure receipts above. The first default-target BIP68 GREEN attempt stalled in the ignored target directory and earned no pass credit.

```bash
bun run scripts/command-timings.ts run --key phase167-15-bip68-block-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_scripts_same_block_created_parent_mtp_and_relative_locks -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-15-bip68-block-green-isolated -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_scripts_same_block_created_parent_mtp_and_relative_locks -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-15-bip68-block-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_scripts -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-15-missing-input-public-rpc-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase167_canonical_history_missing_input_has_public_knots_reason -- --test-threads=1
```

The corrected chainstate commands passed 1/1 and 17/17 named tests. The final public RPC command passed 1/1 on the exact frozen missing-input bytes. The pre-review RPC run remains historical and supplies no final-source credit.

## Complete source frame, review and limits

Fresh before frame: 458 source, 201 native and one supplemental native states, zero drift; nine owners comprised seven true absences and two whole-body originals. The **unified final causal frame** contains 467 unique present source paths: the original 458 plus seven newly added Plan 15 owners and two newly included Root chainstate files. It also contains complete copied bytes and hashes for all 467 source paths and 201 plus one native inputs, 669 whole copies total. The Root RPC test path overlaps the original 458, so it appears once. The final protected set excludes the nine Plan 15 owners and has **458 unique paths**, including all three Root external paths once. Relative to the original 458 membership, only the Plan 15 registrar and Root RPC test changed; the seven Plan 15 owners and two Root chainstate paths are the nine additional members. Native and supplemental-native inputs have zero drift.

The scratch inverse used the complete copied 467-state frame, restored all nine original owner states (two whole bodies and seven true absences), preserved all 458 protected states, then reapplied all 467 final source states without live source writes. Rustfmt, Prettier, owned whitespace/diff review and normal 628-line fit passed; maximum owned source is 615 lines. The only present original Rust change is two module registrations; `sync/canonical_history/tests.rs` remains byte-identical. The earlier partial 458-membership frame and inverse are retained as [historical evidence](../../../.local/open-bitcoin-dev/phase167/plan15/execution-v1/historical-partial-final-frame.json) and do not provide final-frame credit.

Independent Task 1, Task 2 and final evidence reviews passed; canonical Summary QA and Root FINAL15 are pending. Plan 20 and Root retain full native, coverage, Bazel, security, goal, lifecycle and normal-hook Git gates. These results do not constitute pinned Knots-daemon differential qualification, unattended sync activation or Phase 168 acceptance.

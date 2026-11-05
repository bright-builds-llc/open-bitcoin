# Candidate History Validation and Bounded Canonical Regtest

Surface id: `v2-6-candidate-history-validation`. Requirements addressed:
CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06. This is current Phase 167
implementation evidence, with Phase 167 formal acceptance passed under the
complete Plan 20 native, coverage, Bazel, independent review and lifecycle gates. The pinned reference is Bitcoin
Knots `29.3.knots20260210` at
`a9aee730466ac67d35a3c03ee24676be5e045878`.

## Rule owners and evidence classes

[Selected network rules](../../packages/open-bitcoin-consensus/src/chain_spec/rules.rs)
are private immutable facts from a selected `ChainSpec`. The
[checked candidate history](../../packages/open-bitcoin-consensus/src/candidate_history.rs)
and [candidate rules](../../packages/open-bitcoin-consensus/src/candidate_history/rules.rs)
validate structural branch context; their `IndependentlySeeded` origin is test
evidence, not earned canonical lineage. A canonical product body additionally
needs the actual current header admission, work owner, coins view and accepted
stage. The [chainstate gate](../../packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs)
binds those owners, and the [sealed history opener](../../packages/open-bitcoin-node/src/sync/canonical_history.rs)
consumes a complete original-store proof before publishing a local runtime.

| Evidence class | What it establishes | What it does not establish |
| --- | --- | --- |
| Pinned source and corpus bytes | Exact selected rule formulas, historical identities and independent expected outputs | Execution or an admitted chain |
| Independently seeded historical cases | Mainnet/Testnet3 script exceptions, Mainnet BIP30 repeats and Regtest flag-sensitive regression behavior through the real kernel | Canonical product authority for those networks |
| Bounded local default-Regtest canonical route | Proved genesis plus 301 accepted descendants, private submission and complete retained reopen in actual Open Bitcoin processes | Whole-chain or public/default operation, wallet authority or an actual pinned-Knots daemon differential |

The pinned source counterparts are
[`kernel/chainparams.cpp`](../../packages/bitcoin-knots/src/kernel/chainparams.cpp),
[`validation.cpp`](../../packages/bitcoin-knots/src/validation.cpp),
[`consensus/tx_verify.cpp`](../../packages/bitcoin-knots/src/consensus/tx_verify.cpp),
[`pow.cpp`](../../packages/bitcoin-knots/src/pow.cpp),
[`coins.cpp`](../../packages/bitcoin-knots/src/coins.cpp),
[`script/interpreter.cpp`](../../packages/bitcoin-knots/src/script/interpreter.cpp)
and [`undo.h`](../../packages/bitcoin-knots/src/undo.h). The
[source breadcrumb catalog](source-breadcrumbs.json) binds each first-party
Rust owner to its exact existing Knots comment. Local filesystem and Fjall
inspection helpers with `none` comments have no direct Knots counterpart; the
Plan 19 source inventory records their individual rationale.

## Selected contextual rules

The [source rule table](../../packages/open-bitcoin-consensus/src/chain_spec/rules.rs)
selects 100-block coinbase maturity for all four profiles. Regtest halves at
150 blocks; the other selected defaults halve at 210,000. These facts alone do
not enable ordinary history on another selector.

| Selected profile | BIP34 | BIP66 | BIP65 | CSV/BIP113 | Segwit | Product ordinary route |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Mainnet | 227,931 | 363,725 | 388,381 | 419,328 | 481,824 | Refused |
| Testnet3 (`testnet` CLI) | 21,111 | 330,776 | 581,885 | 770,112 | 834,624 | Refused |
| Default Signet | 1 | 1 | 1 | 1 | 1 | Refused; challenge validation remains missing |
| Default Regtest | 1 | 1 | 1 | 1 | 0 | Bounded private local canonical submission |

Exact hash-specific script flag replacements happen before deployment
additions. The [script interpreter](../../packages/open-bitcoin-consensus/src/script/witness.rs)
and [transaction validator](../../packages/open-bitcoin-consensus/src/transaction.rs)
consume the derived flags; the [block sigop budget](../../packages/open-bitcoin-consensus/src/block/sigops.rs)
counts the coinbase and every other transaction. Contextual headers use the
selected difficulty, exact parent and prior median time; body admission retains
the originally observed clock. The MTP window contains the candidate and ten
parents, or eleven parents before the candidate, never a twelfth sample.
Sequence locks use parent MTP after CSV. These source seams and named native
regressions are pinned by the [Phase 167 checker](../../scripts/check-phase167-candidate-history.ts)
and its [mutation suite](../../scripts/check-phase167-candidate-history.test.ts);
actual behavior remains the native Rust tests.

## Historical corpus and the BIP30 restoration difference

The [historical manifest](../../packages/testdata/phase167/manifest.json)
(SHA-256 `16649d2cee73bfd769632bd8b59a06af50ee2d8bb69a2a71dcaad8be88d53b09`)
pins seven required block identities and twelve Knots source files. The
[raw blocks](../../packages/testdata/phase167/blocks.json)
(SHA-256 `6e81f7f06037d8c1e18b75ddef67d16aa6ab09a505ca616db612de1f29e0c335`),
[contexts](../../packages/testdata/phase167/contexts.json)
(SHA-256 `3067e98fb235c97226482a6c1b9fad2484fe58610f5ed1c25e8860a0a970fd5a`),
[BIP30 typed projection](../../packages/testdata/phase167/bip30-fixtures.txt)
(SHA-256 `7b91df3ceee0f702c46f98911f4dc92a85c85a0327697e424810c46a379fff5a`)
and [Regtest bytes](../../packages/testdata/phase167/regtest-history.json)
(SHA-256 `1104ddd93f61b91ed0e8d531dc7e2df347162b473fa9936fa47668623edd3377`)
are bounded, source-derived inputs. Their
[rule expectations](fixtures/phase167-rule-expectations.json)
(SHA-256 `4dddecfbb11a93eac11474d23f80bfff08736713a02941f7f21feb32b7a88c24`),
[historical results](fixtures/phase167-historical-results.json)
(SHA-256 `5948da8d8c5776e9bfeecd71202fe99db2125a7ca8f38d8d0b8e4f5b93700cde`) and
[Regtest expectations](fixtures/phase167-regtest-expectations.json)
(SHA-256 `c695b6bf4e3dce3f8f1d9478bb5f711ce6d35b3d5153f9c6e5d9630150b8ce07`)
retain separate evidence classes.

The other three required identities are exact seeded script-exception cases:

| Selected network | Exception purpose | Display hash |
| --- | --- | --- |
| Mainnet | P2SH flag replacement | `00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22` |
| Mainnet | Taproot flag replacement | `0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad` |
| Testnet3 | P2SH flag replacement | `00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105` |

Together with the four BIP30 originals/repeats below, these are the manifest's
seven required historical identities. None is a Mainnet or Testnet3 product
submission receipt.

| Mainnet block role | Height | Display hash |
| --- | ---: | --- |
| Original coinbase for repeat at 91,842 | 91,812 | `00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f` |
| Repeat | 91,842 | `00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec` |
| Original coinbase for repeat at 91,880 | 91,722 | `00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e` |
| Repeat | 91,880 | `00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721` |

BIP30 scans the full pre-block UTXO view before any transaction can spend a
collision. Only those exact Mainnet repeat pairs or a matching selected BIP34
ancestor can skip the scan, and checking resumes at height 1,983,702. An
arbitrary duplicate, changed old coin or wrong network/height/hash refuses.
The [Plan 16 historical tests](../../.planning/phases/167-candidate-history-validation-rules/167-16-SUMMARY.md)
record the source-bound seeded cases; they do not activate ordinary Mainnet.

**Intentional difference:** pinned Knots' normal disconnect does not restore an
overwritten old coinbase. Open Bitcoin's
[pre-block capture and restoration](../../packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs)
records the complete prior `Coin`—output, coinbase status, creation height and
creation MTP—plus exact outpoint, transaction index, block/height binding and
integrity. Its [durable codec](../../packages/open-bitcoin-node/src/storage/snapshot_codec/overwrite_undo.rs)
preserves those fields so disconnect and retained reopen restore the old coin
losslessly. This stronger CVLD-06 contract is explicit parity difference, not
an assertion that Knots itself restores the old coin.

## Bounded local default-Regtest route and limits

The [actual Plan 14 RPC/CLI process test](../../packages/open-bitcoin-cli/src/operator/runtime/support/phase167_tests.rs)
accepted 301 descendants after proved genesis. After all writer handles
closed, parsed offline `sync status` reopened the same physical default-Regtest
datadir and matched the RPC accepted height/hash, exact work and retained BASIC
endpoints. The [complete retained-history proof](../../packages/open-bitcoin-node/src/storage/canonical_history/proof.rs)
checks the copied and original store before the
[configured runtime](../../packages/open-bitcoin-node/src/sync/canonical_history.rs)
consumes it. A raw persisted `ScriptsValid` label, seeded envelope or copied
scalar parameters cannot mint that owner. Wrong selectors, incomplete retained
bodies/undo, unproved stores and a concurrent second owner refuse before an
accepted history change.

The reviewed finite history envelope is 306 node/header positions: actual
proof is recorded at 305 known/303 active, and 306 known/304 active is the
reviewed boundary, **not** a measured performance maximum. A complete 307
position candidate refuses. This is local bounded behavior, not an arbitrary
whole-chain replay claim. [Plan 11 measurements](../../.planning/phases/167-candidate-history-validation-rules/167-11-SUMMARY.md#actual-bounded-resource-evidence)
retain the logical/physical byte, row, copy and time observations separately.

For operator review, use a disposable, already proved and closed
**default-Regtest** datadir built by the linked actual test or the
[bounded genesis procedure](v2-6-canonical-durable-bootstrap.md#operator-reproduction)
followed by independently reviewed local descendant admission. Set
`phase167_review_dir` to its absolute parent directory. Run either repo-local
status form after the daemon and every writer handle have exited:

```bash
bun run scripts/command-timings.ts run --key phase167-docs-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase167_review_dir/open-bitcoin.jsonc" --datadir "$phase167_review_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase167-docs-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$phase167_review_dir/open-bitcoin.jsonc" --datadir "$phase167_review_dir/data" --network regtest --format json sync status
```

The offline status command is existing-only; an admitted Fjall open can have
filesystem effects, while pre-admission incompatible-source refusal preserves
the source within its finite checks. Networking remains default off. Mainnet,
testnet3 and Signet ordinary configuration remain refused; default Signet also
lacks challenge validation. The generic `require_ordinary_history()` gate
remains false for every selector. Actual pinned-Knots daemon differential,
whole-chain, public-default networking, unattended full sync,
production-service and production-funds qualification remain deferred. The
canonical RPC context carries `WalletState::Unavailable`; no wallet authority
is claimed. Phase 168 owns actual pinned-daemon differential work.

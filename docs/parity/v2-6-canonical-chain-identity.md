# Canonical Chain Identity and Compatibility Gates

Surface id: `v2-6-canonical-chain-identity`. Requirements addressed: CBOT-01,
CBOT-04 and CVLD-05. Phase 164 implements identity-only recognition and ordinary
refusal. [Phase verification](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-VERIFICATION.md) passed 27/27 after full default native acceptance, clean source review and 30 closed security controls. The three requirements are closed for this identity-only/refusal scope; ordinary history remains unsupported.

## Source pin and evidence class

The baseline is Bitcoin Knots `29.3.knots20260210`, pinned at
`a9aee730466ac67d35a3c03ee24676be5e045878`. Exact source anchors are
[`CreateGenesisBlock`, lines 40–77](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp#L40-L77),
the [mainnet assertions, lines 137–140](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp#L137-L140),
[testnet3, lines 295–298](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp#L295-L298),
[Signet profile and genesis, lines 455–532](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp#L455-L532),
and [regtest, lines 633–636](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp#L633-L636).
The pinned [genesis connection special case](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp#L2733-L2740)
and [Signet solution validation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/signet.cpp#L125-L150)
remain distinct from recognizing bytes.

The evidence class is **identity-only, source-derived vectors**. The
[independent serialization research](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-RESEARCH.md#exact-canonical-genesis-and-test-vectors)
froze the expected headers, common coinbase, hashes and whole-body digests from
the pinned source. [Plan 01](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-01-SUMMARY.md)
records passing exact-vector and adversarial identity tests. No actual Knots
daemon qualification, canonical accepted history, bootstrap or reopen follows
from these comparisons. No production dependency or crate was added.

## Exact identities and current support

All four genesis blocks have version 1, zero previous hash and the same single
204-byte non-witness coinbase transaction, paying 5,000,000,000 satoshis. Its txid
and header merkle root in display order are
`4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b`.
Each serialized header is 80 bytes and complete block is 285 bytes. Predicates
compare every header field and the complete transaction body, including witness
metadata; shared transaction bytes, a null parent or easy valid PoW do not prove
the selected identity.

| Existing CLI selector | Canonical identity |       Time |      Nonce | Bits         | Display genesis hash                                               | Ordinary history |
| --------------------- | ------------------ | ---------: | ---------: | ------------ | ------------------------------------------------------------------ | ---------------- |
| `mainnet`             | mainnet            | 1231006505 | 2083236893 | `0x1d00ffff` | `000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f` | Refused          |
| `testnet`             | testnet3           | 1296688602 |  414098458 | `0x1d00ffff` | `000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943` | Refused          |
| `signet`              | default Signet     | 1598918400 |   52613770 | `0x1e0377ae` | `00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6` | Refused          |
| `regtest`             | default regtest    | 1296688602 |          2 | `0x207fffff` | `0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206` | Refused          |

`testnet` retains its existing parser spelling and means **testnet3**; `testnet3`
is the diagnostic identity, not a new CLI value. Testnet4 and custom Signet are
excluded. The default Signet challenge is
`512103ad5e0edad18cb1f0fc0d28a3d4f1f3e445640337489abb10404f2d1e086be430210359ef5021964fe22d6f8e05b2463c9540ce96883fe3b278760f048f5189f2e6c452ae`.
Signet's shared genesis does not prove that challenge or a valid block solution.

Every profile exposes `scope=identity_only` and `ordinary_history=false`, with
missing `canonical_bootstrap`, `exact_work` and `candidate_history`. Signet also
requires `signet_challenge_validation`. Phase 165 owns exact work, Phase 166
canonical durable bootstrap/reopen, and Phase 167 candidate-history validation.
The [Phase 165 work foundation](v2-6-shared-exact-work.md) now supplies exact
numeric carriers and earned live-owner checks. The ordinary profile still reports
`exact_work` missing: no canonical bootstrap or recovered work authority follows
from that foundation or its fixture evidence.
Until implemented and verified, ordinary headers, bodies, stored/local connects,
snapshot import, compact dispatch, reorg and startup refuse before acceptance.
An exact genesis match grants no accepted tip, spendable genesis outputs or
ordinary capability. A raw store, supplied owner, caller-selected work or
`Complete` fixture coverage cannot upgrade it.

## Intentional current product difference

Unlike pinned Knots' canonical startup and validation, **every ordinary Open
Bitcoin history mode currently refuses**. Every existing nonempty unproved
store refuses without backend open; missing and empty paths also refuse because
canonical bootstrap/work/context prerequisites are absent. This is an
intentional fail-closed product difference recorded in [the parity index](index.json).
It is not wrong-network detection, positive canonical reopen compatibility or
operational bootstrap. Even canonical-looking files cannot prove stored identity.

The [filesystem-only gate](../../packages/open-bitcoin-node/src/storage/chain_compatibility.rs)
inspects ancestor shape and at most one directory entry. It never opens Fjall,
reads source-file contents, creates a backend, migrates schema/coins, initializes
history, resumes prune intent or publishes BASIC records. Refusal evidence
compares physical names, types, bytes, lengths, permissions, modification times
and symlink targets; access times are excluded. This is software preservation
evidence, not a hardware crash guarantee.

| Observation                                                   | Fixed reason                   | Stored network/root/work |
| ------------------------------------------------------------- | ------------------------------ | ------------------------ |
| `missing` or `empty_directory`                                | `unsupported_validation_scope` | Unavailable              |
| `existing_unproved`                                           | `existing_state_unproved`      | Unavailable              |
| `unsafe_path` (including a symlink or non-directory ancestor) | `unsafe_path`                  | Unavailable              |
| `permission_denied` or `inspection_io_other`                  | `inspection_io_failure`        | Unavailable              |

Diagnostics expose `selected_network`, `selected_genesis`, `observation`,
`observed_network=unavailable`, `observed_root=unavailable`,
`observed_work=unavailable`, scope, ordinary-history refusal and missing
prerequisite codes. The selected hash is expected identity, never observed
stored identity. Fixed diagnostic text stays below 1,024 UTF-8 bytes and excludes
caller paths, file contents and backend error text.

Phase 165 appends `work_decision` after the fixed guidance: missing/empty sources
use `bootstrap_unavailable`, existing unproved sources use `existing_unproved`,
unsafe paths use `unsafe_path`, and inspection I/O failures use
`inspection_io_failure`. Observed work stays unavailable. The
[work evidence page](v2-6-shared-exact-work.md#legacy-representation-and-non-opening-refusal)
separates actual local Open Bitcoin refusal processes from deferred pinned-Knots
daemon qualification; it does not grant ordinary history support.

Ordinary daemon and metrics startup, RPC context/fallback construction, offline
CLI sync status/pause/resume, managed-wallet name lookup and backup use these
gates. CLI JSON refusal uses `chain_compatibility_refused`; human output retains
the fixed reason. Soak resource enrichment reports bounded
`chain_compatibility_unavailable`; soak start refuses before any run index,
ledger or report write. Existing separately authenticated **live RPC**
status/index/send/control paths retain separate ownership and do not open local
storage. Their success never authorizes a local store or bootstrap.

Preserve existing data and review it separately. Do not relabel an unavailable
identity as a wrong network. Use a separate datadir only after the missing
validation prerequisites are implemented and verified; a new directory currently
refuses too. No repair, conversion, import or destructive reindex is attempted or
recommended as a bypass.

## Historical evidence remains bounded

The [v2.5 integrated proof](v2-5-integrated-filter-proof.md) and
[archive-safe provenance](v2-5-archive-safe-provenance.md) retain their original
source IDs, receipt bytes, hashes, dates, counts, labels and outcomes. Their
synthetic roots, caller-selected work and original maturity limits remain
explicit: the long fixture uses normal maturity 100, smaller fixtures may use
custom maturity, and Open Bitcoin's inherited halving interval 210000 differs
from pinned Knots regtest 150. Those receipts do not establish canonical
whole-chain differential parity or most-work authority.

Current named synthetic fixture factories preserve that narrower regression
scope. [Plan 13](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-13-SUMMARY.md)
records actual coupled GREEN and the integrated historical lifecycle regression;
[Plan 06](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-06-SUMMARY.md)
records actual CLI refusal/preservation and separate live-RPC tests. Fixture
success cannot enable an ordinary config/environment path. Phase 164 adds no
new full-node, whole-chain, mainnet sync, public-default networking, production
service/readiness or production-funds claim. Actual Knots differential
qualification remains Phase 168-owned.

## Repo-local UAT: expected outcomes

These are reproduction instructions, not execution receipts. Run from the repo
root. Help lists `--network mainnet|testnet|signet|regtest`, `--datadir`,
`--config`, `--format human|json`, and `sync status|pause|resume`.

```bash
bun run scripts/command-timings.ts run --key phase164-uat-cargo-help -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --help
bun run scripts/command-timings.ts run --key phase164-uat-bazel-help -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --help
```

Create owned temporary roots. Canonicalizing the parent avoids macOS's `/tmp`
symlink; do not substitute a user datadir or existing service configuration.
Leave `missing` absent. The explicit absent config path avoids host JSONC input;
these sources have no RPC credentials and activate no public networking.

```bash
phase164_root="$(mktemp -d)"
phase164_root="$(cd "$phase164_root" && pwd -P)"
mkdir "$phase164_root/empty"
```

Status may report a stopped/unavailable node without opening storage. That
observation does not establish usable history. Offline sync controls must fail
with the selected identity and unavailable stored network/root/work:

```bash
bun run scripts/command-timings.ts run --key phase164-uat-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase164_root/absent.jsonc" --datadir "$phase164_root/empty" --network regtest --format json status
bun run scripts/command-timings.ts run --key phase164-uat-cargo-refusal -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase164_root/absent.jsonc" --datadir "$phase164_root/empty" --network testnet --format json sync status
bun run scripts/command-timings.ts run --key phase164-uat-bazel-refusal -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$phase164_root/absent.jsonc" --datadir "$phase164_root/missing" --network signet --format json sync pause
bun run scripts/command-timings.ts run --key phase164-uat-cargo-resume-refusal -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase164_root/absent.jsonc" --datadir "$phase164_root/empty" --network mainnet sync resume
```

Expected refusal: nonzero exit, `chain_compatibility_refused` in JSON error output,
`unsupported_validation_scope`, `observation=empty_directory` or `missing`,
`ordinary_history=false`, and the missing prerequisites. Testnet reports
`selected_network=testnet3`; Signet includes `signet_challenge_validation`.
Both paths retain their original shape. No files are created inside `empty`,
and `missing` stays absent. Expected command failure is the gate under review;
do not treat it as a successful sync, workaround or command to repeat against
valuable data. This example does not create a synthetic backend or prove an
existing backend's identity. Plan 02's closed-store tests supply that separate
non-opening preservation evidence.

## Actual documentation validation

Plan 07 inspected the already-built repo binary's root, status, sync and soak
start help. It executed 24 actual JSON offline sync status/pause/resume attempts
across four selectors and owned missing/empty roots: every attempt refused,
the empty directory stayed empty and the missing path stayed absent. These
direct-binary checks did not run the Cargo/Bazel UAT commands above, open a
backend or qualify a Knots daemon. Scoped catalog/reference checks passed and
historical inventory entries remained unchanged. The
[Plan 07 summary](../../.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-07-SUMMARY.md)
records the exact check scope and existing historical-checker reconciliation
still owned by Plan 08 and full phase verification.

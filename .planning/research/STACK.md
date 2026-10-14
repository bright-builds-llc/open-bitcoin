# Stack Research

**Domain:** Open Bitcoin v2.6 Canonical Chain Bootstrap and Validation Parity
**Researched:** 2026-10-13
**Confidence:** HIGH for inspected pins/source; MEDIUM for proposed integration and qualification design
**Baseline:** Bitcoin Knots `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`

## Recommendation

Add **no production dependency and no new workspace crate**. Promote existing first-party `ProofWork` into every work authority, add canonical chain specifications and versioned durable authority records inside existing crates, and build a test-only differential runner around an actual source-built pinned Knots executable.

This is research, not executed oracle/build/runtime evidence. The accepted scope is genuine default-regtest validation, bounded classified mainnet historical contexts, and explicit refusal for existing unsupported validation modes. Current `testnet` identifies testnet3; neither testnet4 nor complete/custom Signet validation is added implicitly. Retain one active Fjall chainstate and explicit/default-off public networking.

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, and architecture, verification, testing, Rust and TypeScript standards. The authorized new-milestone workflow supplies planning context. Local pinned sources take precedence over newer upstream documentation; no library discovery is needed because the recommended dependency delta is zero.

## Recommended Stack

### Core Technologies

| Technology                  | Version / exact source                               | Purpose                                                              | Why Recommended                                                                                      |
| --------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Rust / edition              | 1.94.1 / 2024                                        | Canonical identity, rules, work and validation                       | Existing `rust-toolchain.toml`, Cargo and Bazel agree; keep pure domain decisions in Rust.           |
| Existing first-party crates | Workspace 0.1.0                                      | Primitives, codec, consensus, network, chainstate, node and adapters | Required seams already exist; expand modules rather than introducing parallel implementations.       |
| Fjall                       | 3.1.10 at `aa30dca811399a201e0b9595da93a4582dcb2b57` | Single durable chainstate, bodies/undo and BASIC                     | Preserve current owner, coins fences and recovery semantics; no storage migration to another engine. |
| Bun / TypeScript            | 1.3.9                                                | Process orchestration, manifests, bounds and receipts                | Canonical repository automation runtime; no `package.json` or `bun install` bootstrap exists.        |
| Bazel / rules_rust          | 8.6.0 / 0.69.0                                       | First-party Bzlmod smoke builds and provenance                       | Retain `.bazelversion`, `MODULE.bazel` and Cargo.lock-derived crate universe.                        |
| Bitcoin Knots daemon        | Exact baseline above                                 | Independent ordinary-validation oracle                               | Test-only source build, never a runtime/production dependency or replacement domain model.           |

### Supporting Libraries and Existing Facilities

| Facility                  | Existing version / location                            | Use in v2.6                                                                                                  |
| ------------------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
| `ProofWork([u64; 4])`     | `consensus/src/chain_proof.rs`                         | Reuse exact proof, comparison and arithmetic; add lossless encoding/display and authority integration.       |
| First-party codecs        | `open-bitcoin-codec`                                   | Canonical genesis/block/transaction bytes and bounded decoding; avoid production Bitcoin libraries.          |
| serde / serde_json        | 1.0.228 / 1.0.149                                      | Versioned durable DTOs and evidence manifests; work must not travel through JavaScript `number`.             |
| Existing RPC harness      | `open-bitcoin-test-harness`; ureq 3.3.0, base64 0.22.1 | Reuse target/case/report abstractions if they fit; add explicit bounds rather than a second HTTP dependency. |
| Existing cryptography     | secp256k1 0.31.1 locked                                | Keep current signature surface; this milestone does not justify replacing it.                                |
| Python BASIC helper       | `scripts/basic-filter-oracle.py`                       | Retain its documented narrow Knots-helper compatibility exception; protocol/arithmetic evidence only.        |
| Timing/build coordination | `scripts/command-timings.ts`, `command-timing-lock.ts` | Stable keys, checkout-scoped cooperative Cargo/Bazel lock and 60-second heartbeat evidence.                  |

### Required First-Party Changes

- **Consensus:** immutable selected-chain parameters, canonical genesis data and branch-derived contextual rules/script flags. Keep clocks and ancestry loading outside pure decisions.
- **Work:** replace operative `u128` and synthetic score paths in headers, positions, candidates, managed reorg, live sync, status and recovery with one `ProofWork` representation. Width alone does not establish validated ancestry.
- **Durability:** affected snapshot DTOs currently store `u128`; store schema is 2 and legacy blobs 1/2 remain readable. Introduce explicit canonical authority/encoding versions and read-only compatibility refusal before mutable recovery. Preserve unrelated wallet/mempool readability.
- **Encoding:** specify fixed 32-byte work order and canonical RPC hexadecimal display; distinguish numeric bytes from hash display order. Validate length/identity and recompute ancestry work on recovery. Never round-trip authority through decimal JSON numbers.
- **Genesis/BASIC:** bootstrap exact retained genesis body/header and coins best-block without spendable genesis outputs; activate BASIC after durable canonical history exists.
- **Evidence:** versioned corpus, source/build/executable manifests, bounded process runner and normalization. Keep raw observations so normalization cannot erase meaningful rejection differences.

Pinned `arith_uint256` and existing `ProofWork` use modulo-`2^256` arithmetic. Compact-target overflow rejection is a separate rule. Do not silently replace wrapping work arithmetic with saturation or an extra refusal; any deliberate divergence requires a parity entry. Independently cover values above 128 bits because a small regtest chain will not naturally reach them. [S2, S4]

## Test-Only Oracle Build Toolchain

Use the pinned CMake build, outside its source tree, for only the required daemon/CLI targets. CMake is an oracle build tool; it does not replace first-party Cargo/Bazel. No package installation, configuration or build was executed for this research.

| Prerequisite             | Pinned minimum / reference                                           | Decision                                                                                        |
| ------------------------ | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| CMake                    | 3.22 minimum                                                         | Record actual qualified version and generator in the build receipt.                             |
| C++ compiler             | C++20; Clang 16.0 or GCC 11.1 minimum                                | Use host-supported compiler; on macOS follow pinned Xcode Command Line Tools guidance.          |
| Boost headers            | 1.73.0 minimum; pin's depends recipe uses 1.81.0                     | Needed by daemon/CLI even with wallet disabled.                                                 |
| libevent                 | 2.1.8 minimum; depends recipe uses 2.1.12-stable                     | Required daemon/RPC build prerequisite.                                                         |
| Build driver / discovery | Make or supported CMake generator; pkgconf in pinned platform guides | Record exact host package/tool versions; do not use floating “latest” recommendations.          |
| Python                   | 3.10 minimum for pinned scripts/tests                                | Existing helper and upstream functional framework only; not new repo automation by convenience. |

These are the requirements of the selected source, not evidence that every newer compiler/package combination works. The pinned `depends/` recipes provide source versions/checksums when stronger dependency reproducibility is needed. A host-package build is acceptable with recorded exact inputs; it is not automatically bit-for-bit reproducible. Wallet-off avoids SQLite/Berkeley DB; GUI, ZMQ, UPnP, external signer and Tor subprocess support are unnecessary for this oracle. [S5–S7]

### Proposed Provisioning Shape

The following commands are a reviewable build shape, **not executed results**. Create the designated output directory and select/record bounded job count during implementation; do not automatically fetch/install prerequisites during offline verification.

```bash
cmake -S packages/bitcoin-knots -B .local/open-bitcoin-dev/knots-v26-build \
  -DCMAKE_BUILD_TYPE=Release -DENABLE_WALLET=OFF -DBUILD_GUI=OFF \
  -DBUILD_DAEMON=ON -DBUILD_CLI=ON -DBUILD_TESTS=OFF \
  -DBUILD_TX=OFF -DBUILD_UTIL=OFF -DBUILD_BITCOINCONSENSUS_LIB=OFF \
  -DWITH_ZMQ=OFF -DWITH_MINIUPNPC=OFF \
  -DENABLE_EXTERNAL_SIGNER=OFF -DENABLE_TOR_SUBPROCESS=OFF
cmake --build .local/open-bitcoin-dev/knots-v26-build \
  --target bitcoind bitcoin-cli --parallel 2
```

The job count is a conservative example, not a measured optimal setting. Use a new/verified dedicated build directory; do not trust an unrelated cached executable. No system install is needed. A provisioning wrapper should record timings and serialize access to its CMake output directory; existing Cargo-target locking alone is not proof of CMake coordination.

## Oracle Identity, Isolation and Control

1. **Bind source to build:** verify exact submodule HEAD, expected source tree and tracked cleanliness before building; exclude untracked/shadow build inputs or record them explicitly. Capture compiler/linker/library identities, configure options, build outcome and binary SHA-256 in an immutable build receipt.
1. **Bind execution to receipt:** resolve the executable's canonical path, require its digest to match the trusted build receipt, capture `-version`, and recheck source identity as appropriate. A matching version string or current source checkout alone cannot prove that a binary was built from that source.
1. **Isolate each run:** separate temporary datadirs/configs for Knots and Open Bitcoin, explicit loopback RPC ports and run-owned credentials/cookies, controlled environment/working directory, no ambient config/wallet/datadir reuse, and deterministic candidate order.
1. **Disable external activity:** source-supported `-networkactive=0`, `-listen=0`, `-dnsseed=0`, `-fixedseeds=0`, `-discover=0`, `-listenonion=0` and `-assumevalid=0`; record effective options. Restrict RPC bind/allowlist to loopback. Record checkpoint policy separately. An OS network restriction can strengthen isolation when available; flags alone are not an OS sandbox.
1. **Use ordinary validation:** submit identical serialized candidate bytes through Knots `submitblock` and the real Open Bitcoin durable validation boundary. Use loopback RPC for control/observations; extend existing test-only adapters when a product RPC is absent instead of inventing product capability.
1. **Control time honestly:** regtest `setmocktime` is available; give Open Bitcoin the same deterministic observation via its shell boundary. This RPC is regtest-only and cannot be assumed for mainnet historical contexts.
1. **Observe transitions:** retain raw RPC results/errors and compare tip, height, exact work, selected UTXOs and BASIC commitments after accepted/rejected/fork/reopen steps. Knots may return `duplicate` or `inconclusive`; neither is silently equivalent to a new accepted block.
1. **Bound lifecycle:** explicit startup/readiness, request, case and total-run deadlines; input/output/log byte caps; block/case/branch limits; bounded readiness backoff; exit-status/signal capture; graceful stop then bounded escalation, child reaping and cleanup receipts. Timeout/truncation/missing oracle/incomplete case is failure or unavailable, never passing parity.

Existing Rust sandbox/process helpers and the bounded BASIC runner are useful starting points, not certification of these stronger guarantees. Extend/report cleanup and process outcomes explicitly; do not rely solely on best-effort `Drop` cleanup. Reserve loopback ports with collision-aware handoff/retry. Knots block submission can update uncommitted structures: save submitted bytes and verify observed block identity/serialization so “same input” remains auditable. [S3, S8–S10]

## Qualification Versus Offline Verification

| Execution class                      | Required behavior                                                                                                                                                        | What it proves                                                                                           |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| Actual pinned-oracle qualification   | Provision source-built oracle once per recorded environment; run genuine canonical corpus against both processes; retain complete receipts.                              | Independent differential result for that exact source, corpus, options and execution.                    |
| Routine `bash scripts/verify.sh`     | Run deterministic core/integration regressions and qualified-corpus/manifest checks without implicit public networking, prerequisite installation or full Knots rebuild. | Current first-party regressions against recorded expectations; distinguish replay from a new oracle run. |
| Dedicated local/CI qualification job | Use explicit preprovisioned or separately built identity-verified binary; fail when unavailable/wrong or mismatched; cache only with provenance.                         | Fresh oracle receipt; local loopback execution is compatible with excluding public-network CI.           |
| Existing BASIC Python oracle         | Continue source-pinned protocol/helper checks under current native registration.                                                                                         | BASIC/wire/arithmetic expectations within its original boundary; not whole-chain consensus execution.    |

Actual qualification is mandatory for v2.6 differential acceptance. Frozen results and helper formulas cannot substitute. Native verification does **not** need to fetch/build Knots on every invocation. Changed canonical rules, runner/normalization, oracle build inputs or corpus invalidate affected qualification receipts; required qualification must rerun before closure. Select precise CI provisioning and receipt-invalidation rules in phase planning.

Keep the full native verifier as the final first-party contract. Ad-hoc Cargo/Bazel work must retain the existing timing/lock wrapper, for example:

```bash
bun run scripts/command-timings.ts run --key v26-consensus-test -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus
bun run scripts/command-timings.ts run --key v26-cli-bazel -- bazel build //packages/open-bitcoin-cli:open_bitcoin
bash scripts/verify.sh
```

These commands were not run here. Do not overlap Cargo jobs against one target directory. Bound oracle case deadlines from measured qualification; a timing estimate or quiet 60-second polling interval is not itself a timeout. Capture heartbeat/process liveness before escalating a stalled build.

## Alternatives Considered and What NOT to Use

| Recommended                                     | Alternative / avoid                                                             | Reason                                                                                                              |
| ----------------------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Existing four-limb `ProofWork`                  | New bigint/Bitcoin production crate                                             | Existing exact arithmetic is present; integration/encoding, not missing math infrastructure, is the problem.        |
| Existing consensus/node/harness modules         | New workspace crate or daemon service                                           | No demonstrated boundary requires one; avoid parallel rule/work/process authorities.                                |
| Bun runner + Rust production validation         | New Python orchestration/domain implementation                                  | Violates canonical script boundary; existing narrow helper exception is not blanket permission.                     |
| Actual source-built daemon                      | `libbitcoinconsensus` alone                                                     | Script-library execution cannot establish bootstrap, chain selection, contextual rules or durable lifecycle parity. |
| Serialized loopback RPC control                 | Public peers, containers as a mandatory new stack, ambient installed `bitcoind` | Explicit local control is enough; add OS isolation only where justified and available.                              |
| Versioned lossless records + compatibility gate | In-place widening of legacy numbers                                             | Legacy synthetic roots/work/context remain unproven; changing width cannot confer provenance.                       |
| Default-regtest ordinary corpus                 | Activation/maturity overrides or sparse mainnet claimed as full validation      | Changes evidence scope; bounded mainnet fixtures must declare ancestry/UTXO/deployment prerequisites.               |

## Version Compatibility and Open Prerequisites

| Boundary          | Required compatibility decision                                                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Rust/Cargo/Bazel  | Keep Rust 1.94.1 in all paths; preserve Cargo.lock/Bzlmod synchronization when source changes require regeneration.                              |
| Work/durable DTOs | Define endianness, display, authority envelope and incompatible legacy refusal together; avoid mixed-width deployed authority.                   |
| Oracle/platform   | Exact CMake/compiler/Boost/libevent versions, build output and binary digest remain to be qualified; no host build success was measured.         |
| Python            | Reuse current documented exception; any additional upstream-helper dependence needs a concrete compatibility reason and reviewed local guidance. |
| Prune corpus      | Size canonical history for pinned regtest eligibility/keep-window rules and bounded runtime, rather than recycling maturity-one proof.           |
| Network coverage  | Regtest ordinary proof; classified mainnet contexts; early explicit refusal where signet/testnet3 consensus prerequisites remain unsupported.    |

## Sources

- **S1 — HIGH:** Local `rust-toolchain.toml`, `.bun-version`, `.bazelversion`, `MODULE.bazel`, `packages/Cargo.toml`, `packages/Cargo.lock` and first-party Cargo manifests; version pins inspected.
- **S2 — HIGH:** Local `consensus/src/chain_proof.rs`, `network/src/header_store.rs`, node `storage.rs`/`storage/snapshot_codec.rs`; existing exact math, operative `u128` and schema facts.
- **S3 — HIGH:** Local `open-bitcoin-test-harness/src/{target,isolation}.rs`, `scripts/phase162-filter-oracle.ts`, `scripts/basic-filter-oracle.py`, timing/lock scripts.
- **S4 — HIGH:** [Pinned chain proof](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp) and [integer arithmetic](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/arith_uint256.h).
- **S5 — HIGH:** [Pinned dependency requirements](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/doc/dependencies.md), accessed 2026-10-13; local contents inspected.
- **S6 — HIGH:** [Pinned CMake options](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/CMakeLists.txt) and [Boost discovery](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/cmake/module/AddBoostIfNeeded.cmake).
- **S7 — HIGH:** [Pinned Unix guide](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/doc/build-unix.md), [macOS guide](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/doc/build-osx.md) and [depends recipes](https://github.com/bitcoinknots/bitcoin/tree/a9aee730466ac67d35a3c03ee24676be5e045878/depends/packages).
- **S8 — HIGH:** [Pinned initialization/options](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp), networking/assumevalid/RPC binding.
- **S9 — HIGH:** [Pinned mining RPC](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/mining.cpp), `submitblock` processing/results.
- **S10 — HIGH:** [Pinned node RPC](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/node.cpp) and [blockchain RPC](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/blockchain.cpp), regtest clock, work and BASIC observations.
- **MEDIUM:** Build/runner/encoding and qualification workflow are recommendations requiring implementation/measurement. No build, test, oracle execution, Git mutation or capability acceptance occurred in this research.

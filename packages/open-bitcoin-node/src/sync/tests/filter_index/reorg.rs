// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Production validated spending forks, immutable history and actual durable reopen.

use super::{catch_up::fixtures::TurnHistory, recovery::ValidatedHistory, *};
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::{
    chainstate::{AnchoredBlock, Chainstate, FlushMode, FlushPolicyTime},
    consensus::{ScriptVerifyFlags, transaction_txid},
};
use open_bitcoin_mempool::ReorgLifecycleContext;

mod branches;
mod failures;
pub(super) mod fixtures;
mod measurements;
mod protection;
mod retention;

use fixtures::{ForkBranch, ForkFixture};

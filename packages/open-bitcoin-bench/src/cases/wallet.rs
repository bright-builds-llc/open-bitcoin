// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/wallet_balance.cpp
// - packages/bitcoin-knots/src/bench/coin_selection.cpp
// - packages/bitcoin-knots/src/bench/wallet_create_tx.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp

use open_bitcoin_wallet::Wallet;

use crate::{
    error::BenchError,
    fixtures::BenchFixtures,
    registry::{BenchCase, BenchGroupId, WALLET_MAPPING},
};

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: "wallet.balance-selection-signing",
    group: BenchGroupId::Wallet,
    description: "Rescan wallet state, calculate balances, select coins, build, and sign a transaction.",
    knots_mapping: &WALLET_MAPPING,
    run_once: run_wallet_case,
}];

fn run_wallet_case() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let mut wallet = Wallet::from_snapshot(fixtures.wallet.wallet_snapshot.clone());
    wallet
        .rescan_chainstate(&fixtures.wallet.chainstate_snapshot)
        .map_err(|error| {
            BenchError::case_failed("wallet.balance-selection-signing", error.to_string())
        })?;

    let balance = wallet.balance(100).map_err(|error| {
        BenchError::case_failed("wallet.balance-selection-signing", error.to_string())
    })?;
    if balance.spendable.to_sats() <= 0 {
        return Err(BenchError::case_failed(
            "wallet.balance-selection-signing",
            "wallet fixture did not produce spendable balance",
        ));
    }

    let built = wallet
        .build_and_sign(&fixtures.wallet.build_request, 100)
        .map_err(|error| {
            BenchError::case_failed("wallet.balance-selection-signing", error.to_string())
        })?;
    if built.transaction.inputs.is_empty() || built.selected_inputs.is_empty() {
        return Err(BenchError::case_failed(
            "wallet.balance-selection-signing",
            "wallet build did not select inputs",
        ));
    }

    let input_contexts = wallet.input_contexts_for(&built).map_err(|error| {
        BenchError::case_failed("wallet.balance-selection-signing", error.to_string())
    })?;
    if input_contexts.len() != built.transaction.inputs.len() {
        return Err(BenchError::case_failed(
            "wallet.balance-selection-signing",
            "wallet input context count does not match signed transaction",
        ));
    }

    Ok(())
}

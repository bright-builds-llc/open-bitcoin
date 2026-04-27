// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use open_bitcoin_chainstate::ChainstateSnapshot;

use super::{
    Wallet, WalletBalance, WalletRescanState, WalletUtxo, amount_from_sats, compare_wallet_utxos,
};
use crate::WalletError;

pub(super) fn rescan_chainstate(
    wallet: &mut Wallet,
    snapshot: &ChainstateSnapshot,
) -> Result<(), WalletError> {
    let descriptor_scripts = wallet
        .descriptors
        .iter()
        .map(|record| Ok((record.id, record.descriptor.script_pubkey()?)))
        .collect::<Result<Vec<_>, WalletError>>()?;

    let mut utxos = Vec::new();
    for (outpoint, coin) in &snapshot.utxos {
        let maybe_descriptor_id = descriptor_scripts.iter().find_map(|(descriptor_id, _)| {
            wallet.descriptor(*descriptor_id).and_then(|record| {
                record
                    .descriptor
                    .matching_index(&coin.output.script_pubkey)
                    .ok()
                    .flatten()
                    .map(|_| *descriptor_id)
            })
        });
        let Some(descriptor_id) = maybe_descriptor_id else {
            continue;
        };
        utxos.push(WalletUtxo {
            descriptor_id,
            outpoint: outpoint.clone(),
            output: coin.output.clone(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
            is_coinbase: coin.is_coinbase,
        });
    }

    utxos.sort_by(compare_wallet_utxos);
    wallet.utxos = utxos;
    wallet.maybe_tip_height = snapshot.tip().map(|tip| tip.height);
    wallet.maybe_tip_median_time_past = snapshot.tip().map(|tip| tip.median_time_past);
    Ok(())
}

pub(super) fn balance(
    wallet: &Wallet,
    coinbase_maturity: u32,
) -> Result<WalletBalance, WalletError> {
    let spend_height = spend_height(wallet);
    let mut total_sats = 0_i64;
    let mut spendable_sats = 0_i64;
    let mut immature_sats = 0_i64;

    for utxo in &wallet.utxos {
        let value = utxo.output.value.to_sats();
        total_sats += value;
        if utxo.is_coinbase && spend_height < utxo.created_height.saturating_add(coinbase_maturity)
        {
            immature_sats += value;
        } else {
            spendable_sats += value;
        }
    }

    Ok(WalletBalance {
        total: amount_from_sats(total_sats)?,
        spendable: amount_from_sats(spendable_sats)?,
        immature: amount_from_sats(immature_sats)?,
    })
}

pub(super) fn is_spendable(wallet: &Wallet, utxo: &WalletUtxo, coinbase_maturity: u32) -> bool {
    let Some(descriptor) = wallet.descriptor(utxo.descriptor_id) else {
        return false;
    };
    if !descriptor.descriptor.can_sign() {
        return false;
    }
    if utxo.is_coinbase
        && spend_height(wallet) < utxo.created_height.saturating_add(coinbase_maturity)
    {
        return false;
    }

    true
}

pub(super) fn spend_height(wallet: &Wallet) -> u32 {
    wallet
        .maybe_tip_height
        .map_or(0, |height| height.saturating_add(1))
}

pub(super) fn rescan_state_from_progress(
    maybe_scanned_through_height: Option<u32>,
    maybe_target_height: Option<u32>,
    maybe_next_height: Option<u32>,
    is_scanning: bool,
) -> Result<WalletRescanState, WalletError> {
    let target_height = maybe_target_height.unwrap_or_default();
    if is_scanning {
        return Ok(WalletRescanState::Scanning {
            next_height: maybe_next_height.unwrap_or_default(),
            target_height,
        });
    }

    let scanned_through_height = maybe_scanned_through_height.unwrap_or_default();
    if scanned_through_height < target_height {
        return Ok(WalletRescanState::Partial {
            scanned_through_height,
            target_height,
        });
    }

    Ok(WalletRescanState::Fresh {
        scanned_through_height,
        target_height,
    })
}

// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::collections::HashMap;

use open_bitcoin_core::{chainstate::ChainPosition, primitives::Txid};
use serde::{Deserialize, Serialize};

use super::{ChainPositionDto, ConfirmedTxidCountDto, decode_versioned, encode_versioned};
use crate::{StorageError, StorageNamespace};

pub(crate) type HydratedChainMeta = (Vec<ChainPosition>, Option<HashMap<Txid, u32>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ChainMetaDto {
    active_chain: Vec<ChainPositionDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    maybe_confirmed_txid_counts: Option<Vec<ConfirmedTxidCountDto>>,
}

pub(crate) fn encode_chain_meta(
    active_chain: &[ChainPosition],
    maybe_confirmed_txid_counts: Option<&HashMap<Txid, u32>>,
) -> Result<Vec<u8>, StorageError> {
    let maybe_confirmed_txid_counts = maybe_confirmed_txid_counts.map(|counts| {
        let mut encoded = counts
            .iter()
            .map(|(txid, active_chain_occurrences)| ConfirmedTxidCountDto {
                txid: txid.to_byte_array(),
                active_chain_occurrences: *active_chain_occurrences,
            })
            .collect::<Vec<_>>();
        encoded.sort_unstable_by_key(|entry| entry.txid);
        encoded
    });
    encode_versioned(
        StorageNamespace::Chainstate,
        &ChainMetaDto {
            active_chain: active_chain.iter().map(ChainPositionDto::from).collect(),
            maybe_confirmed_txid_counts,
        },
    )
}

pub(crate) fn decode_chain_meta(bytes: &[u8]) -> Result<HydratedChainMeta, StorageError> {
    let dto: ChainMetaDto = decode_versioned(StorageNamespace::Chainstate, bytes)?;
    let active_chain = dto
        .active_chain
        .into_iter()
        .map(ChainPosition::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let maybe_confirmed_txid_counts = dto
        .maybe_confirmed_txid_counts
        .map(|encoded| {
            let mut counts = HashMap::with_capacity(encoded.len());
            for entry in encoded {
                if entry.active_chain_occurrences == 0 {
                    return Err(super::corruption(
                        StorageNamespace::Chainstate,
                        "confirmed transaction occurrence count must be non-zero",
                    ));
                }
                let txid = Txid::from_byte_array(entry.txid);
                if counts
                    .insert(txid, entry.active_chain_occurrences)
                    .is_some()
                {
                    return Err(super::corruption(
                        StorageNamespace::Chainstate,
                        "duplicate confirmed transaction count",
                    ));
                }
            }
            Ok(counts)
        })
        .transpose()?;
    Ok((active_chain, maybe_confirmed_txid_counts))
}

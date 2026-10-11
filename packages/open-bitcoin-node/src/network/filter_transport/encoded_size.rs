// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/net.cpp

//! Borrowed exact sizes, before allocation or serializer entry.

use super::FilterTransportError as Error;
use open_bitcoin_core::primitives::{Block, Transaction};
use open_bitcoin_network::WireNetworkMessage as Message;

/// Explicit local outbound policy; inbound's existing 1MiB queue stays independent.
pub(super) const MAX_GENERIC_PAYLOAD: u64 = 4_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct WireSize {
    pub(super) payload: u64,
    pub(super) frame: u64,
    pub(super) peak: u64,
}

fn compact(value: u64) -> Result<u64, Error> {
    if value > open_bitcoin_codec::MAX_SIZE {
        return Err(Error::CompactSizeTooLarge);
    }
    Ok(match value {
        0..=252 => 1,
        253..=65_535 => 3,
        _ => 5,
    })
}

fn add(left: u64, right: u64) -> Result<u64, Error> {
    left.checked_add(right).ok_or(Error::Exhausted)
}

fn mul(left: u64, right: u64) -> Result<u64, Error> {
    left.checked_mul(right).ok_or(Error::Exhausted)
}

fn bounded(value: u64) -> Result<u64, Error> {
    if value > MAX_GENERIC_PAYLOAD {
        return Err(Error::LocalPayloadLimit);
    }
    Ok(value)
}

fn grow(total: &mut u64, bytes: u64) -> Result<(), Error> {
    *total = bounded(add(*total, bytes)?)?;
    Ok(())
}

fn vector(fixed: u64, count: usize, width: u64) -> Result<u64, Error> {
    bounded(add(
        add(fixed, compact(count as u64)?)?,
        mul(count as u64, width)?,
    )?)
}

fn blob(bytes: &[u8]) -> Result<u64, Error> {
    bounded(add(compact(bytes.len() as u64)?, bytes.len() as u64)?)
}

fn transaction_size(tx: &Transaction) -> Result<u64, Error> {
    let mut size = add(
        8,
        add(
            compact(tx.inputs.len() as u64)?,
            compact(tx.outputs.len() as u64)?,
        )?,
    )?;
    // Reject excessive vectors before even the serializer's witness-presence scan.
    bounded(add(
        size,
        add(
            mul(tx.inputs.len() as u64, 41)?,
            mul(tx.outputs.len() as u64, 9)?,
        )?,
    )?)?;
    let mut witness_size = 0;
    let mut has_witness = false;
    for input in &tx.inputs {
        grow(&mut size, add(40, blob(input.script_sig.as_bytes())?)?)?;
        let stack = input.witness.stack();
        has_witness |= !stack.is_empty();
        grow(&mut witness_size, compact(stack.len() as u64)?)?;
        bounded(add(witness_size, stack.len() as u64)?)?;
        for item in stack {
            grow(&mut witness_size, blob(item)?)?;
        }
    }
    for output in &tx.outputs {
        grow(&mut size, add(8, blob(output.script_pubkey.as_bytes())?)?)?;
    }
    if has_witness {
        grow(&mut size, add(2, witness_size)?)?;
    }
    Ok(size)
}

fn transactions_size(fixed: u64, transactions: &[Transaction]) -> Result<(u64, u64), Error> {
    let mut size = vector(fixed, transactions.len(), 0)?;
    bounded(add(size, mul(transactions.len() as u64, 10)?)?)?;
    let mut peak = size;
    for tx in transactions {
        let tx_size = transaction_size(tx)?;
        // The nested encoder Vec survives its append into the parent payload.
        peak = peak.max(add(size, mul(2, tx_size)?)?);
        grow(&mut size, tx_size)?;
    }
    Ok((size, peak))
}

fn differential_position(shift: &mut u64, delta: u64) -> Result<u64, Error> {
    compact(delta)?;
    let position = add(*shift, delta)?;
    if position > u64::from(u16::MAX) {
        return Err(Error::InvalidMessage);
    }
    *shift = add(position, 1)?;
    Ok(position)
}

fn compact_block_size(
    block: &open_bitcoin_codec::CompactBlockPayload,
) -> Result<(u64, u64), Error> {
    let short = block.short_ids.len() as u64;
    let prefilled = block.prefilled_transactions.len() as u64;
    let count = add(short, prefilled)?;
    if count == 0 || count > u64::from(u16::MAX) {
        return Err(Error::InvalidMessage);
    }
    let mut size = vector(88, block.short_ids.len(), 6)?;
    grow(&mut size, compact(prefilled)?)?;
    bounded(add(size, mul(prefilled, 11)?)?)?;
    // Existing structure validation owns both collected u64 deltas and u16 positions.
    let mut peak = mul(prefilled, 10)?;
    let mut shift = 0;
    for (ordinal, entry) in block.prefilled_transactions.iter().enumerate() {
        let position = differential_position(&mut shift, entry.index_delta)?;
        if position > add(short, ordinal as u64)?
            || entry.transaction.inputs.is_empty()
            || entry.transaction.outputs.is_empty()
        {
            return Err(Error::InvalidMessage);
        }
        grow(&mut size, compact(entry.index_delta)?)?;
        let tx_size = transaction_size(&entry.transaction)?;
        peak = peak.max(add(size, mul(2, tx_size)?)?);
        grow(&mut size, tx_size)?;
    }
    Ok((size, peak))
}

fn index_request_size(
    request: &open_bitcoin_codec::BlockTransactionsRequest,
) -> Result<(u64, u64), Error> {
    if request.index_deltas.len() > usize::from(u16::MAX) + 1 {
        return Err(Error::InvalidMessage);
    }
    let mut size = vector(32, request.index_deltas.len(), 0)?;
    let mut shift = 0;
    for delta in &request.index_deltas {
        differential_position(&mut shift, *delta)?;
        grow(&mut size, compact(*delta)?)?;
    }
    Ok((size, mul(request.index_deltas.len() as u64, 2)?))
}

/// Exact borrowed payload and maximum simultaneous logical retained encoder bytes.
/// Hash padding is dropped before final frame construction; these peaks are not summed.
pub(super) fn wire_size(message: &Message) -> Result<WireSize, Error> {
    let (payload, encoder_peak) = match message {
        Message::MalformedCFRequest(_) => return Err(Error::InvalidMessage),
        Message::GetCFilters(_) | Message::GetCFHeaders(_) => (37, 37),
        Message::GetCFCheckpt(_) => (33, 33),
        Message::CFilter(response) => (vector(33, response.filter.len(), 1)?, 0),
        Message::CFHeaders(response) => (vector(65, response.filter_hashes.len(), 32)?, 0),
        Message::CFCheckpt(response) => (vector(33, response.filter_headers.len(), 32)?, 0),
        Message::Version(version) => (add(85, blob(version.user_agent.as_bytes())?)?, 98),
        Message::Verack | Message::WtxidRelay | Message::SendHeaders | Message::GetAddr => (0, 0),
        Message::Ping { .. } | Message::Pong { .. } => (8, 8),
        Message::SendCompact(_) => (9, 9),
        Message::GetHeaders { locator, .. } => (vector(36, locator.block_hashes.len(), 32)?, 0),
        Message::Headers(headers) => {
            let size = vector(0, headers.headers.len(), 81)?;
            (
                size,
                if headers.headers.is_empty() {
                    size
                } else {
                    add(size, 79)?
                },
            )
        }
        Message::Addr(addresses) => {
            // The existing Addr serializer's PHASE92_ADDR_BATCH_LIMIT is crate-private.
            if addresses.addresses.len() > 64 {
                return Err(Error::InvalidMessage);
            }
            let size = vector(0, addresses.addresses.len(), 30)?;
            (
                size,
                if addresses.addresses.is_empty() {
                    size
                } else {
                    add(size, 26)?
                },
            )
        }
        Message::Inv(list) | Message::GetData(list) | Message::NotFound(list) => {
            if list.inventory.len() > open_bitcoin_network::MAX_INV_SIZE {
                return Err(Error::InvalidMessage);
            }
            let size = vector(0, list.inventory.len(), 36)?;
            (
                size,
                if list.inventory.is_empty() {
                    size
                } else {
                    add(size, 36)?
                },
            )
        }
        Message::Tx(tx) => (transaction_size(tx)?, 0),
        Message::Block(Block { transactions, .. }) => transactions_size(80, transactions)?,
        Message::CompactBlock(block) => compact_block_size(block)?,
        Message::GetBlockTxn(request) => index_request_size(request)?,
        Message::BlockTxn(response) => transactions_size(32, &response.transactions)?,
    };
    let payload = bounded(payload)?;
    let frame = add(payload, 24)?;
    let padded = mul(add(payload, 72)? / 64, 64)?;
    // encode_wire's MessageHeader also retains its command String until return.
    let frame_stage = add(add(payload, frame)?, message.command_name().len() as u64)?;
    let peak = encoder_peak
        .max(add(payload, padded)?)
        .max(add(payload, 64)?)
        .max(frame_stage);
    Ok(WireSize {
        payload,
        frame,
        peak,
    })
}

#[cfg(test)]
mod tests;

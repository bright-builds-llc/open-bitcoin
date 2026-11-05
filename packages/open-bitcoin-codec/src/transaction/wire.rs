// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/transaction.h
// - packages/bitcoin-knots/src/serialize.h

use crate::{CodecError, primitives::Reader};
use open_bitcoin_primitives::{
    Amount, ScriptBuf, Transaction, TransactionInput, TransactionOutput,
};

/// Signed serialized output data, without a monetary-validity claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireOutput {
    pub value_sats: i64,
    pub script_pubkey: ScriptBuf,
}

/// Complete transaction data, without an intrinsic-validity or authority claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireTransaction {
    pub version: i32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<WireOutput>,
    pub lock_time: u32,
}

/// Borrowed output serialization facts.
pub struct OutputRef<'a> {
    pub value_sats: i64,
    pub script_pubkey: &'a ScriptBuf,
}

/// Serialization facts only; implementations confer no consensus authority.
pub trait TransactionView {
    fn version(&self) -> i32;
    fn inputs(&self) -> &[TransactionInput];
    fn outputs(&self) -> impl ExactSizeIterator<Item = OutputRef<'_>>;
    fn lock_time(&self) -> u32;
}

impl TransactionView for Transaction {
    fn version(&self) -> i32 {
        self.version
    }
    fn inputs(&self) -> &[TransactionInput] {
        &self.inputs
    }
    fn outputs(&self) -> impl ExactSizeIterator<Item = OutputRef<'_>> {
        self.outputs.iter().map(|output| OutputRef {
            value_sats: output.value.to_sats(),
            script_pubkey: &output.script_pubkey,
        })
    }
    fn lock_time(&self) -> u32 {
        self.lock_time
    }
}

impl TransactionView for WireTransaction {
    fn version(&self) -> i32 {
        self.version
    }
    fn inputs(&self) -> &[TransactionInput] {
        &self.inputs
    }
    fn outputs(&self) -> impl ExactSizeIterator<Item = OutputRef<'_>> {
        self.outputs.iter().map(|output| OutputRef {
            value_sats: output.value_sats,
            script_pubkey: &output.script_pubkey,
        })
    }
    fn lock_time(&self) -> u32 {
        self.lock_time
    }
}

impl From<Transaction> for WireTransaction {
    fn from(transaction: Transaction) -> Self {
        Self {
            version: transaction.version,
            inputs: transaction.inputs,
            outputs: transaction
                .outputs
                .into_iter()
                .map(|output| WireOutput {
                    value_sats: output.value.to_sats(),
                    script_pubkey: output.script_pubkey,
                })
                .collect(),
            lock_time: transaction.lock_time,
        }
    }
}

impl WireTransaction {
    /// Move exact scripts/witness into the domain only after checked monetary conversion.
    pub fn try_into_transaction(self) -> Result<Transaction, CodecError> {
        let outputs = self
            .outputs
            .into_iter()
            .map(|output| {
                Ok(TransactionOutput {
                    value: Amount::from_sats(output.value_sats)?,
                    script_pubkey: output.script_pubkey,
                })
            })
            .collect::<Result<_, CodecError>>()?;
        Ok(Transaction {
            version: self.version,
            inputs: self.inputs,
            outputs,
            lock_time: self.lock_time,
        })
    }
}

/// Decode one strict serialized record; storage does not imply script execution validity.
pub fn parse_serialized_transaction(bytes: &[u8]) -> Result<Transaction, CodecError> {
    let mut reader = Reader::new(bytes);
    let transaction = super::parse_transaction_mode::<super::DomainMode<true>>(&mut reader, true)?;
    reader.finish()?;
    Ok(transaction)
}

#[cfg(test)]
pub(crate) mod tests;

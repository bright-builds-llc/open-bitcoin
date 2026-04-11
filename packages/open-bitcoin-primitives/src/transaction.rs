use crate::amount::Amount;
use crate::hash::{Txid, Wtxid};
use crate::script::{ScriptBuf, ScriptWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub const NULL_INDEX: u32 = u32::MAX;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: ScriptBuf,
    pub sequence: u32,
    pub witness: ScriptWitness,
}

impl TransactionInput {
    pub const SEQUENCE_FINAL: u32 = 0xffff_ffff;
    pub const MAX_SEQUENCE_NONFINAL: u32 = Self::SEQUENCE_FINAL - 1;
    pub const SEQUENCE_LOCKTIME_DISABLE_FLAG: u32 = 1 << 31;
    pub const SEQUENCE_LOCKTIME_TYPE_FLAG: u32 = 1 << 22;
    pub const SEQUENCE_LOCKTIME_MASK: u32 = 0x0000_ffff;
    pub const SEQUENCE_LOCKTIME_GRANULARITY: i32 = 9;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionOutput {
    pub value: Amount,
    pub script_pubkey: ScriptBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn has_witness(&self) -> bool {
        self.inputs.iter().any(|input| !input.witness.is_empty())
    }

    pub fn txid(&self) -> Option<Txid> {
        self.inputs.first().map(|input| input.previous_output.txid)
    }

    pub fn wtxid(&self) -> Option<Wtxid> {
        self.txid()
            .map(|txid| Wtxid::from_byte_array(txid.to_byte_array()))
    }
}

#[cfg(test)]
mod tests {
    use crate::amount::Amount;
    use crate::hash::Txid;
    use crate::script::{ScriptBuf, ScriptWitness};

    use super::{OutPoint, Transaction, TransactionInput, TransactionOutput};

    #[test]
    fn has_witness_detects_non_empty_witness_stacks() {
        let tx = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([1_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![vec![1_u8]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        };

        assert!(tx.has_witness());
        assert_eq!(tx.txid(), Some(Txid::from_byte_array([1_u8; 32])));
        assert!(tx.wtxid().is_some());
    }
}

// Shared test-only raw recipe. No storage, runtime authority or filter expectations.
use super::core_types::{
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
        transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet},
};
use std::collections::BTreeMap;

pub const RECIPE_ID: &str = "phase162-normal-maturity-1";
pub const WALLET_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)";
pub const WALLET_SATS: i64 = 25_000;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub blocks: Vec<Block>,
    pub spent_scripts: Vec<Vec<Vec<u8>>>,
    pub wallet_outpoint: OutPoint,
    pub wallet_script: ScriptBuf,
    pub params: ConsensusParams,
    pub flags: ScriptVerifyFlags,
}

impl Recipe {
    pub fn new(params: ConsensusParams, flags: ScriptVerifyFlags, count: u32) -> Self {
        assert_eq!(params.coinbase_maturity, 100, "ordinary runtime maturity");
        assert!(
            (102..=1100).contains(&count),
            "bounded normal-maturity recipe"
        );
        let wallet_script = wallet_script();
        let mut blocks: Vec<Block> = Vec::with_capacity(count as usize);
        for height in 0..count {
            let maybe_funding = match height {
                101 => Some(&blocks[1].transactions[0]),
                102.. => blocks.last().and_then(|b| b.transactions.last()),
                _ => None,
            };
            blocks.push(child(
                blocks.last(),
                height,
                0x51,
                maybe_funding,
                &wallet_script,
            ));
        }
        let wallet_outpoint = OutPoint {
            txid: transaction_txid(&blocks[20].transactions[0]).expect("public funding txid"),
            vout: 1,
        };
        let spent_scripts = spent_scripts(&blocks);
        Self {
            blocks,
            spent_scripts,
            wallet_outpoint,
            wallet_script,
            params,
            flags,
        }
    }

    /// Alternate blocks reuse only their canonical ancestor's actual unspent outputs.
    pub fn fork(&self, ancestor: u32, count: u32, tag: u8) -> Vec<Block> {
        assert!((101..self.blocks.len() as u32).contains(&ancestor));
        assert!((1..=8).contains(&count));
        let mut branch: Vec<Block> = Vec::with_capacity(count as usize);
        for offset in 1..=count {
            let parent = branch.last().unwrap_or(&self.blocks[ancestor as usize]);
            branch.push(child(
                Some(parent),
                ancestor + offset,
                tag,
                parent.transactions.last(),
                &self.wallet_script,
            ));
        }
        branch
    }

    pub fn fork_spent_scripts(&self, ancestor: u32, branch: &[Block]) -> Vec<Vec<Vec<u8>>> {
        let mut ancestry = self.blocks[..=ancestor as usize].to_vec();
        ancestry.extend_from_slice(branch);
        spent_scripts(&ancestry).split_off(ancestor as usize + 1)
    }
}

pub fn wallet_script() -> ScriptBuf {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor("receive", DescriptorRole::External, WALLET_DESCRIPTOR)
        .expect("fixed public regtest descriptor");
    wallet
        .default_receive_address()
        .expect("public receive script")
        .script_pubkey
}

/// Resolve outpoints in transaction order, including same-block spends.
pub fn spent_scripts(blocks: &[Block]) -> Vec<Vec<Vec<u8>>> {
    let mut outputs = BTreeMap::new();
    blocks
        .iter()
        .map(|block| {
            let mut spent = Vec::new();
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    if input.previous_output == OutPoint::null() {
                        continue;
                    }
                    spent.push(
                        outputs
                            .remove(&input.previous_output)
                            .expect("preceding raw output"),
                    );
                }
                let txid = transaction_txid(transaction).expect("raw transaction identity");
                for (vout, output) in transaction.outputs.iter().enumerate() {
                    outputs.insert(
                        OutPoint {
                            txid,
                            vout: vout as u32,
                        },
                        output.script_pubkey.as_bytes().to_vec(),
                    );
                }
            }
            spent
        })
        .collect()
}

fn child(
    maybe_parent: Option<&Block>,
    height: u32,
    tag: u8,
    maybe_funding: Option<&Transaction>,
    wallet_script: &ScriptBuf,
) -> Block {
    let mut number = height.to_le_bytes().to_vec();
    while number.last() == Some(&0) {
        number.pop();
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut height_script = vec![number.len() as u8];
    height_script.extend(number);
    height_script.extend([1, tag]);
    let mut outputs = vec![TransactionOutput {
        value: Amount::from_sats(5_000_000_000 - if height == 20 { WALLET_SATS } else { 0 })
            .expect("subsidy"),
        script_pubkey: executable_tag(tag),
    }];
    if height == 20 {
        outputs.push(TransactionOutput {
            value: Amount::from_sats(WALLET_SATS).expect("small funding"),
            script_pubkey: wallet_script.clone(),
        });
    }
    let mut transactions = vec![Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(height_script).expect("canonical height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs,
        lock_time: 0,
    }];
    if let Some(funding) = maybe_funding {
        let first = spend(funding, tag);
        let second = spend(&first, tag.wrapping_add(1));
        transactions.extend([first, second]);
    }
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: maybe_parent
                .map(|b| block_hash(&b.header))
                .unwrap_or_default(),
            merkle_root: block_merkle_root(&transactions).expect("merkle").0,
            time: 1000 + height * 100 + u32::from(tag),
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded easy regtest mining");
    block
}

fn executable_tag(tag: u8) -> ScriptBuf {
    ScriptBuf::from_bytes(vec![1, tag, 0x75, 0x51]).expect("tag, drop, true")
}

fn spend(previous: &Transaction, tag: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: transaction_txid(previous).expect("funding txid"),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(previous.outputs[0].value.to_sats() - 1000)
                .expect("deduct fee"),
            script_pubkey: executable_tag(tag),
        }],
        lock_time: 0,
    }
}

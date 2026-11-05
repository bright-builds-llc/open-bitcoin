// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Store-bound recovery of an earned genesis chainstate owner.

use super::*;
use crate::{
    chainstate::{BasicFilterStartupMode, initialize_configured},
    storage::fjall_store::canonical_genesis::RecoveredCanonicalGenesis,
};
use open_bitcoin_core::{
    chainstate::FlushPolicyTime, consensus::chain_spec::CanonicalGenesisProof,
};

impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Transfer the replay-earned active core onto the same held durable coins.
    pub(crate) fn from_canonical_history(
        store: FjallNodeStore,
        recovered: crate::storage::fjall_store::canonical_genesis::RecoveredCanonicalHistory,
        mode: BasicFilterStartupMode,
    ) -> Result<
        (
            Self,
            CanonicalGenesisProof,
            open_bitcoin_network::HeaderStore,
        ),
        StorageError,
    > {
        let (genesis, headers, replay) = recovered
            .into_parts(&store)
            .map_err(|_| receipt_error("canonical history store proof changed"))?;
        let active = replay
            .admission_snapshot()
            .map_err(|_| receipt_error("canonical history active snapshot unavailable"))?;
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _view, cache) =
            initialize_configured(&store, now, now, 0, false, u64::MAX, mode)?;
        // Retained orphan undo remains independently proved in storage. Only
        // the actual replay-earned active prefix belongs in the live core.
        let mut chainstate = Chainstate::from_coins_cache(
            cache,
            active.active_chain,
            active.undo_by_block,
            active.maybe_confirmed_txid_counts,
        );
        replay
            .rebind_into(&mut chainstate)
            .map_err(|_| receipt_error("canonical history complete coins binding mismatch"))?;
        let managed = Self::from_recovered_chainstate(
            FjallChainstateStore::from_store(store),
            chainstate,
            lifecycle,
        )?;
        Ok((managed, genesis, headers))
    }

    /// Consume original-store evidence before recovery or index configuration writes.
    pub(crate) fn from_canonical_genesis(
        store: FjallNodeStore,
        recovered: RecoveredCanonicalGenesis,
        mode: BasicFilterStartupMode,
    ) -> Result<(Self, CanonicalGenesisProof), StorageError> {
        let proof = recovered
            .check(&store)
            .map_err(|_| receipt_error("canonical genesis store proof changed"))?
            .clone();
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _view, cache) =
            initialize_configured(&store, now, now, 0, false, u64::MAX, mode)?;
        let chainstate = Chainstate::from_canonical_genesis(&proof, cache)
            .map_err(|_| receipt_error("canonical genesis complete coins binding mismatch"))?;
        // BASIC lineage is conditional on the genuine active index proof. The
        // earned core genesis owner exists independently when indexing is off.
        let managed = Self::from_recovered_chainstate(
            FjallChainstateStore::from_store(store),
            chainstate,
            lifecycle,
        )?;
        Ok((managed, proof))
    }
}

#[cfg(test)]
mod tests;

// Bounded test-only adapter of accepted26 tagged fields; seeded facts earn no authority.
#[cfg(test)]
pub(super) mod historical_fixture {
    use open_bitcoin_core::{
        chainstate::{ChainPosition, Coin},
        consensus::{
            CandidateHistory, CanonicalChain, HistoryHeader, block_merkle_root, transaction_txid,
            transaction_wtxid,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput, Txid,
        },
    };
    use std::collections::HashMap;
    pub(in crate::chainstate::fjall_store) const DATA: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase167/bip30-fixtures.txt"
    ));
    type ParseResult<T> = Result<T, String>;

    // This is the finite tagged projection grammar, never a Bitcoin wire or JSON parser.
    struct Rows<'a> {
        rows: Vec<Vec<&'a str>>,
        index: usize,
    }
    impl<'a> Rows<'a> {
        fn new(text: &'a str) -> ParseResult<Self> {
            if text.len() > 1_048_576
                || !text.is_ascii()
                || !text.ends_with('\n')
                || text.contains('\r')
            {
                return Err("file bounds/encoding".into());
            }
            let mut rows = Vec::new();
            for line in text.lines() {
                if line.len() > 16_384 {
                    return Err("line bound".into());
                }
                if line.starts_with('#') {
                    continue;
                }
                rows.push(line.split('\t').collect());
            }
            if rows.len() > 1024 {
                return Err("record bound".into());
            }
            Ok(Self { rows, index: 0 })
        }
        fn take(&mut self, tag: &str, count: usize) -> ParseResult<Vec<&'a str>> {
            let row = self.rows.get(self.index).ok_or("missing record")?;
            if row.len() != count || row[0] != tag {
                return Err(format!("expected {tag}/{count}, row {}", self.index));
            }
            self.index += 1;
            Ok(row.clone())
        }
    }
    fn number<T: std::str::FromStr + ToString>(text: &str) -> ParseResult<T> {
        let value: T = text.parse().map_err(|_| "integer range")?;
        if value.to_string() != text {
            return Err("noncanonical integer".into());
        }
        Ok(value)
    }
    fn bytes(text: &str) -> ParseResult<Vec<u8>> {
        if text == "-" {
            return Ok(Vec::new());
        }
        if text.is_empty()
            || !text.len().is_multiple_of(2)
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("hex".into());
        }
        text.as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let s = std::str::from_utf8(pair).map_err(|_| "hex encoding")?;
                u8::from_str_radix(s, 16).map_err(|_| "hex range".into())
            })
            .collect()
    }
    fn hash(text: &str) -> ParseResult<[u8; 32]> {
        let mut value: [u8; 32] = bytes(text)?.try_into().map_err(|_| "hash width")?;
        value.reverse(); // The only display-to-internal conversion; scripts never reverse.
        Ok(value)
    }
    fn parse_script(text: &str) -> ParseResult<ScriptBuf> {
        ScriptBuf::from_bytes(bytes(text)?).map_err(|e| e.to_string())
    }
    fn header(rows: &mut Rows<'_>, role: &str, ordinal: usize) -> ParseResult<HistoryHeader> {
        let r = rows.take("HEADER", 12)?;
        if r[1] != role || number::<usize>(r[2])? != ordinal || r[3] != "mainnet" {
            return Err("header role/order/network".into());
        }
        let row = HistoryHeader::new(
            CanonicalChain::Mainnet,
            number(r[4])?,
            BlockHeader {
                version: number(r[6])?,
                previous_block_hash: BlockHash::from_byte_array(hash(r[7])?),
                merkle_root: open_bitcoin_core::primitives::MerkleRoot::from_byte_array(hash(
                    r[8],
                )?),
                time: number(r[9])?,
                bits: number(r[10])?,
                nonce: number(r[11])?,
            },
        );
        if row.block_hash != BlockHash::from_byte_array(hash(r[5])?) {
            return Err("header identity".into());
        }
        Ok(row)
    }
    fn transaction(rows: &mut Rows<'_>, role: &str, index: usize) -> ParseResult<Transaction> {
        let r = rows.take("TX", 9)?;
        if r[1] != role || number::<usize>(r[2])? != index {
            return Err("transaction order".into());
        }
        let (inputs, outputs) = (number::<usize>(r[7])?, number::<usize>(r[8])?);
        if inputs == 0 || outputs == 0 || inputs > 1024 || outputs > 1024 {
            return Err("transaction bounds".into());
        }
        let mut tx = Transaction {
            version: number(r[3])?,
            lock_time: number(r[4])?,
            ..Transaction::default()
        };
        for i in 0..inputs {
            let v = rows.take("IN", 9)?;
            if v[1] != role || number::<usize>(v[2])? != index || number::<usize>(v[3])? != i {
                return Err("input order".into());
            }
            let mut witness = Vec::new();
            let count: usize = number(v[8])?;
            if count > 1024 {
                return Err("witness bound".into());
            }
            for j in 0..count {
                let w = rows.take("WITNESS", 6)?;
                if w[1] != role
                    || number::<usize>(w[2])? != index
                    || number::<usize>(w[3])? != i
                    || number::<usize>(w[4])? != j
                {
                    return Err("witness order".into());
                }
                witness.push(bytes(w[5])?);
            }
            tx.inputs.push(TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array(hash(v[4])?),
                    vout: number(v[5])?,
                },
                script_sig: parse_script(v[6])?,
                sequence: number(v[7])?,
                witness: ScriptWitness::new(witness),
            });
        }
        for i in 0..outputs {
            let v = rows.take("OUT", 6)?;
            if v[1] != role || number::<usize>(v[2])? != index || number::<usize>(v[3])? != i {
                return Err("output order".into());
            }
            tx.outputs.push(TransactionOutput {
                value: Amount::from_sats(number(v[4])?).map_err(|e| e.to_string())?,
                script_pubkey: parse_script(v[5])?,
            });
        }
        if transaction_txid(&tx).map_err(|e| e.to_string())? != Txid::from_byte_array(hash(r[5])?)
            || transaction_wtxid(&tx)
                .map_err(|e| e.to_string())?
                .to_byte_array()
                != hash(r[6])?
        {
            return Err("transaction identity".into());
        }
        Ok(tx)
    }
    fn coin(rows: &mut Rows<'_>, role: &str) -> ParseResult<(OutPoint, Coin)> {
        let r = rows.take("COIN", 9)?;
        if r[1] != role || !matches!(r[6], "0" | "1") {
            return Err("coin role/boolean".into());
        }
        Ok((
            OutPoint {
                txid: Txid::from_byte_array(hash(r[2])?),
                vout: number(r[3])?,
            },
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(number(r[4])?).map_err(|e| e.to_string())?,
                    script_pubkey: parse_script(r[5])?,
                },
                is_coinbase: r[6] == "1",
                created_height: number(r[7])?,
                created_median_time_past: number(r[8])?,
            },
        ))
    }
    pub(in crate::chainstate::fjall_store) struct Case {
        pub(in crate::chainstate::fjall_store) block: Block,
        pub(in crate::chainstate::fjall_store) original: Block,
        pub(in crate::chainstate::fjall_store) history: CandidateHistory,
        pub(in crate::chainstate::fjall_store) positions: Vec<ChainPosition>,
        pub(in crate::chainstate::fjall_store) coins: HashMap<OutPoint, Coin>,
        pub(in crate::chainstate::fjall_store) old: (OutPoint, Coin),
        pub(in crate::chainstate::fjall_store) now: i64,
    }
    fn median(rows: &[HistoryHeader]) -> i64 {
        let mut times: Vec<_> = rows.iter().map(|r| i64::from(r.header.time)).collect();
        times.sort_unstable();
        times[times.len() / 2]
    }
    fn linked(rows: &[HistoryHeader]) -> ParseResult<()> {
        if rows.windows(2).any(|w| {
            w[0].height != w[1].height + 1 || w[0].header.previous_block_hash != w[1].block_hash
        }) {
            return Err("header links".into());
        }
        Ok(())
    }
    pub(in crate::chainstate::fjall_store) fn load(
        text: &str,
    ) -> ParseResult<(Vec<Case>, HistoryHeader)> {
        let mut rows = Rows::new(text)?;
        if rows.take("OPEN_BITCOIN_PHASE167_BIP30", 2)?[1] != "1"
            || rows.take("PIN", 2)?[1] != "a9aee730466ac67d35a3c03ee24676be5e045878"
        {
            return Err("version/pin".into());
        }
        // Source identities are fixed accepted26 data, compared exactly, never opened as paths.
        for expected in DATA.lines().filter(|l| l.starts_with("SOURCE\t")) {
            if rows.take("SOURCE", 3)?.join("\t") != expected {
                return Err("source identity".into());
            }
        }
        let anchor = header(&mut rows, "detached_bip34", 0)?;
        if anchor.height != 227931
            || anchor.block_hash
                != BlockHash::from_byte_array(hash(
                    "000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8",
                )?)
        {
            return Err("anchor height".into());
        }
        let mut cases = Vec::new();
        for (repeat, original_height, repeat_count, original_count) in
            [(91842, 91812, 1, 2), (91880, 91722, 2, 1)]
        {
            let meta = rows.take("CASE", 14)?;
            let expected_meta = DATA
                .lines()
                .find(|line| line.starts_with(&format!("CASE\tmainnet\t{repeat}\t")))
                .ok_or("fixed case metadata")?;
            if meta.join("\t") != expected_meta {
                return Err("case provenance".into());
            }
            if meta[1] != "mainnet"
                || number::<u32>(meta[2])? != repeat
                || number::<u32>(meta[4])? != original_height
            {
                return Err("case identity".into());
            }
            for i in [6, 8] {
                let _: u32 = number(meta[i])?;
            }
            for i in [7, 9] {
                let _: [u8; 32] = hash(meta[i])?;
            }
            let candidate = header(&mut rows, "candidate", 0)?;
            let original = header(&mut rows, "original", 0)?;
            if candidate.height != repeat
                || original.height != original_height
                || candidate.block_hash != BlockHash::from_byte_array(hash(meta[3])?)
                || original.block_hash != BlockHash::from_byte_array(hash(meta[5])?)
            {
                return Err("body header binding".into());
            }
            let mut history = Vec::new();
            for i in 0..155 {
                history.push(header(&mut rows, "history", i)?);
            }
            let mut all = history.clone();
            for i in 0..10 {
                all.push(header(&mut rows, "mtp_prefix", i)?);
            }
            let mut original_window = Vec::new();
            for i in 0..11 {
                original_window.push(header(&mut rows, "original_mtp", i)?);
            }
            linked(&all)?;
            linked(&original_window)?;
            if all[0].height != repeat - 1
                || all[0].block_hash != candidate.header.previous_block_hash
                || original_window[0].height != original_height - 1
                || original_window[0].block_hash != original.header.previous_block_hash
            {
                return Err("window binding".into());
            }
            let checked = CandidateHistory::check_seeded(
                CanonicalChain::Mainnet.spec().rules(),
                candidate.header.clone(),
                history[0].clone(),
                history[1..].to_vec(),
                None,
            )
            .map_err(|e| e.to_string())?;
            if checked.parent_median_time_past() != number::<i64>(meta[11])?
                || median(&original_window) != number::<i64>(meta[12])?
                || checked.expiry_median_time_past() != number::<i64>(meta[13])?
            {
                return Err("MTP binding".into());
            }
            // Relative suffix work starts at zero here; it is never global canonical work.
            let mut relative_work = open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
            let positions = history
                .iter()
                .enumerate()
                .rev()
                .map(|(i, row)| {
                    relative_work = relative_work.wrapping_add(
                        open_bitcoin_core::consensus::chain_proof::block_proof(row.header.bits)
                            .map_err(|e| e.to_string())?,
                    );
                    Ok(ChainPosition::new(
                        row.header.clone(),
                        row.height,
                        relative_work,
                        median(&all[i..i + 11]),
                    ))
                })
                .collect::<ParseResult<Vec<_>>>()?;
            let mut blocks = Vec::new();
            for (role, row, count) in [
                ("repeat", candidate, repeat_count),
                ("original", original, original_count),
            ] {
                let mut transactions = Vec::new();
                for i in 0..count {
                    transactions.push(transaction(&mut rows, role, i)?);
                }
                let block = Block {
                    header: row.header,
                    transactions,
                };
                if block_merkle_root(&block.transactions)
                    .map_err(|e| e.to_string())?
                    .0
                    != block.header.merkle_root
                {
                    return Err("merkle binding".into());
                }
                blocks.push(block);
            }
            let old = coin(&mut rows, "overwritten")?;
            let expected = Coin {
                output: blocks[1].transactions[0].outputs[0].clone(),
                is_coinbase: true,
                created_height: original_height,
                created_median_time_past: median(&original_window),
            };
            if old.0.vout != 0
                || old.0.txid
                    != transaction_txid(&blocks[1].transactions[0]).map_err(|e| e.to_string())?
                || old.1 != expected
            {
                return Err("original complete coin".into());
            }
            let mut coins = HashMap::from([old.clone()]);
            if repeat == 91880 {
                let external = coin(&mut rows, "external")?;
                if external.0 != blocks[0].transactions[1].inputs[0].previous_output
                    || external.1.is_coinbase
                    || external.1.created_height != 90644
                    || external.1.created_median_time_past != 1289277562
                    || external.1.output.value.to_sats() != 4480000000
                    || external.1.output.script_pubkey
                        != parse_script("76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac")?
                    || coins.insert(external.0, external.1).is_some()
                {
                    return Err("external input".into());
                }
            }
            if number::<u32>(rows.take("END_CASE", 2)?[1])? != repeat {
                return Err("end case".into());
            }
            cases.push(Case {
                block: blocks.remove(0),
                original: blocks.remove(0),
                history: checked,
                positions,
                coins,
                old,
                now: number(meta[10])?,
            });
        }
        if rows.take("END", 2)?[1] != "2" || rows.index != rows.rows.len() {
            return Err("trailing data".into());
        }
        Ok((cases, anchor))
    }
}

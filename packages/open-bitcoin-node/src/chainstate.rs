use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition, Chainstate, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Block,
};

pub trait ChainstateStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot>;
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot);
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryChainstateStore {
    maybe_snapshot: Option<ChainstateSnapshot>,
}

impl MemoryChainstateStore {
    pub fn snapshot(&self) -> Option<&ChainstateSnapshot> {
        self.maybe_snapshot.as_ref()
    }
}

impl ChainstateStore for MemoryChainstateStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        self.maybe_snapshot.clone()
    }

    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.maybe_snapshot = Some(snapshot);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedChainstate<S> {
    store: S,
    chainstate: Chainstate,
}

impl<S: ChainstateStore> ManagedChainstate<S> {
    pub fn from_store(store: S) -> Self {
        let chainstate = store
            .load_snapshot()
            .map(Chainstate::from_snapshot)
            .unwrap_or_default();

        Self { store, chainstate }
    }

    pub fn chainstate(&self) -> &Chainstate {
        &self.chainstate
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn connect_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        self.connect_block_with_current_time(
            block,
            chain_work,
            i64::from(block.header.time),
            verify_flags,
            consensus_params,
        )
    }

    pub fn connect_block_with_current_time(
        &mut self,
        block: &Block,
        chain_work: u128,
        current_time: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        let position = self.chainstate.connect_block_with_current_time(
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        self.persist();

        Ok(position)
    }

    pub fn disconnect_tip(
        &mut self,
        block: &Block,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        let position = self.chainstate.disconnect_tip(block)?;
        self.persist();

        Ok(position)
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, open_bitcoin_core::chainstate::ChainstateError> {
        let transition = self.chainstate.reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        self.persist();

        Ok(transition)
    }

    pub fn into_parts(self) -> (S, Chainstate) {
        (self.store, self.chainstate)
    }

    fn persist(&mut self) {
        self.store.save_snapshot(self.chainstate.snapshot());
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::{
        consensus::{ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
    };

    use crate::chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};

    const EASY_BITS: u32 = 0x207f_ffff;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn serialized_script_num(value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![0x00];
        }

        let mut magnitude = value as u64;
        let mut encoded = Vec::new();
        while magnitude > 0 {
            encoded.push((magnitude & 0xff) as u8);
            magnitude >>= 8;
        }

        let mut script = Vec::with_capacity(encoded.len() + 2);
        script.push(encoded.len() as u8);
        script.extend(encoded);
        script.push(0x51);
        script
    }

    fn coinbase_transaction(height: u32, value: i64) -> Transaction {
        let mut script_sig = serialized_script_num(i64::from(height));
        script_sig.push(0x51);
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&script_sig),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        }
    }

    fn mine_header(block: &mut Block) {
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("expected nonce at easy target");
    }

    fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
        let transactions = vec![coinbase_transaction(height, value)];
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash,
                merkle_root,
                time: 1_231_006_500 + height,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);
        block
    }

    #[test]
    fn memory_store_round_trips_saved_snapshots() {
        // Arrange
        let mut store = MemoryChainstateStore::default();
        let snapshot = open_bitcoin_core::chainstate::ChainstateSnapshot::new(
            Vec::new(),
            Default::default(),
            Default::default(),
        );

        // Act
        store.save_snapshot(snapshot.clone());

        // Assert
        assert_eq!(store.load_snapshot(), Some(snapshot));
    }

    #[test]
    fn managed_chainstate_persists_after_connect() {
        // Arrange
        let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);

        // Act
        let position = managed
            .connect_block(
                &genesis,
                1,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("managed chainstate should persist connected snapshot");

        // Assert
        assert_eq!(position.height, 0);
        let saved_snapshot = managed
            .store()
            .load_snapshot()
            .expect("snapshot should be present");
        assert_eq!(saved_snapshot.tip(), Some(&position));
        assert_eq!(managed.chainstate().tip(), Some(&position));
    }
}

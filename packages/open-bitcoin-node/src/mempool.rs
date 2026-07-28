// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/node/mempool_args.cpp
// - packages/bitcoin-knots/src/policy/policy.cpp

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{Block, Transaction},
};
use open_bitcoin_mempool::{
    AdmissionContext, AdmissionResult, BlockLifecycleContext, Mempool, MempoolError,
    MempoolOutcome, MempoolTransition, PolicyConfig, PolicyTime, PreparedMempoolTransition,
    RollingMempoolFeeRate, SubmitPackageCommand, SubmittedPackageResult,
};

use crate::{ChainstateStore, ManagedChainstate};

#[cfg(test)]
use open_bitcoin_mempool::SubmissionPackageKind;
#[cfg(test)]
use std::cell::{Cell, RefCell};

#[cfg(test)]
thread_local! {
    static PACKAGE_SUBMIT_COUNT: Cell<usize> = const { Cell::new(0) };
    static LAST_SUBMITTED_PACKAGE: RefCell<Option<SubmittedPackageResult>> =
        const { RefCell::new(None) };
}

#[derive(Debug, Clone)]
pub struct ManagedMempool {
    mempool: Mempool,
}

impl Default for ManagedMempool {
    fn default() -> Self {
        Self::new(PolicyConfig::default())
    }
}

impl ManagedMempool {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            mempool: Mempool::new(config),
        }
    }

    pub fn mempool(&self) -> &Mempool {
        &self.mempool
    }

    pub(crate) fn mempool_mut(&mut self) -> &mut Mempool {
        &mut self.mempool
    }

    /// Prepares singleton admission against the caller's immutable chain snapshot.
    #[allow(dead_code)] // Phase 134 establishes the sealed preparation API before routing callers.
    pub(crate) fn prepare_transaction_with_context<S: ChainstateStore>(
        &self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        self.mempool.prepare_transaction_with_context(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
            context,
        )
    }

    /// Prepares package admission against the supplied immutable chain snapshot.
    #[allow(dead_code)] // Phase 134 establishes the sealed preparation API before routing callers.
    pub(crate) fn prepare_package(
        &self,
        command: SubmitPackageCommand,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        self.mempool
            .prepare_package(command, chainstate, verify_flags, consensus_params)
    }

    /// Prepares deterministic expiry without changing managed mempool state.
    #[allow(dead_code)] // Phase 134 establishes the sealed preparation API before routing callers.
    pub(crate) fn prepare_expiry(
        &self,
        now: PolicyTime,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        self.mempool.prepare_expiry(now)
    }

    /// Prepares connected-block removal without changing managed mempool state.
    #[allow(dead_code)] // Phase 134 establishes the sealed preparation API before routing callers.
    pub(crate) fn prepare_connected_block_transition(
        &self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        self.mempool
            .prepare_connected_block_transition(block, context)
    }

    /// Installs a rolling floor for operator evidence and Phase-131 pressure seams.
    pub fn set_rolling_mempool_fee_rate(
        &mut self,
        rate: RollingMempoolFeeRate,
    ) -> Result<(), MempoolError> {
        self.mempool.set_rolling_mempool_fee_rate(rate)
    }

    /// Submits one checked package against the caller's immutable chain snapshot.
    pub fn submit_package(
        &mut self,
        command: SubmitPackageCommand,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<SubmittedPackageResult, MempoolError> {
        #[cfg(test)]
        let probe_candidate =
            command.package.kind() == SubmissionPackageKind::ChildWithUnconfirmedParents;
        #[cfg(test)]
        if probe_candidate {
            PACKAGE_SUBMIT_COUNT.with(|count| count.set(count.get() + 1));
        }
        let submitted =
            self.mempool
                .submit_package(command, chainstate, verify_flags, consensus_params)?;
        #[cfg(test)]
        if probe_candidate {
            LAST_SUBMITTED_PACKAGE.with(|last| last.replace(Some(submitted.clone())));
        }
        Ok(submitted)
    }

    #[cfg(test)]
    pub(crate) fn reset_package_submit_probe_for_test() {
        PACKAGE_SUBMIT_COUNT.with(|count| count.set(0));
        LAST_SUBMITTED_PACKAGE.with(|last| last.replace(None));
    }

    #[cfg(test)]
    pub(crate) fn package_submit_count_for_test() -> usize {
        PACKAGE_SUBMIT_COUNT.with(Cell::get)
    }

    #[cfg(test)]
    pub(crate) fn take_last_submitted_package_for_test() -> Option<SubmittedPackageResult> {
        LAST_SUBMITTED_PACKAGE.with(|last| last.borrow_mut().take())
    }

    #[cfg(test)]
    pub(crate) fn record_package_dispatch_for_test(submitted: &SubmittedPackageResult) {
        PACKAGE_SUBMIT_COUNT.with(|count| count.set(count.get().saturating_add(1)));
        LAST_SUBMITTED_PACKAGE.with(|last| last.replace(Some(submitted.clone())));
    }

    /// Submits a transaction with canonical metadata supplied by the node shell.
    pub fn submit_transaction_with_context<S: ChainstateStore>(
        &mut self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<AdmissionResult, MempoolError> {
        self.mempool.accept_transaction_with_context(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
            context,
        )
    }

    /// Submits a transaction and returns attempt details plus committed lifecycle facts.
    pub fn submit_transaction_transition_with_context<S: ChainstateStore>(
        &mut self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<MempoolTransition, MempoolError> {
        self.mempool.accept_transaction_transition_with_context(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
            context,
        )
    }

    /// Fail-closed no-context admission retained for intermediate workspace compatibility.
    ///
    /// Plan 130-06 migrates remaining node callers. Plan 130-11 removes this adapter
    /// after the final RPC caller has migrated.
    #[deprecated(
        note = "Plan 130-06 migrates node callers; Plan 130-11 migrates the final RPC caller and removes this fail-closed adapter"
    )]
    #[allow(deprecated)]
    pub fn submit_transaction<S: ChainstateStore>(
        &mut self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, MempoolError> {
        self.mempool.accept_transaction(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
        )
    }

    /// Fail-closed no-context outcome retained for intermediate workspace compatibility.
    ///
    /// Plan 130-07 migrates reorg callers. Plan 130-11 removes this adapter
    /// after the final RPC caller has migrated.
    #[deprecated(
        note = "Plan 130-07 migrates reorg callers; Plan 130-11 removes this fail-closed adapter"
    )]
    #[allow(deprecated)]
    pub fn submit_transaction_outcome<S: ChainstateStore>(
        &mut self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<MempoolOutcome, MempoolError> {
        self.mempool.accept_transaction_outcome(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
        )
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::consensus::crypto::hash160;
    use open_bitcoin_core::{
        consensus::{
            ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
            transaction_txid,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
    };
    use open_bitcoin_mempool::{AdmissionContext, MempoolOrigin, MempoolOutcome, PolicyTime};

    use crate::{ManagedChainstate, ManagedMempool, MemoryChainstateStore};

    const EASY_BITS: u32 = 0x207f_ffff;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn redeem_script() -> ScriptBuf {
        script(&[0x51])
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = hash160(redeem_script().as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
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
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(
        previous_txid: open_bitcoin_core::primitives::Txid,
        value: i64,
    ) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: p2sh_script(),
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
    #[allow(deprecated)] // Verifies the fail-closed adapter retained through Plan 130-11.
    fn managed_mempool_submits_against_managed_chainstate() {
        let mut chainstate = ManagedChainstate::from_store(MemoryChainstateStore::default());
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
        let spendable = build_block(
            open_bitcoin_core::consensus::block_hash(&genesis.header),
            1,
            500_000_000,
        );
        chainstate
            .connect_block(
                &genesis,
                1,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("genesis should connect");
        chainstate
            .connect_block(
                &spendable,
                2,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("follow-up block should connect");

        let mut mempool = ManagedMempool::default();
        let result = mempool
            .submit_transaction(
                &chainstate,
                spend_transaction(
                    transaction_txid(&genesis.transactions[0]).expect("txid"),
                    499_999_000,
                ),
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("transaction should be admitted");
        let contextual_transaction = spend_transaction(
            transaction_txid(&spendable.transactions[0]).expect("txid"),
            499_999_000,
        );
        let contextual_result = mempool
            .submit_transaction_with_context(
                &chainstate,
                contextual_transaction.clone(),
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
                AdmissionContext::peer(PolicyTime::from_unix_seconds(42)),
            )
            .expect("contextual transaction should be admitted");
        let duplicate = mempool
            .submit_transaction_outcome(
                &chainstate,
                contextual_transaction,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("duplicate outcome");

        assert!(mempool.mempool().entry(&result.accepted).is_some());
        assert_eq!(
            mempool
                .mempool()
                .entry(&contextual_result.accepted)
                .expect("contextual entry")
                .metadata
                .origin,
            MempoolOrigin::Peer
        );
        assert!(matches!(duplicate, MempoolOutcome::Duplicate { .. }));
    }
}

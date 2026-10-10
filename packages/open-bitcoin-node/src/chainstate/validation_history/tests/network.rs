// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp

use super::*;
use crate::{ManagedNetworkHandle, ManagedPeerNetwork};
use open_bitcoin_core::consensus::{crypto::hash160, transaction_txid};
use open_bitcoin_mempool::{MempoolOutcome, PolicyTime, RelayIntent, ReorgLifecycleContext};

fn remine(block: &mut Block) {
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy POW");
}

#[test]
fn phase159_validation_history_accept_network_refusal_precedes_chainstate_preview_and_mempool_patch()
 {
    // Arrange / Act / Assert
    for unresolved in [true, false] {
        let (dir, store, mut manager) = fresh("network-refusal", false);
        let mut p2sh = vec![0xa9, 20];
        p2sh.extend_from_slice(&hash160(&[0x51]));
        p2sh.push(0x87);
        let script = ScriptBuf::from_bytes(p2sh).expect("standard P2SH");
        let mut genesis = block(BlockHash::default(), 0, 0x51);
        genesis.transactions[0].outputs[0].script_pubkey = script.clone();
        remine(&mut genesis);
        accept(&mut manager, &genesis, 0).expect("genuine genesis");
        let child = block(block_hash(&genesis.header), 1, 0x51);
        if unresolved {
            store.set_validation_history_fault(HistoryPublicationFault::BeforeCommit);
        }
        let accepted = accept(&mut manager, &child, 1);
        assert_eq!(accepted.is_err(), unresolved);
        let network = ManagedPeerNetwork::from_initialized_chainstate(
            manager,
            Default::default(),
            Default::default(),
            16,
            Default::default(),
            Default::default(),
            false,
        );
        let handle = ManagedNetworkHandle::new(network);
        let params = ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        };
        let spend = Transaction {
            version: 2,
            lock_time: 0,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: transaction_txid(&genesis.transactions[0]).expect("txid"),
                    vout: 0,
                },
                script_sig: ScriptBuf::from_bytes(vec![1, 0x51]).expect("redeem"),
                sequence: u32::MAX,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(4_999_990_000).expect("value"),
                script_pubkey: script,
            }],
        };
        assert!(matches!(
            handle
                .submit_local_transaction_outcome_at(
                    spend.clone(),
                    ScriptVerifyFlags::P2SH,
                    params,
                    2_000,
                    RelayIntent::Requested
                )
                .expect("admission"),
            MempoolOutcome::Accepted { .. }
        ));
        let before_chain = handle.chainstate_snapshot().expect("before chain");
        let before_mempool = handle.mempool_info().expect("before mempool");
        assert_eq!(before_mempool.transaction_count, 1);
        let mut parent = block_hash(&child.header);
        let mut branch = Vec::new();
        for height in 2..=130 {
            let mut next = block(parent, height, 0x51);
            if height == 2 {
                next.transactions.push(spend.clone());
                remine(&mut next);
            }
            parent = block_hash(&next.header);
            branch.push(AnchoredBlock {
                block: next,
                chain_work: u128::from(height) + 1,
            });
        }
        let result = if unresolved {
            handle
                .connect_local_block(&branch[0].block, ScriptVerifyFlags::P2SH, params)
                .map(|_| ())
        } else {
            handle
                .reorg_to_branch(
                    &[],
                    &branch,
                    ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000)),
                    ScriptVerifyFlags::P2SH,
                    params,
                )
                .map(|_| ())
        };
        assert!(result.is_err());
        assert_eq!(
            handle.chainstate_snapshot().expect("unchanged chain"),
            before_chain
        );
        assert_eq!(
            handle.mempool_info().expect("unremoved transaction"),
            before_mempool
        );
        drop(handle);
        drop(store);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}

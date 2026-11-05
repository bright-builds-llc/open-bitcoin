// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

pub(super) fn branch(chain: CanonicalChain, first: u32, last: u32) -> Vec<HistoryHeader> {
    let mut previous = BlockHash::default();
    let mut rows = Vec::new();
    for height in first..=last {
        let header = if height == 0 {
            chain.spec().genesis_header().clone()
        } else {
            BlockHeader {
                version: 4,
                previous_block_hash: previous,
                time: 1_600_000_000u32.saturating_add(height),
                bits: chain.spec().rules().pow_limit_bits(),
                nonce: height,
                ..BlockHeader::default()
            }
        };
        previous = block_hash(&header);
        rows.push(HistoryHeader::new(chain, height, header));
    }
    rows.reverse();
    rows
}

pub(super) fn check_rows(
    chain: CanonicalChain,
    mut rows: Vec<HistoryHeader>,
) -> Result<CandidateHistory, HistoryError> {
    let parent = rows.remove(0);
    let candidate = BlockHeader {
        previous_block_hash: parent.block_hash,
        time: parent.header.time + 1,
        bits: parent.header.bits,
        version: 4,
        ..BlockHeader::default()
    };
    CandidateHistory::check(chain.spec().rules(), candidate, parent, rows)
}

fn relink(rows: &mut [HistoryHeader]) {
    let mut maybe_previous = None;
    for row in rows.iter_mut().rev() {
        if let Some(previous) = maybe_previous {
            row.header.previous_block_hash = previous;
        }
        row.block_hash = block_hash(&row.header);
        maybe_previous = Some(row.block_hash);
    }
}

#[test]
fn repeated_checked_branch_hash_refuses_before_parent_link_mismatch() {
    // Arrange
    let mut rows = branch(CanonicalChain::Regtest, 0, 3);
    let mut repeated = rows[1].clone();
    repeated.height = rows[2].height;
    rows[2] = repeated;

    // Act
    let result = check_rows(CanonicalChain::Regtest, rows);

    // Assert
    assert_eq!(result, Err(HistoryError::RepeatedHash { height: 1 }));
}

#[test]
fn phase167_candidate_history_true_short_genesis_windows_succeed() {
    for parent_height in 0..11 {
        // Arrange
        let rows = branch(CanonicalChain::Regtest, 0, parent_height);
        let mut times: Vec<_> = rows.iter().map(|row| i64::from(row.header.time)).collect();
        times.sort_unstable();
        // Act
        let history = check_rows(CanonicalChain::Regtest, rows).expect("linked short history");
        // Assert
        assert_eq!(history.origin(), HistoryOrigin::GenesisLinked);
        assert_eq!(history.height(), parent_height + 1);
        assert_eq!(history.parent_median_time_past(), times[times.len() / 2]);
        assert_eq!(history.candidate_hash(), block_hash(history.candidate()));
    }
}

#[test]
fn phase167_candidate_history_seeded_coverage_is_explicit() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, 850, 1004);
    let oldest_hash = rows.last().expect("oldest").block_hash;
    // Act
    let history = check_rows(CanonicalChain::Mainnet, rows).expect("complete required windows");
    // Assert
    assert_eq!(
        history.origin(),
        HistoryOrigin::CheckedSuffix {
            oldest_height: 850,
            oldest_hash
        }
    );
    assert_eq!(history.parent_median_time_past(), 1_600_000_999);
    assert_eq!(history.expiry_median_time_past(), 1_600_000_855);
    assert_eq!(
        history.median_time_past_at(849),
        Err(HistoryError::MissingAncestor { height: 849 })
    );
}

#[test]
fn phase167_candidate_history_truncated_high_mtp_refuses() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, 995, 1004);
    // Act / Assert
    assert_eq!(
        check_rows(CanonicalChain::Mainnet, rows),
        Err(HistoryError::MissingAncestor { height: 994 })
    );
}

#[test]
fn phase167_candidate_history_expiry_requires_its_own_complete_mtp() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, 851, 1004);
    // Act / Assert
    assert_eq!(
        check_rows(CanonicalChain::Mainnet, rows),
        Err(HistoryError::MissingAncestor { height: 850 })
    );
}

#[test]
fn phase167_candidate_history_wrong_network_hash_and_height_refuse() {
    for failure in 0..3 {
        // Arrange
        let mut rows = branch(CanonicalChain::Regtest, 0, 10);
        let expected = match failure {
            0 => {
                rows[3].chain = CanonicalChain::Mainnet;
                HistoryError::WrongNetwork { height: 7 }
            }
            1 => {
                rows[3].block_hash = BlockHash::default();
                HistoryError::HashMismatch { height: 7 }
            }
            _ => {
                rows[3].height = 6;
                HistoryError::HeightMismatch {
                    expected: 7,
                    actual: 6,
                }
            }
        };
        // Act / Assert
        assert_eq!(check_rows(CanonicalChain::Regtest, rows), Err(expected));
    }
}

#[test]
fn phase167_candidate_history_gaps_cycles_and_foreign_branch_refuse() {
    for failure in 0..3 {
        // Arrange
        let mut rows = branch(CanonicalChain::Regtest, 0, 10);
        match failure {
            0 => {
                rows.remove(4);
            }
            1 => {
                rows[4] = rows[2].clone();
            }
            _ => {
                rows[4].header.nonce += 1;
                rows[4].block_hash = block_hash(&rows[4].header);
            }
        }
        // Act / Assert
        assert!(check_rows(CanonicalChain::Regtest, rows).is_err());
    }
}

#[test]
fn phase167_candidate_history_wrong_candidate_parent_and_genesis_refuse() {
    // Arrange
    let mut rows = branch(CanonicalChain::Regtest, 0, 0);
    let parent = rows.remove(0);
    // Act / Assert
    assert!(
        CandidateHistory::check(
            CanonicalChain::Regtest.spec().rules(),
            BlockHeader::default(),
            parent,
            rows
        )
        .is_err()
    );
    // Arrange
    let mut rows = branch(CanonicalChain::Regtest, 0, 0);
    rows[0].header.nonce += 1;
    rows[0].block_hash = block_hash(&rows[0].header);
    // Act / Assert
    assert_eq!(
        check_rows(CanonicalChain::Regtest, rows),
        Err(HistoryError::WrongGenesis)
    );
}

#[test]
fn phase167_candidate_history_retarget_anchor_is_exact_and_linked() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, 0, 2015);
    // Act
    let history = check_rows(CanonicalChain::Mainnet, rows.clone()).expect("boundary context");
    // Assert
    assert_eq!(
        history.maybe_retarget_anchor().map(|row| row.height),
        Some(0)
    );
    assert_eq!(
        check_rows(CanonicalChain::Mainnet, rows[..2015].to_vec()),
        Err(HistoryError::MissingAncestor { height: 0 })
    );
}

#[test]
fn phase167_candidate_history_min_difficulty_recovery_uses_branch_boundary() {
    // Arrange
    let rows = branch(CanonicalChain::Testnet3, 2000, 2200);
    // Act
    let history = check_rows(CanonicalChain::Testnet3, rows).expect("period recovery");
    // Assert
    assert_eq!(
        history
            .maybe_min_difficulty_recovery()
            .map(|row| row.height),
        Some(2016)
    );
}

#[test]
fn phase167_candidate_history_bip34_activation_cannot_be_its_own_ancestor() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, 227_776, 227_930);
    // Act
    let history = check_rows(CanonicalChain::Mainnet, rows).expect("before BIP34 ancestor exists");
    // Assert
    assert_eq!(history.height(), 227_931);
    assert_eq!(history.maybe_bip34_ancestor(), None);
    assert!(
        check_rows(
            CanonicalChain::Mainnet,
            branch(CanonicalChain::Mainnet, 227_932, 228_100)
        )
        .is_err()
    );
}

#[test]
fn phase167_candidate_history_height_overflow_refuses() {
    // Arrange
    let rows = branch(CanonicalChain::Mainnet, u32::MAX, u32::MAX);
    // Act / Assert
    let parent = rows[0].clone();
    assert_eq!(
        CandidateHistory::check(
            CanonicalChain::Mainnet.spec().rules(),
            BlockHeader::default(),
            parent,
            Vec::new()
        ),
        Err(HistoryError::HeightOverflow)
    );
}

#[test]
fn phase167_candidate_history_independent_bip34_seed_stays_tainted() {
    // Arrange
    let chain = CanonicalChain::Mainnet;
    let mut rows = branch(chain, 692_106, 692_260);
    let parent = rows.remove(0);
    let candidate = BlockHeader {
        previous_block_hash: parent.block_hash,
        time: parent.header.time + 1,
        ..BlockHeader::default()
    };
    let seed = branch(chain, 227_931, 227_931).remove(0);
    // Act
    let history = CandidateHistory::check_seeded(
        chain.spec().rules(),
        candidate.clone(),
        parent.clone(),
        rows.clone(),
        Some(seed.clone()),
    )
    .expect("conditional seed");
    // Assert
    assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
    assert_eq!(
        history.bip34_context(),
        Bip34Context::IndependentlySeeded(seed.block_hash)
    );
    assert_eq!(history.maybe_bip34_ancestor(), None);
    assert_eq!(
        CandidateHistory::check(
            chain.spec().rules(),
            candidate.clone(),
            parent.clone(),
            rows.clone()
        ),
        Err(HistoryError::MissingAncestor { height: 227_931 })
    );
    assert_eq!(
        CandidateHistory::check_seeded(chain.spec().rules(), candidate, parent, rows, None),
        Err(HistoryError::MissingAncestor { height: 227_931 })
    );
}

#[test]
fn phase167_candidate_history_seed_identity_network_and_height_are_checked() {
    for failure in 0..3 {
        // Arrange
        let chain = CanonicalChain::Mainnet;
        let mut rows = branch(chain, 692_106, 692_260);
        let parent = rows.remove(0);
        let candidate = BlockHeader {
            previous_block_hash: parent.block_hash,
            time: parent.header.time + 1,
            ..BlockHeader::default()
        };
        let mut seed = branch(chain, 227_931, 227_931).remove(0);
        match failure {
            0 => seed.chain = CanonicalChain::Testnet3,
            1 => seed.block_hash = BlockHash::default(),
            _ => seed.height += 1,
        }
        // Act / Assert
        assert!(
            CandidateHistory::check_seeded(
                chain.spec().rules(),
                candidate,
                parent,
                rows,
                Some(seed)
            )
            .is_err()
        );
    }
}

#[test]
fn phase167_candidate_history_unneeded_anchor_is_not_unknown_or_absent() {
    // Arrange / Act
    let history = check_rows(
        CanonicalChain::Regtest,
        branch(CanonicalChain::Regtest, 850, 1004),
    )
    .expect("no selected BIP34 commitment");
    // Assert
    assert_eq!(history.bip34_context(), Bip34Context::NoSelectedCommitment);
    // Arrange / Act
    let history = check_rows(
        CanonicalChain::Mainnet,
        branch(CanonicalChain::Mainnet, 1_983_547, 1_983_701),
    )
    .expect("recheck forces scan");
    // Assert
    assert_eq!(history.bip34_context(), Bip34Context::UnneededAtRecheck);
}

#[test]
fn phase167_candidate_history_mtp_sorts_exactly_eleven_not_twelve() {
    // Arrange
    let mut rows = branch(CanonicalChain::Mainnet, 850, 1004);
    for (row, time) in rows
        .iter_mut()
        .zip([100, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 100])
    {
        row.header.time = time;
    }
    relink(&mut rows);
    // Act
    let history = check_rows(CanonicalChain::Mainnet, rows).expect("nonmonotonic linked windows");
    // Assert
    assert_eq!(history.parent_median_time_past(), 5);
}

#[test]
fn phase167_candidate_history_recovery_equality_and_late_boundary() {
    for delay in [1200, 1201] {
        // Arrange
        let chain = CanonicalChain::Testnet3;
        let mut rows = branch(chain, 2000, 2200);
        rows[5].header.bits = 0x1c0f_ffff;
        relink(&mut rows);
        let parent = rows.remove(0);
        let candidate = BlockHeader {
            time: parent.header.time + delay,
            previous_block_hash: parent.block_hash,
            ..BlockHeader::default()
        };
        // Act
        let history = CandidateHistory::check(chain.spec().rules(), candidate, parent, rows)
            .expect("recovery context");
        // Assert
        assert_eq!(
            history
                .maybe_min_difficulty_recovery()
                .map(|row| (row.height, row.header.bits)),
            if delay == 1200 {
                Some((2195, 0x1c0f_ffff))
            } else {
                None
            }
        );
    }
}

#[test]
fn phase167_candidate_history_truncated_recovery_path_refuses() {
    // Arrange
    let rows = branch(CanonicalChain::Testnet3, 2036, 2200);
    // Act / Assert
    assert_eq!(
        check_rows(CanonicalChain::Testnet3, rows),
        Err(HistoryError::MissingAncestor { height: 2035 })
    );
}

#[test]
fn phase167_candidate_history_seed_overlapping_linked_bip34_cannot_disagree() {
    // Arrange
    let chain = CanonicalChain::Mainnet;
    let mut rows = branch(chain, 227_800, 228_100);
    let parent = rows.remove(0);
    let candidate = BlockHeader {
        time: parent.header.time + 1,
        previous_block_hash: parent.block_hash,
        ..BlockHeader::default()
    };
    let seed = branch(chain, 227_931, 227_931).remove(0);
    // Act / Assert
    assert_eq!(
        CandidateHistory::check_seeded(chain.spec().rules(), candidate, parent, rows, Some(seed)),
        Err(HistoryError::HashMismatch { height: 227_931 })
    );
}

#[test]
fn phase167_candidate_history_seed_constructor_cannot_claim_genesis_origin() {
    // Arrange
    let chain = CanonicalChain::Regtest;
    let parent = branch(chain, 0, 0).remove(0);
    let candidate = BlockHeader {
        previous_block_hash: parent.block_hash,
        ..BlockHeader::default()
    };
    // Act
    let history = CandidateHistory::check_seeded(
        chain.spec().rules(),
        candidate.clone(),
        parent.clone(),
        Vec::new(),
        None,
    )
    .expect("explicit seed even at genesis");
    // Assert
    assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
    assert_eq!(history.rules().origin(), HistoryOrigin::IndependentlySeeded);
    // Arrange / Act / Assert
    assert_eq!(
        CandidateHistory::check_seeded(
            chain.spec().rules(),
            candidate,
            parent.clone(),
            Vec::new(),
            Some(parent)
        ),
        Err(HistoryError::UnexpectedBip34Seed)
    );
}

#[test]
fn phase167_candidate_history_linked_bip34_is_present_after_activation() {
    // Arrange
    let chain = CanonicalChain::Mainnet;
    let rows = branch(chain, 227_776, 227_931);
    let expected = rows[0].block_hash;
    // Act
    let history = check_rows(chain, rows).expect("activation parent is linked");
    // Assert
    assert_eq!(history.bip34_context(), Bip34Context::Linked(expected));
    assert_eq!(
        history.maybe_bip34_ancestor().map(|row| row.block_hash),
        Some(expected)
    );
}

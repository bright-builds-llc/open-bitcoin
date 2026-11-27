// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp

use std::collections::HashMap;

use open_bitcoin_chainstate::{Chainstate, ChainstateError, CoinsCache, MemoryCoinsView};
use open_bitcoin_consensus::chain_spec::{
    CanonicalChain, CanonicalGenesisProof, MAINNET_OFFLINE_PREFIX_MAX_HEIGHT,
};

#[path = "../../testdata/mainnet-prefix-headers.rs"]
mod prefix;

fn canonical_mainnet() -> Result<Chainstate, Box<dyn std::error::Error>> {
    let spec = CanonicalChain::Mainnet.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block())?;
    let coins = CoinsCache::from_parent(MemoryCoinsView::from_coins(
        HashMap::new(),
        Some(proof.hash()),
    ));
    Ok(Chainstate::from_canonical_genesis(&proof, coins)?)
}

#[test]
fn mainnet_chainstate_refuses_admission_past_the_prefix() -> Result<(), Box<dyn std::error::Error>>
{
    // Arrange
    let mut chain = canonical_mainnet()?;
    let headers = prefix::mainnet_prefix_headers()?;
    let candidate: Vec<_> = headers
        .into_iter()
        .map(|header| (header, 2_000_000_000))
        .collect();
    chain.admit_canonical_reorg_headers(
        &[],
        &candidate[..MAINNET_OFFLINE_PREFIX_MAX_HEIGHT as usize],
    )?;

    // Act
    let result = chain.admit_canonical_reorg_headers(&[], &candidate);

    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::Serialization { context: "candidate history", reason })
            if reason == "mainnet-offline-prefix-limit"
    ));
    assert_eq!(chain.tip().map(|tip| tip.height), Some(0));
    Ok(())
}

#[test]
fn mainnet_chainstate_refuses_replacement_admission() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let mut chain = canonical_mainnet()?;
    let tip = chain.tip().ok_or("expected proved genesis")?.header.clone();
    let first = prefix::mainnet_prefix_headers()?
        .into_iter()
        .next()
        .ok_or("expected first header")?;

    // Act
    let result = chain.admit_canonical_reorg_headers(&[tip], &[(first, 2_000_000_000)]);

    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::Serialization { context: "candidate history", reason })
            if reason == "mainnet-offline-prefix-requires-tip-extension"
    ));
    assert_eq!(chain.tip().map(|tip| tip.height), Some(0));
    Ok(())
}

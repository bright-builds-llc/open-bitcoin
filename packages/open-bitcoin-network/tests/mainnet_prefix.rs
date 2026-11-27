// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp

use open_bitcoin_consensus::chain_spec::{
    CanonicalChain, CanonicalGenesisProof, MAINNET_OFFLINE_PREFIX_MAX_HEIGHT,
};
use open_bitcoin_network::{HeaderStore, NetworkError};

#[path = "../../testdata/mainnet-prefix-headers.rs"]
mod prefix;

#[test]
fn mainnet_header_store_refuses_a_child_beyond_the_prefix() -> Result<(), Box<dyn std::error::Error>>
{
    // Arrange
    let spec = CanonicalChain::Mainnet.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block())?;
    let mut store = HeaderStore::from_canonical_genesis(&proof)?;
    let mut headers = prefix::mainnet_prefix_headers()?;
    let boundary = headers.pop().ok_or("expected height-201 header")?;
    for header in headers {
        store
            .insert_canonical_header_detailed(header, 2_000_000_000)
            .map_err(|error| error.into_network_error())?;
    }
    let before = store.best_tip().cloned().ok_or("expected prefix tip")?;
    assert_eq!(before.height, MAINNET_OFFLINE_PREFIX_MAX_HEIGHT);

    // Act
    let result = store.insert_canonical_header_detailed(boundary, 2_000_000_000);

    // Assert
    assert!(matches!(
        result.map_err(|error| error.into_network_error()),
        Err(NetworkError::InvalidHeader { reject_reason, .. })
            if reject_reason == "mainnet-offline-prefix-limit"
    ));
    assert_eq!(store.best_tip(), Some(&before));
    Ok(())
}

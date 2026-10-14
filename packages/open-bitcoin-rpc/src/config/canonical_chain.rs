// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/chainparams.cpp

use super::RuntimeConfig;
use open_bitcoin_node::{
    SyncNetwork,
    core::{consensus::CanonicalChain, wallet::AddressNetwork},
};

/// Conflicting selectors cannot silently choose different chain authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalChainSelectionError;

impl core::fmt::Display for CanonicalChainSelectionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(
            "inconsistent_network_selectors: configured chain and sync network must agree",
        )
    }
}

impl std::error::Error for CanonicalChainSelectionError {}

/// Resolve existing selectors into one canonical identity without granting validation support.
pub fn selected_canonical_chain(
    runtime: &RuntimeConfig,
) -> Result<CanonicalChain, CanonicalChainSelectionError> {
    let configured = match runtime.chain {
        AddressNetwork::Mainnet => CanonicalChain::Mainnet,
        AddressNetwork::Testnet => CanonicalChain::Testnet3,
        AddressNetwork::Signet => CanonicalChain::Signet,
        AddressNetwork::Regtest => CanonicalChain::Regtest,
    };
    let sync = match runtime.sync.runtime.network {
        SyncNetwork::Mainnet => CanonicalChain::Mainnet,
        SyncNetwork::Testnet => CanonicalChain::Testnet3,
        SyncNetwork::Signet => CanonicalChain::Signet,
        SyncNetwork::Regtest => CanonicalChain::Regtest,
    };
    if configured != sync {
        return Err(CanonicalChainSelectionError);
    }
    Ok(configured)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_chain_selection_rejects_every_inconsistent_pair() {
        // Arrange
        let addresses = [
            AddressNetwork::Mainnet,
            AddressNetwork::Testnet,
            AddressNetwork::Signet,
            AddressNetwork::Regtest,
        ];
        let networks = [
            SyncNetwork::Mainnet,
            SyncNetwork::Testnet,
            SyncNetwork::Signet,
            SyncNetwork::Regtest,
        ];
        let chains = [
            CanonicalChain::Mainnet,
            CanonicalChain::Testnet3,
            CanonicalChain::Signet,
            CanonicalChain::Regtest,
        ];
        for (address_index, chain) in addresses.into_iter().enumerate() {
            for (sync_index, network) in networks.into_iter().enumerate() {
                let mut runtime = RuntimeConfig {
                    chain,
                    ..RuntimeConfig::default()
                };
                runtime.sync.runtime.network = network;
                // Act
                let result = selected_canonical_chain(&runtime);
                // Assert
                if address_index == sync_index {
                    assert_eq!(result, Ok(chains[address_index]));
                } else {
                    assert_eq!(result, Err(CanonicalChainSelectionError));
                }
            }
        }
    }
}

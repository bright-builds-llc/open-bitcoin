// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/pow.cpp

mod bip30_repeats;
mod bip30_restore;
mod contexts;
mod ordinary_flags;
mod scripts;
mod source;

fn verify_selected_header_listing(
    item: &serde_json::Value,
    source: &serde_json::Value,
    header: &open_bitcoin_core::primitives::BlockHeader,
    hash: &str,
) -> Result<(), String> {
    let parent = source::display(header.previous_block_hash.as_bytes());
    let merkle = source::display(header.merkle_root.as_bytes());
    let parent_matches = if source["height"].as_u64() == Some(0)
        && header.previous_block_hash.as_bytes() == &[0; 32]
    {
        item["previousblockhash"].is_null()
            || item["previousblockhash"].as_str() == Some(parent.as_str())
    } else {
        item["previousblockhash"].as_str() == Some(parent.as_str())
    };
    let mismatches: Vec<_> = [
        ("id", item["id"].as_str() == Some(hash)),
        (
            "height",
            item["height"].as_u64() == source["height"].as_u64(),
        ),
        (
            "version",
            item["version"].as_i64() == Some(i64::from(header.version)),
        ),
        (
            "timestamp",
            item["timestamp"].as_u64() == Some(u64::from(header.time)),
        ),
        (
            "bits",
            item["bits"].as_u64() == Some(u64::from(header.bits)),
        ),
        (
            "nonce",
            item["nonce"].as_u64() == Some(u64::from(header.nonce)),
        ),
        ("previousblockhash", parent_matches),
        (
            "merkle_root",
            item["merkle_root"].as_str() == Some(merkle.as_str()),
        ),
    ]
    .into_iter()
    .filter_map(|(name, matches)| (!matches).then_some(name))
    .collect();
    if !mismatches.is_empty() {
        return Err(format!(
            "selected header listing facts differ from 80-byte source: {hash}: {}",
            mismatches.join(",")
        ));
    }
    Ok(())
}

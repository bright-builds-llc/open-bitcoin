// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/coins.cpp

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateSnapshot, Coin},
    consensus::{
        CandidateHistory, CanonicalChain, HistoryHeader, HistoryOrigin, block_hash,
        block_merkle_root, block_proof, check_block_header, crypto::Sha256, transaction_txid,
    },
    primitives::{Block, OutPoint, Txid},
};
use serde_json::Value;

pub(super) struct Case {
    pub block: Block,
    pub history: CandidateHistory,
    pub snapshot: ChainstateSnapshot,
    pub now: i64,
    pub purpose: String,
    pub overwrite_original: Vec<String>,
}

fn data(name: &str, limit: u64) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../testdata/phase167")
        .join(name);
    let size = fs::metadata(&path).expect("source corpus metadata").len();
    assert!(size <= limit, "bounded source corpus: {name}");
    fs::read(path).expect("complete bounded source corpus")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn display(bytes: &[u8]) -> String {
    hex(&bytes.iter().rev().copied().collect::<Vec<_>>())
}

pub(super) fn bytes(value: &str) -> Vec<u8> {
    assert!(value.len().is_multiple_of(2) && value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = (pair[0] as char).to_digit(16).expect("hex high");
            let low = (pair[1] as char).to_digit(16).expect("hex low");
            ((high << 4) | low) as u8
        })
        .collect()
}

fn hash(value: &str) -> [u8; 32] {
    let mut data: [u8; 32] = bytes(value).try_into().expect("display hash width");
    data.reverse();
    data
}

fn string<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field]
        .as_str()
        .unwrap_or_else(|| panic!("missing {field}"))
}

fn number(value: &Value, field: &str) -> u64 {
    value[field]
        .as_u64()
        .unwrap_or_else(|| panic!("missing {field}"))
}

fn chain(name: &str) -> CanonicalChain {
    match name {
        "mainnet" => CanonicalChain::Mainnet,
        "testnet3" => CanonicalChain::Testnet3,
        _ => panic!("unsupported historical corpus network"),
    }
}

fn median(rows: &[HistoryHeader]) -> i64 {
    let mut times: Vec<_> = rows.iter().map(|row| i64::from(row.header.time)).collect();
    times.sort_unstable();
    times[times.len() / 2]
}

fn positions(branch: &[HistoryHeader]) -> Vec<ChainPosition> {
    let mut work = open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
    branch
        .iter()
        .enumerate()
        .rev()
        .map(|(index, row)| {
            work = work.wrapping_add(block_proof(row.header.bits).expect("header work"));
            ChainPosition::new(
                row.header.clone(),
                row.height,
                work,
                median(&branch[index..(index + 11).min(branch.len())]),
            )
        })
        .collect()
}

/// Validate bounded imported bytes and selected response provenance without opening storage.
fn checked_corpus(
    manifest_bytes: &[u8],
    blocks_bytes: &[u8],
    contexts_bytes: &[u8],
    purposes: &[&str],
) -> Result<(Value, Value, Value), String> {
    fn field<'a>(row: &'a Value, name: &str) -> Result<&'a str, String> {
        row.get(name)
            .and_then(Value::as_str)
            .ok_or_else(|| format!("missing {name}"))
    }

    fn decoded(raw: &str) -> Result<Vec<u8>, String> {
        if !raw.len().is_multiple_of(2) || !raw.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("invalid source hex".to_owned());
        }
        raw.as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let high = (pair[0] as char).to_digit(16).ok_or("hex high")?;
                let low = (pair[1] as char).to_digit(16).ok_or("hex low")?;
                Ok(((high << 4) | low) as u8)
            })
            .collect::<Result<_, &str>>()
            .map_err(str::to_owned)
    }

    let manifest: Value = serde_json::from_slice(manifest_bytes)
        .map_err(|error| format!("manifest JSON: {error}"))?;
    let blocks: Value =
        serde_json::from_slice(blocks_bytes).map_err(|error| format!("blocks JSON: {error}"))?;
    let contexts: Value = serde_json::from_slice(contexts_bytes)
        .map_err(|error| format!("contexts JSON: {error}"))?;
    if field(&manifest, "pin")? != "a9aee730466ac67d35a3c03ee24676be5e045878"
        || field(&manifest, "evidence_class")? != "independently-seeded-historical"
        || field(&manifest, "blocks_sha256")? != hex(&Sha256::digest(blocks_bytes))
        || field(&manifest, "contexts_sha256")? != hex(&Sha256::digest(contexts_bytes))
        || blocks["schema"].as_u64() != Some(1)
        || contexts["schema"].as_u64() != Some(1)
    {
        return Err("source pin, digest or schema mismatch".to_owned());
    }
    let rows = blocks["blocks"].as_array().ok_or("missing block rows")?;
    let envelopes = contexts["envelopes"]
        .as_array()
        .ok_or("missing envelopes")?;
    if rows.len() != 7 || envelopes.len() != rows.len() {
        return Err("missing or extra source envelope".to_owned());
    }
    let headers = contexts["headers"].as_array().ok_or("missing headers")?;
    let transactions = contexts["transactions"]
        .as_array()
        .ok_or("missing transactions")?;
    let responses = contexts["responses"]
        .as_array()
        .ok_or("missing responses")?;
    if headers.len() > 65_536 || transactions.len() > 65_536 || responses.len() > 65_536 {
        return Err("source row bound exceeded".to_owned());
    }
    let by_url: HashMap<&str, &Value> = responses
        .iter()
        .map(|response| Ok((field(response, "url")?, response)))
        .collect::<Result<_, String>>()?;
    if by_url.len() != responses.len() {
        return Err("duplicate source response URL".to_owned());
    }
    let mut checked_header_responses = HashMap::new();
    let mut selected = HashSet::new();
    for (row, envelope) in rows.iter().zip(envelopes) {
        if !purposes.contains(&field(row, "purpose")?) {
            continue;
        }
        selected.insert(field(row, "purpose")?);
        let network = field(row, "network")?;
        let block_hash = field(row, "hash")?;
        if field(envelope, "network")? != network
            || field(envelope, "hash")? != block_hash
            || envelope["height"] != row["height"]
            || field(row, "sha256")? != hex(&Sha256::digest(&decoded(field(row, "raw")?)?))
            || !field(row, "url")?.ends_with(&format!("/block/{block_hash}/raw"))
        {
            return Err("selected block or envelope source binding".to_owned());
        }
        let block = open_bitcoin_codec::parse_block(&decoded(field(row, "raw")?)?)
            .map_err(|error| format!("selected body decode: {error}"))?;
        if display(open_bitcoin_core::consensus::block_hash(&block.header).as_bytes()) != block_hash
            || block_merkle_root(&block.transactions)
                .map_err(|error| format!("selected merkle: {error}"))?
                .0
                != block.header.merkle_root
            || check_block_header(&block.header).is_err()
        {
            return Err("selected body identity, merkle or proof".to_owned());
        }
        let branch = envelope["candidate_history"]
            .as_array()
            .ok_or("missing branch")?;
        if branch.is_empty() || branch.len() > 4096 {
            return Err("invalid selected branch length".to_owned());
        }
        for name in branch {
            let hash = name.as_str().ok_or("branch hash")?;
            let source = headers
                .iter()
                .find(|header| header["network"] == network && header["hash"] == hash)
                .ok_or("selected branch header absent")?;
            let header = open_bitcoin_codec::parse_block_header(&decoded(field(source, "raw")?)?)
                .map_err(|error| format!("selected header decode: {error}"))?;
            if display(open_bitcoin_core::consensus::block_hash(&header).as_bytes()) != hash
                || check_block_header(&header).is_err()
            {
                return Err("selected header identity or proof".to_owned());
            }
            let url = field(source, "response")?;
            if !url.contains("/blocks/") {
                return Err("header response endpoint".to_owned());
            }
            if !checked_header_responses.contains_key(url) {
                let response = by_url.get(url).ok_or("header response missing")?;
                let raw = decoded(field(response, "raw")?)?;
                if hex(&Sha256::digest(&raw)) != field(response, "sha256")? {
                    return Err("header response digest".to_owned());
                }
                let listing: Value = serde_json::from_slice(&raw)
                    .map_err(|error| format!("header response JSON: {error}"))?;
                checked_header_responses.insert(url, listing);
            }
            let item = checked_header_responses[url]
                .as_array()
                .and_then(|items| items.iter().find(|item| item["id"] == hash))
                .ok_or("selected header absent from response")?;
            super::verify_selected_header_listing(item, source, &header, hash)?;
        }
        for source in envelope["external_utxos"]
            .as_array()
            .ok_or("missing external UTXOs")?
        {
            let txid = field(source, "txid")?;
            let tx = transactions
                .iter()
                .find(|tx| tx["network"] == network && tx["txid"] == txid)
                .ok_or("selected transaction absent")?;
            let url = field(tx, "response")?;
            if !url.ends_with(&format!("/tx/{txid}/raw")) {
                return Err("transaction response endpoint".to_owned());
            }
            let response = by_url.get(url).ok_or("transaction response missing")?;
            if field(response, "transaction")? != format!("{network}:{txid}")
                || field(response, "sha256")? != hex(&Sha256::digest(&decoded(field(tx, "raw")?)?))
            {
                return Err("transaction response digest or pointer".to_owned());
            }
        }
    }
    if !purposes.iter().all(|purpose| selected.contains(purpose)) {
        return Err("requested source purpose missing".to_owned());
    }
    Ok((manifest, blocks, contexts))
}

#[test]
fn phase167_historical_bip30_checked_source_accepts_exact_pinned_bytes() {
    // Arrange: immutable pinned bytes are checked before opening a store.
    let manifest = data("manifest.json", 16_384);
    let blocks = data("blocks.json", 4 * 1024 * 1024);
    let contexts = data("contexts.json", 77 * 1024 * 1024);
    // Act
    let checked = checked_corpus(
        &manifest,
        &blocks,
        &contexts,
        &[
            "bip30-repeat-91842",
            "bip30-original-91812",
            "bip30-repeat-91880",
            "bip30-original-91722",
        ],
    );
    // Assert: a valid source must earn a checked, fallible receipt.
    assert!(checked.is_ok(), "valid pinned corpus: {checked:?}");
}

#[test]
fn phase167_historical_bip30_checked_source_refuses_altered_body_context_and_envelope() {
    // Arrange: mutations also refresh the manifest digest, so structural proof is exercised.
    let manifest_bytes = data("manifest.json", 16_384);
    let blocks_bytes = data("blocks.json", 4 * 1024 * 1024);
    let contexts_bytes = data("contexts.json", 77 * 1024 * 1024);
    let selected = [
        "bip30-repeat-91842",
        "bip30-original-91812",
        "bip30-repeat-91880",
        "bip30-original-91722",
    ];
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest");
    let mut blocks: Value = serde_json::from_slice(&blocks_bytes).expect("blocks");
    let raw = blocks["blocks"][3]["raw"].as_str().expect("repeat raw");
    let altered = format!("{}{}", if &raw[..1] == "0" { "1" } else { "0" }, &raw[1..]);
    blocks["blocks"][3]["raw"] = Value::String(altered.clone());
    blocks["blocks"][3]["sha256"] = Value::String(hex(&Sha256::digest(&bytes(&altered))));
    let altered_blocks = serde_json::to_vec(&blocks).expect("altered body JSON");
    let mut body_manifest = manifest.clone();
    body_manifest["blocks_sha256"] = Value::String(hex(&Sha256::digest(&altered_blocks)));
    // Act and Assert: altered body identity is refused before any store is opened.
    assert!(
        checked_corpus(
            &serde_json::to_vec(&body_manifest).expect("manifest"),
            &altered_blocks,
            &contexts_bytes,
            &selected
        )
        .is_err()
    );
    eprintln!("CASE RG16-BIP30-SOURCE-ALTERED-BODY PASS");

    let mut contexts: Value = serde_json::from_slice(&contexts_bytes).expect("contexts");
    let parent = contexts["headers"]
        .as_array()
        .expect("headers")
        .iter()
        .find(|row| row["height"] == 91841 && row["network"] == "mainnet")
        .expect("selected parent header");
    let response_url = parent["response"]
        .as_str()
        .expect("response URL")
        .to_owned();
    let parent_hash = parent["hash"].as_str().expect("parent hash").to_owned();
    let response = contexts["responses"]
        .as_array_mut()
        .expect("responses")
        .iter_mut()
        .find(|row| row["url"] == response_url)
        .expect("selected response");
    let mut listing: Value =
        serde_json::from_slice(&bytes(response["raw"].as_str().expect("response raw")))
            .expect("selected header listing");
    let item = listing
        .as_array_mut()
        .expect("listing")
        .iter_mut()
        .find(|row| row["id"] == parent_hash)
        .expect("selected listed header");
    item["timestamp"] = Value::from(item["timestamp"].as_u64().expect("timestamp") + 1);
    let altered_response = serde_json::to_vec(&listing).expect("altered listing");
    response["raw"] = Value::String(hex(&altered_response));
    response["sha256"] = Value::String(hex(&Sha256::digest(&altered_response)));
    let listed_contexts = serde_json::to_vec(&contexts).expect("altered response context");
    let mut listed_manifest = manifest.clone();
    listed_manifest["contexts_sha256"] = Value::String(hex(&Sha256::digest(&listed_contexts)));
    let listed_error = checked_corpus(
        &serde_json::to_vec(&listed_manifest).expect("manifest"),
        &blocks_bytes,
        &listed_contexts,
        &selected,
    )
    .expect_err("changed listing refuses");
    assert!(listed_error.starts_with("selected header listing facts differ from 80-byte source"));
    assert!(listed_error.contains("timestamp"));
    contexts = serde_json::from_slice(&contexts_bytes).expect("fresh contexts");
    let header = contexts["headers"]
        .as_array_mut()
        .expect("headers")
        .iter_mut()
        .find(|row| row["height"] == 91841 && row["network"] == "mainnet")
        .expect("selected parent header");
    let raw = header["raw"].as_str().expect("header raw");
    header["raw"] = Value::String(format!(
        "{}{}",
        if &raw[..1] == "0" { "1" } else { "0" },
        &raw[1..]
    ));
    let altered_contexts = serde_json::to_vec(&contexts).expect("altered context JSON");
    let mut context_manifest = manifest.clone();
    context_manifest["contexts_sha256"] = Value::String(hex(&Sha256::digest(&altered_contexts)));
    assert!(
        checked_corpus(
            &serde_json::to_vec(&context_manifest).expect("manifest"),
            &blocks_bytes,
            &altered_contexts,
            &selected
        )
        .is_err()
    );
    eprintln!("CASE RG16-BIP30-SOURCE-ALTERED-CONTEXT PASS");

    contexts["envelopes"]
        .as_array_mut()
        .expect("envelopes")
        .remove(3);
    let missing_envelope = serde_json::to_vec(&contexts).expect("missing envelope JSON");
    let mut envelope_manifest = manifest;
    envelope_manifest["contexts_sha256"] = Value::String(hex(&Sha256::digest(&missing_envelope)));
    assert!(
        checked_corpus(
            &serde_json::to_vec(&envelope_manifest).expect("manifest"),
            &blocks_bytes,
            &missing_envelope,
            &selected
        )
        .is_err()
    );
    eprintln!("CASE RG16-BIP30-SOURCE-MISSING-ENVELOPE PASS");
    assert_eq!(data("manifest.json", 16_384), manifest_bytes);
    assert_eq!(data("blocks.json", 4 * 1024 * 1024), blocks_bytes);
    assert_eq!(data("contexts.json", 77 * 1024 * 1024), contexts_bytes);
}

/// Parse the bounded corpus once and materialize only requested exact purposes.
/// Task 2 can select both repeat and original rows without a second parser.
pub(super) fn load_cases(purposes: &[&str]) -> Vec<Case> {
    let manifest_bytes = data("manifest.json", 16_384);
    let blocks_bytes = data("blocks.json", 4 * 1024 * 1024);
    let contexts_bytes = data("contexts.json", 77 * 1024 * 1024);
    let (_, blocks, contexts) =
        checked_corpus(&manifest_bytes, &blocks_bytes, &contexts_bytes, purposes)
            .expect("fallible source and response proof before any store open");
    let raw_headers = contexts["headers"].as_array().expect("headers");
    assert!(raw_headers.len() <= 65_536);
    let mut by_hash = HashMap::with_capacity(raw_headers.len());
    for source in raw_headers {
        let network = chain(string(source, "network"));
        let header = open_bitcoin_codec::parse_block_header(&bytes(string(source, "raw")))
            .expect("80-byte header");
        assert_eq!(
            display(block_hash(&header).as_bytes()),
            string(source, "hash")
        );
        check_block_header(&header).expect("source header PoW");
        let row = HistoryHeader::new(network, number(source, "height") as u32, header);
        assert!(
            by_hash
                .insert(
                    (
                        string(source, "network").to_owned(),
                        string(source, "hash").to_owned()
                    ),
                    row
                )
                .is_none()
        );
    }
    let raw_transactions = contexts["transactions"].as_array().expect("transactions");
    assert!(raw_transactions.len() <= 65_536);
    let mut by_txid = HashMap::with_capacity(raw_transactions.len());
    for source in raw_transactions {
        let tx = open_bitcoin_codec::parse_transaction(&bytes(string(source, "raw")))
            .expect("source transaction");
        assert_eq!(
            display(transaction_txid(&tx).expect("source txid").as_bytes()),
            string(source, "txid")
        );
        assert!(
            by_txid
                .insert(
                    (
                        string(source, "network").to_owned(),
                        string(source, "txid").to_owned()
                    ),
                    tx
                )
                .is_none()
        );
    }
    let rows = blocks["blocks"].as_array().expect("block rows");
    let envelopes = contexts["envelopes"].as_array().expect("envelopes");
    assert_eq!(rows.len(), 7);
    assert_eq!(envelopes.len(), rows.len());
    rows.iter()
        .zip(envelopes)
        .filter(|(row, _)| purposes.contains(&string(row, "purpose")))
        .map(|(row, envelope)| {
            let network_name = string(row, "network");
            let network = chain(network_name);
            assert_eq!(string(envelope, "network"), network_name);
            assert_eq!(string(envelope, "hash"), string(row, "hash"));
            let raw = bytes(string(row, "raw"));
            assert_eq!(hex(&Sha256::digest(&raw)), string(row, "sha256"));
            let block = open_bitcoin_codec::parse_block(&raw).expect("exact serialized block");
            assert_eq!(
                display(block_hash(&block.header).as_bytes()),
                string(row, "hash")
            );
            assert_eq!(
                block_merkle_root(&block.transactions)
                    .expect("source merkle tree")
                    .0,
                block.header.merkle_root
            );
            check_block_header(&block.header).expect("candidate PoW");
            assert_eq!(
                display(block.header.previous_block_hash.as_bytes()),
                string(envelope, "parent")
            );
            let height = number(row, "height") as u32;
            assert_eq!(number(envelope, "height") as u32, height);
            let named = envelope["candidate_history"]
                .as_array()
                .expect("branch names");
            assert!(named.len() <= 4096);
            let branch: Vec<_> = named
                .iter()
                .rev()
                .map(|name| {
                    by_hash
                        .get(&(
                            network_name.to_owned(),
                            name.as_str().expect("branch hash").to_owned(),
                        ))
                        .expect("source branch header")
                        .clone()
                })
                .collect();
            for (actual, expected) in branch.iter().zip(named.iter().rev()) {
                assert_eq!(
                    display(actual.block_hash.as_bytes()),
                    expected.as_str().expect("branch hash")
                );
            }
            assert!(
                branch
                    .windows(2)
                    .all(
                        |pair| pair[0].header.previous_block_hash == pair[1].block_hash
                            && pair[0].height == pair[1].height + 1
                    )
            );
            assert_eq!(branch[0].height + 1, height);
            let maybe_seed = envelope["bip34"]["hash"].as_str().map(|anchor| {
                by_hash
                    .get(&(network_name.to_owned(), anchor.to_owned()))
                    .expect("source BIP34 anchor")
                    .clone()
            });
            let history = CandidateHistory::check_seeded(
                network.spec().rules(),
                block.header.clone(),
                branch[0].clone(),
                branch[1..].to_vec(),
                maybe_seed,
            )
            .expect("checked historical rules");
            assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
            let mut coins = HashMap::new();
            for source in envelope["external_utxos"]
                .as_array()
                .expect("complete required coins")
            {
                let txid_display = string(source, "txid");
                let tx = by_txid
                    .get(&(network_name.to_owned(), txid_display.to_owned()))
                    .expect("independent previous transaction");
                let vout = number(source, "vout") as usize;
                let output = tx
                    .outputs
                    .get(vout)
                    .expect("source previous output")
                    .clone();
                assert_eq!(output.value.to_sats().to_string(), string(source, "value"));
                assert_eq!(
                    hex(output.script_pubkey.as_bytes()),
                    string(source, "script")
                );
                let outpoint = OutPoint {
                    txid: Txid::from_byte_array(hash(txid_display)),
                    vout: vout as u32,
                };
                let coin = Coin {
                    output,
                    is_coinbase: source["coinbase"].as_bool().expect("coinbase fact"),
                    created_height: number(source, "created_height") as u32,
                    created_median_time_past: source["created_mtp"].as_i64().expect("creating MTP"),
                };
                assert!(coins.insert(outpoint, coin).is_none());
            }
            let snapshot = ChainstateSnapshot::new(positions(&branch), coins, HashMap::new());
            assert_eq!(
                snapshot.tip().expect("parent").block_hash,
                block.header.previous_block_hash
            );
            let overwrite_original = envelope["overwrite_original"]
                .as_array()
                .expect("source original links")
                .iter()
                .map(|item| item.as_str().expect("original hash").to_owned())
                .collect();
            Case {
                block,
                history,
                snapshot,
                now: i64::from(branch[0].header.time),
                purpose: string(row, "purpose").to_owned(),
                overwrite_original,
            }
        })
        .collect()
}

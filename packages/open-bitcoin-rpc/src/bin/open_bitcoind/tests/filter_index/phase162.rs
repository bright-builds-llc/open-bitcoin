// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Configured daemon evidence from the shared normal-maturity accepted recipe.

use open_bitcoin_node::core as core_types;

pub(crate) mod recipe {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/recipe.rs"
    ));
}

mod process {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/process.rs"
    ));
}

#[path = "phase162/fixtures.rs"]
pub(crate) mod fixtures;

#[path = "phase162/lifecycle.rs"]
pub(crate) mod lifecycle;

#[path = "phase162/retention.rs"]
pub(crate) mod retention;

#[path = "phase162/rpc.rs"]
pub(crate) mod rpc;

#[path = "phase162/inbound.rs"]
pub(crate) mod inbound;

#[path = "phase162/restrictions.rs"]
pub(crate) mod restrictions;

#[path = "phase162/outbound.rs"]
pub(crate) mod outbound;

pub(crate) fn oracle_case<'a>(
    fixture: &'a fixtures::Phase162Fixture,
    name: &str,
) -> &'a serde_json::Value {
    fixture.provenance.oracle["cases"]
        .as_array()
        .expect("independent raw-input cases")
        .iter()
        .find(|row| row["name"] == name)
        .expect("independent named ancestry")
}

pub(crate) fn decode_hex(raw: &str) -> Vec<u8> {
    assert_eq!(raw.len() % 2, 0);
    raw.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16).expect("hex byte")
        })
        .collect()
}

pub(crate) fn oracle_hash(row: &serde_json::Value) -> core_types::primitives::BlockHash {
    core_types::primitives::BlockHash::from_byte_array(
        decode_hex(row["block_hash_raw"].as_str().expect("raw hash"))
            .try_into()
            .expect("32-byte raw hash"),
    )
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn independent_oracle(recipe: &recipe::Recipe) -> serde_json::Value {
    use std::process::Command;
    let branch = recipe.fork(998, 4, 0x62);
    let raw = |name: String, previous: String, block: &core_types::primitives::Block| {
        serde_json::json!({"name":name, "previous_name":previous,
            "raw_block":hex(&core_types::codec::encode_block(block).expect("raw protocol block"))})
    };
    let mut blocks: Vec<_> = recipe
        .blocks
        .iter()
        .enumerate()
        .map(|(height, block)| {
            raw(
                format!("active-{height}"),
                if height == 0 {
                    String::new()
                } else {
                    format!("active-{}", height - 1)
                },
                block,
            )
        })
        .collect();
    for (offset, block) in branch.iter().enumerate() {
        blocks.push(raw(
            format!("fork-{}", 999 + offset),
            if offset == 0 {
                "active-998".to_owned()
            } else {
                format!("fork-{}", 998 + offset)
            },
            block,
        ));
    }
    let input = serde_json::json!({"recipe_id":recipe::RECIPE_ID,"network_magic":"fabfb5da","blocks":blocks});
    let ordered_digest = hex(&core_types::consensus::crypto::Sha256::digest(
        &serde_json::to_vec(&input["blocks"]).expect("ordered raw blocks"),
    ));
    let mut command = Command::new("bun");
    command
        .arg("scripts/phase162-filter-oracle.ts")
        .current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let output = process::run(
        &mut command,
        serde_json::to_vec(&input).expect("raw JSON"),
        process::ORACLE_LIMITS,
    )
    .expect("bounded fixed argv oracle completion");
    assert!(output.reaped, "oracle child must be reaped");
    assert!(output.status.success());
    let result: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("raw evidence JSON");
    assert_eq!(result["recipe_id"], recipe::RECIPE_ID);
    assert_eq!(result["ordered_input_digest"], ordered_digest);
    for (height, spent) in recipe.spent_scripts.iter().enumerate() {
        assert_eq!(
            result["cases"][height]["spent"],
            serde_json::json!(spent.iter().map(|script| hex(script)).collect::<Vec<_>>())
        );
    }
    for (offset, spent) in recipe.fork_spent_scripts(998, &branch).iter().enumerate() {
        assert_eq!(
            result["cases"][1001 + offset]["spent"],
            serde_json::json!(spent.iter().map(|script| hex(script)).collect::<Vec<_>>())
        );
    }
    result
}

#[test]
fn phase162_daemon_fixture_initial_partial_prefix_and_all_handle_close() {
    // Arrange / Act
    let fixture = fixtures::Phase162Fixture::seed();
    // Assert
    fixture.assert_initial_prefix();
    assert!(open_bitcoin_node::FjallNodeStore::open(&fixture.path).is_err());
    let handoff = fixture.close();
    drop(open_bitcoin_node::FjallNodeStore::open(&handoff.path).expect("all handles closed"));
    std::fs::remove_dir_all(&handoff.path).expect("closed fixture cleanup");
    assert!(!handoff.path.exists());
}

use std::ffi::OsString;

use open_bitcoin_rpc::method::{
    GetBalancesResponse, GetBlockchainInfoResponse, GetNetworkInfoResponse, GetWalletInfoResponse,
    RequestParameters, SupportedMethod, WalletBalanceDetails,
};

use crate::args::{CliCommand, ColorSetting, parse_cli_args};

use super::{GetInfoOutputMode, GetInfoSnapshot, build_getinfo_batch, render_getinfo};

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[test]
fn getinfo_builds_expected_four_call_batch_and_rejects_extra_args() {
    // Arrange
    let parsed = parse_cli_args(&[os("-getinfo")], "").expect("plain getinfo args");
    let extra_args =
        parse_cli_args(&[os("-getinfo"), os("unexpected")], "").expect("getinfo args parse");
    let CliCommand::GetInfo(command) = parsed.command else {
        panic!("expected getinfo command");
    };
    let CliCommand::GetInfo(extra_command) = extra_args.command else {
        panic!("expected getinfo command");
    };

    // Act
    let batch = build_getinfo_batch(&command).expect("batch");
    let extra_args_error = build_getinfo_batch(&extra_command).expect_err("extra args must fail");

    // Assert
    assert_eq!(batch.output_mode, GetInfoOutputMode::Human);
    assert_eq!(batch.color, ColorSetting::Auto);
    assert_eq!(
        batch.calls,
        vec![
            super::GetInfoBatchCall {
                method: SupportedMethod::GetNetworkInfo,
                params: RequestParameters::None,
            },
            super::GetInfoBatchCall {
                method: SupportedMethod::GetBlockchainInfo,
                params: RequestParameters::None,
            },
            super::GetInfoBatchCall {
                method: SupportedMethod::GetWalletInfo,
                params: RequestParameters::None,
            },
            super::GetInfoBatchCall {
                method: SupportedMethod::GetBalances,
                params: RequestParameters::None,
            },
        ]
    );
    assert_eq!(
        extra_args_error.to_string(),
        "-getinfo takes no arguments except --json",
    );
}

#[test]
fn getinfo_json_mode_is_stable_for_automation() {
    // Arrange
    let parsed = parse_cli_args(&[os("-getinfo"), os("--json")], "").expect("json getinfo args");
    let CliCommand::GetInfo(command) = parsed.command else {
        panic!("expected getinfo command");
    };
    let batch = build_getinfo_batch(&command).expect("json batch");
    let snapshot = GetInfoSnapshot {
        network: GetNetworkInfoResponse {
            version: 293_000,
            subversion: "/OpenBitcoin:0.1.0/".to_string(),
            protocolversion: 70_016,
            localservices: "NETWORK".to_string(),
            localrelay: true,
            connections: 5,
            connections_in: 2,
            connections_out: 3,
            relayfee: 1_000,
            incrementalfee: 1_000,
            warnings: Vec::new(),
        },
        blockchain: GetBlockchainInfoResponse {
            chain: "regtest".to_string(),
            blocks: 144,
            headers: 144,
            maybe_best_block_hash: Some("00".repeat(32)),
            maybe_median_time_past: Some(1_714_007_000),
            verificationprogress: 0.995,
            initialblockdownload: false,
            warnings: Vec::new(),
        },
        maybe_wallet: Some(GetWalletInfoResponse {
            network: "regtest".to_string(),
            descriptor_count: 4,
            utxo_count: 2,
            maybe_tip_height: Some(144),
            maybe_tip_median_time_past: Some(1_714_007_000),
        }),
        maybe_balances: Some(GetBalancesResponse {
            mine: WalletBalanceDetails {
                trusted_sats: 125_000_000,
                untrusted_pending_sats: 0,
                immature_sats: 0,
            },
        }),
    };

    // Act
    let json_output =
        render_getinfo(&snapshot, batch.output_mode, batch.color).expect("json output");
    let human_output = render_getinfo(&snapshot, GetInfoOutputMode::Human, ColorSetting::Never)
        .expect("human output");

    // Assert
    assert_eq!(batch.output_mode, GetInfoOutputMode::Json);
    assert_eq!(
        json_output,
        r#"{
  "network": {
    "version": 293000,
    "subversion": "/OpenBitcoin:0.1.0/",
    "protocolversion": 70016,
    "localservices": "NETWORK",
    "localrelay": true,
    "connections": 5,
    "connections_in": 2,
    "connections_out": 3,
    "relayfee": 1000,
    "incrementalfee": 1000,
    "warnings": []
  },
  "blockchain": {
    "chain": "regtest",
    "blocks": 144,
    "headers": 144,
    "bestblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
    "mediantime": 1714007000,
    "verificationprogress": 0.995,
    "initialblockdownload": false,
    "warnings": []
  },
  "wallet": {
    "network": "regtest",
    "descriptor_count": 4,
    "utxo_count": 2,
    "maybe_tip_height": 144,
    "maybe_tip_median_time_past": 1714007000
  },
  "balances": {
    "mine": {
      "trusted_sats": 125000000,
      "untrusted_pending_sats": 0,
      "immature_sats": 0
    }
  }
}"#
    );
    assert_eq!(
        human_output,
        "Chain: regtest\nBlocks: 144\nHeaders: 144\nVerification progress: 99.5000%\n\nNetwork: in 2, out 3, total 5\nVersion: 293000\nRelay fee (sats/kvB): 1000\n\nWallet network: regtest\nDescriptor count: 4\nTracked UTXOs: 2\nWallet tip height: 144\nWallet tip median time past: 1714007000\n\nTrusted balance (sats): 125000000\n\nWarnings: (none)"
    );
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/settings.cpp

use super::*;
use crate::config::blockfilter::{
    BasicFilterIndexSetting as Setting, FilterOptionValue as Value, resolve_filter_index,
};

fn values(strings: &[&str]) -> Vec<Value> {
    strings
        .iter()
        .map(|value| Value::String((*value).into()))
        .collect()
}

fn assert_resolution(cli: &[&str], network: &[&str], defaults: &[&str], expected: Setting) {
    // Arrange
    let (cli, network, defaults) = (values(cli), values(network), values(defaults));
    // Act
    let result = resolve_filter_index(&cli, &network, &defaults);
    // Assert
    assert_eq!(result, Ok(expected));
    assert_eq!(expected.is_enabled(), expected == Setting::Basic);
    assert_eq!(expected.is_explicit(), expected != Setting::Unspecified);
}

fn assert_refusal(cli: &[&str], network: &[&str], defaults: &[&str], rejected: &str) {
    // Arrange
    let (cli, network, defaults) = (values(cli), values(network), values(defaults));
    // Act
    let result = resolve_filter_index(&cli, &network, &defaults);
    // Assert
    let error = result.expect_err("selected named list must refuse invalid entries");
    assert!(error.to_string().contains(rejected), "{error}");
}

#[test]
fn phase157_blockfilter_single_forms() {
    assert_resolution(&[], &[], &[], Setting::Unspecified);
    for value in ["", "1", "basic"] {
        assert_resolution(&[value], &[], &[], Setting::Basic);
        assert_resolution(&[], &[], &[value], Setting::Basic);
    }
    assert_resolution(&["0"], &[], &[], Setting::Disabled);
    assert_resolution(&[], &[], &["0"], Setting::Disabled);
}

#[test]
fn phase157_blockfilter_cli_last_scalar_and_named_list() {
    for scalar in ["0", "1", ""] {
        assert_refusal(&[scalar, "basic"], &[], &[], scalar);
        let expected = if scalar == "0" {
            Setting::Disabled
        } else {
            Setting::Basic
        };
        assert_resolution(&["basic", scalar], &[], &[], expected);
    }
    assert_resolution(&["basic", "basic"], &[], &[], Setting::Basic);
}

#[test]
fn phase157_blockfilter_config_first_scalar_and_named_list() {
    for scalar in ["0", "1", ""] {
        let expected = if scalar == "0" {
            Setting::Disabled
        } else {
            Setting::Basic
        };
        assert_resolution(&[], &[], &[scalar, "basic"], expected);
        assert_refusal(&[], &[], &["basic", scalar], scalar);
    }
    assert_resolution(&[], &[], &["basic", "basic"], Setting::Basic);
}

#[test]
fn phase157_blockfilter_named_mode_merges_config_sources() {
    for value in ["0", "unknown"] {
        assert_refusal(&["basic"], &[], &[value], value);
    }
    assert_resolution(&["1"], &[], &["unknown"], Setting::Basic);
    assert_refusal(&["unknown"], &[], &["1"], "unknown");
    assert_resolution(&[], &["0"], &["basic"], Setting::Disabled);
    assert_refusal(&[], &["basic"], &["0"], "0");
    assert_resolution(&["basic"], &["basic"], &["basic"], Setting::Basic);
}

#[test]
fn phase157_blockfilter_negation_resets_and_revives_config() {
    // Arrange
    let negated = Value::Negation(false);
    let basic = Value::String("basic".into());
    let bad = Value::String("unknown".into());
    for cli in [vec![negated.clone()], vec![basic.clone(), negated.clone()]] {
        // Act
        let result = resolve_filter_index(&cli, &[], std::slice::from_ref(&bad));
        // Assert
        assert_eq!(result, Ok(Setting::Disabled));
    }
    // Arrange
    let revived = vec![bad, negated, basic.clone()];
    // Act
    let result = resolve_filter_index(&revived, &[], &[basic]);
    let refused = resolve_filter_index(&revived, &[], &values(&["0"]));
    // Assert
    assert_eq!(result, Ok(Setting::Basic));
    assert!(refused.is_err());
}

#[test]
fn phase157_blockfilter_config_negation_and_double_negative() {
    // Arrange
    let reset_config = vec![
        Value::String("unknown".into()),
        Value::Negation(false),
        Value::String("basic".into()),
    ];
    // Act
    let reset = resolve_filter_index(&[], &[], &reset_config);
    let double_negative =
        resolve_filter_index(&[Value::Negation(true)], &[], &values(&["unknown"]));
    // Assert
    assert_eq!(reset, Ok(Setting::Basic));
    assert_eq!(double_negative, Ok(Setting::Basic));
}

#[test]
fn phase157_blockfilter_scalar_bypasses_stale_bad_strings() {
    for value in ["unknown", "v0", "2"] {
        assert_resolution(&[value, "0"], &[], &[], Setting::Disabled);
        assert_resolution(&[value, "1"], &[], &[], Setting::Basic);
        assert_resolution(&[value, ""], &[], &[], Setting::Basic);
        assert_resolution(&[], &[], &["1", value], Setting::Basic);
    }
}

#[test]
fn phase157_blockfilter_excluded_and_bounded_unknown_errors() {
    for value in ["v0", "2"] {
        assert_refusal(&[value], &[], &[], "BASIC-only");
    }
    // Arrange
    let payload = "x\n秘密".repeat(1_000);
    // Act
    let error =
        resolve_filter_index(&[Value::String(payload)], &[], &[]).expect_err("unknown type");
    // Assert
    assert!(error.to_string().len() < 300);
    assert!(!error.to_string().contains('\n'));
    assert!(!error.to_string().contains("秘密"));
}

fn load_case(cli: &[&str], conf: &str, expected: Setting) {
    // Arrange
    let directory = TestDirectory::new("phase157-blockfilter");
    fs::write(directory.child("bitcoin.conf"), conf).expect("config fixture");
    let cli = cli.iter().map(|value| os(value)).collect::<Vec<_>>();
    // Act
    let config =
        load_runtime_config_for_args(&cli, &directory.path).expect("supported filter setting");
    // Assert
    assert_eq!(config.block_filter_index, expected);
    assert_activation_unchanged(&config);
}

fn assert_activation_unchanged(config: &RuntimeConfig) {
    let neutral = RuntimeConfig::default();
    assert_eq!(config.sync, neutral.sync);
    assert_eq!(config.inbound, neutral.inbound);
    assert_eq!(config.relay, neutral.relay);
    assert_eq!(config.block_serving, neutral.block_serving);
    assert_eq!(config.prune_mode, neutral.prune_mode);
}

fn loader_refusal(cli: &[&str], conf: &str, category: &str) {
    // Arrange
    let directory = TestDirectory::new("phase157-refusal");
    fs::write(directory.child("bitcoin.conf"), conf).expect("config fixture");
    let cli = cli.iter().map(|value| os(value)).collect::<Vec<_>>();
    // Act
    let result = load_runtime_config_for_args(&cli, &directory.path);
    // Assert
    let error = result
        .expect_err("invalid selected filter setting")
        .to_string();
    assert!(error.contains(category), "{error}");
    assert!(error.len() < 300);
}

#[test]
fn phase157_blockfilter_loader_supported_cli_syntax_stays_offline() {
    load_case(&[], "", Setting::Unspecified);
    for prefix in ["-", "--"] {
        load_case(&[&format!("{prefix}blockfilterindex")], "", Setting::Basic);
        for value in ["", "1", "basic", "0"] {
            let expected = if value == "0" {
                Setting::Disabled
            } else {
                Setting::Basic
            };
            load_case(
                &[&format!("{prefix}blockfilterindex={value}")],
                "",
                expected,
            );
        }
    }
    loader_refusal(
        &["-blockfilterindex", "basic"],
        "",
        "Invalid parameter basic",
    );
}

#[test]
fn phase157_blockfilter_loader_repeated_cli_matches_scalar_list_table() {
    for scalar in ["0", "1", ""] {
        let arg = format!("-blockfilterindex={scalar}");
        loader_refusal(&[&arg, "-blockfilterindex=basic"], "", "unknown");
        let expected = if scalar == "0" {
            Setting::Disabled
        } else {
            Setting::Basic
        };
        load_case(&["-blockfilterindex=basic", &arg], "", expected);
    }
    load_case(
        &["-blockfilterindex=basic", "-blockfilterindex=basic"],
        "",
        Setting::Basic,
    );
}

#[test]
fn phase157_blockfilter_loader_repeated_config_matches_first_scalar_table() {
    for scalar in ["0", "1", ""] {
        let conf = format!("blockfilterindex={scalar}\nblockfilterindex=basic\n");
        let expected = if scalar == "0" {
            Setting::Disabled
        } else {
            Setting::Basic
        };
        load_case(&[], &conf, expected);
        loader_refusal(
            &[],
            &format!("blockfilterindex=basic\nblockfilterindex={scalar}\n"),
            "unknown",
        );
    }
    load_case(
        &[],
        "blockfilterindex=basic\nblockfilterindex=basic\n",
        Setting::Basic,
    );
}

#[test]
fn phase157_blockfilter_loader_cli_named_mode_includes_lower_priority_config() {
    for value in ["0", "unknown"] {
        loader_refusal(
            &["-blockfilterindex=basic"],
            &format!("blockfilterindex={value}\n"),
            "unknown",
        );
    }
    load_case(
        &["-blockfilterindex=1"],
        "blockfilterindex=unknown\n",
        Setting::Basic,
    );
    loader_refusal(
        &["-blockfilterindex=unknown"],
        "blockfilterindex=1\n",
        "unknown",
    );
}

#[test]
fn phase157_blockfilter_loader_active_network_precedes_default_and_ignores_inactive() {
    load_case(
        &["-regtest"],
        "blockfilterindex=basic\n[regtest]\nblockfilterindex=0\n",
        Setting::Disabled,
    );
    loader_refusal(
        &["-regtest"],
        "blockfilterindex=0\n[regtest]\nblockfilterindex=basic\n",
        "unknown",
    );
    load_case(
        &["-regtest"],
        "[signet]\nblockfilterindex=unknown\n[regtest]\nblockfilterindex=basic\n",
        Setting::Basic,
    );
    load_case(
        &[],
        "regtest=1\nregtest.blockfilterindex=basic\nmain.blockfilterindex=v0\n",
        Setting::Basic,
    );
    load_case(
        &["-blockfilterindex=1", "-regtest"],
        "blockfilterindex=unknown\n[regtest]\nblockfilterindex=unknown\n",
        Setting::Basic,
    );
}

#[test]
fn phase157_blockfilter_loader_negation_reset_and_revival() {
    load_case(
        &["-noblockfilterindex"],
        "blockfilterindex=unknown\n",
        Setting::Disabled,
    );
    load_case(
        &["-blockfilterindex=basic", "-noblockfilterindex"],
        "",
        Setting::Disabled,
    );
    load_case(
        &[
            "-blockfilterindex=unknown",
            "-noblockfilterindex",
            "-blockfilterindex=basic",
        ],
        "blockfilterindex=basic\n",
        Setting::Basic,
    );
    loader_refusal(
        &["-noblockfilterindex", "-blockfilterindex=basic"],
        "blockfilterindex=0\n",
        "unknown",
    );
    load_case(
        &[],
        "blockfilterindex=unknown\nnoblockfilterindex=1\nblockfilterindex=basic\n",
        Setting::Basic,
    );
    load_case(&[], "noblockfilterindex=1\n", Setting::Disabled);
    load_case(&[], "noblockfilterindex=0\n", Setting::Basic);
}

#[test]
fn phase157_blockfilter_loader_double_negative_uses_pinned_integer_prefix() {
    for value in [
        "0", "false", "true", "unknown", "+-1", "-+1", " 0junk", "000",
    ] {
        load_case(
            &[&format!("-noblockfilterindex={value}")],
            "blockfilterindex=unknown\n",
            Setting::Basic,
        );
    }
    for value in ["", "1", "-1", "+2tail", " 0001junk", "99999999999999999999"] {
        load_case(
            &[&format!("-noblockfilterindex={value}")],
            "blockfilterindex=unknown\n",
            Setting::Disabled,
        );
    }
}

#[test]
fn phase157_blockfilter_loader_includeconf_preserves_parent_then_include_order() {
    // Arrange
    let directory = TestDirectory::new("phase157-include");
    fs::write(
        directory.child("bitcoin.conf"),
        "includeconf=extra.conf\nblockfilterindex=1\n",
    )
    .expect("parent");
    fs::write(directory.child("extra.conf"), "blockfilterindex=unknown\n").expect("include");
    // Act
    let config = load_runtime_config_for_args(&[], &directory.path).expect("first parent scalar");
    // Assert
    assert_eq!(config.block_filter_index, Setting::Basic);
    assert_activation_unchanged(&config);
    // Arrange
    fs::write(
        directory.child("bitcoin.conf"),
        "includeconf=extra.conf\nblockfilterindex=basic\n",
    )
    .expect("parent named");
    // Act
    let result = load_runtime_config_for_args(&[], &directory.path);
    // Assert
    assert!(
        result
            .expect_err("merged included unknown")
            .to_string()
            .contains("unknown")
    );
    // Arrange
    fs::write(
        directory.child("extra.conf"),
        "noblockfilterindex=1\nblockfilterindex=basic\n",
    )
    .expect("reset include");
    // Act
    let config =
        load_runtime_config_for_args(&[], &directory.path).expect("reset revives include basic");
    // Assert
    assert_eq!(config.block_filter_index, Setting::Basic);
    assert_activation_unchanged(&config);
}

#[test]
fn phase157_blockfilter_loader_preserves_other_explicit_controls() {
    // Arrange
    let directory = TestDirectory::new("phase157-policy");
    fs::write(directory.child("open-bitcoin.jsonc"), "{\"prune\":1}")
        .expect("manual prune fixture");
    let mut args = vec![
        cli_arg("openbitcoinconf", &directory.child("open-bitcoin.jsonc")),
        os("-rpcuser=operator"),
        os("-rpcpassword=test-only-secret"),
        os("-rpcport=19001"),
        os("-openbitcoinrelay=1"),
    ];
    let mut expected =
        load_runtime_config_for_args(&args, &directory.path).expect("explicit policies");
    args.push(os("-blockfilterindex=basic"));
    expected.block_filter_index = Setting::Basic;
    // Act
    let config =
        load_runtime_config_for_args(&args, &directory.path).expect("index with explicit policies");
    // Assert
    assert_eq!(config, expected);
}

#[test]
fn phase157_blockfilter_loader_excluded_and_unknown_diagnostics_are_redacted() {
    for value in ["v0", "2"] {
        loader_refusal(&[&format!("-blockfilterindex={value}")], "", "BASIC-only");
        loader_refusal(&[], &format!("blockfilterindex={value}\n"), "BASIC-only");
        load_case(
            &[&format!("-blockfilterindex={value}"), "-blockfilterindex=0"],
            "",
            Setting::Disabled,
        );
    }
    // Arrange
    let secret = "credential-not-for-diagnostics".repeat(1_000);
    let directory = TestDirectory::new("phase157-redacted");
    fs::write(
        directory.child("bitcoin.conf"),
        format!("blockfilterindex={secret}\n"),
    )
    .expect("unknown fixture");
    // Act
    let error = load_runtime_config_for_args(&[], &directory.path)
        .expect_err("unknown type")
        .to_string();
    // Assert
    assert!(error.len() < 300);
    assert!(!error.contains("credential-not-for-diagnostics"));
    assert!(!error.contains(directory.path.to_string_lossy().as_ref()));
}

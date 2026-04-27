// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{path::PathBuf, time::Duration};

use super::{
    StatusCollectorInput, StatusDetectionEvidence, StatusLiveRpcAdapterInput, StatusRenderMode,
    StatusRequest, StatusRpcAuthSource,
};
use crate::operator::{
    NetworkSelection,
    config::{OperatorConfigResolution, OperatorConfigSource},
};

#[test]
fn status_request_defines_render_mode_without_snapshot_dependency() {
    // Act
    let request = StatusRequest {
        render_mode: StatusRenderMode::Json,
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin.jsonc")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        include_live_rpc: true,
        no_color: true,
    };

    // Assert
    assert_eq!(request.render_mode, StatusRenderMode::Json);
    assert!(request.include_live_rpc);
    assert!(request.no_color);
}

#[test]
fn status_collector_input_keeps_rpc_config_and_detection_evidence_typed() {
    // Arrange
    let config_resolution = OperatorConfigResolution {
        ordered_sources: OperatorConfigSource::ordered().to_vec(),
        path_reports: Vec::new(),
        maybe_config_path: None,
        maybe_data_dir: None,
        maybe_network: Some(NetworkSelection::Regtest),
    };
    let request = StatusRequest {
        render_mode: StatusRenderMode::Human,
        maybe_config_path: None,
        maybe_data_dir: None,
        maybe_network: Some(NetworkSelection::Regtest),
        include_live_rpc: true,
        no_color: false,
    };

    // Act
    let input = StatusCollectorInput {
        request,
        config_resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
        },
        maybe_live_rpc: Some(StatusLiveRpcAdapterInput {
            endpoint: "http://127.0.0.1:8332".to_string(),
            auth_source: StatusRpcAuthSource::CookieFile {
                path: PathBuf::from("/tmp/.cookie"),
            },
            timeout: Duration::from_secs(2),
        }),
    };

    // Assert
    assert_eq!(input.request.render_mode, StatusRenderMode::Human);
    assert!(input.maybe_live_rpc.is_some());
    assert!(input.detection_evidence.detected_installations.is_empty());
}

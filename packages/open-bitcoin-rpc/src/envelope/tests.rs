use super::{JsonRpcId, JsonRpcVersion, RpcErrorEnvelope, RpcRequestEnvelope, RpcSuccessEnvelope};
use crate::error::{RpcErrorCode, RpcErrorDetail};

#[test]
fn request_and_response_envelopes_preserve_v2_metadata() {
    // Arrange
    let request = RpcRequestEnvelope {
        jsonrpc: Some(JsonRpcVersion::V2),
        method: "getnetworkinfo".to_string(),
        params: Vec::<String>::new(),
        id: Some(JsonRpcId::Number(7)),
    };

    // Act
    let success = RpcSuccessEnvelope {
        jsonrpc: JsonRpcVersion::V2,
        result: "ok".to_string(),
        id: Some(JsonRpcId::Number(7)),
    };
    let error = RpcErrorEnvelope {
        jsonrpc: JsonRpcVersion::V2,
        error: RpcErrorDetail::new(RpcErrorCode::MethodNotFound, "missing"),
        id: Some(JsonRpcId::Number(7)),
    };

    // Assert
    assert_eq!(request.jsonrpc, Some(JsonRpcVersion::V2));
    assert_eq!(success.id, Some(JsonRpcId::Number(7)));
    assert_eq!(error.error.code, RpcErrorCode::MethodNotFound);
}

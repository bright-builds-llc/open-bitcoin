use std::fmt;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use ureq::Agent;

pub trait HarnessTarget {
    fn name(&self) -> &str;

    fn request(&mut self, method: &str, params: Value) -> Result<Value, HarnessError>;
}

#[derive(Debug)]
pub enum HarnessError {
    Transport(String),
    HttpStatus(u16),
    InvalidResponse(String),
}

impl fmt::Display for HarnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(message) => write!(f, "transport error: {message}"),
            Self::HttpStatus(status) => write!(f, "unexpected HTTP status: {status}"),
            Self::InvalidResponse(message) => write!(f, "invalid response: {message}"),
        }
    }
}

impl std::error::Error for HarnessError {}

pub struct RpcHttpTarget {
    name: String,
    endpoint_url: String,
    authorization_header: String,
    agent: Agent,
    next_id: i64,
}

impl RpcHttpTarget {
    pub fn new(
        name: impl Into<String>,
        rpc_addr: impl Into<String>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Self {
        let rpc_addr = rpc_addr.into();
        let endpoint_url = if rpc_addr.starts_with("http://") || rpc_addr.starts_with("https://") {
            rpc_addr
        } else {
            format!("http://{rpc_addr}")
        };
        Self {
            name: name.into(),
            endpoint_url,
            authorization_header: basic_auth_header(username.as_ref(), password.as_ref()),
            agent: Agent::new_with_config(
                Agent::config_builder().http_status_as_error(false).build(),
            ),
            next_id: 1,
        }
    }
}

impl HarnessTarget for RpcHttpTarget {
    fn name(&self) -> &str {
        &self.name
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value, HarnessError> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": self.next_id,
        });
        self.next_id += 1;

        let response = self
            .agent
            .post(&self.endpoint_url)
            .header("Authorization", &self.authorization_header)
            .send_json(&request)
            .map_err(|error| HarnessError::Transport(error.to_string()))?;
        let status = response.status().as_u16();
        if status != 200 {
            return Err(HarnessError::HttpStatus(status));
        }

        response
            .into_body()
            .read_json()
            .map_err(|error| HarnessError::InvalidResponse(error.to_string()))
    }
}

fn basic_auth_header(username: &str, password: &str) -> String {
    let encoded = STANDARD.encode(format!("{username}:{password}"));
    format!("Basic {encoded}")
}

#[cfg(test)]
mod tests {
    use super::basic_auth_header;

    #[test]
    fn basic_auth_header_encodes_credentials() {
        // Arrange
        let username = "alice";
        let password = "secret";

        // Act
        let header = basic_auth_header(username, password);

        // Assert
        assert_eq!(header, "Basic YWxpY2U6c2VjcmV0");
    }
}

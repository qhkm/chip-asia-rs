use crate::error::ChipError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::time::Duration;

pub struct ChipClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
}

impl ChipClient {
    pub fn builder() -> ChipClientBuilder {
        ChipClientBuilder::default()
    }
}

#[derive(Default)]
pub struct ChipClientBuilder {
    base_url: Option<String>,
    token: Option<String>,
    timeout: Option<Duration>,
}

impl ChipClientBuilder {
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.trim_end_matches('/').to_string());
        self
    }

    pub fn bearer_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn build(self) -> Result<ChipClient, ChipError> {
        let base_url = self
            .base_url
            .ok_or_else(|| ChipError::Config("base_url is required".into()))?;
        let token = self
            .token
            .ok_or_else(|| ChipError::Config("bearer_token is required".into()))?;

        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| ChipError::Config(format!("Invalid token: {}", e)))?;
        headers.insert(AUTHORIZATION, auth_value);

        let mut client_builder = reqwest::Client::builder().default_headers(headers);
        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        } else {
            client_builder = client_builder.timeout(Duration::from_secs(60));
        }

        let http = client_builder
            .build()
            .map_err(|e| ChipError::Config(format!("Failed to build HTTP client: {}", e)))?;
        Ok(ChipClient { http, base_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_client() {
        let client = ChipClient::builder()
            .base_url("https://gate.chip-in.asia/api/v1")
            .bearer_token("test-token")
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn builder_fails_without_base_url() {
        let client = ChipClient::builder().bearer_token("test-token").build();
        assert!(client.is_err());
    }

    #[test]
    fn builder_fails_without_token() {
        let client = ChipClient::builder()
            .base_url("https://gate.chip-in.asia/api/v1")
            .build();
        assert!(client.is_err());
    }

    #[test]
    fn builder_strips_trailing_slash() {
        let client = ChipClient::builder()
            .base_url("https://gate.chip-in.asia/api/v1/")
            .bearer_token("test-token")
            .build()
            .unwrap();
        assert!(!client.base_url.ends_with('/'));
    }

    #[test]
    fn builder_with_timeout() {
        let client = ChipClient::builder()
            .base_url("https://gate.chip-in.asia/api/v1")
            .bearer_token("test-token")
            .timeout(Duration::from_secs(30))
            .build();
        assert!(client.is_ok());
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChipError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("Missing required parameter: {0}")]
    MissingParam(&'static str),

    #[error("Signature verification failed")]
    VerificationFailed,

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip_error_display_api_error() {
        let err = ChipError::Api {
            status: 400,
            message: "Bad Request".into(),
        };
        assert_eq!(err.to_string(), "API error 400: Bad Request");
    }

    #[test]
    fn chip_error_display_missing_param() {
        let err = ChipError::MissingParam("brand_id");
        assert_eq!(err.to_string(), "Missing required parameter: brand_id");
    }

    #[test]
    fn chip_error_display_config() {
        let err = ChipError::Config("base_url is required".into());
        assert_eq!(
            err.to_string(),
            "Invalid configuration: base_url is required"
        );
    }

    #[test]
    fn chip_error_display_verification_failed() {
        let err = ChipError::VerificationFailed;
        assert_eq!(err.to_string(), "Signature verification failed");
    }
}

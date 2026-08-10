use chrono::Utc;
use serde::Serialize;

use localid_application::TokenExchangeResult;

/// OAuth token exchange response.
#[derive(Debug, Serialize)]
pub struct TokenResponseBody {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub expires_at: String,
}

impl From<TokenExchangeResult> for TokenResponseBody {
    fn from(result: TokenExchangeResult) -> Self {
        let expires_at = result.expires_at();

        Self {
            access_token: result.access_token().to_string(),
            token_type: "Bearer".to_string(),
            expires_in: (expires_at - Utc::now()).num_seconds(),
            refresh_token: result.refresh_token().to_string(),
            expires_at: expires_at.to_rfc3339(),
        }
    }
}

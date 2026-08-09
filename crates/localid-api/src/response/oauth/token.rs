use serde::Serialize;

use localid_application::TokenExchangeResult;

/// OAuth token exchange response.
#[derive(Debug, Serialize)]
pub struct TokenResponseBody {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
}

impl From<TokenExchangeResult> for TokenResponseBody {
    fn from(result: TokenExchangeResult) -> Self {
        Self {
            access_token: result.access_token().to_string(),
            refresh_token: result.refresh_token().to_string(),
            expires_at: result.expires_at().to_rfc3339(),
        }
    }
}

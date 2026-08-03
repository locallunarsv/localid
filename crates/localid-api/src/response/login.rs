use chrono::{DateTime, Utc};
use serde::Serialize;

use localid_application::TokenResponse;

/// HTTP login response.
#[derive(Debug, Serialize)]
pub struct LoginResponseBody {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl From<TokenResponse> for LoginResponseBody {
    fn from(response: TokenResponse) -> Self {
        Self {
            access_token: response.access_token().to_owned(),
            refresh_token: response.refresh_token().to_owned(),
            expires_at: response.expires_at(),
        }
    }
}

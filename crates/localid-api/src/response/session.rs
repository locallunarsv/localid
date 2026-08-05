use chrono::{DateTime, Utc};

use localid_application::SessionResponse;

use serde::Serialize;

/// HTTP response body for current session.
#[derive(Debug, Serialize)]
pub struct SessionResponseBody {
    /// Session identifier.
    pub id: String,

    /// Identity identifier.
    pub identity_id: String,

    /// Session creation time.
    pub created_at: DateTime<Utc>,

    /// Session expiration time.
    pub expires_at: DateTime<Utc>,

    /// Whether session is active.
    pub active: bool,
}

impl From<SessionResponse> for SessionResponseBody {
    fn from(response: SessionResponse) -> Self {
        Self {
            id: response.id.to_string(),
            identity_id: response.identity_id.to_string(),
            created_at: response.created_at,
            expires_at: response.expires_at,
            active: response.active,
        }
    }
}

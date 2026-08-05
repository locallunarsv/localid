use chrono::{DateTime, Utc};

use localid_identity::IdentityId;
use localid_session::{Session, SessionId};

/// Application response representing a session.
#[derive(Debug, Clone)]
pub struct SessionResponse {
    /// Session identifier.
    pub id: SessionId,

    /// Identity owning this session.
    pub identity_id: IdentityId,

    /// Session creation timestamp.
    pub created_at: DateTime<Utc>,

    /// Session expiration timestamp.
    pub expires_at: DateTime<Utc>,

    /// Whether the session is currently active.
    pub active: bool,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            id: session.id(),
            identity_id: session.identity_id(),
            created_at: session.created_at(),
            expires_at: session.expires_at(),
            active: session.is_active(),
        }
    }
}

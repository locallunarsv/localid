use serde::Serialize;

use localid_identity::Identity;

/// OAuth UserInfo response body.
#[derive(Debug, Serialize)]
pub struct UserInfoResponseBody {
    /// Subject identifier.
    pub sub: String,

    /// Identity lifecycle state.
    pub status: String,
}

impl From<Identity> for UserInfoResponseBody {
    fn from(identity: Identity) -> Self {
        Self {
            sub: identity.id().to_string(),
            status: format!("{:?}", identity.lifecycle_state()),
        }
    }
}

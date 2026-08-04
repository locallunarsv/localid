use serde::Serialize;

use localid_application::VerifyTokenResponse;

/// HTTP token verification response.
#[derive(Debug, Serialize)]
pub struct VerifyTokenResponseBody {
    pub identity_id: String,
    pub session_id: String,
}

impl From<VerifyTokenResponse> for VerifyTokenResponseBody {
    fn from(response: VerifyTokenResponse) -> Self {
        Self {
            identity_id: response.identity_id().to_string(),
            session_id: response.session_id().to_string(),
        }
    }
}

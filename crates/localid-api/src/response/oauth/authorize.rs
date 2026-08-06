use serde::Serialize;

use localid_application::AuthorizationResult;

/// OAuth authorization response.
#[derive(Debug, Serialize)]
pub struct AuthorizeResponseBody {
    pub code_id: String,
}

impl From<AuthorizationResult> for AuthorizeResponseBody {
    fn from(result: AuthorizationResult) -> Self {
        Self {
            code_id: result.code_id().to_string(),
        }
    }
}

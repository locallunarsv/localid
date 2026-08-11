use serde::Serialize;

use localid_application::AuthorizationResult;

/// OAuth authorization response.
#[derive(Debug, Serialize)]
pub struct AuthorizeResponseBody {
    pub code_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl From<AuthorizationResult> for AuthorizeResponseBody {
    fn from(result: AuthorizationResult) -> Self {
        Self {
            code_id: result.code_id().to_string(),
            state: result.request_state().map(ToOwned::to_owned),
        }
    }
}

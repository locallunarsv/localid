use localid_oauth_authorization::AuthorizationCode;

/// Result dari OAuth authorization request.
#[derive(Debug, Clone)]
pub struct AuthorizationResult {
    code: AuthorizationCode,
    authorization_code: String,
}

impl AuthorizationResult {
    /// Creates authorization result.
    #[must_use]
    pub const fn new(code: AuthorizationCode, authorization_code: String) -> Self {
        Self {
            code,
            authorization_code,
        }
    }

    /// Returns authorization code aggregate.
    #[must_use]
    pub const fn code(&self) -> &AuthorizationCode {
        &self.code
    }

    /// Returns generated authorization code secret.
    #[must_use]
    pub fn authorization_code(&self) -> &str {
        &self.authorization_code
    }

    /// Returns authorization code identifier.
    #[must_use]
    pub const fn code_id(&self) -> localid_oauth_authorization::AuthorizationCodeId {
        self.code.id()
    }

    /// Returns OAuth state parameter.
    #[must_use]
    pub fn request_state(&self) -> Option<&str> {
        self.code.request_state()
    }
}

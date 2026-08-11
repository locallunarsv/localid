use localid_oauth_authorization::AuthorizationCode;

/// Result dari OAuth authorization request.
#[derive(Debug, Clone)]
pub struct AuthorizationResult {
    code: AuthorizationCode,
}

impl AuthorizationResult {
    /// Creates authorization result.
    #[must_use]
    pub const fn new(code: AuthorizationCode) -> Self {
        Self { code }
    }

    /// Returns authorization code aggregate.
    #[must_use]
    pub const fn code(&self) -> &AuthorizationCode {
        &self.code
    }

    /// Returns authorization code identifier.
    #[must_use]
    pub const fn code_id(&self) -> localid_oauth_authorization::AuthorizationCodeId {
        self.code.id()
    }
    /// Return State
    pub fn request_state(&self) -> Option<&str> {
        self.code.request_state()
    }
}

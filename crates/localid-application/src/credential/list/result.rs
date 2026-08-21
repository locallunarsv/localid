use localid_credential::Credential;

/// Result of listing Credentials owned by an Identity.
#[derive(Debug, Clone)]
pub struct ListCredentialsResult {
    credentials: Vec<Credential>,
}

impl ListCredentialsResult {
    /// Creates a Credential listing result.
    #[must_use]
    pub const fn new(credentials: Vec<Credential>) -> Self {
        Self { credentials }
    }

    /// Returns the Credentials.
    #[must_use]
    pub fn credentials(&self) -> &[Credential] {
        &self.credentials
    }
}

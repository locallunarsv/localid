use std::str::FromStr;

use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_password::PasswordSecret;
use serde::Deserialize;

/// HTTP login request payload.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    client_id: String,
    credential_id: String,
    password: String,
}

impl LoginRequest {
    /// Returns client identifier.
    pub fn client_id(&self) -> Result<ClientId, uuid::Error> {
        ClientId::from_str(&self.client_id)
    }

    /// Returns credential identifier.
    pub fn credential_id(&self) -> Result<CredentialId, uuid::Error> {
        CredentialId::from_str(&self.credential_id)
    }

    /// Returns password material.
    pub fn password(&self) -> Result<PasswordSecret, localid_password::PasswordError> {
        PasswordSecret::new(&self.password)
    }
}

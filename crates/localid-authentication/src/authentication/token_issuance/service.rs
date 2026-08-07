use localid_client::ClientId;
use localid_identity::IdentityId;

use crate::AuthenticateResult;

/// Token issuance capability.
///
/// Creates session, access token, and refresh token
/// for an authenticated identity.
pub trait TokenIssuanceService {
    /// Error returned by token issuance.
    type Error;

    /// Issues authentication artifacts.
    fn issue(
        &mut self,
        identity_id: IdentityId,
        client_id: ClientId,
    ) -> Result<AuthenticateResult, Self::Error>;
}

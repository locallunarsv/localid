use localid_application::ClientAuthenticationPort;

use localid_database_postgres::{DatabaseError, PostgresOAuthClientRepository};

use localid_oauth_client::{OAuthClient, OAuthClientRepository};

use super::SharedRepository;

impl ClientAuthenticationPort for SharedRepository<PostgresOAuthClientRepository> {
    type Error = DatabaseError;

    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        self.with(|repository| repository.find_by_client_id(client_id))
    }
}

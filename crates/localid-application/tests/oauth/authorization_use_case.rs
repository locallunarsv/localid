use std::collections::HashMap;

use localid_application::{ AuthorizationPort, AuthorizeCommand, AuthorizeUseCase };

use localid_client::ClientId;
use localid_identity::IdentityId;

use localid_oauth_authorization::AuthorizationCode;

use localid_oauth_client::{ OAuthClient, OAuthClientId };

struct FakeAuthorizationAdapter {
    clients: HashMap<String, OAuthClient>,
    codes: Vec<AuthorizationCode>,
}

impl FakeAuthorizationAdapter {
    fn new(client: OAuthClient) -> Self {
        let mut clients = HashMap::new();

        clients.insert(client.client_id().to_owned(), client);

        Self {
            clients,
            codes: Vec::new(),
        }
    }
}

impl AuthorizationPort for FakeAuthorizationAdapter {
    type Error = ();

    fn find_client(&self, client_id: &str) -> Result<Option<OAuthClient>, Self::Error> {
        Ok(self.clients.get(client_id).cloned())
    }

    fn save_code(&mut self, code: AuthorizationCode) -> Result<(), Self::Error> {
        self.codes.push(code);

        Ok(())
    }
}

fn create_client() -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        "demo-client",
        "Demo OAuth Client",
        "secret-hash",
        vec!["http://localhost/callback".to_owned()]
    )
}

#[test]
fn authorization_should_create_code() {
    let client = create_client();

    let command = AuthorizeCommand::new(
        localid_client::ClientId::new(),
        IdentityId::new(),
        "http://localhost/callback",
        vec!["openid".to_owned()]
    );

    let adapter = FakeAuthorizationAdapter::new(client);

    let mut use_case = AuthorizeUseCase::new(adapter);

    let result = use_case.execute(command);

    assert!(result.is_ok());
}

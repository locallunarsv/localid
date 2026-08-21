use std::{process, str::FromStr};

use localid_application::oauth::client::{
    ActivateOAuthClientCommand, ActivateOAuthClientUseCase, CreateOAuthClientCommand,
    CreateOAuthClientUseCase, DeleteOAuthClientCommand, DeleteOAuthClientUseCase,
    DisableOAuthClientCommand, DisableOAuthClientUseCase, GetOAuthClientQuery,
    GetOAuthClientUseCase, ListOAuthClientsUseCase,
};

use localid_oauth_client::OAuthClientId;

use crate::{cli::ClientCommand, context::oauth_client_repository};

/// Executes an OAuth client administration command.
pub async fn execute(command: ClientCommand) {
    match command {
        ClientCommand::List => {
            list().await;
        }

        ClientCommand::Get { client_id } => {
            let client_id = parse_oauth_client_id(&client_id);
            get(client_id).await;
        }

        ClientCommand::Create {
            name,
            redirect_uris,
        } => {
            create(name, redirect_uris).await;
        }
        ClientCommand::Disable { client_id } => {
            let client_id = parse_oauth_client_id(&client_id);
            disable(client_id).await;
        }
        ClientCommand::Activate { client_id } => {
            let client_id = parse_oauth_client_id(&client_id);
            activate(client_id).await;
        }
        ClientCommand::Delete { client_id } => {
            let client_id = parse_oauth_client_id(&client_id);
            delete(client_id).await;
        }
    }
}

fn parse_oauth_client_id(value: &str) -> OAuthClientId {
    match OAuthClientId::from_str(value) {
        Ok(client_id) => client_id,

        Err(error) => {
            eprintln!("Invalid OAuth client ID: {error}");
            process::exit(1);
        }
    }
}

/// Lists OAuth clients.
pub async fn list() {
    let repository = oauth_client_repository().await;

    let use_case = ListOAuthClientsUseCase::new(repository);

    match use_case.execute() {
        Ok(result) => {
            for client in result.clients() {
                println!(
                    "{} {} {:?} {}",
                    client.id(),
                    client.client_id(),
                    client.state(),
                    client.name(),
                );
            }
        }

        Err(error) => {
            eprintln!("Failed to list OAuth clients: {error:?}");
            process::exit(1);
        }
    }
}

/// Gets an OAuth client by internal identifier.
pub async fn get(client_id: OAuthClientId) {
    let repository = oauth_client_repository().await;

    let use_case = GetOAuthClientUseCase::new(repository);
    let query = GetOAuthClientQuery::new(client_id);

    let result = match use_case.execute(query) {
        Ok(result) => result,

        Err(error) => {
            eprintln!("Failed to get OAuth client: {error:?}");
            process::exit(1);
        }
    };

    let client = result.client();

    println!("OAuth Client ID: {}", client.id());
    println!("Local Client ID: {}", client.local_client_id());
    println!("Client ID: {}", client.client_id());
    println!("Name: {}", client.name());
    println!("State: {:?}", client.state());

    println!("Redirect URIs:");

    for redirect_uri in client.redirect_uris() {
        println!("  {redirect_uri}");
    }
}

/// Creates an OAuth client.
pub async fn create(name: String, redirect_uris: Vec<String>) {
    let repository = oauth_client_repository().await;

    let mut use_case = CreateOAuthClientUseCase::new(repository);

    let command = CreateOAuthClientCommand::new(name, redirect_uris);

    match use_case.execute(command) {
        Ok(result) => {
            println!("OAuth client created");
            println!("Client ID: {}", result.client_id());
            println!("Client secret: {}", result.client_secret());
            println!("Save the client secret now; it will not be shown again.");
        }

        Err(error) => {
            eprintln!("Failed to create OAuth client: {error:?}");
            process::exit(1);
        }
    }
}

/// Disables an OAuth client.
pub async fn disable(client_id: OAuthClientId) {
    let repository = oauth_client_repository().await;

    let mut use_case = DisableOAuthClientUseCase::new(repository);
    let command = DisableOAuthClientCommand::new(client_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("OAuth client disabled");
            println!("OAuth Client ID: {client_id}");
        }

        Err(error) => {
            eprintln!("Failed to disable OAuth client: {error:?}");
            process::exit(1);
        }
    }
}

/// Activates an OAuth client.
pub async fn activate(client_id: OAuthClientId) {
    let repository = oauth_client_repository().await;

    let mut use_case = ActivateOAuthClientUseCase::new(repository);
    let command = ActivateOAuthClientCommand::new(client_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("OAuth client activated");
            println!("OAuth Client ID: {client_id}");
        }

        Err(error) => {
            eprintln!("Failed to activate OAuth client: {error:?}");
            process::exit(1);
        }
    }
}

/// Deletes an OAuth client.
pub async fn delete(client_id: OAuthClientId) {
    let repository = oauth_client_repository().await;

    let mut use_case = DeleteOAuthClientUseCase::new(repository);
    let command = DeleteOAuthClientCommand::new(client_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("OAuth client deleted");
            println!("OAuth Client ID: {client_id}");
        }

        Err(error) => {
            eprintln!("Failed to delete OAuth client: {error:?}");
            process::exit(1);
        }
    }
}

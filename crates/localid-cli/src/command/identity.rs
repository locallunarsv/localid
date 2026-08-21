use std::process;
use std::str::FromStr;

use localid_application::identity::{
    DeleteIdentityCommand, DeleteIdentityUseCase, DisableIdentityCommand, DisableIdentityUseCase,
    EnableIdentityCommand, EnableIdentityUseCase, GetIdentityUseCase, IdentityRepositoryAdapter,
    ListIdentitiesUseCase,
};

use crate::cli::IdentityCommand;
use localid_identity::{Identity, IdentityId};
use localid_repository::IdentityRepository;

use crate::context::identity_repository;

fn parse_identity_id(value: &str) -> IdentityId {
    match IdentityId::from_str(value) {
        Ok(identity_id) => identity_id,

        Err(error) => {
            eprintln!("Invalid identity ID: {error}");
            process::exit(1);
        }
    }
}

/// Creates a new Identity.
pub async fn create() {
    let mut repository = identity_repository().await;

    let identity_id = IdentityId::new();
    let identity = Identity::new(identity_id);

    if let Err(error) = repository.save(identity) {
        eprintln!("Failed to create identity: {error:?}");
        process::exit(1);
    }

    println!("Identity created");
    println!("Identity ID: {identity_id}");
}

/// Lists identities.
pub async fn list() {
    let repository = identity_repository().await;

    let adapter = IdentityRepositoryAdapter::new(repository);
    let use_case = ListIdentitiesUseCase::new(adapter);

    match use_case.execute() {
        Ok(result) => {
            for identity in result.identities() {
                println!("{} {:?}", identity.id(), identity.lifecycle_state());
            }
        }

        Err(error) => {
            eprintln!("Failed to list identities: {error:?}");
            process::exit(1);
        }
    }
}

/// Gets an Identity by identifier.
pub async fn get(identity_id: IdentityId) {
    let repository = identity_repository().await;

    let adapter = IdentityRepositoryAdapter::new(repository);
    let mut use_case = GetIdentityUseCase::new(adapter);

    let result = match use_case.execute(identity_id) {
        Ok(result) => result,

        Err(error) => {
            eprintln!("Failed to get identity: {error:?}");
            process::exit(1);
        }
    };

    let identity = result.identity();

    println!("Identity ID: {}", identity.id());
    println!("Lifecycle state: {:?}", identity.lifecycle_state());
}

/// Disables an Identity.
pub async fn disable(identity_id: IdentityId) {
    let repository = identity_repository().await;

    let mut use_case = DisableIdentityUseCase::new(repository);
    let command = DisableIdentityCommand::new(identity_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Identity disabled");
            println!("Identity ID: {identity_id}");
        }

        Err(error) => {
            eprintln!("Failed to disable identity: {error:?}");
            process::exit(1);
        }
    }
}

/// Enables an Identity.
pub async fn enable(identity_id: IdentityId) {
    let repository = identity_repository().await;

    let mut use_case = EnableIdentityUseCase::new(repository);
    let command = EnableIdentityCommand::new(identity_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Identity enabled");
            println!("Identity ID: {identity_id}");
        }

        Err(error) => {
            eprintln!("Failed to enable identity: {error:?}");
            process::exit(1);
        }
    }
}

/// Deletes an Identity.
pub async fn delete(identity_id: IdentityId) {
    let repository = identity_repository().await;

    let mut use_case = DeleteIdentityUseCase::new(repository);
    let command = DeleteIdentityCommand::new(identity_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Identity deleted");
            println!("Identity ID: {identity_id}");
        }

        Err(error) => {
            eprintln!("Failed to delete identity: {error:?}");
            process::exit(1);
        }
    }
}

/// Executes an Identity administration command.
pub async fn execute(command: IdentityCommand) {
    match command {
        IdentityCommand::Create => create().await,

        IdentityCommand::List => list().await,

        IdentityCommand::Get { identity_id } => {
            let identity_id = parse_identity_id(&identity_id);
            get(identity_id).await;
        }

        IdentityCommand::Disable { identity_id } => {
            let identity_id = parse_identity_id(&identity_id);
            disable(identity_id).await;
        }

        IdentityCommand::Enable { identity_id } => {
            let identity_id = parse_identity_id(&identity_id);
            enable(identity_id).await;
        }

        IdentityCommand::Delete { identity_id } => {
            let identity_id = parse_identity_id(&identity_id);
            delete(identity_id).await;
        }
    }
}

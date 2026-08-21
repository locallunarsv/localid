use std::process;

use localid_application::credential::{
    disable::{DisableCredentialCommand, DisableCredentialUseCase},
    enable::{EnableCredentialCommand, EnableCredentialUseCase},
    get::GetCredentialUseCase,
    list::ListCredentialsUseCase,
    password::{
        create::{CreatePasswordCredentialCommand, CreatePasswordCredentialUseCase},
        rotate::{RotatePasswordCredentialCommand, RotatePasswordCredentialUseCase},
    },
    revoke::{RevokeCredentialCommand, RevokeCredentialUseCase},
};

use std::str::FromStr;

use crate::cli::{CredentialCommand, PasswordCredentialCommand};

use localid_database_postgres::{
    PostgresCredentialRepository, PostgresIdentityRepository, PostgresPasswordMaterialRepository,
};
use localid_password::PasswordSecret;
use localid_password_argon2::Argon2PasswordHasher;
use tokio::runtime::Handle;

use crate::context::{credential_repository, database_config};
use localid_credential::CredentialId;
use localid_identity::IdentityId;

fn parse_identity_id(value: &str) -> IdentityId {
    match IdentityId::from_str(value) {
        Ok(identity_id) => identity_id,

        Err(error) => {
            eprintln!("Invalid identity ID: {error}");
            process::exit(1);
        }
    }
}

fn parse_credential_id(value: &str) -> CredentialId {
    match CredentialId::from_str(value) {
        Ok(credential_id) => credential_id,

        Err(error) => {
            eprintln!("Invalid credential ID: {error}");
            process::exit(1);
        }
    }
}

/// Lists Credentials owned by an Identity.
pub async fn list(identity_id: IdentityId) {
    let repository = credential_repository().await;

    let use_case = ListCredentialsUseCase::new(repository);

    let result = match use_case.execute(identity_id) {
        Ok(result) => result,

        Err(error) => {
            eprintln!("Failed to list credentials: {error}");
            process::exit(1);
        }
    };

    for credential in result.credentials() {
        println!(
            "{} {:?} {:?}",
            credential.id(),
            credential.kind(),
            credential.lifecycle_state()
        );
    }
}

/// Gets a Credential by identifier.
pub async fn get(credential_id: CredentialId) {
    let repository = credential_repository().await;

    let use_case = GetCredentialUseCase::new(repository);

    let result = match use_case.execute(credential_id) {
        Ok(result) => result,

        Err(error) => {
            eprintln!("Failed to get credential: {error:?}");
            process::exit(1);
        }
    };

    let credential = result.credential();

    println!("Credential ID: {}", credential.id());
    println!("Identity ID: {}", credential.identity_id());
    println!("Kind: {:?}", credential.kind());
    println!("Lifecycle state: {:?}", credential.lifecycle_state());
}

/// Disables a Credential.
pub async fn disable(credential_id: CredentialId) {
    let repository = credential_repository().await;

    let mut use_case = DisableCredentialUseCase::new(repository);
    let command = DisableCredentialCommand::new(credential_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Credential disabled");
            println!("Credential ID: {credential_id}");
        }

        Err(error) => {
            eprintln!("Failed to disable credential: {error}");
            process::exit(1);
        }
    }
}

/// Enables a Credential.
pub async fn enable(credential_id: CredentialId) {
    let repository = credential_repository().await;

    let mut use_case = EnableCredentialUseCase::new(repository);
    let command = EnableCredentialCommand::new(credential_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Credential enabled");
            println!("Credential ID: {credential_id}");
        }

        Err(error) => {
            eprintln!("Failed to enable credential: {error}");
            process::exit(1);
        }
    }
}

/// Revokes a Credential.
pub async fn revoke(credential_id: CredentialId) {
    let repository = credential_repository().await;

    let mut use_case = RevokeCredentialUseCase::new(repository);
    let command = RevokeCredentialCommand::new(credential_id);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Credential revoked");
            println!("Credential ID: {credential_id}");
        }

        Err(error) => {
            eprintln!("Failed to revoke credential: {error}");
            process::exit(1);
        }
    }
}

/// Creates a password Credential for an Identity.
pub async fn create_password(identity_id: IdentityId) {
    let password = match rpassword::prompt_password("Password: ") {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Failed to read password: {error}");
            process::exit(1);
        }
    };

    let confirmation = match rpassword::prompt_password("Confirm password: ") {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Failed to read password confirmation: {error}");
            process::exit(1);
        }
    };

    if password != confirmation {
        eprintln!("Password confirmation does not match");
        process::exit(1);
    }

    let password = match PasswordSecret::new(password) {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Invalid password: {error}");
            process::exit(1);
        }
    };

    let database = database_config();
    let runtime = Handle::current();

    let identity_repository =
        match PostgresIdentityRepository::connect(&database, runtime.clone()).await {
            Ok(repository) => repository,

            Err(error) => {
                eprintln!("Failed to initialize PostgreSQL identity repository: {error:?}");
                process::exit(1);
            }
        };

    let credential_repository =
        match PostgresCredentialRepository::connect(&database, runtime.clone()).await {
            Ok(repository) => repository,

            Err(error) => {
                eprintln!("Failed to initialize PostgreSQL credential repository: {error:?}");
                process::exit(1);
            }
        };

    let password_material_repository =
        match PostgresPasswordMaterialRepository::connect(&database, runtime).await {
            Ok(repository) => repository,

            Err(error) => {
                eprintln!(
                    "Failed to initialize PostgreSQL password material repository: {error:?}"
                );
                process::exit(1);
            }
        };

    let password_hasher = Argon2PasswordHasher::new();

    let mut use_case = CreatePasswordCredentialUseCase::new(
        identity_repository,
        credential_repository,
        password_material_repository,
        password_hasher,
    );

    let command = CreatePasswordCredentialCommand::new(identity_id, password);

    match use_case.execute(command) {
        Ok(result) => {
            println!("Password credential created");
            println!("Credential ID: {}", result.credential_id());
            println!("Identity ID: {identity_id}");
        }

        Err(error) => {
            eprintln!("Failed to create password credential: {error:?}");
            process::exit(1);
        }
    }
}

/// Rotates the password for a password Credential.
pub async fn rotate_password(credential_id: CredentialId) {
    let password = match rpassword::prompt_password("New password: ") {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Failed to read new password: {error}");
            process::exit(1);
        }
    };

    let confirmation = match rpassword::prompt_password("Confirm new password: ") {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Failed to read password confirmation: {error}");
            process::exit(1);
        }
    };

    if password != confirmation {
        eprintln!("Password confirmation does not match");
        process::exit(1);
    }

    let password = match PasswordSecret::new(password) {
        Ok(password) => password,

        Err(error) => {
            eprintln!("Invalid password: {error}");
            process::exit(1);
        }
    };

    let database = database_config();
    let runtime = Handle::current();

    let credential_repository =
        match PostgresCredentialRepository::connect(&database, runtime.clone()).await {
            Ok(repository) => repository,

            Err(error) => {
                eprintln!("Failed to initialize PostgreSQL credential repository: {error:?}");
                process::exit(1);
            }
        };

    let password_material_repository =
        match PostgresPasswordMaterialRepository::connect(&database, runtime).await {
            Ok(repository) => repository,

            Err(error) => {
                eprintln!(
                    "Failed to initialize PostgreSQL password material repository: {error:?}"
                );
                process::exit(1);
            }
        };

    let password_hasher = Argon2PasswordHasher::new();

    let mut use_case = RotatePasswordCredentialUseCase::new(
        credential_repository,
        password_material_repository,
        password_hasher,
    );

    let command = RotatePasswordCredentialCommand::new(credential_id, password);

    match use_case.execute(command) {
        Ok(()) => {
            println!("Password rotated");
            println!("Credential ID: {credential_id}");
        }

        Err(error) => {
            eprintln!("Failed to rotate password: {error}");
            process::exit(1);
        }
    }
}

/// Executes a Credential administration command.
pub async fn execute(command: CredentialCommand) {
    match command {
        CredentialCommand::List { identity_id } => {
            let identity_id = parse_identity_id(&identity_id);
            list(identity_id).await;
        }

        CredentialCommand::Get { credential_id } => {
            let credential_id = parse_credential_id(&credential_id);
            get(credential_id).await;
        }

        CredentialCommand::Disable { credential_id } => {
            let credential_id = parse_credential_id(&credential_id);
            disable(credential_id).await;
        }

        CredentialCommand::Enable { credential_id } => {
            let credential_id = parse_credential_id(&credential_id);
            enable(credential_id).await;
        }

        CredentialCommand::Revoke { credential_id } => {
            let credential_id = parse_credential_id(&credential_id);
            revoke(credential_id).await;
        }

        CredentialCommand::Password { command } => match command {
            PasswordCredentialCommand::Create { identity_id } => {
                let identity_id = parse_identity_id(&identity_id);
                create_password(identity_id).await;
            }

            PasswordCredentialCommand::Rotate { credential_id } => {
                let credential_id = parse_credential_id(&credential_id);
                rotate_password(credential_id).await;
            }
        },
    }
}

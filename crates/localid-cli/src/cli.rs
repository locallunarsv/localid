use clap::{Parser, Subcommand};

/// LocalID administrative CLI.
#[derive(Debug, Parser)]
#[command(name = "localid")]
#[command(about = "LocalID administrative CLI")]
pub struct Cli {
    /// Top-level command.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level LocalID CLI commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage identities.
    Identity {
        /// Identity command.
        #[command(subcommand)]
        command: IdentityCommand,
    },

    /// Manage Credentials.
    Credential {
        /// Credential command.
        #[command(subcommand)]
        command: CredentialCommand,
    },

    /// Manage OAuth clients.
    Client {
        /// OAuth client command.
        #[command(subcommand)]
        command: ClientCommand,
    },
}

/// Identity administration commands.
#[derive(Debug, Subcommand)]
pub enum IdentityCommand {
    /// Creates a new identity.
    Create,

    /// Lists identities.
    List,

    /// Gets an identity by identifier.
    Get {
        /// Identity identifier.
        identity_id: String,
    },

    /// Disables an identity.
    Disable {
        /// Identity identifier.
        identity_id: String,
    },

    /// Enables an identity.
    Enable {
        /// Identity identifier.
        identity_id: String,
    },

    /// Deletes an identity.
    Delete {
        /// Identity identifier.
        identity_id: String,
    },
}

/// Credential administration commands.
#[derive(Debug, Subcommand)]
pub enum CredentialCommand {
    /// Lists Credentials owned by an Identity.
    List {
        /// Owning Identity identifier.
        identity_id: String,
    },

    /// Gets a Credential by identifier.
    Get {
        /// Credential identifier.
        credential_id: String,
    },

    /// Disables a Credential.
    Disable {
        /// Credential identifier.
        credential_id: String,
    },

    /// Enables a Credential.
    Enable {
        /// Credential identifier.
        credential_id: String,
    },

    /// Revokes a Credential.
    Revoke {
        /// Credential identifier.
        credential_id: String,
    },

    /// Manage password Credentials.
    Password {
        /// Password Credential command.
        #[command(subcommand)]
        command: PasswordCredentialCommand,
    },
}

/// Password Credential administration commands.
#[derive(Debug, Subcommand)]
pub enum PasswordCredentialCommand {
    /// Creates a password Credential.
    Create {
        /// Owning Identity identifier.
        identity_id: String,
    },

    /// Rotates the password for a Credential.
    Rotate {
        /// Credential identifier.
        credential_id: String,
    },
}

/// OAuth client administration commands.
#[derive(Debug, Subcommand)]
pub enum ClientCommand {
    /// Creates an OAuth client.
    Create {
        /// Client display name.
        #[arg(long)]
        name: String,

        /// Registered redirect URI.
        #[arg(long = "redirect-uri", required = true)]
        redirect_uris: Vec<String>,
    },

    /// Lists OAuth clients.
    List,

    /// Gets an OAuth client by internal identifier.
    Get {
        /// Internal OAuth client identifier.
        client_id: String,
    },
    /// Disables an OAuth client.
    Disable {
        /// Internal OAuth client identifier.
        client_id: String,
    },
    /// Activates an OAuth client.
    Activate {
        /// Internal OAuth client identifier.
        client_id: String,
    },
    /// Deletes an OAuth client.
    Delete {
        /// Internal OAuth client identifier.
        client_id: String,
    },
}

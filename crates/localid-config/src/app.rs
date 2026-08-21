//! Application configuration.

use std::{
    env,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{DatabaseConfig, Environment, ServerConfig};

const DEFAULT_CONFIG_PATH: &str = "/etc/localid/localid.toml";

/// Complete LocalID application configuration.
/// Complete LocalID application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Runtime environment.
    pub environment: Environment,

    /// Database configuration.
    pub database: DatabaseConfig,

    /// Server configuration.
    pub server: ServerConfig,
}
impl AppConfig {
    /// Parses application configuration from TOML source.
    ///
    /// # Errors
    ///
    /// Returns an error when the TOML source is invalid or does not match
    /// the expected LocalID configuration structure.
    pub fn from_toml(source: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(source)
    }

    /// Loads application configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or when its contents
    /// cannot be parsed as LocalID configuration.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, AppConfigLoadError> {
        let source = fs::read_to_string(path).map_err(AppConfigLoadError::Read)?;

        Self::from_toml(&source).map_err(AppConfigLoadError::Parse)
    }

    /// Returns the configured LocalID configuration file path.
    ///
    /// Uses `LOCALID_CONFIG` when present, otherwise falls back to
    /// `/etc/localid/localid.toml`.
    #[must_use]
    pub fn config_path() -> PathBuf {
        env::var_os("LOCALID_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH))
    }

    /// Loads LocalID application configuration from the configured path.
    ///
    /// Uses `LOCALID_CONFIG` when present, otherwise loads
    /// `/etc/localid/localid.toml`.
    ///
    /// Environment variables override values loaded from the configuration
    /// file.
    ///
    /// # Errors
    ///
    /// Returns an error when the configuration file cannot be read or parsed,
    /// or when an environment override contains an invalid value.
    pub fn load() -> Result<Self, AppConfigLoadError> {
        let config = Self::from_file(Self::config_path())?;

        config.apply_env_overrides()
    }

    fn apply_env_overrides(mut self) -> Result<Self, AppConfigLoadError> {
        if let Ok(url) = env::var("LOCALID_DATABASE_URL") {
            self.database.url = url;
        }

        if let Ok(max_connections) = env::var("LOCALID_DATABASE_MAX_CONNECTIONS") {
            self.database.max_connections = max_connections
                .parse::<u32>()
                .map_err(|_| AppConfigLoadError::InvalidDatabaseMaxConnections(max_connections))?;
        }

        if let Ok(host) = env::var("LOCALID_SERVER_HOST") {
            self.server.host = host;
        }

        if let Ok(port) = env::var("LOCALID_SERVER_PORT") {
            self.server.port = port
                .parse::<u16>()
                .map_err(|_| AppConfigLoadError::InvalidServerPort(port))?;
        }

        if let Ok(issuer) = env::var("LOCALID_ISSUER") {
            self.server.issuer = issuer;
        }

        if let Ok(signing_key_path) = env::var("LOCALID_SIGNING_KEY_PATH") {
            self.server.signing_key_path = signing_key_path;
        }

        Ok(self)
    }
}

/// Error returned while loading LocalID application configuration.
#[derive(Debug)]
pub enum AppConfigLoadError {
    /// Configuration file could not be read.
    Read(std::io::Error),

    /// Configuration file contents could not be parsed.
    Parse(toml::de::Error),

    /// Server port environment override is invalid.
    InvalidServerPort(String),

    /// Database maximum connections environment override is invalid.
    InvalidDatabaseMaxConnections(String),
}

impl fmt::Display for AppConfigLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => {
                write!(formatter, "failed to read configuration file: {error}")
            }
            Self::Parse(error) => {
                write!(formatter, "failed to parse configuration file: {error}")
            }
            Self::InvalidServerPort(value) => {
                write!(formatter, "invalid LOCALID_SERVER_PORT: {value}")
            }
            Self::InvalidDatabaseMaxConnections(value) => {
                write!(
                    formatter,
                    "invalid LOCALID_DATABASE_MAX_CONNECTIONS: {value}"
                )
            }
        }
    }
}

impl Error for AppConfigLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read(error) => Some(error),
            Self::Parse(error) => Some(error),
            Self::InvalidServerPort(_) => None,
            Self::InvalidDatabaseMaxConnections(_) => None,
        }
    }
}

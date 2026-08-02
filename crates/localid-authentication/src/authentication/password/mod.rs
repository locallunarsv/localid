mod config;
mod default_service;
mod request;
mod service;

pub use config::PasswordAuthenticationDependencies;
pub use default_service::DefaultPasswordAuthenticationService;
pub use request::AuthenticatePasswordRequest;
pub use service::PasswordAuthenticationService;

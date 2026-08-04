//! Authentication adapters.

mod password;
mod refresh_token;
mod token_verification;

pub use password::PasswordAuthenticationAdapter;
pub use refresh_token::RefreshTokenAdapter;
pub use token_verification::TokenVerificationAdapter;

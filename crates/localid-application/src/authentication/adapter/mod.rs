//! Authentication adapters.

mod password;
mod refresh_token;

pub use password::PasswordAuthenticationAdapter;
pub use refresh_token::RefreshTokenAdapter;

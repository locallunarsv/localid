#![deny(missing_docs)]

//! Token domain for LocalID.

mod token;

pub use token::{IssuedToken, Token, TokenError, TokenId, TokenIssuer, TokenLifecycleState};

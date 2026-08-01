#![deny(missing_docs)]

//! Token domain for LocalID.

mod token;

pub use token::{Token, TokenError, TokenId, TokenLifecycleState};

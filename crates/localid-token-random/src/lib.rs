#![deny(missing_docs)]

//! Random token issuer implementation.

mod issuer;

pub use issuer::RandomTokenIssuer;

pub use localid_token::{IssuedToken, TokenIssuer};

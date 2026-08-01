#![deny(missing_docs)]

//! Authentication orchestration for LocalID.

mod authentication;

pub use authentication::{
    AuthenticateRequest, AuthenticateResult, AuthenticationError, AuthenticationService,
    CredentialVerifier,
};

#![deny(missing_docs)]

//! Authentication orchestration for LocalID.
//!
//! This crate coordinates Identity, Credential, and Session domains without
//! depending on transport protocols or concrete storage implementations.

mod authentication;

pub use authentication::{
    AuthenticateRequest, AuthenticateResult, AuthenticationError, AuthenticationEvidence,
    AuthenticationService, CredentialVerifier, DefaultAuthenticationService, SessionFactory,
};

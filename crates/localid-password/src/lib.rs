#![deny(missing_docs)]

//! Password domain and hashing contracts for LocalID.

mod password;

pub use password::{
    PasswordCredential, PasswordError, PasswordHash, PasswordHasher, PasswordSecret,
};

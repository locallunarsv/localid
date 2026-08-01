#![deny(missing_docs)]

//! Password domain and hashing contracts for LocalID.

mod password;

pub use password::{
    PasswordError, PasswordHash, PasswordHasher, PasswordMaterial, PasswordSecret, PasswordVerifier,
};

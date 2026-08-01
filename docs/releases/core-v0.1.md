# LocalID Core v0.1

**Release Date:** 2026-08-02
**Status:** Internal Milestone

## Overview

LocalID Core v0.1 establishes the initial domain, repository, authentication, and in-memory storage foundations of LocalID.

This milestone validates the core architecture through unit tests, integration tests, and an end-to-end authentication flow.

## Domain Crates

### `localid-identity`

Provides the canonical representation of a digital subject.

Implemented capabilities:

- stable Identity identifiers;
- Active, Disabled, and Deleted lifecycle states;
- enable, disable, and delete behaviors;
- lifecycle transition validation.

### `localid-credential`

Represents authentication Credentials owned by an Identity.

Implemented capabilities:

- stable Credential identifiers;
- Password, Passkey, and API Key Credential kinds;
- Active, Disabled, and Revoked lifecycle states;
- enable, disable, and revoke behaviors;
- lifecycle transition validation.

### `localid-session`

Represents authenticated Sessions owned by an Identity.

Implemented capabilities:

- stable Session identifiers;
- Active and Revoked lifecycle states;
- creation and expiration timestamps;
- time-derived expiration;
- explicit Session revocation;
- Session validity evaluation.

## Repository Contracts

The `localid-repository` crate defines storage-independent contracts for:

- Identity repositories;
- Credential repositories;
- Session repositories.

The domain crates do not depend on repository or storage implementations.

## Authentication

The `localid-authentication` crate provides:

- authentication requests and results;
- authentication evidence;
- Credential verification contracts;
- Session factory contracts;
- authentication service contracts;
- a default authentication service.

The default authentication flow:

1. loads the requested Credential;
2. verifies that the Credential is active;
3. loads the owning Identity;
4. verifies that the Identity is active;
5. verifies the presented authentication evidence;
6. creates a Session;
7. persists the Session;
8. returns the authenticated Session.

## In-Memory Storage

The `localid-storage-memory` crate provides a shared in-memory implementation of all repository contracts.

Cloned storage handles share state through synchronized in-memory storage.

This adapter is intended for:

- development;
- testing;
- architecture validation;
- ephemeral deployments.

## Architecture Validation

The Core v0.1 architecture is validated through:

- domain unit tests;
- public API integration tests;
- repository adapter tests;
- authentication contract tests;
- an end-to-end authentication flow.

The end-to-end test verifies that:

- an Identity can be persisted;
- a Credential can be associated with the Identity;
- authentication evidence can be verified;
- a Session can be created;
- the Session can be persisted;
- the authenticated Session is returned to the caller.

## Quality Gates

The workspace passes:

- `cargo fmt --all`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --workspace`;
- `cargo doc --workspace --no-deps`.

## Current Limitations

Core v0.1 does not yet provide:

- concrete password verification;
- Passkey verification;
- API Key verification;
- persistent database storage;
- REST or gRPC APIs;
- token issuance;
- authorization;
- account profiles;
- production deployment tooling.

## Next Direction

The next development phase should focus on concrete authentication capabilities and persistent adapters while preserving the domain and dependency boundaries established in Core v0.1.

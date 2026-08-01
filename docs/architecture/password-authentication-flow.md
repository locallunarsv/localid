# Password Authentication Flow

**Version:** 0.1
**Status:** Accepted for Initial Implementation

---

## Purpose

This document defines the initial password authentication flow for LocalID.

It describes how the Authentication layer coordinates Credential, Identity, Password Material, password verification, Session creation, and persistence.

The flow intentionally excludes transport protocols, databases, cookies, tokens, and concrete password-hashing algorithms.

---

## Participants

The password authentication flow uses the following components:

- `AuthenticatePasswordRequest`;
- `CredentialRepository`;
- `IdentityRepository`;
- `PasswordMaterialRepository`;
- `PasswordVerifier`;
- `SessionFactory`;
- `SessionRepository`;
- `AuthenticateResult`.

---

## Input

Password authentication begins with an `AuthenticatePasswordRequest`.

The request contains:

- the identifier of the target Credential;
- the plain-text password supplied by the caller.

The request does not contain an `IdentityId`.

The owning Identity is resolved through the Credential aggregate to avoid accepting an inconsistent Credential and Identity pair from the caller.

---

## Authentication Flow

```text
AuthenticatePasswordRequest
        │
        ▼
Load Credential
        │
        ▼
Verify Credential Kind
        │
        ▼
Verify Credential Lifecycle
        │
        ▼
Load Owning Identity
        │
        ▼
Verify Identity Lifecycle
        │
        ▼
Load Password Material
        │
        ▼
Verify Password
        │
        ▼
Create Session
        │
        ▼
Persist Session
        │
        ▼
AuthenticateResult
```

---

## Step 1 — Load Credential

The Authentication service loads the Credential using the `CredentialId` contained in the request.

Possible outcomes:

- the Credential is found;
- the Credential does not exist;
- the Credential repository fails.

Authentication cannot continue without a valid Credential aggregate.

---

## Step 2 — Verify Credential Kind

The loaded Credential must have the `Password` Credential Kind.

A Credential with another kind must not be processed by the password authentication service.

Examples of incompatible kinds include:

- Passkey;
- ApiKey.

Credential-specific authentication services must process their corresponding Credential kinds.

---

## Step 3 — Verify Credential Lifecycle

The Credential must be active.

Authentication is rejected when the Credential is:

- Disabled;
- Revoked.

A disabled Credential may become usable again through an explicit lifecycle transition.

A revoked Credential is terminal and cannot return to active use.

---

## Step 4 — Load Owning Identity

The Authentication service obtains the owning `IdentityId` from the Credential aggregate.

It then loads the corresponding Identity through `IdentityRepository`.

Possible outcomes:

- the Identity is found;
- the Identity does not exist;
- the Identity repository fails.

---

## Step 5 — Verify Identity Lifecycle

The owning Identity must be active.

Authentication is rejected when the Identity is:

- Disabled;
- Deleted.

Credential validity does not override Identity lifecycle restrictions.

---

## Step 6 — Load Password Material

The Authentication service loads `PasswordMaterial` using the Credential identifier.

Password Material contains password-specific authentication data associated with the Credential, including the stored `PasswordHash`.

Possible outcomes:

- Password Material is found;
- Password Material does not exist;
- the Password Material repository fails.

The primary Credential aggregate does not store password-specific material.

---

## Step 7 — Verify Password

The Authentication service delegates password verification to `PasswordVerifier`.

The verifier receives:

- the stored `PasswordMaterial`;
- the supplied `PasswordSecret`.

The verifier returns:

- `true` when the password matches;
- `false` when the password does not match;
- an error when verification cannot be completed.

The Authentication service does not perform password hashing directly.

---

## Step 8 — Create Session

After successful password verification, the Authentication service requests a new Session from `SessionFactory`.

The Session must belong to the authenticated Identity.

The factory determines:

- the Session identifier;
- creation time;
- expiration time;
- any future Session creation policy.

The Authentication service does not access the system clock directly.

---

## Step 9 — Persist Session

The new Session is persisted through `SessionRepository`.

Authentication is not considered successful if the Session cannot be persisted.

This prevents returning a Session that cannot subsequently be resolved by the system.

---

## Step 10 — Return Result

After successful Session persistence, the Authentication service returns an `AuthenticateResult`.

The result contains the authenticated Session.

---

## Failure Rules

Authentication must fail when:

- the Credential does not exist;
- the Credential kind is not Password;
- the Credential is not active;
- the owning Identity does not exist;
- the owning Identity is not active;
- Password Material does not exist;
- password verification fails;
- password verification cannot be completed;
- Session creation fails;
- Session persistence fails;
- a repository operation fails.

---

## Security Error Semantics

Public transport adapters should avoid revealing whether a Credential, Identity, or Password Material record exists.

The application service may use precise internal errors for diagnostics and testing.

Transport adapters may map authentication-related failures to a generic response such as:

```text
invalid credentials
```

Infrastructure failures should be logged and handled separately from invalid authentication evidence.

---

## Responsibility Boundaries

### Password Authentication Service

Responsible for:

- orchestrating the password authentication flow;
- loading required domain records;
- enforcing Credential and Identity availability;
- invoking password verification;
- creating and persisting a Session;
- returning the authentication result.

Not responsible for:

- hashing passwords;
- implementing Argon2;
- generating Session timestamps directly;
- storing aggregates directly;
- defining HTTP or gRPC responses;
- issuing access or refresh tokens;
- authorizing application actions.

### Password Verifier

Responsible for:

- comparing a supplied `PasswordSecret` with stored `PasswordMaterial`.

Not responsible for:

- loading repositories;
- resolving an Identity;
- creating Sessions;
- managing lifecycle state.

### Session Factory

Responsible for:

- constructing a valid Session for an authenticated Identity.

Not responsible for:

- verifying passwords;
- persisting Sessions;
- authenticating an Identity.

---

## Initial Implementation Scope

The initial implementation supports:

- password Credentials;
- active Credential validation;
- active Identity validation;
- password verification;
- Session creation;
- Session persistence;
- authentication result creation.

The initial implementation does not support:

- password reset;
- password expiration;
- password history;
- multi-factor authentication;
- throttling;
- lockout policy;
- risk-based authentication;
- Session renewal;
- token issuance.

---

## Future Considerations

Future revisions may introduce:

- generic public authentication failure responses;
- authentication attempt auditing;
- rate limiting;
- account or Credential locking;
- password rehashing after successful verification;
- password-change requirements;
- multi-factor authentication;
- device and Client context;
- transactional Session persistence.

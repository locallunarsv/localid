# Session

**Version:** 0.2
**Status:** Draft

---

## Introduction

Session represents an authenticated interaction between a digital subject and LocalID.

A Session belongs to exactly one Identity and is created only after successful authentication.

Session is independent from both Identity and Credential lifecycles.

---

## Purpose

Session provides a stable representation of an authenticated interaction.

It enables LocalID to reason about authenticated access independently of transport mechanisms such as cookies, bearer tokens, HTTP sessions, or gRPC metadata.

---

## Minimum Session

The minimum Session consists of:

- a unique identifier;
- an owning Identity identifier;
- a lifecycle state;
- a creation time;
- an expiration time.

Session does **not** require:

- a cookie;
- a JWT;
- an access token;
- a refresh token;
- transport-specific metadata.

These are implementation details outside the Session domain.

---

## Responsibilities

Session is responsible for:

- representing an authenticated interaction;
- belonging to exactly one Identity;
- maintaining its own lifecycle;
- storing its validity period.

---

## Non-Responsibilities

Session is **not** responsible for:

- authenticating an Identity;
- verifying Credentials;
- authorizing requests;
- managing Identity lifecycle;
- managing Credential lifecycle;
- defining transport protocols;
- enforcing authentication policy.

---

## Aggregate Boundary

Session is an independent aggregate.

Session references its owner through `IdentityId`.

Identity does not own Session objects.

Credential does not own Session objects.

Changes to a Session do not require modifications to either the Identity or Credential aggregates.

---

## Invariants

The following rules must always hold:

- Every Session belongs to exactly one Identity.
- Every Session references an existing Identity.
- Every Session has exactly one identifier.
- Every Session has exactly one lifecycle state.
- Session expiration time is after its creation time.
- Revoked Sessions cannot become active again.

---

## Lifecycle

Session currently supports the following lifecycle states:

- Active;
- Revoked.

Expiration is derived from time rather than represented as a lifecycle state.

A Session is considered valid only when:

- its lifecycle state is `Active`; and
- the current time is before its expiration time.

---

## Domain Behaviors

Candidate behaviors include:

- create;
- revoke.

Additional behaviors may be introduced as the domain evolves.

---

## Domain Errors

The initial domain error model has not yet been finalized.

Candidate domain errors include:

- InvalidLifecycleTransition;
- InvalidExpirationTime.

The final error model will be derived from accepted lifecycle rules.

---

## Implementation Status

Implemented:

- None.

Planned:

- SessionId;
- SessionLifecycleState;
- SessionError;
- Session aggregate.

---

## Out of Scope

The following concerns are intentionally outside the Session component:

- Authentication;
- Authorization;
- Credential verification;
- Cookies;
- JWT;
- Refresh tokens;
- Transport protocols.

---

## Open Questions

The following questions remain unresolved:

- Should Session reference `CredentialId`?
- Should Session support renewal?
- Should Session record client information?
- Should Session support inactivity timeout?

---

## Notes

This specification defines the Session domain model only.

It intentionally excludes implementation details such as:

- database schema;
- transport protocols;
- token formats;
- programming language.

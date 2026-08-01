# Session Discovery

**Status:** In Progress

---

## Goal

Understand the role, ownership, lifecycle, and boundaries of Session before defining its component specification or implementation.

---

## Guiding Questions

- What is a Session?
- When does a Session begin to exist?
- Which Identity owns a Session?
- Does a Session belong to a Client?
- Does a Session need to remember which Credential created it?
- Can one Identity have multiple active Sessions?
- What makes a Session valid?
- What ends a Session?
- Is expiration different from revocation?
- Can an expired or revoked Session become active again?

---

## Initial Observations

A Session represents an authenticated interaction.

A Session begins only after successful authentication.

A Session belongs to exactly one Identity.

An Identity may have zero, one, or multiple Sessions.

A Session has a lifecycle independent from Identity and Credential.

Removing a Session does not remove its owning Identity.

Revoking a Credential does not necessarily remove the historical Session record, although it may invalidate active Sessions through an application policy.

Session validity should not be defined by transport details such as cookies, HTTP headers, JWTs, or gRPC metadata.

---

## Candidate Responsibilities

Session may be responsible for:

- representing an authenticated interaction;
- referencing exactly one Identity;
- maintaining its own lifecycle;
- defining when the authenticated interaction expires;
- supporting explicit revocation;
- providing a stable reference for authentication state.

Session is not responsible for:

- verifying Credentials;
- authenticating an Identity;
- managing Identity lifecycle;
- managing Credential lifecycle;
- authorizing application actions;
- defining transport tokens or cookie formats.

---

## Candidate Invariants

The following rules are expected to hold:

- Every Session belongs to exactly one Identity.
- Every Session has exactly one identifier.
- Every Session has exactly one lifecycle state.
- Session expiration occurs after Session creation.
- Revoked Sessions cannot return to active use.
- Expired Sessions cannot return to active use.
- Removing a Session does not remove its owning Identity.

---

## Candidate Lifecycle States

The following states are currently being considered:

- Active;
- Expired;
- Revoked.

The final lifecycle and transition rules have not yet been accepted.

---

## Open Questions

- Should Session reference `ClientId` from the beginning?
- Should Session reference the `CredentialId` used during authentication?
- Is expiration a lifecycle transition or a derived condition based on time?
- Should Session support renewal or extension?
- Should revocation always be terminal?
- Should inactivity timeout and absolute expiration be modeled separately?
- Should a Session remain as a historical record after expiration or revocation?

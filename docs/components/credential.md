# Credential

**Version:** 0.1
**Status:** Draft

---

## Introduction

Credential is the domain concept representing proof of authentication.

A Credential belongs to exactly one Identity and enables authentication through a specific credential mechanism.

Credential is independent from the Identity lifecycle and owns its own lifecycle.

---

## Purpose

Credential provides verifiable proof that can be used by the Authentication domain to authenticate an Identity.

---

## Minimum Credential

The minimum Credential consists of:

- a unique identifier;
- an owning Identity identifier;
- a Credential Kind;
- a lifecycle state.

Credential does not require:

- a password hash;
- a passkey;
- an API key;
- OAuth metadata.

Those are implementation details of specific credential kinds.

---

## Responsibilities

Credential is responsible for:

- representing proof of authentication;
- belonging to exactly one Identity;
- maintaining its own lifecycle;
- identifying its Credential Kind.

---

## Non-Responsibilities

Credential is not responsible for:

- authenticating an Identity;
- creating Sessions;
- authorizing access;
- managing Identity lifecycle;
- storing profile information.

---

## Aggregate Boundary

Credential is an independent aggregate.

Credential references its owner through `IdentityId`.

Identity does not own Credential objects.

---

## Invariants

The following rules must always hold:

- Every Credential belongs to exactly one Identity.
- Every Credential has exactly one Credential Kind.
- Every Credential has exactly one lifecycle state.
- Removing a Credential does not remove its owning Identity.
- Credential identifiers are unique.

---

## Credential Kind

Credential currently supports the following conceptual kinds:

- Password;
- Passkey;
- ApiKey;
- OAuth.

Each Credential Kind shares the same ownership model.

---

## Lifecycle

Credential lifecycle has not yet been finalized.

Its lifecycle will be specified independently from the Identity lifecycle.

---

## Out of Scope

The following concerns are intentionally outside the Credential component:

- Authentication;
- Authorization;
- Session;
- Identity lifecycle;
- Profile management.

---

## Open Questions

- What lifecycle states should Credential support?
- Should all Credential kinds share the same lifecycle?
- Can multiple Credentials of the same kind belong to one Identity?
- Should OAuth remain a Credential or become an external identity link?

---

## Notes

This specification intentionally avoids implementation details such as:

- password hashing algorithms;
- passkey formats;
- database schema;
- transport protocols;
- programming language.

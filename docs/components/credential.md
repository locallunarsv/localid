# Credential

**Version:** 0.3  
**Status:** Implemented (Core v0.1)

---

## Introduction

Credential is the domain concept representing proof of authentication.

A Credential belongs to exactly one Identity and represents a single authentication mechanism.

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

Credential does **not** require:

- a password verifier;
- passkey data;
- API key material.

These are implementation details of specific Credential kinds.

---

## Responsibilities

Credential is responsible for:

- representing proof of authentication;
- belonging to exactly one Identity;
- maintaining the identifier of its owning Identity;
- maintaining its own lifecycle;
- identifying its Credential Kind.

---

## Non-Responsibilities

Credential is **not** responsible for:

- authenticating an Identity;
- creating Sessions;
- authorizing access;
- managing Identity lifecycle;
- storing profile information;
- enforcing authentication policy.

---

## Aggregate Boundary

Credential is an independent aggregate.

Credential references its owner through `IdentityId`.

Identity does not own Credential objects.

Changes to a Credential do not require modifications to the Identity aggregate.

---

## Invariants

The following rules must always hold:

- Every Credential belongs to exactly one Identity.
- Every Credential has exactly one Credential Kind.
- Every Credential has exactly one lifecycle state.
- Credential identifiers are unique.
- Removing or revoking a Credential does not remove its owning Identity.

---

## Credential Kind

Credential currently supports the following kinds:

- Password;
- Passkey;
- ApiKey.

Each Credential Kind shares the same ownership model.

Additional kinds may be introduced in future revisions.

---

## Lifecycle

A newly created Credential begins in the `Active` lifecycle state.

Credential currently supports the following lifecycle states:

- Active;
- Disabled;
- Revoked.

The following lifecycle transitions are supported:

```text
Active <────> Disabled

Active   ───► Revoked
Disabled ───► Revoked

Revoked is terminal.
```

Credential lifecycle is independent from the lifecycle of its owning Identity.

---

## Domain Behaviors

Credential currently supports the following behaviors:

- create;
- disable;
- enable;
- revoke.

Behavior rules:

- disabling is idempotent;
- enabling is idempotent;
- revoking is idempotent;
- revoked Credentials cannot be enabled;
- revoked Credentials cannot be disabled.

---

## Domain Errors

The current domain error model consists of:

- InvalidLifecycleTransition.

Additional domain errors may be introduced as the Credential domain evolves.

---

## Implementation Status

Implemented:

- CredentialId;
- CredentialKind;
- CredentialLifecycleState;
- CredentialError;
- Credential.

Not yet implemented:

- Password verifier model;
- Passkey data model;
- API key material;
- Repository abstraction;
- Domain events.

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

The following questions remain unresolved:

- Should additional Credential kinds be introduced?
- Should Credential expiration be modeled as a lifecycle state or as time-based metadata?
- Can multiple active Credentials of the same kind belong to one Identity?

---

## Notes

This specification defines the Credential domain model only.

It intentionally excludes implementation details such as:

- password hashing algorithms;
- passkey formats;
- API key generation;
- database schema;
- transport protocols;
- programming language.

# Identity

**Version:** 0.2
**Status:** Draft

## Introduction

Identity is the foundational domain concept of LocalID.

An Identity is the canonical and stable representation of a digital subject recognized and managed by LocalID.

An Identity may represent a human, a service account, or another type of digital subject.

Credentials, sessions, profile information, and authentication mechanisms may change throughout its lifetime, but the Identity remains the stable reference to that subject.

## Purpose

The purpose of Identity is to provide a stable and canonical reference for every digital subject managed by LocalID.

Other components may associate information or behavior with an Identity through its stable identifier.

## Minimum Identity

The minimum Identity consists of:

- a unique identity;
- a lifecycle state.

An Identity does not require:

- a username;
- a display name;
- an email address;
- profile information;
- a Credential;
- a Session.

These concepts may reference an Identity, but they do not define whether the Identity exists.

## Responsibilities

An Identity is responsible for:

- maintaining its canonical identity;
- maintaining its lifecycle;
- maintaining its lifecycle state;
- providing a stable reference for other components.

## Non-Responsibilities

An Identity is not responsible for:

- storing Credentials;
- storing passwords;
- authenticating a subject;
- creating Sessions;
- issuing Tokens;
- authorizing access;
- storing profile information;
- auditing activities.

These responsibilities belong to other components or concerns.

## Lifecycle

An Identity begins to exist when it is created.

During its lifetime, its lifecycle state may change. Changes to Credentials, Sessions, or profile information do not change whether the Identity exists.

The complete lifecycle and allowed state transitions will be defined separately.

## Lifecycle States

The lifecycle states have not yet been finalized.

The current candidates are:

- Active;
- Disabled;
- Locked;
- Deleted.

These states and their transition rules remain subject to a dedicated RFC.

## Invariants

The following statements must always remain true:

- Every Identity is uniquely identifiable.
- Every Identity has exactly one lifecycle state.
- Identity exists independently of Credentials.
- Identity exists independently of Sessions.
- Identity exists independently of profile information.
- Removing all Credentials does not remove the Identity.
- Profile changes do not change the canonical Identity.
- Other components reference Identity through its stable identifier.
- Identity does not own authentication or authorization behavior.

## Domain Events

Potential Identity domain events include:

- IdentityCreated;
- IdentityStateChanged;
- IdentityDisabled;
- IdentityEnabled;
- IdentityLocked;
- IdentityUnlocked;
- IdentityDeleted.

The final event model will be defined after the lifecycle and state-transition rules are accepted.

## Domain Errors

Potential domain errors include:

- IdentityNotFound;
- InvalidIdentityState;
- InvalidIdentityStateTransition.

The final error model will be derived from accepted domain rules.

## Out of Scope

The following concerns are intentionally outside the Identity component:

- Credential;
- Password;
- Authentication;
- Authorization;
- Session;
- Token;
- Profile;
- Client;
- Audit.

## Accepted Decisions

The following decision has been accepted:

- RFC-001: Identity is a stable and canonical reference, not a digital profile.

## Open Questions

The following questions remain unresolved:

- Which lifecycle states should Identity support?
- Which transitions should be allowed between lifecycle states?
- Can an Identity be permanently deleted?
- Can a deleted Identity be restored?
- Does Username belong to Identity or another component?
- Should Profile become a dedicated component?
- Should human and non-human subjects share the same Identity model?

## Notes

This document describes the Identity domain concept only.

It intentionally avoids implementation details such as programming language, identifier format, database, transport protocol, framework, or API design.

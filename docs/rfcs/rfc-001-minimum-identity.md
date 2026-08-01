# RFC-001: Minimum Identity

**Status:** Accepted

## Problem

What is the minimum information required for something to be considered an Identity within LocalID?

## Context

Identity is the foundational domain concept of LocalID.

Before defining attributes such as username, email, display name, or credentials, LocalID must establish what fundamentally makes something an Identity.

This decision defines the boundary of the Identity component and influences the design of every component that references an Identity.

## Considered Options

### Option A — Identity as a Stable Reference

Identity is the canonical and stable representation of a digital subject.

Its minimum information consists of:

- a unique identity;
- a lifecycle state.

Profile information, credentials, authentication data, and sessions are separate concerns.

#### Advantages

- Keeps the Identity model focused.
- Remains stable when associated information changes.
- Reduces coupling with other components.
- Supports both human and non-human subjects.
- Allows profile and authentication capabilities to evolve independently.

#### Trade-offs

- Related information must be obtained from other components.
- Additional components may be required as the platform grows.

### Option B — Identity as a Digital Profile

Identity directly owns information such as:

- username;
- display name;
- email address;
- avatar;
- phone number.

#### Advantages

- Fewer domain concepts for a small system.
- Profile data can be accessed directly from Identity.

#### Disadvantages

- Identity accumulates unrelated responsibilities.
- Profile changes affect the foundational identity model.
- Human-specific attributes may not apply to service accounts.
- The component becomes increasingly coupled as features grow.

## Decision

LocalID adopts **Identity as a Stable Reference**.

An Identity is the canonical representation of a digital subject within LocalID.

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

## Consequences

- Identity remains valid even when it has no Credential.
- Removing all Credentials does not remove the Identity.
- Profile information can change without changing the Identity.
- Human identities and service accounts may share the same foundational Identity model.
- Other components should reference Identity through its stable identifier.
- Identity lifecycle rules must be defined separately.

## Deferred Questions

The following questions are outside this RFC:

- Which lifecycle states exist?
- What transitions are allowed between lifecycle states?
- Can an Identity be permanently deleted?
- Does Username belong to Identity, Profile, or another component?
- Should human and service-account attributes use separate models?

These questions will be addressed in later RFCs.

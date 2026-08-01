# Identity

**Version:** 0.1
**Status:** Draft

---

# Introduction

Identity is the primary domain concept of LocalID.

It represents a digital subject that is known and managed by the platform.

Every authenticated interaction, session, credential, and client relationship ultimately belongs to an Identity.

Identity is the foundation upon which the rest of the platform is built.

---

# Purpose

The purpose of an Identity is to provide a stable and unique representation of a digital subject throughout its lifecycle.

Identity should remain independent from authentication mechanisms, transport protocols, storage technologies, and implementation details.

---

# Responsibilities

An Identity is responsible for:

- maintaining its own identity;
- maintaining its own lifecycle;
- maintaining its own status;
- maintaining its own profile information;
- providing a stable reference for other components.

---

# Non-Responsibilities

An Identity is **not** responsible for:

- storing passwords;
- storing credentials;
- authenticating users;
- creating sessions;
- issuing tokens;
- authorizing access;
- auditing activities.

These responsibilities belong to other components.

---

# Lifecycle

An Identity begins when it is created.

During its lifetime it may change status, update its profile information, become temporarily unavailable, or eventually be permanently removed from active use.

The lifecycle should be predictable and explicitly defined.

---

# States

The initial lifecycle states are expected to include:

- Active
- Disabled
- Locked
- Deleted

The exact transition rules will be defined separately.

---

# Invariants

The following statements should always remain true:

- Every Identity is uniquely identifiable.
- Every Identity has exactly one lifecycle state.
- An Identity never owns authentication logic.
- An Identity never owns credential data.
- An Identity remains the canonical reference used by other components.

Additional invariants may be introduced as the domain evolves.

---

# Domain Events

Identity may produce domain events such as:

- IdentityCreated
- IdentityUpdated
- IdentityDisabled
- IdentityEnabled
- IdentityLocked
- IdentityUnlocked
- IdentityDeleted

The event model will be specified separately.

---

# Domain Errors

Examples of domain-level errors include:

- IdentityAlreadyExists
- IdentityNotFound
- InvalidIdentityState
- InvalidStateTransition

The complete error model will be defined during implementation.

---

# Out of Scope

The following concepts are intentionally outside the Identity component:

- Credential
- Password
- Session
- Authentication
- Authorization
- Token
- Client
- Audit

---

# Open Questions

The following questions are intentionally left unanswered until the domain is better understood:

- Can an Identity exist without any Credential?
- Can a deleted Identity ever be restored?
- Can a username be changed?
- Can a username be reused after deletion?
- Is a display name required?
- Should service accounts and human identities share the same model?
- Should profile information belong to Identity or another component?

---

# Notes

This specification describes the domain concept only.

It intentionally avoids implementation details such as programming language, framework, database, transport protocol, identifier format, or API design.

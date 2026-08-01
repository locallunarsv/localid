# Credential Discovery

**Status:** In Progress

---

## Goal

Understand the role of Credential before defining its public API and implementation.

---

## Guiding Questions

- What is a Credential?
- Why does a Credential exist?
- Can an Identity exist without a Credential?
- Can a Credential exist without an Identity?
- Can one Identity own multiple Credentials?
- Should every Credential have the same lifecycle?
- Should different Credential kinds share the same model?

---

## Initial Observations

Credential is proof that allows a digital subject to authenticate.

Credential is not the digital subject itself.

Credential always belongs to exactly one Identity.

An Identity may own zero, one, or many Credentials.

Different Credential kinds may exist while sharing the same ownership model.

Removing a Credential does not remove its owning Identity.

Credential existence depends on Identity existence.

Authentication verifies Credentials rather than Identities directly.

---

## Candidate Responsibilities

Credential is responsible for:

- representing a proof of authentication;
- belonging to exactly one Identity;
- maintaining its own lifecycle;
- identifying its credential kind;
- exposing information required for verification.

Credential is not responsible for:

- authenticating an Identity;
- creating Sessions;
- authorizing requests;
- storing profile information;
- deciding authentication policy;
- managing Identity lifecycle.

---

## Candidate Invariants

(To be discussed)

---

## Candidate Credential Model

Credential is modeled as a single domain concept.

Different authentication mechanisms are represented by a Credential Kind rather than separate aggregate roots.

Candidate kinds include:

- Password;
- Passkey;
- ApiKey;
- OAuth.

Each Credential:

- belongs to exactly one Identity;
- has exactly one Credential Kind;
- owns its own lifecycle.

Authentication behavior depends on the Credential Kind rather than the aggregate structure.

## Open Questions

- Should every Credential Kind share the same lifecycle?
- Should OAuth be modeled as a Credential or as an external identity link?
- Can multiple Credentials of the same kind belong to one Identity?

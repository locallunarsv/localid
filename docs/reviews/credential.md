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

(To be discussed)

---

## Candidate Invariants

(To be discussed)

---

## Open Questions

(To be filled)

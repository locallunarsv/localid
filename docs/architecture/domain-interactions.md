# Domain Interactions

## Identity

Identity owns no other aggregates.

Identity provides a stable identifier that other domains reference.

---

## Credential

Credential references exactly one Identity.

Credential never modifies the Identity aggregate.

---

## Session

Session references exactly one Identity.

Session does not own or modify Credential objects.

Authentication determines whether a Credential is sufficient to create a Session.

---

## Dependency Graph

Authentication
│
▼
Session
│
▼
Credential
│
▼
Identity

---

## Interaction Rules

- Identity does not depend on Credential or Session.
- Credential depends only on Identity.
- Session depends only on Identity.
- Authentication orchestrates Identity, Credential, and Session.

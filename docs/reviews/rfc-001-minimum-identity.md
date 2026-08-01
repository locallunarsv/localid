# RFC-001: Minimum Identity

**Status:** Draft

---

# Problem

What is the minimum information required for something to be considered an Identity within LocalID?

---

# Motivation

Identity is the foundational domain concept of LocalID.

Before deciding attributes such as username, email, or display name, we must first understand what fundamentally makes something an Identity.

This decision will define the boundary of the Identity component and influence every other component in the platform.

---

# Discussion

Two design approaches are considered.

## Option A — Identity as a Stable Reference

Identity is a canonical reference representing a digital subject.

Profile information, authentication data, and other attributes belong to separate concerns.

Example:

- Identity ID
- Lifecycle Status

### Advantages

- Small and focused domain model.
- Stable over time.
- Easier to extend.
- Lower coupling between components.

---

## Option B — Identity as a Digital Profile

Identity owns profile information directly.

Example:

- Identity ID
- Username
- Display Name
- Email
- Avatar
- Phone Number

### Advantages

- Simpler for small applications.
- Fewer domain objects.

### Disadvantages

- Higher coupling.
- Identity grows quickly.
- Harder to evolve into a larger platform.

---

# Preliminary Conclusion

The current direction favors **Option A**.

Identity should primarily represent the canonical identity of a digital subject.

Profile information should remain a separate concern until there is sufficient domain evidence to merge it into the Identity component.

This is a working conclusion and may be revisited in the future.

---

# Open Questions

- Is Status part of the minimum Identity?
- Is Username part of Identity or Profile?
- Should Display Name belong to Identity?
- Can an Identity exist without profile information?
- Can an Identity exist without any Credential?

---

# Decision

**Not accepted yet.**

Further discussion is required before this RFC becomes an accepted architectural decision.

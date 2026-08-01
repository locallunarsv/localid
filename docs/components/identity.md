# Identity

**Version:** 0.3  
**Status:** Implemented (Core v0.1)

---

## Introduction

Identity is the foundational domain concept of LocalID.

An Identity is the canonical and stable representation of a digital subject managed by LocalID.

An Identity may represent a human, a service account, or another digital subject.

Credentials, Sessions, profile information, authentication mechanisms, and transport technologies may change throughout the lifetime of an Identity. None of these determine whether the Identity exists.

---

## Purpose

Identity provides a stable and canonical reference for every digital subject managed by LocalID.

Other domain components associate their own information with an Identity through its stable identifier.

---

## Minimum Identity

The minimum Identity consists of:

- a unique identifier;
- a lifecycle state.

Identity does **not** require:

- a username;
- a display name;
- an email address;
- profile information;
- a Credential;
- a Session.

These concepts reference an Identity but do not define its existence.

---

## Responsibilities

Identity is responsible for:

- maintaining its stable identifier;
- maintaining its lifecycle;
- enforcing lifecycle rules;
- providing a stable reference for other domain components.

---

## Non-Responsibilities

Identity is **not** responsible for:

- storing Credentials;
- storing passwords;
- authenticating subjects;
- creating Sessions;
- issuing Tokens;
- authorizing access;
- storing profile information;
- maintaining security lock state;
- auditing activities.

These concerns belong to other domain components.

---

## Lifecycle

An Identity begins to exist when it is created.

A newly created Identity always begins in the **Active** lifecycle state.

Throughout its lifetime the lifecycle state may change according to domain rules.

Changes to Credentials, Sessions, or profile information never determine whether an Identity exists.

---

## Lifecycle States

Identity currently supports three lifecycle states:

- Active;
- Disabled;
- Deleted.

Deleted is the terminal lifecycle state.

The following lifecycle transitions are currently supported:

```text
Active <────> Disabled

Active   ───► Deleted
Disabled ───► Deleted

Deleted is terminal.
```

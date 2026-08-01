# LocalID Domain Map

**Version:** 0.1

---

## Overview

LocalID is composed of multiple domain components.

Each component owns its own business rules and public API.

Dependencies between components should always point toward more fundamental domains.

---

## Domain Dependency

```text
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
```

Identity is the foundational domain.

No domain component below Identity currently exists.

---

## Planned Domain Components

### Identity

Responsible for:

- canonical identity;
- lifecycle;
- stable identifier.

Status:

- ✅ Implemented

---

### Credential

Responsible for:

- authentication credentials;
- password;
- passkey;
- API key;
- credential lifecycle.

Status:

- 🚧 Planned

---

### Session

Responsible for:

- authenticated sessions;
- expiration;
- revocation.

Status:

- 🚧 Planned

---

### Authentication

Responsible for:

- verifying credentials;
- creating sessions;
- enforcing authentication policies.

Status:

- 🚧 Planned

---

## Dependency Rules

Identity must not depend on any other LocalID domain.

Credential may depend on Identity.

Session may depend on:

- Identity;
- Credential.

Authentication may depend on:

- Identity;
- Credential;
- Session.

Circular dependencies are not allowed.

---

## Notes

This document defines architectural boundaries between domain components.

Implementation details are intentionally excluded.

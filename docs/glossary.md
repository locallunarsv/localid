# Glossary

**Version:** 0.1
**Status:** Draft

---

# Introduction

This document defines the ubiquitous language used throughout LocalID.

Every architectural discussion, documentation, implementation, and test should follow the terminology defined here.

Each concept has one meaning.

---

# Identity

## Definition

A digital subject recognized and managed by LocalID.

An Identity may represent a person, a service account, or another digital entity.

## Responsibilities

An Identity owns:

- unique identity;
- username;
- display name;
- lifecycle;
- status.

## Non-Responsibilities

An Identity does **not** own:

- credentials;
- passwords;
- authentication;
- authorization;
- sessions;
- tokens.

---

# Credential

## Definition

Proof that an Identity possesses something required for authentication.

## Examples

A Credential may be:

- password;
- passkey;
- recovery code;
- API key;
- client certificate.

A password is one type of Credential.

---

# Authentication

## Definition

The process of verifying that an Identity possesses a valid Credential.

Authentication answers the question:

> "Who are you?"

Authentication is a process, not a stored domain entity.

---

# Authorization

## Definition

The process of determining what an authenticated Identity is allowed to do.

Authorization answers the question:

> "What are you allowed to do?"

Authorization is separate from Authentication.

---

# Session

## Definition

A record representing an authenticated interaction between an Identity and a Client.

A Session begins after successful authentication and ends when it expires or is revoked.

---

# Client

## Definition

An application that relies on LocalID for identity services.

## Examples

Examples of Clients include:

- Loomnotes;
- internal dashboards;
- self-hosted applications;
- command-line tools.

A Client is an application, not a browser or a person.

---

# Token

## Definition

A secret value presented by a Client to reference or prove an authenticated Session.

A Token is an implementation detail of Session management.

A Token is **not** a Session.

---

# Password

## Definition

A human-memorable secret used as one type of Credential.

Passwords should never be stored in plain text.

---

# Password Hash

## Definition

A one-way cryptographic representation of a Password.

Password hashes are stored instead of plain-text passwords.

---

# Human Identity

## Definition

An Identity representing a human user.

---

# Service Account

## Definition

An Identity representing a non-human actor used by software or services.

---

# Component

## Definition

A cohesive part of LocalID that owns a specific area of the domain.

Each Component should have a single, clearly defined responsibility.

---

# Domain

## Definition

The business concepts and business rules managed by LocalID.

The domain is independent from implementation technologies.

---

# Ubiquitous Language Rule

Every important domain concept should be defined in this document before it becomes part of the implementation.

If a new concept cannot be clearly defined, it should not yet become part of the codebase.

---

# Preferred Terminology

| Prefer         | Avoid                 |
| -------------- | --------------------- |
| Identity       | User                  |
| Authentication | Auth                  |
| Authorization  | AuthZ                 |
| Credential     | Cred                  |
| Authenticate   | Login (domain layer)  |
| Revoke Session | Logout (domain layer) |

---

# Notes

This glossary is a living document.

New domain concepts should be added intentionally and remain consistent throughout the project.

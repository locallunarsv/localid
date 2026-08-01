# LocalID Documentation

Welcome to the LocalID documentation.

This directory contains the architectural knowledge, design decisions, and domain specifications that define the LocalID platform.

The documentation is considered part of the product and evolves together with the implementation.

---

# Reading Order

New contributors are encouraged to read the documents in the following order:

1. `project-charter.md`
2. `philosophy.md`
3. `glossary.md`
4. `components/`
5. `reviews/`

This order introduces the project's purpose, philosophy, terminology, stable specifications, and finally the ongoing design discussions.

---

# Directory Structure

## `project-charter.md`

Defines the project's vision, mission, goals, and scope.

---

## `philosophy.md`

Defines the architectural philosophy and guiding principles used throughout LocalID.

---

## `glossary.md`

Defines the ubiquitous language used across documentation, implementation, and discussions.

---

## `components/`

Contains stable domain specifications.

Each document describes a single domain component and acts as the primary source of truth for that component.

Examples:

- Identity
- Credential
- Session
- Authentication
- Client

---

## `reviews/`

Contains design discussions, open questions, RFCs, and ideas that have not yet become part of the official specification.

A review document may challenge existing assumptions or explore alternative designs.

Only accepted decisions should be promoted into the corresponding specification.

---

# Documentation Principles

Documentation should be:

- intentional;
- concise;
- technology-independent whenever possible;
- continuously maintained.

Documentation should explain **why** something exists before explaining **how** it is implemented.

---

# Documentation Workflow

Every major feature follows this lifecycle:

Review

↓

Discussion

↓

Accepted Decision

↓

Component Specification

↓

Implementation

↓

Tests

---

# Living Documentation

This documentation is expected to evolve alongside LocalID.

Changes should improve clarity without compromising consistency.

# Component Specifications

This directory contains the official domain specifications for LocalID components.

A component specification records accepted domain knowledge. It should describe what a component means, what it owns, and which rules it must preserve.

## Required Sections

A component specification should define, when applicable:

- purpose;
- responsibilities;
- non-responsibilities;
- lifecycle;
- states;
- invariants;
- domain events;
- domain errors;
- dependencies;
- out-of-scope concerns;
- accepted decisions;
- open questions.

## Rules

- Only accepted decisions belong in a component specification.
- Unresolved discussions belong in `../reviews/`.
- Formal design proposals and decisions belong in `../rfcs/`.
- Specifications should remain independent from transport, storage, frameworks, and deployment technologies.
- Implementation must follow the corresponding component specification.

## Current Components

- `identity.md` — canonical representation and lifecycle of a digital subject.

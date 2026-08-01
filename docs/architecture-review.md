# Architecture Review

**Version:** 0.1  
**Status:** Active

This document provides a recurring review checklist for LocalID architecture and documentation.

It is used to identify inconsistencies, accidental coupling, unclear responsibilities, and specification drift before implementation decisions become expensive to change.

## Project Direction

- [ ] The current design supports the Project Charter.
- [ ] The design follows the project Philosophy.
- [ ] New scope does not contradict established non-goals.
- [ ] Complexity is justified by a concrete requirement.

## Terminology

- [ ] Domain terms are defined consistently in the Glossary.
- [ ] Each important term has one meaning.
- [ ] Component specifications use established terminology.
- [ ] Unnecessary abbreviations and ambiguous aliases are avoided.

## Component Boundaries

- [ ] Every component has a clear purpose.
- [ ] Responsibilities and non-responsibilities are explicit.
- [ ] Components do not own concepts belonging to another component.
- [ ] Dependencies between components are intentional.
- [ ] No circular component dependency has been introduced.

## Domain Integrity

- [ ] Business rules are expressed as domain rules.
- [ ] Invariants are explicit and testable.
- [ ] Lifecycle states and transitions are defined where applicable.
- [ ] Invalid states are difficult to represent.
- [ ] Domain concepts are not shaped by persistence schemas.

## Technology Independence

- [ ] Domain specifications do not depend on transport protocols.
- [ ] Domain specifications do not depend on storage technologies.
- [ ] Domain specifications do not depend on frameworks.
- [ ] Implementation details are kept outside domain definitions.

## RFC Consistency

- [ ] Accepted RFCs are reflected in component specifications.
- [ ] Specifications do not contradict accepted RFCs.
- [ ] Superseded decisions are clearly identified.
- [ ] Significant architectural decisions have recorded rationale.

## Documentation Quality

- [ ] Documents have a clear purpose.
- [ ] Definitions are not unnecessarily duplicated.
- [ ] Open questions are separated from accepted decisions.
- [ ] Empty or obsolete documents have been removed.
- [ ] The documentation index reflects the current structure.

## Implementation Readiness

A component is ready for implementation only when:

- [ ] its purpose is understood;
- [ ] its boundary is stable enough;
- [ ] its core terminology is defined;
- [ ] its responsibilities are explicit;
- [ ] its main invariants are defined;
- [ ] its lifecycle is understood, when applicable;
- [ ] unresolved questions do not block implementation;
- [ ] accepted RFCs have been applied to its specification.

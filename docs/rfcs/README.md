# Requests for Comments

This directory contains formal design proposals and architectural decisions for LocalID.

RFCs are used when a decision has significant domain or architectural consequences, involves meaningful alternatives, or requires a permanent record of its rationale.

## RFC Lifecycle

An RFC may have one of the following statuses:

- **Draft** — the proposal is still being developed.
- **Accepted** — the proposal has been approved.
- **Rejected** — the proposal was considered but not adopted.
- **Superseded** — the decision has been replaced by another RFC.

## Workflow

1. Identify a significant design question.
2. Explore the question in `../reviews/` when discovery is still required.
3. Write an RFC containing the problem, context, alternatives, trade-offs, and proposed decision.
4. Review and either accept or reject the RFC.
5. Apply accepted decisions to the corresponding component specification.
6. Implement the accepted specification.
7. Preserve the RFC as part of the project's decision history.

## Rules

- One RFC should address one primary decision.
- Accepted RFCs must be reflected in the relevant specification.
- RFCs should explain why a decision was made, not only what was selected.
- Implementation-specific RFCs should not redefine established domain concepts.

## Index

- `rfc-001-minimum-identity.md` — defines Identity as a stable and canonical reference.

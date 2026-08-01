# Identity Lifecycle Discovery

**Status:** In Progress

## Goal

Understand the lifecycle of an Identity before defining its lifecycle states and state-transition rules.

## Guiding Questions

- When does an Identity begin to exist?
- When does an Identity stop existing?
- Can an Identity exist without Credentials?
- Can an Identity exist without Sessions?
- Should an Identity ever disappear?
- What does `Deleted` mean within LocalID?

## Observations

### Observation 1 — Creation establishes existence

An Identity begins to exist when it is created.

Authentication does not create an Identity. Authentication only verifies evidence associated with an existing Identity.

### Observation 2 — Credentials do not define existence

An Identity may exist without any Credential.

Credentials provide evidence for authentication, but they do not determine whether an Identity exists.

### Observation 3 — Sessions do not define existence

An Identity may exist without any active Session.

Sessions represent authenticated interactions and have a lifecycle independent from Identity.

### Observation 4 — Removing Credentials does not remove Identity

Removing all Credentials associated with an Identity does not remove the Identity.

Identity and Credential have separate responsibilities and lifecycles.

### Observation 5 — Identity is a canonical reference

Identity is the stable and canonical reference used by other LocalID components.

Its existence does not depend on profile information, authentication mechanisms, transport protocols, or storage technologies.

## Candidate States

The following lifecycle states are currently being considered:

- Active;
- Disabled;
- Locked;
- Deleted.

These are candidates only. Their meanings and permitted transitions have not yet been finalized.

## Open Questions

- Should `Deleted` be a lifecycle state or a terminal deletion action?
- Should an Identity ever be physically removed?
- Can a deleted Identity be restored?
- What is the precise difference between `Disabled` and `Locked`?
- Does every newly created Identity begin in the `Active` state?
- Are additional lifecycle states required?

## Current Direction

Identity continues to exist independently from Credentials and Sessions.

The meaning of deletion and the final lifecycle state model require further discovery before a formal RFC is written.

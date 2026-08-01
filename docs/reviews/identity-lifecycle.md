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

## Disabled and Locked Discovery

### Disabled

`Disabled` represents an intentional administrative decision that prevents an Identity from being used normally.

A disabled Identity:

- still exists;
- remains a canonical historical reference;
- cannot authenticate;
- does not become active automatically;
- requires an explicit administrative action to become enabled again.

Disabling is not necessarily caused by suspicious activity. Possible reasons include:

- temporary suspension;
- organizational policy;
- manual deactivation;
- the subject no longer requiring access.

### Locked

`Locked` represents a security or policy restriction applied because continued authentication attempts may be unsafe.

A locked Identity:

- still exists;
- cannot authenticate while locked;
- may be locked automatically by a security policy;
- may become unlockable after a condition is satisfied;
- may require manual intervention, automatic expiry, or another recovery process.

Possible causes include:

- repeated failed authentication attempts;
- suspected credential compromise;
- risk detection;
- explicit security action.

### Distinction

`Disabled` and `Locked` have different intent.

| State    | Primary intent                | Typical initiator                  | Recovery                                 |
| -------- | ----------------------------- | ---------------------------------- | ---------------------------------------- |
| Disabled | Administrative unavailability | Administrator or management policy | Explicit enable action                   |
| Locked   | Security protection           | Security policy or administrator   | Unlock action or policy-defined recovery |

A disabled Identity is unavailable because it should not currently be used.

A locked Identity is unavailable because using it may currently be unsafe.

## Preliminary Rules

The following rules are currently proposed:

- `Disabled` and `Locked` are distinct lifecycle states.
- Neither state removes the Identity.
- Neither state permits authentication.
- Disabling should not be treated as a security incident by default.
- Locking should preserve enough context to explain why the Identity was locked.
- Enabling and unlocking are separate domain behaviors.
- A generic `set_status` operation should not replace these behaviors.

## Additional Open Questions

- Can an Identity be disabled while it is locked?
- Should locking remember the previous lifecycle state?
- Can a disabled Identity be automatically locked?
- Does unlocking always produce `Active`?
- Can a lock expire automatically?
- Does enabling a previously locked Identity bypass the lock?
- Should lock reason and lock expiry belong to Identity or a security-policy component?

## Design Question

Should `Locked` be modeled as:

1. a lifecycle state; or
2. an independent security state?

Further investigation is required before finalizing the Identity state model.

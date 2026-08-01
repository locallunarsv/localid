# Identity Design Review

**Status:** In Progress

## Goal

Explore the responsibilities and domain behaviors of Identity before finalizing its lifecycle and state machine.

## Current Understanding

Identity is the stable and canonical representation of a digital subject within LocalID.

Its minimum form consists of:

- a unique identity;
- a lifecycle state.

Identity exists independently from Credentials, Sessions, and profile information.

## Candidate Behaviors

The following behaviors are currently considered part of the Identity lifecycle.

### Create

Creates a new Identity and establishes its existence within LocalID.

Questions still to resolve:

- Which lifecycle state should a newly created Identity receive?
- Can creation begin in a non-active state?

### Disable

Makes an Identity intentionally unavailable for normal use.

The precise effects of disabling still need to be defined.

### Enable

Returns a disabled Identity to an available lifecycle state.

The valid source states for this behavior still need to be defined.

### Lock

Restricts an Identity because of a security or policy condition.

Locking should remain conceptually different from administrative disabling.

### Unlock

Removes a lock condition from an Identity.

The destination state after unlocking still needs to be defined.

## Behaviors Requiring Further Discussion

### Delete

It has not yet been decided whether deletion means:

- transitioning to a terminal `Deleted` state;
- marking the Identity as historically inactive;
- physically removing the Identity;
- or using another lifecycle concept.

### Restore

Restore is only relevant if deletion is reversible.

No restore behavior should be accepted until the meaning of deletion is finalized.

## Deferred Behaviors

### Rename

Rename is intentionally deferred.

Username, display name, and profile ownership have not yet been assigned to the Identity component. Identity should not expose rename behavior until that ownership is established.

## Design Rules

Identity behavior should be expressed using domain language.

Preferred examples:

- `disable`;
- `enable`;
- `lock`;
- `unlock`.

Generic mutation operations such as `set_status` should be avoided because they can bypass lifecycle rules and obscure domain intent.

State changes should occur only through valid domain behaviors.

## Relationship Between Behavior and Lifecycle

Candidate behaviors will be used to derive:

- lifecycle states;
- valid state transitions;
- domain invariants;
- domain events;
- domain errors.

The state machine should not be finalized until these behaviors and their meanings are sufficiently understood.

## Open Questions

- What state does a newly created Identity enter?
- What exactly does disabling prevent?
- What conditions cause an Identity to become locked?
- Does unlocking always return an Identity to `Active`?
- Can a disabled Identity also be locked?
- Is deletion reversible?
- Should Identity ever be physically removed?
- Who or what may initiate each behavior?

## Current Direction

The likely initial lifecycle behaviors are:

- create;
- disable;
- enable;
- lock;
- unlock.

Delete and restore remain unresolved.

No lifecycle state machine has been accepted yet.

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

## Domain Scenarios

The following scenarios are used to validate the proposed Identity lifecycle model.

---

### Scenario 1 — Administrator disables an Identity

Initial State

- Lifecycle: Active
- Security: Unlocked

Action

Administrator disables the Identity.

Expected Result

- Lifecycle becomes Disabled.
- Security remains Unlocked.
- Authentication is denied.

Questions

- Should all active Sessions be revoked?
- Should all refresh tokens become invalid?

---

### Scenario 2 — Identity is locked after repeated failed authentication

Initial State

- Lifecycle: Active
- Security: Unlocked

Action

The authentication policy locks the Identity.

Expected Result

- Lifecycle remains Active.
- Security becomes Locked.
- Authentication is denied.

Questions

- Is unlocking automatic?
- Should the lock expire?

---

### Scenario 3 — Administrator enables a previously disabled Identity

Initial State

- Lifecycle: Disabled
- Security: Unlocked

Action

Administrator enables the Identity.

Expected Result

- Lifecycle becomes Active.
- Security remains Unlocked.

Questions

- Should enabling restore previous Sessions?

---

### Scenario 4 — Credentials are removed

Initial State

- Lifecycle: Active
- Security: Unlocked

Action

All Credentials are revoked.

Expected Result

- Identity still exists.
- Authentication is impossible.
- New Credentials may be added later.

This validates RFC-001.

---

### Scenario 5 — Identity is deleted

Initial State

- Lifecycle: Active
- Security: Unlocked

Action

Delete is requested.

Expected Result

Unknown.

Further domain discovery is required before the behavior can be defined.

This scenario motivates future RFCs.

### Scenario 6 — Active Identity is locked, then administratively disabled

Initial State

- Lifecycle: Active
- Security: Locked

Action

An administrator disables the Identity.

Expected Result

- Lifecycle becomes Disabled.
- Security remains Locked.
- Authentication remains denied.

Rationale

Administrative disabling should not silently remove an existing security restriction.

Questions

- Should the lock remain visible while the Identity is disabled?
- Should enabling the Identity require the lock to be resolved separately?

---

### Scenario 7 — Disabled and locked Identity is enabled

Initial State

- Lifecycle: Disabled
- Security: Locked

Action

An administrator enables the Identity.

Expected Result

- Lifecycle becomes Active.
- Security remains Locked.
- Authentication remains denied.

Rationale

Enabling changes administrative availability. It should not automatically override a security restriction.

Questions

- Should the administrator receive an explicit warning that the Identity remains locked?
- Should a combined enable-and-unlock operation ever be allowed?

---

### Scenario 8 — Lock expires while Identity is disabled

Initial State

- Lifecycle: Disabled
- Security: Locked

Action

The lock duration expires according to security policy.

Expected Result

- Lifecycle remains Disabled.
- Security becomes Unlocked.
- Authentication remains denied because the Identity is still disabled.

Rationale

Lifecycle and security restrictions should be evaluated independently.

Questions

- Should lock expiry be automatic?
- Should the expiry produce a domain event?
- Which component owns lock-expiration policy?

---

### Scenario 9 — Identity is unlocked while disabled

Initial State

- Lifecycle: Disabled
- Security: Locked

Action

An authorized actor unlocks the Identity.

Expected Result

- Lifecycle remains Disabled.
- Security becomes Unlocked.
- Authentication remains denied.

Rationale

Unlocking removes a security restriction but does not reverse an administrative decision.

Questions

- Who is authorized to unlock an Identity?
- Should unlocking require a reason or audit context?

---

### Scenario 10 — Authentication is attempted against a deleted Identity

Initial State

- Lifecycle: Deleted
- Security: Unlocked or Locked

Action

A Client attempts to authenticate as the Identity.

Expected Result

- Authentication is denied.
- No new Session is created.
- The Identity remains available only as a historical reference.

Rationale

A deleted Identity should not return to normal use through authentication.

Questions

- Is Security State still meaningful after deletion?
- Should deletion clear or preserve the previous security state?
- Should authentication reveal that the Identity is deleted, or return a generic failure?

## Deleted State Discovery

`Deleted` is currently considered a candidate terminal lifecycle state.

The purpose of this state is to preserve Identity as a historical and canonical reference while preventing any future operational use.

### Proposed Meaning

A deleted Identity:

- still exists as a historical reference;
- cannot authenticate;
- cannot create new Sessions;
- cannot receive new Credentials;
- cannot return to Active through ordinary lifecycle behavior;
- remains referentially valid for audit and historical records.

### Proposed Rules

- Deletion is a deliberate domain action.
- Deletion is distinct from administrative disabling.
- Deletion is terminal unless a future RFC introduces restoration.
- Deletion should revoke all active Sessions.
- Existing Credentials become unusable.
- Physical data removal is outside the Identity lifecycle.

### Deleted State Scenarios

#### Scenario 11 — Active Identity is deleted

**Initial State**

- Lifecycle: Active
- Security: Unlocked

**Expected Result**

- Lifecycle becomes Deleted.
- Authentication is denied.
- Active Sessions are revoked.
- Identity remains available as a historical reference.

---

#### Scenario 12 — Disabled Identity is deleted

**Initial State**

- Lifecycle: Disabled
- Security: Unlocked

**Expected Result**

- Lifecycle becomes Deleted.
- Authentication remains denied.
- Identity remains available as a historical reference.

---

#### Scenario 13 — Locked Identity is deleted

**Initial State**

- Lifecycle: Active
- Security: Locked

**Expected Result**

- Lifecycle becomes Deleted.
- Authentication remains denied.
- Historical lock information may still be retained for audit.

---

#### Scenario 14 — Enable is requested for a deleted Identity

**Expected Result**

- Operation is rejected.
- Lifecycle remains Deleted.

---

#### Scenario 15 — A new Credential is added to a deleted Identity

**Expected Result**

- Operation is rejected.
- No Credential is created.

## Candidate Lifecycle Model

Current candidate:

```text
Lifecycle

Active <────> Disabled

Active   ───► Deleted
Disabled ───► Deleted

Deleted is terminal.
```

Security remains independent:

```text
Security

Unlocked <────> Locked
```

## Remaining Questions

- Who is allowed to delete an Identity?
- Is deletion reversible?
- Should deleted Identities ever be physically removed?
- How long should deleted Identities be retained?

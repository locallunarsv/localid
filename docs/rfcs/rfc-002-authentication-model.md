# RFC-002 Authentication Model

## Status

Draft

## Problem

The current authentication model uses generic AuthenticationEvidence and
CredentialVerifier abstractions.

As additional credential kinds (Password, Passkey, API Key, OAuth) are
introduced, these abstractions become increasingly coupled through enum
variants and dispatch logic.

## Decision

Authentication will be modeled as credential-specific application services.

Examples:

- PasswordAuthenticationService
- PasskeyAuthenticationService
- ApiKeyAuthenticationService

Each service owns its own request model and verification flow.

Shared concepts remain limited to:

- AuthenticateResult
- SessionFactory
- Repository contracts

## Consequences

Advantages:

- Stronger type safety.
- No expanding AuthenticationEvidence enum.
- No generic verifier abstraction.
- Independent evolution of each authentication mechanism.

Trade-offs:

- More service types.
- Slightly more boilerplate.

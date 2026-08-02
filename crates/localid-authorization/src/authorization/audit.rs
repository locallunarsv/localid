use chrono::{DateTime, Utc};

use super::{AuthorizationDecision, AuthorizationRequest, Resource};

use localid_identity::IdentityId;
use localid_session::SessionId;

/// Authorization audit record.
///
/// Stores information about an authorization evaluation
/// for security auditing and tracing.
#[derive(Debug, Clone)]
pub struct AuthorizationAudit<'a, R>
where
    R: Resource + ?Sized,
{
    identity_id: IdentityId,
    session_id: SessionId,
    permission: String,
    resource_id: Option<String>,
    decision: AuthorizationDecision,
    created_at: DateTime<Utc>,
    _resource: core::marker::PhantomData<&'a R>,
}

impl<'a, R> AuthorizationAudit<'a, R>
where
    R: Resource + ?Sized,
{
    /// Creates an authorization audit record.
    #[must_use]
    pub fn new(
        identity_id: IdentityId,
        session_id: SessionId,
        request: &AuthorizationRequest<'a, R>,
        decision: AuthorizationDecision,
    ) -> Self {
        Self {
            identity_id,
            session_id,
            permission: request.permission().name().to_owned(),
            resource_id: request
                .resource()
                .map(|resource| resource.resource_id().to_owned()),
            decision,
            created_at: Utc::now(),
            _resource: core::marker::PhantomData,
        }
    }

    /// Returns evaluated identity.
    #[must_use]
    pub const fn identity_id(&self) -> IdentityId {
        self.identity_id
    }

    /// Returns evaluated session.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns permission name.
    #[must_use]
    pub fn permission(&self) -> &str {
        &self.permission
    }

    /// Returns resource identifier.
    #[must_use]
    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
    }

    /// Returns authorization decision.
    #[must_use]
    pub const fn decision(&self) -> &AuthorizationDecision {
        &self.decision
    }

    /// Returns audit timestamp.
    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::*;

    struct TestResource;

    impl crate::Resource for TestResource {
        fn resource_id(&self) -> &str {
            "test-resource"
        }
    }

    #[test]
    fn creates_authorization_audit() {
        let identity = IdentityId::new();
        let session = SessionId::new();

        let permission = crate::Permission::new("user.read").expect("permission valid");

        let request = crate::AuthorizationRequest::<TestResource>::without_resource(&permission);

        let audit =
            AuthorizationAudit::new(identity, session, &request, AuthorizationDecision::Allowed);

        assert_eq!(audit.permission(), "user.read");

        assert!(audit.created_at() <= Utc::now());
    }
}

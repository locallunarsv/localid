use super::{AuthorizationContext, AuthorizationDeniedReason, AuthorizationRequest, OwnedResource};

/// Authorization policy that allows access
/// when the requester owns the resource.
#[derive(Debug, Clone, Copy, Default)]
pub struct OwnershipAuthorizationPolicy;

impl OwnershipAuthorizationPolicy {
    /// Creates a new ownership authorization policy.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl<R> super::AuthorizationPolicy<R> for OwnershipAuthorizationPolicy
where
    R: OwnedResource + ?Sized,
{
    fn evaluate(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> Result<(), AuthorizationDeniedReason> {
        match request.resource() {
            Some(resource) if context.identity_id() == resource.owner_id() => Ok(()),

            Some(_) | None => Err(AuthorizationDeniedReason::ResourceOwnershipFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::OwnershipAuthorizationPolicy;

    use crate::{
        AuthorizationContext, AuthorizationDeniedReason, AuthorizationPolicy, AuthorizationRequest,
        OwnedResource, Permission, Resource,
    };

    struct TestDocument {
        id: String,
        owner_id: IdentityId,
    }

    impl Resource for TestDocument {
        fn resource_id(&self) -> &str {
            &self.id
        }
    }

    impl OwnedResource for TestDocument {
        fn owner_id(&self) -> IdentityId {
            self.owner_id
        }
    }

    #[test]
    fn allows_resource_owner() {
        let identity = IdentityId::new();

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: identity,
        };

        let permission = Permission::new("document.update").expect("permission should be valid");

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = OwnershipAuthorizationPolicy::new();

        assert!(policy.evaluate(&context, &request).is_ok());
    }

    #[test]
    fn denies_non_owner() {
        let identity = IdentityId::new();

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: IdentityId::new(),
        };

        let permission = Permission::new("document.update").expect("permission should be valid");

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = OwnershipAuthorizationPolicy::new();

        assert_eq!(
            policy.evaluate(&context, &request),
            Err(AuthorizationDeniedReason::ResourceOwnershipFailed)
        );
    }
}

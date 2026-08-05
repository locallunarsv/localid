use super::{
    AuthorizationContext, AuthorizationDeniedReason, AuthorizationPolicy, AuthorizationRequest,
    Resource,
};

/// Authorization policy that allows access when
/// any contained policy allows the request.
#[derive(Debug, Clone, Copy)]
pub struct AnyAuthorizationPolicy<A, B> {
    first: A,
    second: B,
}

impl<A, B> AnyAuthorizationPolicy<A, B> {
    /// Creates a policy that evaluates two policies using OR logic.
    #[must_use]
    pub const fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B, R> AuthorizationPolicy<R> for AnyAuthorizationPolicy<A, B>
where
    A: AuthorizationPolicy<R>,
    B: AuthorizationPolicy<R>,
    R: Resource + ?Sized,
{
    fn evaluate(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> Result<(), AuthorizationDeniedReason> {
        match self.first.evaluate(context, request) {
            Ok(()) => Ok(()),

            Err(first_error) => match self.second.evaluate(context, request) {
                Ok(()) => Ok(()),
                Err(_) => Err(first_error),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::AnyAuthorizationPolicy;

    use crate::{
        AuthorizationContext, AuthorizationPolicy, AuthorizationRequest, ExactPermissionMatcher,
        OwnedResource, OwnershipAuthorizationPolicy, Permission, Resource, Role,
        RoleBasedAuthorizationPolicy,
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
    fn allows_when_first_policy_allows() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.read").expect("permission should be valid");

        let role = Role::new("reader", vec![permission.clone()]).expect("role should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![role]);

        let request = AuthorizationRequest::<TestDocument>::without_resource(&permission);

        let policy = AnyAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_ok());
    }

    #[test]
    fn allows_when_second_policy_allows() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.read").expect("permission should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: identity,
        };

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = AnyAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_ok());
    }

    #[test]
    fn denies_when_all_policies_fail() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.read").expect("permission should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: IdentityId::new(),
        };

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = AnyAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_err());
    }
}

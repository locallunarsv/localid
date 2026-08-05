use super::{
    AuthorizationContext, AuthorizationDeniedReason, AuthorizationPolicy, AuthorizationRequest,
    Resource,
};

/// Authorization policy that allows access when
/// all contained policies allow the request.
#[derive(Debug, Clone, Copy)]
pub struct AllAuthorizationPolicy<A, B> {
    first: A,
    second: B,
}

impl<A, B> AllAuthorizationPolicy<A, B> {
    /// Creates a policy that evaluates two policies using AND logic.
    #[must_use]
    pub const fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B, R> AuthorizationPolicy<R> for AllAuthorizationPolicy<A, B>
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
        self.first.evaluate(context, request)?;
        self.second.evaluate(context, request)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::AllAuthorizationPolicy;

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
    fn allows_when_all_policies_allow() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.delete").expect("permission should be valid");

        let role = Role::new("admin", vec![permission.clone()]).expect("role should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![role]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: identity,
        };

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = AllAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_ok());
    }

    #[test]
    fn denies_when_owner_but_missing_role() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.delete").expect("permission should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: identity,
        };

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = AllAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_err());
    }

    #[test]
    fn denies_when_role_but_not_owner() {
        let identity = IdentityId::new();

        let permission = Permission::new("document.delete").expect("permission should be valid");

        let role = Role::new("admin", vec![permission.clone()]).expect("role should be valid");

        let context = AuthorizationContext::new(identity, SessionId::new(), vec![role]);

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner_id: IdentityId::new(),
        };

        let request = AuthorizationRequest::with_resource(&permission, &document);

        let policy = AllAuthorizationPolicy::new(
            RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new()),
            OwnershipAuthorizationPolicy::new(),
        );

        assert!(policy.evaluate(&context, &request).is_err());
    }
}

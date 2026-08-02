use super::{
    AuthorizationContext, AuthorizationDeniedReason, AuthorizationRequest, PermissionMatcher,
    Resource,
};

/// Authorization policy.
///
/// Defines business rules used to decide whether
/// an authorization request is allowed.
pub trait AuthorizationPolicy<R>
where
    R: Resource + ?Sized,
{
    /// Evaluates authorization request.
    ///
    /// Returns `Ok(())` when authorized.
    ///
    /// Returns [`AuthorizationDeniedReason`] when rejected.
    fn evaluate(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> Result<(), AuthorizationDeniedReason>;
}

/// Default role-based authorization policy.
///
/// Authorizes requests based on permissions
/// granted by roles in the authorization context.
#[derive(Debug, Clone, Copy)]
pub struct RoleBasedAuthorizationPolicy<M> {
    matcher: M,
}

impl<M> RoleBasedAuthorizationPolicy<M> {
    /// Creates a new role-based authorization policy.
    #[must_use]
    pub const fn new(matcher: M) -> Self {
        Self { matcher }
    }
}

impl<M, R> AuthorizationPolicy<R> for RoleBasedAuthorizationPolicy<M>
where
    M: PermissionMatcher,
    R: Resource + ?Sized,
{
    fn evaluate(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> Result<(), AuthorizationDeniedReason> {
        let allowed = context
            .roles()
            .iter()
            .flat_map(|role| role.permissions())
            .any(|granted| self.matcher.matches(granted, request.permission()));

        if allowed {
            Ok(())
        } else {
            Err(AuthorizationDeniedReason::MissingPermission)
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::{AuthorizationPolicy, RoleBasedAuthorizationPolicy};

    use crate::{
        AuthorizationContext, AuthorizationDeniedReason, AuthorizationRequest,
        ExactPermissionMatcher, Permission, Resource, Role,
    };

    struct TestResource;

    impl Resource for TestResource {
        fn resource_id(&self) -> &str {
            "resource-1"
        }
    }

    #[test]
    fn allows_permission_from_role() {
        let permission = Permission::new("user.read").expect("permission should be valid");

        let role = Role::new("reader", vec![permission.clone()]);

        let context = AuthorizationContext::new(IdentityId::new(), SessionId::new(), vec![role]);

        let request = AuthorizationRequest::<TestResource>::without_resource(&permission);

        let policy = RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new());

        assert!(policy.evaluate(&context, &request).is_ok());
    }

    #[test]
    fn denies_missing_permission() {
        let role = Role::new(
            "reader",
            vec![Permission::new("user.read").expect("permission should be valid")],
        );

        let context = AuthorizationContext::new(IdentityId::new(), SessionId::new(), vec![role]);

        let permission = Permission::new("user.delete").expect("permission should be valid");

        let request = AuthorizationRequest::<TestResource>::without_resource(&permission);

        let policy = RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new());

        assert_eq!(
            policy.evaluate(&context, &request),
            Err(AuthorizationDeniedReason::MissingPermission)
        );
    }
}

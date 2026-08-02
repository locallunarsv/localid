use super::{
    AuthorizationContext, AuthorizationDecision, AuthorizationPolicy, AuthorizationRequest,
    Resource,
};

/// Authorization evaluation service.
///
/// Evaluates whether an authorization context grants
/// access to a requested authorization request.
pub trait AuthorizationService<R>
where
    R: Resource + ?Sized,
{
    /// Evaluates authorization.
    fn authorize(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> AuthorizationDecision;
}

/// Default authorization service implementation.
///
/// Uses an injected authorization policy to evaluate
/// authorization decisions.
#[derive(Debug, Clone, Copy)]
pub struct DefaultAuthorizationService<P> {
    policy: P,
}

impl<P> DefaultAuthorizationService<P> {
    /// Creates a new authorization service.
    #[must_use]
    pub const fn new(policy: P) -> Self {
        Self { policy }
    }
}

impl<P, R> AuthorizationService<R> for DefaultAuthorizationService<P>
where
    P: AuthorizationPolicy<R>,
    R: Resource + ?Sized,
{
    fn authorize(
        &self,
        context: &AuthorizationContext,
        request: &AuthorizationRequest<'_, R>,
    ) -> AuthorizationDecision {
        match self.policy.evaluate(context, request) {
            Ok(()) => AuthorizationDecision::Allowed,

            Err(reason) => AuthorizationDecision::Denied { reason },
        }
    }
}

#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;
    use localid_session::SessionId;

    use super::{AuthorizationService, DefaultAuthorizationService};

    use crate::{
        AuthorizationContext, AuthorizationRequest, ExactPermissionMatcher, Permission, Role,
        RoleBasedAuthorizationPolicy,
    };

    fn context_with_role(role: Role) -> AuthorizationContext {
        AuthorizationContext::new(IdentityId::new(), SessionId::new(), vec![role])
    }

    struct TestResource;

    impl crate::Resource for TestResource {
        fn resource_id(&self) -> &str {
            "test-resource"
        }
    }

    #[test]
    fn allows_existing_permission() {
        let permission = Permission::new("user.read").expect("permission should be valid");

        let role = Role::new("reader", vec![permission.clone()]);

        let context = context_with_role(role);

        let request = AuthorizationRequest::<TestResource>::without_resource(&permission);

        let service = DefaultAuthorizationService::new(RoleBasedAuthorizationPolicy::new(
            ExactPermissionMatcher::new(),
        ));

        let decision = service.authorize(&context, &request);

        assert!(decision.is_allowed());
    }

    #[test]
    fn denies_missing_permission() {
        let role = Role::new(
            "reader",
            vec![Permission::new("user.read").expect("permission should be valid")],
        );

        let context = context_with_role(role);

        let permission = Permission::new("user.delete").expect("permission should be valid");

        let request = AuthorizationRequest::<TestResource>::without_resource(&permission);

        let service = DefaultAuthorizationService::new(RoleBasedAuthorizationPolicy::new(
            ExactPermissionMatcher::new(),
        ));

        let decision = service.authorize(&context, &request);

        assert!(decision.is_denied());
    }
}

use super::{AllAuthorizationPolicy, AnyAuthorizationPolicy};

/// Extension methods for composing authorization policies.
pub trait AuthorizationPolicyExt: Sized {
    /// Combines policies using OR logic.
    fn or<P>(self, other: P) -> AnyAuthorizationPolicy<Self, P>
    where
        P: Sized;

    /// Combines policies using AND logic.
    fn and<P>(self, other: P) -> AllAuthorizationPolicy<Self, P>
    where
        P: Sized;
}

impl<T> AuthorizationPolicyExt for T
where
    T: Sized,
{
    fn or<P>(self, other: P) -> AnyAuthorizationPolicy<Self, P>
    where
        P: Sized,
    {
        AnyAuthorizationPolicy::new(self, other)
    }

    fn and<P>(self, other: P) -> AllAuthorizationPolicy<Self, P>
    where
        P: Sized,
    {
        AllAuthorizationPolicy::new(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ExactPermissionMatcher, OwnershipAuthorizationPolicy, RoleBasedAuthorizationPolicy,
    };

    use super::AuthorizationPolicyExt;

    #[test]
    fn composes_policy_with_or() {
        let policy = RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new())
            .or(OwnershipAuthorizationPolicy::new());

        let _ = policy;
    }
    #[test]
    fn composes_policy_with_and() {
        let policy = RoleBasedAuthorizationPolicy::new(ExactPermissionMatcher::new())
            .and(OwnershipAuthorizationPolicy::new());

        let _ = policy;
    }
}

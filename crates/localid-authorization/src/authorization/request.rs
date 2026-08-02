use super::{Permission, Resource};

/// Represents an authorization request.
///
/// Contains the permission being requested and an optional
/// protected resource.
#[derive(Debug)]
pub struct AuthorizationRequest<'a, R>
where
    R: Resource + ?Sized,
{
    permission: &'a Permission,
    resource: Option<&'a R>,
}

impl<'a, R> AuthorizationRequest<'a, R>
where
    R: Resource + ?Sized,
{
    /// Creates a new authorization request without resource.
    #[must_use]
    pub const fn without_resource(permission: &'a Permission) -> Self {
        Self {
            permission,
            resource: None,
        }
    }

    /// Creates a new authorization request with resource.
    #[must_use]
    pub const fn with_resource(permission: &'a Permission, resource: &'a R) -> Self {
        Self {
            permission,
            resource: Some(resource),
        }
    }

    /// Returns requested permission.
    #[must_use]
    pub const fn permission(&self) -> &'a Permission {
        self.permission
    }

    /// Returns requested resource.
    #[must_use]
    pub const fn resource(&self) -> Option<&'a R> {
        self.resource
    }
}

#[cfg(test)]
mod tests {
    use super::AuthorizationRequest;
    use crate::Permission;

    struct TestResource;

    impl crate::Resource for TestResource {
        fn resource_id(&self) -> &str {
            "resource-1"
        }
    }

    #[test]
    fn creates_request_without_resource() {
        let permission = Permission::new("user.read").expect("permission should be valid");

        let request = AuthorizationRequest::<TestResource>::without_resource(&permission);

        assert_eq!(request.permission().name(), "user.read");

        assert!(request.resource().is_none());
    }

    #[test]
    fn creates_request_with_resource() {
        let permission = Permission::new("document.read").expect("permission should be valid");

        let resource = TestResource;

        let request = AuthorizationRequest::with_resource(&permission, &resource);

        assert!(request.resource().is_some());
    }
}

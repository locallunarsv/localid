use localid_identity::IdentityId;

use super::Resource;

/// Represents a resource owned by an identity.
pub trait OwnedResource: Resource {
    /// Returns the owner identity identifier.
    fn owner_id(&self) -> IdentityId;
}
#[cfg(test)]
mod tests {
    use localid_identity::IdentityId;

    use super::OwnedResource;
    use crate::Resource;

    struct TestDocument {
        id: String,
        owner: IdentityId,
    }

    impl Resource for TestDocument {
        fn resource_id(&self) -> &str {
            &self.id
        }
    }

    impl OwnedResource for TestDocument {
        fn owner_id(&self) -> IdentityId {
            self.owner
        }
    }

    #[test]
    fn returns_owner_identity() {
        let owner = IdentityId::new();

        let document = TestDocument {
            id: "doc-1".to_owned(),
            owner,
        };

        assert_eq!(document.owner_id(), owner);
    }
}

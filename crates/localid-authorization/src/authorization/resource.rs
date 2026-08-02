/// Represents a protected authorization resource.
///
/// Resources are domain objects that can be evaluated
/// by authorization policies.
pub trait Resource {
    /// Returns the stable resource identifier.
    fn resource_id(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::Resource;

    struct TestDocument {
        id: String,
    }

    impl Resource for TestDocument {
        fn resource_id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn returns_resource_identifier() {
        let document = TestDocument {
            id: "document-1".to_owned(),
        };

        assert_eq!(document.resource_id(), "document-1");
    }
}

/// Command for creating OAuth client.
#[derive(Debug, Clone)]
pub struct CreateOAuthClientCommand {
    name: String,
    redirect_uris: Vec<String>,
}

impl CreateOAuthClientCommand {
    /// Creates OAuth client command.
    #[must_use]
    pub fn new(name: impl Into<String>, redirect_uris: Vec<String>) -> Self {
        Self {
            name: name.into(),
            redirect_uris,
        }
    }

    /// Returns client name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns redirect URIs.
    #[must_use]
    pub fn redirect_uris(&self) -> &[String] {
        &self.redirect_uris
    }
}

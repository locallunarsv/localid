use localid_application::AuthorizationResult;

/// Builds OAuth authorization redirect URL.
pub fn build_authorization_redirect(redirect_uri: &str, result: &AuthorizationResult) -> String {
    let mut url = format!("{}?code={}", redirect_uri, result.code_id());

    if let Some(state) = result.request_state() {
        url.push_str("&state=");
        url.push_str(state);
    }

    url
}

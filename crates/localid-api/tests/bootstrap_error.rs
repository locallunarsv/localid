mod common;

use localid_api::bootstrap::{create_state_with_config, BootstrapError};
use localid_config::{Environment, ServerConfig};

use common::{test_database, test_lock};

#[tokio::test(flavor = "multi_thread")]
async fn bootstrap_should_return_crypto_error_for_invalid_signing_key_path() {
    let _guard = test_lock().lock().await;

    let mut server = ServerConfig::new("http://localhost:8080");

    server.signing_key_path = "/proc/localid/signing-key.pem".to_string();

    let error =
        match create_state_with_config(test_database(), server, Environment::Production).await {
            Ok(_) => panic!("invalid signing key path should fail bootstrap"),
            Err(error) => error,
        };

    assert!(matches!(error, BootstrapError::Crypto(_)));
}

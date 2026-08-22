mod common;

use common::{test_database, test_lock};

use localid_api::bootstrap::create_state;
use localid_config::Environment;

#[tokio::test(flavor = "multi_thread")]
async fn production_bootstrap_should_not_create_demo_seed() {
    let _guard = test_lock().lock().await;

    let bootstrap = create_state(test_database(), Environment::Production).await;

    assert!(bootstrap.demo_seed.is_none());
    assert!(bootstrap.credential_id.is_none());
    assert!(bootstrap.identity_id.is_none());
    assert!(bootstrap.client_id.is_none());
    assert!(bootstrap.oauth_client_id.is_none());
    assert!(bootstrap.oauth_client_public_id.is_none());
    assert!(bootstrap.oauth_client_secret.is_none());
    assert!(bootstrap.oauth_client_other_public_id.is_none());
}

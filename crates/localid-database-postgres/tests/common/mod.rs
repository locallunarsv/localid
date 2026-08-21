use std::{env, sync::OnceLock};

use localid_config::DatabaseConfig;
use tokio::sync::Mutex;

/// Returns database configuration for integration tests.
pub fn test_database() -> DatabaseConfig {
    let url = env::var("LOCALID_TEST_DATABASE_URL")
        .expect("LOCALID_TEST_DATABASE_URL must be set for integration tests");

    DatabaseConfig::new(url)
}

#[allow(dead_code)]
static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

/// Returns the global integration-test database lock.
#[allow(dead_code)]
pub fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

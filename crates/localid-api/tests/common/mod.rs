use localid_config::DatabaseConfig;
use std::sync::OnceLock;
use tokio::sync::Mutex;

pub fn test_database() -> DatabaseConfig {
    DatabaseConfig::new("postgres://localid_user:localid_password@localhost/localid")
}

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

pub fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

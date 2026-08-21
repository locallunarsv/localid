use std::{
    fs,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use localid_config::AppConfig;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn app_config_should_parse_from_toml() {
    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let config = AppConfig::from_toml(source).expect("application configuration should parse");

    assert_eq!(config.database.url(), "postgres://localhost/localid");
    assert_eq!(config.database.max_connections(), 10);

    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.server.issuer, "https://id.home.arpa");
    assert_eq!(
        config.server.signing_key_path,
        "~/.local/share/localid/keys/signing-key.pem"
    );
}

#[test]
fn app_config_should_load_from_file() {
    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    let config = AppConfig::from_file(&path).expect("application configuration file should load");

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.database.url(), "postgres://localhost/localid");
    assert_eq!(config.database.max_connections(), 10);
    assert_eq!(config.server.issuer, "https://id.home.arpa");
}

#[test]
fn app_config_should_load_from_configured_path() {
    let _guard = env_lock().lock().expect("environment lock should succeed");
    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-load-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    let previous_database_url = std::env::var_os("LOCALID_DATABASE_URL");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::remove_var("LOCALID_DATABASE_URL");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_CONFIG");

        if let Some(value) = previous_database_url {
            std::env::set_var("LOCALID_DATABASE_URL", value);
        }
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.database.url(), "postgres://localhost/localid");
    assert_eq!(config.server.issuer, "https://id.home.arpa");
}

#[test]
fn environment_should_override_database_url() {
    let _guard = env_lock().lock().expect("environment lock should succeed");
    let source = r#"
        environment = "development"

        [database]
        url = "postgres://file-value/localid"

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-env-override-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_DATABASE_URL", "postgres://env-value/localid");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_DATABASE_URL");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.database.url(), "postgres://env-value/localid");
}

#[test]
fn environment_should_override_issuer() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://file.example.com"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-issuer-override-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_ISSUER", "https://env.example.com");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_ISSUER");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.server.issuer, "https://env.example.com");
}

#[test]
fn environment_should_override_signing_key_path() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
        signing_key_path = "/file/keys/signing-key.pem"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path =
        std::env::temp_dir().join(format!("localid-config-signing-key-override-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_SIGNING_KEY_PATH", "/env/keys/signing-key.pem");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_SIGNING_KEY_PATH");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.server.signing_key_path, "/env/keys/signing-key.pem");
}

#[test]
fn environment_should_override_server_address() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        host = "127.0.0.1"
        port = 8080
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-server-override-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_SERVER_HOST", "0.0.0.0");
        std::env::set_var("LOCALID_SERVER_PORT", "9090");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_SERVER_PORT");
        std::env::remove_var("LOCALID_SERVER_HOST");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 9090);
}

#[test]
fn invalid_server_port_override_should_fail() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
        port = 8080
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!("localid-config-invalid-port-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_SERVER_PORT", "invalid");
    }

    let error = AppConfig::load().expect_err("invalid server port should fail");

    unsafe {
        std::env::remove_var("LOCALID_SERVER_PORT");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(error.to_string(), "invalid LOCALID_SERVER_PORT: invalid");
}

#[test]
fn environment_should_override_database_max_connections() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"
        max_connections = 10

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path =
        std::env::temp_dir().join(format!("localid-config-db-max-connections-{unique}.toml"));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_DATABASE_MAX_CONNECTIONS", "25");
    }

    let config = AppConfig::load().expect("application configuration should load");

    unsafe {
        std::env::remove_var("LOCALID_DATABASE_MAX_CONNECTIONS");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(config.database.max_connections(), 25);
}

#[test]
fn invalid_database_max_connections_override_should_fail() {
    let _guard = env_lock().lock().expect("environment lock should succeed");

    let source = r#"
        environment = "development"

        [database]
        url = "postgres://localhost/localid"

        [server]
        issuer = "https://id.home.arpa"
    "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be valid")
        .as_nanos();

    let path = std::env::temp_dir().join(format!(
        "localid-config-invalid-db-max-connections-{unique}.toml"
    ));

    fs::write(&path, source).expect("temporary configuration should be written");

    unsafe {
        std::env::set_var("LOCALID_CONFIG", &path);
        std::env::set_var("LOCALID_DATABASE_MAX_CONNECTIONS", "invalid");
    }

    let error = AppConfig::load().expect_err("invalid database max connections should fail");

    unsafe {
        std::env::remove_var("LOCALID_DATABASE_MAX_CONNECTIONS");
        std::env::remove_var("LOCALID_CONFIG");
    }

    fs::remove_file(&path).expect("temporary configuration should be removed");

    assert_eq!(
        error.to_string(),
        "invalid LOCALID_DATABASE_MAX_CONNECTIONS: invalid"
    );
}

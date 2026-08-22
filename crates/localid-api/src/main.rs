use std::net::SocketAddr;

use localid_api::{bootstrap::create_state_with_config, create_router};

use localid_config::AppConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::load().unwrap_or_else(|error| {
        eprintln!("Failed to load LocalID configuration: {error}");
        std::process::exit(1);
    });

    let address = format!("{}:{}", config.server.host, config.server.port)
        .parse::<SocketAddr>()
        .unwrap_or_else(|error| {
            eprintln!("Invalid server address: {error}");
            std::process::exit(1);
        });

    let bootstrap = create_state_with_config(config.database, config.server, config.environment)
        .await
        .unwrap_or_else(|error| {
            eprintln!("Failed to initialize LocalID: {error}");
            std::process::exit(1);
        });

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    println!("LocalID API listening on {address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|error| {
            eprintln!("Failed to bind LocalID server on {address}: {error}");
            std::process::exit(1);
        });

    if let Err(error) = axum::serve(listener, app).await {
        eprintln!("LocalID server error: {error}");
        std::process::exit(1);
    }
}

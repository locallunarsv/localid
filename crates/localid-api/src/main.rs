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

    let bootstrap =
        create_state_with_config(config.database, config.server, config.environment).await;

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    println!("LocalID API listening on {address}");

    axum::serve(
        tokio::net::TcpListener::bind(address)
            .await
            .expect("failed to bind server"),
        app,
    )
    .await
    .expect("server error");
}

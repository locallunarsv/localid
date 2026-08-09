use std::net::SocketAddr;

use localid_api::{bootstrap::create_state, create_router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bootstrap = create_state();

    println!("Demo credential_id: {}", bootstrap.credential_id);
    println!("Demo client_id: {}", bootstrap.client_id);
    println!("Demo oauth_client_id: {}", bootstrap.oauth_client_id);
    println!(
        "Demo oauth_client_public_id: {}",
        bootstrap.oauth_client_public_id
    );

    let app = create_router(
        bootstrap.state,
        bootstrap.auth_state,
        bootstrap.authorization_state,
    );

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("LocalID API listening on {}", address);

    axum::serve(
        tokio::net::TcpListener::bind(address)
            .await
            .expect("failed to bind server"),
        app,
    )
    .await
    .expect("server error");
}

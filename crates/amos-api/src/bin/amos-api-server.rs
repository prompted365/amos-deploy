use amos_api::{create_app, AppState};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Get configuration from environment
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "development-secret-key".to_string());

    // Create application state
    let state = AppState::new(jwt_secret);

    // Create the application
    let app = create_app(state);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("AMOS API server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
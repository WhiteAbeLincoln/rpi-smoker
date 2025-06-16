use axum::{
    response::Json,
    routing::get,
    Router,
};
use clap::Parser;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to bind the server to
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host to bind the server to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Enable hardware features (GPIO, sensors)
    #[arg(long)]
    enable_hardware: bool,
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rpi-smoker-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "hardware_enabled": cfg!(feature = "rpi-hardware")
    }))
}

async fn hello_world() -> Json<Value> {
    Json(json!({
        "message": "Hello from RPI Smoker Backend!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rpi_smoker_backend=debug,tower_http=debug".into()),
        )
        .init();

    let args = Args::parse();

    // Check if we're on a Raspberry Pi or hardware features are enabled
    if args.enable_hardware {
        #[cfg(all(feature = "rpi-hardware", target_os = "linux"))]
        {
            info!("Hardware features enabled");
            // TODO: Initialize GPIO and sensor interfaces
        }
        #[cfg(not(all(feature = "rpi-hardware", target_os = "linux")))]
        {
            warn!("Hardware features requested but not available on this platform or not compiled in.");
            warn!("To enable hardware features: use --features rpi-hardware on Linux");
        }
    } else {
        info!("Running in development mode without hardware features");
    }

    let app = create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Starting server on {}:{}", args.host, args.port);
    info!("Health check available at http://{}:{}/health", args.host, args.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

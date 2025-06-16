use clap::Parser;
use rpi_smoker_backend::{
    api::create_router,
    config::{AppConfig, HardwareConfig},
};
use std::net::SocketAddr;
use tokio::signal;
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

    /// Request timeout in seconds
    #[arg(long, default_value = "30")]
    timeout: u64,

    /// Enable hardware features (GPIO, sensors)
    #[arg(long)]
    enable_hardware: bool,
}

fn create_app_config(args: &Args) -> AppConfig {
    AppConfig {
        port: args.port,
        host: args.host.clone(),
        request_timeout_seconds: args.timeout,
        hardware: HardwareConfig {
            enable_gpio: args.enable_hardware,
            ..Default::default()
        },
        ..Default::default()
    }
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
    let config = create_app_config(&args);

    // Check if we're on a Raspberry Pi or hardware features are enabled
    if config.hardware.enable_gpio {
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

    let app = create_router(&config);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Starting server on {}:{}", config.host, config.port);
    info!(
        "Health check available at http://{}:{}/api/health",
        config.host, config.port
    );
    info!("API routes available under /api/*");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Run server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Handle graceful shutdown signals
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("Received SIGTERM, starting graceful shutdown");
        },
    }
}

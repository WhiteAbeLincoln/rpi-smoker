use clap::Parser;
use rpi_smoker::{api::create_router, config::CliOverrides};
use rpi_smoker::{config::AppConfig, state::AppState};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::signal;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about = "Raspberry Pi Smoker Controller", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "config.json")]
    pub config: PathBuf,

    /// Host to bind the server to
    #[arg(long)]
    pub host: Option<String>,

    #[arg(long)]
    pub port: Option<u16>,

    /// Request timeout in seconds
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Disable hardware features and run in mock mode
    #[arg(long)]
    pub mock_mode: Option<bool>,
}

fn create_cli_overrides(args: &Cli) -> CliOverrides {
    CliOverrides {
        port: args.port,
        host: args.host.clone(),
        timeout: args.timeout,
        mock_mode: args.mock_mode,
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

    let args = Cli::parse();

    // Load configuration
    let cli_overrides = create_cli_overrides(&args);
    let config = AppConfig::load(&args.config, Some(&cli_overrides))
        .map_err(|e| {
            eprintln!("Failed to load configuration: {e}");
            std::process::exit(1);
        })
        .unwrap();

    info!("Configuration loaded from: {}", args.config.display());

    // Check if we're on a Raspberry Pi or hardware features are enabled
    if !config.hardware.mock_mode {
        #[cfg(all(feature = "rpi-hardware", target_os = "linux"))]
        {
            info!("Hardware features enabled");
            // TODO: Initialize GPIO and sensor interfaces
        }
        #[cfg(not(all(feature = "rpi-hardware", target_os = "linux")))]
        {
            warn!(
                "Hardware features requested but not available on this platform or not compiled in."
            );
            warn!("To enable hardware features: use --features rpi-hardware on Linux");
        }
    } else {
        info!("Running in mock mode without hardware features");
    }

    let port = config.server.port;
    let host: &str = &config.server.host;
    let timeout = config.server.request_timeout_seconds;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting server on {}:{}", host, port);
    info!(
        "Health check available at http://{}:{}/api/health",
        host, port
    );
    info!("API routes available under /api/*");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let shared_state = AppState::new(config);
    let app = create_router(timeout, &shared_state);
    // TODO: spawn thread for sensors

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

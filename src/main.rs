use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use drift_rs::telemetry::state::SharedState;
use drift_rs::api;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3030)]
    port: u16,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Memory buffer capacity for telemetry
    #[arg(short, long, default_value_t = 1000)]
    capacity: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Advanced Tracing Configuration
    let level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize Shared State
    let state = Arc::new(SharedState::new(args.capacity));

    let app = api::create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Drift telemetry engine listening on {}", addr);
    info!("Buffer capacity: {} packets", args.capacity);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

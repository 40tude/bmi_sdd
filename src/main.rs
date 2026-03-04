// Rust guideline compliant 2026-02-16

//! Entry point: CLI parsing, tracing initialisation, and server startup.

use mimalloc::MiMalloc;

// M-MIMALLOC-APPS: use mimalloc for improved allocation performance
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

/// BMI calculator web service.
#[derive(Debug, Parser)]
#[command(about = "Stateless BMI calculator web service")]
struct Cli {
    /// TCP port to listen on (overridden by PORT environment variable).
    #[arg(long, default_value_t = 3000)]
    port: u16,
    /// Tracing log level filter (e.g. debug, info, warn, error).
    #[arg(long, default_value = "info")]
    log_level: String,
}

// --- T023: Port resolution with env var override ---

/// Resolves the effective port from an optional `PORT` env value and `cli_port` fallback.
///
/// Accepts the env value as a parameter for deterministic, mutation-free testing.
/// Callers should pass `std::env::var("PORT").ok()` as the first argument.
///
/// Spec FR-009: environment variable wins over CLI argument.
fn resolve_port_inner(env_port: Option<String>, cli_port: u16) -> u16 {
    env_port.and_then(|v| v.parse().ok()).unwrap_or(cli_port)
}

/// Resolves the effective port: `PORT` env var takes precedence over `cli_port`.
#[must_use]
pub fn resolve_port(cli_port: u16) -> u16 {
    resolve_port_inner(std::env::var("PORT").ok(), cli_port)
}

// --- T012: Main entry point ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let port = resolve_port(cli.port);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_new(&cli.log_level).unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::event!(
        name: "server.listen.start",
        tracing::Level::INFO,
        server.address = %addr,
        "server listening on {{server.address}}",
    );

    axum::serve(listener, bmi_sdd::build_router()).await?;
    Ok(())
}

// --- T023: Unit tests for resolve_port ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_env_var_overrides_cli() {
        assert_eq!(resolve_port_inner(Some("8080".to_owned()), 3000), 8080);
    }

    #[test]
    fn cli_port_used_when_env_absent() {
        assert_eq!(resolve_port_inner(None, 3000), 3000);
    }

    #[test]
    fn invalid_port_env_var_falls_back_to_cli() {
        assert_eq!(
            resolve_port_inner(Some("not-a-number".to_owned()), 3000),
            3000
        );
    }
}

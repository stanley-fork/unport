mod client;
mod config;
mod daemon;
mod detect;
mod process;
mod proxy;
mod types;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unport")]
#[command(about = "Local development port manager with automatic domain routing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage the unport daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonAction,
    },
    /// Start the app in current directory and register with daemon
    Start,
    /// Stop a running service by domain
    Stop {
        /// Domain name to stop
        domain: String,
    },
    /// List all registered services
    List,
}

#[derive(Subcommand)]
enum DaemonAction {
    /// Start the daemon
    Start {
        /// Run daemon in background (detached)
        #[arg(short = 'd', long = "detach")]
        detach: bool,
    },
    /// Stop the daemon
    Stop,
    /// Show daemon status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Daemon { action } => match action {
            DaemonAction::Start { detach } => daemon::run(detach).await,
            DaemonAction::Stop => client::stop_daemon().await,
            DaemonAction::Status => client::daemon_status().await,
        },
        Commands::Start => client::start().await,
        Commands::Stop { domain } => client::stop_service(&domain).await,
        Commands::List => client::list().await,
    }
}

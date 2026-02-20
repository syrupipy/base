//! Mempool rebroadcaster binary entry point.

use base_cli_utils::{LogConfig, LogFormat, LogLevel, StdoutLogConfig};
use clap::Parser;
use dotenvy::dotenv;
use mempool_rebroadcaster::Rebroadcaster;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about = "A mempool rebroadcaster service")]
struct Args {
    #[arg(long, env, required = true)]
    geth_mempool_endpoint: String,

    #[arg(long, env, required = true)]
    reth_mempool_endpoint: String,

    #[arg(long, env, default_value = "info")]
    log_level: LogLevel,

    /// Log format
    #[arg(long, env, default_value = "text")]
    log_format: LogFormat,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    LogConfig {
        global_level: args.log_level.into(),
        stdout_logs: Some(StdoutLogConfig { format: args.log_format }),
        file_logs: None,
    }
    .init_tracing_subscriber()
    .expect("Failed to initialize tracing");

    let rebroadcaster = Rebroadcaster::new(args.geth_mempool_endpoint, args.reth_mempool_endpoint);
    let result = rebroadcaster.run().await;

    match result {
        Ok(result) => {
            info!(
                success_geth_to_reth = result.success_geth_to_reth,
                success_reth_to_geth = result.success_reth_to_geth,
                unexpected_failed_geth_to_reth = result.unexpected_failed_geth_to_reth,
                unexpected_failed_reth_to_geth = result.unexpected_failed_reth_to_geth,
                "finished broadcasting txns",
            );
        }
        Err(e) => {
            error!(error = ?e, "error running rebroadcaster");
            std::process::exit(1);
        }
    }
}

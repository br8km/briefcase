use anyhow::Result;
use briefcase::{cli, logging};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    if matches!(&cli.command, cli::Commands::Version) {
        return cli::run(cli).await;
    }

    // Initialize logging
    let log_dir = logging::get_log_dir()?;
    std::fs::create_dir_all(&log_dir)?;
    logging::init_logging(&log_dir)?;

    cli::run(cli).await
}

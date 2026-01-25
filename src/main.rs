use anyhow::Result;
use briefcase::cli;
use briefcase::logging;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let log_dir = logging::get_log_dir()?;
    std::fs::create_dir_all(&log_dir)?;
    logging::init_logging(&log_dir)?;

    let cli = cli::Cli::parse();
    cli::run(cli).await
}

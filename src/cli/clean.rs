use crate::config;
use anyhow::Result;
use clap::Args;
use log::info;
use std::path::PathBuf;

#[derive(Args)]
pub struct CleanArgs {
    /// Skip confirmation prompt
    #[arg(long, short)]
    pub force: bool,
}

pub async fn run(args: CleanArgs) -> Result<()> {
    let data_dir = config::get_data_dir()?;
    let log_dir = config::get_log_dir()?;

    if !args.force {
        println!(
            "This will delete all files in:\n  - {}\n  - {}",
            data_dir.display(),
            log_dir.display()
        );
        println!("\nAre you sure? [y/N]: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    clean_directory(&data_dir, "data")?;
    clean_directory(&log_dir, "logs")?;

    println!("Cleanup complete.");
    Ok(())
}

fn clean_directory(dir: &PathBuf, name: &str) -> Result<()> {
    if !dir.exists() {
        info!("{} directory does not exist, skipping", name);
        return Ok(());
    }

    let entries: Vec<_> = std::fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();

    if entries.is_empty() {
        println!("{} directory is already empty", name);
        return Ok(());
    }

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        } else {
            std::fs::remove_file(&path)?;
        }
        println!("Removed: {}", path.display());
    }

    Ok(())
}

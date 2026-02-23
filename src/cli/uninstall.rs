use crate::config;
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::process::Command;

#[derive(Args)]
pub struct UninstallArgs {
    /// Delete all data, logs, config, and the binary itself
    #[arg(long, short)]
    pub all: bool,
}

pub async fn run(args: UninstallArgs) -> Result<()> {
    let data_dir = config::get_data_dir()?;
    let log_dir = config::get_log_dir()?;
    let config_path = config::get_config_path()?;

    clean_directory(&data_dir, "data")?;
    clean_directory(&log_dir, "logs")?;

    if !args.all {
        println!(
            "This will delete all files in:\n  - {}\n  - {}\n  - The binary itself",
            data_dir.display(),
            log_dir.display()
        );
        println!("\nDo you also want to delete the config file? [y/N]: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Uninstalled (data, logs, binary). Config file kept.");
            delete_binary()?;
            return Ok(());
        }
    }

    if config_path.exists() {
        std::fs::remove_file(&config_path)?;
        println!("Removed config: {}", config_path.display());
    }

    if let Some(config_dir) = config_path.parent() {
        if config_dir.exists()
            && config_dir
                .read_dir()
                .map(|mut d| d.next().is_none())
                .unwrap_or(true)
        {
            std::fs::remove_dir(config_dir)?;
            println!("Removed config directory: {}", config_dir.display());
        }
    }

    println!("Uninstalled everything.");
    delete_binary()?;

    Ok(())
}

fn clean_directory(dir: &PathBuf, name: &str) -> Result<()> {
    if !dir.exists() {
        println!("{} directory does not exist, skipping", name);
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

fn delete_binary() -> Result<()> {
    let exe_path = std::env::current_exe()?;

    println!("Deleting binary: {}", exe_path.display());

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args([
                "/C",
                "timeout",
                "/T",
                "1",
                "/NOBREAK",
                "&",
                "del",
                "/F",
                "/Q",
                &exe_path.display().to_string(),
            ])
            .spawn()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("sh")
            .args([
                "-c",
                &format!("sleep 1 && rm -f \"{}\"", exe_path.display()),
            ])
            .spawn()?;
    }

    println!("Uninstall complete. Goodbye!");
    std::process::exit(0);
}

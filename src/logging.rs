use chrono::Local;
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn init_logging(log_dir: &Path) -> anyhow::Result<()> {
    let log_dir = log_dir.to_path_buf();
    env_logger::Builder::new()
        .format(move |buf, record| {
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%z");

            // Get current monthly log file
            let current_month = Local::now().format("%Y-%m").to_string();
            let log_filename = format!("{}.log", current_month);
            let log_path = log_dir.join(&log_filename);

            // Write directly to the file
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
                let _ = writeln!(
                    file,
                    "{} {} {}: {}",
                    timestamp,
                    record.level(),
                    record.target(),
                    record.args()
                );
            }

            // Also write to stderr for console output
            writeln!(
                buf,
                "{} {} {}: {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    Ok(())
}

pub fn get_log_dir() -> anyhow::Result<PathBuf> {
    crate::config::get_log_dir()
}

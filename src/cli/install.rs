#[cfg(unix)]
use anyhow::Context;
use anyhow::{anyhow, Result};
use clap::Args;
#[cfg(unix)]
use std::path::Path;
use std::path::PathBuf;

#[derive(Args)]
pub struct InstallArgs {
    /// Destination directory for the binary
    #[arg(long, value_name = "DIR")]
    pub path: Option<PathBuf>,
}

pub async fn run(args: InstallArgs) -> Result<()> {
    #[cfg(not(unix))]
    {
        let _ = args;
        return Err(anyhow!(
            "The install command is currently supported on Unix/Linux only"
        ));
    }

    #[cfg(unix)]
    {
        let source = std::env::current_exe().context("Failed to determine the current binary")?;
        let destination_dir = args.path.unwrap_or(default_bin_dir()?);
        let destination = install_binary(&source, &destination_dir)?;

        println!("Installed binary: {}", destination.display());

        Ok(())
    }
}

#[cfg(unix)]
fn default_bin_dir() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("XDG_BIN_HOME") {
        if !path.is_empty() {
            return Ok(PathBuf::from(path));
        }
    }

    dirs::home_dir()
        .map(|home| home.join(".local").join("bin"))
        .ok_or_else(|| {
            anyhow!("Could not determine the home directory for the default install path")
        })
}

#[cfg(unix)]
fn install_binary(source: &Path, destination_dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(destination_dir).with_context(|| {
        format!(
            "Failed to create install directory: {}",
            destination_dir.display()
        )
    })?;

    let destination = destination_dir.join(env!("CARGO_PKG_NAME"));
    if same_file(source, &destination)? {
        return Ok(destination);
    }

    let permissions = std::fs::metadata(source)?.permissions();
    std::fs::copy(source, &destination)
        .with_context(|| format!("Failed to install binary to {}", destination.display()))?;
    std::fs::set_permissions(&destination, permissions)?;

    Ok(destination)
}

#[cfg(unix)]
fn same_file(source: &Path, destination: &Path) -> Result<bool> {
    if !destination.exists() {
        return Ok(false);
    }

    Ok(std::fs::canonicalize(source)? == std::fs::canonicalize(destination)?)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_install_binary_copies_file_to_destination_directory() {
        let temp_dir = tempdir().unwrap();
        let source = temp_dir.path().join("source-binary");
        let destination_dir = temp_dir.path().join("bin");
        fs::write(&source, b"binary contents").unwrap();

        let destination = install_binary(&source, &destination_dir).unwrap();

        assert_eq!(destination, destination_dir.join(env!("CARGO_PKG_NAME")));
        assert_eq!(fs::read(destination).unwrap(), b"binary contents");
    }

    #[test]
    fn test_install_binary_does_not_copy_file_onto_itself() {
        let temp_dir = tempdir().unwrap();
        let source = temp_dir.path().join(env!("CARGO_PKG_NAME"));
        fs::write(&source, b"binary contents").unwrap();

        let destination = install_binary(&source, temp_dir.path()).unwrap();

        assert_eq!(destination, source);
        assert_eq!(fs::read(source).unwrap(), b"binary contents");
    }
}

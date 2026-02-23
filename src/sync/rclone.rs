use anyhow::Result;
use std::path::Path;
use tokio::process::Command;

pub async fn sync_to_remote(local_path: &Path, remote_path: &str, dry_run: bool) -> Result<()> {
    let source = local_path.display().to_string();
    let mut cmd = Command::new("rclone");
    cmd.arg("sync");

    if dry_run {
        cmd.arg("--dry-run");
    }

    cmd.arg(&source).arg(remote_path);

    let output = cmd
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn rclone: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Rclone sync failed: {}", stderr))
    }
}

pub async fn sync_folder_to_remote(
    local_folder: &Path,
    remote_path: &str,
    dry_run: bool,
) -> Result<()> {
    let source = local_folder.display().to_string();
    let mut cmd = Command::new("rclone");
    cmd.arg("sync");

    if dry_run {
        cmd.arg("--dry-run");
    }

    cmd.arg(&source).arg(remote_path);

    let output = cmd
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn rclone: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Rclone sync failed: {}", stderr))
    }
}

pub async fn mkdir_remote(remote_dir: &str) -> Result<()> {
    let mut cmd = Command::new("rclone");
    cmd.arg("mkdir").arg(remote_dir);

    let output = cmd
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn rclone: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("Rclone mkdir failed: {}", stderr))
    }
}

pub async fn test_remote_connection(remote_path: &str) -> Result<bool> {
    // Test by listing root directory silently
    let mut cmd = Command::new("rclone");
    cmd.arg("lsd").arg("--quiet"); // Suppress output

    let remote_arg = format!("{}:", remote_path.trim_end_matches(':'));

    let output = cmd
        .arg(&remote_arg)
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn rclone: {}", e))?;

    // Success means connection and list succeeded
    Ok(output.status.success())
}

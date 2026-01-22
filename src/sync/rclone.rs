use anyhow::Result;
use serde_json::{json, Value};
use std::path::Path;

pub fn sync_to_remote(local_path: &Path, remote_path: &str, dry_run: bool) -> Result<Value> {
    let params = json!({
        "srcFs": format!("local:{}", local_path.display()),
        "dstFs": remote_path,
        "_async": false,
        "dryRun": dry_run
    });

    let rpc_result = librclone::rpc("sync/copy", serde_json::to_string(&params)?);
    match rpc_result {
        Ok(json_str) => Ok(serde_json::from_str(&json_str)?),
        Err(e) => Err(anyhow::anyhow!("Rclone error: {}", e)),
    }
}

pub fn list_remote(remote_path: &str) -> Result<Value> {
    let params = json!({
        "fs": remote_path,
        "remote": ""
    });

    let rpc_result = librclone::rpc("list", serde_json::to_string(&params)?);
    match rpc_result {
        Ok(json_str) => Ok(serde_json::from_str(&json_str)?),
        Err(e) => Err(anyhow::anyhow!("Rclone error: {}", e)),
    }
}

pub fn test_remote_connection(remote_path: &str) -> Result<bool> {
    match list_remote(remote_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

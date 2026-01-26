use anyhow::Result;
use serde_json::{json, Map, Value};
use std::path::Path;

pub fn sync_to_remote(local_path: &Path, remote_path: &str, dry_run: bool) -> Result<Value> {
    let mut params = Map::new();
    params.insert(
        "srcFs".to_string(),
        Value::String(format!("local:{}", local_path.display())),
    );
    params.insert("dstFs".to_string(), Value::String(remote_path.to_string()));
    params.insert("_async".to_string(), Value::Bool(false));
    params.insert("dryRun".to_string(), Value::Bool(dry_run));

    let rpc_result = librclone::rpc("sync/sync", serde_json::to_string(&Value::Object(params))?);
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

pub fn mkdir_remote(remote_dir: &str) -> Result<Value> {
    let params = json!({
        "fs": remote_dir,
        "remote": ""
    });

    let rpc_result = librclone::rpc("mkdir", serde_json::to_string(&params)?);
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

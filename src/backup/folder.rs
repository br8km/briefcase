use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub async fn copy_sensitive_folder(source_dir: &Path, temp_dir: &Path) -> Result<()> {
    let temp_sensitive = temp_dir.join("sensitive");
    fs::create_dir_all(&temp_sensitive).await?;

    copy_dir_recursive(source_dir, &temp_sensitive).await?;

    Ok(())
}

async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    let mut stack = vec![(src.to_path_buf(), dst.to_path_buf())];

    while let Some((src_path, dst_path)) = stack.pop() {
        fs::create_dir_all(&dst_path).await?;

        let mut entries = fs::read_dir(&src_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_file = dst_path.join(file_name);

            if entry_path.is_dir() {
                stack.push((entry_path, dst_file));
            } else {
                fs::copy(&entry_path, &dst_file).await?;
            }
        }
    }

    Ok(())
}

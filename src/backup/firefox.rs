use anyhow::{anyhow, Result};
use std::path::Path;
use tokio::fs;

pub async fn export_firefox_data(profile_dir: &Path, temp_dir: &Path) -> Result<()> {
    if !profile_dir.exists() {
        return Err(anyhow!(
            "Firefox profile directory does not exist: {:?}",
            profile_dir
        ));
    }

    // Create destination if needed
    fs::create_dir_all(temp_dir).await?;

    // Copy essential Firefox files
    let files_to_copy = [
        "places.sqlite",        // Bookmarks and history
        "logins.json",          // Passwords
        "key4.db",              // Key database for passwords
        "cookies.sqlite",       // Cookies
        "favicons.sqlite",      // Favicons
        "sessionstore.jsonlz4", // Session data
        "prefs.js",             // Preferences
    ];

    for file in &files_to_copy {
        let src = profile_dir.join(file);
        if src.exists() {
            let dest = temp_dir.join(file);
            fs::copy(&src, &dest).await?;
        }
    }

    Ok(())
}

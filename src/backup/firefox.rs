use anyhow::{anyhow, Result};
use log::info;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
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

    export_bookmarks(profile_dir, temp_dir).await?;

    // Copy essential Firefox files
    let files_to_copy = ["logins.json", "key4.db", "prefs.js"];

    for file in &files_to_copy {
        let src = profile_dir.join(file);
        if src.exists() {
            let dest = temp_dir.join(file);
            let source_size = fs::metadata(&src).await?.len();
            info!("Copying Firefox file {} ({} bytes)", file, source_size);
            fs::copy(&src, &dest).await?;
            let dest_size = fs::metadata(&dest).await?.len();
            info!("Copied Firefox file {} ({} bytes)", file, dest_size);
        } else {
            info!("Firefox file {} not found, skipping", file);
        }
    }

    Ok(())
}

async fn export_bookmarks(profile_dir: &Path, temp_dir: &Path) -> Result<()> {
    let places_src = profile_dir.join("places.sqlite");
    if !places_src.exists() {
        info!("Firefox file places.sqlite not found, skipping bookmark export");
        return Ok(());
    }

    let places_copy = temp_dir.join("places.sqlite.export");
    let places_size = fs::metadata(&places_src).await?.len();
    info!(
        "Copying Firefox file places.sqlite for bookmark export ({} bytes)",
        places_size
    );
    fs::copy(&places_src, &places_copy).await?;

    let bookmarks_path = temp_dir.join("bookmarks.html");
    let places_copy_for_export = places_copy.clone();
    let bookmarks_path_for_export = bookmarks_path.clone();
    tokio::task::spawn_blocking(move || {
        export_bookmarks_html(&places_copy_for_export, &bookmarks_path_for_export)
    })
    .await??;

    fs::remove_file(&places_copy).await?;
    let bookmarks_size = fs::metadata(&bookmarks_path).await?.len();
    info!(
        "Exported Firefox bookmarks to bookmarks.html ({} bytes)",
        bookmarks_size
    );

    Ok(())
}

#[derive(Debug)]
struct BookmarkEntry {
    id: i64,
    title: Option<String>,
    item_type: i64,
    url: Option<String>,
}

fn export_bookmarks_html(database_path: &Path, output_path: &Path) -> Result<()> {
    let connection = Connection::open(database_path)?;
    let mut statement = connection.prepare(
        "SELECT b.id, b.parent, b.title, b.type, p.url
         FROM moz_bookmarks b
         LEFT JOIN moz_places p ON b.fk = p.id
         ORDER BY b.parent, b.position",
    )?;

    let mut by_parent: HashMap<i64, Vec<BookmarkEntry>> = HashMap::new();
    let rows = statement.query_map([], |row| {
        let parent: i64 = row.get(1)?;
        Ok((
            parent,
            BookmarkEntry {
                id: row.get(0)?,
                title: row.get(2)?,
                item_type: row.get(3)?,
                url: row.get(4)?,
            },
        ))
    })?;

    for row in rows {
        let (parent, entry) = row?;
        by_parent.entry(parent).or_default().push(entry);
    }

    let mut output = File::create(output_path)?;
    writeln!(output, "<!DOCTYPE NETSCAPE-Bookmark-file-1>")?;
    writeln!(
        output,
        "<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">"
    )?;
    writeln!(output, "<TITLE>Bookmarks</TITLE>")?;
    writeln!(output, "<H1>Bookmarks</H1>")?;
    writeln!(output, "<DL><p>")?;
    write_bookmark_children(&mut output, &by_parent, 1, 1)?;
    writeln!(output, "</DL><p>")?;

    Ok(())
}

fn write_bookmark_children(
    output: &mut File,
    by_parent: &HashMap<i64, Vec<BookmarkEntry>>,
    parent: i64,
    depth: usize,
) -> Result<()> {
    let Some(entries) = by_parent.get(&parent) else {
        return Ok(());
    };

    let indent = "    ".repeat(depth);
    for entry in entries {
        match entry.item_type {
            1 => {
                let Some(url) = &entry.url else {
                    continue;
                };
                let title = entry.title.as_deref().unwrap_or(url);
                writeln!(
                    output,
                    "{}<DT><A HREF=\"{}\">{}</A>",
                    indent,
                    escape_html(url),
                    escape_html(title)
                )?;
            }
            2 => {
                if parent == 1 && entry.title.as_deref() == Some("tags") {
                    continue;
                }

                let title = entry.title.as_deref().unwrap_or("Bookmarks");
                writeln!(output, "{}<DT><H3>{}</H3>", indent, escape_html(title))?;
                writeln!(output, "{}<DL><p>", indent)?;
                write_bookmark_children(output, by_parent, entry.id, depth + 1)?;
                writeln!(output, "{}</DL><p>", indent)?;
            }
            3 => writeln!(output, "{}<HR>", indent)?,
            _ => {}
        }
    }

    Ok(())
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

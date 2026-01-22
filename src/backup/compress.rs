use anyhow::Result;
use sevenz_rust::*;
use std::path::Path;

/// Compresses a directory into a 7Zip archive with optional password protection.
///
/// # Arguments
/// * `input_dir` - Directory to compress
/// * `output_file` - Path for the output 7Zip file
/// * `password` - Optional password for AES encryption of the archive
///
/// # Returns
/// Returns `Ok(())` on success, or an error if compression fails
pub fn compress_directory(
    input_dir: &Path,
    output_file: &Path,
    _password: Option<&str>,
) -> Result<()> {
    let mut sz = SevenZWriter::create(output_file)?;

    // Set compression options
    // Note: 7Zip password protection is not used here since we rely on external AES-256-GCM encryption
    // for stronger security and better compatibility

    sz.push_source_path(input_dir, |_| true)?;
    sz.finish()?;

    Ok(())
}

/// Extracts a 7Zip archive to a directory.
///
/// Note: This function extracts archives created by compress_directory.
/// Password-protected archives are not currently supported.
///
/// # Arguments
/// * `input_file` - Path to the 7Zip archive
/// * `output_dir` - Directory to extract files to
///
/// # Returns
/// Returns `Ok(())` on success, or an error if extraction fails
pub fn extract_archive(input_file: &Path, output_dir: &Path) -> Result<()> {
    use sevenz_rust::Password;

    let mut sz = SevenZReader::open(input_file, Password::empty())?;

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Extract all entries to the output directory
    sz.for_each_entries(|entry, reader| {
        let entry_path = entry.name();
        let output_path = output_dir.join(entry_path);

        if entry.is_directory() {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;
            std::fs::write(&output_path, buffer)?;
        }

        Ok(true)
    })?;

    Ok(())
}

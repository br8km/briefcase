use anyhow::Result;
use sevenz_rust::*;
use std::path::Path;

pub fn compress_directory(
    input_dir: &Path,
    output_file: &Path,
    _password: Option<&str>,
) -> Result<()> {
    let mut sz = SevenZWriter::create(output_file)?;

    // TODO: Set encryption and compression options
    // if let Some(pwd) = password {
    //     sz.set_content_methods(vec![
    //         sevenz_rust::AesEncoderOptions::new(pwd.to_string()).into(),
    //         sevenz_rust::LZMA2Options::default().into(),
    //     ]);
    // } else {
    //     sz.set_content_methods(vec![
    //         sevenz_rust::LZMA2Options::default().into(),
    //     ]);
    // }

    sz.push_source_path(input_dir, |_| true)?;
    sz.finish()?;

    Ok(())
}

// pub fn extract_archive(input_file: &Path, output_dir: &Path, password: Option<&str>) -> Result<()> {
//     let mut sz = SevenZReader::open(input_file)?;
//
//     if let Some(pwd) = password {
//         sz.set_password(pwd.to_string());
//     }
//
//     sz.extract_to(output_dir)?;
//
//     Ok(())
// }

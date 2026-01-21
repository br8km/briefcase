use crate::crypto::encrypt;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct CryptoArgs {
    /// Action: validate, decrypt
    action: String,
    /// Password
    #[arg(short, long)]
    password: String,
    /// Input file for decrypt
    #[arg(short, long)]
    input: Option<String>,
    /// Output directory for decrypt
    #[arg(short, long)]
    output: Option<String>,
}

pub async fn run(args: CryptoArgs) -> Result<()> {
    match args.action.as_str() {
        "validate" => {
            println!("Password validation: Not implemented");
            // TODO: Validate password against stored key
        }
        "decrypt" => {
            let input = args
                .input
                .ok_or_else(|| anyhow::anyhow!("Input file required"))?;
            let output = args
                .output
                .ok_or_else(|| anyhow::anyhow!("Output directory required"))?;

            encrypt::decrypt_file(
                &std::path::Path::new(&input),
                &std::path::Path::new(&output),
                &args.password,
            )?;

            println!("Decryption completed");
        }
        _ => {
            println!("Invalid action. Use validate or decrypt");
        }
    }

    Ok(())
}

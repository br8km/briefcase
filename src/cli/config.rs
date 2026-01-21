use crate::config;
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ConfigArgs {
    /// Action: init, edit, validate, show
    pub action: String,

    /// Password for key generation (required for init)
    #[arg(long)]
    pub password: Option<String>,

    /// Password hint
    #[arg(long)]
    pub password_hint: Option<String>,

    /// Custom config file path
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

pub async fn run(args: ConfigArgs) -> Result<()> {
    let config_path = args
        .file
        .unwrap_or_else(|| config::get_config_path().unwrap());

    match args.action.as_str() {
        "init" => {
            let password = args
                .password
                .ok_or_else(|| anyhow::anyhow!("Password required for init"))?;
            let hint = args
                .password_hint
                .ok_or_else(|| anyhow::anyhow!("Password hint required for init"))?;
            // Generate key from password
            // For now, just set
            let mut config = crate::models::config::Config::default();
            config.general.password_key = password; // In real, hash
            config.general.password_hint = hint;
            config::save_config(&config, &config_path)?;
            println!("Config initialized at {:?}", config_path);
        }
        "validate" => {
            let config = config::load_config(&config_path)?;
            config::validate_config(&config)?;
            println!("Config is valid");
        }
        "show" => {
            let config = config::load_config(&config_path)?;
            println!("{}", toml::to_string_pretty(&config)?);
        }
        "edit" => {
            // Open editor
            println!("Edit config at {:?}", config_path);
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid action: {}", args.action));
        }
    }
    Ok(())
}

use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging(log_dir: &PathBuf) -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::daily(log_dir, "briefcase.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "briefcase=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    Ok(())
}

pub fn get_log_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("briefcase")
        .join("log")
}

use std::path::PathBuf;
use yt_dlp::Youtube;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Create the "data" directory if it doesn't exist
    std::fs::create_dir_all("data")?;

    let executables_dir = PathBuf::from("data/libs");
    let output_dir = PathBuf::from("data/output");

    let fetcher = Youtube::with_new_binaries(executables_dir, output_dir).await?;
    Ok(())
}

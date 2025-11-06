use server::config::config_loader::load;
use tracing::{Level, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let dotenvy_env = match load() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Failed to load .env file: {}", e);
            std::process::exit(1);
        }
    };

    info!("YESSSSS");
    info!("Loaded configuration: {:?}", dotenvy_env);           

}
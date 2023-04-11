use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[actix::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("Hello, world!");
}

use std::time::Duration;

use actix::prelude::*;
use anyhow::Context as AnyhowContext;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    signal, time,
};

use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use gossip_glomers::{Event, GlommerMessage, GlommerPayload, MyActor};

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
    // start tracering subscriber
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let stdin = io::stdin();
    // let inputs =
    //     serde_json::Deserializer::from_reader(stdin).into_iter::<GlommerMessage<GlommerPayload>>();
    let stdin_reader = BufReader::new(stdin);
    let mut lines = stdin_reader.lines();

    // start new actor
    let addr = MyActor::start_default();

    let mut interval = time::interval(Duration::from_millis(1000));

    // for input in inputs {
    //     let glommer_message =
    //         input.context("Maelstrom input from STDIN could not be deserialized")?;
    //     MyActor::send_glommer_message(&addr, glommer_message).await?;
    // }
    loop {
        tokio::select! {
            line = lines.next_line() => {
                let line = line.context("Could not read STDIN")?;

                match line {
                    Some(line) => {
                        let glommer_message: GlommerMessage<GlommerPayload> =
                            serde_json::from_str(&line)
                                .context("Maelstrom input from STDIN could not be deserialized")?;

                        MyActor::send_glommer_message(&addr, glommer_message).await?;
                    }
                    None => {
                        info!("STDIN closed shutting down");
                        break;
                    }
                }

            }
            _ = interval.tick() => {
                addr.send(Event::Wake).await.context("Failed to send wake event")??;
            }
            _ = shutdown_signal() => {
                tracing::info!("Shutdown signal received, starting graceful shutdown");
                break;
            }
        }
    }

    Ok(())
}

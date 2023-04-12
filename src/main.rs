use std::io;

use actix::prelude::*;
use anyhow::Context as AnyhowContext;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use gossip_glomers::{GlommerMessage, GlommerPayload, MyActor};

#[actix::main]
async fn main() -> anyhow::Result<()> {
    // start tracering subscriber
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(io::stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let stdin = std::io::stdin().lock();
    let inputs =
        serde_json::Deserializer::from_reader(stdin).into_iter::<GlommerMessage<GlommerPayload>>();

    // start new actor
    let addr = MyActor::new().start();

    for input in inputs {
        let glommer_message =
            input.context("Maelstrom input from STDIN could not be deserialized")?;
        MyActor::send_glommer_message(&addr, glommer_message).await?;
    }

    dbg!(addr);

    Ok(())
}

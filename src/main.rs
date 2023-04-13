use std::time::Duration;

use actix::{
    clock::{interval_at, Instant},
    prelude::*,
};
use anyhow::Context as AnyhowContext;
use rand::Rng;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use gossip_glomers::{EventGossip, GlommerMessage, GlommerPayload, MyActor};

#[actix::main]
async fn main() -> anyhow::Result<()> {
    // start tracering subscriber
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr).with_ansi(false))
        .with(EnvFilter::from_default_env())
        .init();

    let stdin = io::stdin();
    // let inputs =
    //     serde_json::Deserializer::from_reader(stdin).into_iter::<GlommerMessage<GlommerPayload>>();
    let stdin_reader = BufReader::new(stdin);
    let mut lines = stdin_reader.lines();

    // start new actor
    let addr = MyActor::start_default();

    let mut rng = rand::thread_rng();

    // It is better to spread the gossip messages
    let gossip_interval_duration = 250;
    let gossip_start =
        Instant::now() + Duration::from_millis(rng.gen_range(20..=gossip_interval_duration));
    let mut gossip_interval = interval_at(
        gossip_start,
        Duration::from_millis(gossip_interval_duration),
    );
    gossip_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

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
            _ = gossip_interval.tick() => {
                addr.send(EventGossip).await.context("Failed to send gossip event")??;
            }
            //_ = log_interval.tick() => {
            //    addr.send(Event::Log).await.context("Failed to send log event")??;
            //}
        }
    }

    Ok(())
}

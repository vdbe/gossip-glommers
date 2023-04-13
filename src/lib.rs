use std::{
    collections::{HashMap, HashSet},
    fmt,
    io::StdoutLock,
    time::Duration,
};

use actix::{clock::Interval, prelude::*};
use tokio::time;
use tracing::trace;

use formatter::NewLineFormatter;
pub use message::{
    gossip::{EventGossip, GossipState},
    GlommerMessage, GlommerPayload,
};

mod formatter;
mod message;

pub type ActorResult = anyhow::Result<()>;
pub type Output = serde_json::Serializer<StdoutLock<'static>, NewLineFormatter>;

pub struct MyActor {
    id: usize,

    node_id: String,
    node_ids: Vec<String>,

    topology: HashMap<String, Vec<String>>,
    messages: HashSet<usize>,

    gossip_state: GossipState,

    pub interval: Interval,
    output: Output,
}

impl fmt::Debug for MyActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyActor")
            .field("id", &self.id)
            .field("node_id", &self.node_id)
            .field("node_ids", &self.node_ids)
            .field("topology", &self.topology)
            .field("messages", &self.messages)
            .finish()
    }
}

impl Default for MyActor {
    fn default() -> Self {
        let stdout = std::io::stdout().lock();
        let output = serde_json::Serializer::with_formatter(stdout, NewLineFormatter::new());

        let interval = time::interval(Duration::from_millis(1000));

        Self {
            id: 1,

            node_id: String::new(),
            node_ids: Vec::new(),

            topology: HashMap::new(),
            messages: HashSet::new(),

            gossip_state: GossipState::default(),
            interval,
            output,
        }
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        trace!("MyActor is alive");
    }
}

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub enum Event {
    Log,
}

impl Handler<Event> for MyActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Event, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Event::Log => {}
        }

        Ok(())
    }
}

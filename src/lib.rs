use std::{collections::HashMap, fmt, io::StdoutLock};

use actix::prelude::*;

use formatter::NewLineFormatter;
pub use message::{GlommerMessage, GlommerPayload};
use tracing::info;

mod formatter;
mod message;

pub type ActorResult = anyhow::Result<()>;
pub type Output = serde_json::Serializer<StdoutLock<'static>, NewLineFormatter>;

pub struct MyActor {
    id: usize,

    node_id: String,
    node_ids: Vec<String>,

    topology: HashMap<String, Vec<String>>,
    messages: Vec<usize>,

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

        Self {
            id: 1,

            node_id: String::new(),
            node_ids: Vec::new(),

            topology: HashMap::new(),
            messages: Vec::new(),

            output,
        }
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("MyActor is alive");
    }
}

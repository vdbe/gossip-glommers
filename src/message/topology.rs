use std::collections::HashMap;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::trace;

use super::{ActorMessage, GlommerPayload, MyActor};
use crate::ActorResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    topology: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyOk;

impl Message for ActorMessage<Topology> {
    type Result = ActorResult;
}

impl Message for ActorMessage<TopologyOk> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Topology>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Topology>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Topology received");

        self.topology = msg.payload.topology;

        let payload = GlommerPayload::TopologyOk(TopologyOk);

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<TopologyOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, _msg: ActorMessage<TopologyOk>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("TopologyOk received");

        Ok(())
    }
}

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::GlommerPayload;

use super::{ActorMessage, ActorResult, MyActor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitOk;

impl Message for ActorMessage<InitOk> {
    type Result = ActorResult;
}

impl Message for ActorMessage<Init> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Init>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Init>, _ctx: &mut Self::Context) -> Self::Result {
        info!("Init received");

        self.node_id = msg.payload.node_id;
        self.node_ids = msg.payload.node_ids;

        let payload = GlommerPayload::InitOk(InitOk);
        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<InitOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, _msg: ActorMessage<InitOk>, _ctx: &mut Self::Context) -> Self::Result {
        info!("InitOk received");

        Ok(())
    }
}

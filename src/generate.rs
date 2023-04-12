use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::{ActorMessage, ActorResult, GlommerPayload, MyActor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOk {
    id: String,
}

impl Message for ActorMessage<GenerateOk> {
    type Result = ActorResult;
}

impl Message for ActorMessage<Generate> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Generate>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Generate>, _ctx: &mut Self::Context) -> Self::Result {
        info!("generate received");

        let payload = GlommerPayload::GenerateOk(GenerateOk {
            id: format!("{}-{}", self.node_id, self.id),
        });

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<GenerateOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, _msg: ActorMessage<GenerateOk>, _ctx: &mut Self::Context) -> Self::Result {
        info!("generateOk received");

        Ok(())
    }
}

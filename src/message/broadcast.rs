use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::trace;

use super::{ActorMessage, GlommerPayload, MyActor};
use crate::ActorResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    message: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastOk;

impl Message for ActorMessage<Broadcast> {
    type Result = ActorResult;
}

impl Message for ActorMessage<BroadcastOk> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Broadcast>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Broadcast>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Broadcast received");

        self.messages.insert(msg.payload.message);

        let payload = GlommerPayload::BroadcastOk(BroadcastOk);

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<BroadcastOk>> for MyActor {
    type Result = ActorResult;

    fn handle(
        &mut self,
        _msg: ActorMessage<BroadcastOk>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        trace!("BroadcastOk received");

        Ok(())
    }
}

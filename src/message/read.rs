use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::trace;

use super::{ActorMessage, GlommerPayload, MyActor};
use crate::ActorResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOk {
    messages: Vec<usize>,
}

impl Message for ActorMessage<Read> {
    type Result = ActorResult;
}

impl Message for ActorMessage<ReadOk> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Read>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Read>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Read received");

        let payload = GlommerPayload::ReadOk(ReadOk {
            messages: self.messages.clone(),
        });

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<ReadOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, _msg: ActorMessage<ReadOk>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("ReadOk received");

        Ok(())
    }
}

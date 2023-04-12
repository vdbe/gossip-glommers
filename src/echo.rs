use actix::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::{ActorMessage, ActorResult, GlommerPayload, MyActor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Echo {
    echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoOk {
    echo: String,
}

impl Message for ActorMessage<EchoOk> {
    type Result = ActorResult;
}

impl Message for ActorMessage<Echo> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Echo>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Echo>, _ctx: &mut Self::Context) -> Self::Result {
        info!("Echo received");

        let payload = GlommerPayload::EchoOk(EchoOk {
            echo: msg.payload.echo,
        });

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<EchoOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, _msg: ActorMessage<EchoOk>, _ctx: &mut Self::Context) -> Self::Result {
        info!("EchoOk received");

        Ok(())
    }
}

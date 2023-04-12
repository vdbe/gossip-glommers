use std::{fmt, io::StdoutLock};

use actix::prelude::*;
use anyhow::Context as AnyhowContext;
use serde::{Deserialize, Serialize};

use echo::{Echo, EchoOk};
use formatter::NewLineFormatter;
use init::{Init, InitOk};

mod echo;
mod formatter;
mod init;

pub type ActorResult = anyhow::Result<()>;
pub type Output = serde_json::Serializer<StdoutLock<'static>, NewLineFormatter>;

pub struct MyActor {
    node_id: String,
    node_ids: Vec<String>,
    id: usize,
    output: Output,
}

impl fmt::Debug for MyActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyActor")
            .field("node_id", &self.node_id)
            .field("node_ids", &self.node_ids)
            .finish()
    }
}

impl MyActor {
    pub fn new() -> Self {
        let stdout = std::io::stdout().lock();
        let output = serde_json::Serializer::with_formatter(stdout, NewLineFormatter::new());

        Self {
            id: 1,
            node_id: String::new(),
            node_ids: Vec::new(),
            output,
        }
    }

    fn reply(
        &mut self,
        message: GlommerMessage<()>,
        payload: GlommerPayload,
    ) -> anyhow::Result<()> {
        let reply = GlommerMessage {
            src: message.dest,
            dest: message.src,
            body: GlommerBody {
                id: Some(self.id),
                in_reply_to: message.body.id,
                payload,
            },
        };

        self.id += 1;

        reply
            .serialize(&mut self.output)
            .context("serialize response to init")
    }
}

impl Default for MyActor {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMessage<Payload> {
    pub message: GlommerMessage<()>,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlommerMessage<Payload> {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    // An object: the payload of the message
    pub body: GlommerBody<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlommerBody<Payload> {
    /// A unique integer identifier
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,

    /// For req/response, the msg_id of the request
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GlommerPayload {
    Init(Init),
    InitOk(InitOk),
    Echo(Echo),
    EchoOk(EchoOk),
}

impl MyActor {
    pub async fn send_glommer_message(
        addr: &Addr<MyActor>,
        glommer_message: GlommerMessage<GlommerPayload>,
    ) -> anyhow::Result<()> {
        let message: GlommerMessage<()> = GlommerMessage {
            src: glommer_message.src,
            dest: glommer_message.dest,
            body: GlommerBody {
                id: glommer_message.body.id,
                in_reply_to: glommer_message.body.in_reply_to,
                payload: (),
            },
        };

        let response = match glommer_message.body.payload {
            GlommerPayload::Init(init) => {
                let actor_message = ActorMessage {
                    message,
                    payload: init,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::InitOk(init_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: init_ok,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::Echo(echo) => {
                let actor_message = ActorMessage {
                    message,
                    payload: echo,
                };

                addr.send(actor_message).await
            }
            GlommerPayload::EchoOk(echo_ok) => {
                let actor_message = ActorMessage {
                    message,
                    payload: echo_ok,
                };

                addr.send(actor_message).await
            }
        };

        response
            .context("send to handler")?
            .context("handler failed")
    }
}

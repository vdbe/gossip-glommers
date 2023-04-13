use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use anyhow::Context as AnyhowContext;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

use super::{ActorMessage, GlommerPayload, MyActor};
use crate::ActorResult;

#[derive(Debug, Clone, Default)]
pub struct GossipState {
    pub neighberhood: Vec<String>,

    known: HashMap<String, HashSet<usize>>,
    sent_messages: HashMap<usize, HashSet<usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gossip {
    seen: HashSet<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipOk;

#[derive(Debug, Copy, Clone, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct EventGossip;

impl Message for ActorMessage<Gossip> {
    type Result = ActorResult;
}

impl Message for ActorMessage<GossipOk> {
    type Result = ActorResult;
}

impl Handler<ActorMessage<Gossip>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<Gossip>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Gossip received");

        debug!("Gossip received from {}", &msg.message.src);

        if let Some(known_by_sender) = self.gossip_state.known.get_mut(&msg.message.src) {
            known_by_sender.extend(msg.payload.seen.iter().copied());
            self.messages.extend(msg.payload.seen);
        } else {
            let payload = msg.payload.seen.iter().copied().collect();
            self.gossip_state
                .known
                .insert(msg.message.src.clone(), payload);
            self.messages.extend(msg.payload.seen);
        };

        let payload = GlommerPayload::GossipOk(GossipOk);

        self.reply(msg.message, payload)
    }
}

impl Handler<ActorMessage<GossipOk>> for MyActor {
    type Result = ActorResult;

    fn handle(&mut self, msg: ActorMessage<GossipOk>, _ctx: &mut Self::Context) -> Self::Result {
        trace!("GossipOk received");
        let reply_to = &msg.message.body.in_reply_to.context("empty reply_to")?;

        debug!(
            "GossipOk received from {} in reply to {}",
            &msg.message.src, reply_to
        );

        let messages_confirmed = self
            .gossip_state
            .sent_messages
            .remove(reply_to)
            .context("GossipOk reply to unkown Gossip")?;

        let known_by_sender = self.gossip_state.known.get_mut(&msg.message.src);

        if let Some(known_by_sender) = known_by_sender {
            known_by_sender.extend(messages_confirmed);
        } else {
            self.gossip_state
                .known
                .insert(msg.message.src, messages_confirmed);
        };

        Ok(())
    }
}

impl Handler<EventGossip> for MyActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: EventGossip, _ctx: &mut Self::Context) -> Self::Result {
        trace!("Gossip event recived");

        for neighbor_i in 0..self.gossip_state.neighberhood.len() {
            let neighbor = &self.gossip_state.neighberhood[neighbor_i];

            let unknown_by_neighbor: HashSet<usize> =
                if let Some(known_by_neighbor) = self.gossip_state.known.get(neighbor) {
                    self.messages
                        .difference(known_by_neighbor)
                        .copied()
                        .collect()
                } else {
                    self.messages.iter().copied().collect()
                };

            if unknown_by_neighbor.is_empty() {
                continue;
            }

            let payload = GlommerPayload::Gossip(Gossip {
                seen: unknown_by_neighbor.iter().copied().collect(),
            });

            let message_id = self
                .send(neighbor.clone(), None, payload)
                .context("send gossip to neighbor")?;

            debug!(
                "Gossip send to {} with id {}",
                &self.gossip_state.neighberhood[neighbor_i], message_id
            );

            self.gossip_state
                .sent_messages
                .insert(message_id, unknown_by_neighbor);
        }

        Ok(())
    }
}

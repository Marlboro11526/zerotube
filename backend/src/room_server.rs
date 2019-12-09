use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;
use std::collections::{HashMap, HashSet};

#[derive(Message)]
pub struct Connect {}

#[derive(Message)]
pub struct Disconnect {}

pub struct RoomServer {}

impl Actor for RoomServer {
    type Context = WebsocketContext<Self>;
}

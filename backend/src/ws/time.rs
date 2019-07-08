use actix::prelude::*;
use actix_web::{web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws::{self, Message, ProtocolError, WebsocketContext};
use chrono::Local;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub fn route(request: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", request);

    // broken in Chrome until actix-web PR #835 lands due to front-end specifying protocol
    ws::start(TimeWebSocket::new(), &request, stream)
}

struct TimeWebSocket {
    heartbeat: Instant,
}

impl Actor for TimeWebSocket {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
        self.test(context);
    }
}

impl StreamHandler<Message, ProtocolError> for TimeWebSocket {
    fn handle(&mut self, message: Message, context: &mut Self::Context) {
        println!("WS MESSAGE: {:?}", message);

        match message {
            Message::Binary(bin) => context.binary(bin),
            Message::Close(_) => context.stop(),
            Message::Nop => (),
            Message::Ping(message) => {
                self.heartbeat = Instant::now();
                context.pong(&message);
            }
            Message::Pong(_) => self.heartbeat = Instant::now(),
            Message::Text(text) => context.text(text),
        }
    }
}

impl TimeWebSocket {
    fn new() -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }

    fn heartbeat(&self, context: &mut <Self as Actor>::Context) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                println!("HEARTBEAT FAILED, DISCONNECTING");

                context.stop();

                return;
            }

            context.ping("");
        });
    }

    fn test(&self, context: &mut <Self as Actor>::Context) {
        context.run_interval(Duration::from_secs(1), |actor, context| {
            context.text(Local::now().to_rfc2822().as_str());
        });
    }

    fn response(&self, context: &mut <Self as Actor>::Context, request: String) {
        context.text(format!("YOU SAY {}, I SAY BAR", request));
    }
}

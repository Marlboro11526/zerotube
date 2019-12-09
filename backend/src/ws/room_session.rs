use crate::db::{entities::room::Room as DbRoom, media, rooms};
use crate::messages::error::ErrorResponse;
use crate::messages::media::{AddMediaLocation, AddMediaRequest, RemoveMediaRequest};
use crate::models::media::Media;
use crate::room_server::RoomServer;
use actix::prelude::*;
use actix_web::{
    web::{Data, Path, Payload},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws::{self, ProtocolError, WebsocketContext};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsRoomSession {
    id: Uuid,
    heartbeat: Instant,
    room: DbRoom,
    server: Addr<RoomServer>,
    pool: Pool,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Request {
    AddMedia { request: AddMediaRequest },
    RemoveMedia { request: RemoveMediaRequest },
}

#[derive(Serialize)]
#[serde(tag = "response")]
enum Response {
    Success,
    Error { reason: String },
}

impl WsRoomSession {
    fn handle_request(&self, request: Request) -> Response {
        let result = match request {
            Request::AddMedia { request } => self.add_media_to_room(request),
            Request::RemoveMedia { request } => unimplemented!(),
        };

        match result {
            Ok(result) => result,
            Err(result) => result,
        }
    }

    fn add_media_to_room(&self, request: AddMediaRequest) -> Result<Response, Response> {
        let connection = self.pool.get().unwrap();

        let index = match request.location {
            AddMediaLocation::Next => request.current + 1,
            AddMediaLocation::Last => {
                let latest = media::get_latest_media_index_for_room_with_url(
                    self.room.url.as_str(),
                    &connection,
                )
                .map_err(|_| Response::Error {
                    reason: "Unable to get latest media index".to_string(),
                })?;

                latest + 1
            }
        };

        let media = Media::new(&request.url, index).map_err(|_| Response::Error {
            reason: "Unable to create Media model".to_string(),
        })?;

        media::add_media_to_room_with_url(self.room.url.as_str(), media, &connection).map_err(
            |_| Response::Error {
                reason: "Unable to add media".to_string(),
            },
        )?;

        Ok(Response::Success)
    }

    fn heartbeat(&self, context: &mut <Self as Actor>::Context) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                context.stop();
            } else {
                context.ping("");
            }
        });
    }
}

impl Actor for WsRoomSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
        let connection = &self.pool.get().unwrap();

        let initial_media = media::get_media_for_room_with_url(&self.room.url, &connection);

        if initial_media.is_ok() {
        } else {
            let response = Response::Error {
                reason: "Failed to connect to room".to_string(),
            };

            context.text(serde_json::to_string(&response).expect("Serialisation error"));
            context.stop();
        }
    }
}

impl StreamHandler<ws::Message, ProtocolError> for WsRoomSession {
    fn handle(&mut self, message: ws::Message, context: &mut Self::Context) {
        match message {
            ws::Message::Binary(_) => (),
            ws::Message::Close(_) => context.stop(),
            ws::Message::Nop => (),
            ws::Message::Ping(_) => {
                self.heartbeat = Instant::now();
                context.pong("PONG");
            }
            ws::Message::Pong(_) => self.heartbeat = Instant::now(),
            ws::Message::Text(request) => {
                let response = match serde_json::from_str(&request) {
                    Ok(request) => self.handle_request(request),
                    Err(_) => Response::Error {
                        reason: "Malformed request".to_string(),
                    },
                };

                context.text(serde_json::to_string(&response).expect("Serialisation error"));
            }
        }
    }
}

pub fn route(
    pool: Data<Pool>,
    request: HttpRequest,
    room_url: Path<String>,
    server: Data<Addr<RoomServer>>,
    stream: Payload,
) -> Result<HttpResponse, Error> {
    let room = rooms::get_room_with_url_internal(&room_url, &pool.get().unwrap())?
        .ok_or(ErrorResponse::NotFound)?;

    ws::start(
        WsRoomSession {
            id: Uuid::new_v4(),
            heartbeat: Instant::now(),
            room,
            server: server.get_ref().clone(),
            pool: pool.get_ref().clone(),
        },
        &request,
        stream,
    )
}

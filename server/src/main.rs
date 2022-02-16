use glam::Vec2;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize, Clone)]
pub struct State {
    pub pos: Vec2,
    pub r: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RemoteState {
    pub id: usize,
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    Welcome(usize),
    GoodBye(usize),
    Update(Vec<RemoteState>),
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    State(State),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let status = warp::path!("status").map(move || warp::reply::html("hello"));

    warp::serve(status).run(([0, 0, 0, 0], 3030)).await;
}

async fn send_msg(msg: ServerMessage) {
    let buffer = serde_json::to_vec(&msg).unwrap();
}

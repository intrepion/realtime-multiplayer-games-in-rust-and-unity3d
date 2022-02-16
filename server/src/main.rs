use glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use warp::ws::{Message, WebSocket};
use warp::Filter;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    State(State),
}

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    Welcome(usize),
    GoodBye(usize),
    Update(Vec<RemoteState>),
}

type OutBoundChannel = mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;
type Users = Arc<RwLock<HashMap<usize, OutBoundChannel>>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct RemoteState {
    pub id: usize,
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct State {
    pub pos: Vec2,
    pub r: f32,
}

async fn broadcast(msg: ServerMessage, users: &Users) {
    let users = users.read().await;
    for (_, tx) in users.iter() {
        send_msg(tx, &msg).await;
    }
}

fn create_send_channel(
    ws_sender: futures_util::stream::SplitSink<WebSocket, Message>,
) -> OutBoundChannel {
    use futures_util::FutureExt;
    use futures_util::StreamExt;
    use tokio_stream::wrappers::UnboundedReceiverStream;

    let (sender, receiver) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(receiver);

    tokio::task::spawn(rx.forward(ws_sender).map(|result| {
        if let Err(e) = result {
            log::error!("websocket send error: {}", e);
        }
    }));

    sender
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let users = Users::default();

    let users = warp::any().map(move || users.clone());

    let game = warp::path("game")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));

    let status = warp::path!("status").map(move || warp::reply::html("hello"));

    let routes = status.or(game);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn send_msg(tx: &OutBoundChannel, msg: &ServerMessage) {
    let buffer = serde_json::to_vec(msg).unwrap();

    let msg = Message::binary(buffer);

    tx.send(Ok(msg)).unwrap();
}

async fn send_welcome(out: &OutBoundChannel) -> usize {
    let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    let states = ServerMessage::Welcome(id);

    send_msg(out, &states).await;

    id
}

async fn user_connected(ws: WebSocket, users: Users) {
    use futures_util::StreamExt;

    let (ws_sender, mut ws_receiver) = ws.split();

    let send_channel = create_send_channel(ws_sender);

    let my_id = send_welcome(&send_channel).await;

    log::debug!("new user connected: {}", my_id);

    users.write().await.insert(my_id, send_channel);

    while let Some(result) = ws_receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::warn!("websocket receive error: '{}'", e);
                break;
            }
        };

        log::debug!("user sent message: {:?}", msg);
    }

    log::debug!("user disconnected: {}", my_id);

    users.write().await.remove(&my_id);

    broadcast(ServerMessage::GoodBye(my_id), &users).await;
}

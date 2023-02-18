use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    Router,
    routing::{put, get},
    response::{IntoResponse, Result},
    http::StatusCode,
};
use std::sync::Arc;
use axum_extra::extract::cookie::{CookieJar, Cookie};
use dashmap::DashMap;
use tracing::error;

pub use incircle_protocol::{Request, Response, prost::Message as _};
pub use ring_channel::{ring_channel as channel, RingSender as Sender, RingReceiver as Receiver};

use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};
use rand::{thread_rng, Rng};

fn gen_session_id() -> String {
    let random_bytes = thread_rng().gen::<[u8; 32]>();
    BASE64_STANDARD_NO_PAD.encode(random_bytes)
}

struct AppState {
    sender: Sender<(String, Request)>, 
    receiver: Receiver<(String, Response)>,
    conns: DashMap<String, SplitSink<WebSocket, Message>>, 
}

async fn join(
    jar: CookieJar,
) -> impl IntoResponse {
    let sid = jar.get("sid").map(|c| c.value().to_owned()).unwrap_or_else(gen_session_id);
    (jar.add(Cookie::new("sid", sid)), StatusCode::OK)
}

async fn ws(
    jar: CookieJar,
    ws: WebSocketUpgrade, 
    State(state): State<Arc<AppState>>,
) -> Result<&'static str>
{
    let sid = jar.get("sid").map(|c| c.value().to_owned()).unwrap_or_else(gen_session_id);
    let mut sender = state.sender.clone();
    ws.on_upgrade(|socket| async move {
        let (writer, mut reader) = socket.split();
        state.conns.insert(sid.clone(), writer);
        if let Some(Ok(Message::Binary(data))) = reader.next().await {
            match Request::decode(data.as_ref()) {
                Ok(req) => match sender.send((sid, req)) {
                    Ok(None) => (),
                    Ok(Some(_)) => error!("Event receiver buffer is full. discard the old event."),
                    Err(err) => error!("{:?}", err),
                },
                Err(err) => error!("{}", err),
            };
        }
    });
    Ok("Ok")
}

async fn health() -> &'static str {
    "Ok"
}

pub async fn start(
    addr: std::net::SocketAddr,
    sender: Sender<(String, Request)>,
    mut receiver: Receiver<(String, Response)>,
) -> Result<(), axum::Error> {
    let app_state = Arc::new(AppState {
        sender, 
        receiver: receiver.clone(), 
        conns: DashMap::new(),
    });
    {
        let app_state = app_state.clone();
        tokio::spawn(async move {
            while let Some((sid, res)) = receiver.next().await {
                let mut buf = Vec::with_capacity(res.encoded_len());
                if let Some(mut socket) = app_state.conns.get_mut(&sid) {
                    res.encode(&mut buf).unwrap();
                    if let Err(err) = socket.send(Message::Binary(buf)).await {
                        error!("{:?}", err);
                    }
                }
            }
        });
    }
    let app = Router::new()
        .route("/ws", get(ws))
        .route("/session", put(join))
        .route("/health", get(health))
        .with_state(app_state);
    Ok(axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .map_err(|t| axum::Error::new(t))?)
}

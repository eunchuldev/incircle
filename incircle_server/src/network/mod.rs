mod server;

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use derive_more::{Deref, DerefMut};
//use tokio::sync::mpsc::{channel, Receiver, Sender, error::TrySendError};
use ring_channel::{ring_channel as channel, RingSender as Sender, RingReceiver as Receiver};
//use server::{Sender, Receiver, channel};
use std::num::NonZeroUsize;
use tokio::runtime::Runtime;
use incircle_protocol::{Request, Response};


#[derive(Resource, Deref, DerefMut)]
pub struct MessageReceiver<T>(Receiver<T>);

#[derive(Resource, Deref, DerefMut)]
pub struct MessageSender<T>(Sender<T>);

#[derive(Resource, Deref, DerefMut)]
pub struct TokioRuntime(Runtime);

pub struct NetworkPlugin {
    addr: std::net::SocketAddr,
    max_connections: usize,
    max_buffer_size: usize,
}

impl Default for NetworkPlugin {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:8000".parse().unwrap(),
            max_connections: 16384,
            max_buffer_size: 65536,
        }
    }
}

impl Plugin for NetworkPlugin
{
    fn build(&self, app: &mut App) {
        let max_buffer_size = self.max_buffer_size;
        let addr = self.addr;
        app
            .add_startup_system(move |mut commands: Commands| {
                let (req_tx, req_rx) = channel::<(String, Request)>(NonZeroUsize::new(max_buffer_size).unwrap());
                let (res_tx, res_rx) = channel::<(String, Response)>(NonZeroUsize::new(max_buffer_size).unwrap());

                let rt = tokio::runtime::Runtime::new().unwrap();

                rt.block_on(async move {
                    server::start(addr, req_tx, res_rx).await
                }).unwrap();

                commands.insert_resource(MessageSender(res_tx));
                commands.insert_resource(MessageReceiver(req_rx));
            });
    }
}

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use tokio::sync::mpsc::{Receiver, Sender, error::TrySendError};

#[derive(Component)]
pub struct Player {
    pub code: String,
    pub registored_at: Instant,
}

#[derive(Component)]
pub struct Logined;

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerCodes(HashMap<String, Entity>);

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerEventReceiver<T>(Receiver<T>);


#[derive(Resource)]
pub struct PlayerEventSender<T>(Sender<(String, T)>);

impl<T> PlayerEventSender<T> {
    pub fn try_send(&self, player: &Player, msg: T) -> Result<(), TrySendError<(String, T)>> {
        Ok(self.0.try_send((player.code.clone(), msg))?)
    }
}



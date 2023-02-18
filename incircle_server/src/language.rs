use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_core::FrameCount;
use crate::physics::{SpatialIndex, Position, Size};
use crate::character::{Stats};

#[derive(Clone)]
pub enum Sentiment {
    Positive,
    Natural,
    Negative,
}
impl Default for Sentiment { fn default() -> Self { Sentiment::Natural } }

#[derive(Clone)]
pub enum Emotion {
    Sad,
    Angry,
    Happy,
    Fun,
    Natural,
}
impl Default for Emotion { fn default() -> Self { Emotion::Natural } }

#[derive(Component, Clone, Default)]
pub struct Word {
    pub text: String,
    pub sentiment: Sentiment,
    pub emotion: Emotion,
}

#[derive(Clone, Default)]
pub struct VocabItem {
    pub word: Word,
    pub level: i32,
    pub xp: f32,
}

#[derive(Component, Clone, Default)]
pub struct Vocab {
    pub items: Vec<VocabItem>,
}

#[derive(Component, Clone, Default)]
#[component(storage = "SparseSet")]
pub struct Speaking {
    pub word: Word,
    pub power: f32,
    pub expire_frame: u32,
}

pub fn stop_speaking(
    mut commands: Commands, 
    query: Query<(Entity, &Speaking)>,
    frame: Res<FrameCount>,
) {
    for (entity, speaking) in query.iter() {
        if speaking.expire_frame >= frame.0 {
            commands.entity(entity).remove::<Speaking>();
        }
    }
}

pub fn listen(
    mut commands: Commands, 
    speakers: Query<(&Position, &Speaking)>, 
    mut listeners: Query<&mut Stats>,
    spatial_index: Res<SpatialIndex>,
) {
    for (pos, speak) in speakers.iter() {
        for entity in spatial_index.search(*pos, Size { w: speak.power, h: speak.power }) {
            if let Ok(mut stats) = listeners.get_mut(entity) {
                match speak.word.sentiment {
                    Sentiment::Positive => stats.mental_health += speak.power,
                    Sentiment::Negative => stats.mental_health -= speak.power,
                    _ => ()
                }
            }
            //commands.entity(entity).get::<Speaking>();
        }
    }
}


#[derive(Default, Clone)]
pub struct LanguagePlugin; 

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(listen)
            .add_system(stop_speaking);
    }
}


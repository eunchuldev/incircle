use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bitmask_enum::bitmask;
use crate::physics::{Position, RigidBodyBundle};

#[derive(Component, Default)]
pub struct Attributes {
    pub physics: f32,
    pub ego: f32,

    pub physics_exp: f32,
    pub ego_exp: f32,
}

#[derive(Component, Default)]
pub struct Xps {
    pub physics_exp: f32,
    pub ego_exp: f32,
}

#[derive(Component, Default)]
pub struct Stats {
    pub speed: f32,
    pub max_mental_health: f32,
    pub mental_health: f32,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Moving {
    direction: f32,
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    attrs: Attributes,
    xps: Xps,
    stats: Stats,
    body: RigidBodyBundle,
}

/*
#[derive(Component)]
#[bitmask(u64)]
pub enum Status {
    NoticeCharacter = 1,
}

impl Status {
    pub fn noticed_character_pos(&self) -> Position {
        const MAX: f32 = 0b11 as f32;
        Position {
            x: ((self.bits >> 1 & 0b11) as f32) * 2.0 / MAX - 1.0,
            y: ((self.bits >> 3 & 0b11) as f32) * 2.0 / MAX - 1.0,
        }
    }
    pub fn noticed_character_threat(&self) -> f32 {
        (self.bits >> 5 & 0b11) as f32 / 0b11 as f32
    }
    pub fn notice_character(&mut self, pos: Position, threat: f32) {
        self.bits = self.bits & !0b11_11110 | Self::NoticeCharacter.bits();
        self.bits |= (((pos.x + 1.0) * 0.5 * 0b11 as f32).round() as u64) << 1;
        self.bits |= (((pos.y + 1.0) * 0.5 * 0b11 as f32).round() as u64) << 3;
        self.bits |= ((threat * 0b11 as f32).round() as u64) << 5;
    }
}
*/

pub fn calculate_stats(mut query: Query<(&Attributes, &mut Stats), Changed<Attributes>>) {
    for (attrs, mut stats) in query.iter_mut() {
        stats.speed = attrs.physics.powf(0.3) * 30.0;
        stats.max_mental_health = attrs.ego * 10.;
    }
}

pub fn move_character(mut query: Query<(&mut Position, &Stats, &Moving)>) {
    for (mut pos, stats, moving) in query.iter_mut() {
        pos.x += stats.speed * moving.direction.cos();
        pos.y += stats.speed * moving.direction.sin();
    }
}


#[derive(Default, Clone)]
pub struct CharacterPlugin; 

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(calculate_stats)
            .add_system(move_character);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_status() {
        let mut status = Status::none();
        assert_eq!(status.contains(Status::NoticeCharacter), false);
        status.notice_character(Position { x: 0.25, y: -0.75 }, 0.9);
        assert_eq!(status.contains(Status::NoticeCharacter), true);
        assert!((status.noticed_character_pos().x - 1.0/3.0).abs() < 0.1);
        assert_eq!(status.noticed_character_pos().y, -1.0);
        assert_eq!(status.noticed_character_threat(), 1.0);
    }
    */
}


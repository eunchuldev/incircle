mod spatial_index;

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_app::{PluginGroup, PluginGroupBuilder};

pub use spatial_index::{SpatialIndex, SpatialTracking, SpatialIndexPlugin};

#[derive(Component, Clone, Copy, Default, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/*
#[derive(Component, Clone, Copy, Default, Debug, PartialEq)]
pub struct Velocity {
    pub angle: f32,
    pub magnitude: f32,
}
*/

#[derive(Component, Clone, Copy, Default, Debug, PartialEq)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Bundle, Clone, Default)]
pub struct RigidBodyBundle {
    size: Size,
    position: Position,
    spatial_tracking: SpatialTracking,
}

/*
pub fn translate(
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut pos, Velocity { magnitude, angle }) in query.iter_mut() {
        pos.x += magnitude * angle.cos();
        pos.y += magnitude * angle.sin();
    }
}

#[derive(Default, Clone)]
pub struct TransformPlugin; 

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(translate);
    }
}
*/


#[derive(Default, Clone)]
pub struct PhysicsPlugins; 

impl PluginGroup for PhysicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            //.add(TransformPlugin)
            .add(SpatialIndexPlugin::default())
    }
}

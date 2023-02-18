use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use rtree_rs::{RTree, Rect};
use super::{Position, Size};

#[derive(Component, Default, Clone)]
pub struct SpatialTracking {
    pub last_pos: Position,
    pub last_size: Size,
}

#[derive(Resource)]
pub struct SpatialIndex {
    rtree: RTree::<2, f32, Entity>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            rtree: RTree::new(),
        }
    }
    pub fn search<'a> (&'a self, pos: Position, size: Size) -> impl Iterator<Item=Entity> + 'a
    {
        self.rtree.search(Rect::new([pos.x-size.w*0.5, pos.y-size.h*0.5], [pos.x+size.w*0.5, pos.y+size.h*0.5])).map(|t| *t.data)
    }
    pub fn insert(&mut self, pos: Position, size: Size, v: Entity) {
        self.rtree.insert(Rect::new([pos.x-size.w*0.5, pos.y-size.h*0.5], [pos.x+size.w*0.5, pos.y+size.h*0.5]), v);
    }
    pub fn remove(&mut self, pos: Position, size: Size, v: Entity) -> Option<Entity> {
        self.rtree.remove(Rect::new([pos.x-size.w*0.5, pos.y-size.h*0.5], [pos.x+size.w*0.5, pos.y+size.h*0.5]), &v).map(|t| t.1)
    }
}

#[derive(Default, Clone, Resource)]
pub struct SpatialIndexPlugin {
    skip_delta: f32,
}

impl SpatialIndexPlugin {
    pub fn new() -> Self {
        Self::default() 
    }
    pub fn with_skip_delta(mut self, skip_delta: f32) -> Self {
        self.skip_delta = skip_delta;
        self
    }
}

impl Plugin for SpatialIndexPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.clone())
            .insert_resource(SpatialIndex::new())
            .add_system(update_spatial_index);
            //.add_system_to_stage(CoreStage::PreUpdate, attach_tracker)
            //.add_system_to_stage(CoreStage::PostUpdate, update_gis);
    }
}

pub fn update_spatial_index(
    plugin: Res<SpatialIndexPlugin>,
    mut gis: ResMut<SpatialIndex>,
    mut query: Query<(Entity, &mut SpatialTracking, &Position, &Size, ChangeTrackers<Position>), Or<(Changed<Position>, Changed<Size>)>>
) {
    for (entity, mut track, pos, size, pos_tracker) in query.iter_mut() {
        if pos_tracker.is_added() {
            gis.insert(*pos, *size, entity);
            (*track).last_pos = *pos;
            (*track).last_size = *size;
        } else {
            let pos_delta_square = (pos.x - track.last_pos.x).powi(2) + (pos.y - track.last_pos.y).powi(2);
            let size_delta_square = (size.w - track.last_size.w).powi(2) + (size.h - track.last_size.h).powi(2);
            if plugin.skip_delta <= 0. || pos_delta_square.max(size_delta_square) > plugin.skip_delta.powi(2) {
                gis.remove(track.last_pos, track.last_size, entity);
                gis.insert(*pos, *size, entity);
                (*track).last_pos = *pos;
                (*track).last_size = *size;
            }
        }
    }
}

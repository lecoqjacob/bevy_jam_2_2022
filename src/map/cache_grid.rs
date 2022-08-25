use std::collections::{HashMap, HashSet};

use bevy::math::Vec3Swizzles;

use crate::map::*;

pub const CHUNK_RESOLUTION: usize = 20;

#[derive(Debug, Default)]
pub struct CacheGrid {
    pub grid: HashMap<(i8, i8), HashSet<Entity>>,
    pub associations: HashMap<Entity, (i8, i8)>,
}

impl CacheGrid {
    pub fn update_entity(&mut self, entity: Entity, pos: Vec2) {
        let x = pos.x;
        let y = pos.y;

        let i = (y / CHUNK_RESOLUTION as f32) as i8;
        let j = (x / CHUNK_RESOLUTION as f32) as i8;

        if let Some((old_i, old_j)) = self.associations.get(&entity) {
            let old_i = *old_i;
            let old_j = *old_j;
            if i == old_i && j == old_j {
                return;
            }
            if let Some(set) = self.grid.get_mut(&(old_i, old_j)) {
                set.remove(&entity);
                if set.is_empty() {
                    self.grid.remove(&(old_i, old_j));
                }
            }
        }

        self.grid.entry((i, j)).or_insert_with(HashSet::default);
        self.grid.get_mut(&(i, j)).unwrap().insert(entity);
        self.associations.insert(entity, (i, j));
    }

    pub fn get_nearby_entities(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        let mut result = vec![];

        let x = position.x;
        let y = position.y;

        let x_begin = x - radius;
        let y_begin = y - radius;
        let i_begin = (y_begin / CHUNK_RESOLUTION as f32) as i8;
        let j_begin = (x_begin / CHUNK_RESOLUTION as f32) as i8;

        let i_to = (radius * 2.0 / CHUNK_RESOLUTION as f32).ceil() as i8;
        let j_to = (radius * 2.0 / CHUNK_RESOLUTION as f32).ceil() as i8;

        let i_end = i_begin + i_to;
        let j_end = j_begin + j_to;

        for i in i_begin..=i_end {
            for j in j_begin..=j_end {
                if let Some(set) = self.grid.get(&(i, j)) {
                    result.extend(set.iter());
                }
            }
        }

        result
    }
}

pub fn cache_grid_update_system(
    mut cache_grid: ResMut<CacheGrid>,
    zombie_query: Query<(Entity, &Transform), Changed<Transform>>,
) {
    for (entity, transform) in zombie_query.iter() {
        cache_grid.update_entity(entity, transform.translation.xy());
    }
}

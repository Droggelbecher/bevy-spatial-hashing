
use std::collections::{HashMap, hash_map};
use std::iter::Iterator;
use std::fmt::Debug;

use bevy::{
    prelude::*,
    math::{vec2, ivec2},
};

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    pub spacing: f32
}

impl Grid {
    fn index1d(&self, ordinate: f32) -> i32 {
        (ordinate / self.spacing).floor() as i32
    }

    fn position1d_low(&self, index: i32) -> f32 {
        (index as f32) * self.spacing
    }

    fn index2d(&self, position: Vec2) -> IVec2 {
        ivec2(
            self.index1d(position.x),
            self.index1d(position.y)
        )
    }
}

pub trait Query: Debug {
    fn first_cell(&self, grid: Grid) -> IVec2;
    fn next_cell(&self, cell: IVec2, grid: Grid) -> Option<IVec2>;
    fn in_range(&self, position: Vec2) -> bool;
}

#[derive(Debug, Default)]
pub struct SquareQuery {
    center: Vec2,
    radius: f32,
}

impl SquareQuery {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Query for SquareQuery {

    fn first_cell(&self, grid: Grid) -> IVec2 {
        grid.index2d(self.center - vec2(self.radius, self.radius))
    }

    fn next_cell(&self, mut cell_index: IVec2, grid: Grid) -> Option<IVec2> {
        cell_index.x += 1;

        if grid.position1d_low(cell_index.x) > self.center.x + self.radius {
            cell_index.x = grid.index1d(self.center.x - self.radius);
            cell_index.y += 1;
        }

        if grid.position1d_low(cell_index.y) > self.center.y + self.radius {
            return None;
        }

        Some(cell_index)
    }

    fn in_range(&self, position: Vec2) -> bool {
        position.x >= self.center.x - self.radius &&
            position.x < self.center.x + self.radius &&
            position.y >= self.center.y - self.radius &&
            position.y < self.center.y + self.radius
    }
}


// TODO: Hide this hebind Iterator trait
pub struct SpatialHashmapIterator<'a, Q: Query> {
    query: Q,
    current_cell: IVec2,
    entity_iterator: Option<hash_map::Iter<'a, Entity, Vec2>>,
    shm: &'a HashMap<IVec2, HashMap<Entity, Vec2>>,
    grid: Grid,
}


impl<'a, Q> Iterator for SpatialHashmapIterator<'a, Q> where Q: Query {
    type Item = (Entity, Vec2);

    fn next(&mut self) -> Option<Self::Item> {

        // If we have an entity iterator (i.e. we are in a valid cell),
        // iterate until we find an entity that is in range or the iterator is exhausted.

        if let Some(mut it) = self.entity_iterator.take() {
            while let Some((&entity, &pos)) = it.next() {
                if self.query.in_range(pos) {
                    // put back the iterator for next time
                    self.entity_iterator = Some(it);
                    return Some((entity, pos));
                }
            }

            // No point in putting back an exhausted iterator
            //self.entity_iterator = Some(it);
        }

        // Is there a next cell we should be looking at?
        while let Some(next_cell) = self.query.next_cell(self.current_cell, self.grid) {
            self.current_cell = next_cell;
            if let Some(entities) = self.shm.get(&self.current_cell) {
                self.entity_iterator = Some(entities.iter());
                return self.next();
            }
        }

        // No cells left to check, we have seen it all!
        None
    }
}

#[derive(Debug)]
pub struct SpatialHashmap {
    pub grid: Grid,
    hashmap: HashMap<IVec2, HashMap<Entity, Vec2>>,
}

impl SpatialHashmap {

    pub fn new(grid_size: f32) -> Self {
        Self {
            grid: Grid { spacing: grid_size },
            hashmap: default()
        }
    }

    pub fn insert(&mut self, position: Vec2, entity: Entity) {
        let index = self.grid.index2d(position);
        self.hashmap
            .entry(index)
            .or_insert(default())
            .insert(entity, position);
    }

    pub fn update(&mut self, entity: Entity, previous_position: Vec2, new_position: Vec2) {
        let prev_index = self.grid.index2d(previous_position);
        let new_index = self.grid.index2d(new_position);

        if new_index != prev_index {
            // If old cell exists, remove entry from it
            self.hashmap.entry(prev_index).and_modify(|h| { h.remove(&entity); });
        }

        // Ensure new cell exists and insert entity into it
        self.hashmap.entry(new_index)
            .or_default()
            .insert(entity, new_position);

    }

    pub fn remove(&mut self, position: Vec2, entity: Entity) {
        let index = self.grid.index2d(position);
        self.hashmap
            .entry(index)
            .or_insert(default())
            .remove(&entity);
    }

    pub fn query<Q: Query>(&self, query: Q) -> SpatialHashmapIterator<'_, Q> {
        let first_cell = query.first_cell(self.grid);

        SpatialHashmapIterator {
            query,
            current_cell: first_cell,
            entity_iterator: self.hashmap.get(&first_cell).map(|x| x.iter()),
            shm: &self.hashmap,
            grid: self.grid,
        }
    }

}


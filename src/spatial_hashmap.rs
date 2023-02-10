
use std::collections::{HashMap, HashSet};
use math::round::floor;
use std::iter::{Iterator, Filter};

use lending_iterator::prelude::*;

use bevy::{
    prelude::*,
    math::{vec2, ivec2},
};


// TODO: How can we make the iterator general such that it can:
// - Use an arbitrary function to walk grid cells
// - Use an arbitrary function to check for reachability of individual entities?
//
// (a) Could pass closures -> bad, causes extra allocation!
// (b) function pointers and then some "userdata" passed along? -> ugly!
// (c) Implement various iterators
//     can that be made in a way that is not a lot of code overhead?

trait QueryRange {
    fn first_cell(&self) -> IVec2;
    fn next_cell(&self, cell: IVec2) -> Option<IVec2>;
    fn in_range(&self, position: Vec2) -> bool;
}


struct SpatialHashmapIterator<R: QueryRange, I: Iterator<Item = Entity>> {
    query_range: R,
    //top_left_grid_index: IVec2,
    //bottom_right_grid_index: IVec2,

    current_cell: IVec2,
    //entity_iterator: HashMap<Vec2, Entity>::Iterator,
    //entity_iterator: Filter< HashMap<Vec2, Entity>::Iter, R::in_range >,
    
    // This iterator should actually be a Filter that uses
    // self.query_range.in_range.
    // https://doc.rust-lang.org/std/iter/struct.Filter.html
    // Can we even do this? How do we initialize this thing?
    entity_iterator: I,
}


impl Iterator for SpatialHashmapIterator {
    type Item = Entity;

    /*
        for ix in round(position.0 - radius) .. round(position.0 + radius) {
            for iy in round(position.1 - radius) .. round(position.1 + radius) {
                if let Some(points) = self.hashmap.get(ivec2(ix, iy)) {
                    for point in points {
                        if point_in_square(position, 
            } // iy
        } // ix
    */
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, entity)) = self.entity_iterator.next() {
            return entity;
        }

        if let Some(next_cell) = self.query_range.next_cell(self.current_cell) {
            self.current_cell = next_cell;
            // TODO assert current cell exists in shm?
            // XXX
            self.entity_iterator = Self::I(self.shm.get(self.current_cell).iter());
            return self.next();
        }


        // ----

        // Increment x position of cell until..
        // position is out of range -> continue with y position
        // A non-empty cell is found -> yield entities from that cell

        'y: loop {
            'x: loop {
                self.grid_index.0 += 1;

                if !grid_index_in_range(grid_index) {
                    break 'x; // proceed with next y coordinate
                }
                if let Some(cell_entities) = self.hashmap.get(self.grid_index) {
                    return self.next();
                }
            }
    }
}



pub struct SpatialHashmap {
    grid_size: f32,
    hashmap: HashMap<IVec2, HashSet<Entity>>,
}

impl SpatialHashmap {

    pub fn insert(&mut self, position: Vec2, entity: Entity) {
        let index = grid_index(position);
        self.hashmap
            .entry(index)
            .or_insert(default())
            .insert(entity)
    }

    pub fn remove(&mut self, position: Vec2, entity: Entity) {
        let index = grid_index(position);
        self.hashmap
            .entry(index)
            .or_insert(default())
            .remove(entity)
    }

    pub fn query_square(&self, position: Vec2, radius: f32) {

    }

    fn point_in_square(point: Vec2, square_center: Vec2, square_radius: f32) -> bool {
        point.0 >= square_center.0 - radius &&
            point.0 <= square_center.0 + radius &&
            point.1 >= square_center.1 - radius &&
            point.1 <= square_center.1 + radius
    }

    fn round(ordinate: f32) -> i32 {
        floor(ordinate / grid_size)
    }

    fn grid_index(position: Vec2) -> IVec2 {
        ivec2(
            round(position.0),
            round(position.1)
        )
    }
}

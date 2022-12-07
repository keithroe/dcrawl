use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: map::Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: map::Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(map::TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: map::TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        'create_rooms: while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 11),
                rng.range(1, SCREEN_HEIGHT - 11),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    continue 'create_rooms;
                }
            }

            room.for_each(|p| {
                let idx = self.map.try_idx(p).unwrap();
                self.map.tiles[idx] = map::TileType::Floor;
            });
            self.rooms.push(room);
        }
    }

    fn build_vertical_tunnel(&mut self, x: i32, y0: i32, y1: i32) {
        use std::cmp::{max, min};
        for y in min(y0, y1)..=max(y0, y1) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = map::TileType::Floor;
            }
        }
    }

    fn build_horizontal_tunnel(&mut self, x0: i32, x1: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x0, x1)..=max(x0, x1) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = map::TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.build_horizontal_tunnel(prev.x, new.x, new.y);
                self.build_vertical_tunnel(prev.x, prev.y, new.y);
            } else {
                self.build_horizontal_tunnel(prev.x, new.x, prev.y);
                self.build_vertical_tunnel(new.x, prev.y, new.y);
            }
        }
    }
}

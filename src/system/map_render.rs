use crate::prelude::*;

pub fn map_render(map: Res<map::Map>, camera: Res<camera::Camera>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let p = Point::new(x, y);
            let offset = Point::new( camera.left_x, camera.top_y);
            if map.in_bounds(p) {
                let idx = map::Map::idx(x,y);
                let glyph = match map.tiles[idx] {
                    map::TileType::Floor => to_cp437('.'),
                    map::TileType::Wall => to_cp437('#'),
                };
                draw_batch.set( p-offset, ColorPair::new(WHITE,BLACK), glyph); 
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

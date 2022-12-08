use crate::prelude::*;

pub fn entity_render( 
    camera: Res<camera::Camera>,
    query: Query<(&component::Pos, &component::Render)>
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    for (pos, render) in &query {
        draw_batch.set( pos.p - offset, render.color, render.glyph);
    }
    draw_batch.submit(5000).expect("Batch error");
}

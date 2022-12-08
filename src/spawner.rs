use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.spawn((
        component::Player,
        component::Pos { p: pos },
        component::Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

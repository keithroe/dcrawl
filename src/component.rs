pub use crate::prelude::*;

#[derive(Component)]
pub struct Pos {
    pub p: Point,
}

#[derive(Component)]
pub struct Render {
    pub color : ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component)]
pub struct Player;
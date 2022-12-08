use crate::prelude::*;

pub fn test(query: Query<(&component::Player, &mut component::Pos)>) {}

pub fn player_input(
    map: Res<map::Map>,
    key: Res<resource::Key>,
    mut camera: ResMut<camera::Camera>,
    mut query: Query<(&component::Player, &mut component::Pos)>,
) {
    if let Some(key) = key.key {
        let delta = match key {
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::D => Point::new(1, 0),
            _ => Point::zero(),
        };

        for (player, mut pos) in &mut query {
            let dest = pos.p + delta;
            if map.can_enter(dest) {
                pos.p = dest;
                camera.on_player_move(dest);
            }
        }
    }
}

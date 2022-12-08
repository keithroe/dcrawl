mod component;
mod map_builder;
mod resource;
mod spawner;
mod system;

mod prelude {
    pub use crate::*;
    pub use crate::system::*;
    pub use crate::resource::*;
    pub use bevy_ecs::prelude::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}
use prelude::*;

struct State {
    world: World,
    schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let mut world = World::new();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = map_builder::MapBuilder::new(&mut rng);
        world.insert_resource(map_builder.map);
        world.insert_resource(camera::Camera::new(map_builder.player_start));
        spawner::spawn_player(&mut world, map_builder.player_start);

        let mut schedule = Schedule::default();
        build_schedule(&mut schedule);
        Self { world, schedule }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.world.insert_resource(resource::Key{key: ctx.key});
        self.schedule.run(&mut self.world);
        render_draw_buffer(ctx).expect("Render error");
    }
}

//fn main() -> BError {
fn main() {
    let context = BTermBuilder::new()
        .with_title("DungeonCrawler")
        .with_fps_cap(30.0_f32)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()
        .unwrap();

    main_loop(context, State::new());
}

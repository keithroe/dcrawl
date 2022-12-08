use crate::prelude::*;

mod player_input;
mod entity_render;
mod map_render;

pub fn build_schedule( schedule: &mut Schedule) {

    #[derive(StageLabel)]
    pub struct PlayerInputLabel;

    // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
    // before moving on to the next Stage
    schedule.add_stage(PlayerInputLabel, SystemStage::parallel()
        .with_system(system::player_input::player_input)
        .with_system(system::map_render::map_render)
        .with_system(system::entity_render::entity_render)
    );
}


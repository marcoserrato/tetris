mod render;
mod player_input;
mod movement;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render::render_system())
        .add_system(player_input::player_input_system())
        .add_system(movement::movement_system())
        .build()
}

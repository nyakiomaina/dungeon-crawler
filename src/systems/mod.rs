mod player_input;
mod map_render;

use crate::prelude::*;

// function creates the legion schedule-an execution plan for your systems
pub fn build_scheduler() -> Schedule {
    // Creating a new Schedule using the builder pattern.
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}
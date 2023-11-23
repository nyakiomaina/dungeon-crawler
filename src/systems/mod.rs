mod player_input;
mod map_render;
mod entity_render;
mod collisions;

use crate::prelude::*;

// function creates the legion schedule-an execution plan for your systems
pub fn build_scheduler() -> Schedule {
    // Creating a new Schedule using the builder pattern.
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(collisions::collisions_system())
        .build()
}
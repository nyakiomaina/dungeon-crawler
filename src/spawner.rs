// handling spawning entities
use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) { // func requires a mutable reference to the World and the location to spawn the player                                         
    ecs.push(( // component created by calling push -- commponents separated ina tuple
        // CALLING PUSH CREATES A NEW ENTITY COMPOSE OF THE LISTED COMPONENTS
            Player{},
            pos,
            Render{ // PLAYERS APPEARANCE
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }));
}

// CALLING SPAWN_PLAYER FUNCTION ADDS THE PLAYER AND THEIR COMPONENTS TO THE ECS
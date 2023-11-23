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

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((Enemy, pos, Render{
        color: ColorPair::new(WHITE,BLACK),
        glyph: match rng.range(0,4) {
            0 => to_cp437('E'),
            1 => to_cp437('O'),
            2 => to_cp437('o'),
            _ => to_cp437('g')
        }
    }));
}
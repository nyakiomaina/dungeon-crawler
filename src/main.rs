mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map:Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self { 
            map:Map::new(),
            player:Player::new(
                Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
            ),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx:&mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.update(ctx, &self.map);
        self.player.render(ctx);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Dragon")
        .with_fps_cap(30.0)
        .build()
        .map_err(|e| e)?;

    main_loop(context, State::new())
        .map_err(|e| e)
}

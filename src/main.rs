mod map;
mod player;
mod map_builder;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;

pub const DISPLAY_WIDTH:i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT:i32 = SCREEN_HEIGHT / 2;

struct State {
    map:Map,
    player: Player,
    map_builder: MapBuilder,
    camera: Camera,
}

impl State {
    // fn new() -> Self {
    //     Self { 
    //         map:Map::new(),
    //         player:Player::new(
    //             Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
    //         ),
    //     }
    // }
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map.clone(),
            player:Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
            map_builder,
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
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()
        .map_err(|e| e)?;

    main_loop(context, State::new())
        .map_err(|e| e)
}

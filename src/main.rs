// Modules related to various game functionalities.
mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

// A prelude module to consolidate and simplify imports for the rest of the codebase.
mod prelude {
    // Importing essential libraries and modules for game development.
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    
    // Constants for screen dimensions.
    pub const SCREEN_WIDTH:i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    
    // Re-exporting modules from the crate for easy access.
    pub use crate::map::*; 
    // pub use crate::player::*; 
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

// Using the prelude to simplify the main code.
use prelude::*;

// Constants for the display dimensions.
pub const DISPLAY_WIDTH:i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT:i32 = SCREEN_HEIGHT / 2;

// The main game state structure.
struct State {
    ecs: World, // The Entity Component System world.
    resources: Resources, // Resources used in the game.
    systems: Schedule, // Systems schedule for game logic.
    map:Map,
    // map_builder: MapBuilder;
    // camera: Camera; 
}

impl State {
    // Constructor for the game state.
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Spawning the player at the designated starting position.
        spawn_player(&mut ecs, map_builder.player_start);
        
        // Inserting the map and camera into resources.
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
            map: Map::new(),
            // player:Player::new(map_builder.player_start); 
            // camera: Camera::new(map_builder.player_start); 
            // map_builder; 
        }
    }
}

// Implementing the game state logic.
impl GameState for State {
    // The main game loop.
    fn tick(&mut self, ctx:&mut BTerm) {
        // Setting and clearing the active consoles.
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        
        // Inserting the current key event into resources.
        self.resources.insert(ctx.key);
        
        // Executing all the game systems.
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render Error")

        // Old rendering and update logic, replaced by systems.
        // self.map.render(ctx, &self.camera);
        // self.player.update(ctx, &self.map, &mut self.camera);
        // self.player.render(ctx, &self.camera);
    }
}

// Main function to initialize and run the game.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Setting up the game context with various configurations.
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

    // Starting the game loop with the initial state.
    main_loop(context, State::new())
        .map_err(|e| e)
}

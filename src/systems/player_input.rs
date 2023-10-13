use crate::prelude::*;

#[system] //annotates the player_input function with proc macro system
#[write_component(Point)] // request writable access to the component type Point (must rquest writ access if you intend to change the contents of a component in your system)
#[read_component(Player)] // request read-only access to the component type
pub fn player_input(
    ecs:&mut SubWorld, // like a World but can only see the cimponents you requested
    // resources -> proc macro -> requests access to types stored in the Legion's resource handler
    #[resource]map:&Map,
    #[resource]key:&Option<VirtualKeyCode>,
    #[resource]camera:&mut Camera
)
{
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1,0),
            VirtualKeyCode::Up => Point::new(0,-1),
            VirtualKeyCode::Down => Point::new(0,1),
            _=> Point::new(0,0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()
                .filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
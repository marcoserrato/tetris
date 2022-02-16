// Player input system

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Piece)]
#[read_component(ActivePiece)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>) {
    let active_piece = <(Entity, &Point, &Piece)>::query().filter(component::<ActivePiece>()).iter(ecs).nth(0).unwrap();
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::zero()
        };

        if delta.x != 0 || delta.y != 0 {
            commands.
                push(((), WantsToMove {
                    piece: *active_piece.0,
                    destination: *active_piece.1 + delta
                }));
        };
    };
}

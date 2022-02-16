// Move system, consumers move events to move the active piece

use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToMove)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer) {
    commands.add_component(want_move.piece, want_move.destination);
    commands.remove(*entity);
}

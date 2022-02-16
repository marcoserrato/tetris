mod components;
mod system;

mod prelude {
    pub use std::{thread, time};
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH /2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::system::*;
    pub use crate::components::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    system: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        ecs.push((
            Point::new(1, 0),
            Piece::new(Shape::L),
            ActivePiece{}
        ));
        ecs.push((
            Point::new(1, 1),
            Piece::new(Shape::Z)
        ));
        ecs.push((
            Point::new(1, 2),
            Piece::new(Shape::Flat)
        ));
        ecs.push((
            Point::new(1, 3),
            Piece::new(Shape::Square)
        ));
        let resources = Resources::default();
        Self {
            ecs,
            resources,
            system: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.system.execute(
            &mut self.ecs,
            &mut self.resources
        );
        render_draw_buffer(ctx).expect("Render Error")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Tetris")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)
        .with_resource_path("resources/")
        .with_tile_dimensions(32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}

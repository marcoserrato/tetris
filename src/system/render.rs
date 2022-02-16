use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Piece)]
pub fn render(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let mut renderables = <(&Point, &Piece)>::query().filter(component::<Piece>());
    renderables
        .iter(ecs)
        .for_each(|(p, piece)| {
            let color = match piece.shape {
                Shape::L => ORANGE,
                Shape::J => BLUE,
                Shape::T => PURPLE,
                Shape::Square => YELLOW,
                Shape::Flat => CYAN,
                Shape::Z => RED,
                Shape::S => GREEN
            };
            draw_batch.set(
                *p,
                ColorPair::new(color, WHITE),
                to_cp437('#'));
        });
    draw_batch.submit(0).expect("Batch error");
}

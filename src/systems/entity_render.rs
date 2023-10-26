use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(esc: &SubWorld, #[resource] camera: &Camera ) {
    //batch for drawing
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1); // set the drawing layer target
    // calculate offset based on camera's position
    let offset = Point::new(camera.left_x, camera.top_y);


    //query entity with Point and Render components

    <(&Point, &Render)>::query().iter(esc).for_each(|(pos, render)| {
        draw_batch.set(
            *pos - offset,
            render.color,
            render.glyph
        );
    });
    draw_batch.submit(5000).expect("Batch error");
}
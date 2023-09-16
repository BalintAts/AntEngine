use super::super::components_module::components::{Position, Renderable};
use super::super::game_object_module::game_object::GameObject;
use std::cell::RefCell;

pub fn apply_render_system(
    game_objects: &Vec<GameObject>,
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
) {
    for gameobject in game_objects {
        render(
            &gameobject.renderable,
            &gameobject.position,
            buffer,
            width,
            height,
        )
    }
}

pub fn clear(buffer: &mut Vec<u32>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = 0;
        }
    }
}

pub fn render(
    renderable: &Renderable,
    // go_pos: &Position,
    go_pos: &RefCell<Position>,
    buffer: &mut Vec<u32>,
    screen_width: usize,
    screen_height: usize,
) {
    let rounded_x = (go_pos.borrow().x + renderable.x).round() as usize;
    let rounded_y = (go_pos.borrow().y + renderable.y).round() as usize;

    let length = buffer.len().clone();

    for pix_y in rounded_y..(rounded_y + renderable.get_bounding_height()) {
        for pix_x in rounded_x..(rounded_x + renderable.get_bounding_width()) {
            let signed_buffer_index = pix_y * screen_width + pix_x;
            let buffer_index = signed_buffer_index as usize;
            let shape_center_x = renderable.get_bounding_width() / 2;
            let shape_center_y = renderable.get_bounding_height() / 2;

            if buffer_index < buffer.len() - shape_center_x - shape_center_y
                && buffer_index >= shape_center_x * shape_center_y
            {
                buffer[buffer_index] = renderable.draw(
                    // buffer[buffer_index % length] = shape.draw(
                    pix_x
                        .saturating_sub(rounded_x)
                        .saturating_sub(shape_center_x),
                    pix_y
                        .saturating_sub(rounded_y)
                        .saturating_sub(shape_center_y),
                );
                //ide nem a pixel kkordináták kellenek, hanem a bounding boxban levő koordináták
            }
        }
    }
}

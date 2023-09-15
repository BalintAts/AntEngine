use super::super::components_module::components::{Position, Renderable};
use super::super::game_object_module::game_object::GameObject;

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
    shape: &Renderable,
    go_pos: &Position,
    buffer: &mut Vec<u32>,
    screen_width: usize,
    screen_height: usize,
) {
    let rounded_x = (go_pos.x + shape.x).round() as i32;
    let rounded_y = (go_pos.y + shape.y).round() as i32;

    for pix_y in rounded_y..(rounded_y + shape.get_bounding_height()) {
        for pix_x in rounded_x..(rounded_x + shape.get_bounding_width()) {
            let signed_buffer_index = pix_y * screen_width as i32 + pix_x;
            let buffer_index = signed_buffer_index as usize;
            if buffer_index
                < buffer.len()
                    - shape.get_bounding_width() as usize / 2
                    - shape.get_bounding_height() as usize / 2
                && buffer_index
                    >= shape.get_bounding_width() as usize / 2
                        * shape.get_bounding_height() as usize
                        / 2
            {
                buffer[buffer_index] = shape.draw(rounded_x, rounded_y); //ide nem a pixel kkordináták kellenek, hanem a bounding boxban levő koordináták
            }
        }
    }
}

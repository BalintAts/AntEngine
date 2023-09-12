use minifb::{Key, Scale, Window, WindowOptions};
use rand::Rng;
use std::collections::HashMap;

mod gameobject;
mod visualScriptInterpreter;
use gameobject::{render, to_move, GameObject, Position, Renderable, Velocity};

const WIDTH: usize = 900;
const HEIGHT: usize = 600;

fn main() {
    // let mut gameObjects: Box<Vec<&GameObject>> = Box::new(Vec::new());
    // create_gameobjects_and_put_into(&mut gameObjects);

    let mut game_objects: Vec<GameObject> = Vec::new();
    create_and_push_gameobject_random(&mut game_objects);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Something",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X1,
            resize: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let mut frame_counter_to_spawn = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for game_object in &mut *game_objects {
            to_move(&mut game_object.position, &mut game_object.velocity);
        }
        apply_render_system(&game_objects, &mut buffer);
        if frame_counter_to_spawn == 10 {
            create_and_push_gameobject_random(&mut game_objects);
            frame_counter_to_spawn = 0;
        }

        window.update_with_buffer(&buffer).unwrap();

        clear(&mut buffer);

        frame_counter_to_spawn += 1;
    }
}

//rendersystem

fn apply_render_system(gameObjects: &Vec<GameObject>, buffer: &mut Vec<u32>) {
    for gameobject in gameObjects {
        render(
            &gameobject.renderable,
            &gameobject.position,
            buffer,
            WIDTH,
            HEIGHT,
        )
    }
}
//fn render(graphcis)
// forloop graphics forloop shapes

// fn render(gameObject: &GameObject, buffer: &mut Vec<u32>) {
//     let rounded_x = gameObject.pos_x.round() as i32;
//     let rounded_y = gameObject.pos_y.round() as i32;

//     for y in rounded_y..(rounded_y + gameObject.bounding_height) {
//         for x in rounded_x..(rounded_x + gameObject.bounding_width) {
//             let signed_buffer_index = y * WIDTH as i32 + x;
//             let buffer_index = signed_buffer_index as usize;
//             if (buffer_index
//                 < (WIDTH - gameObject.bounding_width as usize / 2)
//                     * (HEIGHT - gameObject.bounding_height as usize / 2)
//                 && buffer_index
//                     >= gameObject.bounding_width as usize / 2 * gameObject.bounding_height as usize
//                         / 2)
//             {
//                 buffer[buffer_index] = gameObject.draw(x as i32, y as i32);
//             }
//         }
//     }
// }

fn clear(buffer: &mut Vec<u32>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = 0;
        }
    }
}

//spawnersystem
fn create_and_push_gameobject_random(game_objects: &mut Vec<GameObject>) {
    //ez majd a level impl-jében lesz, és akkor nem lesz paraméter
    //todo: ezt nem így kell, hanem a kompnonenseket kell létrehozni, majd adni nekik közös id-t
    let id = String::from("something");
    let mut rng = rand::thread_rng();
    let p_x = rng.gen_range(0..WIDTH) as f64;
    let p_y = rng.gen_range(0..HEIGHT) as f64;
    let r = rng.gen_range(0..30) as f64;

    let gameobject = GameObject {
        id: String::from("1"),
        position: Position {
            x: p_x,
            y: p_y,
            id: id.clone(),
        },
        velocity: Velocity {
            x: p_x,
            y: p_y,
            id: id.clone(),
        },
        renderable: Renderable {
            r: r,
            id: String::from("something"),
            x: 0.0,
            y: 0.0,
            color: 0xffff00,
        },
        childs: Vec::new(),
    };

    game_objects.push(gameobject);
}

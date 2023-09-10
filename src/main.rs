use minifb::{Key, Scale, Window, WindowOptions};
use rand::Rng;
use std::collections::HashMap;

mod gameobject;
mod visualScriptInterpreter;
use gameobject::Circle;
use gameobject::GameObject;

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
            game_object.to_move();
            render(&game_object, &mut buffer); //ez nem jó, gameObjektenként növekszik, miközben csak az utolsó marad látható
        }
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
fn render(gameObject: &GameObject, buffer: &mut Vec<u32>) {
    let rounded_x = gameObject.pos_x.round() as i32;
    let rounded_y = gameObject.pos_y.round() as i32;

    for y in rounded_y..(rounded_y + gameObject.bounding_height) {
        for x in rounded_x..(rounded_x + gameObject.bounding_width) {
            let signed_buffer_index = y * WIDTH as i32 + x;
            let buffer_index = signed_buffer_index as usize;
            if (buffer_index
                < (WIDTH - gameObject.bounding_width as usize / 2)
                    * (HEIGHT - gameObject.bounding_height as usize / 2)
                && buffer_index
                    >= gameObject.bounding_width as usize / 2 * gameObject.bounding_height as usize
                        / 2)
            {
                buffer[buffer_index] = gameObject.draw(x as i32, y as i32);
            }
        }
    }
}

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
    let mut rng = rand::thread_rng();
    let p_x = rng.gen_range(0..WIDTH) as f64;
    let p_y = rng.gen_range(0..HEIGHT) as f64;
    let r = rng.gen_range(0..30) as f64;

    let gameobject = GameObject {
        id: String::from("1"),
        pos_x: p_x,
        pos_y: p_y,
        vel_x: 0.0,
        vel_y: 0.0,
        rotation: 0.0,
        shape: Circle { radius: r },
        color: 0xffff00,
        // x: p_x - r,   TODO: OFFSET!!!
        // y: p_y - r,
        bounding_width: 2 * r.round() as i32,
        bounding_height: 2 * r.round() as i32,
        physics_body: Circle { radius: r },
        script: String::from(""),
        childs: HashMap::new(),
    };

    game_objects.push(gameobject);
}

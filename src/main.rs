use minifb::{Key, Scale, Window, WindowOptions};
use rand::Rng;
use std::cell::RefCell;

mod components_module;
use components_module::components::{Position, Renderable, Velocity};

mod renderer;
use renderer::render_system::{apply_render_system, clear};

mod game_object_module;
use game_object_module::game_object::GameObject;

mod movement;
use movement::movement_system::apply_distance_interaction;

const WIDTH: usize = 900;
const HEIGHT: usize = 600;

fn main() {
    let mut game_objects: Vec<GameObject> = Vec::new();
    for i in 0..250 {
        create_and_push_gameobject_random(&mut game_objects);
    }
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

    // let mut frame_counter_to_spawn = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for game_object in &mut *game_objects {
        //     // to_move(&mut game_object.position, &mut game_object.velocity);
        // }
        apply_distance_interaction(&game_objects);
        apply_render_system(&game_objects, &mut buffer, WIDTH, HEIGHT);
        // if frame_counter_to_spawn == 10 {
        //     create_and_push_gameobject_random(&mut game_objects);
        //     frame_counter_to_spawn = 0;
        // }

        window.update_with_buffer(&buffer).unwrap();

        clear(&mut buffer, WIDTH, HEIGHT);

        // frame_counter_to_spawn += 1;
    }
}

fn create_and_push_gameobject_random(game_objects: &mut Vec<GameObject>) {
    //ez majd a level impl-jében lesz, és akkor nem lesz paraméter
    //todo: ezt nem így kell, hanem a kompnonenseket kell létrehozni, majd adni nekik közös id-t

    let id = String::from("something");
    let mut rng = rand::thread_rng();
    let p_x = rng.gen_range(0..WIDTH) as f64;
    let p_y = rng.gen_range(0..HEIGHT) as f64;
    // let r = rng.gen_range(0..30) as f64;
    let r = 5.0;

    let velocity = Velocity { x: 0.0, y: 0.0 };
    let position = Position { x: p_x, y: p_y };

    let gameobject = GameObject {
        id: String::from("1"),
        position: RefCell::new(position),
        velocity: RefCell::new(velocity),
        renderable: Renderable {
            r: r,
            x: 0.0,
            y: 0.0,
            color: 0xffff00,
        },
        childs: Vec::new(),
    };

    game_objects.push(gameobject);
}

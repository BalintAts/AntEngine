use std::borrow::BorrowMut;
use std::cell::RefCell;

use super::super::components_module::components::{Position, Velocity};
use super::super::game_object_module::game_object::GameObject;

// pub fn to_move(mut pos: &mut Position, mut vel: &mut Velocity) {
//     fall(&mut pos, &mut vel);
// }

// fn fall(pos: &mut Position, vel: &mut Velocity) {
//     vel.y += 0.1;
//     pos.y += vel.y;
// }

pub fn apply_distance_interaction(game_objects: &Vec<GameObject>) {
    for g_o_first in game_objects {
        for g_o_second in game_objects {
            if g_o_first == g_o_second {
                continue;
            }
            gravitate(&g_o_first, &g_o_second);

            // let g_o_value = *g_o_first;

            // velocity_first_value.borrow_mut().x += gravity.0;
            // velocity_first_value.borrow_mut().y += gravity.1;
        }
    }
}

fn gravitate(g_o_1: &GameObject, g_o_2: &GameObject) -> (f64, f64) {
    let dx = g_o_2.position.borrow().x - g_o_1.position.borrow().x;
    let dy = g_o_2.position.borrow().y - g_o_1.position.borrow().y;

    let r = (dx * dx + dy * dy).sqrt();

    if r == 0.0 {
        return (0.0, 0.0);
    }

    // let f_magnitude = (g * p1.mass * p2.mass) / (r * r);

    // let f_x = f_magnitude * (dx / r);
    // let f_y = f_magnitude * (dy / r);
    let factor = 0.0001;

    let f_x = factor * (dx / r) / r * r;
    let f_y = factor * (dy / r) / r * r;

    // let vel1_value = &vel1;
    // *vel1_value.x.borrow_mut() += f_x;
    // *vel1_value.y.borrow_mut() += f_y; //todo ennek utána nézni!

    g_o_1.velocity.borrow_mut().x += f_x; //todo ennek utána nézni!
    g_o_1.velocity.borrow_mut().y += f_y; //todo ennek utána nézni!

    g_o_1.position.borrow_mut().x += g_o_1.velocity.borrow().x;
    g_o_1.position.borrow_mut().y += g_o_1.velocity.borrow().y;

    //g_o_2 -t mozgatja a második visszatérés a loopba
    (f_x, f_y)
}

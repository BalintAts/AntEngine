use super::super::components_module::components::{Position, Velocity};

pub fn to_move(mut pos: &mut Position, mut vel: &mut Velocity) {
    fall(&mut pos, &mut vel);
}

fn fall(pos: &mut Position, vel: &mut Velocity) {
    vel.y += 0.5;
    pos.y += vel.y;
}

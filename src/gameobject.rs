use std::collections::HashMap;

//todo: implement default
pub struct GameObject {
    pub id: String, //todo: str or String?
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub rotation: f64,
    pub shape: Circle,
    pub color: u32, //hexa
    pub bounding_width: i32,
    pub bounding_height: i32,
    pub physics_body: Circle, //Shape trait
    pub script: String,       //Json
    pub childs: HashMap<String, GameObject>,
}

//components
pub struct Position {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

pub struct Rotation {
    //tranfrom kell
    pub id: String,
    pub theta: f64,
}

pub struct Velocity {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

pub struct Shape {
    //circle
    pub r: f64,
    // pub tr: GhraphicsTransform  (relative to gameobject trancform)
}

pub struct Graphic {
    //trait graphic
    //lista shape-ekkel, sprite-ekkel (sptritelista animációhoz) és a posiition + rotation 9 scale offsetekkel
    pub shapes: Vec<Shape>,
}

//movement system
fn to_move(mut pos: &mut Position, mut vel: &mut Velocity) {
    fall(&mut pos, &mut vel);
}

fn fall(pos: &mut Position, vel: &mut Velocity) {
    vel.y += 0.5;
    pos.y += vel.y;
}

//end movement system

fn render(graphic: Graphic) //GameObjectTransform, RelativeTransform){
{
}

impl GameObject {
    // pub fn to_move(&mut self) {
    //     self.fall();
    // }

    // fn fall(&mut self) {
    //     self.vel_y += 0.5;
    //     self.pos_y += self.vel_y;
    // }

    pub fn draw(&self, pixel_index_x: i32, pixel_index_y: i32) -> u32 {
        if self.shape.draw(
            self.pos_x + self.bounding_width as f64 / 2 as f64 - pixel_index_x as f64,
            self.pos_y + self.bounding_height as f64 / 2 as f64 - pixel_index_y as f64,
        ) {
            return self.color;
        }
        return 0x0000ff;
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    fn draw(&self, pixel_index_x: f64, pixel_index_y: f64) -> bool {
        return (pixel_index_x) * (pixel_index_x) + (pixel_index_y) * (pixel_index_y)
            <= self.radius * self.radius;
    }
}

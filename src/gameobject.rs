use std::collections::HashMap;

//todo: implement default

//ez nem lesz, a GO csak egy Id
pub struct GameObject {
    pub id: String, //todo: str or String?
    pub position: Position,
    pub velocity: Velocity,
    // pub rotation: f64,
    pub renderable: Renderable, //lehetne shape array
    // pub physics_body: Renderable, //Shape trait
    // pub script: String,           //Json
    pub childs: Vec<String>, //id-s of entities
}

//HASZNÁLJ GENERIKET!!!   <T>  !!!  Nested types:  <T<K>>

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

pub struct Renderable {
    //circle
    pub id: String,
    pub r: f64,
    pub x: f64, //relative offset from go
    pub y: f64,
    pub color: u32, // pub tr: GhraphicsTransform  (relative to gameobject trancform)
}

impl Renderable {
    //Circle in this case
    pub fn Get_Bounding_Width(&self) -> i32 {
        return self.r as i32 * 2; //interer, mert pixel coordinategj
    }

    pub fn Get_Bounding_Height(&self) -> i32 {
        return self.r as i32 * 2;
    }

    fn draw(&self, pixel_index_x: i32, pixel_index_y: i32) -> u32 {
        if (pixel_index_x) * (pixel_index_x) + (pixel_index_y) * (pixel_index_y)
            <= self.r as i32 * self.r as i32
        {
            return self.color;
        }
        return 0x0000ff;
    }
}

pub struct Graphic {
    //trait graphic
    //lista shape-ekkel, sprite-ekkel (sptritelista animációhoz) és a posiition + rotation 9 scale offsetekkel
    pub shapes: Vec<Renderable>,
}

//movement system
pub fn to_move(mut pos: &mut Position, mut vel: &mut Velocity) {
    fall(&mut pos, &mut vel);
}

fn fall(pos: &mut Position, vel: &mut Velocity) {
    vel.y += 0.5;
    pos.y += vel.y;
}
//end movement system

pub fn render(
    shape: &Renderable,
    go_pos: &Position,
    buffer: &mut Vec<u32>,
    screen_width: usize,
    screen_height: usize,
) {
    let rounded_x = (go_pos.x + shape.x).round() as i32;
    let rounded_y = (go_pos.y + shape.y).round() as i32;

    for pix_y in rounded_y..(rounded_y + shape.Get_Bounding_Height()) {
        for pix_x in rounded_x..(rounded_x + shape.Get_Bounding_Width()) {
            let signed_buffer_index = pix_y * screen_width as i32 + pix_x;
            let buffer_index = signed_buffer_index as usize;
            if buffer_index
                < buffer.len()
                    - shape.Get_Bounding_Width() as usize / 2
                    - shape.Get_Bounding_Height() as usize / 2
                && buffer_index
                    >= shape.Get_Bounding_Width() as usize / 2
                        * shape.Get_Bounding_Height() as usize
                        / 2
            {
                buffer[buffer_index] = shape.draw(rounded_x, rounded_y); //ide nem a pixel kkordináták kellenek, hanem a bounding boxban levő koordináták
            }
        }
    }
}

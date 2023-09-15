pub struct Position {
    pub id: String,
    pub x: f64,
    pub y: f64,
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
    pub fn get_bounding_width(&self) -> i32 {
        return self.r as i32 * 2; //interer, mert pixel coordinategj
    }

    pub fn get_bounding_height(&self) -> i32 {
        return self.r as i32 * 2;
    }

    pub fn draw(&self, pixel_index_x: i32, pixel_index_y: i32) -> u32 {
        if (pixel_index_x) * (pixel_index_x) + (pixel_index_y) * (pixel_index_y)
            <= self.r as i32 * self.r as i32
        {
            return self.color;
        }
        return 0x0000ff;
    }
}

pub struct Graphic {
    pub shapes: Vec<Renderable>,
}

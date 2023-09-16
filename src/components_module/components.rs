use std::cell::RefCell;

#[derive(PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq)]
pub struct Renderable {
    //circle
    pub r: f64,
    pub x: f64, //relative offset from go
    pub y: f64,
    pub color: u32, // pub tr: GhraphicsTransform  (relative to gameobject trancform)
}

impl Renderable {
    //Circle in this case
    pub fn get_bounding_width(&self) -> usize {
        return self.r as usize * 2; //interer, mert pixel coordinategj
    }

    pub fn get_bounding_height(&self) -> usize {
        return self.r as usize * 2;
    }

    pub fn draw(&self, pixel_index_x: usize, pixel_index_y: usize) -> u32 {
        // if (pixel_index_x) * (pixel_index_x) + (pixel_index_y) * (pixel_index_y)
        //     <= self.r as usize * self.r as usize
        // {
        //     return self.color;
        // }
        return 0x0000ff;
    }
}

// pub struct Graphic {
//     pub shapes: Vec<Renderable>,
// }

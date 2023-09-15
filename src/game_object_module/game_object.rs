use super::super::components_module::components::{Graphic, Position, Renderable, Velocity};

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

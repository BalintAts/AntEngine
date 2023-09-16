use super::super::components_module::components::{Position, Renderable, Velocity};
use std::cell::RefCell;

#[derive(PartialEq)]
pub struct GameObject {
    pub id: String, //todo: str or String?
    pub position: RefCell<Position>,
    // pub velocity: Cell<Velocity>,
    // pub velocity: Velocity,
    pub velocity: RefCell<Velocity>,

    // pub rotation: f64,
    pub renderable: Renderable, //lehetne shape array
    // pub physics_body: Renderable, //Shape trait
    // pub script: String,           //Json
    pub childs: Vec<String>, //id-s of entities

                             //todo: copy, clone-nak utána nézni!
}

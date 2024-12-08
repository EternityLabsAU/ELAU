use std::vec;

use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct CurvesFromData {
    data: Vec<i32>,

    base: Base<Node3D>
}

#[godot_api]
impl INode3D for CurvesFromData {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            data: Vec::with_capacity(10),

            base,
        }
    }
}

#[godot_api]
impl CurvesFromData {
    #[func]
    fn get_size_of_data(&mut self) -> i16 {
        self.data.len() as i16
    }
}
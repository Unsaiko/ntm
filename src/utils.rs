use glam::{Vec3, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
   pub x: f32,
   pub y: f32,
   pub z: f32,
   pub u: f32,
   pub v: f32,
}

impl Vertex {
    pub fn new(position: Vec3, uv: Vec2) -> Vertex {
        Vertex { x: position.x, y: position.y, z: position.z, u: uv.x, v: uv.y }
    }
}

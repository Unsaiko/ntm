use crate::{generator::Heightmap, ui::Widget};

mod perlin;

pub use {
    perlin::PerlinModifier,
};

pub trait HeightmapModifier : Widget
{
    fn modify(&self, heightmap: &mut Heightmap);
}
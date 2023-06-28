use crate::generator::{Noise, Heightmap};
use rand::Rng;
use crate::ui::Widget;

use super::HeightmapModifier;

#[derive(Clone, Copy)]
pub struct PerlinModifier {
    pub scale: f32,
    pub octaves: usize,
}

impl PerlinModifier {
    pub fn new(scale: f32, octaves: usize) -> PerlinModifier {
        PerlinModifier { scale: scale, octaves: octaves }
    }
}

impl Widget for PerlinModifier {
    fn on_render(&mut self, frame: &imgui::Ui) {
        frame.text("Scale");
        imgui::Drag::new("##PerlinSize").build(&frame, &mut self.scale);

        frame.text("Octaves");
        imgui::Drag::new("##PerlinOctaves").range(1, 15).build(&frame, &mut self.octaves);
    }
}

impl HeightmapModifier for PerlinModifier {
    fn modify(&self, heightmap: &mut Heightmap) {
        let mut rng = rand::thread_rng();
        let noise = Noise::new(rng.gen::<u64>());

        for x in 0..heightmap.size() as usize {
            for y in 0..heightmap.size() as usize {
                let rel_x = x as f32 / self.scale;
                let rel_y = y as f32 / self.scale;
                heightmap.set_height(x, y, (noise.perlin_octaves(rel_x, rel_y, self.octaves, 0.5)
                    * 255.0) as u8);
            }
        }
    }
}
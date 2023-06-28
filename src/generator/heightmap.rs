use array2d::Array2D;

use crate::{renderer::Texture2D};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Heightmap {
    size_index: usize,
    current_size: u32,
    sizes: Vec<u32>,
    data: Array2D<u8>
}

impl Heightmap {
    pub fn new() -> Heightmap {
        let sizes = vec![512, 1024, 2048];
        let size_index = 0;
        let current_size = sizes[size_index];

        Heightmap { size_index: size_index, sizes: sizes,
            data: Array2D::filled_with(0, current_size as usize, current_size as usize),
            current_size: current_size
        }
    }

    pub fn size(&self) -> u32 {
        self.current_size
    }

    pub fn get_sizes(&self) -> &[u32] {
        &self.sizes
    }

    pub fn set_height(&mut self, x: usize, y: usize, value: u8) { // maybe return except on out of bounds?
        if x > self.current_size as usize || y > self.current_size as usize {
            return;
        }

        self.data[(x, y)] = value;
    }

    pub fn set_size(&mut self, size: u32) {
        self.current_size = size;
        self.resize();
    }

    fn resize(&mut self) {
        self.data = Array2D::filled_with(0, self.current_size as usize, self.current_size as usize);
    }

    pub fn as_texture2d(&mut self) -> Texture2D {
        let mut img = image::ImageBuffer::new(self.current_size, self.current_size);
        for x in 0..self.current_size {
            for y in 0..self.current_size {
                let color = self.data[(x as usize, y as usize)];
                img.put_pixel(x, y, image::Rgb([color, color, color]));
            }
        }

        Texture2D::from_memory(self.current_size, self.current_size, gl::RGB, &img)
    }
    
    pub fn save(&self, path: &str) {
        image::save_buffer(path, &self.data.as_row_major(), self.current_size, self.current_size,
            image::ColorType::L8).unwrap();
    }
}
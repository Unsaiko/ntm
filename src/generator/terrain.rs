use array2d::Array2D;
use gl::ELEMENT_ARRAY_BUFFER;
use glam::{Vec3, Vec2, Mat4};

use crate::renderer::Buffer;
use crate::utils::Vertex;

#[derive(Clone)]
pub struct Terrain {
    verticies: Buffer,
    indicies: Buffer,
    model_matrix: Mat4,
}

impl Terrain {
    pub fn new(width: u32, height: u32) -> Terrain {
        let mut vertices = Array2D::filled_with(Vertex::new(Vec3::ZERO,
            Vec2::ZERO), width as usize, height as usize);
        Self::generate_vertices(&mut vertices);
        let mut vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        vertex_buffer.set_data(&vertices.as_column_major(), gl::STATIC_DRAW);

        let mut indices = Vec::<u32>::new();
        Self::generate_indices(&mut indices, width, height);
        let mut index_buffer = Buffer::new(ELEMENT_ARRAY_BUFFER);
        index_buffer.set_data(&indices, gl::STATIC_DRAW);

        let position = Vec3::new(-((width / 2) as f32), 0.0, -((height / 2) as f32));
        let model_matrix = Mat4::from_scale(Vec3::new(3.0, 1.0, 3.0)) * Mat4::from_translation(position);

        Terrain { verticies: vertex_buffer, indicies: index_buffer, model_matrix: model_matrix }
    }

    pub fn get_vert_buffer(&self) -> Buffer {
        self.verticies.clone()
    }

    pub fn get_ind_buffer(&self) -> Buffer {
        self.indicies.clone()
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        self.model_matrix
    }

    fn generate_vertices(vertices: &mut Array2D<Vertex>) {
        for w in 0..vertices.column_len() {
            for d in 0..vertices.row_len() {
                vertices[(w,d)] = Vertex::new(Vec3::new(w as f32, 0.0, d as f32),
                Vec2::new((w as f32) / (vertices.column_len() as f32 - 1.0) , (d as f32) / (vertices.row_len() as f32 - 1.0)));
            }
        }
    }

    fn generate_indices(indices: &mut Vec<u32>, width: u32, height: u32) {
        let mut vertex = 0;

        for _w in 0..width-1
        {
            for _d in 0..height-1
            {  
                indices.push(vertex + width);
                indices.push(vertex);
                indices.push(vertex + width + 1);
                indices.push(vertex + 1);

               vertex += 1;
            }
           vertex += 1;
        }
    }
}
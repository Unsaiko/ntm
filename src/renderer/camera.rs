use glam::{Mat4, Vec3, Vec2, Quat, EulerRot};
use sdl2::{event::{Event, WindowEvent}, mouse::{MouseButton}};

pub struct Camera {
    fov: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
    dist: f32,
    position: Vec3,
    rotation: Vec2,
    focal: Vec3,
    view_matrix: Mat4,
    proj_matrix: Mat4
}

impl Camera {
    pub fn new(aspect: f32) -> Camera {
        let mut ret = Camera { fov: 1.05, aspect_ratio: aspect, z_near: 0.01, z_far: 1000.0, dist: -65.0, position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec2::new(0.0, 0.0), focal: Vec3::new(0.0,0.0,0.0), view_matrix: Mat4::IDENTITY,
            proj_matrix: Mat4::IDENTITY };

        ret.recalc_projection();
        ret.recalc_view();

        ret
    }

    pub fn event_dispatch(&mut self, event: Event) {
        match event {
            Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                self.set_aspect((width / height) as f32)
            },
            
            Event::MouseMotion { xrel: x, yrel: y, mousestate: mouse_state, .. } => {
                if mouse_state.is_mouse_button_pressed(MouseButton::Middle) {
                    self.mouse_pan(Vec2::new(x as f32, y as f32).normalize());
                } else if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
                    self.mouse_rotate(Vec2::new(y as f32, x as f32).normalize())
                }
            },

            Event::MouseWheel { y: direction, .. } => {
                self.dist += (direction as f32);
                self.recalc_view();
            },
            _ => {}
        }
    }

    fn mouse_pan(&mut self, offset: Vec2) {
        self.focal += -self.get_right() * offset.x * 0.25;
        self.focal += self.get_up() * offset.y * 0.25;
        self.recalc_view()
    }

    fn mouse_rotate(&mut self, offset: Vec2) {
        self.rotation.x += f32::to_radians(offset.x * 3.0);
        self.rotation.y += f32::to_radians(offset.y * 3.0);
        self.recalc_view();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect_ratio = aspect;
        self.recalc_projection();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix.clone()
    }

    pub fn get_proj_matrix(&self) -> Mat4 {
        self.proj_matrix.clone()
    }

    pub fn get_up(&self) -> Vec3 {
        self.get_orientation().mul_vec3(Vec3::new(0.0, 1.0, 0.0)).normalize()
    }

    pub fn get_right(&self) -> Vec3 {
        self.get_orientation().mul_vec3(Vec3::new(1.0, 0.0, 0.0)).normalize()
    }

    pub fn get_forward(&self) -> Vec3 {
        self.get_orientation().mul_vec3(Vec3::new(0.0, 0.0, 1.0)).normalize()
    }

    pub fn get_orientation(&self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, self.rotation.x, self.rotation.y, 0.0)
    }

    pub fn recalc_position(&mut self) {
        self.position = self.focal + self.get_forward() * self.dist;
    }

    fn recalc_projection(&mut self) {
        self.proj_matrix = Mat4::perspective_lh(self.fov, self.aspect_ratio, self.z_near, self.z_far);
    }

    fn recalc_view(&mut self) {
        self.recalc_position();
        
        self.view_matrix = Mat4::from_translation(self.position) * Mat4::from_quat(self.get_orientation());
        self.view_matrix = self.view_matrix.inverse();
    }
}
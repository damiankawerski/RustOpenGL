use crate::geometry::Geometry;
use crate::glsl::GLSLProgram;
use glam::{Mat4, Vec3};
use std::collections::HashMap;

pub struct GLWidget {
    frame: u64,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    rot_x: f32,
    rot_y: f32,
    zoom: f32,
    shaders: HashMap<String, GLSLProgram>,
    geometry: HashMap<String, Geometry>,
    geometry_mat: HashMap<String, Mat4>,
}

impl GLWidget {
    pub fn new() -> Self {
        Self {
            frame: 0,
            pos_x: 0,
            pos_y: 0,
            width: 800,
            height: 600,
            rot_x: 0.0,
            rot_y: 0.0,
            zoom: 1.0,
            shaders: HashMap::new(),
            geometry: HashMap::new(),
            geometry_mat: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.create_shaders();
        self.create_geometry();
    }

    fn create_shaders(&mut self) {
        let program = GLSLProgram::new();
        let mut stat = program.compile_shader_from_file("src/shaders/vs.glsl", gl::VERTEX_SHADER);
        stat &= program.compile_shader_from_file("src/shaders/fs.glsl", gl::FRAGMENT_SHADER);
        stat &= program.link();
        if !stat {
            println!("Some problem with shader!");
        }
        self.shaders.insert("basic".to_string(), program);
    }

    fn add_sphere(&mut self, name: &str, radius: f32, color: Vec3) {
        let geom = crate::primitives::new_sphere_geometry(radius, 32, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn create_geometry(&mut self) {
        let axes = crate::primitives::new_axes_geometry();
        self.geometry.insert("main_axes".to_string(), axes);
        self.geometry_mat.insert("main_axes".to_string(), Mat4::IDENTITY);

        self.add_sphere("sun",       0.15, Vec3::new(0.2, 1.0, 0.0));
        self.add_sphere("earth",     0.10, Vec3::new(0.0, 1.0, 0.0));
        self.add_sphere("moon",      0.05, Vec3::new(1.0, 0.0, 0.0));
        self.add_sphere("mars",      0.10, Vec3::new(1.0, 0.5, 0.0));
        self.add_sphere("mars_moon", 0.05, Vec3::new(1.0, 0.5, 0.5));
    }

    fn render_body(&self, shader: &GLSLProgram, name: &str, transform: Mat4) {
        if let Some(geom) = self.geometry.get(name) {
            let base_mat = self.geometry_mat.get(name).copied().unwrap_or(Mat4::IDENTITY);
            shader.set_uniform_mat4("MVMat", &(transform * base_mat));
            geom.render();
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.pos_x = x;
        self.pos_y = y;

        let rotation_speed = 0.01;
        self.rot_y = (x as f32 - self.width as f32 / 2.0) * rotation_speed;
        self.rot_x = (y as f32 - self.height as f32 / 2.0) * rotation_speed;
    }

    pub fn wheel(&mut self, delta_y: f32) {
        self.zoom += delta_y / 1000.0;
        self.zoom = self.zoom.clamp(0.1, 10.0);
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn paint(&mut self) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        if let Some(shader) = self.shaders.get("basic") {
            shader.use_program();

            let view = Mat4::from_scale(Vec3::splat(self.zoom))
                * Mat4::from_axis_angle(Vec3::X, self.rot_x)
                * Mat4::from_axis_angle(Vec3::Y, self.rot_y);
            shader.set_uniform_mat4("ViewMat", &view);

            let radians = (0.4 * self.frame as f32).to_radians();
            let orbit_rot = Mat4::from_axis_angle(Vec3::Z, radians);

            let tx_earth = Mat4::from_translation(Vec3::new(0.6, 0.0, 0.0));
            let tx_mars  = Mat4::from_translation(Vec3::new(-0.6, 0.0, 0.0));

            let earth_transform = orbit_rot * tx_earth * Mat4::from_axis_angle(Vec3::Z, radians * 4.0);
            let mars_transform  = orbit_rot * tx_mars  * Mat4::from_axis_angle(Vec3::Z, radians * 3.0);

            let moon_orbit  = Mat4::from_axis_angle(Vec3::Z, radians * 2.0);
            let moon_offset = Mat4::from_translation(Vec3::new(0.2, 0.0, 0.0));

            self.render_body(shader, "sun",       Mat4::IDENTITY);
            self.render_body(shader, "main_axes", earth_transform);
            self.render_body(shader, "earth",     earth_transform);
            self.render_body(shader, "moon",      orbit_rot * tx_earth * moon_orbit * moon_offset);
            self.render_body(shader, "mars",      mars_transform);
            self.render_body(shader, "mars_moon", orbit_rot * tx_mars  * moon_orbit * moon_offset);
        } else {
            println!("WARNING: No shader program");
        }

        self.frame += 1;
    }
}

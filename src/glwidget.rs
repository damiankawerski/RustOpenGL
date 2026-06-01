use crate::camera::Camera;
use crate::geometry::Geometry;
use crate::glsl::GLSLProgram;
use glam::{Mat4, Vec2, Vec3};
use std::collections::{HashMap, HashSet};
use winit::keyboard::KeyCode;
use crate::frame::Frame;

pub struct GLWidget {
    frame: u64,
    width: i32,
    height: i32,
    camera: Camera,
    keys: HashSet<KeyCode>,
    projection_mat: Mat4,
    shaders: HashMap<String, GLSLProgram>,
    geometry: HashMap<String, Geometry>,
    geometry_mat: HashMap<String, Mat4>,
    textures: HashMap<String, crate::textures_2d::Texture2d>,
}

impl GLWidget {
    pub fn new() -> Self {
        Self {
            frame: 0,
            width: 800,
            height: 600,
            camera: Camera::new(),
            keys: HashSet::new(),
            projection_mat: Mat4::perspective_rh_gl(
                60.0_f32.to_radians(),
                800.0 / 600.0,
                0.1,
                1000.0,
            ),
            shaders: HashMap::new(),
            geometry: HashMap::new(),
            geometry_mat: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.create_shaders();
        self.create_geometry();
        self.create_textures();
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

        let tex_program = GLSLProgram::new();
        let mut tex_stat = tex_program.compile_shader_from_file("src/shaders/tex_vs.glsl", gl::VERTEX_SHADER);
        tex_stat &= tex_program.compile_shader_from_file("src/shaders/tex_fs.glsl", gl::FRAGMENT_SHADER);
        tex_stat &= tex_program.link();
        if !tex_stat {
            println!("Some problem with texture shader!");
        }
        self.shaders.insert("tex_ads".to_string(), tex_program);
    }

    fn create_textures(&mut self) {
        let texture = crate::textures_2d::Texture2d::new();
        if texture.load_from_file("src/textures/2d/grass_2k.jpg") {
            println!("Loaded wood texture");
            self.textures.insert("wood".to_string(), texture);
        } else {
            println!("Failed to load wood texture");
        }
    }


    fn add_sphere(&mut self, name: &str, radius: f32, color: Vec3) {
        let geom = crate::primitives::new_sphere_geometry(radius, 32, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn add_plane(&mut self, name: &str, size: Vec2, color: Vec3) {
        let geom = crate::primitives::new_plane_geometry(size, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn add_box(&mut self, name: &str, size: Vec3, color: Vec3) {
        let geom = crate::primitives::new_box_geometry(size, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn add_cylinder(&mut self, name: &str, radius: f32, height: f32, color: Vec3) {
        let geom = crate::primitives::new_cylinder_geometry(radius, height, 32, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn add_cone(&mut self, name: &str, radius: f32, height: f32, color: Vec3) {
        let geom = crate::primitives::new_cone_geometry(radius, height, 32, color);
        self.geometry.insert(name.to_string(), geom);
        self.geometry_mat.insert(name.to_string(), Mat4::IDENTITY);
    }

    fn create_geometry(&mut self) {
        let axes = crate::primitives::new_axes_geometry();
        self.geometry.insert("main_axes".to_string(), axes);
        self.geometry_mat
            .insert("main_axes".to_string(), Mat4::IDENTITY);

        self.add_plane("plane", Vec2::new(10.0, 10.0), Vec3::new(0.5, 0.5, 0.5));

        self.add_cylinder("cylinder", 1.0, 2.0, Vec3::new(0.5, 0.5, 0.5));
        self.add_cone("cone", 1.0, 2.0, Vec3::new(0.5, 0.5, 0.5));
        self.add_box("box", Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.5, 0.5));
        self.add_sphere("sphere", 1.0, Vec3::new(1.0, 1.0, 0.0));
    }

    fn render_body(&self, shader: &GLSLProgram, name: &str, transform: Mat4) {
        if let Some(geom) = self.geometry.get(name) {
            let base_mat = self
                .geometry_mat
                .get(name)
                .copied()
                .unwrap_or(Mat4::IDENTITY);
            shader.set_uniform_mat4("MVMat", &(transform * base_mat));
            geom.render();
        }
    }

    pub fn mouse_move(&mut self, dx: f64, dy: f64) {
        let sensitivity = 0.003_f32;
        self.camera
            .rotate(-dx as f32 * sensitivity, -dy as f32 * sensitivity);
    }

    pub fn key_down(&mut self, key: KeyCode) {
        self.keys.insert(key);
    }

    pub fn key_up(&mut self, key: KeyCode) {
        self.keys.remove(&key);
    }

    fn process_camera(&mut self) {
        let speed = 0.05;
        let right = self.camera.forward.cross(self.camera.up).normalize();
        if self.keys.contains(&KeyCode::KeyW) {
            self.camera.pos += self.camera.forward * speed;
        }
        if self.keys.contains(&KeyCode::KeyS) {
            self.camera.pos -= self.camera.forward * speed;
        }
        if self.keys.contains(&KeyCode::KeyA) {
            self.camera.pos -= right * speed;
        }
        if self.keys.contains(&KeyCode::KeyD) {
            self.camera.pos += right * speed;
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.projection_mat = Mat4::perspective_rh_gl(
            60.0_f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            1000.0,
        );
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn paint(&mut self) {
        self.process_camera();
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.05, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
        }

        if let Some(shader) = self.shaders.get("basic") {
            shader.use_program();

            let view = self.camera.matrix();

            let sun_angle = self.frame as f32 * 0.01;
            let sun_radius = 5.0_f32;
            let sun_height = 4.0_f32;
            let sun_pos = Vec3::new(sun_radius * sun_angle.cos(), sun_height, sun_radius * sun_angle.sin());
            let sun_movement_frame = Frame {
                pos: sun_pos,
                forward: (sun_pos).normalize(),
                up: Vec3::Y,
            };

            shader.set_uniform_mat4("ViewMat", &view);
            shader.set_uniform_mat4("ProjectionMat", &self.projection_mat);

            shader.set_uniform_vec3("LightPos", &Vec3::ZERO);
            shader.set_uniform_vec3("LightColor", &Vec3::new(1.0, 1.0, 1.0));
            shader.set_uniform_vec3("MaterialAmbient", &Vec3::new(0.05, 0.05, 0.05));
            shader.set_uniform_vec3("MaterialDiffuse", &Vec3::new(1.0, 1.0, 1.0));
            shader.set_uniform_vec3("MaterialSpecular", &Vec3::new(0.4, 0.4, 0.4));

            let upright = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);

            // plane rendered below with textured shader
            self.render_body(shader, "main_axes", Mat4::IDENTITY);
            self.render_body(shader, "cylinder", Mat4::from_translation(Vec3::new(-2.0, 0.0, 0.0)) * upright);
            self.render_body(shader, "cone",     Mat4::from_translation(Vec3::new( 2.0, 0.0, 0.0)) * upright);
            //self.render_body(shader, "box",      Mat4::from_translation(Vec3::new( 0.0, 0.5, -2.0)));

  
            self.render_body(shader, "sphere", sun_movement_frame.matrix());
        } else {
            println!("WARNING: No shader program");
        }

        if let (Some(shader), Some(texture)) = (
            self.shaders.get("tex_ads"),
            self.textures.get("wood"),
        ) {
            shader.use_program();

            let view = self.camera.matrix();
            let plane_mat = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);

            shader.set_uniform_mat4("ViewMat", &view);
            shader.set_uniform_mat4("ProjectionMat", &self.projection_mat);
            shader.set_uniform_vec3("LightPos", &Vec3::ZERO);
            shader.set_uniform_vec3("LightColor", &Vec3::new(1.0, 1.0, 1.0));
            shader.set_uniform_vec3("MaterialAmbient", &Vec3::new(0.1, 0.1, 0.1));
            shader.set_uniform_vec3("MaterialSpecular", &Vec3::new(0.1, 0.1, 0.1));
            shader.set_uniform_int("TextureSampler", 0);

            texture.bind(0);
            self.render_body(shader, "plane", plane_mat);
        }

        self.frame += 1;
    }
}

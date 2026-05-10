use crate::geometry::Geometry;
use crate::glsl::GLSLProgram;
use std::collections::HashMap;

pub static glmIdentity: glm::Matrix4<f32> = glm::mat4(
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
);
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
    geometry_mat: HashMap<String, glm::Mat4>,
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

    fn create_geometry(&mut self) {
        let axes = crate::primitives::new_axes_geometry();
        self.geometry.insert("main_axes".to_string(), axes);
        let identity = glm::mat4(
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        );
        self.geometry_mat.insert("main_axes".to_string(), identity);

        let earth =
            crate::primitives::new_sphere_geometry(0.1, 32, glm::vec3(0.0, 1.0, 0.0));
        self.geometry.insert("earth".to_string(), earth);

        self.geometry_mat.insert("earth".to_string(), identity);

        let sun =
            crate::primitives::new_sphere_geometry(0.15, 32, glm::vec3(0.2, 1.0, 0.0));
        self.geometry.insert("sun".to_string(), sun);
        self.geometry_mat.insert("sun".to_string(), identity);

        let moon =
            crate::primitives::new_sphere_geometry(0.05, 32, glm::vec3(1.0, 0.0, 0.0));
        self.geometry.insert("moon".to_string(), moon);
        self.geometry_mat.insert("moon".to_string(), identity);

        let mars =
            crate::primitives::new_sphere_geometry(0.1, 32, glm::vec3(1.0, 0.5, 0.0));
        self.geometry.insert("mars".to_string(), mars);
        self.geometry_mat.insert("mars".to_string(), identity);

        let mars_moon =
            crate::primitives::new_sphere_geometry(0.05, 32, glm::vec3(1.0, 0.5, 0.5));
        self.geometry.insert("mars_moon".to_string(), mars_moon);
        self.geometry_mat.insert("mars_moon".to_string(), identity);
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.pos_x = x;
        self.pos_y = y;

        let rotation_speed = 0.01;
        // Środek obrotu od środka ekranu (zamiast względem 0,0)
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

            shader.set_uniform_mat4("MVMat", &glmIdentity);

            let mut view = glmIdentity;
            view = glm::ext::scale(&view, glm::vec3(self.zoom, self.zoom, self.zoom));
            view = glm::ext::rotate(&view, self.rot_x, glm::vec3(1.0, 0.0, 0.0));
            view = glm::ext::rotate(&view, self.rot_y, glm::vec3(0.0, 1.0, 0.0));

            shader.set_uniform_mat4("ViewMat", &view);

            let tx = glm::ext::translate(&glmIdentity, glm::vec3(0.6, 0.0, 0.0));
            let radians = glm::radians(0.4 * self.frame as f32);
            let rot = glm::ext::rotate(&glmIdentity, radians, glm::vec3(0.0, 0.0, 1.0));

            let rot_earth = glm::ext::rotate(&glmIdentity, radians * 4.0, glm::vec3(0.0, 0.0, 1.0));
            let earth_transform = rot * tx * rot_earth;

            let tx_mars = glm::ext::translate(&glmIdentity, glm::vec3(-0.6, 0.0, 0.0));
            let rot_mars = glm::ext::rotate(&glmIdentity, radians * 3.0, glm::vec3(0.0, 0.0, 1.0));
            let mars_transform = rot * tx_mars * rot_mars;

            if let Some(geom) = self.geometry.get("sun") {
                let base_mat = self.geometry_mat.get("sun").copied().unwrap_or(glmIdentity);
                let final_mat = glmIdentity * base_mat;
                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }

            if let Some(geom) = self.geometry.get("main_axes") {
                let base_mat = self
                    .geometry_mat
                    .get("main_axes")
                    .copied()
                    .unwrap_or(glmIdentity);
                let final_mat = glmIdentity * earth_transform * base_mat;
                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }

            if let Some(geom) = self.geometry.get("earth") {
                let base_mat = self
                    .geometry_mat
                    .get("earth")
                    .copied()
                    .unwrap_or(glmIdentity);

                let final_mat = glmIdentity * earth_transform * base_mat;

                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }

            if let Some(geom) = self.geometry.get("moon") {
                let base_mat = self
                    .geometry_mat
                    .get("moon")
                    .copied()
                    .unwrap_or(glmIdentity);

                let moon_tx = glm::ext::translate(&glmIdentity, glm::vec3(0.2, 0.0, 0.0));

                let moon_orbit_rot =
                    glm::ext::rotate(&glmIdentity, radians * 2.0, glm::vec3(0.0, 0.0, 1.0));

                let moon_transform = rot * tx * moon_orbit_rot * moon_tx;
                let final_mat = glmIdentity * moon_transform * base_mat;

                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }

            if let Some(geom) = self.geometry.get("mars") {
                let base_mat = self
                    .geometry_mat
                    .get("mars")
                    .copied()
                    .unwrap_or(glmIdentity);

                let final_mat = glmIdentity * mars_transform * base_mat;

                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }

            if let Some(geom) = self.geometry.get("mars_moon") {
                let base_mat = self
                    .geometry_mat
                    .get("mars_moon")
                    .copied()
                    .unwrap_or(glmIdentity);

                let moon_tx = glm::ext::translate(&glmIdentity, glm::vec3(0.2, 0.0, 0.0));

                let moon_orbit_rot =
                    glm::ext::rotate(&glmIdentity, radians * 2.0, glm::vec3(0.0, 0.0, 1.0));

                // Środek orbity Marsa to rot * tx_mars, nie rot * tx (które odpowiada Ziemi)
                let moon_transform = rot * tx_mars * moon_orbit_rot * moon_tx;
                let final_mat = glmIdentity * moon_transform * base_mat;

                shader.set_uniform_mat4("MVMat", &final_mat);
                geom.render();
            }
        } else {
            println!("WARNING: No shader program");
        }

        self.frame += 1;
    }
}

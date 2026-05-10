use glam::Vec3;
use std::collections::HashMap;

pub struct Geometry {
    vao: u32,
    primitive_mode: u32,
    n_verts: i32,
    n_indices: i32,
    ibo: u32,
    buffer_objects: HashMap<u32, u32>,
}

impl Geometry {
    pub fn new() -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self {
            vao,
            primitive_mode: gl::TRIANGLES,
            n_verts: 0,
            n_indices: 0,
            ibo: 0,
            buffer_objects: HashMap::new(),
        }
    }

    pub fn set_primitive_mode(&mut self, mode: u32) {
        self.primitive_mode = mode;
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            if self.ibo != 0 {
                gl::DrawElements(
                    self.primitive_mode,
                    self.n_indices,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            } else {
                gl::DrawArrays(self.primitive_mode, 0, self.n_verts);
            }

            gl::BindVertexArray(0);
        }
    }

    pub fn set_indices(&mut self, data: &[u32]) {
        self.n_indices = data.len() as i32;
        unsafe {
            if self.ibo == 0 {
                gl::GenBuffers(1, &mut self.ibo);
            }
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::BindVertexArray(0);
        }
    }

    pub fn set_vertices(&mut self, index: u32, data: &[Vec3]) {
        self.n_verts = data.len() as i32;
        self.set_attribute(index, data);
    }

    pub fn set_attribute(&mut self, index: u32, data: &[Vec3]) {
        let mut buffer = 0;
        if let Some(&b) = self.buffer_objects.get(&index) {
            buffer = b;
        } else {
            unsafe {
                gl::GenBuffers(1, &mut buffer);
            }
            self.buffer_objects.insert(index, buffer);
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let components = std::mem::size_of::<Vec3>() / std::mem::size_of::<f32>();
            gl::VertexAttribPointer(
                index,
                components as i32,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(index);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        unsafe {
            if self.ibo != 0 {
                gl::DeleteBuffers(1, &self.ibo);
            }

            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }

            for &b in self.buffer_objects.values() {
                gl::DeleteBuffers(1, &b);
            }
        }
    }
}

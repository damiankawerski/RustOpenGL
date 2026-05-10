use glam::{Mat4, Vec3};
use std::ffi::CString;
use std::fs;

pub struct GLSLProgram {
    handle: u32,
}

impl GLSLProgram {
    pub fn new() -> Self {
        let handle = unsafe { gl::CreateProgram() };
        Self { handle }
    }

    pub fn compile_shader_from_file(&self, filename: &str, shader_type: u32) -> bool {
        if let Ok(source) = fs::read_to_string(filename) {
            println!("Compiling shader from file: {}", filename);
            println!("Shader source:\n{}", source);
            self.compile_shader_from_string(&source, shader_type)
        } else {
            false
        }
    }

    pub fn compile_shader_from_string(&self, source: &str, shader_type: u32) -> bool {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        if self.check_shader_status(shader) {
            unsafe {
                gl::AttachShader(self.handle, shader);
                gl::DeleteShader(shader);
            }
            return true;
        }

        unsafe {
            gl::DeleteShader(shader);
        }
        false
    }

    pub fn link(&self) -> bool {
        unsafe {
            gl::LinkProgram(self.handle);
        }
        self.check_program_status()
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, value: &Vec3) {
        let c_name = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.handle, c_name.as_ptr()) };
        if loc != -1 {
            let arr = value.to_array();
            unsafe {
                gl::Uniform3fv(loc, 1, arr.as_ptr());
            }
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &Mat4) {
        let c_name = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.handle, c_name.as_ptr()) };
        if loc != -1 {
            let arr = value.to_cols_array();
            unsafe {
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, arr.as_ptr());
            }
        }
    }

    fn check_shader_status(&self, shader: u32) -> bool {
        let mut status = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        }
        status == gl::TRUE as i32
    }

    fn check_program_status(&self) -> bool {
        let mut status = 0;
        unsafe {
            gl::GetProgramiv(self.handle, gl::LINK_STATUS, &mut status);
        }
        status == gl::TRUE as i32
    }
}

impl Drop for GLSLProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

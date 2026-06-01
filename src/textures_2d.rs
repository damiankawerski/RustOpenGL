use gl::types::GLuint;
use image::ImageReader;

pub struct Texture2d {
    handle: GLuint,
}

impl Texture2d {
    pub fn new() -> Self {
        let mut handle = 0;
        unsafe { gl::GenTextures(1, &mut handle) };
        Self { handle }
    }

    pub fn load_from_file(&self, path: &str) -> bool {
        let img = match ImageReader::open(path).map_err(|e| e.to_string())
            .and_then(|r| r.decode().map_err(|e| e.to_string()))
        {
            Ok(img) => img.into_rgba8(),
            Err(e) => {
                eprintln!("Failed to load texture '{}': {}", path, e);
                return false;
            }
        };

        let (width, height) = img.dimensions();
        let pixels = img.into_raw();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        true
    }

    pub fn bind(&self, tex_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.handle) };
    }
}

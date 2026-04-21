use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SwapInterval;
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowAttributes;

use std::ffi::CString;
use std::num::NonZeroU32;

struct Window {
    shader_program: u32,
    vao: u32,
    vbo: u32,
    color_vbo: u32,
}

#[allow(non_snake_case)]
impl Window {
    fn new() -> Self {
        Self {
            shader_program: 0,
            vao: 0,
            vbo: 0,
            color_vbo: 0,
        }
    }

    unsafe fn initializeGL(&mut self) {
        let vertex_shader_src = r#"
            #version 330 core
            layout (location = 0) in vec3 pos;
            layout (location = 1) in vec4 color;
            out vec4 vColor;
            void main() {
                gl_Position = vec4(pos,  1.0);
                vColor = color;
            }
        "#;

        let fragment_shader_src = r#"
            #version 330 core
            in vec4 vColor;
            out vec4 FragColor;
            void main() {
                FragColor = vColor;
            }
        "#;

        self.shader_program = unsafe { create_program(vertex_shader_src, fragment_shader_src) };

        let vertices: [f32; 9] = [
            0.0, 0.0, 0.0, // Wierzchołek 1
            0.0, 1.0, 0.0, // Wierzchołek 2
            1.0, 0.0, 0.0, // Wierzchołek 3
        ];

        let colors = [
            1.0, 0.0, 0.0, 1.0, // Czerwony
            0.0, 1.0, 0.0, 1.0, // Zielony
            0.0, 0.0, 1.0, 1.0, // Niebieski
        ]; 

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.color_vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (3 * std::mem::size_of::<f32>()) as i32;

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (colors.len() * std::mem::size_of::<f32>()) as isize,
                colors.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let color_stride = (4 * std::mem::size_of::<f32>()) as i32;
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, color_stride, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    fn resizeGL(&self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    fn paintGL(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.12, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = WindowAttributes::default()
        .with_title("OpenGL Triangle")
        .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32));

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .max_by_key(|config| config.num_samples())
                .expect("No GL configs found")
        })
        .unwrap();

    let winit_window = window.unwrap();
    let raw_window_handle = winit_window.window_handle().unwrap().as_raw();
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(Some(raw_window_handle));

    let gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    };

    let attrs = winit_window
        .build_surface_attributes(Default::default())
        .unwrap();
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = gl_context.make_current(&gl_surface).unwrap();

    gl::load_with(|s| {
        let s = CString::new(s).unwrap();
        gl_display.get_proc_address(&s) as *const _
    });

    gl_surface
        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .unwrap();

    let mut window = Window::new();
    unsafe {
        window.initializeGL();
    }

    let size = winit_window.inner_size();
    window.resizeGL(size.width, size.height);

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        if size.width > 0 && size.height > 0 {
                            gl_surface.resize(
                                &gl_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                            window.resizeGL(size.width, size.height);
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.paintGL();
                    gl_surface.swap_buffers(&gl_context).unwrap();
                }
                _ => {}
            }
        })
        .unwrap();
}

unsafe fn create_shader(src: &str, kind: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(kind);
        let c_src = CString::new(src).unwrap();
        gl::ShaderSource(shader, 1, &c_src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        let mut ok = 0i32;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);

        if ok == 0 {
            let mut len = 0i32;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // Skip null terminator
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut _,
            );
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buf)
            );
        }

        shader
    }
}

unsafe fn create_program(vert_src: &str, frag_src: &str) -> u32 {
    unsafe {
        let vert = create_shader(vert_src, gl::VERTEX_SHADER);
        let frag = create_shader(frag_src, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);
        gl::LinkProgram(program);

        let mut ok = 0i32;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut ok);

        if ok == 0 {
            let mut len = 0i32;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // Skip null terminator
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut _,
            );
            panic!("Program linking failed: {}", String::from_utf8_lossy(&buf));
        }

        gl::DeleteShader(vert);
        gl::DeleteShader(frag);

        program
    }
}

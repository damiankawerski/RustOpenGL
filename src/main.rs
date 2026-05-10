mod geometry;
mod glsl;
mod glwidget;
mod primitives;

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

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = WindowAttributes::default()
        .with_title("GIPO Lab 2")
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

    let mut gl_widget = glwidget::GLWidget::new();
    gl_widget.initialize();

    let size = winit_window.inner_size();
    gl_widget.resize(size.width as i32, size.height as i32);

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
                            gl_widget.resize(size.width as i32, size.height as i32);
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        gl_widget.mouse_move(position.x as i32, position.y as i32);
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                            gl_widget.wheel(y * 100.0); // Simple scaling for lines
                        }
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            gl_widget.wheel(pos.y as f32);
                        }
                    },
                    _ => {}
                },
                Event::AboutToWait => {
                    gl_widget.paint();
                    gl_surface.swap_buffers(&gl_context).unwrap();
                }
                _ => {}
            }
        })
        .unwrap();
}

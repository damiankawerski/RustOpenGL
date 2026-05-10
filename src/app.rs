use glutin::context::{PossiblyCurrentContext};
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};
use crate::glwidget;
use std::num::NonZeroU32;

pub struct App {
    pub window: Window,
    pub gl_context: PossiblyCurrentContext,
    pub gl_surface: Surface<WindowSurface>,
    pub gl_widget: glwidget::GLWidget,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                self.gl_surface.resize(
                    &self.gl_context,
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                );
                self.gl_widget.resize(size.width as i32, size.height as i32);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.gl_widget.mouse_move(position.x as i32, position.y as i32);
            }
            WindowEvent::RedrawRequested => {
                self.gl_widget.paint();
                self.gl_surface.swap_buffers(&self.gl_context).unwrap();
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                    self.gl_widget.wheel(y * 100.0);
                }
                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                    self.gl_widget.wheel(pos.y as f32);
                }
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.request_redraw();
    }
}


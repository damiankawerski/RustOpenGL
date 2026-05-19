use crate::glwidget;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};

pub struct App {
    pub window: Window,
    pub gl_context: PossiblyCurrentContext,
    pub gl_surface: Surface<WindowSurface>,
    pub gl_widget: glwidget::GLWidget,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = self
            .window
            .set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| self.window.set_cursor_grab(CursorGrabMode::Confined));
        self.window.set_cursor_visible(false);
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            self.gl_widget.mouse_move(dx, dy);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.gl_widget.paint();
                self.gl_surface.swap_buffers(&self.gl_context).unwrap();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if key == KeyCode::Escape && event.state == winit::event::ElementState::Pressed
                    {
                        let _ = self.window.set_cursor_grab(CursorGrabMode::None);
                        self.window.set_cursor_visible(true);
                    }
                    match event.state {
                        winit::event::ElementState::Pressed => self.gl_widget.key_down(key),
                        winit::event::ElementState::Released => self.gl_widget.key_up(key),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.request_redraw();
    }
}

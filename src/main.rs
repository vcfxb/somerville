//! The Somerville application binary.

use winit::{application::ApplicationHandler, window::{Window, WindowAttributes}};

fn main() -> anyhow::Result<()> {
    let ev_loop = winit::event_loop::EventLoop::new()?;

    struct App;

    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            event_loop.create_window(Window::default_attributes()).unwrap();
        }
    
        fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
            loop {}
        }
    }

    ev_loop.run_app(&mut App)?;

    Ok(())
}

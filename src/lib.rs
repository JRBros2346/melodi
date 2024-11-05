pub use tracing::{self, debug, error, info, trace, warn};
pub use winit;

#[derive(Default)]
pub struct PlatformState {
    window: Option<winit::window::Window>,
}

impl winit::application::ApplicationHandler for PlatformState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(winit::window::Window::default_attributes())
                .unwrap(),
        )
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Destroyed => {}
            winit::event::WindowEvent::Resized(size) => {}
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {}
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => {}
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {}
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {}
            _ => (),
        }
    }
}

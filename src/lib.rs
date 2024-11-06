pub use tracing::{self, debug, error, info, trace, warn};
pub use winit;
use winit::dpi::{Position, Size};

pub struct PlatformState {
    window: Option<winit::window::Window>,
    title: String,
    position: Position,
    size: Size,
}

impl PlatformState {
    pub fn new<T, P, S>(title: T, position: P, size: S) -> Self
    where
        T: Into<String>,
        P: Into<Position>,
        S: Into<Size>,
    {
        Self {
            window: None,
            title: title.into(),
            position: position.into(),
            size: size.into(),
        }
    }
}

impl winit::application::ApplicationHandler for PlatformState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title(&self.title)
                        .with_position(self.position)
                        .with_inner_size(self.size),
                )
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
            winit::event::WindowEvent::RedrawRequested => {}
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Destroyed => {}
            winit::event::WindowEvent::Resized(size) => {
                // todo!("window resize");
            }
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                // todo!("input processing");
            }
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                // todo!("input processing");
            }
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                // todo!("input processing")
            }
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                // todo!("input processing");
            }
            _ => (),
        }
    }
}

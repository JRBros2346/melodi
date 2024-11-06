use super::*;
use winit::{
    application::ApplicationHandler,
    dpi::{Position, Size},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct AppConfig {
    pub title: String,
    pub position: Position,
    pub size: Size,
}

pub struct App {
    window: Option<Window>,
    title: String,
    position: Position,
    size: Size,
    last_time: f64,
}

impl App {
    pub fn with_config(app_config: AppConfig) -> Self {
        error!("ERROR");
        warn!("WARN");
        info!("INFO");
        debug!("DEBUG");
        trace!("TRACE");
        Self {
            window: None,
            title: app_config.title,
            position: app_config.position,
            size: app_config.size,
            last_time: 0.,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_position(self.position)
                        .with_inner_size(self.size),
                )
                .unwrap(),
        )
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {}
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Destroyed => {}
            WindowEvent::Resized(size) => {
                let _ = size;
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let _ = (device_id, event, is_synthetic);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let _ = (device_id, position);
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                let _ = (device_id, delta, phase);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let _ = (device_id, state, button);
            }
            _ => (),
        }
    }
}

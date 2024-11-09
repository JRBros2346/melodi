use super::*;
use winit::{
    application::ApplicationHandler,
    dpi::{Position, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
};

pub struct Config {
    pub title: String,
    pub position: Position,
    pub size: Size,
}

pub struct Strings<G> {
    suspended: bool,
    window: Option<Window>,
    last_time: f64,
    game: G,
}

impl<G: GameState> Strings<G> {
    pub fn with_game(mut game: G) -> Result<Self, Box<dyn std::error::Error>> {
        error!("ERROR");
        warn!("WARN");
        info!("INFO");
        debug!("DEBUG");
        trace!("TRACE");

        game.resize(game.app_config().size)?;

        Ok(Self {
            suspended: false,
            window: None,
            last_time: 0.,
            game,
        })
    }
}

impl<G: GameState> ApplicationHandler for Strings<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        self.suspended = false;
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.game.app_config().title)
                        .with_position(self.game.app_config().position)
                        .with_inner_size(self.game.app_config().size),
                )
                .unwrap(),
        )
    }
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.suspended = true;
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                if !self.suspended {
                    self.game.update(0.).unwrap_or_else(|e| {
                        error!("Game Update Failed.. {e}");
                        event_loop.exit();
                    });
                    self.game.render(0.).unwrap_or_else(|e| {
                        error!("Game Render Failed.. {e}");
                        event_loop.exit();
                    });
                }
            }
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

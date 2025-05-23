use std::time::Instant;

use super::*;
use winit::{
    application::ApplicationHandler,
    dpi::{Position, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
};

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub position: Position,
    pub size: Size,
}

pub struct Strings<G> {
    suspended: bool,
    window: Option<Window>,
    _last_time: Option<Instant>,
    game: G,
}

impl<G: GameState> Strings<G> {
    /// Get a reference to the application's configuration.
    ///
    /// This method returns a reference to the `Config` struct which contains the title,
    /// position, and size of the window.
    ///
    /// The config is owned by the game and therefore this method returns a reference to
    /// the game's config.
    pub fn config(&self) -> &Config {
        self.game.app_config()
    }
    /// Create a new `Strings` instance with the given game.
    ///
    /// The game will be initialized with the `init` method and the window will be created
    /// with the `create_window` method.
    ///
    /// The event loop will be set to poll mode and the `Strings` instance will take ownership
    /// of the game.
    ///
    /// The method will return an `Err` if the game fails to initialize or the window fails
    /// to create.
    pub fn with_game(mut game: G) -> Result<Self, Box<dyn std::error::Error>> {
        // Log the event levels to the console
        error!("ERROR");
        warn!("WARN");
        info!("INFO");
        debug!("DEBUG");
        trace!("TRACE");

        // Initialize the game
        game.resize(game.app_config().size)?;

        // Create the window
        Ok(Self {
            suspended: false,
            window: None,
            _last_time: None,
            game,
        })
    }
}

impl<G, U> ApplicationHandler<GameEvent<U>> for Strings<G>
where
    G: GameState,
    U: 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug!("Resuming application");
        event_loop.set_control_flow(ControlFlow::Poll);
        self.suspended = false;
        debug!("Creating window with title: {}", self.game.app_config().title);
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.game.app_config().title)
                        .with_position(self.game.app_config().position)
                        .with_inner_size(self.game.app_config().size),
                )
                .unwrap_or_else(|e| {
                    error!("Failed to create window: {e}");
                    event_loop.exit();
                    panic!(); // Ensure the program exits
                }),
        );
        info!("Window created successfully");
    }
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        debug!("Suspending application");
        let _ = event_loop;
        self.suspended = true;
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let _ = window_id;
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
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GameEvent<U>) {
        let _ = (event_loop, event);
    }
}

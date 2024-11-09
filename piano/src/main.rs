use std::convert::Infallible;

use strings::*;
use tracing::Level;

struct Game {
    app_config: Config,
}

impl GameState for Game {
    type GameError = Infallible;
    fn app_config(&self) -> &Config {
        &self.app_config
    }

    fn init() -> Result<Self, Self::GameError> {
        debug!("Game Initialized");
        Ok(Self {
            app_config: Config {
                title: "Piano".into(),
                position: PhysicalPosition { x: 100, y: 100 }.into(),
                size: PhysicalSize {
                    width: 1280,
                    height: 720,
                }
                .into(),
            },
        })
    }

    fn update(&mut self, delta_time: f64) -> Result<(), Self::GameError> {
        let _ = delta_time;
        Ok(())
    }

    fn render(&mut self, delta_time: f64) -> Result<(), Self::GameError> {
        let _ = delta_time;
        Ok(())
    }

    fn resize<S: Into<Size>>(&mut self, size: S) -> Result<(), Self::GameError> {
        let _ = size;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .compact()
            .finish(),
    )?;

    let mut app = Strings::with_game(Game::init()?)?;
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}

use std::convert::Infallible;

use melodi::*;
use tracing::Level;
use winit::event_loop::EventLoop;

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
                title: "Harmoni".into(),
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

    let mut app = Melodi::with_game(Game::init()?)?;
    let event_loop = EventLoop::<GameEvent<()>>::with_user_event().build()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}

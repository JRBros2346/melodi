use std::sync::atomic::{AtomicBool, Ordering};

use crate::core::{event, input, log};
use crate::game::{Error as GameError, Game};
use crate::platform::{Error as PlatformError, PlatformState};

// Application configuration.
pub struct AppConfig {
    // Window starting position x axis, if applicable.
    pub x: i16,

    // Window starting position y axis, if applicable.
    pub y: i16,

    // Window starting width, if applicable.
    pub width: u16,

    // Window starting height, if applicable.
    pub height: u16,

    // The application name used in windowing, if applicable.
    pub name: String,
}

pub struct App {
    game: Box<dyn Game>,
    running: bool,
    suspended: bool,
    platform: PlatformState,
    width: u16,
    height: u16,
    _last_time: f64,
}

static INIT: AtomicBool = AtomicBool::new(false);

impl App {
    pub fn create(game: Box<dyn Game>, app_config: AppConfig) -> Result<Self> {
        if INIT.load(Ordering::Relaxed) {
            crate::error!("`App::create()` called more than once.");
            return Err(Error::MultipleCreateError);
        }

        // Initialize subsystems.
        let _ = log::init();
        input::init();

        // TODO: Remove this.
        crate::fatal!("The value is: {}", std::f64::consts::PI);
        crate::error!("The value is: {}", std::f64::consts::PI);
        crate::warn!("The value is: {}", std::f64::consts::PI);
        crate::info!("The value is: {}", std::f64::consts::PI);
        crate::debug!("The value is: {}", std::f64::consts::PI);
        crate::trace!("The value is: {}", std::f64::consts::PI);

        let mut out = Self {
            game,
            running: true,
            suspended: false,
            platform: PlatformState::startup(
                &app_config.name,
                app_config.x,
                app_config.y,
                app_config.width,
                app_config.height,
            )
            .map_err(Error::Platform)?,
            width: app_config.width,
            height: app_config.height,
            _last_time: 0.0,
        };

        if let Err(event::Error) = event::init() {
            crate::error!("Event Syatem Failed Initialization. App cannot continue");
            return Err(Error::MultipleCreateError);
        }

        if let Err(e) = out.game.initialize() {
            crate::fatal!("Game failed to initialize.");
            return Err(Error::Game(e));
        }

        out.game.on_resize(out.width, out.height);

        INIT.store(true, Ordering::Relaxed);

        Ok(out)
    }
    pub fn run(mut self) -> Result<()> {
        let mut res = Ok(());
        crate::info!("{}", super::mem::get_memory_usage());
        while self.running {
            if !self.platform.pump_messages().map_err(Error::Platform)? {
                self.running = false;
            }
            if !self.suspended {
                if let Err(e) = self.game.update(0.) {
                    crate::fatal!("Game::update() failed, shutting down.");
                    self.running = false;
                    res = Err(Error::Game(e));
                    break;
                }
                if let Err(e) = self.game.render(0.) {
                    crate::fatal!("Game::render() failed, shutting down.");
                    self.running = false;
                    res = Err(Error::Game(e));
                    break;
                }

                // NOTE: Input update/state copying should always be handled
                // after any input should be recorded; I.E. before this line.
                // As a safety, input is the last thing to be updated before
                // this frame ends.
                input::update(0.);
            }
        }

        self.running = false;

        event::close();
        input::close();

        self.platform.shutdown().map_err(Error::Platform)?;

        crate::core::log::close();

        res
    }
}

type Result<T> = std::result::Result<T, Error>;
pub enum Error {
    MultipleCreateError,
    Platform(PlatformError),
    Game(GameError),
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultipleCreateError => write!(f, "MultipleCreateError"),
            Self::Platform(e) => write!(f, "{:?}", e),
            Self::Game(e) => write!(f, "{:?}", e),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultipleCreateError => write!(f, "Attempt to create multiple `App` instances"),
            Self::Platform(e) => write!(f, "{}", e),
            Self::Game(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MultipleCreateError => None,
            Self::Platform(e) => Some(e),
            Self::Game(e) => Some(e),
        }
    }
}

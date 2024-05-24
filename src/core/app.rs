use std::sync::atomic::{AtomicBool, Ordering};

use crate::game::*;
use crate::platform::*;
use crate::core::event;

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
    pub fn create(game: Box<dyn Game>, app_config: AppConfig) -> Result<Self, AppCreateError> {
        if INIT.load(Ordering::Relaxed) {
            crate::error!("`App::create()` called more than once.");
            return Err(AppCreateError::MultipleCreateError);
        }

        // Initialize subsystems.
        let _ = crate::core::log::init();

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
            .map_err(AppCreateError::Platform)?,
            width: app_config.width,
            height: app_config.height,
            _last_time: 0.0,
        };

        if !event::init() {
            crate::error!("Event Syatem Failed Initialization. App cannot continue");
            return Err(AppCreateError::MultipleCreateError);
        }

        if let Err(e) = out.game.initialize() {
            crate::fatal!("Game failed to initialize.");
            return Err(AppCreateError::Game(e));
        }

        out.game.on_resize(out.width, out.height);

        INIT.store(true, Ordering::Relaxed);

        Ok(out)
    }
    pub fn run(mut self) -> Result<(), AppRunError> {
        let mut res = Ok(());
        crate::info!("{}", super::mem::get_memory_usage());
        while self.running {
            if !self
                .platform
                .pump_messages()
                .map_err(AppRunError::Platform)?
            {
                self.running = false;
            }
            if !self.suspended {
                if let Err(e) = self.game.update(0.) {
                    crate::fatal!("Game::update() failed, shutting down.");
                    self.running = false;
                    res = Err(AppRunError::Game(e));
                    break;
                }
                if let Err(e) = self.game.render(0.) {
                    crate::fatal!("Game::render() failed, shutting down.");
                    self.running = false;
                    res = Err(AppRunError::Game(e));
                    break;
                }
            }
        }

        self.running = false;

        event::close();

        self.platform.shutdown().map_err(AppRunError::Platform)?;

        crate::core::log::close();

        res
    }
}

pub enum AppCreateError {
    MultipleCreateError,
    Platform(PlatformError),
    Game(GameError),
}
impl std::fmt::Debug for AppCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultipleCreateError => write!(f, "MultipleCreateError"),
            Self::Platform(e) => write!(f, "{:?}", e),
            Self::Game(e) => write!(f, "{:?}", e),
        }
    }
}
impl std::fmt::Display for AppCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultipleCreateError => write!(f, "Attempt to create multiple `App` instances"),
            Self::Platform(e) => write!(f, "{}", e),
            Self::Game(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for AppCreateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MultipleCreateError => None,
            Self::Platform(e) => Some(e),
            Self::Game(e) => Some(e),
        }
    }
}

pub enum AppRunError {
    Platform(PlatformError),
    Game(GameError),
}
impl std::fmt::Debug for AppRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Platform(e) => write!(f, "{:?}", e),
            Self::Game(e) => write!(f, "{:?}", e),
        }
    }
}
impl std::fmt::Display for AppRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Platform(e) => write!(f, "{}", e),
            Self::Game(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for AppRunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Platform(e) => Some(e),
            Self::Game(e) => Some(e),
        }
    }
}

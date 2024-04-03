use std::sync::atomic::AtomicBool;

use crate::game::*;
use crate::platform::*;

// Application configuration.
pub struct AppConfig {
    // Window starting position x axis, if applicable.
    pub x: i32,

    // Window starting position y axis, if applicable.
    pub y: i32,

    // Window starting width, if applicable.
    pub width: u32,

    // Window starting height, if applicable.
    pub height: u32,

    // The application name used in windowing, if applicable.
    pub name: String,
}

pub struct App {
    game: Box<dyn Game>,
    running: bool,
    suspended: bool,
    platform: PlatformState,
    width: u32,
    height: u32,
    last_time: f64,
}

#[allow(non_upper_case_globals)]
static initialized: AtomicBool = AtomicBool::new(false);

impl App {
    pub fn create(game: Box<dyn Game>, app_config: AppConfig) -> Result<Self, String> {
        if initialized.load(std::sync::atomic::Ordering::Relaxed) {
            crate::error!("`App::create()` called more than once.");
            return Err(String::new());
        }

        // Initialize subsystems.
        crate::core::log::init();

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
            )?,
            width: app_config.width,
            height: app_config.height,
            last_time: 0.0,
        };

        if let Err(e) = out.game.initialize() {
            crate::fatal!("Game failed to initialize.");
            return Err(e);
        }

        out.game.on_resize(out.width, out.height);

        initialized.store(true, std::sync::atomic::Ordering::Relaxed);

        Ok(out)
    }
    pub fn run(mut self) -> Result<(), String> {
        let mut res = Ok(());
        while self.running {
            if !self.platform.pump_messages()? {
                self.running = false;
            }
            if let Err(e) = self.game.update(0.) {
                crate::fatal!("Game::update() failed, shutting down.");
                self.running = false;
                res = Err(e);
                break;
            }
            if let Err(e) = self.game.render(0.) {
                crate::fatal!("Game::render() failed, shutting down.");
                self.running = false;
                res = Err(e);
                break;
            }
        }

        self.running = false;

        self.platform.shutdown()?;

        res
    }
}

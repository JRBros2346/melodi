use std::sync::atomic::AtomicBool;

use crate::platform::*;

#[repr(C)]
pub struct AppConfig<'a> {
    pub start_x: i32,
    pub start_y: i32,
    pub start_width: i32,
    pub start_height: i32,
    pub name: &'a str,
}

pub struct App {
    running: bool,
    suspended: bool,
    platform: PlatformState,
    width: i32,
    height: i32,
    last_time: f64,
}

static initialized: AtomicBool = AtomicBool::new(false);

impl App {
    pub fn create(app_config: &AppConfig) -> Result<Self, String> {
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

        let out = Self {
            running: true,
            suspended: false,
            platform: PlatformState::startup(
                app_config.name,
                app_config.start_x,
                app_config.start_y,
                app_config.start_width,
                app_config.start_height,
            )?,
            width: app_config.start_width,
            height: app_config.start_height,
            last_time: 0.0,
        };

        initialized.store(true, std::sync::atomic::Ordering::Relaxed);

        Ok(out)
    }
    pub fn run(mut self) -> Result<(), String> {
        while self.running {
            if !self.platform.pump_messages()? {
                self.running = false;
            }
        }

        self.running = false;

        self.platform.shutdown()?;

        Ok(())
    }
}

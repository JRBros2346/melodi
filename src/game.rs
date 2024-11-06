use winit::dpi::Size;

use crate::Config;

pub trait GameState: Sized {
    type GameError: std::error::Error + 'static;
    fn app_config(&self) -> &Config;
    fn init() -> Result<Self, Self::GameError>;
    fn update(&mut self, delta_time: f64) -> Result<(), Self::GameError>;
    fn render(&mut self, delta_time: f64) -> Result<(), Self::GameError>;
    fn resize<S: Into<Size>>(&mut self, size: S) -> Result<(), Self::GameError>;
}

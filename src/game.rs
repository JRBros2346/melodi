pub trait Game {
    // The game's initialize function.
    fn initialize(&mut self) -> Result<()>;
    // The game's update function.
    fn update(&mut self, delta_time: f64) -> Result<()>;
    // The game's render function.
    fn render(&self, delta_time: f64) -> Result<()>;
    // To handle resizes, if applicable.
    fn on_resize(&mut self, width: u16, height: u16);
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameError")
    }
}
impl std::error::Error for Error {}

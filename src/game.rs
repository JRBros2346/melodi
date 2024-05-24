pub trait Game {
    // The game's initialize function.
    fn initialize(&mut self) -> Result<(), GameError>;
    // The game's update function.
    fn update(&mut self, delta_time: f64) -> Result<(), GameError>;
    // The game's render function.
    fn render(&self, delta_time: f64) -> Result<(), GameError>;
    // To handle resizes, if applicable.
    fn on_resize(&mut self, width: u16, height: u16);
}

#[derive(Debug)]
pub enum GameError {}
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameError")
    }
}
impl std::error::Error for GameError {}

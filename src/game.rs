pub trait Game {
    // The game's initialize function.
    fn initialize(&mut self) -> Result<(), String>;
    // The game's update function.
    fn update(&mut self, delta_time: f64) -> Result<(), String>;
    // The game's render function.
    fn render(&self, delta_time: f64) -> Result<(), String>;
    // To handle resizes, if applicable.
    fn on_resize(&mut self, width: u32, height: u32);
}

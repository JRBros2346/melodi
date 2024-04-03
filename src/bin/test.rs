struct MyGame;
impl strings::game::Game for MyGame {
    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn update(&mut self, delta_time: f64) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, delta_time: f64) -> Result<(), String> {
        Ok(())
    }
    fn on_resize(&mut self, width: u32, height: u32) {}
}
#[strings::create_game]
fn create() -> Result<Box<dyn strings::game::Game>, String> {
    Ok(Box::new(MyGame))
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     App::create(&AppConfig {
//         start_x: 100,
//         start_y: 100,
//         start_width: 1280,
//         start_height: 720,
//         name: "Kohi Engine Testbed",
//     })?
//     .run()?;

//     Ok(())
// }

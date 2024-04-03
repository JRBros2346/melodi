use strings::game::Game;
use strings::core::app::AppConfig;

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
fn create() -> Result<(Box<dyn Game>, AppConfig), String> {
    Ok((Box::new(MyGame), AppConfig {
        x: 100,
        y: 100,
        width: 1280,
        height: 720,
        name: "Strings Engine Testbed".to_string(),
    }))
}

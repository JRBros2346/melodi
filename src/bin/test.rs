use strings::core::app::*;
use strings::game::*;

struct MyGame;
impl strings::game::Game for MyGame {
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    fn update(&mut self, _delta_time: f64) -> Result<()> {
        Ok(())
    }
    fn render(&self, _delta_time: f64) -> Result<()> {
        Ok(())
    }
    fn on_resize(&mut self, _width: u16, _height: u16) {}
}

strings::create_game! {
    {
        Ok((
            Box::new(MyGame),
            AppConfig {
                x: 100,
                y: 100,
                width: 1280,
                height: 720,
                name: "Strings Engine Testbed".to_string(),
            }
        ))
    }
}

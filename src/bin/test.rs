use strings::core::app::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::create(&AppConfig {
        start_x: 100,
        start_y: 100,
        start_width: 1280,
        start_height: 720,
        name: "Kohi Engine Testbed",
    })?
    .run()?;

    Ok(())
}

use strings::platform::*;
use strings::*;
fn main() {
    fatal!("The value is: {}", std::f64::consts::PI);
    error!("The value is: {}", std::f64::consts::PI);
    warn!("The value is: {}", std::f64::consts::PI);
    info!("The value is: {}", std::f64::consts::PI);
    debug!("The value is: {}", std::f64::consts::PI);
    trace!("The value is: {}", std::f64::consts::PI);

    if let Ok(mut platform_state) =
        PlatformState::startup("Strings Engine Testbed", 100, 100, 1280, 720)
    {
        for _ in 0..100 {
            platform_state.pump_messages();
            std::thread::sleep(std::time::Duration::from_millis(100))
        }
        platform_state.shutdown().unwrap();
    }
}

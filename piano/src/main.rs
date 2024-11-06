use strings::*;
use winit::dpi::{PhysicalPosition, PhysicalSize};

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut platform_state = PlatformState::new(
        "Piano",
        PhysicalPosition { x: 100, y: 100 },
        PhysicalSize {
            width: 1280,
            height: 720,
        },
    );
    event_loop.run_app(&mut platform_state).unwrap();
}

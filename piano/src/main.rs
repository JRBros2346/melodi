use strings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::with_config(AppConfig {
        title: "Piano".into(),
        position: PhysicalPosition { x: 100, y: 100 }.into(),
        size: PhysicalSize {
            width: 1280,
            height: 720,
        }
        .into(),
    });
    event_loop.run_app(&mut app)?;
    Ok(())
}

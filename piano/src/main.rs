use strings::*;
use tracing::{debug, error, info, trace, warn, Level};

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .unwrap();
    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");
}

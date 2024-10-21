use strings::*;

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::new(),
    )
    .unwrap();
    error!("ERROR");
    warn!("WARN");
    info!("INFO");
    debug!("DEBUG");
    trace!("TRACE");
}

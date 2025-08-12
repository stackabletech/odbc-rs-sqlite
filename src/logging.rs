use std::sync::Once;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        println!("INIT LOGGING");
        tracing_subscriber::registry()
            .with(fmt::layer().compact())
            .with(EnvFilter::from_default_env())
            .init();
    });
}

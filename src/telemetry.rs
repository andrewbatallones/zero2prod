use tracing::{dispatcher::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: &str,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    // This is a higher-ranked trait bound (HRTB)
    // Means that Sink implements the `MakeWriter` trait
    // for all choices of the lifetime parameter
    // https://doc.rust-lang.org/nomicon/hrtb.html
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // This will make use of the log crate trait to start outputting log information.
    // This will be replaced with the tracing logic
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Sets the level of logging (default is "info")
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // This sets how the logger will be formatted?
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // Redirect all log's events to the subscriber
    LogTracer::init().expect("failed to set logger");

    // This will tell the app what the attach the span to.
    set_global_default(subscriber.into()).expect("Failed to set subscriber");
}

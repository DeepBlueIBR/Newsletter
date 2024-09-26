use std::net::TcpListener;
use zero2prod::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{ get_subscriber, init_subscriber};
use sqlx::PgPool;
use tracing::{ Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{ BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{ layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;



#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout
    ); 

    //set_global_default(subscriber).expect("Failed to set subscriber");
    // Panic if we can't read configuration.
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Now the hard-coded '8000' it's removed, and now it originates from our settings.
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())

}

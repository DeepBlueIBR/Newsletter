use std::net::TcpListener;
use zero2prod::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{ get_subscriber, init_subscriber};
use sqlx::PgPool;
use tracing::{ Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{ BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{ layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use secrecy::ExposeSecret;



#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration.
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string()
        .expose_secret()
    )
        .expect("Failed to create Postgres connection pool.");
    // Now the hard-coded '8000' it's removed, and now it originates from our settings.
    let address = format!("{}:{}", configuration.application.host,configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())

}

use battlemon_rest::startup::Application;
use battlemon_rest::{config, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("battlemon_rest".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    tracing::info!("Loading app config");
    let config = config::get_config().expect("Failed to read configuration");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}

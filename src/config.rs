use anyhow::Context;
use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

/// Configuration for the server.
#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    /// The hostname of the server.
    pub host: String,
    /// The port of the server.
    pub port: u16,
}
/// Configuration for the database.
/// This is loaded from the `configuration.yaml` file.
#[derive(Deserialize, Clone)]
pub struct Settings {
    /// The database connection string.
    pub database: DatabaseSettings,
    /// The application settings.
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Clone)]
/// Configuration for the database.
pub struct DatabaseSettings {
    /// The database's username.
    pub username: String,
    /// The database's password.
    pub password: String,
    /// The database's port.
    pub host: String,
    /// The database's name.
    pub port: u16,
    /// The database's hostname.
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }
}

pub fn get_config() -> anyhow::Result<Settings> {
    let config_path = std::env::current_dir()
        .context("Failed to determine the current directory")?
        .join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    // todo: switch to toml
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yaml")))
        .add_source(config::File::from(config_path.join(&environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings
        .try_deserialize()
        .context("Failed to deserialize config files into `ApplicationSettings`")
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`",
                other
            )),
        }
    }
}

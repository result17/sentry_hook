use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
  let config = Config::builder()
      .add_source(File::with_name("config"))
      .build()?;
  config.try_deserialize::<Settings>()
}

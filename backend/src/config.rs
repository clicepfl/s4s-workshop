use envconfig::Envconfig;
use std::sync::OnceLock;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DATA_DIR")]
    pub data_dir: String,
    #[envconfig(from = "SOCK_DIR")]
    pub socks_dir: String,
    #[envconfig(from = "SOCK_VOL")]
    pub socks_vol: String,
    #[envconfig(from = "RUNNER_IMAGE")]
    pub runner_image: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn config() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().unwrap())
}

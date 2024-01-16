use std::net::{IpAddr, Ipv4Addr};
use serde_derive::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: HttpSourceConfig,
    #[serde(default)]
    pub osc: Option<OscConfig>,
}

impl Config {
    pub async fn read() -> color_eyre::Result<Self> {
        let mut file = File::open("config.toml").await?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        let config = toml::from_str(&buffer)?;

        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpSourceConfig {
    #[serde(default = "HttpSourceConfig::default_address")]
    pub address: IpAddr,
    #[serde(default = "HttpSourceConfig::default_port")]
    pub port: u16,
}

impl Default for HttpSourceConfig {
    fn default() -> Self {
        Self {
            address: Self::default_address(),
            port: Self::default_port(),
        }
    }
}

impl HttpSourceConfig {
    fn default_address() -> IpAddr {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }

    fn default_port() -> u16 {
        8080
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OscConfig {
    pub target: IpAddr,
    pub port: u16,
}

use crate::KvError;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerConfig {
    pub general: GeneralConfig,
    pub storage: StorageConfig,
    pub tls: ServerTlsConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientConfig {
    pub general: GeneralConfig,
    pub tls: ClientTlsConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub addr: String,
    #[serde(default)]
    pub network: NetWorkType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetWorkType {
    Tcp,
    Quic,
}

impl Default for NetWorkType {
    fn default() -> Self {
        Self::Tcp
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LogConfig {
    pub enable_log_file: bool,
    pub enable_jaeger: bool,
    pub path: String,
    pub log_level: String,
    pub rotation: RotationConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RotationConfig {
    Hourly,
    Never,
    Daily,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ServerTlsConfig {
    pub key: String,
    pub cert: String,
    pub ca: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientTlsConfig {
    pub domain: String,
    pub ca: Option<String>,
    pub identity: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "args")]
pub enum StorageConfig {
    MemTable,
    SledDb(String),
}

impl ServerConfig {
    pub async fn load(path: &str) -> Result<Self, KvError> {
        let config = fs::read_to_string(path).await?;
        let config: Self = toml::from_str(&config)?;
        Ok(config)
    }
}

impl ClientTlsConfig {
    pub async fn load(path: &str) -> Result<Self, KvError> {
        let config = fs::read_to_string(path).await?;
        let config: Self = toml::from_str(&config)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientConfig, ServerConfig};

    #[test]
    fn server_config_should_be_loaded() {
        let config: Result<ServerConfig, toml::de::Error> =
            toml::from_str(include_str!("../fixtures/server.conf"));
        assert!(config.is_ok());
    }

    #[test]
    fn client_config_should_be_loaded() {
        let config: Result<ClientConfig, toml::de::Error> =
            toml::from_str(include_str!("../fixtures/client.conf"));
        assert!(config.is_ok());
    }
}

use crate::memory::MemTable;
use anyhow::{anyhow, Result};
use s2n_quic::Server;
use tracing::instrument;

mod config;
mod error;
mod pb;
mod storage;
mod service;

pub use config::*;
pub use error::*;
pub use pb::abi::*;
pub use storage::*;

#[instrument(skip_all)]
pub async fn start_server_with_config(config: &ServerConfig) -> Result<()> {
    let addr = &config.general.addr;

    match config.general.network {
        NetWorkType::Tcp => todo!(),
        NetWorkType::Quic => match config.storage {
            StorageConfig::MemTable => {
                start_quic_server(addr.to_string(), MemTable::new(), &config.tls).await?
            }
            StorageConfig::SledDb(_) => todo!(),
        },
    }

    Ok(())
}

pub async fn start_quic_server(
    addr: String,
    store: impl Storage,
    tls: &ServerTlsConfig,
) -> Result<()> {
    let listener = Server::builder()
        .with_tls((tls.cert.as_str(), tls.key.as_str()))?
        .with_io(addr.as_str())?
        .start()
        .map_err(|e| anyhow!("Fail to start server . Error :{}", e))?;


    Ok(())
}

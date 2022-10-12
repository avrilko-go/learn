use anyhow::Result;
use learn::{start_server_with_config, RotationConfig, ServerConfig};
use std::env;
use std::str::FromStr;
use tokio::fs;
use tracing::span;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

#[tokio::main]
async fn main() -> Result<()> {
    let config = match env::var("KV_SERVER_CONFIG") {
        Ok(path) => fs::read_to_string(&path).await?,
        Err(_) => include_str!("../fixtures/quic_server.conf").to_string(),
    };

    let config: ServerConfig = toml::from_str(&config)?;
    let log = &config.log;

    env::set_var("RUST_LOG", &log.log_level);

    let std_out = fmt::layer().compact();

    let file_appender = match log.rotation {
        RotationConfig::Hourly => tracing_appender::rolling::hourly(&log.path, "server.log"),
        RotationConfig::Never => tracing_appender::rolling::never(&log.path, "server.log"),
        RotationConfig::Daily => tracing_appender::rolling::daily(&log.path, "server.log"),
    };

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let level = LevelFilter::from_str(&log.log_level)?;
    let fmt_layer = fmt::layer()
        .event_format(format().compact())
        .with_writer(non_blocking);

    let log_file_level = match log.enable_log_file {
        true => level,
        false => LevelFilter::OFF,
    };

    let jaeger_level = match log.enable_jaeger {
        true => level,
        false => LevelFilter::OFF,
    };

    let trace = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("kv-server")
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(trace);
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(std_out)
        .with(fmt_layer.with_filter(log_file_level))
        .with(opentelemetry.with_filter(jaeger_level))
        .init();

    let root = span!(tracing::Level::INFO, "app_start");
    let _enter = root.enter();

    start_server_with_config(&config).await?;
    Ok(())
}

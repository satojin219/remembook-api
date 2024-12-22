use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use registry::AppRegistryImpl;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    // Environment、環境によって出力するログレベルを変更する。本番環境ではinfo以上のログを出力する。ローカル環境ではdebug以上のログを出力する。
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // 環境変数に設定されたログレベルを取得する。環境変数が設定されていない場合は、デフォルトのログレベルを取得する。
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(subscriber)
        .try_init()?;
    Ok(())
}
async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config));

    let app = Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .with_state(registry);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

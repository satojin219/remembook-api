use shared::config::DatabaseConfig;

use sqlx::postgres::{PgConnectOptions, PgPool, PgSslMode};

fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
        .ssl_mode(PgSslMode::Require)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);
pub mod model;

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}

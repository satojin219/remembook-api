use std::sync::Arc;

use adapter::database::ConnectionPool;
use adapter::redis::RedisClient;
use adapter::repository::auth::AuthRepositoryImpl;
use adapter::repository::book::BookRepositoryImpl;
use adapter::repository::user::UserRepositoryImpl;
use kernel::repository::auth::AuthRepository;
use kernel::repository::book::BookRepository;
use kernel::repository::user::UserRepository;
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    book_repository: Arc<dyn BookRepository>,
}

impl AppRegistryImpl {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(),
            redis_client.clone(),
            app_config.auth.ttl,
        ));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));

        Self {
            auth_repository,
            user_repository,
            book_repository,
        }
    }
}

pub trait AppRegistryExt {
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }

    fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;

use std::sync::Arc;

use adapter::database::ConnectionPool;
use adapter::redis::RedisClient;
use adapter::repository::answer::AnswerRepositoryImpl;
use adapter::repository::auth::AuthRepositoryImpl;
use adapter::repository::book::BookRepositoryImpl;
use adapter::repository::question::QuestionRepositoryImpl;
use adapter::repository::summary::SummaryRepositoryImpl;
use adapter::repository::user::UserRepositoryImpl;
use kernel::repository::answer::AnswerRepository;
use kernel::repository::auth::AuthRepository;
use kernel::repository::book::BookRepository;
use kernel::repository::question::QuestionRepository;
use kernel::repository::summary::SummaryRepository;
use kernel::repository::user::UserRepository;
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    book_repository: Arc<dyn BookRepository>,
    summary_repository: Arc<dyn SummaryRepository>,
    question_repository: Arc<dyn QuestionRepository>,
    answer_repository: Arc<dyn AnswerRepository>,
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
        let summary_repository = Arc::new(SummaryRepositoryImpl::new(pool.clone()));
        let question_repository = Arc::new(QuestionRepositoryImpl::new(pool.clone()));
        let answer_repository = Arc::new(AnswerRepositoryImpl::new(pool.clone()));
        Self {
            auth_repository,
            user_repository,
            book_repository,
            summary_repository,
            question_repository,
            answer_repository,
        }
    }
}

pub trait AppRegistryExt {
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn summary_repository(&self) -> Arc<dyn SummaryRepository>;
    fn question_repository(&self) -> Arc<dyn QuestionRepository>;
    fn answer_repository(&self) -> Arc<dyn AnswerRepository>;
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

    fn summary_repository(&self) -> Arc<dyn SummaryRepository> {
        self.summary_repository.clone()
    }

    fn question_repository(&self) -> Arc<dyn QuestionRepository> {
        self.question_repository.clone()
    }

    fn answer_repository(&self) -> Arc<dyn AnswerRepository> {
        self.answer_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;

use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use super::session_repository_interface::SessionRepository;

#[async_trait]
pub trait Repositories {
    type SessionRepo: SessionRepository;

    fn new(db: Arc<Pool<Postgres>>) -> Self;
    fn session_repo(&self) -> &Self::SessionRepo;
}

pub trait TestRepositories {
    type SessionRepo: SessionRepository;

    fn session_repo(&self) -> &Self::SessionRepo;
}

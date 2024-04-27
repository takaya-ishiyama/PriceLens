use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::organization_repository_interface::OrganizationRepository;

#[async_trait]
pub trait Repositories {
    type OrganizationRepo: OrganizationRepository;

    fn new(db: Arc<Pool<Postgres>>) -> Self;
    fn organization_repo(&self) -> &Self::OrganizationRepo;
}

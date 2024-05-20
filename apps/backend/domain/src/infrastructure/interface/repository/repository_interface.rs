use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::{
    item_repository_interface::ItemRepository,
    organization_repository_interface::OrganizationRepository,
};

#[async_trait]
pub trait Repositories {
    type OrganizationRepo: OrganizationRepository;
    type ItemRepo: ItemRepository;

    fn new(db: Arc<Pool<Postgres>>) -> Self;
    fn organization_repo(&self) -> &Self::OrganizationRepo;
    fn item_repo(&self) -> &Self::ItemRepo;
}

use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    item_repository_interface::ItemRepository,
    organization_repository_interface::OrganizationRepository, repository_interface::Repositories,
};
use sqlx::{Pool, Postgres};

use super::{
    item_repository::ItemRepositoryImpl, organization_repository::OrganizationRepositoryImpl,
};

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    organization_repo: OrganizationRepositoryImpl,
    item_repo: ItemRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type OrganizationRepo = OrganizationRepositoryImpl;
    type ItemRepo = ItemRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            organization_repo: OrganizationRepositoryImpl::new(db.clone()),
            item_repo: ItemRepositoryImpl::new(db.clone()),
        }
    }

    fn organization_repo(&self) -> &Self::OrganizationRepo {
        &self.organization_repo
    }
    fn item_repo(&self) -> &Self::ItemRepo {
        &self.item_repo
    }
}

use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    organization_repository_interface::OrganizationRepository, repository_interface::Repositories,
};
use sqlx::{Pool, Postgres};

use super::organization_repository::OrganizationRepositoryImpl;

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    organization_repo: OrganizationRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type OrganizationRepo = OrganizationRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            organization_repo: OrganizationRepositoryImpl::new(db.clone()),
        }
    }

    fn organization_repo(&self) -> &Self::OrganizationRepo {
        &self.organization_repo
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use crate::value_object::organaization::{
    organization::Organization, organization_type::ORGANIZATION_TYPE,
};

#[automock]
#[async_trait]
pub trait OrganizationRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn create<'a>(
        &self,
        name: &str,
        organization_type: &ORGANIZATION_TYPE,
        private_key: Option<&'a str>,
    ) -> Result<Organization, String>;
    async fn find_one_by_id(&self, id: &str) -> Result<Organization, String>;
    async fn exist_same_name(&self, name: &str) -> Result<bool, String>;
}

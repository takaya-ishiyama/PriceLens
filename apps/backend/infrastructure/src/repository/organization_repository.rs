use std::sync::Arc;

use axum::async_trait;
use domain::{
    infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
    value_object::organaization::organization::Organization,
};
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct OrganizationRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    // fn create(&self, user_id: &str) -> Result<Organization, String> {
    //     todo!()
    // }
    // fn find_one(&self, id: &str) -> Result<Organization, String> {
    //     todo!()
    // }
    // fn find_all_with_pagination(
    //     &self,
    //     limit: i32,
    //     offset: i32,
    // ) -> Result<Vec<Organization>, String> {
    //     todo!()
    // }
}

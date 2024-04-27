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
    /* async fn find_many_by_name_with_pagination(
        &self,
        name: &str,
        limit: &i32,
        offset: &i32,
    )  -> Result<Vec<Organization>, String>;
    */
    // async fn find_all_with_pagination(
    //     &self,
    //     limit: &i32,
    //     offset: &i32,
    // ) -> Result<Vec<Organization>, String>;
    // async fn find_many(&self, user_id: &str) -> Result<Vec<Organization>, String>;
    // async fn update(&self, Organization: &OrganizationUpdateInput) -> Result<Organization, String>;
    // async fn upsert(&self, Organization: &OrganizationUpdateInput) -> Result<Organization, String>;
}

// pub struct OrganizationUpdateInput {
//     id: String,
//     name: String,
//     organization_type: ORGANIZATION_TYPE,
// }

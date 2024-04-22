use std::sync::Arc;

use axum::async_trait;
use domain::{
    infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
    value_object::organaization::organization::Organization,
    value_object::organaization::organization_type::ORGANIZATION_TYPE,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct OrganizationRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "organization_type", rename_all = "lowercase")]
pub enum OrganizationType {
    PUBLIC,
    PRIVATE,
}

#[derive(FromRow)]
struct CreatedOrganization {
    id: i64,
    name: String,
    organization_type: OrganizationType,
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn create<'a>(
        &self,
        name: &str,
        organization_type: &ORGANIZATION_TYPE,
        private_key: Option<&'a str>,
    ) -> Result<Organization, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let o_type = match organization_type {
            ORGANIZATION_TYPE::PUBLIC => OrganizationType::PUBLIC,
            ORGANIZATION_TYPE::PRIVATE => OrganizationType::PRIVATE,
        };

        let organization = sqlx::query_as::<_, CreatedOrganization>(
            "INSERT INTO organization (name, organization_type) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(o_type)
        .fetch_one(&mut *tx)
        .await;

        match organization {
            Ok(organization) => {
                tx.commit().await.unwrap();

                let _organization_type = match organization.organization_type {
                    OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };
                Ok(Organization::new(
                    organization.id.to_string(),
                    organization.name,
                    _organization_type,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }
}

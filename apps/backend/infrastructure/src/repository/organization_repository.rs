use std::sync::Arc;

use axum::async_trait;
use domain::{
    infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
    value_object::organaization::organization::Organization,
    value_object::organaization::organization_type::ORGANIZATION_TYPE,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, Acquire, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct OrganizationRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "organization_type", rename_all = "lowercase")]
pub enum OrganizationType {
    #[sqlx(rename = "PUBLIC")]
    PUBLIC,
    #[sqlx(rename = "PRIVATE")]
    PRIVATE,
}

#[derive(FromRow)]
struct CreatedOrganization {
    id: Uuid,
    name: String,
    organization_type: OrganizationType,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
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

        let _organization_type = match organization_type {
            ORGANIZATION_TYPE::PUBLIC => OrganizationType::PUBLIC,
            ORGANIZATION_TYPE::PRIVATE => OrganizationType::PRIVATE,
        };

        let organization = sqlx::query_as!(
            CreatedOrganization,
            r#"INSERT INTO organization (name, organization_type) VALUES ($1, $2) RETURNING id,name,organization_type AS "organization_type!: OrganizationType",created_at,updated_at"#,
            name,
            _organization_type as OrganizationType
        )
        .fetch_one(&mut *tx)
        .await;

        match organization {
            Ok(organization) => {
                tx.commit().await.unwrap();
                let tmp_organization_type = match organization.organization_type {
                    OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };
                Ok(Organization::new(
                    organization.id.to_string(),
                    organization.name,
                    tmp_organization_type,
                    // ORGANIZATION_TYPE::PUBLIC,
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
#[cfg(test)]
mod tests {

    use sqlx::PgPool;

    use crate::test::setup_testdb::setup_database;

    use super::*;

    #[sqlx::test]
    async fn test_create_true_response(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = OrganizationRepositoryImpl::new(db);

        let ognz_type = ORGANIZATION_TYPE::PUBLIC;
        let organization = repo.create("test", &ognz_type, Some("")).await.unwrap();

        assert_eq!(organization.get_params().name, "test".to_string());

        Ok(())
    }

    // #[sqlx::test]
    // async fn test_create_insert_to_db(pool: PgPool) -> sqlx::Result<()> {
    //     let db = Arc::new(pool);
    //     let repo = OrganizationRepositoryImpl::new(db);
    //     let conn = pool.acquire().await;

    //     let ognz_type = ORGANIZATION_TYPE::PUBLIC;
    //     let organization = repo.create("test", &ognz_type, Some("")).await.unwrap();

    //     let getognz = sqlx::query!(
    //         "SELECT * FROM organization WHERE id = ?",
    //         organization.get_params().id
    //     )
    //     .fetch_one(&mut pool)
    //     .await?;

    //     assert_eq!(organization.get_params().name, "test".to_string());

    //     Ok(())
    // }
}

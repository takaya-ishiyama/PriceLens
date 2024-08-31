use std::sync::Arc;

use async_graphql::connection::PageInfo;
use axum::async_trait;
use domain::{
    infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
    value_object::organaization::organization::Organization,
    value_object::organaization::organization_type::ORGANIZATION_TYPE,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::chrono::{Local, NaiveDate, NaiveDateTime},
    Acquire, Pool, Postgres,
};
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

#[derive(FromRow)]
struct FindManyOrganization {
    id: Uuid,
    name: String,
    organization_type: OrganizationType,
}

#[async_trait]
// FIXME:分割したい
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
                let tmp_organization_type = match organization.organization_type {
                    OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };

                tx.commit().await.unwrap();
                Ok(Organization::new(
                    &organization.id.to_string(),
                    &organization.name,
                    &tmp_organization_type,
                    "",
                    "",
                    "",
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }
    async fn find_one_by_id(&self, id: &str) -> Result<Organization, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let _id = Uuid::parse_str(id).unwrap();

        let organization = sqlx::query_as!(
            CreatedOrganization,
            r#"SELECT id,name,organization_type AS "organization_type!: OrganizationType",created_at,updated_at FROM organization WHERE id = $1"#,
            _id
        )
        .fetch_one(&mut *tx)
        .await;

        match organization {
            Ok(organization) => {
                let tmp_organization_type = match organization.organization_type {
                    OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };

                tx.commit().await.unwrap();
                Ok(Organization::new(
                    &organization.id.to_string(),
                    &organization.name,
                    &tmp_organization_type,
                    "",
                    "",
                    "",
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }
    async fn exist_same_name(&self, name: &str) -> Result<bool, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let exists = sqlx::query!(
            r#"SELECT EXISTS (SELECT 1 FROM organization WHERE name = $1) as "exists!""#,
            name
        )
        .fetch_one(&mut *conn)
        .await;

        match exists {
            Ok(exists) => Ok(exists.exists),
            Err(e) => Err(e.to_string()),
        }
    }
    async fn find_many_by_name(&self, name: &str) -> Result<Vec<Organization>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let found_ogn = sqlx::query_as!(
            FindManyOrganization,
            r#"SELECT id,name,organization_type AS "organization_type!: OrganizationType" FROM organization WHERE name = $1"#,
            name
        )
        .fetch_all(&mut *tx)
        .await;

        match found_ogn {
            Ok(ogn) => ogn
                .iter()
                .map(|_org| {
                    let tmp_organization_type = match _org.organization_type {
                        OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                        OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                    };
                    Ok(Organization::new(
                        &_org.id.to_string(),
                        &_org.name,
                        &tmp_organization_type,
                        "",
                        "",
                        "",
                    ))
                })
                .collect(),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_all_with_pagenate(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Vec<Organization>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        // afterをbeforeはどちらかしか来ないからdate = match(first, last)とかでいいかも。その場合after,beforeのハンドリングも考える
        let after_cursor = match after {
            Some(_after) => {
                let tmp = Uuid::parse_str(&_after);
                match tmp {
                    Ok(tmp) => tmp,
                    Err(_) => Uuid::nil(),
                }
            }
            None => Uuid::nil(),
        };

        let before_cursor = match before {
            Some(_before) => {
                let tmp = Uuid::parse_str(&_before);
                match tmp {
                    Ok(tmp) => tmp,
                    Err(_) => Uuid::nil(),
                }
            }
            None => Uuid::nil(),
        };

        // ulidにしたほうがいいかも。だるい
        let onzs = match (first, last) {
            (Some(first), _) => {
                if after_cursor.is_nil() {
                    let found_ogn = sqlx::query_as!(
                        FindManyOrganization,
                        r#"
                            SELECT
                              id,name,organization_type AS "organization_type!: OrganizationType" 
                              FROM organization 
                              ORDER BY created_at DESC 
                              LIMIT 10;
                        "# // first // limitをフロントから受け付ける場合のみ。とりま固定値でいいや
                    )
                    .fetch_all(&mut *tx)
                    .await;

                    match found_ogn {
                        Ok(ogn) => ogn,
                        Err(e) => return Err(e.to_string()),
                    }
                } else {
                    let found_ogn = sqlx::query_as!(
                        FindManyOrganization,
                        r#"
                    SELECT
                      id,name,organization_type AS "organization_type!: OrganizationType" 
                      FROM organization 
                      WHERE created_at > (
                        SELECT created_at
                        FROM organization
                        WHERE id = $1
                      ) 
                      ORDER BY created_at DESC 
                      LIMIT 10;
                    "#,
                        after_cursor,
                        // first // limitをフロントから受け付ける場合のみ。とりま固定値でいいや
                    )
                    .fetch_all(&mut *tx)
                    .await;

                    match found_ogn {
                        Ok(ogn) => ogn,
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            (None, Some(last)) => {
                if before_cursor.is_nil() {
                    let found_ogn = sqlx::query_as!(
                        FindManyOrganization,
                        r#"
                    SELECT
                      id,name,organization_type AS "organization_type!: OrganizationType" 
                      FROM organization 
                      ORDER BY created_at ASC 
                      LIMIT 10;
                    "#,
                        // last // limitをフロントから受け付ける場合のみ。とりま固定値でいいや
                    )
                    .fetch_all(&mut *tx)
                    .await;

                    match found_ogn {
                        Ok(ogn) => ogn,
                        Err(e) => return Err(e.to_string()),
                    }
                } else {
                    let found_ogn = sqlx::query_as!(
                        FindManyOrganization,
                        r#"
                    SELECT
                      id,name,organization_type AS "organization_type!: OrganizationType" 
                      FROM organization 
                      WHERE created_at < (
                        SELECT created_at
                        FROM organization
                        WHERE id = $1
                      ) 
                      ORDER BY created_at ASC 
                      LIMIT 10;
                    "#,
                        before_cursor,
                        // last // limitをフロントから受け付ける場合のみ。とりま固定値でいいや
                    )
                    .fetch_all(&mut *tx)
                    .await;

                    match found_ogn {
                        Ok(ogn) => ogn,
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            (None, None) => Vec::new(),
        };

        onzs.iter()
            .map(|org| {
                let tmp_organization_type = match org.organization_type {
                    OrganizationType::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    OrganizationType::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };
                Ok(Organization::new(
                    &org.id.to_string(),
                    &org.name,
                    &tmp_organization_type,
                    "",
                    "",
                    "",
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use sqlx::PgPool;

    use crate::test::{setup_testdb::setup_database, stb_data::stb_organization};

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

    #[sqlx::test]
    async fn test_find_one(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let stb = stb_organization();
        let db = Arc::new(pool);
        let repo = OrganizationRepositoryImpl::new(db);

        // sqlx::query!(
        //     r#"
        //     INSERT INTO organization (id, name, organization_type, created_at, updated_at)
        //     VALUES ('17b5ac0c-1429-469a-8522-053f7bf0f80d','名無しの組織', 'PUBLIC', '2021-09-01 00:00:00', '2021-09-01 00:00:00')
        //     "#
        // );

        // let organization = repo.find_one_by_id(&stb[0].get_params().id).await.unwrap();
        let organization = repo
            .find_one_by_id("17b5ac0c-1429-469a-8522-053f7bf0f80d")
            .await
            .unwrap();

        assert_eq!(organization.get_params().id, stb[0].get_params().id);

        Ok(())
    }
}

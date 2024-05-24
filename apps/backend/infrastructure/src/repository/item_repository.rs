use std::sync::Arc;

use axum::async_trait;
use domain::{
    infrastructure::interface::repository::item_repository_interface::ItemRepository,
    value_object::Item::item::Item,
};
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, Acquire, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ItemRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct CreatedItem {
    id: Uuid,
    organization_id: Uuid,
    name: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
#[derive(FromRow)]
struct FindNameItem {
    id: Uuid,
    name: String,
}

#[async_trait]
impl ItemRepository for ItemRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn create(&self, name: &str, organization_id: &str) -> Result<Item, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let organization_id_of_uuid = match Uuid::parse_str(organization_id) {
            Ok(uuid) => uuid,
            Err(e) => return Err(format!("Invalid UUID: {}", e)),
        };

        let created_item = sqlx::query_as!(
            CreatedItem,
            r#"INSERT INTO Item (name, organization_id) VALUES ($1, $2) RETURNING id,name,organization_id,created_at,updated_at"#,
            name,
            organization_id_of_uuid
        )
        .fetch_one(&mut *tx)
        .await;

        match created_item {
            Ok(item) => {
                tx.commit().await.unwrap();
                Ok(Item::new(
                    &item.id.to_string(),
                    &item.organization_id.to_string(),
                    &item.name,
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
    async fn find_names(&self, organization_id: &str) -> Result<Vec<Item>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let organization_id_of_uuid = Uuid::parse_str(organization_id).unwrap();

        let find_items = sqlx::query_as!(
            FindNameItem,
            r#"SELECT id,name FROM Item WHERE organization_id = $1"#,
            organization_id_of_uuid
        )
        .fetch_all(&mut *tx)
        .await;

        match find_items {
            Ok(item) => {
                tx.commit().await.unwrap();
                Ok(Vec::from(item)
                    .iter()
                    .map(|i| Item::new(&i.id.to_string(), organization_id, &i.name, "", "", ""))
                    .collect())
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

    use crate::test::{setup_testdb::setup_database, stb_data::stb_organization};

    use super::*;

    #[sqlx::test]
    async fn test_create_item(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = ItemRepositoryImpl::new(db);

        let item = repo
            .create("test", "13af8f1e-6681-4d9e-b0ec-37078cf44628")
            .await
            .unwrap();

        assert_eq!(1, 1);

        Ok(())
    }
}

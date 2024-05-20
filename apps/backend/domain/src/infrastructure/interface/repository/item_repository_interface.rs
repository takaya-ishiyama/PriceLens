use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use crate::value_object::Item::item::Item;

#[automock]
#[async_trait]
pub trait ItemRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn create(&self, organization_id: &str, name: &str) -> Result<Item, String>;
    async fn find_names(&self, organization_id: &str) -> Result<Vec<Item>, String>;
}

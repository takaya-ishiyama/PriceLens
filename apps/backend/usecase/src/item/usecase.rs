use domain::{
    infrastructure::interface::repository::item_repository_interface::ItemRepository,
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::Item::item::Item,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ItemInteractor<'r, R: Repositories> {
    item_repo: &'r R::ItemRepo,
}

impl<'r, R: Repositories> ItemInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            item_repo: repositories.item_repo(),
        }
    }

    pub async fn create_item(&self, name: &str, organization_id: &str) -> Result<Item, String> {
        let item = self.item_repo.create(name, &organization_id).await;
        match item {
            Ok(item) => Ok(item),
            Err(e) => Err(e),
        }
    }
}

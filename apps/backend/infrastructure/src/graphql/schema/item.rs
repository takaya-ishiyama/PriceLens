use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct ItemSchema {
    id: String,
    name: String,
    organization_id: String,
}

impl ItemSchema {
    pub fn new(id: String, name: String, organization_id: String) -> Self {
        Self {
            id,
            name,
            organization_id,
        }
    }
}

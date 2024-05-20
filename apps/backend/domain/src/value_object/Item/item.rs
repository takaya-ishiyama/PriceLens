#[derive(Debug)]
pub struct Item {
    id: String,
    organization_id: String,
    name: String,
    created_at: String,
    updated_at: String,
    deleted_at: String,
}

impl Item {
    pub fn new(
        id: &str,
        organization_id: &str,
        name: &str,
        created_at: &str,
        updated_at: &str,
        deleted_at: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            organization_id: organization_id.to_string(),
            name: name.to_string(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            deleted_at: deleted_at.to_string(),
        }
    }
}

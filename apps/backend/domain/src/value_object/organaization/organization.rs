use super::organization_type::ORGANIZATION_TYPE;

pub struct Organization {
    id: String,
    name: String,
    organization_type: ORGANIZATION_TYPE,
    created_at: String,
    updated_at: String,
    deleted_at: String,
}

impl Organization {
    pub fn new(
        id: String,
        name: String,
        organization_type: ORGANIZATION_TYPE,
        created_at: String,
        updated_at: String,
        deleted_at: String,
    ) -> Self {
        Self {
            id,
            name,
            organization_type,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}

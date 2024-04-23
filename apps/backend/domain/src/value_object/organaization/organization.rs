use super::organization_type::ORGANIZATION_TYPE;

pub struct Organization {
    id: String,
    name: String,
    organization_type: ORGANIZATION_TYPE,
    created_at: String,
    updated_at: String,
    deleted_at: String,
}

pub struct GetParams {
    pub id: String,
    pub name: String,
    pub organization_type: ORGANIZATION_TYPE,
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

    /** qgl型に渡すための関数。命名を考える */
    pub fn get_params(&self) -> GetParams {
        GetParams {
            id: self.id.clone(),
            name: self.name.clone(),
            organization_type: self.organization_type.clone(),
        }
    }
}

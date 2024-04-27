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
        id: &str,
        name: &str,
        organization_type: &ORGANIZATION_TYPE,
        created_at: &str,
        updated_at: &str,
        deleted_at: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            organization_type: organization_type.clone(),
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            deleted_at: deleted_at.to_string(),
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

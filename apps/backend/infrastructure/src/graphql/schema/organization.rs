use async_graphql::SimpleObject;
use domain::value_object::organaization::organization_type::ORGANIZATION_TYPE;

#[derive(SimpleObject)]
pub struct OrganizationSchema {
    id: String,
    name: String,
    organization_type: ORGANIZATION_TYPE,
}

pub impl OrganizationSchema {
    pub fn new(id: String, name: String, organization_type: ORGANIZATION_TYPE) -> Self {
        Self {
            id,
            name,
            organization_type,
        }
    }
}

use async_graphql::{Enum, SimpleObject};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ORGANIZATION_TYPE {
    PUBLIC,
    PRIVATE,
}
#[derive(SimpleObject)]
pub struct OrganizationSchema {
    id: String,
    name: String,
    organization_type: ORGANIZATION_TYPE,
}

impl OrganizationSchema {
    pub fn new(id: String, name: String, organization_type: ORGANIZATION_TYPE) -> Self {
        Self {
            id,
            name,
            organization_type,
        }
    }
}

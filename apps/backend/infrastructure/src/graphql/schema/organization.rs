use async_graphql::{Enum, SimpleObject};
use domain::value_object::organaization::{
    organization::Organization, organization_type::ORGANIZATION_TYPE as DOMAIN_ORGANIZATION_TYPE,
};

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
    pub fn new(organization: Organization) -> Self {
        let params = organization.get_params();
        let organization_type = match params.organization_type {
            DOMAIN_ORGANIZATION_TYPE::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
            DOMAIN_ORGANIZATION_TYPE::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
        };
        Self {
            id: params.id,
            name: params.name,
            organization_type,
        }
    }

    pub fn new_from_domain_organizations(organizations: Vec<Organization>) -> Vec<Self> {
        organizations
            .iter()
            .map(|_org| {
                let params = _org.get_params();
                let org_type = match params.organization_type {
                    DOMAIN_ORGANIZATION_TYPE::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    DOMAIN_ORGANIZATION_TYPE::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };
                Self {
                    id: params.id,
                    name: params.name,
                    organization_type: org_type,
                }
            })
            .collect()
    }
}

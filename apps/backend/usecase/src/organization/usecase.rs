use domain::{
    infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::organaization::{
        organization::Organization, organization_type::ORGANIZATION_TYPE,
    },
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct OrganizationInteractor<'r, R: Repositories> {
    organization_repo: &'r R::OrganizationRepo,
}

impl<'r, R: Repositories> OrganizationInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            organization_repo: repositories.organization_repo(),
        }
    }

    pub async fn create_organization(
        &self,
        name: &str,
        organization_type: &ORGANIZATION_TYPE,
        private_key: Option<&'r str>,
    ) -> Result<Organization, String> {
        let organization = self
            .organization_repo
            .create(name, organization_type, private_key)
            .await?;
        Ok(organization)
    }
}
